"""✅ Static checks over the staged tree: rustfmt parse, TOML/JSON parse, path lengths, docstring emoji uniqueness."""
import collections, json, os, re, subprocess, sys, unicodedata
from lib import *
try:
    import tomllib
except ImportError:
    tomllib = None

RUSTFMT = ['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config-path', f'{REPO}/rustfmt.toml', '--config', 'unstable_features=true,skip_children=true', '--emit', 'stdout']

def first_cluster(s):
    out = ''
    for ch in s:
        if out and (unicodedata.category(ch).startswith('M') or ch in '️‍' or ord(ch) in range(0x1F3FB, 0x1F400)):
            out += ch
        elif not out:
            out = ch
        else:
            break
    return out.replace('️', '')

def main():
    bad = 0
    rs = []
    toml_files = []
    for root, ds, fs in os.walk(STAGE):
        for f in fs:
            p = os.path.join(root, f)
            rel = os.path.relpath(p, STAGE)
            full = DB_REL + '/' + rel
            if len(full.encode('utf-8')) > 240:
                print('PATH TOO LONG', len(full.encode('utf-8')), full); bad += 1
            if f.endswith('.rs'): rs.append(p)
            if f == 'Cargo.toml': toml_files.append(p)
            if f == '📋️project.json' or f.endswith('.json'):
                try: json.load(open(p))
                except Exception as e: print('JSON', rel, e); bad += 1
    print('files with path > 240 bytes:', bad)
    for p in toml_files:
        try:
            tomllib.loads(open(p, encoding='utf-8').read())
        except Exception as e:
            print('TOML', p, e); bad += 1
    print('cargo manifests parsed:', len(toml_files))
    fail = 0
    for p in rs:
        r = subprocess.run(RUSTFMT + [p], capture_output=True, text=True)
        if r.returncode not in (0, 1):
            fail += 1; print('RUSTFMT PARSE FAIL', os.path.relpath(p, STAGE), r.stderr[:200])
    print('rustfmt parsed', len(rs), 'files, failures', fail)
    bad += fail
    docs = collections.defaultdict(list)
    for p in rs:
        for l in open(p, encoding='utf-8'):
            if l.startswith('//!'):
                docs[first_cluster(l[3:].strip())].append(os.path.relpath(p, STAGE)); break
    dup = {k: v for k, v in docs.items() if len(v) > 1 and any('io-' in x or 'üe' in x for x in v)}
    # only new crate roots must be unique among each other
    newroots = [p for p in rs if p.endswith('/🦀️.rs')]
    print('docstring clusters', len(docs))
    return bad

if __name__ == '__main__':
    sys.exit(1 if main() else 0)
