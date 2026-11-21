"""🧾️ Reproduces the PDF sample wire lower bound with the governed native SPR cohort."""

import json
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/raw-raster-spr-proof"
CACHE = Path(sys.argv[1]).resolve()
OUT.mkdir(parents=True, exist_ok=True)

for key, width, height in (("small", 16, 17), ("large", 521, 257)):
    samples = [(i * 73) % 256 for i in range(width * height * 3)]
    (OUT / f"{key}.json").write_text(json.dumps(samples, separators=(",", ":")), encoding="utf8")

[spr] = CACHE.glob("*/*/out/libsemio_os_spr-b5da2752a63afca5.rlib")
args = ["--test", "--edition=2024", str(TICKET / "raw-raster-spr-proof.rs"), "-o", str(OUT / "probe"),
        "--extern", f"semio_os_spr={spr.with_suffix('.rmeta')}", "--extern", f"semio_os_spr={spr}"]
args += [f"-Ldependency={path}" for path in sorted({
    p.parent for p in CACHE.glob("*/*/out/*") if p.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")
})]
response = OUT / "rustc.args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(OUT / "probe"), "--nocapture"], check=True)
