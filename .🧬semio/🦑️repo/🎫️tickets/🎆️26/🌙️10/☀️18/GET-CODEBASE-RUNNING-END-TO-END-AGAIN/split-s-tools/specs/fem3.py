from specs.lib import *
F = '✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🪜️solve-step/🦀️.rs'
c = Cut(F, 'fem3.rs')
c.impl('impl Fem3dNumericalChild {', [719])
c.child('🍓', 'staged-step', 'staged_step', [(6, 718)], 'Staged step: the numerical child staged solve step across solid mesh, assembly, PCG and the finishing stages.')
c.child('🥦', 'close-ladder', 'close_ladder', [(719, 1064)], 'Close ladder: the numerical child terminal check, close rung and interactive close step.')
SPEC = c.spec(keep_empty=('PageVisualJob',))
