#!/usr/bin/env python3
"""📊 Pass A schema-ownership audit ledger.

Reads the immutable repository tree at HEAD via `git ls-tree` / `git cat-file`
(never the dirty worktree), classifies every tracked path into one primary
bucket plus tags, computes JSON content flags for schema-relevant JSON blobs,
and detects cross-platform path-integrity collisions.

Outputs:
  - 📊️wp0-ledger.json        aggregate counts + coverage + collisions + capped listings
  - 🗑️generated/wp0-ledger-full.jsonl   one line per tracked entry (full detail, no cap)
  - 📓️wp0-ledger.md          narrative report

Run: python3 wp0-ledger.py   (cwd must be the repo root; reads HEAD via git plumbing)
"""
import json
import re
import subprocess
import sys
import threading
import unicodedata
import collections
import os

REPO = "/Users/ueli/Documents/semio"
TICKET = os.path.join(
    REPO,
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS",
)
GEN = os.path.join(TICKET, "🗑️generated")
OUT_JSON = os.path.join(TICKET, "📊️wp0-ledger.json")
OUT_JSONL = os.path.join(GEN, "wp0-ledger-full.jsonl")
OUT_MD = os.path.join(TICKET, "📓️wp0-ledger.md")

MAX_SUMMARY_BYTES = 2 * 1024 * 1024
OVERSIZE_THRESHOLD = 2 * 1024 * 1024
PEEK_BYTES = 8192
JSON_PARSE_CAP = 20 * 1024 * 1024

VENDOR_PAT = re.compile(r'^(node_modules|target(-.*)?|storybook-static|temp|test-results)$', re.I)
SCHEMA_NAMED_EXACT = {'🧬️schema.json', '🛂️schema.json'}
FORMAT_EXT_SUFFIXES = ('.proto', '.graphql', '.wit', '.avsc', '.xsd', '.ksy',
                        '.ebnf', '.abnf', '.g4', '.spicy', '.sql',
                        '.grammar.semio', '.protocol.semio')
SOURCE_EXTS = ('.rs', '.ts', '.tsx', '.go', '.py', '.cs', '.c', '.cpp', '.h',
               '.swift', '.kt', '.java', '.mjs', '.cjs')
CONFIG_EXTS = ('.toml', '.yaml', '.yml', '.xml', '.plist')
CONFIG_JSON_RE = re.compile(r'^(package|project|nx|workspace|tsconfig(\..+)?)\.json$', re.I)
DOC_EXTS = ('.md',)
ASSET_EXTS = ('.png', '.jpg', '.jpeg', '.gif', '.svg', '.ico', '.webp', '.bmp',
              '.tiff', '.woff', '.woff2', '.ttf', '.otf', '.eot', '.mp4',
              '.mp3', '.wav', '.pdf')
VS_CHARS = ('︎', '️')
TICKET_PREFIX = ".🧬semio/🦑️repo/🎫️tickets/"


def run(cmd, **kw):
    return subprocess.run(cmd, cwd=REPO, capture_output=True, check=True, **kw)


def get_head():
    return run(["git", "rev-parse", "HEAD"]).stdout.decode().strip()


def parse_ls_tree(raw):
    entries = []
    for chunk in raw.split(b'\0'):
        if not chunk:
            continue
        meta, tab, path_b = chunk.partition(b'\t')
        if not tab:
            continue
        parts = meta.split()
        mode, typ, sha, size_s = parts[0], parts[1], parts[2], parts[3]
        size = None if size_s == b'-' else int(size_s)
        path = path_b.decode('utf-8', errors='surrogateescape')
        invalid_encoding = any(0xD800 <= ord(c) <= 0xDFFF for c in path)
        entries.append({
            'mode': mode.decode(), 'type': typ.decode(), 'sha': sha.decode(),
            'size': size, 'path': path, 'invalid_encoding': invalid_encoding,
        })
    return entries


