"""🧾️ Finisher measurement: prints, per candidate of this ticket, the facts the ticket brief asks for (dimensions, mass balance, rig, poses, clips, thermal regions, solid counts per scale and list, references) and every departure from the brief's minimum."""
import json, sys, os

HERE = os.path.dirname(os.path.abspath(__file__))


def facts(slug):
    """📐️ The measured facts and brief departures of one candidate."""
    asset = json.load(open(os.path.join(HERE, "assets", slug, "🔣️.json")))["asset"]
    bones = asset["rig"]["bones"]
    mass = sum((bone.get("mass") or {}).get("kg", 0) for bone in bones)
    poses = [pose["id"] for pose in asset["poses"] if not pose.get("key")]
    clips = [clip["id"] for clip in asset["clips"]]
    flaws = []
    for need in ("stored", "lifted", "reinstalled"):
        if not any(pose == need or pose.startswith(need) for pose in poses):
            flaws.append(f"no pose {need}")
    if "lift" not in clips:
        flaws.append("no clip lift")
    if len(clips) < 2:
        flaws.append("fewer than two clips")
    for tag in ("component", "reclaimed"):
        if tag not in asset["tags"]:
            flaws.append(f"tag {tag} missing")
    if abs(mass - asset["dimensions"]["massKg"]) > 0.005 * asset["dimensions"]["massKg"]:
        flaws.append(f"bone masses {mass:.2f} kg against massKg {asset['dimensions']['massKg']}")
    if asset["presentation"]["defaultPose"] != "stored":
        flaws.append(f"defaultPose {asset['presentation']['defaultPose']}")
    if asset["completion"] != "authored":
        flaws.append("completion " + asset["completion"])
    if len(asset["exports"]) != 12:
        flaws.append(f"{len(asset['exports'])} exports")
    for scale in ("50", "100", "200", "500"):
        if asset["representations"][scale]["status"] != "authored" or asset["volumes"][scale]["status"] != "authored":
            flaws.append(f"1:{scale} not authored")
    return asset, bones, mass, poses, clips, flaws


def main():
    """🖨️ Prints the facts of the named slugs or of every candidate."""
    slugs = sys.argv[1:] or sorted(os.listdir(os.path.join(HERE, "assets")))
    for slug in slugs:
        asset, bones, mass, poses, clips, flaws = facts(slug)
        size = asset["dimensions"]
        print(f"## {slug}")
        print(f"dimensions h {size['heightM']} w {size['widthM']} d {size.get('depthM')} mass {size['massKg']} kg; bone masses {mass:.2f} kg")
        print(f"bones {len(bones)}: " + ", ".join(f"{bone['id']}({bone['joint']['type']})" for bone in bones))
        print(f"anchors {len(asset['anchors'])}; materials " + ", ".join(material["id"] for material in asset["materials"]))
        print("poses " + ", ".join(f"{pose['id']}[{len(pose.get('contacts', []))}c {len(pose.get('loads', []))}l]" for pose in asset["poses"]))
        print("clips " + ", ".join(f"{clip['id']}({clip['durationS']}s)" for clip in asset["clips"]))
        print("regions " + ", ".join(f"{region['id']}={region['areaM2']}m2/{len(region['layers'])}L" for region in asset["thermal"]["regions"]) + f"; emitters {len(asset['thermal']['emitters'])}; operating {len(asset['thermal']['operating'])}")
        for scale in ("50", "100", "200", "500"):
            volume, drawn = asset["volumes"][scale], asset["representations"][scale]
            views = "/".join(str(len(drawn["geometry"][view])) for view in ("elevation", "front", "plan")) if drawn["status"] == "authored" else drawn["status"]
            print(f"1:{scale} body {len(volume.get('body', []))} thermal {len(volume.get('thermal', []))} structure {len(volume.get('structure', []))}; 2D shapes {views}; range {drawn.get('validRange')}")
        print(f"references {len(asset['provenance']['references'])}; tags {asset['tags']}")
        print("FLAWS: " + ("; ".join(flaws) or "none"))


if __name__ == "__main__":
    main()
