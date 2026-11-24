/** 🛰️ The cheap whole-catalog part of the matrix: `bun catalog-scan.ts <tests-dir> <out.jsonl> [ids]` appends one JSON line per asset with the derived-view flags of every authored scale (as `checkDerived`), the geometry findings of every pose at every authored scale (extent sync, `boneSync`, `anchorLandmarks`, dimensions of the default pose, as `checkGeometry`) and the missing analysis cue of the forces and moments scenes of the default pose. Clips, transitions, band ends and symbols are left to the matrix. */
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const tests = resolve(process.argv[2]!);
const out = resolve(process.argv[3]!);
const only = process.argv[4]?.split(",");
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { derivedFlags } = await import(join(tests, "🔭️multiview-coverage", "🪞️derived.ts"));
const { anchorLandmarks, boneSync, distanceToShapes } = await import(join(tests, "🔭️multiview-coverage", "⚓️landmarks.ts"));
const { evaluate } = await import(join(tests, "..", "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { resolveViews } = await import(join(tests, "..", "🔨️modules", "🔭️multiview", "🟦️.ts"));

const done = new Set(existsSync(out) ? readFileSync(out, "utf8").split("\n").filter(Boolean).map((line) => JSON.parse(line).id as string) : []);
const VIEWS = ["elevation", "front", "plan"] as const;
const bump = (map: Record<string, number>, key: string): void => void (map[key] = (map[key] ?? 0) + 1);
const hasCue = (scene: any): boolean => scene.layers.some((layer: any) => layer.items.some((item: any) => item.kind === "path" && (item.fill?.pattern !== undefined || (item.stroke?.dashMm?.length ?? 0) > 0)));

for (const asset of engine.loadCatalog()) {
  if (done.has(asset.id) || (only !== undefined && !only.includes(asset.id))) continue;
  const started = Date.now();
  const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
  const row: any = { id: asset.id, category: asset.category, authored, unavailable: Object.keys(asset.representations).filter((key) => !authored.includes(key)), poses: asset.poses.length, clips: asset.clips.length, derived: [], geometry: {}, examples: {}, cue: [], errors: [] };
  const tolerance = engine.toleranceM(asset);
  const children = new Map<string, string[]>();
  for (const bone of asset.rig.bones) if (bone.parent !== undefined && bone.parent !== null) (children.get(bone.parent) ?? children.set(bone.parent, []).get(bone.parent)!).push(bone.id);
  const subtrees = new Map<string, Set<string>>();
  const descendants = (bone: string): Set<string> => {
    const known = subtrees.get(bone);
    if (known !== undefined) return known;
    const found = new Set<string>([bone]);
    for (const child of children.get(bone) ?? []) for (const id of descendants(child)) found.add(id);
    subtrees.set(bone, found);
    return found;
  };
  const shapesAt: Record<string, any> = {};
  for (const key of authored) {
    try {
      shapesAt[key] = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
    } catch (error) {
      row.errors.push(`evaluate ${key}: ${(error as Error).message.slice(0, 160)}`);
    }
  }
  for (const key of authored) {
    if (shapesAt[key] === undefined) continue;
    const others = Object.fromEntries(authored.filter((other) => other !== key && shapesAt[other] !== undefined).map((other) => [`1:${other}`, shapesAt[other]]));
    for (const flag of derivedFlags(shapesAt[key], others)) row.derived.push({ scale: key, ...flag });
    const counts: Record<string, number> = (row.geometry[key] = {});
    let resting = false;
    const note = (code: string, text: string): void => {
      bump(counts, code);
      if (resting) bump((row.rest ??= {}), code);
      row.examples[code] ??= `1:${key} ${text}`;
    };
    for (const pose of asset.poses) {
      resting = pose.id === asset.presentation.defaultPose;
      let state: any;
      try {
        state = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
      } catch (error) {
        note("evaluate-error", `${pose.id}: ${(error as Error).message.slice(0, 160)}`);
        continue;
      }
      const e = engine.extentOf(state.shapes.elevation);
      const f = engine.extentOf(state.shapes.front);
      const p = engine.extentOf(state.shapes.plan);
      const agree = (code: string, a: [number, number], b: [number, number]): void => {
        const delta = Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]));
        if (!(delta <= tolerance)) note(code, `${pose.id}: [${a.map((v) => v.toFixed(3))}] vs [${b.map((v) => v.toFixed(3))}] tol ${tolerance.toFixed(3)}`);
      };
      if (!e) note("elevation-empty", pose.id);
      if (!f) note("front-empty", pose.id);
      if (!p) note("plan-empty", pose.id);
      if (e && p) agree("sync-forward", [e.minX, e.maxX], [p.minX, p.maxX]);
      if (e && f) agree("sync-height", [e.minY, e.maxY], [f.minY, f.maxY]);
      if (f && p) agree("sync-lateral", [f.minX, f.maxX], [p.minY, p.maxY]);
      for (const finding of boneSync(state, tolerance)) {
        note(finding.check, `${pose.id}: ${finding.message}`);
        const [first, other] = finding.pair.split("/") as ["elevation" | "front", "front" | "plan"];
        const idsOf = (view: string): string => state.shapes[view].filter((shape: any) => shape.shape.bone === finding.bone && shape.shape.role !== "ground").map((shape: any) => shape.shape.id).sort().join(",");
        const twin = finding.tier !== "bone" ? "group" : idsOf(first) === idsOf(other) ? "same shape id" : "other shape id";
        const drawn = state.shapes[other].filter((shape: any) => shape.shape.bone === finding.bone && shape.shape.role !== "ground").map((shape: any) => (shape.shape.solid !== undefined ? "solid" : shape.shape.kind === "member" ? "member" : shape.shape.followTip === true ? "followTip" : "flat"));
        bump((row.syncClasses ??= {}), `${finding.check} | ${pose.id === asset.presentation.defaultPose ? "rest" : "posed"} | ${[...new Set(drawn)].sort().join("+")} | ${twin}`);
      }
      for (const finding of anchorLandmarks(state, tolerance)) {
        note("anchor-landmark", `${pose.id}: ${finding.message}`);
        const entry = state.anchors[finding.anchor];
        const point = finding.view === "elevation" ? { x: entry.at.x, y: entry.at.y } : finding.view === "front" ? { x: entry.z, y: entry.at.y } : { x: entry.at.x, y: entry.z };
        const drawn = state.shapes[finding.view].filter((shape: any) => shape.shape.role !== "ground");
        const subtree = distanceToShapes(point, drawn.filter((shape: any) => descendants(finding.bone).has(shape.shape.bone)));
        const any = distanceToShapes(point, drawn);
        const kind = `${entry.anchor.kind}${entry.anchor.z === undefined ? " without z" : ""}`;
        const cls = subtree <= tolerance ? "on a descendant bone" : any <= tolerance ? "on another bone" : "off every drawn shape";
        bump((row.anchorClasses ??= {}), `${kind} | ${finding.view} | ${cls}`);
        ((row.anchorIds ??= {})[finding.anchor] ??= {})[`${finding.view} ${cls}`] = Math.max(row.anchorIds[finding.anchor][`${finding.view} ${cls}`] ?? 0, Math.round(finding.deltaMm));
      }
      if (pose.id !== asset.presentation.defaultPose) continue;
      const d = asset.dimensions;
      const size = (code: string, measured: number, nominal: number): void => {
        if (!(Math.abs(measured - nominal) <= engine.d18(nominal))) note(code, `${measured.toFixed(3)} vs ${nominal.toFixed(3)}`);
      };
      if (e) size("dimension-elevation-width", e.maxX - e.minX, d.widthM), size("dimension-elevation-height", e.maxY - e.minY, d.heightM);
      if (f) size("dimension-front-width", f.maxX - f.minX, d.depthM), size("dimension-front-height", f.maxY - f.minY, d.heightM);
      if (p) size("dimension-plan-length", p.maxX - p.minX, d.widthM), size("dimension-plan-width", p.maxY - p.minY, d.depthM);
    }
    for (const mode of ["forces", "moments"] as const) {
      try {
        const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator: Number(key) }, mode, engine.drawOptions(mode, Number(key)));
        for (const view of VIEWS) {
          const drawing = multi.views[view];
          if (drawing.kind === "unavailable") row.cue.push({ scale: key, mode, view, unavailable: drawing.code });
          else if (!hasCue(drawing.scene)) row.cue.push({ scale: key, mode, view });
        }
      } catch (error) {
        row.errors.push(`resolve ${key} ${mode}: ${(error as Error).message.slice(0, 160)}`);
      }
    }
  }
  row.ms = Date.now() - started;
  appendFileSync(out, `${JSON.stringify(row)}\n`);
  console.error(`${asset.id} ${row.ms} ms`);
}
