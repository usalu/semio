import os

root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"


def rw(p, f):
    p = root + p
    s = open(p, encoding="utf-8", newline="").read()
    nl = "\r\n" if "\r\n" in s else "\n"
    s = f(s.replace("\r\n", "\n"))
    open(p + ".tmp", "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
    os.replace(p + ".tmp", p)


def cat(s):
    def sub(a, b):
        nonlocal s
        assert a in s, a[:80]
        s = s.replace(a, b, 1)

    sub('import { evaluate } from "../🧮️evaluation/🟦️.ts";', 'import { evaluate, flattenGeometry } from "../🧮️evaluation/🟦️.ts";')
    i = s.index('      if (followTip && !path.endsWith(shape.kind === "capsule"')
    j = s.index("\n", i)
    s = s[:i] + '      if (followTip && !path.endsWith(".geometry.front") && !(path.endsWith(".geometry.plan"))) add(sp, "follow-tip-viewpoint", `Shape "${shape.id}" sets followTip, which exists for front and plan shapes only.`, `Form "${shape.id}" setzt followTip, das nur für Vorderansicht- und Grundrissformen vorkommt.`);\n      if (followTip && path.endsWith(".geometry.front") && shape.kind === "capsule") add(sp, "follow-tip-viewpoint", `Shape "${shape.id}" is a capsule with followTip in the front view; use a rect or poly there.`, `Form "${shape.id}" ist eine Kapsel mit followTip in der Vorderansicht; dort ein Rechteck oder Polygon verwenden.`);' + s[j:]
    i = s.index('        if (!path.endsWith(".geometry.front") && !path.endsWith(".geometry.plan")) add(sp, "solid-viewpoint"')
    j = s.index("\n", i)
    s = s[:i] + s[j + 1 :]
    marker = '    const drawn = new Set(rep.geometry.elevation.flatMap('
    add = '''    elevationLateral(path, rep.geometry);
'''
    sub(marker, add + marker)
    anchor = "  const checkShapes = (path: string"
    helper = '''  let spreadSet: Set<Slug> | undefined;
  const spreadBones = (): Set<Slug> => {
    if (spreadSet !== undefined) return spreadSet;
    const own = new Set<Slug>();
    const scan = (offsets: Readonly<Record<Slug, BoneOffset>> | undefined): void => {
      for (const [id, offset] of Object.entries(offsets ?? {})) if ((offset.abductionDeg ?? 0) !== 0 || (offset.twistDeg ?? 0) !== 0) own.add(id);
    };
    for (const pose of asset.poses) scan(pose.bones);
    for (const clip of asset.clips) for (const key of clip.keys) scan(key.bones);
    spreadSet = new Set<Slug>();
    for (const bone of asset.rig.bones) for (let cursor: typeof bone | undefined = bone; cursor !== undefined; cursor = cursor.parent === null ? undefined : byId.get(cursor.parent)) if (own.has(cursor.id)) { spreadSet.add(bone.id); break; }
    return spreadSet;
  };
  const elevationLateral = (path: string, geometry: { readonly elevation: readonly Shape[]; readonly front: readonly Shape[] }): void => {
    const spread = spreadBones();
    if (spread.size === 0) return;
    const limit = Math.max(0.05, 0.1 * Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM));
    geometry.elevation.forEach((shape, index) => {
      if (shape.solid !== undefined || shape.role === "ground" || !spread.has(shape.bone)) return;
      const twin = geometry.front.find((candidate) => candidate.id === shape.id && candidate.bone === shape.bone && candidate.solid === undefined);
      if (twin === undefined) return;
      const xs = flattenGeometry(twin).rings.flat().map((p) => p.x);
      const width = Math.max(...xs) - Math.min(...xs);
      if (width > limit) add(`${path}.geometry.elevation[${index}]`, "elevation-lateral", `Shape "${shape.id}" on bone "${shape.bone}" is ${width.toFixed(3)} m wide laterally, but the bone is spread or twisted in a pose or clip: author it as a solid (extruded or revolved) so the side view follows, or make it thin.`, `Form "${shape.id}" am Knochen "${shape.bone}" ist seitlich ${width.toFixed(3)} m breit, aber der Knochen wird in einer Pose oder einem Clip gespreizt oder verdreht: als Körper (extrudiert oder rotiert) erstellen, damit die Seitenansicht folgt, oder dünn halten.`);
    });
  };
'''
    sub(anchor, helper + anchor)
    return s


rw("🔨️modules/🗂️catalog/🟦️.ts", cat)


def tsd(s):
    a = "/** 🌀️ How a front or plan shape is a solid of its rig-plane profile, authored in the bone's local frame like an elevation shape and placed with the full bone transform, so it reads right at every pose and clip instant."
    assert a in s
    s = s.replace(a, "/** 🌀️ How a shape of any view is a solid of its rig-plane profile, authored in the bone's local frame like an elevation shape and placed with the full 3D frame of its bone (spread, twist and sagittal rotation) and projected to the view (elevation, front or plan), so one 3D definition reads right in all three views at every pose and clip instant.", 1)
    return s


rw("🧬️schema/🟦️.ts", tsd)


def js(s):
    a = '"description": "How a front or plan shape is a solid of the rig plane profile, authored in its bone\'s local frame like an elevation shape and placed with the full bone transform.'
    assert a in s
    return s.replace(a, '"description": "How a shape of any view (elevation, front or plan) is a solid of the rig plane profile, authored in its bone\'s local frame like an elevation shape, placed with the full 3D frame of its bone and projected to the view; the elevation, front and plan of one solid are one 3D definition.', 1)


rw("🧬️schema/🔣️.json", js)
