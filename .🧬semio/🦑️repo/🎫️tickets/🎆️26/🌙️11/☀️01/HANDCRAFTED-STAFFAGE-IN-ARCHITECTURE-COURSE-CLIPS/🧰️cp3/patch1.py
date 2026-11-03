import json, sys
from pathlib import Path
p = Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
lines = s.split("\n")
i = next(k for k, l in enumerate(lines) if l.startswith("    bim = [v for v"))
j = next(k for k, l in enumerate(lines) if l.startswith('    log("DRIVER DONE")'))
body = lines[i:j]
func = ['def run_bim(state: dict):', '    """🏗️ Render the BIM previews in one process (one exact geometry build)."""'] + [l.replace("if bim and not wanted:", "if bim:") for l in body]
lines[i:j] = []
k = next(n for n, l in enumerate(lines) if l.startswith("def main():"))
lines[k:k] = func + ["", ""]
s = "\n".join(lines)
s = s.replace("order = wanted or cheap + mid + ENERGY", 'order = wanted or cheap + mid + ["bim"] + ENERGY')
s = s.replace("        video_id = ready[0]\n        pending.remove(video_id)\n", '        video_id = ready[0]\n        pending.remove(video_id)\n        if video_id == "bim":\n            run_bim(state)\n            continue\n')
p.write_text(s, encoding="utf-8")
