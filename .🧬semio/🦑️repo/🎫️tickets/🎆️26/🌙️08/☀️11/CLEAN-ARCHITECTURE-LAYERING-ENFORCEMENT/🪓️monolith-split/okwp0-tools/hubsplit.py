# 🧭️ OS-KERNEL-WP0 mount-hub split of the kernel crate root (K/🦀️.rs).
# usage: hubsplit.py dry|apply   (dry prints the unit table and verifies losslessness; apply writes the hub files and rewrites the root)
# Every unit is cut by unique line-prefix anchors, appended to the hub of the work package that owns its crate, and its path literals are rebased by "../../".
import os, re, sys
R = '/Users/ueli/Documents/semio'
K = R + '/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
ROOT_RS = K + '/🦀️.rs'
HUBDIR = K + '/📥️mounts'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
HUBS = {
    1: ('🌱️wp1-leaves', '🌱️', 'WP1 non-store leaves and hubs: os_dsl, os_pack, list_delta, os_spr, os_plugin_module_schema, os_semio, os_extension, native_catalog'),
    8: ('🧪️wp8-tests', '🧪️', 'WP8 root-mounted tests of the facade'),
}
# 🚫️ The `io` and `os_store` mounts stay in the kernel root: their modules define `#[macro_export]` macros, and a module declared inside an `include!`d file is macro-expanded, so `crate::<macro>!` paths would be refused (differential, test configuration: 10 `macro_expanded_macro_exports_accessed_by_absolute_paths`).
# (hub, first-line prefix, last-line spec, lines of context before the first anchor); last-line spec: 'braced' = first line equal to "}" after the start, else a prefix
UNITS = [
    (1, 'pub mod os_dsl {', 'braced', 1),
    (1, 'pub mod os_pack {', 'braced', 1),
    (1, '/// 🪡️ Generic positional list deltas', 'pub mod list_delta;', 0),
    (1, 'pub mod os_spr {', 'braced', 1),
    (1, '// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C1', 'pub mod os_plugin_module_schema;', 0),
    (1, '#[path = "../../🔨️modules/🧬️semio/🦀️.rs"]', 'pub mod os_semio;', 0),
    (1, '// 🧩️ `.sxt` extension package pack/unpack/verify is host-only', 'pub mod os_extension;', 0),
    (1, '#[path="../../🔨️modules/🔌️plugin/📇️registry/📥️runtime/🦀️.rs"]', 'pub mod native_catalog;', 0),
    (8, '//#region 🧪️Tests', '//#endregion 🧪️Tests', 0),
    (8, '#[path="../../../../🔨️modules/🌱️value/🔁️codec/🧪️tests/♻️retirement/🦀️.rs"]', 'mod canonical_intrinsic_retirement_tests;', 1),
    (8, '#[path = "../../🔨️modules/📡️spr/🧵️channel/🧪️tests/📤️return-content/🦀️.rs"]', 'mod channel_return_content_tests;', 1),
    (8, '#[path = "../../🔨️modules/🌿️vcs/🧪️tests/🧪️branch-provenance/🦀️.rs"]', 'mod vcs_branch_provenance_tests;', 1),
]
PATHLIT = re.compile(r'(#\[\s*path\s*=\s*"|include!\(\s*")([^"]+)"')

def cut(lines, spec):
    hub, first, last, pre = spec
    a = [i for i, l in enumerate(lines) if l.startswith(first)]
    assert len(a) == 1, f'anchor not unique or missing: {first!r} ({len(a)})'
    a = a[0]
    if last == 'braced': b = next(i for i in range(a + 1, len(lines)) if lines[i] == '}')
    else:
        bs = [i for i in range(a, len(lines)) if lines[i].startswith(last)]
        assert bs, f'end anchor missing: {last!r}'
        b = bs[0]
    return hub, a - pre, b

