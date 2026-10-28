"""📼️ quietfc-g: writes the g13 recording fixture (the learned classes of `repo --tool=bazel run @semio-tech/repo-dashboard-rs:build` when the quiet admit at 2048 MB had to grow, gate-g 2026-10-27 00:28-01:14). Usage: python3 -I make_g13.py <path>. The log (`gate-g-tools/run/g13-dash.log`) names the facts: 6 earlier runs, the ratio clamped at 200 % of the default (8 jobs: 35840 MB), a share-bound wait of 28672 MB, 12-15 GB spare, a real peak of 4.6-4.8 GB; the sample times and durations are the dashboard's (150-240 s of Cargo work), no sample kept an absolute peak because the batch had not run before."""
import json, sys

NOW = 1793047546789
rows = [{"atMs": NOW - 3_600_000 * (index + 1), "ratioBasisPoints": 20000, "activeMilliseconds": 190_000 + 9_000 * index, "peakAtMilliseconds": 120_000 + 4_000 * index} for index in range(6)]
leaf = "run:build:@semio-tech/repo-dashboard-rs:build"
value = {
    "note": "g13-dash (gate-g, 2026-10-27 00:28-01:14): the learned classes of `repo --tool=bazel run @semio-tech/repo-dashboard-rs:build` as the quiet admit at 2048 MB had to grow: 6 earlier runs sized at 200 percent of the 17920 MB default of 8 jobs, so 35840 MB, clamped to 28672 MB by the share bound, waited 45 min with 12-15 GB spare while the run really peaks at 4.6-4.8 GB (the gate verdicts g8-dash 4.5 GB, g12-dash 4.8 GB). Reconstructed from the log facts, see make_g13.py.",
    "leaf": leaf,
    "args": ["run", "@semio-tech/repo-dashboard-rs:build", "--parallel=8"],
    "nowMs": NOW,
    "totalMegabytes": 61440,
    "spareMegabytes": 12119,
    "peakMegabytes": [4600, 4800],
    "store": {"version": 1, "classes": {"run": {"samples": rows}, "run:build": {"samples": rows}, leaf: {"samples": rows}}},
}
open(sys.argv[1], "w", encoding="utf-8").write(json.dumps(value, ensure_ascii=False, indent=1) + "\n")
