"""📚 Preserve complete terminal validation envelopes before generated-output cleanup."""
import argparse
import hashlib
import importlib.metadata
import importlib.util
import json
from pathlib import Path
import platform
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
parser = argparse.ArgumentParser()
parser.add_argument("name")
parser.add_argument("--tests", type=int)
args = parser.parse_args()
if not args.name.replace("-", "").isalnum():
    raise ValueError("Invalid envelope name")
folder = TICKET / "🗑️generated" / args.name
record = json.loads((folder / "result.json").read_text(encoding="utf-8"))
spec = importlib.util.spec_from_file_location("export_provenance", TICKET / "scene-provenance.py")
provenance = importlib.util.module_from_spec(spec)
spec.loader.exec_module(provenance)
source = record.get("source_start", {})
paths = [Path(key.removeprefix("@external:")) if key.startswith("@external:") else ROOT / key for key in source]
current = bool(source) and provenance.fingerprints(ROOT, paths) == source
runtime = record.get("runtime_start", {})
runtime_current = runtime.get("python") == sys.version and runtime.get("executable") == sys.executable and runtime.get("platform") == platform.platform() and bool(runtime.get("packages")) and all(importlib.metadata.version(name) == value for name, value in runtime.get("packages", {}).items())
accepted = record.get("success") is True and record.get("unchanged") is True and record.get("failures") == 0 and record.get("errors") == 0 and source == record.get("source_end") and runtime == record.get("runtime_end") and current and runtime_current
if args.tests is not None and (not accepted or record.get("tests") != args.tests):
    raise AssertionError("Terminal envelope is not a current accepted result")
rows = ["# " + args.name, "", "This preserves the terminal envelope and complete declared source/runtime maps. Runtime currentness compares the actual interpreter, platform and package versions; recorded start/end rendering configuration equality remains in the envelope. It does not claim that this export process uses the original output-directory configuration.", "", "```json", json.dumps({"accepted": accepted, "current_sources": current, "current_runtime_identity": runtime_current}, indent=2), "```", ""]
for name in ("start.json", "result.json", "checks.jsonl", "workers.json"):
    path = folder / name
    if path.is_file():
        data = path.read_bytes()
        rows.extend(("## " + name, "", "SHA256: " + hashlib.sha256(data).hexdigest(), "", "```json", data.decode("utf-8").rstrip(), "```", ""))
(TICKET / (args.name + "-proof.md")).write_text("\n".join(rows), encoding="utf-8")
print("[DEBUG] Exported terminal envelope " + args.name + ": accepted=" + str(accepted))
