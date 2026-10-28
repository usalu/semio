"""📼️ quietfc-g: writes the g29 recording fixture (the learned classes of `repo --tool=bazel run @semio-tech/print-rs:test` as they were on disk when gate-g's g29-print starved at 09:0x on 2026-10-27: 4 earlier runs sized at 104 % of the default, `run` carrying the pooled absolute peaks of other products (up to 14866 MB) that bounded the print class at 18585 MB, the quiet reservation 4096 MB, 9-14 GB spare on a 32 GB host). Usage: python3 -I make_g29.py <estimates.json copy> <path>; drops the classes that no print lineage reads."""
import json, sys

store = json.load(open(sys.argv[1], encoding="utf-8"))
leaf = "run:test:@semio-tech/print-rs:test"
keep = {"run", "run:test", "run:build", leaf, leaf + "~busy", leaf + "~quiet", "run:build:@semio-tech/repo-dashboard-rs:build", "run:build:@teaching/architecture-course:build"}
classes = {key: value for key, value in store["classes"].items() if key in keep}
value = {
    "note": "g29-print (gate-g, 2026-10-27 09:0x): print-rs:test starved 1296+ s in 'waiting to grow the reservation from 4096 MB to 18585 MB' with 9-14 GB spare on a 32 GB host (real peak 9.8-11.0 GB). 18585 = the 90th percentile (14866 MB, the dashboard run) of the POOLED peaks of the class `run`, borrowed because print's own classes kept none, plus the 25 % margin. Snapshot of ~/Library/Caches/semio/host-capacity/estimates.json taken after the revert, see make_g29.py.",
    "leaf": leaf,
    "args": ["run", "@semio-tech/print-rs:test", "--parallel=8"],
    "totalMegabytes": 32768,
    "spareMegabytes": 10456,
    "store": {"version": 1, "classes": classes},
}
open(sys.argv[2], "w", encoding="utf-8").write(json.dumps(value, ensure_ascii=False, indent=1) + "\n")
