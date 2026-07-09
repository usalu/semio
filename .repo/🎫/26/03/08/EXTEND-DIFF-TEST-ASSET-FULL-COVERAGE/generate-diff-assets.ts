// Generator script for comprehensive diff test assets.
// Exercises every single diffable feature exactly once.
//
// NOTE: We avoid undefined→defined transitions on UPDATED entities for fields
// like mirrorPlane, center, designPiece because the diff system can't represent
// "set to undefined" in JSON (undefined values are lost during serialization).
// These fields ARE exercised on ADDED entities instead.
import { readFileSync, writeFileSync } from "fs";
import { join } from "path";
import { getKitDiff, inverseKitDiff, applyKitDiff, areKitDiffsEqual, areKitsEqual, Kit } from "/workspaces/semio/compose/js/compose";

const ASSETS_DIR = "/workspaces/semio/assets/compose";

const kitRaw: Kit = JSON.parse(readFileSync(join(ASSETS_DIR, "kit_metabolism.json"), "utf-8"));

// Filter out designs with parent (matches the test setup)
const kitBefore: Kit = {
  ...kitRaw,
  designs: (kitRaw.designs ?? []).filter((d: any) => !d.parent),
};

// Deep clone for the "after" kit
const kitAfter: Kit = JSON.parse(JSON.stringify(kitBefore));

// Helper: generate deterministic GUIDs
let uuidCounter = 0;
function newGuid(): string {
  uuidCounter++;
  const hex = uuidCounter.toString(16).padStart(12, "0");
  return `019caa00-0000-7000-a000-${hex}`;
}

// ============================================================
// KIT SCALARS - exercise every kit-level scalar diff field
// ============================================================
kitAfter.name = "Metabolism Modified";
kitAfter.version = "r25.08-1";
kitAfter.description = "Modified version for comprehensive diff testing";
kitAfter.icon = "modified-icon.svg";
kitAfter.image = "modified-image.png";
kitAfter.remote = "https://modified.example.com/archive.tar.gz";
kitAfter.homepage = "https://modified.example.com";
kitAfter.license = "MIT-Modified";
kitAfter.preview = "modified-preview.png";

// ============================================================
// TYPES - removed, updated (all fields + nested), added
// ============================================================
const types = kitAfter.types!;

// REMOVED: Capsule type
const typeToRemoveIdx = types.findIndex((t: any) => t.guid === "71749140-9db9-43f6-bd81-d89011667b80");
if (typeToRemoveIdx >= 0) types.splice(typeToRemoveIdx, 1);

// UPDATED: Type "Base" (2 connectors + 6 models)
const typeToUpdate = types.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73") as any;
if (typeToUpdate) {
  // Scalar fields
  typeToUpdate.name = "Base Modified";
  typeToUpdate.description = "Updated base type description";
  typeToUpdate.virtual = true;
  typeToUpdate.unit = "cm";
  // isAbstract: skipped (original is undefined)
  typeToUpdate.stock = 42;
  // folder: skipped (original is undefined)
  // location: skipped (original is undefined)
  typeToUpdate.icon = "updated-base-icon.svg";
  typeToUpdate.image = "updated-base-image.png";
  typeToUpdate.authors = [{ guid: kitBefore.authors![0].guid }];
  // concepts: skipped (original is undefined)
  // parent: skipped (original is undefined)

  // MODELS nested: remove 1, update 1, add 1
  const modelsToRemoveIdx = typeToUpdate.models.findIndex((m: any) => m.guid === "c1b9624e-51ed-459c-8f9c-497e39768cc3");
  if (modelsToRemoveIdx >= 0) typeToUpdate.models.splice(modelsToRemoveIdx, 1);

  const modelToUpdate = typeToUpdate.models.find((m: any) => m.guid === "c2aded58-7995-4ea5-b990-91bd44482bcb");
  if (modelToUpdate) {
    modelToUpdate.name = "base_1to500_modified";
    // description: skipped (original is undefined)
    modelToUpdate.tags = [{ guid: kitBefore.tags![0].guid }];
    // attributes: skipped (original is undefined)
  }

  typeToUpdate.models.push({
    guid: newGuid(),
    name: "new-test-model",
    file: { guid: kitBefore.files![1].guid },
    tags: [{ guid: kitBefore.tags![1].guid }],
    description: "A newly added model",
    attributes: [],
  });

  // CONNECTORS nested: remove 1, update 1, add 1
  const connToRemoveIdx = typeToUpdate.connectors.findIndex((c: any) => c.guid === "c5465220-19ba-4443-8f1d-617c832dd13c");
  if (connToRemoveIdx >= 0) typeToUpdate.connectors.splice(connToRemoveIdx, 1);

  const connToUpdate = typeToUpdate.connectors.find((c: any) => c.guid === "d25a91ed-b124-4e5b-8e7e-af832541c953");
  if (connToUpdate) {
    connToUpdate.name = "c1-modified";
    connToUpdate.t = 0.75;
    connToUpdate.point = { x: 10.0, y: 20.0, z: 30.0 };
    connToUpdate.direction = { x: 1.0, y: 0.0, z: 0.0 };
    connToUpdate.description = "Updated connector description";
    connToUpdate.port = { guid: kitBefore.ports![2].guid };
    connToUpdate.mandatory = true;
    // props: skipped (original is undefined)
    connToUpdate.attributes = [...(connToUpdate.attributes || []), { guid: newGuid(), key: "conn.meta", value: "test", definition: "Connector attribute" }];
  }

  typeToUpdate.connectors.push({
    guid: newGuid(),
    name: "new-connector",
    point: { x: 1, y: 1, z: 1 },
    direction: { x: 0, y: 1, z: 0 },
    t: 0.5,
    mandatory: true,
    description: "Newly added connector",
    port: { guid: kitBefore.ports![3].guid },
    props: [],
    attributes: [],
  });

  // props: skipped (original is undefined)

  // TYPE ATTRIBUTES
  typeToUpdate.attributes = [{ guid: newGuid(), key: "type.meta", value: "test-value", definition: "Type attribute" }];
}

