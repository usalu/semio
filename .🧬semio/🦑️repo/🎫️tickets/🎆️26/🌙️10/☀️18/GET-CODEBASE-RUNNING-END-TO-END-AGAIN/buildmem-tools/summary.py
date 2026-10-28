"""📈 [buildmem] One line per harness run: wall, peak RSS, footprint max/p90, live after GC, from 🗑️generated/buildmem/<label>.{json,fp}. usage: summary.py <label>..."""
import json, os, sys
O = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "buildmem")
for label in sys.argv[1:]:
    j = json.load(open(os.path.join(O, label + ".json")))
    fps = []
    for line in open(os.path.join(O, label + ".fp")):
        cells = line.split()
        if "fp" in cells:
            at = cells.index("fp")
            try:
                value = float(cells[at + 1]); unit = cells[at + 2]
                fps.append(value * (1024 if unit == "GB" else 1 / 1024 if unit == "KB" else 1))
            except (IndexError, ValueError):
                pass
    fps.sort()
    end = [r for r in j["series"] if r["tag"] == "afterGC"]
    live = end[-1]["heap"] + end[-1]["extra"] if end else None
    print(f"{label}: wall {j['seconds']} s, peak RSS {j['peakRssMB']} MB, footprint max {round(fps[-1]) if fps else '?'} MB p90 {round(fps[int(len(fps)*0.9)]) if fps else '?'} MB, live after GC {live} MB, projects {j['projects']}")
