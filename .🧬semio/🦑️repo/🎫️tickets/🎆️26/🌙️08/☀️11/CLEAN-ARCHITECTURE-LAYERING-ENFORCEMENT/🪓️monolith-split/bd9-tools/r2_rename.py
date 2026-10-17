#!/usr/bin/env python3
"""Rename leaf-family crates after their family, repo-wide, in one coordinated rewrite.

usage: r2_rename.py RENAMES.json [--apply]   (run from the repo root; RENAMES = [[old, new, leafSlug, newLabel], ...])
"""
import json, os, re, subprocess, sys

RENAMES = [tuple(r) for r in json.load(open([a for a in sys.argv if a.endswith('.json')][0], encoding='utf-8'))]
BOUND_H = r'(?![A-Za-z0-9_-])'
BOUND_U = r'(?![A-Za-z0-9_])'
OLD_MANIFEST = {}
for _row in json.load(open('📋️build-owners.json', encoding='utf-8'))['owners']:
    OLD_MANIFEST[_row['name']] = _row.get('manifest')


def tracked_hits():
    files = set()
    for old, new, _, _ in RENAMES:
        for needle in (old, old.replace('-', '_')):
            r = subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'grep', '-l', '-F', '--', needle, '--', '.', ':!.🧬semio'], capture_output=True, text=True)
            files.update(l for l in r.stdout.split('\n') if l)
    return sorted(files)


def rename_text(text):
    for old, new, _, _ in RENAMES:
        text = re.sub(re.escape(old) + BOUND_H, new, text)
        text = re.sub(re.escape(old.replace('-', '_')) + BOUND_U, new.replace('-', '_'), text)
    return text


def fix_lock(text):
    head, *blocks = re.split(r'\n(?=\[\[package\]\])', text)
    out = []
    changed = False
    for b in blocks:
        nb = rename_text(b)
        if nb != b:
            changed = True

            def sort_deps(m):
                items = sorted(re.findall(r'^ "([^"]+)",?$', m.group(2), re.M))
                return m.group(1) + ''.join(f' "{i}",\n' for i in items) + m.group(3)

            nb = re.sub(r'(dependencies = \[\n)((?: "[^"]+",?\n)+)(\]\n?)', sort_deps, nb)
        out.append(nb)
    if not changed:
        return text
    key = lambda b: re.search(r'name = "([^"]+)"', b).group(1)
    out = [b for _, _, b in sorted(((key(b), i, b) for i, b in enumerate(out)), key=lambda x: (x[0], x[1]))]
    return '\n'.join([head] + out)


def main():
    apply = '--apply' in sys.argv
    files = tracked_hits()
    report = []
    for f in files:
        text = open(f, encoding='utf-8').read()
        if f.endswith('📋️build-owners.json'):
            d = json.loads(text)
            for row in d['owners']:
                for old, new, _, _ in RENAMES:
                    if row.get('name') == old:
                        row['name'] = new
            d['owners'].sort(key=lambda r: r['name'])
            new_text = json.dumps(d, indent=2, ensure_ascii=False) + '\n'
        elif f.endswith('Cargo.lock'):
            new_text = fix_lock(text)
        else:
            new_text = rename_text(text)
            for old, new, leaf, label in RENAMES:
                if f == OLD_MANIFEST.get(old):
                    new_text = re.sub(r'(description = "Component crate of [^:]+: )' + re.escape(leaf) + '"', r'\g<1>' + label + '"', new_text)
        if new_text != text:
            report.append(f)
            if apply:
                tmp = f + '.bd9tmp'
                open(tmp, 'w', encoding='utf-8').write(new_text)
                os.replace(tmp, f)
    if apply:
        for old, new, leaf, label in RENAMES:
            root = os.path.dirname(OLD_MANIFEST[old]) + '/🦀️.rs'
            t = open(root, encoding='utf-8').read()
            n2 = re.sub(r'^(//! \S+ )' + re.escape(leaf) + r'( of `)', r'\g<1>' + label + r'\g<2>', t, count=1)
            if n2 != t:
                open(root, 'w', encoding='utf-8').write(n2)
    print(json.dumps({'files': len(report), 'list': report}, ensure_ascii=False, indent=1))


main()
