#!/bin/zsh
# 📏️ Measures the 1:50 outer area of every thermal region: writes the candidate with absurd areas, reads the validator's measured values and stores them in areas.json, then regenerates the candidate with them.
HERE=${0:A:h}
T="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
T=${T:A}
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "$HERE/🗑️generated"
python3 -c "import json; print(json.dumps({r: 9.0 for r in ['violin-top','violin-back','violin-neck','worn-varnish','fingerboard-fittings','chinrest','bridge','strings','bow','case-cover','case-lining','case-hardware','strap']}))" > "$HERE/🗑️generated/probe-areas.json"
cd "$HERE" && python3 author-used-violin.py "$HERE/🗑️generated/probe-areas.json" > /dev/null
cd "$T" && $BUN r3-validate.ts --file "$HERE/🔣️.json" --scale 50 --rules thermal-coverage --no-print > "$HERE/🗑️generated/areas.txt" 2>&1
python3 - "$HERE" <<'PY'
import re, sys, json, pathlib
here = pathlib.Path(sys.argv[1])
text = (here / "🗑️generated" / "areas.txt").read_text()
areas = {m.group(1): float(m.group(2)) for m in re.finditer(r'region "([^"]+)" at 1:50 is ([0-9.]+) m', text)}
(here / "areas.json").write_text(json.dumps(areas, indent=1) + "\n")
print("[DEBUG] measured", areas)
PY
cd "$HERE" && python3 author-used-violin.py "$HERE/areas.json"
