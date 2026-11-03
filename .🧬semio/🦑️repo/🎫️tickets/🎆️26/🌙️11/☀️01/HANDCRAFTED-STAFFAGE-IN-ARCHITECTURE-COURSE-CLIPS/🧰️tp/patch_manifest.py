"""🧩 tp: retarget cast environments and add the summer device casts of the comparison clip (one atomic rewrite of the manifest)."""
import json
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"
RETARGET = {
    "indoor-warm-summer": [
        "cooling-internal-gains-sleeper", "cooling-internal-gains-seated-worker", "cooling-internal-gains-standing", "cooling-internal-gains-walker",
        "cooling-internal-gains-jogger", "cooling-internal-gains-laptop", "cooling-internal-gains-laptop-closed", "cooling-internal-gains-server-rack",
        "cooling-internal-gains-lamp", "cooling-internal-gains-lamp-dimmed", "optimization-room-cooler", "optimization-extrema-cooler", "cooling-heating-comparison-cooler",
    ],
    "indoor-heated-winter": [
        "heating-internal-gains-office-worker", "heating-internal-gains-laptop", "heating-internal-gains-server-rack", "heating-internal-gains-pendant-lamp-warm-up",
        "heating-internal-gains-pendant-lamp", "heating-conduction-tealight", "heating-conduction-person", "heating-conduction-toaster", "heating-calculation-laptop",
        "heating-calculation-lamp", "physics-fundamentals-condenser",
    ],
    "winter-overcast": ["physics-fundamentals-evaporator"],
}
SUMMER = {
    "cooling-heating-comparison-toaster-summer": "cooling-heating-comparison-toaster",
    "cooling-heating-comparison-laptop-summer": "cooling-heating-comparison-laptop",
    "cooling-heating-comparison-lamp-summer": "cooling-heating-comparison-lamp",
}
lines = PATH.read_text(encoding="utf-8").split("\n")
manifest = json.loads("\n".join(lines))["casts"]
for preset, ids in RETARGET.items():
    for cast in ids:
        old = manifest[cast]["environment"]
        for number, line in enumerate(lines):
            if line.lstrip().startswith(f'"{cast}":'):
                lines[number] = line.replace(f'"environment": "{old}"', f'"environment": "{preset}"')
for new, source in SUMMER.items():
    for number, line in enumerate(lines):
        if line.lstrip().startswith(f'"{source}":') and new not in "\n".join(lines):
            row = line.replace(f'"{source}"', f'"{new}"', 1).replace('"environment": "indoor-heated-winter"', '"environment": "indoor-warm-summer"').rstrip(",") + ","
            lines.insert(number + 1, row)
            break
text = "\n".join(lines)
json.loads(text)
PATH.write_text(text, encoding="utf-8", newline="\n")
print("ok", len(json.loads(text)["casts"]))
