"""🧬 Count physically captured archive invalidations before selecting a direct WASI replay."""
import json
import re
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
receipt = json.loads((generated / "compiler-retry/w-component-84590.invocation.json").read_text())

def root(path):
    dump = subprocess.check_output([receipt["compiler"], "-Zls=root", str(path)], encoding="utf-8")
    return dict(re.findall(r'^\d+ (\w+)-(\w+) hash ', dump, re.M))

def entries(arguments):
    for flag, value in zip(arguments, arguments[1:]):
        if flag == "-L" and value.startswith("dependency="):
            yield Path(value.split("=", 1)[1])

paths = {}
for directory in entries(receipt["arguments"]):
    for file in directory.glob("lib*.rmeta"):
        paths[file.name] = file

hub = next(Path(arg.split("=", 1)[1]) for arg in receipt["arguments"] if arg.startswith("semio_hub_bim=") and arg.endswith(".rmeta"))
dependencies = root(hub)
critical = {
    "semio_s_artifact_bim_model_" + package
    for package in ("curtain_walls", "components", "inferences", "gbxml", "io", "bim")
}
directly_invalid = set(critical)
archive_bytes = 0
invalidated = {}
for name, fingerprint in dependencies.items():
    if not (name.startswith("semio_s_artifact_bim_model_") or name.startswith("semio_hub_bim")):
        continue
    path = paths.get(f"lib{name}-{fingerprint}.rmeta")
    if path is None:
        continue
    stale_by = critical.intersection(root(path))
    if name in critical or stale_by:
        linked = path.with_suffix(".rlib")
        invalidated[name] = (stale_by, linked.stat().st_size if linked.exists() else None, path)
        if linked.exists():
            archive_bytes += linked.stat().st_size
print(f"[DEBUG] Old reactor leaf depends on `{hub.name}`")
print(f"[DEBUG] Existing leaf metadata has {len(dependencies)} transitive archive identities")
print(f"[DEBUG] A lower bound is {len(invalidated)} changed BIM archives for the mounted gbXML producer alone")
print(f"[DEBUG] The old archives in that lower bound total {archive_bytes / 2**30:.2f} GiB; the hub archive and reactor are additional")
for name, (reasons, size, path) in sorted(invalidated.items()):
    print(f"{name} {'absent' if size is None else size // (1 << 20)} MiB {','.join(sorted(reasons))} {path.name}")
print(f"[DEBUG] Hub bytes: {hub.with_suffix('.rlib').stat().st_size}" if hub.with_suffix('.rlib').exists() else "[DEBUG] Hub archive is absent")
