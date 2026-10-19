"""🧮 C6UI row-table splitter for the typegen schema table (`TYPES`, 92 `SchemaMetadata` rows in one const): every row becomes `pub(super) const <NAME>: SchemaMetadata = ...;` in a semantic child module; `TYPES` lists the row consts in the ORIGINAL order, so `validate` and `render_typescript` see the identical sequence.
usage: projsplit.py stage|verify|install"""
import os, re, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = '/Users/ueli/Documents/semio'
REL = '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🦀️.rs'
STAGE = '/Users/ueli/.cache/c7w/stage/proj'
V = '️'
CUTS = ['AbsoluteLayout', 'GridLayout', 'PresenceUpdate', 'SliderProps', 'UiContractViolation']
FIRST_EMOJI = ['📐', '🧱', '🧩', '🎨', '🧮']


def kebab(n):
    return re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '-', n).lower()


def build_groups(rows):
    names = [r['name'] for r in rows]
    starts = [names.index(c) for c in CUTS] + [len(names)]
    out = []
    for i in range(len(CUTS)):
        grp = names[starts[i]:starts[i + 1]]
        folder = 'schema-rows-%s-to-%s' % (kebab(grp[0]), kebab(grp[-1]))
        mod = 'schema_rows_%s_to_%s' % (kebab(grp[0]).replace('-', '_'), kebab(grp[-1]).replace('-', '_'))
        doc = 'Schema rows from `%s` to `%s`: %d versioned wire type rows of the owned semantic UI TypeScript projection, in table order.' % (grp[0], grp[-1], len(grp))
        out.append((FIRST_EMOJI[i], folder, mod, doc, ' '.join(grp)))
    return out


GROUPS = []


def const_name(n):
    return re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', n).upper()


def parse(text):
    ls = text.split('\n')
    start = next(i for i, l in enumerate(ls) if l.startswith('pub const TYPES: &[SchemaMetadata] = &['))
    rows = []
    i = start + 1
    while ls[i] != '];':
        assert ls[i] == '    SchemaMetadata {', (i + 1, ls[i])
        j = i + 1
        while ls[j] != '    },':
            j += 1
        body = ls[i + 1:j]
        name = re.fullmatch(r'        name: "([^"]+)",', body[0]).group(1)
        assert body[1] == '        version: 1,' or re.fullmatch(r'        version: \d+,', body[1]), body[1]
        assert body[2].startswith('        typescript: r####"'), body[2]
        assert body[-1].endswith('"####,'), (name, body[-1][-20:])
        rows.append(dict(name=name, body=body))
        i = j + 1
    return ls, start, i, rows


def stage():
    text = open(os.path.join(HERE, 'orig', 'proj.rs'), encoding='utf-8').read()
    ls, start, end, rows = parse(text)
    byname = {r['name']: r for r in rows}
    GROUPS[:] = build_groups(rows)
    assert len(byname) == len(rows) == 92
    grouped = [n for g in GROUPS for n in g[4].split()]
    assert sorted(grouped) == sorted(byname), (set(grouped) ^ set(byname))
    shutil.rmtree(STAGE, ignore_errors=True)
    os.makedirs(STAGE)
    where = {}
    for emoji, folder, mod, doc, names in GROUPS:
        out = ['//! %s%s %s See [`super`].' % (emoji, V, doc), '', 'use super::*;']
        for n in rows_order(rows, names.split()):
            r = byname[n]
            b = r['body']
            out.append('')
            out.append('pub(super) const %s: SchemaMetadata = SchemaMetadata {' % const_name(n))
            out.append('    ' + b[0].strip())
            out.append('    ' + b[1].strip())
            out.append('    ' + b[2].strip())
            out.extend(b[3:])
            out.append('};')
            where[n] = mod
        d = os.path.join(STAGE, emoji + V + folder)
        os.makedirs(d)
        open(os.path.join(d, '🦀️.rs'), 'w', encoding='utf-8').write('\n'.join(out) + '\n')
    head = ls[:start]
    tail = ls[end + 1:]
    wiring = []
    for emoji, folder, mod, doc, names in GROUPS:
        wiring.append('#[path = "%s%s%s/🦀️.rs"]' % (emoji, V, folder))
        wiring.append('mod %s;' % mod)
        used = sorted(const_name(n) for n in names.split())
        line = 'use %s::{%s};' % (mod, ', '.join(used))
        wiring.extend(wrap_use(mod, used))
        wiring.append('')
    entries = ['    %s::%s,' % (where[r['name']], const_name(r['name'])) for r in rows]
    entries = ['    %s,' % const_name(r['name']) for r in rows]
    types = ['pub const TYPES: &[SchemaMetadata] = &['] + entries + ['];']
    new = head + [''] + wiring + types + tail
    new_text = '\n'.join(new)
    new_text = re.sub(r'\n{3,}', '\n\n', new_text)
    open(os.path.join(STAGE, '🦀️.rs'), 'w', encoding='utf-8').write(new_text)
    print('staged', {g[1]: g[4].count(' ') + 1 for g in GROUPS})


def rows_order(rows, names):
    pos = {r['name']: i for i, r in enumerate(rows)}
    return sorted(names, key=lambda n: pos[n])


def wrap_use(mod, names):
    one = 'use %s::{%s};' % (mod, ', '.join(names))
    if len(one) <= 100:
        return [one]
    out = ['use %s::{' % mod]
    cur = '    '
    for n in names:
        piece = n + ','
        if len(cur) + len(piece) + (1 if cur.strip() else 0) > 100:
            out.append(cur)
            cur = '    '
        cur += (' ' if cur.strip() else '') + piece
    out.append(cur)
    out.append('};')
    return out


