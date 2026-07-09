import { readFileSync } from "fs";
import { join } from "path";
import { getKitDiff, inverseKitDiff, applyKitDiff, areKitsEqual, Kit } from "/workspaces/semio/compose/js/compose";

const ASSETS_DIR = "/workspaces/semio/assets/compose";
const kitRaw: Kit = JSON.parse(readFileSync(join(ASSETS_DIR, "kit_metabolism.json"), "utf-8"));
const kitBefore: Kit = { ...kitRaw, designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent) };
const kitAfter: Kit = JSON.parse(JSON.stringify(kitBefore));

let uuidCounter = 0;
function newGuid(): string {
  uuidCounter++;
  const hex = uuidCounter.toString(16).padStart(12, "0");
  return `019caa00-0000-7000-a000-${hex}`;
}

// Apply changes one category at a time, test each
type TestFn = (before: Kit, after: Kit) => void;
const tests: [string, TestFn][] = [];

tests.push([
  "kit-scalars",
  (b, a) => {
    a.name = "Modified";
    a.version = "r25.08-1";
    a.description = "Mod";
    a.icon = "ic";
    a.image = "im";
    a.remote = "https://x";
    a.homepage = "https://h";
    a.license = "MIT";
    a.preview = "p";
  },
]);

tests.push([
  "types-remove",
  (b, a) => {
    const types = a.types!;
    const idx = types.findIndex((t: any) => t.guid === "71749140-9db9-43f6-bd81-d89011667b80");
    if (idx >= 0) types.splice(idx, 1);
  },
]);

tests.push([
  "types-update-scalars",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) {
      t.name = "Base Mod";
      t.description = "Mod";
      t.virtual = true;
      t.unit = "cm";
      t.isAbstract = true;
      t.stock = 42;
      t.folder = "f";
      t.icon = "i";
      t.image = "im";
    }
  },
]);

tests.push([
  "types-update-location",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) t.location = { guid: newGuid() };
  },
]);

tests.push([
  "types-update-parent",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) t.parent = { guid: a.types![2].guid };
  },
]);

tests.push([
  "types-update-authors",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) t.authors = [{ guid: b.authors![0].guid }];
  },
]);

tests.push([
  "types-update-concepts",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) t.concepts = [{ guid: b.concepts![0].guid }];
  },
]);

tests.push([
  "types-update-connectors",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) {
      // Remove first connector
      t.connectors.splice(0, 1);
      // Update remaining
      const c = t.connectors[0];
      c.name = "mod";
      c.t = 0.75;
      c.point = { x: 10, y: 20, z: 30 };
      c.direction = { x: 1, y: 0, z: 0 };
      // Add new connector
      t.connectors.push({ guid: newGuid(), name: "new", point: { x: 0, y: 0, z: 0 }, direction: { x: 0, y: 1, z: 0 }, t: 0.5 });
    }
  },
]);

tests.push([
  "types-update-models",
  (b, a) => {
    const t = a.types!.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
    if (t) {
      t.models.splice(0, 1);
      t.models[0].name = "mod";
      t.models.push({ guid: newGuid(), name: "new", file: { guid: b.files![1].guid } });
    }
  },
]);

tests.push([
  "types-add",
  (b, a) => {
    a.types!.push({ guid: newGuid(), name: "New", connectors: [], models: [], props: [], attributes: [] } as any);
  },
]);

tests.push([
  "designs-remove",
  (b, a) => {
    const idx = a.designs!.findIndex((d: any) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef");
    if (idx >= 0) a.designs!.splice(idx, 1);
  },
]);

tests.push([
  "designs-update-scalars",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) {
      d.name = "Mod";
      d.description = "Mod";
      d.unit = "cm";
      d.isAbstract = true;
      d.folder = "f";
      d.canScale = true;
      d.canMirror = true;
      d.icon = "i";
      d.image = "im";
    }
  },
]);

tests.push([
  "designs-update-pieces",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) {
      const pieces = d.pieces ?? [];
      if (pieces.length >= 2) {
        const removed = pieces.splice(0, 1)[0];
        pieces[0].name = "mod";
        pieces[0].isHidden = true;
        pieces[0].props = [{ guid: newGuid(), quality: { guid: newGuid() }, value: "v", attributes: [] }];
        pieces[0].attributes = [{ guid: newGuid(), key: "k", value: "v" }];
        // Remove connections referencing removed piece
        d.connections = (d.connections ?? []).filter((c: any) => c.connected?.piece?.guid !== removed.guid && c.connecting?.piece?.guid !== removed.guid);
      }
      pieces.push({ guid: newGuid(), name: "new", type: { guid: b.types![0].guid }, plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } }, props: [], attributes: [] } as any);
      d.pieces = pieces;
    }
  },
]);

