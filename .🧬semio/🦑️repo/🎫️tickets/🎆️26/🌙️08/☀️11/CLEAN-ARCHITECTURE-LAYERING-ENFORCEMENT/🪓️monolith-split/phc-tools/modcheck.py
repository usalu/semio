import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/"
ws=wsm.WS()
m=carve.Model(ws, {'crate': H+"📦️packages/🦀️rust/🦀️.rs"})
print('files', len(m.by_file), 'missing', [x.path for x in m.missing])