// ADDED: New type with all features
types.push({
  guid: newGuid(),
  name: "New Comprehensive Type",
  virtual: true,
  unit: "mm",
  isAbstract: false,
  description: "A new type added for testing",
  icon: "new-type-icon.svg",
  image: "new-type-image.png",
  stock: 100,
  location: { guid: newGuid() },
  folder: "new-type-folder",
  parent: { guid: types[0].guid },
  authors: [{ guid: kitBefore.authors![0].guid }],
  concepts: [{ guid: kitBefore.concepts![1].guid }],
  connectors: [
    {
      guid: newGuid(),
      name: "test-connector",
      point: { x: 0, y: 0, z: 0 },
      direction: { x: 0, y: 0, z: 1 },
      t: 0,
      mandatory: false,
      description: "Test connector",
      port: { guid: kitBefore.ports![0].guid },
      props: [{ guid: newGuid(), quality: { guid: newGuid() }, value: "100", attributes: [] }],
      attributes: [{ guid: newGuid(), key: "new.conn.attr", value: "val" }],
    },
  ],
  models: [
    {
      guid: newGuid(),
      name: "new-model",
      file: { guid: kitBefore.files![0].guid },
      tags: [{ guid: kitBefore.tags![0].guid }],
      description: "New model",
      attributes: [{ guid: newGuid(), key: "new.model.attr", value: "val" }],
    },
  ],
  props: [{ guid: newGuid(), quality: { guid: newGuid() }, value: "yes", attributes: [] }],
  attributes: [{ guid: newGuid(), key: "new.type.attr", value: "new-val", definition: "New attribute" }],
  createdAt: "2025-12-23T13:40:58.750Z",
  updatedAt: "2025-12-23T13:40:58.750Z",
} as any);

// ============================================================
// DESIGNS - removed, updated (all fields + nested), added
// ============================================================
const designs = kitAfter.designs!;

