from mk import *
write("model", manifest("model", "Surface tiled map model: Web Mercator projection, tile addressing, camera maths, map descriptors, layer state and the map-policy port", ["canvas","ui_styling","semio-framework-value","serde","serde_json","mvt"]))
host = manifest("host", "Surface tiled map host: raster and vector tile scenes, interaction, picking, label declutter and the browser MapSession bridge", ["canvas","ui_styling","semio-framework-value","serde_json","semio-framework-pixels","model","mvt"], devdeps=["image"], wasm=True)
host = host.replace("[target.'cfg(all(target_arch = \"wasm32\", not(target_env = \"p2\")))'.dependencies]\n","[target.'cfg(all(target_arch = \"wasm32\", not(target_env = \"p2\")))'.dependencies]\nsemio-framework-async = { workspace = true }\n")
write("host", host)
