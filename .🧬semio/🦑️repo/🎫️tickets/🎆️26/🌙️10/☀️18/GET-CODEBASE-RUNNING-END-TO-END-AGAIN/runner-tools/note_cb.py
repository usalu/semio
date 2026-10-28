"""📝 Records the post-G46 budget proof rerun (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
s = t.index("FOR coordinator: LOAD FINDING")
e = t.index("\n", s)
line = ("FOR coordinator: LOAD FINDING CLOSED by G46 cbudget: rerun rn-cb (01:06, load 22-28) `course`, `framework-rs`, `pets-rs` :: test = 3/3 GREEN (course 9.1 s, pets-rs nextest 117.7 s, framework-rs 91.5 s) — the three suites the old 15 s fundamental WALL budget killed at load 40-65 (rn-c4, rn-c9). "
        "Still open and not covered: `ui-tui-core keystrokes_stay_fast_at_fifty_thousand_rows` (in-test wall-clock 60 ms law, 98 ms at load 40; needs a quiet host or a CPU-time measure; no batch).")
t = t[:s] + line + t[e:]
t = t.rstrip("\n") + "\n| rn-cb | course, framework-rs, pets-rs :: test | 01:06 | 1,370 s | 4,490 MB | 0 | GREEN 3/3 (cbudget proof; were RED in rn-c4/rn-c9) |\n"
open(p, "w", encoding="utf8").write(t)
