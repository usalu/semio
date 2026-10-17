"""Registry edits of the c4p12 split: split source roots, callers corpus, cad dev script reader, lib-wiring ratchet. usage: regs.py apply|check"""
import json, os, re, sys
R = '/Users/ueli/Documents/semio'
CAD = '✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any'
NORM = '✏️s/🔌️plugins/📕️norm'
ROOTS = {
    'layout-diff': '✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs',
    'picking': CAD + '/✏️editor/⚙️engine/🧲️picking/🦀️.rs',
    'editor': CAD + '/✏️editor/🦀️.rs',
    'inferences': CAD + '/🧬️schema/💡️inferences/🦀️.rs',
    'compliance': NORM + '/⚖️compliance/🦀️.rs',
    'app-surface': NORM + '/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs',
    'en1990': NORM + '/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs',
}
BUDGET = 150
SRC = R + '/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts'
RATCHET = R + '/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json'
CALLERS = R + '/🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json'
DEV = R + '/✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/📜️script.ts'
EDITOR_CHILD = CAD + '/✏️editor/🪟️editor-and-interaction-scope/🦀️.rs'


def rd(p):
    return open(p, encoding='utf-8').read()


def wr(p, t):
    tmp = p + '.c4p12-tmp'
    open(tmp, 'w', encoding='utf-8').write(t)
    os.replace(tmp, p)


def edit(path, old, new):
    t = rd(path)
    assert t.count(old) == 1, (path[-60:], old[:80], t.count(old))
    wr(path, t.replace(old, new))


def apply():
    t = rd(SRC)
    anchor = '  "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n];'
    assert t.count(anchor) == 1
    add = ''.join('  "%s",\n' % ROOTS[k] for k in ('editor', 'app-surface') if ('"%s"' % ROOTS[k]) not in t)
    wr(SRC, t.replace(anchor, anchor[:-2] + add + '];'))
    edit(CALLERS, '"source": "%s"' % ROOTS['editor'], '"source": "%s"' % EDITOR_CHILD)
    t = rd(DEV)
    assert 'const owner = await Bun.file(ownerPath).text();' in t
    t = t.replace('const owner = await Bun.file(ownerPath).text();', 'const owner = readModuleTreeSource(ownerPath);')
    imp = 'import { runRepoScriptMain }'
    assert t.count(imp) == 1
    t = t.replace(imp, 'import { readModuleTreeSource } from "../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";\n' + imp)
    wr(DEV, t)
    rt = rd(RATCHET)
    for k, p in ROOTS.items():
        n = len(rd(R + '/' + p).split('\n'))
        pat = re.compile(r'^(\s*)"%s": \d+(,?)\n' % re.escape(p), re.M)
        m = pat.search(rt)
        assert m, p
        if n <= BUDGET:
            rt = pat.sub('', rt, count=1)
            print('ratchet removed', k, n)
        else:
            rt = pat.sub(lambda m: '%s"%s": %d%s\n' % (m.group(1), p, n, m.group(2)), rt, count=1)
            print('ratchet lowered', k, n)
    rt = re.sub(r',(\n\s*\})', r'\1', rt)
    wr(RATCHET, rt)
    json.loads(rt)


if sys.argv[1] == 'apply':
    apply()
