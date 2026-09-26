#!/usr/bin/env python3
"""[DEBUG] wg8 temporary: time each step of the native kernel's create_app (reverted after measuring). Usage: apply | revert"""
import pathlib, sys
P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
PAIRS = [
    ("            let component = read_native_component(&wasm_path).await?;\n            let bytes = component.as_slice();\n",
     "            let wg8_started = std::time::Instant::now();\n            let component = read_native_component(&wasm_path).await?;\n            let bytes = component.as_slice();\n            eprintln!(\"[DEBUG] wg8 create_app read {:?}\", wg8_started.elapsed());\n"),
    ("            let compiled = self.compile_serving_others(package_ref.clone(), bytes.to_vec()).await?;\n",
     "            let compiled = self.compile_serving_others(package_ref.clone(), bytes.to_vec()).await?;\n            eprintln!(\"[DEBUG] wg8 create_app compiled {:?}\", wg8_started.elapsed());\n"),
    ("            let instance_id = self.next_instance_id;\n            self.next_instance_id += 1;\n            let plugin_ordinal = self.plugin_ordinal(&plugin_id);\n",
     "            eprintln!(\"[DEBUG] wg8 create_app codec {:?}\", wg8_started.elapsed());\n            let instance_id = self.next_instance_id;\n            self.next_instance_id += 1;\n            let plugin_ordinal = self.plugin_ordinal(&plugin_id);\n"),
    ("            self.replay_routes[replay_route_index] = Some(MountedReplayRouteSeed { actor, plugin: plugin_digest, package: hash.0, window: u64::from(instance_id), artifact: artifact_digest });\n",
     "            eprintln!(\"[DEBUG] wg8 create_app activated {:?}\", wg8_started.elapsed());\n            self.replay_routes[replay_route_index] = Some(MountedReplayRouteSeed { actor, plugin: plugin_digest, package: hash.0, window: u64::from(instance_id), artifact: artifact_digest });\n"),
    ("            self.run_turn(actor, instance_id, first_turn).await?;\n",
     "            self.run_turn(actor, instance_id, first_turn).await?;\n            eprintln!(\"[DEBUG] wg8 create_app first turn {:?}\", wg8_started.elapsed());\n"),
]
text = P.read_text()
for old, new in PAIRS:
    a, b = (old, new) if sys.argv[1] == "apply" else (new, old)
    assert text.count(a) == 1, a[:80]
    text = text.replace(a, b)
P.write_text(text)
print(sys.argv[1], "ok")