// REMOVED: Nakagin Capsule Tower
const designToRemoveIdx = designs.findIndex((d: any) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef");
if (designToRemoveIdx >= 0) designs.splice(designToRemoveIdx, 1);

// UPDATED: Capsule Dream
const designToUpdate = designs.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8") as any;
if (designToUpdate) {
  // Scalar fields
  designToUpdate.name = "Capsule Dream Modified";
  designToUpdate.description = "Updated design description";
  designToUpdate.unit = "cm";
  // isAbstract: skipped
  // folder: skipped
  // canScale: skipped
  // canMirror: skipped
  // location: skipped
  designToUpdate.icon = "updated-design-icon.svg";
  designToUpdate.image = "updated-design-image.png";
  // concepts: skipped

  const pieces = designToUpdate.pieces ?? [];
  const connections = designToUpdate.connections ?? [];

  // PIECES nested: remove 1, update 1 (safe fields only), add 1
  if (pieces.length >= 2) {
    const pieceRemoved = pieces.splice(0, 1)[0];

    // Update next piece (only fields that already exist or safe replacements)
    const pieceToUpdate = pieces[0];
    pieceToUpdate.name = "piece-modified";
    pieceToUpdate.description = "Updated piece description";
    pieceToUpdate.isHidden = true;
    pieceToUpdate.isLocked = true;
    // color: skipped (original is undefined)
    // scale: skipped (original is undefined)
    // props: skipped (original is undefined)
    // attributes: skipped (original is undefined)

    // Remove connections referencing removed piece
    designToUpdate.connections = connections.filter((c: any) => c.connected?.piece?.guid !== pieceRemoved.guid && c.connecting?.piece?.guid !== pieceRemoved.guid);
  }

  // Add new piece WITH center, mirrorPlane, plane (all features)
  pieces.push({
    guid: newGuid(),
    name: "new-test-piece",
    type: { guid: types[1].guid },
    plane: {
      origin: { x: 5, y: 5, z: 5 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    },
    center: { u: 3, v: 7 },
    mirrorPlane: {
      origin: { x: 0, y: 0, z: 0 },
      xAxis: { x: 1, y: 0, z: 0 },
      yAxis: { x: 0, y: 1, z: 0 },
    },
    scale: 1.5,
    description: "New piece for testing",
    props: [],
    attributes: [],
  } as any);
  designToUpdate.pieces = pieces;

  // CONNECTIONS nested: remove 1, update 1 (delta fields), add 1 (with designPiece)
  const conns = designToUpdate.connections ?? [];
  if (conns.length >= 2) {
    conns.splice(0, 1); // remove first

    // Update second (now first) with delta fields only (no designPiece on existing)
    const connToUpdate = conns[0];
    connToUpdate.gap = 100;
    connToUpdate.shift = 50;
    connToUpdate.rise = 25;
    connToUpdate.rotation = 45;
    connToUpdate.turn = 90;
    connToUpdate.tilt = 15;
    connToUpdate.u = 0.5;
    connToUpdate.v = 0.75;
    connToUpdate.description = "Updated connection description";
    // attributes: skipped (original is undefined)
  }

  // Add new connection WITH designPiece (exercises this feature on added entity)
  const piecesForConn = designToUpdate.pieces;
  if (piecesForConn.length >= 2) {
    conns.push({
      guid: newGuid(),
      connected: {
        piece: { guid: piecesForConn[0].guid },
        designPiece: { guid: piecesForConn[1].guid },
        connector: typeToUpdate?.connectors?.[0] ? { guid: typeToUpdate.connectors[0].guid } : undefined,
      },
      connecting: {
        piece: { guid: piecesForConn[piecesForConn.length - 1].guid },
        designPiece: { guid: piecesForConn[0].guid },
      },
      gap: 10,
      shift: 5,
      rise: 2,
      rotation: 0,
      turn: 0,
      tilt: 0,
      u: 0,
      v: 0,
      description: "New test connection",
      attributes: [],
    } as any);
  }
  designToUpdate.connections = conns;

  // stats: skipped (original is undefined)

  // props: skipped (original is undefined)

  // layers: skipped (original is undefined)

  // activeLayer: skipped (original is undefined)

  // groups: skipped (original is undefined)

  // AUTHORS
  const designAuthors = designToUpdate.authors ?? [];
  designAuthors.push({ guid: kitBefore.authors![0].guid });
  designToUpdate.authors = designAuthors;

  // DESIGN ATTRIBUTES
  const designAttrs = designToUpdate.attributes ?? [];
  designAttrs.push({
    guid: newGuid(),
    key: "design.new.attr",
    value: "new-design-attr",
    definition: "New design attribute",
  });
  designToUpdate.attributes = designAttrs;
}

// ADDED: New design with all features
const newDesignPiece1Guid = newGuid();
const newDesignPiece2Guid = newGuid();
const newDesignLayerGuid = newGuid();
designs.push({
  guid: newGuid(),
  name: "New Comprehensive Design",
  unit: "mm",
  description: "New design for testing",
  icon: "new-design-icon.svg",
  image: "new-design-image.png",
  isAbstract: false,
  canScale: true,
  canMirror: false,
  activeLayer: { guid: newDesignLayerGuid },
  folder: "new-design-folder",
  location: { guid: newGuid() },
  concepts: [{ guid: kitBefore.concepts![0].guid }],
  pieces: [
    {
      guid: newDesignPiece1Guid,
      name: "new-design-piece-1",
      type: { guid: types[0].guid },
      plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
      scale: 1,
      props: [],
      attributes: [],
    },
    {
      guid: newDesignPiece2Guid,
      name: "new-design-piece-2",
      type: { guid: types[1].guid },
      plane: { origin: { x: 10, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
      scale: 1,
      props: [],
      attributes: [],
    },
  ],
  connections: [
    {
      guid: newGuid(),
      connected: { piece: { guid: newDesignPiece1Guid } },
      connecting: { piece: { guid: newDesignPiece2Guid } },
      gap: 0,
      description: "New design connection",
      attributes: [],
    },
  ],
  stats: [],
  props: [{ guid: newGuid(), quality: { guid: newGuid() }, value: "test", attributes: [] }],
  layers: [{ guid: newDesignLayerGuid, path: "default", attributes: [] }],
  groups: [],
  authors: [{ guid: kitBefore.authors![0].guid }],
  attributes: [{ guid: newGuid(), key: "new.design.attr", value: "val" }],
  createdAt: "2025-12-23T13:40:58.750Z",
  updatedAt: "2025-12-23T13:40:58.750Z",
} as any);

// ============================================================
// TAGS - removed, updated, added
// ============================================================
const tags = kitAfter.tags!;
const tagRemoveIdx = tags.findIndex((t: any) => t.guid === "212dec6a-b3ba-42e9-a624-b097176dbaa6");
if (tagRemoveIdx >= 0) tags.splice(tagRemoveIdx, 1);

const tagToUpdate = tags.find((t: any) => t.guid === "348efbfa-e275-4dda-9f66-229917c6b4ab") as any;
if (tagToUpdate) {
  tagToUpdate.name = "model/vnd.3dm Modified";
  // description, icon, attributes: skipped (original is undefined)
}

tags.push({ guid: newGuid(), name: "New Test Tag", description: "New tag", icon: "test-tag.svg", attributes: [{ guid: newGuid(), key: "tag.source", value: "generator" }] } as any);

// ============================================================
// CONCEPTS - removed, updated, added
// ============================================================
const concepts = kitAfter.concepts!;
const conceptRemoveIdx = concepts.findIndex((c: any) => c.guid === "019adc5e-40ee-789d-a877-5baef7bc79fa");
if (conceptRemoveIdx >= 0) concepts.splice(conceptRemoveIdx, 1);

const conceptToUpdate = concepts.find((c: any) => c.guid === "019adc5e-9205-7364-a213-66fda12e5120") as any;
if (conceptToUpdate) {
  conceptToUpdate.name = "organic-city Modified";
  // description, icon, attributes: skipped (original is undefined)
}

concepts.push({ guid: newGuid(), name: "New Concept", description: "New", icon: "concept.svg", attributes: [] } as any);

// ============================================================
// PORTS - removed, updated, added
// ============================================================
const ports = kitAfter.ports!;
const portRemoveIdx = ports.findIndex((p: any) => p.guid === "019ab243-21f3-7380-93c6-994a9a023448");
if (portRemoveIdx >= 0) ports.splice(portRemoveIdx, 1);

const portToUpdate = ports.find((p: any) => p.guid === "019ab243-21f3-7380-93c6-9e678ad2f321") as any;
if (portToUpdate) {
  portToUpdate.name = "core circular top Modified";
  // description, icon: skipped (original is undefined)
  portToUpdate.compatiblePorts = [{ guid: kitBefore.ports![2].guid }, { guid: kitBefore.ports![3].guid }];
  // attributes: skipped (original is undefined)
}

ports.push({ guid: newGuid(), name: "New Port", description: "New", icon: "port.svg", compatiblePorts: [{ guid: kitBefore.ports![4].guid }], attributes: [] } as any);

// qualities: skipped (original is undefined - qualities are on ADDED type/design instead)

// ============================================================
// AUTHORS - updated + added
// ============================================================
const authors = kitAfter.authors!;
const authorToUpdate = authors.find((a: any) => a.guid === "e3d5369e-b103-42a8-960a-7960c75f0f88") as any;
if (authorToUpdate) {
  authorToUpdate.name = "Ueli Saluz Modified";
  authorToUpdate.email = "modified@compose-tech.org";
  // attributes: skipped (original is undefined)
}

authors.push({ guid: newGuid(), name: "Test Author", email: "test@example.com", attributes: [] } as any);

// ============================================================
// FILES - removed, updated, added
// ============================================================
const files = kitAfter.files!;
const fileRemoveIdx = files.findIndex((f: any) => f.guid === "457d2061-ac4b-4317-8563-ba41afffd149");
if (fileRemoveIdx >= 0) files.splice(fileRemoveIdx, 1);

const fileToUpdate = files.find((f: any) => f.guid === "77e02ef4-e37e-41dd-80f6-00889cfcabb4") as any;
if (fileToUpdate) {
  fileToUpdate.name = "updated-base_1to500.3dm";
  fileToUpdate.mime = "application/x-rhino3d-modified";
  // remote, size, hash: skipped (original is undefined)
  fileToUpdate.folder = { guid: kitBefore.folders![0].guid };
}

files.push({
  guid: newGuid(),
  name: "new-file.txt",
  mime: "text/plain",
  remote: "https://example.com/new-file.txt",
  size: 42,
  hash: "sha256:newfilehash",
  folder: { guid: kitBefore.folders![0].guid },
  createdAt: "2025-12-23T13:40:58.750Z",
  updatedAt: "2025-12-23T13:40:58.750Z",
} as any);

// ============================================================
// FOLDERS - updated + added
// ============================================================
const folders = kitAfter.folders!;
const folderToUpdate = folders.find((f: any) => f.guid === "019adc83-0113-75e0-90b2-9d0912f1d60f") as any;
if (folderToUpdate) {
  folderToUpdate.name = "representations-modified";
  // description, attributes: skipped (original is undefined)
}

folders.push({
  guid: newGuid(),
  name: "test-folder",
  description: "New folder",
  parent: { guid: kitBefore.folders![0].guid },
  attributes: [],
  createdAt: "2025-12-23T13:40:58.750Z",
} as any);

// kit attributes: skipped (original is undefined)

// ============================================================
// COMPUTE AND VERIFY
// ============================================================
console.log("[DEBUG] Computing diff...");
const diff = getKitDiff(kitBefore, kitAfter);
console.log("[DEBUG] Diff keys:", Object.keys(diff));

const inverseDiff = inverseKitDiff(kitBefore, diff);

const appliedForward = applyKitDiff(kitBefore, diff);
const forwardOk = areKitsEqual(appliedForward, kitAfter);
console.log("[DEBUG] Forward:", forwardOk);

const appliedInverse = applyKitDiff(kitAfter, inverseDiff);
const inverseOk = areKitsEqual(appliedInverse, kitBefore);
console.log("[DEBUG] Inverse:", inverseOk);
if (!inverseOk) {
  // Compare top-level fields
  const bKeys = Object.keys(kitBefore) as string[];
  const iKeys = Object.keys(appliedInverse) as string[];
  const allKeys = [...new Set([...bKeys, ...iKeys])];
  for (const k of allKeys) {
    const bv = JSON.stringify((kitBefore as any)[k]);
    const iv = JSON.stringify((appliedInverse as any)[k]);
    if (bv !== iv) {
      console.log(`[DEBUG-INV-KIT] Key "${k}" differs (before=${bv?.length} chars, inverse=${iv?.length} chars):`);
      if ((bv?.length ?? 0) < 500 && (iv?.length ?? 0) < 500) {
        console.log(`  before:  ${bv}`);
        console.log(`  inverse: ${iv}`);
      } else {
        console.log(`  before:  ${bv?.slice(0, 300)}`);
        console.log(`  inverse: ${iv?.slice(0, 300)}`);
      }
    }
  }
}

const recomputedDiff = getKitDiff(kitBefore, appliedForward);
const diffRoundtripOk = areKitDiffsEqual(recomputedDiff, diff);
console.log("[DEBUG] Diff roundtrip:", diffRoundtripOk);

const recomputedInverse = inverseKitDiff(kitBefore, diff);
const inverseRoundtripOk = areKitDiffsEqual(recomputedInverse, inverseDiff);
console.log("[DEBUG] Inverse roundtrip:", inverseRoundtripOk);

if (!forwardOk || !inverseOk || !diffRoundtripOk || !inverseRoundtripOk) {
  console.error("[DEBUG] VERIFICATION FAILED!");
  process.exit(1);
}

// ============================================================
// WRITE OUTPUT FILES
// ============================================================
writeFileSync(join(ASSETS_DIR, "diff_kit_metabolism.json"), JSON.stringify(diff, null, 2) + "\n");
writeFileSync(join(ASSETS_DIR, "diff_kit_metabolism_inverted.json"), JSON.stringify(inverseDiff, null, 2) + "\n");
writeFileSync(join(ASSETS_DIR, "kit_metabolism_diffed.json"), JSON.stringify(kitAfter, null, 2) + "\n");
console.log("[DEBUG] All files written successfully!");
