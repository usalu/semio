"""📏️ Emits one job per unit of the named crates for real-closure-probe.mjs, which runs the repository's own readRustSourceClosureV1 on them."""
import importlib.util, json, sys
from pathlib import Path
here = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("audit", here / "source-boundary-audit.py")
audit = importlib.util.module_from_spec(spec); spec.loader.exec_module(audit)
root = here.parents[6] if (here.parents[6] / "AGENTS.md").is_file() else Path.cwd()
wanted = set(sys.argv[1:])
jobs = []
for manifest in audit.collect_manifests(root):
    info = audit.manifest_info(manifest)
    if not info or (wanted and info["name"] not in wanted):
        continue
    for kind, label, entry, test, feats in audit.units_of(info):
        jobs.append({"crate": info["name"], "unit": label, "kind": kind, "test": test, "entry": str(entry), "manifestDirectory": str(info["dir"]), "features": sorted(feats)})
json.dump(jobs, sys.stdout, ensure_ascii=False)
