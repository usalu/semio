
from pathlib import Path
import json, hashlib

TICKET = next(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))

hits = []
for p in Path(".").rglob("*mimes.csv"):
    s = str(p)
    if any(x in s for x in ["node_modules", "/target/", "/dist/", "fixture", "storybook-static"]):
        continue
    hits.append(p)

canonical = None
for p in hits:
    if "tickets" in str(p):
        continue
    if "assets" in str(p):
        # skip ui assets
        if any(part.endswith("ui") or "ui" == part[-2:] for part in Path(str(p)).parts):
            # crude: path containing mouse-ui module
            pass
        canonical = p
# Prefer non-ui
for p in hits:
    s = str(p)
    if "tickets" in s:
        continue
    if "🖱" in s:
        continue
    if "assets" in s:
        canonical = p

ui_mimes = [p for p in hits if "🖱" in str(p)]
ui_dirs = []
for p in Path(".").rglob("*"):
    if not p.is_dir():
        continue
    if "node_modules" in str(p) or "/dist/" in str(p) or "/target/" in str(p):
        continue
    if p.name.endswith("list") and "assets" in str(p) and "🖱" in str(p):
        ui_dirs.append(p)
ui_candidates = [d / canonical.name for d in ui_dirs]

csv = canonical.read_text(encoding="utf-8")
rows = [l for l in csv.splitlines()[1:] if l.strip()]

proof = {
    "canonical_path": str(canonical),
    "canonical_exists": canonical.exists(),
    "canonical_rows": len(rows),
    "canonical_sha256": hashlib.sha256(canonical.read_bytes()).hexdigest(),
    "canonical_header": csv.splitlines()[0],
    "ui_mimes_hits": [str(p) for p in ui_mimes],
    "ui_candidate_paths": [str(p) for p in ui_candidates],
    "ui_any_exists": any(p.exists() for p in ui_candidates) or bool(ui_mimes),
    "all_mimes_hits": [str(h) for h in hits],
    "cargo": {},
}
for name in ["w7-cargo-recheck.log", "w7-cargo-check.log"]:
    log = (TICKET/"generators"/name).read_text(encoding="utf-8", errors="ignore")
    for line in log.splitlines():
        if line.startswith("EXIT_"):
            k,v = line.split("=",1)
            proof["cargo"][k] = int(v)

assert proof["canonical_exists"], proof
assert proof["canonical_rows"] == 29, proof["canonical_rows"]
assert proof["ui_any_exists"] is False, proof
assert proof["cargo"].get("EXIT_OS_DEFAULT") == 0, proof["cargo"]
assert proof["cargo"].get("EXIT_OS_FULL") == 0, proof["cargo"]
assert proof["cargo"].get("EXIT_SPACE") == 0, proof["cargo"]
assert proof["cargo"].get("EXIT_FRAMEWORK") == 0, proof["cargo"]

(TICKET/"generators"/"w7-final-proof.json").write_text(json.dumps(proof, indent=2) + "
", encoding="utf-8")
print(json.dumps(proof, indent=2))
