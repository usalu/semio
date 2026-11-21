"""🧾 Verify paid native preview allocation and retirement against repository-built value owners."""
import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated"
STAGE = OUTPUT / "paid-jpg-preview"
STAGE.mkdir(parents=True, exist_ok=True)
CACHE = Path(sys.argv[1])
subprocess.run(["repo", "tools", "exec", "python3", "--", str(TICKET / "paged-jpeg-stage.py")], check=True)

for name in ("live-jpeg-transforms.rs", "jpg-baseline-proof.rs", "source-trait.rs", "jpg-entropy.rs"):
    text = (OUTPUT / "paged-jpeg-stage" / name).read_text(encoding="utf8")
    if name == "jpg-entropy.rs":
        test_links = '#[cfg(test)]\n#[path="🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'
        if text.count(test_links) != 1:
            raise ValueError("Unexpected entropy test declaration")
        text = text.replace(test_links, "", 1)
    (STAGE / name).write_text(text, encoding="utf8")
for segment in ("preview", "preview-owner", "preview-owner-test"):
    (STAGE / f"jpg-{segment}.rs").write_bytes((TICKET / f"paged-jpg-{segment}.rs").read_bytes())
entry = (TICKET / "paged-jpg-preview-probe.rs").read_text(encoding="utf8")
entry += '''
#[path = "jpg-preview-owner.rs"]
mod owner;
#[path = "jpg-preview-owner-test.rs"]
mod original_owner_tests;
'''
(STAGE / "preview-owner-probe.rs").write_text(entry, encoding="utf8")
exe = STAGE / ("preview-owner.exe" if os.name == "nt" else "preview-owner")
args = ["--edition=2021", "--test", "--crate-name", "jpg_preview_native", "-O", str(STAGE / "preview-owner-probe.rs"), "-o", str(exe)]
for name in ("semio_framework_value", "semio_framework_value_persistent"):
    library = next(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
    args += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
args += [f"-Ldependency={directory}" for directory in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib", ".dll", ".so")})]
response = STAGE / "preview-owner.rsp"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
fixture = OUTPUT / "paged-jpeg-assets" / "large.jpg"
subprocess.run([str(exe), "--nocapture"], env={**os.environ, "SEMIO_PDF_JPEG_PREVIEW_FIXTURE": str(fixture)}, check=True)
