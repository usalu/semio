import json, os, shutil, sys
S = os.path.dirname(os.path.abspath(__file__))
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/H29/"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
CT = "🌐️remote-execution/🚀️offload/🔬️checkout-test/"
manifest = {
 "schema.json": CT + "🧬️schema/🔣️.json", "policy.ts": CT + "📐️policy/🟦️.ts", "replica.ts": CT + "🧭️replica/🟦️.ts", "gate.ts": CT + "🚦️gate/🟦️.ts",
 "client.ts": CT + "🔌️client/🟦️.ts", "worker.ts": CT + "👷️worker/🟦️.ts", "tests.ts": CT + "🧪️tests/🟦️.ts", "vectors.json": CT + "🧪️tests/🔣️.json", "oracle.py": CT + "🧪️tests/🐍️.py",
 "scheduler-tests.ts": CT + "🧪️tests/🌐️scheduler/🟦️.ts", "remote.ts": "🔬️test-file/🌐️remote/🟦️.ts", "remote-tests.ts": "🔬️test-file/🧪️tests/🌐️remote/🟦️.ts",
}
os.makedirs(T + "new", exist_ok=True)
for short, relative in manifest.items():
    shutil.copyfile(S + "/overlay/" + B + relative, T + "new/" + short)
json.dump(manifest, open(T + "new/manifest.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for name in ["apply.py", "land.py", "run.sh", "overlay-plugin.ts", "stage.py"]:
    shutil.copyfile(S + "/" + name, T + name)
print("staged", len(manifest), "files")
