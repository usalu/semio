"""📊 C4P39 census digest: native / wasm lib-test row (files bytes lines ratio) of the six bins from a census crates.tsv. usage: crows.py <crates.tsv>"""
import csv, sys
rows = list(csv.DictReader(open(sys.argv[1], encoding="utf-8"), delimiter="\t"))
for r in rows:
    out = [r["crate"].replace("semio-framework-os-renderer-wgpu-", "")]
    for lab in ("native", "wasm32-unknown-unknown", "wasm32-wasip2"):
        out.append(f"{lab[:6]} {r[lab + '_libTestFiles']}/{r[lab + '_libTestBytes']}/{r[lab + '_libTestLines']} ({r[lab + '_libTestRatio']})")
    out.append(f"worst {r['native_worstTestRatio']} over {r['native_overTestUnits']}")
    print("  ".join(out))
