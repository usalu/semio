import sys, re
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
R = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
p = R + '🧪️tests/🔬️unit/🦀️.rs'
s, counts, roots = wrapper_port(p, 'RasterAppFixture', 'RasterApp')
s = read(p); s = std_drop(s, 'RasterAppFixture')[0]; s, n = ctor_fix(s); write(p, s)
print('unit', counts, roots, n)
p = R + '🧪️tests/🔬️unit/🏔️mounted-boot-laws/🦀️.rs'
s, counts, roots = wrapper_port(p, 'MountedRasterApp', 'VcsArtifactApp<EditorApp<RasterPlayApp>>')
s = read(p); s = std_drop(s, 'MountedRasterApp')[0]; s, n = ctor_fix(s); write(p, s)
print('mounted', counts, roots, n)
