import json, os

SEMIO = "/Users/ueli/Documents/semio"
os.chdir(SEMIO)

with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/503877ec-a133-46c2-bb1e-82dfd05403d6/scratchpad/component_dirs.txt") as f:
    dirs = [l.strip()[2:] for l in f if l.strip()]

def surfaces_from_fs(d):
    entries = os.listdir(d)
    present = set()
    for e in entries:
        p = os.path.join(d, e)
        if e.startswith("🦀️") and e.endswith(".rs"): present.add("rust")
        if e.startswith("🟦️") and e.endswith(".ts"): present.add("typescript")
        if e.startswith("🔗️") and e.endswith(".graphql"): present.add("graphql")
        if e.startswith("🛰️") and e.endswith(".proto"): present.add("protobuf")
        if e.startswith("🔣️") and e.endswith(".schema.json"): present.add("json-schema")
        if e == "📝️text" and os.path.isdir(p): present.add("text")
        if e == "💾️binary" and os.path.isdir(p): present.add("binary")
        if (e.startswith("🐍️") and e.endswith(".py")): present.add("python")
        if (e.startswith("🔷️") and e.endswith(".cs")): present.add("csharp")
    return present

results = []
for d in dirs:
    jpath = os.path.join(d, "🔣️.json")
    cpath = os.path.join(d, "🔣️component.json")
    j = json.load(open(jpath, encoding="utf-8"))
    c = json.load(open(cpath, encoding="utf-8"))
    fs_surfaces = surfaces_from_fs(d)
    j_surf = set(j.get("requiredLanguageSurfaces", []))
    c_surf = set(c.get("requiredLanguageSurfaces", []))
    j_match = j_surf == fs_surfaces
    c_match = c_surf == fs_surfaces
    j_extra = j_surf - fs_surfaces
    j_missing = fs_surfaces - j_surf
    c_extra = c_surf - fs_surfaces
    c_missing = fs_surfaces - c_surf
    results.append({
        "dir": d, "fs_surfaces": sorted(fs_surfaces),
        "j_surf": sorted(j_surf), "c_surf": sorted(c_surf),
        "j_match": j_match, "c_match": c_match,
        "j_extra": sorted(j_extra), "j_missing": sorted(j_missing),
        "c_extra": sorted(c_extra), "c_missing": sorted(c_missing),
    })

both_match = [r for r in results if r["j_match"] and r["c_match"]]
only_c = [r for r in results if r["c_match"] and not r["j_match"]]
only_j = [r for r in results if r["j_match"] and not r["c_match"]]
neither = [r for r in results if not r["j_match"] and not r["c_match"]]

print(f"Total: {len(results)}")
print(f"Both match fs: {len(both_match)}")
print(f"Only component.json matches fs: {len(only_c)}")
print(f"Only .json matches fs: {len(only_j)}")
print(f"Neither matches fs: {len(neither)}")

print("\n--- only_j (canonical .json matches fs, component.json doesn't) ---")
for r in only_j[:20]:
    print(r["dir"])
    print("  fs:", r["fs_surfaces"], "j:", r["j_surf"], "c:", r["c_surf"])

print("\n--- neither matches (first 20) ---")
for r in neither[:20]:
    print(r["dir"])
    print("  fs:", r["fs_surfaces"], "j:", r["j_surf"], "c:", r["c_surf"])

with open(os.path.join(SEMIO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RESOLVE-COLLIDED-COMPONENT-JSON-MIGRATION-STRAGGLERS/🗑️generated/analysis2.json"), "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False, indent=2)
