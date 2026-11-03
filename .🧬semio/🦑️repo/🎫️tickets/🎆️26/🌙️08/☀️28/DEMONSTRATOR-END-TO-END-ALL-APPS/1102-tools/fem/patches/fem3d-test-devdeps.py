import sys
B="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any"
def edit(path,anchor,add):
    t=open(path).read()
    if t.count(anchor)!=1 or add in t: sys.exit("anchor/dup mismatch "+path)
    open(path,"w").write(t.replace(anchor,anchor+add))
plugin='semio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\n'
edit(B+"/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml","[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\n",plugin)
edit(B+"/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml","[dev-dependencies]\nsemio-framework-value-native-owned = { workspace = true }\n",plugin)
edit(B+"/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml","[dev-dependencies]\n",'semio-framework-os-kernel = { workspace = true, features = ["protocol-laws", "mutation-testing"] }\n')
