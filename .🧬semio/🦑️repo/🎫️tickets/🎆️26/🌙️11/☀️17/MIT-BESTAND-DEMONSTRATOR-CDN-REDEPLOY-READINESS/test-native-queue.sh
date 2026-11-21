#!/usr/bin/env bash
set -eu
T="$(cd "$(dirname "$0")" && pwd)"
D="$T/🗑️generated/queue-fixture-$$"
mkdir -p "$D/done"
trap 'rm -rf "$D"' EXIT
cat > "$D/driver" <<'DRIVER'
#!/usr/bin/env bash
id="$2"
printf '%s\n' "$FORCE" >> "$QUEUE_GENERATED/calls-$id"
[ "$FORCE" = 1 ] || exit 1
case "$id" in
  browser-good|browser-missing) printf '[native-build] 7 unit(s), 1 component(s) staged, 0 failure(s) in 8 s; report /good/r/report.md\n' > "$QUEUE_GENERATED/$id.log" ;;
  browser-nostage) exit 0 ;;
  browser-bad) exit 1 ;;
  browser-accepted) printf '[native-build] 7 unit(s), 1 component(s) staged, 0 failure(s) in 8 s; report /build/r/report.md\n' > "$QUEUE_GENERATED/$id.log"; exit 75 ;;
  browser-stale) printf '[native-build] 7 unit(s), 1 component(s) staged, 0 failure(s) in 8 s; report /build/r/report.md\n[native-build] 3 unit(s), 0 component(s) staged, 1 failure(s) in 4 s; report /retry/r/report.md\n' > "$QUEUE_GENERATED/$id.log"; exit 75 ;;
  browser-false) printf '[native-build] 7 unit(s), 1 component(s) staged, 0 failure(s) in 8 s; report /build/r/report.md\n' > "$QUEUE_GENERATED/$id.log"; exit 2 ;;
esac
DRIVER
chmod +x "$D/driver"
python3 "$T/fixture-release-receipts.py" --shim "$D"
export PATH="$D:$PATH"
export QUEUE_MODULES="$D/modules"
python3 - "$QUEUE_MODULES" <<'RECEIPTS'
import json, sys
from pathlib import Path
for module in ("good", "accepted", "false", "stale", "nostage"):
    directory = Path(sys.argv[1]) / ("🧩️" + module)
    directory.mkdir(parents=True)
    receipt = dict(pluginId=module, schema="semio.plugin.component-receipt/v1", target="wasm32-wasip2", profile="release")
    receipt.update(dict.fromkeys(("sourceTreeDigest","wasmSha256","provenanceSha256","semanticDescriptorSha256"), "a"*64))
    (directory / "🧾️receipt.json").write_text(json.dumps(receipt))
    (directory / "component.core.wasm").write_bytes(b"\x00asm-a")
RECEIPTS
export QUEUE_GENERATED="$D" QUEUE_DRIVER="$D/driver" JOBS=2
touch "$D/done/browser-good"
bash "$T/native-queue.sh" good accepted
[ "$(wc -l < "$D/calls-browser-good")" -eq 1 ]
[ "$(wc -l < "$D/calls-browser-accepted")" -eq 1 ]
[ -f "$D/confirmed/good.json" ] && [ -f "$D/confirmed/accepted.json" ]
if bash "$T/native-queue.sh" good bad false stale missing nostage; then exit 1; fi
[ "$(wc -l < "$D/calls-browser-bad")" -eq 2 ]
[ "$(wc -l < "$D/calls-browser-false")" -eq 2 ]
[ ! -e "$D/confirmed/nostage.json" ] || { echo "prior receipt accepted for a successful process with no staging" >&2; exit 1; }
[ ! -e "$D/confirmed/stale.json" ] || exit 1
[ ! -e "$D/confirmed/false.json" ] && [ ! -e "$D/confirmed/missing.json" ]
ROOT="$(cd "$T/../../../../../../.." && pwd)"
CASES="$ROOT/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"
python3 - "$D" "$CASES" <<'ROSTER'
import json, sys
from pathlib import Path
root, cases = Path(sys.argv[1]), json.loads(Path(sys.argv[2]).read_bytes())
receipt = json.loads((root / "confirmed/good.json").read_bytes())
inventory = receipt.pop("optimizedBrowserCores")
for group, part in (("pluginIds", "🔌️plugin-modules"), ("extensionIds", "🧩️extension-modules")):
    for module in cases[group]:
        data = json.dumps(dict(receipt, pluginId=module)).encode()
        (root / "confirmed" / (module + ".json")).write_text(json.dumps(dict(receipt, pluginId=module, optimizedBrowserCores=inventory)))
        for folder in (root / "modules", root / "site" / part):
            directory = folder / ("🧩️" + module)
            directory.mkdir(parents=True)
            (directory / "🧾️receipt.json").write_bytes(data)
            (directory / "component.core.wasm").write_bytes(b"\x00asm-a")
            (directory / "🌉️bridge.js").write_bytes(b"bridgeA")
            script = "export { poll } from '../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js';"
            if part == "🧩️extension-modules" and folder != root / "modules":
                script = script.replace("../🪞️vendor", "../../🔌️plugin-modules/🪞️vendor")
            (directory / "guest.js").write_text(script)
        marker = {"files": ["🧾️receipt.json", "🌉️bridge.js", "component.core.wasm", "guest.js"]}
        (root / "modules" / ("🧩️" + module) / ".semio-artifact.json").write_text(json.dumps(marker))
