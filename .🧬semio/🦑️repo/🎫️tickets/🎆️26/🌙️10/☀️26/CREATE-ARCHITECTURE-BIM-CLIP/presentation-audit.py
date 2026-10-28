"""🔎 Audit immutable presentation fingerprints and generated-media provenance."""

import hashlib
import importlib
import json
import os
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path.cwd()
sys.path.insert(0, str(ROOT))
TICKET = Path(__file__).resolve().parent
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
settings = {"language": "de", "audience": "speaker", "quality": "-ql", "resolution": "640,360", "fps": 4}
first = runtime.presentation_state("bim_information", settings, "de")
time.sleep(2)
second = runtime.presentation_state("bim_information", settings, "de")
changed = [path for path in first["sources"].keys() | second["sources"].keys()
           if first["sources"].get(path) != second["sources"].get(path)]
generated = [path for path in second["sources"] if any(part in path for part in ("/🗑️generated/", "/⚡️cache/", "/.venv/"))]
recent = [(path, datetime.fromtimestamp(Path(path).stat().st_mtime, timezone.utc).isoformat())
          for path in second["sources"] if time.time() - Path(path).stat().st_mtime < 1800]
stale = {}

def sources(value):
    if isinstance(value, list):
        for item in value:
            yield from sources(item)
    elif isinstance(value, dict):
        yield from value.get("sources", {}).items()
        for key, item in value.items():
            if key != "sources":
                yield from sources(item)

for path in (TICKET / "🗑️generated/r").rglob("*.render.json"):
    try:
        mismatches = sorted({source for source, digest in sources(json.loads(path.read_text(encoding="utf-8")))
                             if not Path(source).is_file() or hashlib.sha256(Path(source).read_bytes()).hexdigest() != digest})
        if mismatches:
            stale[path.relative_to(TICKET).as_posix()] = mismatches
    except (OSError, ValueError):
        continue
report = ["# Presentation Fingerprint Audit", "", f"UTC: {datetime.now(timezone.utc).isoformat()}", "",
          f"Two sequential snapshots, two seconds apart, equal: {first == second}.",
          f"Authored fingerprint sources: {len(second['sources'])}.",
          f"Generated/cache paths in presentation source set: {len(generated)}.", "",
          "## Changed Snapshot Inputs", "", *("- " + path for path in changed), "",
          "## Recently Modified Authored Inputs", "", *("- " + path + " — " + timestamp for path, timestamp in recent), "",
          "## Existing Media with Stale Recorded Inputs", ""]
for certificate, mismatches in stale.items():
    report.append("- " + certificate)
    report.extend("  - " + source for source in mismatches)
report.extend(["", "No fingerprint or choreography source was changed by this audit.", ""])
(TICKET / "presentation-fingerprint-audit.md").write_text("\n".join(report), encoding="utf-8")
print(json.dumps({"equal": first == second, "sources": len(second["sources"]), "generated": generated, "changed": changed,
                  "recent": recent, "stale_certificates": stale}, ensure_ascii=False))
