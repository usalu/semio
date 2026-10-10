"""Replace one '#region BeatN' block of scene_5.py with the text of a snippet file (temporary helper)."""
import sys
from pathlib import Path

scene = Path("/Users/niloufarghandehariyoon/Developer/semio/tutorial/energy/demand/Heating/5_solar_heat_gain/scene_5.py")
start_marker, end_marker, snippet = sys.argv[1], sys.argv[2], Path(sys.argv[3]).read_text()
src = scene.read_text()
a = src.index(start_marker)
b = src.index(end_marker, a + len(start_marker))
scene.write_text(src[:a] + snippet.rstrip("\n") + "\n\n\n" + src[b:])
print("[DEBUG] spliced", start_marker, "->", end_marker)