def classify(path):
    parts = path.split('/')
    basename = parts[-1]
    lower_base = basename.lower()
    tags = set()

    if path.startswith(TICKET_PREFIX):
        tags.add('historical-ticket')
    if any(VENDOR_PAT.match(seg) for seg in parts[:-1]):
        tags.add('vendor-or-build')
    if '🧬️schema' in parts[:-1]:
        tags.add('schema-module-file')
    ftt = False
    for seg in parts[:-1]:
        if seg.startswith('🧪️') or seg.startswith('🧫️') or seg.lower() in ('fixtures', 'tests', 'test'):
            ftt = True
            break
    if ftt:
        tags.add('fixture-or-test-tree')
    if lower_base.endswith('.schema.json') or basename in SCHEMA_NAMED_EXACT:
        tags.add('schema-named-file')
    if lower_base.endswith(FORMAT_EXT_SUFFIXES):
        tags.add('schema-format-file')
    if lower_base.endswith('.json'):
        tags.add('json')
    if lower_base.endswith(SOURCE_EXTS):
        tags.add('source-ext')
    if lower_base.endswith(CONFIG_EXTS) or CONFIG_JSON_RE.match(basename):
        tags.add('config-ext')
    if lower_base.endswith(DOC_EXTS):
        tags.add('doc-ext')
    if lower_base.endswith(ASSET_EXTS):
        tags.add('asset-ext')

    priority = ['historical-ticket', 'vendor-or-build', 'schema-module-file',
                'fixture-or-test-tree', 'schema-named-file', 'schema-format-file']
    primary = None
    for p in priority:
        if p in tags:
            primary = p
            break
    if primary is None:
        if 'json' in tags:
            primary = 'json-other'
        elif 'source-ext' in tags:
            primary = 'source'
        elif 'config-ext' in tags:
            primary = 'config'
        elif 'doc-ext' in tags:
            primary = 'doc'
        elif 'asset-ext' in tags:
            primary = 'asset'
        else:
            primary = 'other'
    return primary, sorted(tags)


def strip_vs(s):
    for c in VS_CHARS:
        s = s.replace(c, '')
    return s


def compute_collisions(paths):
    case_groups = collections.defaultdict(list)
    nfc_groups = collections.defaultdict(list)
    vs_groups = collections.defaultdict(list)
    long_paths = []
    trailing = []
    for p in paths:
        case_groups[p.lower()].append(p)
        nfc_groups[unicodedata.normalize('NFC', p)].append(p)
        vs_groups[strip_vs(p)].append(p)
        if len(p) > 240:
            long_paths.append({'path': p, 'length': len(p)})
        for seg in p.split('/'):
            if seg.endswith(' ') or seg.endswith('.'):
                trailing.append(p)
                break

    case_insensitive = [{'key': k, 'paths': sorted(set(v))}
                         for k, v in case_groups.items() if len(set(v)) > 1]
    normalization = [{'nfc': k, 'paths': sorted(set(v))}
                      for k, v in nfc_groups.items() if len(set(v)) > 1]
    variation_selector = [{'stripped': k, 'paths': sorted(set(v))}
                           for k, v in vs_groups.items() if len(set(v)) > 1]
    return {
        'caseInsensitive': case_insensitive,
        'normalization': normalization,
        'variationSelector': variation_selector,
        'longPaths': long_paths,
        'trailingSpaceOrDot': sorted(set(trailing)),
    }


def batch_read(shas_needed_full, all_shas):
    """Single `git cat-file --batch` pass. Returns sha -> {'first8k':bytes,'full':bytes|None,'size':int}."""
    result = {}
    if not all_shas:
        return result
    proc = subprocess.Popen(
        ["git", "cat-file", "--batch"], cwd=REPO,
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )

    def writer():
        try:
            for sha in all_shas:
                proc.stdin.write(sha.encode() + b'\n')
            proc.stdin.close()
        except BrokenPipeError:
            pass

    t = threading.Thread(target=writer)
    t.start()

    out = proc.stdout
    for sha in all_shas:
        header = out.readline()
        if not header:
            break
        hparts = header.split()
        if len(hparts) < 3 or hparts[1] == b'missing':
            continue
        h_sha, h_type, h_size = hparts[0].decode(), hparts[1].decode(), int(hparts[2])
        need_full = h_sha in shas_needed_full
        if need_full and h_size <= JSON_PARSE_CAP:
            content = out.read(h_size)
            first8k = content[:PEEK_BYTES]
            full = content
        else:
            first8k = out.read(min(h_size, PEEK_BYTES))
            remaining = h_size - len(first8k)
            if remaining > 0:
                left = remaining
                while left > 0:
                    chunk = out.read(min(left, 1024 * 1024))
                    if not chunk:
                        break
                    left -= len(chunk)
            full = None
        out.read(1)  # trailing newline
        result[h_sha] = {'first8k': first8k, 'full': full, 'size': h_size}
    t.join()
    proc.stdout.close()
    proc.wait()
    return result


XSEMIO_RE = re.compile(r'"(x-semio-[A-Za-z0-9_-]*)"')


