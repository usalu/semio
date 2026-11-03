import re
import sys
from pathlib import Path

E = Path("🎓️teaching/🏛️architecture/⚡️energy")
CLIPS = [
    "❄️cooling/🎬️clip/💧️transmission-humidity", "❄️cooling/🎬️clip/👥️internal-gains", "❄️cooling/🎬️clip/🌤️summer-strategy",
    "🌬️ventilation/🎬️clip/🌡️heat-flow", "🌬️ventilation/🎬️clip/📦️volume", "🌬️ventilation/🎬️clip/🤝️handoff", "🌬️ventilation/🎬️clip/❓️need",
    "🔥️heating/🎬️clip/🌬️convection", "🔥️heating/🎬️clip/🧮️calculation", "🔥️heating/🎬️clip/🌡️fundamentals",
    "📊️demand/🎬️clip/🏭️primary-energy", "📊️demand/🎬️clip/🏷️certificate", "📊️demand/🎬️clip/🧪️practice",
    "📊️demand/🎬️clip/♨️system-losses", "📊️demand/🎬️clip/⚖️regulations", "📊️demand/🎬️clip/⚡️demand-power",
]
for c in CLIPS:
    p = E / c / "🐍️.py"
    if not p.exists():
        print("MISSING", c)
        continue
    cur, counts = None, {}
    for line in p.read_text(encoding="utf-8").splitlines():
        m = re.match(r"class (\w+)\(", line)
        if m:
            cur = m.group(1)
            counts[cur] = 0
        elif cur:
            counts[cur] += len(re.findall(r"\btr\(", line))
    print(c, sorted(counts.items(), key=lambda kv: -kv[1])[:3])