ROSTER
verify() {
  python3 - "$T/release-receipts.py" verify "$D/modules" "$D/confirmed" --site "$D/site" <<'VERIFY'
import importlib.util, sys
spec = importlib.util.spec_from_file_location("release_receipts", sys.argv[1])
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)
gate.authored_release_digests = lambda modules: dict.fromkeys(modules, "a"*64)
sys.argv = sys.argv[1:]
try:
    gate.main()
except (OSError, ValueError, KeyError) as error:
    raise SystemExit(str(error))
VERIFY
}
verify
python3 - "$D" <<'STALE_AUTHORED'
import json, sys
from pathlib import Path
root = Path(sys.argv[1])
for path in (root / "confirmed/energy.json", root / "modules/🧩️energy/🧾️receipt.json", root / "site/🔌️plugin-modules/🧩️energy/🧾️receipt.json"):
    data = json.loads(path.read_bytes())
    original = data["semanticDescriptorSha256"]
    path.write_text(json.dumps({**data, "semanticDescriptorSha256": ("b" if original != "b"*64 else "c")*64}))
    path.with_suffix(".original").write_text(original)
STALE_AUTHORED
if verify; then echo 'stale authored semantic descriptor accepted' >&2; exit 1; fi
python3 - "$D" <<'RESTORE_AUTHORED'
import json, sys
from pathlib import Path
root = Path(sys.argv[1])
for path in (root / "confirmed/energy.json", root / "modules/🧩️energy/🧾️receipt.json", root / "site/🔌️plugin-modules/🧩️energy/🧾️receipt.json"):
    data = json.loads(path.read_bytes())
    original = path.with_suffix(".original")
    path.write_text(json.dumps({**data, "semanticDescriptorSha256": original.read_text()}))
    original.unlink()
RESTORE_AUTHORED
verify
bridge="$D/site/🧩️extension-modules/🧩️flow-extension-text/🌉️bridge.js"
printf bridgeB > "$bridge"
if verify; then echo 'stale extension bridge accepted' >&2; exit 1; fi
cp "$D/modules/🧩️flow-extension-text/🌉️bridge.js" "$bridge"
energy="$D/site/🔌️plugin-modules/🧩️energy/component.core.wasm"
printf '\0asm-b' > "$energy"
if verify; then echo 'stale deployed Wasm accepted' >&2; exit 1; fi
cp "$D/modules/🧩️energy/component.core.wasm" "$energy"
python3 - "$D/site/🔌️plugin-modules/🧩️energy/🧾️receipt.json" <<'STALE'
import json, sys
from pathlib import Path
p = Path(sys.argv[1])
p.write_text(json.dumps(dict(json.loads(p.read_bytes()), sourceTreeDigest="b"*64)))
STALE
if verify; then exit 1; fi
echo 'queue fixture passed' 
