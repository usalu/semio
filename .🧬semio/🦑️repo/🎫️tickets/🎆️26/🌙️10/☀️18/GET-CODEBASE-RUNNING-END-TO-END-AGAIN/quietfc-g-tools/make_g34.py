"""📼️ quietfc-g: writes the g34 recording fixture (the learned classes of `repo --tool=bazel run @semio-tech/print-rs:test` after g34-print1: 13 earlier runs sized at 103 % of the default, 2497 MB quiet, the full 18515 MB = the print class's OWN measured peak (p90 14810 MB of its cold runs, which really peak at 13.6-15.0 GB per product.py) plus the 25 % margin; the sole-run growth that followed took 10239 MB of the 18515 MB with 14335 MB available and kept all 8 jobs). Usage: python3 -I make_g34.py <estimates.json copy> <path>."""
import json, sys

store = json.load(open(sys.argv[1], encoding="utf-8"))
leaf = "run:test:@semio-tech/print-rs:test"
keep = {"run", "run:test", "run:build", leaf, leaf + "~busy", leaf + "~quiet"}
value = {
    "note": "g34-print1 (gate-g, 2026-10-27 10:41): print-rs:test, quiet 2497 MB then 'reservation grown from 2497 MB to 10239 MB of the 18515 MB it asked for' with all 8 jobs kept; real tree peak 14014 MB (g34-print1.json). 18515 is the class's own measured bound (14810 x 1.25), not a bug: cold print runs peak at 13.6-15.0 GB. Snapshot of estimates.json taken at 10:5x, see make_g34.py.",
    "leaf": leaf,
    "args": ["run", "@semio-tech/print-rs:test", "--parallel=8"],
    "totalMegabytes": 32768,
    "availableMegabytes": 14335,
    "store": {"version": 1, "classes": {key: value for key, value in store["classes"].items() if key in keep}},
}
open(sys.argv[2], "w", encoding="utf-8").write(json.dumps(value, ensure_ascii=False, indent=1) + "\n")
