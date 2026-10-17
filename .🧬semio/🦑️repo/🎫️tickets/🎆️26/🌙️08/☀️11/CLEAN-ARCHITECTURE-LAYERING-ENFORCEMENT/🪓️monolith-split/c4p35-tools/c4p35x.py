"""🧱 c4p35 custom builders (stage only, pure functions of the pre-edit text).
exp: the three inline modules `analyze`, `codegen`, `parse` of the proc-macro expansion become file modules with the same module path (body dedented by four, the inner `//!` doc is the file doc);
two pre-existing orphan region markers are completed so that every new file is balanced (an `//#endregion 🔖️Entry` without opener in `analyze`, an `//#region 🔖️Entry` without closer in the root).
jtu: test-law fragments spliced back with `include!` (module identity, test paths and `use` scope unchanged; no widening)."""
import re

V = '️'


def dedent(seg, who):
    out = []
    for l in seg:
        if l.strip() == '':
            out.append('')
        else:
            assert l.startswith('    '), (who, 'inner line not indented', l[:60])
            out.append(l[4:])
    return out


def build_exp(orig):
    ls = orig.split('\n')
    if ls and ls[-1] == '':
        ls.pop()
    mods = [('analyze', '🔬' + V + 'analyze'), ('codegen', '🏗' + V + 'codegen'), ('parse', '✂' + V + 'parse')]
    spans = {}
    for name, folder in mods:
        heads = [i for i, l in enumerate(ls) if l == 'mod %s {' % name]
        assert len(heads) == 1, (name, heads)
        h = heads[0]
        e = next(i for i in range(h + 1, len(ls)) if ls[i] == '}')
        spans[name] = (h, e)
    files = {}
    out = []
    i = 0
    order = sorted(spans.items(), key=lambda kv: kv[1][0])
    folder_of = dict(mods)
    for name, (h, e) in order:
        out.extend(ls[i:h])
        body = dedent(ls[h + 1:e], name)
        while body and body[0] == '':
            body.pop(0)
        while body and body[-1] == '':
            body.pop()
        assert body[0].startswith('//! '), (name, body[0][:40])
        if name == 'analyze':
            hits = [k for k, l in enumerate(body) if l == 'pub fn analyze(ast: MachineAst) -> syn::Result<Ir> {']
            assert len(hits) == 1
            assert body[-1] == '//#endregion 🔖️Entry', body[-1]
            body.insert(hits[0], '//#region 🔖️Entry')
        files[folder_of[name]] = '\n'.join(body) + '\n'
        out.extend(['#[path = "%s/🦀️.rs"]' % folder_of[name], 'mod %s;' % name, ''])
        i = e + 1
    out.extend(ls[i:])
    txt = '\n'.join(out) + '\n'
    anchor = '\n//#region 🧪️Tests\n'
    assert txt.count(anchor) == 1
    txt = txt.replace(anchor, '\n//#endregion 🔖️Entry\n' + anchor)
    txt = re.sub(r'\n{3,}', '\n\n', txt)
    return txt, files


def build_include(orig, frags):
    """frags: list of (folder, (a, b), doc) with 1-based inclusive line ranges; the root gets `include!` at the first line of each range."""
    ls = orig.split('\n')
    if ls and ls[-1] == '':
        ls.pop()
    files = {}
    starts = {}
    skip = set()
    for folder, (a, b), doc in frags:
        seg = ls[a - 1:b]
        while seg and seg[0].strip() == '':
            seg = seg[1:]
        while seg and seg[-1].strip() == '':
            seg = seg[:-1]
        for l in seg:
            assert not re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l), ('relative include in a fragment', l[:80])
            assert not re.search(r'#\[\s*path\s*=', l), ('path attribute in a fragment', l[:80])
        files[folder] = '/// %s\n\n' % doc + '\n'.join(seg) + '\n'
        starts[a] = folder
        skip.update(range(a, b + 1))
    out = []
    for n in range(1, len(ls) + 1):
        if n in starts:
            if out and out[-1].strip():
                out.append('')
            out.append('include!("%s/🦀️.rs");' % starts[n])
            out.append('')
        if n in skip:
            continue
        out.append(ls[n - 1])
    txt = re.sub(r'\n{3,}', '\n\n', '\n'.join(out)).rstrip('\n') + '\n'
    return txt, files
