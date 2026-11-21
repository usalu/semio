"""🧪 Builds and exercises the isolated paged number-array candidate."""

import json
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "paged-number"
OUT.mkdir(parents=True, exist_ok=True)
CACHE = Path(".cache/b/r/m/u/l/pc5147e/prototype/build")
archive = json.loads((TICKET / "archive-cases.json").read_bytes())["largeImage"]
count = archive["width"] * archive["height"]
model = {"mutation": "setImage", "image": {"id": "Gray", "width": archive["width"], "height": archive["height"],
    "colorSpace": {"kind": "deviceGray"}, "bitsPerComponent": 8, "imageMask": False, "interpolate": False,
    "body": {"kind": "samples", "values": [archive["sample"]]}}}
payload = json.dumps(model, separators=(",", ":")).encode()
start = payload.index(b"[0]")
array = b"[0" + b",0" * (count - 1) + b"]"
payload = payload[:start] + array + payload[start + 3:]
assert len(payload) == 8_388_811 and json.loads(payload)["image"]["body"]["values"] == [0] * count
(OUT / "fixture.json").write_bytes(payload)
(OUT / "span.json").write_text(json.dumps({"start": start, "end": start + len(array), "samples": count}), encoding="utf8")

dependencies = {
    name: next(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
    for name in ("semio_framework_value", "semio_framework_pack_json", "semio_framework_value_native_owned",
                 "semio_framework_value_persistent", "semio_framework_dsl_record", "serde_json")
}
source = OUT / "check.rs"
bank = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🧰️bank.rs").resolve()
source.write_text(f'#[path = "{TICKET / "paged-number-array.rs"}"]\nmod array;\n#[path = "{bank}"]\nmod bank;\n' +
                  (TICKET / "paged-number-check.rs").read_text(encoding="utf8"), encoding="utf8")
options = ["--edition=2024", "--test", "-Copt-level=2", str(source), "-o", str(OUT / "check")]
for name, library in dependencies.items():
    options += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
options += [f"-Ldependency={path}" for path in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib")})]
args = OUT / "args"
args.write_text("\n".join(options) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(args)], check=True)
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(OUT / "check"), "--nocapture"], check=True)
