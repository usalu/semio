import glob
from rep import rep
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/'
G=glob.glob(R+'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
rep(G+'/⚙️compute/🦀️.rs',[('#[cfg(any(test, feature = "testkit"))]\npub mod testing','#[cfg(any(test, feature = "funded-turns"))]\npub mod testing')])
rep(G+'/⚙️compute/📦️packages/🦀️rust/Cargo.toml',[('testkit = []','testkit = []\nfunded-turns = []')])
for f in glob.glob(R+'🧪️tests/**/Cargo.toml',recursive=True)+glob.glob(G+'/**/Cargo.toml',recursive=True):
    s=open(f,encoding='utf8').read()
    o='semio-s-artifact-procedural-3d-geometry-compute = { workspace = true, features = ["testkit"] }'
    if o in s: open(f,'w',encoding='utf8').write(s.replace(o,'semio-s-artifact-procedural-3d-geometry-compute = { workspace = true, features = ["funded-turns"] }')); print(f[-70:])
