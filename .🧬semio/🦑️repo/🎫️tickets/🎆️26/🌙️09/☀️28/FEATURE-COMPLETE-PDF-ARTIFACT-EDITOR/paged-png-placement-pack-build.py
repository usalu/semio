"""📦 Link the native PNG Pack/typed history law against one active repository cache."""
import json
import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
CACHE = Path(sys.argv[1])
GENERATED = TICKET / "🗑️generated"
STAGE = GENERATED / "png-placement"
STAGE.mkdir(parents=True, exist_ok=True)
LIBRARIES = (
    "pack", "semio_framework_artifact_reference", "semio_framework_hash",
    "semio_framework_pack_source", "semio_framework_value", "semio_os_spr",
    "semio_os_vcs", "semio_s_artifact_stdio_pdf_mut_family_8",
)
available = {library.name[3:-5]: library for library in CACHE.glob("*/*/out/lib*.rlib")}
roots = ("pack", "semio_s_artifact_stdio_pdf_mut_family_8")
dependencies = set()
for name in roots:
    root = next((file for key, file in available.items() if key.startswith(name + "-")), None)
    if root is None:
        raise SystemExit(f"[DEBUG] Existing cache has no linked {name} result")
    declaration = root.with_suffix(".rmeta.semio-deps")
    if declaration.is_file():
        dependencies.update(name for name, _ in json.loads(declaration.read_text(encoding="utf8"))["dependencies"])
target = STAGE / ("pack-probe.exe" if os.name == "nt" else "pack-probe")
arguments = ["--edition=2021", "-C", "opt-level=1", "-o", str(target), str(TICKET / "paged-png-placement-probe.rs")]
for name in LIBRARIES:
    candidates = [path for key, path in available.items() if key.startswith(name + "-")]
    selected = [path for path in candidates if path.name[3:-5] in dependencies] or candidates
    if len(selected) != 1:
        raise SystemExit(f"[DEBUG] Existing cache cannot select {name}: {selected}")
    path = selected[0]
    arguments.extend(["--extern", f"{name}={path.with_suffix('.rmeta')}", "--extern", f"{name}={path}"])
folders = {file.parent for file in CACHE.glob("*/*/out/*") if file.suffix in (".rmeta", ".rlib", ".dylib", ".dll", ".so")}
arguments.extend(f"-Ldependency={folder}" for folder in sorted(folders))
response = STAGE / "pack.args"
response.write_text("\n".join(arguments) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
subprocess.run([str(target), str(GENERATED / "paged-png"), str(STAGE)], check=True)