def rebase_block(text):
    def sub(m):
        path = m.group(2)
        assert not path.startswith('/'), path
        if path == '.': return m.group(0)
        return m.group(1) + '../../' + path + '"'
    return PATHLIT.sub(sub, text)

def banner(n):
    folder, emoji, theme = HUBS[n]
    return [f'// {emoji} HUB WP{n}: mount hub of {theme}.',
            '// This file is spliced into the crate root by `include!` from the kernel root, so every item below stays a crate-root item;',
            '// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the kernel package folder.',
            '// Carving a crate removes its unit here and puts the facade `pub use` lines of the crate in its place (PLAN-OS-KERNEL-CRATES.md, section 10.2).']

def forward(root_text):
    lines = root_text.split('\n')
    cuts = sorted((cut(lines, u) + (u,) for u in UNITS), key=lambda c: c[1])
    for (_, s1, e1, _), (_, s2, e2, sp) in zip(cuts, cuts[1:]): assert e1 < s2, f'overlap at {sp[1]!r}'
    hubs = {n: [] for n in HUBS}
    for hub, s, e, spec in cuts: hubs[hub].append(lines[s:e + 1])
    out_hubs = {}
    for n in HUBS:
        body = []
        for block in hubs[n]:
            body += [rebase_block(l) for l in block] + ['']
        out_hubs[n] = '\n'.join(banner(n) + [''] + body).rstrip('\n') + '\n'
    keep = []
    hub_at = {}
    for hub, s, e, spec in cuts: hub_at[s] = (hub, e)
    i = 0
    seen_hub_include = set()
    while i < len(lines):
        if i in hub_at:
            hub, e = hub_at[i]
            if hub not in seen_hub_include:
                keep.append(f'include!("📥️mounts/{HUBS[hub][0]}/🦀️.rs");')
                seen_hub_include.add(hub)
            i = e + 1
            continue
        keep.append(lines[i]); i += 1
    return '\n'.join(keep), out_hubs

def main():
    mode = sys.argv[1]
    text = open(ROOT_RS, encoding='utf-8').read()
    if 'include!("📥️mounts/' in text:
        print('root already hubbed'); return
    new_root, hubs = forward(text)
    orig = [l for l in text.split('\n') if l.strip()]
    got = [l for l in new_root.split('\n') if l.strip()]
    for n, h in hubs.items(): got += [l for l in h.split('\n') if l.strip() and not l.startswith('// ' + HUBS[n][1] + ' HUB') and not l.startswith('// This file') and not l.startswith('// its `#[path]`') and not l.startswith('// Carving a crate')]
    def norm(l): return l.replace('"../../', '"')
    from collections import Counter
    a = Counter(orig); b = Counter(l for l in [])
    hubset = set(); 
    for n, h in hubs.items(): hubset |= set(h.split('\n'))
    b = Counter((norm(l) if l in hubset else l) for l in got if not l.startswith('include!("📥️mounts/'))
    lost = a - b; extra = b - a
    print('units', len(UNITS), 'root lines', len(text.split('\n')), '->', len(new_root.split('\n')), 'lost', sum(lost.values()), 'extra', sum(extra.values()))
    for k, v in list(lost.items())[:10]: print('  lost', v, k[:100])
    for k, v in list(extra.items())[:10]: print('  extra', v, k[:100])
    for n, h in hubs.items(): print('hub', n, HUBS[n][0], len(h.split('\n')), 'lines')
    if mode == 'apply':
        assert not lost, 'lossless check failed'
        for n, h in hubs.items():
            d = HUBDIR + '/' + HUBS[n][0]
            os.makedirs(d, exist_ok=True)
            assert not os.path.exists(d + '/🦀️.rs'), d
            open(d + '/🦀️.rs', 'w', encoding='utf-8').write(h)
        cur = open(ROOT_RS, encoding='utf-8').read()
        assert cur == text, 'root changed while splitting'
        open(ROOT_RS, 'w', encoding='utf-8').write(new_root)
        print('applied')

if __name__ == '__main__': main()