tests.push([
  "designs-update-connections",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d && d.connections?.length >= 2) {
      d.connections.splice(0, 1);
      const c = d.connections[0];
      c.gap = 100;
      c.shift = 50;
      c.description = "mod";
      c.attributes = [{ guid: newGuid(), key: "k", value: "v" }];
    }
  },
]);

tests.push([
  "designs-update-stats",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) d.stats = [{ guid: newGuid(), quality: { guid: newGuid() }, unit: "m2" }];
  },
]);

tests.push([
  "designs-update-props",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) d.props = [{ guid: newGuid(), quality: { guid: newGuid() }, value: "v", attributes: [] }];
  },
]);

tests.push([
  "designs-update-layers",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) {
      const lg = newGuid();
      d.layers = [{ guid: lg, path: "layer/test", attributes: [] }];
      d.activeLayer = { guid: lg };
    }
  },
]);

tests.push([
  "designs-update-groups",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d && d.pieces?.length) d.groups = [{ guid: newGuid(), pieces: [{ guid: d.pieces[0].guid }], name: "g", attributes: [] }];
  },
]);

tests.push([
  "designs-update-location",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) d.location = { guid: newGuid() };
  },
]);

tests.push([
  "designs-update-concepts",
  (b, a) => {
    const d = a.designs!.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
    if (d) d.concepts = [{ guid: b.concepts![0].guid }];
  },
]);

tests.push([
  "designs-add",
  (b, a) => {
    a.designs!.push({ guid: newGuid(), name: "New", pieces: [], connections: [], stats: [], props: [], layers: [], groups: [], attributes: [] } as any);
  },
]);

tests.push([
  "tags-ops",
  (b, a) => {
    a.tags!.splice(0, 1);
    a.tags![0].name = "mod";
    a.tags!.push({ guid: newGuid(), name: "New", attributes: [] } as any);
  },
]);

tests.push([
  "concepts-ops",
  (b, a) => {
    a.concepts!.splice(0, 1);
    a.concepts![0].name = "mod";
    a.concepts!.push({ guid: newGuid(), name: "New", attributes: [] } as any);
  },
]);

tests.push([
  "ports-ops",
  (b, a) => {
    a.ports!.splice(0, 1);
    a.ports![0].name = "mod";
    a.ports!.push({ guid: newGuid(), name: "New", attributes: [] } as any);
  },
]);

tests.push([
  "qualities-add",
  (b, a) => {
    a.qualities = [{ guid: newGuid(), key: "area", name: "Floor Area", kind: 1, benchmarks: [{ guid: newGuid(), name: "min", attributes: [] }], attributes: [] } as any];
  },
]);

tests.push([
  "authors-ops",
  (b, a) => {
    a.authors![0].name = "Mod";
    a.authors!.push({ guid: newGuid(), name: "New", email: "t@t.com", attributes: [] } as any);
  },
]);

tests.push([
  "files-ops",
  (b, a) => {
    a.files!.splice(0, 1);
    a.files![0].name = "mod.3dm";
    a.files!.push({ guid: newGuid(), name: "new.txt" } as any);
  },
]);

tests.push([
  "folders-ops",
  (b, a) => {
    a.folders![0].name = "mod";
    a.folders!.push({ guid: newGuid(), name: "new", attributes: [] } as any);
  },
]);

tests.push([
  "kit-attributes",
  (b, a) => {
    a.attributes = [{ guid: newGuid(), key: "k", value: "v" }];
  },
]);

for (const [name, fn] of tests) {
  uuidCounter = 0; // Reset for consistent GUIDs
  const before = JSON.parse(JSON.stringify(kitBefore));
  const after = JSON.parse(JSON.stringify(kitBefore));
  fn(before, after);
  const diff = getKitDiff(before, after);
  const inv = inverseKitDiff(before, diff);
  const appliedFwd = applyKitDiff(before, diff);
  const fwd = areKitsEqual(appliedFwd, after);
  const appliedInv = applyKitDiff(after, inv);
  const invOk = areKitsEqual(appliedInv, before);
  if (!fwd || !invOk) {
    console.log(`[FAIL] ${name}: fwd=${fwd} inv=${invOk}`);
  } else {
    console.log(`[OK]   ${name}`);
  }
}
