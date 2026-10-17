#!/usr/bin/env python3
"""📥 c4p17 installer: copies the staged tree of one tag into the repo. The parent must still equal its pre-edit snapshot; no child folder may exist.
usage: inst17.py <tag> check|install     |     inst17.py <tag> verify   (live files equal the stage, byte for byte)"""
import os, sys, json, hashlib, shutil
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P
import mkspec as M

REPO = P.REPO
GEN = os.path.join(os.path.dirname(T), '🗑️generated')


def plan(tag):
    stage = os.path.join(P.SCR, 'stage', tag)
    target = os.path.join(REPO, M.FILES[tag])
    base = os.path.dirname(target)
    files = {}
    for dirpath, dirs, names in os.walk(stage):
        for n in names:
            rel = os.path.relpath(os.path.join(dirpath, n), stage)
            files[rel] = (os.path.join(dirpath, n), os.path.join(base, rel))
    return stage, target, files


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def main():
    tag, cmd = sys.argv[1], sys.argv[2]
    stage, target, files = plan(tag)
    orig = os.path.join(T, 'orig', tag + '.rs.txt')
    if cmd == 'verify':
        bad = [rel for rel, (s, d) in files.items() if not os.path.exists(d) or sha(s) != sha(d)]
        print(tag, 'files', len(files), 'differ/missing', len(bad), bad[:5])
        sys.exit(1 if bad else 0)
    assert sha(target) == sha(orig), ('repo file differs from the recorded original; refusing', target)
    for rel, (s, d) in files.items():
        if rel != '🦀️.rs':
            assert not os.path.exists(d), ('child exists', d)
    print(tag, 'check ok', len(files), 'files')
    if cmd != 'install':
        return
    journal = []
    for rel, (s, d) in files.items():
        if rel == '🦀️.rs':
            continue
        os.makedirs(os.path.dirname(d), exist_ok=True)
        shutil.copyfile(s, d)
        journal.append(d)
    tmp = target + '.c4p17-tmp'
    shutil.copyfile(files['🦀️.rs'][0], tmp)
    os.replace(tmp, target)
    journal.append(target)
    jp = os.path.join(GEN, 'c4p17-installed.json')
    cur = json.load(open(jp)) if os.path.exists(jp) else {}
    cur[tag] = [os.path.relpath(p, REPO) for p in journal]
    json.dump(cur, open(jp, 'w'), ensure_ascii=False, indent=1)
    print('installed', tag, len(journal), 'files')


main()
