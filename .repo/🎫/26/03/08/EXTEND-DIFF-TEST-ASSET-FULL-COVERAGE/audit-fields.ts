import { readFileSync } from "fs";
const kitRaw = JSON.parse(readFileSync("/workspaces/semio/compose/assets/compose/kit_metabolism.json", "utf-8"));

function showEntity(name: string, entity: any) {
  console.log(`\n=== ${name} ===`);
  console.log(`  Present keys: ${Object.keys(entity).join(", ")}`);
}

// Kit-level
console.log("=== KIT TOP-LEVEL ===");
const kitKeys = Object.keys(kitRaw);
console.log(`Present: ${kitKeys.join(", ")}`);

// Base type (277768b5)
const base = kitRaw.types.find((t: any) => t.guid === "277768b5-9220-4312-bf0d-ab82d9fb6a73");
showEntity("Type: Base", base);

// Base connectors
for (const c of base.connectors ?? []) {
  showEntity(`  Connector: ${c.guid.slice(0,8)} (${c.name})`, c);
}

// Base models
for (const m of base.models ?? []) {
  showEntity(`  Model: ${m.guid.slice(0,8)} (${m.name})`, m);
}

// Capsule Dream design (37ba7ec4)
const capsuleDream = kitRaw.designs.find((d: any) => d.guid === "37ba7ec4-9023-4be7-9ab6-e0ebc80007f8");
showEntity("Design: Capsule Dream", capsuleDream);

// First few pieces
for (const p of (capsuleDream.pieces ?? []).slice(0, 3)) {
  showEntity(`  Piece: ${p.guid.slice(0,8)} (${p.name})`, p);
}

// First few connections
for (const c of (capsuleDream.connections ?? []).slice(0, 3)) {
  showEntity(`  Connection: ${c.guid.slice(0,8)}`, c);
  showEntity(`    Connected side`, c.connected);
  showEntity(`    Connecting side`, c.connecting);
}

// Tags - first to update (212dec6a)
const tag = kitRaw.tags?.find((t: any) => t.guid === "212dec6a-b3ba-42e9-a624-b097176dbaa6");
if (tag) showEntity("Tag: " + tag.name, tag);

// Concepts - first to update (019adc5e-9205)
const concept = kitRaw.concepts?.find((c: any) => c.guid === "019adc5e-9205-7364-a213-66fda12e5120");
if (concept) showEntity("Concept: " + concept.name, concept);

// Ports - first to update (019ab243-21f3-7380-93c6-994a9a023448)
const port = kitRaw.ports?.find((p: any) => p.guid === "019ab243-21f3-7380-93c6-994a9a023448");
if (port) showEntity("Port: " + port.name, port);

// Files - first to update (77e02ef4)
const file = kitRaw.files?.find((f: any) => f.guid === "77e02ef4-e37e-41dd-80f6-00889cfcabb4");
if (file) showEntity("File: " + file.name, file);

// Folders - first to update (019adc83-0113)
const folder = kitRaw.folders?.find((f: any) => f.guid === "019adc83-0113-75e0-90b2-9d0912f1d60f");
if (folder) showEntity("Folder: " + folder.name, folder);

// Authors (e3d5369e)
const author = kitRaw.authors?.find((a: any) => a.guid === "e3d5369e-b103-42a8-960a-7960c75f0f88");
if (author) showEntity("Author: " + author.name, author);

// Qualities
const quality = (kitRaw.qualities ?? [])[0];
if (quality) showEntity("Quality: " + quality.name, quality);

// Attributes on kit
console.log("\n=== Kit attributes (first 3) ===");
for (const a of (kitRaw.attributes ?? []).slice(0, 3)) {
  showEntity(`  Attribute: ${a.key}`, a);
}
