import json, sys
from pathlib import Path
R = Path("C:/git/semio/🎓️teaching/🏛️architecture")
inv = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
man = json.loads((R / "⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
proj = json.loads((R / "⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️staffage_only.json").read_text(encoding="utf-8"))["scene_projections"]["clips"]
users = {}
for clip, casts in inv["clips"].items():
    for c in casts:
        users.setdefault(c, []).append(clip.split("/🎬️clip/")[0].split("/")[-1] + "/" + clip.split("/🎬️clip/")[1].split("/")[0])
rows = ["| cast | asset | view | viewpoint | mirror | scale | scenes (chapter/clip) | scene projection |", "|---|---|---|---|---|---|---|---|"]
for name, e in man.items():
    clips = users.get(name, [])
    projs = sorted({proj[k] for k, v in inv["clips"].items() if name in v})
    rows.append(f"| {name} | {e['asset']} | {e['view']} | {e['viewpoint']} | {'yes' if e.get('mirror') else ''} | {e.get('scale', 50)} | {', '.join(clips) or 'library or unused'} | {', '.join(projs) or '-'} |")
Path(sys.argv[2]).write_text("\n".join(rows) + "\n", encoding="utf-8")
