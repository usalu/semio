"""Check clause lengths of the narration table in storyboard-pareto.md after substituting sample field values."""
import re
import sys
from pathlib import Path

text = (Path(__file__).with_name("storyboard-pareto.md")).read_text(encoding="utf-8")
section = text.split("## 5. Narration", 1)[1].split("## 6.", 1)[0]
sample = {"n": "30", "n_wwr": "5", "n_t": "3", "n_d": "2", "c_L": "22,704 CHF", "e_L": "932 kWh/a", "dc": "4,644 CHF",
          "de": "220 kWh/a", "nf": "6", "dc1": "2,142 CHF", "de1": "116 kWh/a", "r1": "18.5 CHF", "pay1": "12",
          "tariff": "0.25", "life": "30", "nf3": "9", "wwr_r": "57"}
rows = [line.split("|")[1:4] for line in section.splitlines() if line.startswith("| `")]
worst = 0
for key, en, de in rows:
    en, de = (re.sub(r"\{(\w+)\}", lambda m: sample[m.group(1)], value.strip()) for value in (en, de))
    worst = max(worst, len(en), len(de))
    flag = "  <-- LONG" if max(len(en), len(de)) > 95 else ""
    print(f"{key.strip():18s} en={len(en):3d} de={len(de):3d}{flag}")
print("clauses", len(rows), "worst", worst, "chars total en", sum(len(r[1]) for r in rows))
sys.exit(0)
