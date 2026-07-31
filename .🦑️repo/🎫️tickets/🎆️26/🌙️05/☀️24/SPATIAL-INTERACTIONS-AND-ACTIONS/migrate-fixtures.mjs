import { readFileSync, writeFileSync, renameSync, unlinkSync, existsSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "..", "..", "..", "..", "..", "..", "spatial", "fixtures");

function migrateCommitOperation(op) {
  if (!op || typeof op !== "object") return op;
  const k = op.kind;
  if (k === "cell.createBox") {
    return {
      kind: "action",
      action: "primitive.createBoxFromCorners",
      params: { cornerA: op.cornerA, cornerB: op.cornerB, height: op.height },
    };
  }
  if (k === "wire.extrudeToCell") {
    return {
      kind: "action",
      action: "feature.extrudeWireToCell",
      params: { wireId: op.wireId, distance: op.distance, direction: op.direction },
    };
  }
  if (k === "face.offset") {
    return { kind: "action", action: "feature.offsetFaces", params: { faceIds: op.faceIds, distance: op.distance } };
  }
  if (k === "measure.distance") {
    return { kind: "action", action: "measure.vertexDistance", params: { a: op.a, b: op.b } };
  }
  if (k === "measure.area") {
    return { kind: "action", action: "measure.faceArea", params: { faceId: op.faceId } };
  }
  if (k === "measure.volume") {
    return { kind: "action", action: "measure.cellVolume", params: { cellId: op.cellId } };
  }
  return op;
}

function walk(o, fn) {
  if (Array.isArray(o)) {
    for (const x of o) walk(x, fn);
    return;
  }
  if (o && typeof o === "object") {
    fn(o);
    for (const k of Object.keys(o)) walk(o[k], fn);
  }
}

function migrateFile(name) {
  const p = join(root, name);
  let s = readFileSync(p, "utf8");
  s = s.replaceAll('"schema": "spatial.command/v1"', '"schema": "spatial.interaction/v1"');
  s = s.replaceAll('"actions":', '"effects":');
  s = s.replaceAll(/"op":\s*"box.transform",\s*"transform":\s*"([^"]+)"/g, (_, t) => `"op": "action", "action": "box.${t}"`);
  const j = JSON.parse(s);
  delete j.history;
  if (j.commit?.operation) j.commit.operation = migrateCommitOperation(j.commit.operation);
  walk(j.machine, (o) => {
    if (o.effects && Array.isArray(o.effects)) {
      for (const e of o.effects) {
        if (e && e.op === "action" && typeof e.action === "string" && e.action.startsWith("box.")) {
          if (!e.params) e.params = {};
        }
      }
    }
  });
  const out = JSON.stringify(j, null, 2) + "\n";
  const newName = name.replace(".command.json", ".interaction.json");
  writeFileSync(join(root, newName), out, "utf8");
  unlinkSync(p);
  console.log(name, "->", newName);
}

for (const f of ["box.command.json", "extrude-wire.command.json", "offset-surface.command.json", "distance.command.json", "area.command.json"]) {
  migrateFile(f);
}

for (const dead of ["factory.json", "extrude.factory.json", "offset-surface.factory.json"]) {
  const p = join(root, dead);
  if (existsSync(p)) {
    unlinkSync(p);
    console.log("removed", dead);
  }
}
