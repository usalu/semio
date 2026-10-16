import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
R = os.environ['R']; V = '️'
rd = lambda p: open(p, encoding='utf8').read()
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
pr = R + f'/🎨{V}presentation-raster/🦀{V}.rs'; fa = R + f'/🛎{V}frame-actions/🦀{V}.rs'; pb = R + f'/🔬{V}native-socket-probe/🦀{V}.rs'; ar = R + f'/🧠{V}app-runtime/🦀{V}.rs'
t = rd(pr); line = 'pub(crate) const WORLD3D_DEADLINE_ID_BYTES: usize = 256;\n'
assert t.count(line) == 1; w(pr, t.replace(line, ''))
t = rd(fa); anchor = 'pub(crate) const WORLD3D_DEADLINE_CAPACITY: usize = 256;\n'
assert t.count(anchor) == 1; w(fa, t.replace(anchor, anchor + line))
tp = rd(pb); rest, moved = mv.cut(tp, ['thread_local ICON_ATLAS_RUNTIME'])
assert len(moved) == 1
w(pb, rest)
ta = rd(ar)
ta = ta.rstrip('\n') + '\n\n//#region 🖼' + V + 'IconAtlasRuntime\n' + moved[0] + '\n//#endregion 🖼' + V + 'IconAtlasRuntime\n'
w(ar, ta)
print(moved[0])
