"""Name-uniqueness pre-pass over the staged board tree: dedupe identical items, rename colliding private ones."""
import sys, os, re, shutil
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
B = '🎲️board/'
N = B + '🔌️ports/➡️directed/➕️normal/🦀️.rs'
D = B + '🔌️ports/➡️directed/🕸️dag/🦀️.rs'
E = B + '🦀️.rs'

def edit(root, rel, old, new, count=1):
    p = f'{root}/{rel}'
    t = open(p, encoding='utf-8').read()
    assert t.count(old) == count, (rel, old[:70], t.count(old))
    open(p, 'w', encoding='utf-8').write(t.replace(old, new))

def rename_word(root, rel, old, new):
    p = f'{root}/{rel}'
    t = open(p, encoding='utf-8').read()
    t2 = re.sub(r'(?<![A-Za-z0-9_])' + re.escape(old) + r'(?![A-Za-z0-9_])', new, t)
    assert t2 != t, (rel, old)
    open(p, 'w', encoding='utf-8').write(t2)

def main(src, dst):
    shutil.rmtree(dst, ignore_errors=True)
    shutil.copytree(src, dst, symlinks=True)
    # 1 GraphExtension duplicate in ports/directed (identical to the engine root's)
    edit(dst, B + '🔌️ports/➡️directed/🦀️.rs', "// #region 🔖️GraphExtension\n/// 🧩️ Extension hook for domain-specific graph behavior.\npub trait GraphExtension: canvas::CanvasExtension {}\n\n// #endregion 🔖️GraphExtension\n\n", "")
    # 2 selection constants (private duplicates in normal root)
    edit(dst, N, "const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;\nconst SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;\n", "")
    # 3 grid constants: one pub copy in the engine root
    grid = "const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;\nconst GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;\nconst GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;\nconst GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;\nconst GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;\n"
    edit(dst, N, grid, "")
    edit(dst, D, grid, "")
    edit(dst, E, "pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;\n", "pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;\n" + grid.replace('const ', 'pub const '))
    # 4 buchheim helpers: the layered module owns prefixed copies
    LAY = B + '🧬️schema/💡️inferences/📐️layout/🕸️layered/🦀️.rs'
    for nm in ['BuchheimNode', 'buchheim_left_brother', 'buchheim_leftmost_sibling', 'buchheim_next_right', 'buchheim_next_left', 'buchheim_move_subtree', 'buchheim_execute_shifts', 'buchheim_apportion', 'buchheim_first_walk', 'buchheim_second_walk', 'buchheim_third_walk']:
        rename_word(dst, LAY, nm, ('Layered' + nm) if nm[0].isupper() else ('layered_' + nm))
    # 5 dag-input channel
    DI = B + '🚪️io/📝️text/🎯️dag-input/🦀️.rs'
    rename_word(dst, DI, 'channel', 'input_channel') if False else None
    p = f'{dst}/{DI}'; t = open(p, encoding='utf-8').read()
    t = t.replace('fn channel(value:', 'fn input_channel(value:').replace('control.step()?;channel(row)?;', 'control.step()?;input_channel(row)?;')
    t = re.sub(r'control\.step\(\)\?;channel\(', 'control.step()?;input_channel(', t)
    open(p, 'w', encoding='utf-8').write(t)
    RT = B + '🚪️io/📝️text/🎯️dag-input/🧵️retained/🦀️.rs'
    edit(dst, RT, 'super::channel(row)', 'super::input_channel(row)')
    # 6 undirected Edge
    edit(dst, B + '🔌️ports/↔️undirected/🦀️.rs', "pub type Edge = GraphEdge<HandleId>;", "pub type UndirectedEdge = GraphEdge<HandleId>;")
    unwrap_types(dst)
    hoist_test_helper(dst)
    split_leaf_macros(dst)

def split_leaf_macros(dst):
    p = f'{dst}/{D[:-len("🦀️.rs")]}🧹️retirement/🦀️.rs'
    t = open(p, encoding='utf-8').read()
    def sub(m):
        names = [x.strip() for x in m.group(2).split(',') if x.strip()]
        if len(names) <= 1: return m.group(0)
        return '\n'.join(f'{m.group(1)}({n});' for n in names)
    t2 = re.sub(r'(semio_framework_value::artifact_retire_leaf!)\(([^)]*)\);', sub, t)
    assert t2 != t
    open(p, 'w', encoding='utf-8').write(t2)

def hoist_test_helper(dst):
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from oinf_rs import chunk_file
    sp = f'{SC}/snapshot-board/{N[:-len("🦀️.rs")]}🧪️tests/🔬️board-host-standalone/🦀️.rs'
    tx = open(sp, encoding='utf-8').read()
    items, _ = chunk_file(tx)
    it = [i for i in items if i.name == 'board_property_retirement_credits'][0]
    body = tx[it.full_start:it.full_end]
    p = f'{dst}/{N}'
    t = open(p, encoding='utf-8').read()
    open(p, 'w', encoding='utf-8').write(t.rstrip('\n') + '\n\n' + body.strip('\n') + '\n')

def unwrap_types(dst):
    rel = B + '🔌️ports/➡️directed/🦀️.rs'
    p = f'{dst}/{rel}'
    t = open(p, encoding='utf-8').read()
    a = t.index('pub mod types {\n')
    b = t.index('\n}\n', a) + 1
    body = t[a + len('pub mod types {\n'):b - 0]
    lines = body.rstrip('\n').split('\n')
    out = []
    for l in lines:
        l2 = l[4:] if l.startswith('    ') else l
        if l2.lstrip().startswith('//!'): l2 = l2.replace('//!', '//', 1)
        out.append(l2)
    t = t[:a] + '\n'.join(out) + '\n' + t[b + 2:]
    assert t.count('pub use types::*;\n') == 1
    t = t.replace('pub use types::*;\n', '')
    open(p, 'w', encoding='utf-8').write(t)

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])
    print('ok')
