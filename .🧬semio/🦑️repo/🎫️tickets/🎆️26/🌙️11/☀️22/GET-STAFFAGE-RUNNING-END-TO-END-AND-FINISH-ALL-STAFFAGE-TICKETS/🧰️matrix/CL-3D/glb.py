"""🧾️ Where the bytes of a GLB go, read from the file alone: the JSON chunk by top-level key, the binary chunk by what its buffer views carry (POSITION, NORMAL, triangle indices, line indices, the rest), per glTF scene the triangles, line segments and the bytes of the views only that scene uses, and how many accessors and views are shared between scenes. glb.py <file.glb>…"""
import collections, json, struct, sys

SIZE = {5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4}
WIDTH = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4, "MAT4": 16}
for path in sys.argv[1:]:
    data = open(path, "rb").read()
    length = struct.unpack_from("<I", data, 12)[0]
    doc = json.loads(data[20:20 + length])
    print(f"{path.split('/')[-1]}: {len(data)} bytes, json {length}, bin {len(data) - length - 28}; {len(doc['nodes'])} nodes, {len(doc['meshes'])} meshes, {sum(len(m['primitives']) for m in doc['meshes'])} primitives, {len(doc['accessors'])} accessors, {len(doc['bufferViews'])} views, {len(doc['materials'])} materials")
    print("  json: " + ", ".join(f"{k} {len(json.dumps(v, separators=(',', ':')))}" for k, v in sorted(doc.items(), key=lambda kv: -len(json.dumps(kv[1])))[:7]))
    extras = sum(len(json.dumps(p.get("extras", {}), separators=(",", ":"))) for m in doc["meshes"] for p in m["primitives"])
    variants = sum(len(json.dumps(p.get("extensions", {}), separators=(",", ":"))) for m in doc["meshes"] for p in m["primitives"])
    print(f"  primitive extras {extras}, primitive variant mappings {variants}")
    use = {}
    view_scenes = collections.defaultdict(set)
    accessor_scenes = collections.defaultdict(set)
    mesh_scenes = collections.defaultdict(set)
    for s, scene in enumerate(doc["scenes"]):
        stack = list(scene["nodes"])
        while stack:
            node = doc["nodes"][stack.pop()]
            stack += node.get("children", [])
            if "mesh" in node:
                mesh_scenes[node["mesh"]].add(s)
    per_scene = collections.defaultdict(lambda: [0, 0])
    for m, mesh in enumerate(doc["meshes"]):
        for p in mesh["primitives"]:
            lines = p.get("mode", 4) == 1
            for name, a in p["attributes"].items():
                use[a] = name
            use[p["indices"]] = "line indices" if lines else "indices"
            for a in list(p["attributes"].values()) + [p["indices"]]:
                accessor_scenes[a] |= mesh_scenes[m]
                view_scenes[doc["accessors"][a]["bufferView"]] |= mesh_scenes[m]
            for s in mesh_scenes[m]:
                per_scene[s][1 if lines else 0] += doc["accessors"][p["indices"]]["count"] // (2 if lines else 3)
    kinds = collections.Counter()
    counts = collections.Counter()
    seen = set()
    for a, accessor in enumerate(doc["accessors"]):
        view = accessor["bufferView"]
        if view in seen:
            continue
        seen.add(view)
        key = f"{use.get(a, 'other')}/{accessor['componentType']}"
        kinds[key] += doc["bufferViews"][view]["byteLength"]
        counts[key] += 1
    print("  bin: " + ", ".join(f"{k} {v} ({counts[k]} views)" for k, v in kinds.most_common()))
    shared = sum(doc["bufferViews"][v]["byteLength"] for v, scenes in view_scenes.items() if len(scenes) > 1)
    print(f"  views used by more than one scene: {sum(1 for s in view_scenes.values() if len(s) > 1)} of {len(view_scenes)}, {shared} bytes")
    for s, scene in enumerate(doc["scenes"]):
        own = sum(doc["bufferViews"][v]["byteLength"] for v, scenes in view_scenes.items() if scenes == {s})
        print(f"  {scene.get('name')}: {per_scene[s][0]} triangles, {per_scene[s][1]} segments, {own} bytes of its own")
