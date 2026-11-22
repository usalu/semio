"""📇 Finisher A facts: prints the measured facts of catalog plants as a markdown block for the worker reports (dimensions, mass, bones, poses with total loads, clips, thermal regions, emitters, solids per scale and list, drawn shapes per scale and view, references).

Usage: `python3 facts.py <id>...`.
See `../../📋️brief.md` (report section).
"""
import json, pathlib, sys

TICKET = pathlib.Path(__file__).resolve().parent.parent.parent
VEGETATION = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"


def block(ident: str) -> str:
    asset = json.loads((VEGETATION / ident / "🔣️.json").read_text("utf8"))["asset"]
    d = asset["dimensions"]
    lines = [f"### `{ident}`: {asset['title']['en']} / {asset['title']['de']}", ""]
    lines.append(f"- Dimensions: {d['heightM']} m high, {d['widthM']} m wide, {d['depthM']} m deep, {d['massKg']} kg; {len(asset['rig']['bones'])} bones ({asset['rig']['kind']} rig, root `{asset['rig']['root']}`); supports {asset['structure']['supports']}; default pose `{asset['presentation']['defaultPose']}`, default scale 1:{asset['presentation']['defaultScale']}.")
    poses = []
    for pose in asset["poses"]:
        fx = sum(load.get("fxN", 0) for load in pose.get("loads", []))
        fy = sum(load.get("fyN", 0) for load in pose.get("loads", []))
        poses.append(f"`{pose['id']}` ({pose['title']['en']}; loads Σfx {fx:.0f} N, Σfy {fy:.0f} N)")
    lines.append("- Poses: " + "; ".join(poses) + ".")
    lines.append("- Clips: " + "; ".join(f"`{c['id']}` ({c['durationS']} s, {'looping' if c['loop'] else 'one-shot'}, {len(c['keys'])} keys, {len(c.get('tracks', []))} tracks)" for c in asset["clips"]) + ".")
    lines.append("- Thermal regions: " + "; ".join(f"`{r['id']}` {r['kind']}/{r['model']} {r['areaM2']} m²" for r in asset["thermal"]["regions"]) + "; emitters: " + ", ".join(f"`{e['id']}` ({e['kind']}, {len(e.get('anchors', []))} anchors)" for e in asset["thermal"]["emitters"]) + ".")
    for scale in ("500", "200", "100", "50"):
        volume = asset["volumes"][scale]
        rep = asset["representations"][scale]
        shapes = ", ".join(f"{view} {len(items)}" for view, items in rep["geometry"].items())
        solids = ", ".join(f"{name} {len(volume[name])}" for name in ("body", "thermal", "structure") if name in volume)
        lines.append(f"- 1:{scale} ({rep['status']}, valid {json.dumps(rep['validRange'])}): shapes {shapes}; solids {solids}. {rep['decisions']['silhouette']['en']}")
    lines.append("- Symbols: " + ", ".join(f"`{s['id']}`" for s in asset["symbols"]) + "; exports: " + ", ".join(asset["exports"]) + ".")
    lines.append("- References: " + " | ".join(r["citation"] for r in asset["provenance"]["references"]))
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    print("\n".join(block(ident) for ident in sys.argv[1:]))