def rows_of_new(root_text, files):
    consts = {}
    for t in files:
        for m in re.finditer(r'pub\(super\) const (\w+): SchemaMetadata = SchemaMetadata \{\n    name: "([^"]+)",\n    version: (\d+),\n    typescript: (r####".*?"####),\n\};', t, re.S):
            consts[m.group(1)] = (m.group(2), int(m.group(3)), m.group(4))
    m = re.search(r'pub const TYPES: &\[SchemaMetadata\] = &\[\n(.*?)\];', root_text, re.S)
    order = re.findall(r'^    (\w+),$', m.group(1), re.M)
    return [consts[o] for o in order], len(consts)


def rows_of_old(text):
    out = []
    for m in re.finditer(r'    SchemaMetadata \{\n        name: "([^"]+)",\n        version: (\d+),\n        typescript: (r####".*?"####),\n    \},', text, re.S):
        out.append((m.group(1), int(m.group(2)), m.group(3)))
    return out


def verify(base):
    GROUPS[:] = build_groups(parse(open(os.path.join(HERE, 'orig', 'proj.rs'), encoding='utf-8').read())[3])
    old = open(os.path.join(HERE, 'orig', 'proj.rs'), encoding='utf-8').read()
    root = open(os.path.join(base, '🦀️.rs'), encoding='utf-8').read()
    files = []
    for emoji, folder, mod, doc, names in GROUPS:
        p = os.path.join(base, emoji + V + folder, '🦀️.rs')
        files.append(open(p, encoding='utf-8').read())
    new_rows, nconst = rows_of_new(root, files)
    old_rows = rows_of_old(old)
    ok = new_rows == old_rows and nconst == 92 and len(old_rows) == 92
    print('rows old', len(old_rows), 'new', len(new_rows), 'consts', nconst, 'IDENTICAL' if ok else 'DIFFERENT')
    for t in [root] + files:
        pass
    allf = [os.path.join(base, '🦀️.rs')] + [os.path.join(base, g[0] + V + g[1], '🦀️.rs') for g in GROUPS]
    for f in allf:
        r = subprocess.run(['rustfmt', '--edition', '2021', '--check', '--config', 'skip_children=true', f], capture_output=True, text=True)
        if 'error' in r.stderr and 'parse' in r.stderr.lower():
            print('rustfmt PARSE ERROR', f, r.stderr[:300])
            ok = False
        n = len(open(f, encoding='utf-8').read().split('\n'))
        print('  lines', n, f[-70:])
    rest_old = [l for l in old.split('\n') if l.strip()]
    keep = [l for l in root.split('\n') if l.strip()]
    for must in ('pub fn validate() -> Result<(), String> {', 'pub fn render_typescript() -> String {', 'pub struct SchemaMetadata {', 'use std::collections::HashSet;'):
        assert must in root, must
    print('VERIFY', 'OK' if ok else 'FAIL')
    return ok


def install():
    GROUPS[:] = build_groups(parse(open(os.path.join(HERE, 'orig', 'proj.rs'), encoding='utf-8').read())[3])
    live = open(os.path.join(REPO, REL), 'rb').read()
    orig = open(os.path.join(HERE, 'orig', 'proj.rs'), 'rb').read()
    assert live == orig, 'repo file differs from the recorded original; refusing'
    base = os.path.dirname(os.path.join(REPO, REL))
    for g in GROUPS:
        assert not os.path.exists(os.path.join(base, g[0] + V + g[1])), g[1]
    for g in GROUPS:
        d = os.path.join(base, g[0] + V + g[1])
        os.makedirs(d)
        shutil.copyfile(os.path.join(STAGE, g[0] + V + g[1], '🦀️.rs'), os.path.join(d, '🦀️.rs'))
    tmp = os.path.join(REPO, REL) + '.pfa-tmp'
    shutil.copyfile(os.path.join(STAGE, '🦀️.rs'), tmp)
    os.replace(tmp, os.path.join(REPO, REL))
    print('installed proj')


def uninstall():
    GROUPS[:] = build_groups(parse(open(os.path.join(HERE, 'orig', 'proj.rs'), encoding='utf-8').read())[3])
    base = os.path.dirname(os.path.join(REPO, REL))
    old = [g[0] + V + g[1] for g in OLD_GROUPS]
    for d in old:
        p = os.path.join(base, d)
        if os.path.isdir(p):
            shutil.rmtree(p)
    shutil.copyfile(os.path.join(HERE, 'orig', 'proj.rs'), os.path.join(REPO, REL) + '.pfa-tmp')
    os.replace(os.path.join(REPO, REL) + '.pfa-tmp', os.path.join(REPO, REL))
    print('uninstalled proj')

OLD_GROUPS = [('📐', 'layout-schema-rows'), ('🎨', 'presentation-schema-rows'), ('📜', 'document-protocol-schema-rows'), ('👥', 'presence-schema-rows'), ('🧮', 'form-widget-schema-rows'), ('🌳', 'structure-widget-schema-rows')]

if __name__ == '__main__':
    c = sys.argv[1]
    if c == 'uninstall':
        uninstall()
        sys.exit(0)
    if c == 'stage':
        stage()
    elif c == 'verify':
        sys.exit(0 if verify(STAGE if len(sys.argv) < 3 else os.path.join(REPO, os.path.dirname(REL))) else 1)
    elif c == 'install':
        install()