def json_flags(raw_bytes):
    flags = {}
    try:
        text = raw_bytes.decode('utf-8')
    except UnicodeDecodeError:
        flags['parseError'] = 'utf8-decode-error'
        return flags
    try:
        val = json.loads(text)
    except json.JSONDecodeError as e:
        flags['parseError'] = str(e)
        return flags
    flags['parseError'] = None
    flags['isBooleanSchema'] = isinstance(val, bool)
    if isinstance(val, dict):
        flags['hasDollarSchema'] = '$schema' in val
        flags['hasDollarId'] = '$id' in val
        flags['hasDefs'] = ('$defs' in val) or ('definitions' in val)
        flags['isObjectTypeWithProperties'] = val.get('type') == 'object' and 'properties' in val
    else:
        flags['hasDollarSchema'] = False
        flags['hasDollarId'] = False
        flags['hasDefs'] = False
        flags['isObjectTypeWithProperties'] = False
    xs = set(XSEMIO_RE.findall(text))
    flags['hasXSemioKeys'] = bool(xs)
    if xs:
        flags['xSemioKeys'] = sorted(xs)[:20]
    return flags


def main():
    commit = get_head()
    print(f"HEAD = {commit}", file=sys.stderr)

    raw = run(["git", "ls-tree", "-r", "-z", "--long", "HEAD"]).stdout
    entries = parse_ls_tree(raw)
    print(f"parsed {len(entries)} tree entries", file=sys.stderr)

    for e in entries:
        primary, tags = classify(e['path'])
        e['bucket'] = primary
        e['tags'] = tags

    all_paths = [e['path'] for e in entries]
    collisions = compute_collisions(all_paths)

    submodules = [e for e in entries if e['type'] == 'commit']
    symlinks = [e for e in entries if e['mode'] == '120000']
    blob_entries = [e for e in entries if e['type'] == 'blob']

    json_full_needed = set()
    for e in blob_entries:
        if e['path'].lower().endswith('.json') and e['bucket'] not in ('historical-ticket', 'vendor-or-build'):
            json_full_needed.add(e['sha'])

    unique_shas = list(dict.fromkeys(e['sha'] for e in blob_entries))
    print(f"unique blob shas: {len(unique_shas)}, needing full JSON parse: {len(json_full_needed)}", file=sys.stderr)

    cache = batch_read(json_full_needed, unique_shas)
    print("cat-file batch pass complete", file=sys.stderr)

    lfs = []
    binaries_count = 0
    oversized = []
    parse_errors = []
    counts_by_bucket = collections.Counter()
    counts_by_tag = collections.Counter()
    counts_by_flag = collections.Counter()
    json_flag_matrix = collections.defaultdict(collections.Counter)

    full_lines = []
    summary_listing_buckets = {'schema-named-file', 'schema-format-file'}

    schema_module_leaf_counts = collections.Counter()
    schema_module_ext_flag_counts = collections.defaultdict(collections.Counter)

    listing_rows = collections.defaultdict(list)
    ftt_or_jsonother_schema_flagged = []

    for e in blob_entries:
        sha = e['sha']
        c = cache.get(sha)
        flags = {}
        if c is not None:
            first8k = c['first8k']
            is_lfs = first8k.startswith(b'version https://git-lfs')
            is_binary = b'\x00' in first8k
            is_oversized = (e['size'] or 0) > OVERSIZE_THRESHOLD
            flags['isLfsPointer'] = is_lfs
            flags['isBinary'] = is_binary
            flags['isOversized'] = is_oversized
            if is_lfs:
                lfs.append({'path': e['path'], 'blob': sha, 'size': e['size']})
            if is_binary:
                binaries_count += 1
            if is_oversized:
                oversized.append({'path': e['path'], 'blob': sha, 'size': e['size']})

            if sha in json_full_needed and c['full'] is not None:
                jf = json_flags(c['full'])
                flags['json'] = jf
                if jf.get('parseError'):
                    parse_errors.append({'path': e['path'], 'blob': sha, 'error': jf['parseError']})
                for k, v in jf.items():
                    if k in ('parseError', 'xSemioKeys'):
                        continue
                    counts_by_flag[f"{k}={v}"] += 1
                    json_flag_matrix[e['bucket']][f"{k}={v}"] += 1
                if e['bucket'] in ('fixture-or-test-tree', 'json-other') and (
                        jf.get('hasDollarSchema') or jf.get('hasDefs') or jf.get('isObjectTypeWithProperties')):
                    ftt_or_jsonother_schema_flagged.append({
                        'path': e['path'], 'blob': sha, 'size': e['size'],
                        'tags': e['tags'], 'flags': jf,
                    })
            elif sha in json_full_needed and c['full'] is None:
                parse_errors.append({'path': e['path'], 'blob': sha, 'error': 'skipped-exceeds-parse-cap'})
        else:
            flags['coverageGap'] = 'cat-file-batch-miss'

        counts_by_bucket[e['bucket']] += 1
        for t in e['tags']:
            counts_by_tag[t] += 1

        if e['bucket'] == 'schema-module-file':
            schema_module_leaf_counts[e['path'].split('/')[-1]] += 1
            if 'json' in flags:
                for k, v in flags['json'].items():
                    if k in ('parseError', 'xSemioKeys'):
                        continue
                    schema_module_ext_flag_counts[e['path'].split('/')[-1]][f"{k}={v}"] += 1

        if e['bucket'] in summary_listing_buckets:
            listing_rows[e['bucket']].append({
                'path': e['path'], 'blob': sha, 'size': e['size'],
                'tags': e['tags'], 'flags': flags,
            })

        full_lines.append(json.dumps({
            'path': e['path'], 'mode': e['mode'], 'type': e['type'], 'blob': sha,
            'size': e['size'], 'bucket': e['bucket'], 'tags': e['tags'], 'flags': flags,
        }, ensure_ascii=False))

    for e in submodules:
        counts_by_bucket['(submodule)'] += 1
        full_lines.append(json.dumps({
            'path': e['path'], 'mode': e['mode'], 'type': e['type'], 'blob': e['sha'],
            'size': None, 'bucket': e['bucket'], 'tags': e['tags'],
            'flags': {'submodule': True},
        }, ensure_ascii=False))

    os.makedirs(GEN, exist_ok=True)
    with open(OUT_JSONL, 'w', encoding='utf-8') as f:
        f.write('\n'.join(full_lines) + '\n')
    print(f"wrote {OUT_JSONL} ({len(full_lines)} lines)", file=sys.stderr)

    invalid_encoding = [e['path'] for e in entries if e['invalid_encoding']]
    executables = [e['path'] for e in entries if e['mode'] == '100755']

    summary = {
        'commit': commit,
        'generatedBy': 'wp0-ledger.py',
        'counts': {
            'total': len(entries),
            'byBucket': dict(counts_by_bucket),
            'byTag': dict(counts_by_tag),
            'byFlag': dict(counts_by_flag),
        },
        'schemaModuleLeafKinds': dict(schema_module_leaf_counts.most_common(60)),
        'coverage': {
            'submodules': [{'path': e['path'], 'commit': e['sha']} for e in submodules],
            'symlinks': [{'path': e['path'], 'blob': e['sha']} for e in symlinks],
            'lfs': lfs,
            'binariesCount': binaries_count,
            'oversized': oversized,
            'parseErrors': parse_errors,
            'invalidPathEncoding': invalid_encoding,
            'executables': {'count': len(executables), 'samplePaths': executables[:50]},
        },
        'collisions': collisions,
        'listings': {
            'schema-named-file': listing_rows.get('schema-named-file', []),
            'schema-format-file': listing_rows.get('schema-format-file', []),
            'schema-flagged-json-in-fixture-test-or-json-other': ftt_or_jsonother_schema_flagged,
        },
        'note_schema_module_file_full_listing': (
            f"schema-module-file bucket has {counts_by_bucket['schema-module-file']} entries; "
            "too large to enumerate here under the 2MB cap. Full per-file rows (path/blob/size/tags/flags) "
            "are in 🗑️generated/wp0-ledger-full.jsonl (filter bucket==\"schema-module-file\"). "
            "Aggregate leaf-kind counts are in schemaModuleLeafKinds above."
        ),
    }

    blob = json.dumps(summary, ensure_ascii=False, indent=1)
    size_bytes = len(blob.encode('utf-8'))
    print(f"summary json size: {size_bytes} bytes", file=sys.stderr)
    if size_bytes > MAX_SUMMARY_BYTES:
        print("WARNING: summary exceeds 2MB cap, trimming listings", file=sys.stderr)
        for k in list(summary['listings'].keys()):
            summary['listings'][k] = summary['listings'][k][:2000]
        summary['listingsTruncatedTo'] = 2000
        blob = json.dumps(summary, ensure_ascii=False, indent=1)
        size_bytes = len(blob.encode('utf-8'))
        print(f"summary json size after trim: {size_bytes} bytes", file=sys.stderr)

    with open(OUT_JSON, 'w', encoding='utf-8') as f:
        f.write(blob)
    print(f"wrote {OUT_JSON}", file=sys.stderr)

    return summary, entries, counts_by_bucket, counts_by_tag, counts_by_flag


if __name__ == '__main__':
    main()
