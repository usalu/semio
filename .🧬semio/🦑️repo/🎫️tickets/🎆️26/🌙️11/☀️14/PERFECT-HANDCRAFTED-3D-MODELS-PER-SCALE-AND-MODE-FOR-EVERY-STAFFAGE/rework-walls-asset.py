"""🗒️ Compact overview of a catalog component: dimensions, rig, thermal regions and layers, per-scale 2D shapes and decisions."""
import json, sys, os

S = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage")


def main():
    asset = json.load(open(os.path.join(S, "🗂️catalog", "🧱️components", sys.argv[1], "🔣️.json"), encoding="utf-8"))["asset"]
    scales = sys.argv[2].split(",") if len(sys.argv) > 2 else ["50", "100", "200", "500"]
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 200
    print("dimensions", asset["dimensions"], "default pose", asset["presentation"]["defaultPose"])
    for bone in asset["rig"]["bones"]:
        print("bone", {key: bone.get(key) for key in ("id", "parent", "x", "y", "z", "angleDeg", "lengthM")}, bone["joint"]["type"])
    for anchor in asset["anchors"]:
        print("anchor", {key: anchor.get(key) for key in ("id", "bone", "x", "y", "kind", "support")})
    for region in asset["thermal"]["regions"]:
        print("region", region["id"], region["kind"], region["model"], region["material"], region["areaM2"], [(layer["id"], layer["material"], layer.get("thicknessM")) for layer in region["layers"]])
    print("structure", {key: value for key, value in asset["structure"].items() if key != "assumptions"})
    for scale in scales:
        drawing = asset["representations"][scale]
        print("==", scale, drawing["validRange"], drawing.get("sections"))
        for key in ("retained", "merged", "omitted", "thinElements", "hatching"):
            print("  ", key, ":", drawing["decisions"][key]["en"][:500])
        for view, shapes in drawing["geometry"].items():
            for shape in shapes:
                text = json.dumps({key: value for key, value in shape.items() if key not in ("bone", "tone", "z")}, ensure_ascii=False)
                print("  ", view, text[:limit])


if __name__ == "__main__":
    main()
