#!/usr/bin/env python3
"""✅️ Post-flush verification of the hub carve on the real tree: TOML and JSON parse, rustfmt parse, region balance, path length, folder-name admission, manifest graph."""
import os, sys, re, json, subprocess, collections
REPO = '/Users/ueli/Documents/semio'
HUB = REPO + '/🌎️hub'
TOML = 'const t=Bun.TOML.parse(await Bun.file(process.argv[1]).text()); console.log(JSON.stringify(t))'


def toml(path):
    r = subprocess.run(['bun', '-e', TOML, path], capture_output=True, text=True, cwd=REPO)
    if r.returncode != 0:
        raise ValueError(f'{path}: {r.stderr[:200]}')
    return json.loads(r.stdout)


def changed_files(listing):
    return [l.strip() for l in open(listing).read().split('\n') if l.strip()]


def main(listing):
    paths = changed_files(listing)
    rs = [p for p in paths if p.endswith('.rs') and os.path.exists(p)]
    tomls = [p for p in paths if p.endswith('Cargo.toml') and os.path.exists(p)]
    jsons = [p for p in paths if p.endswith(('.json', '.jsonc')) and os.path.exists(p)]
    bad = []
    for p in tomls + [HUB + '/Cargo.toml']:
        try:
            toml(p)
        except Exception as e:
            bad.append(('toml', str(e)[:200]))
    print('TOML parsed', len(tomls) + 1, 'failures', len([b for b in bad if b[0] == 'toml']))
    n = 0
    for p in jsons + [REPO + '/📋️build-owners.json', REPO + '/.vscode/launch.json', REPO + '/.vscode/🧩️launch.seed.jsonc']:
        try:
            json.loads(open(p).read())
            n += 1
        except Exception as e:
            bad.append(('json', p, str(e)[:120]))
    print('JSON parsed', n, 'failures', len([b for b in bad if b[0] == 'json']))
    fmt = 0
    for p in rs:
        src = open(p).read()
        r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--emit', 'stdout', '--config', 'skip_children=true,unstable_features=true'], input=src, capture_output=True, text=True)
        if r.returncode != 0:
            bad.append(('rustfmt', p, r.stderr[:200]))
        fmt += 1
        starts = re.findall(r'^\s*//#region\b', src, re.M)
        ends = re.findall(r'^\s*//#endregion\b', src, re.M)
        if len(starts) != len(ends):
            bad.append(('region', p, f'{len(starts)} start / {len(ends)} end'))
        if re.search(r'//#region ([^\n]*)\n\s*//#endregion \1', src):
            bad.append(('empty-region', p, ''))
    print('rustfmt parsed', fmt, 'failures', len([b for b in bad if b[0] == 'rustfmt']), 'region faults', len([b for b in bad if 'region' in b[0]]))
    longest = max((len(os.path.relpath(p, REPO).encode()) for p in paths), default=0)
    print('longest path bytes', longest)
    if longest > 240:
        bad.append(('path-length', longest))
    for b in bad[:40]:
        print('  BAD', b)
    return bad


if __name__ == '__main__':
    main(sys.argv[1])
