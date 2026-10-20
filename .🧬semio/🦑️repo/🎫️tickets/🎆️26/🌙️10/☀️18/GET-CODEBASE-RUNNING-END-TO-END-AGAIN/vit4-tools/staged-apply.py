import sys
R="/Users/ueli/Documents/semio/"
def edit(path, old, new, label):
    s=open(R+path,encoding="utf-8").read()
    if new in s: print("already applied:",label); return
    if s.count(old)!=1: print("ANCHOR MISSING:",label); return
    if "--write" in sys.argv: open(R+path,"w",encoding="utf-8").write(s.replace(old,new,1)); print("applied:",label)
    else: print("would apply:",label)
F="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts"
edit(F,'directoryScript=fileURLToPath(new URL("../📁️directory/📜️script.ts",import.meta.url));','directoryScript=()=>fileURLToPath(new URL("../📁️directory/📜️script.ts",import.meta.url));',"fixed storage: directory script path resolved lazily (vite client environments load the module without a file URL)")
edit(F,'Object.freeze([directoryScript,action,','Object.freeze([directoryScript(),action,',"fixed storage: call site")
G="✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧵️gis-inference-bridge/🟦️.ts"
edit(G,'const hostManifest="🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/Cargo.toml";','const hostManifest="🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml";',"gis inference bridge: composition moved under 🧩️component")
H="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🔸️carries-one-mirror-activation/🟦️.tsx"
edit(H,'import.meta.url)), "../🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json")','import.meta.url)), "../../🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json")',"engine react: fixture path after the test split one level deeper")
