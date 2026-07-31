#!/usr/bin/env bun
/** 🔍️ Wasm command round-trip verification for note canvas gestures, undo, and shell chrome. */
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const moduleUrl = `file://${join(repoRoot, "framework/product/os/dev/plugin-modules/note/note_plugin.js")}`;
const { loadPluginModule } = await import(join(repoRoot, "framework/core/js/index.ts"));
const handle = await loadPluginModule("note", moduleUrl);
const instanceId = await handle.createApp("note-play");
const viewState = {};

await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "note-play", command: "setActiveExample", args: { exampleId: "semio" } }), viewState);
const composite = await handle.render(instanceId, "note.play.composite", viewState);
const noteCanvas = (composite as { noteCanvas?: Record<string, unknown> }).noteCanvas ?? {};
const semioDoc = JSON.parse((noteCanvas.documentJson as string) ?? "{}") as { blocks?: unknown[] };
console.log("[DEBUG] semio example blocks:", semioDoc.blocks?.length);
if (semioDoc.blocks?.length !== 3) throw new Error(`expected 3 blocks from semio example, got ${semioDoc.blocks?.length}`);

const beginEvents = JSON.stringify([
  { op: "addBlock", block: { kind: "ink", id: "verify-ink", name: "Ink", x: 10, y: 10, width: 1, height: 1, visible: true, locked: false, points: [], strokeWidth: 3, color: [0, 0, 0, 1] }, parentId: null, index: null },
]);
await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "note-play", command: "applyNoteEvents", args: { eventsJson: beginEvents, phase: "begin", selectIds: ["verify-ink"] } }), viewState);
const liveEvents = JSON.stringify([
  {
    op: "updateBlock",
    blockId: "verify-ink",
    block: {
      kind: "ink",
      id: "verify-ink",
      name: "Ink",
      x: 10,
      y: 10,
      width: 1,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [20, 0],
      ],
      strokeWidth: 3,
      color: [0, 0, 0, 1],
    },
  },
]);
await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "note-play", command: "applyNoteEvents", args: { eventsJson: liveEvents, phase: "live" } }), viewState);
await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "note-play", command: "applyNoteEvents", args: { eventsJson: "[]", phase: "commit" } }), viewState);
const afterGesture = (await handle.render(instanceId, "note.play.composite", viewState)) as { noteCanvas?: Record<string, unknown> };
const afterDoc = JSON.parse((afterGesture.noteCanvas?.documentJson as string) ?? "{}") as { blocks?: unknown[] };
console.log("[DEBUG] blocks after ink gesture:", afterDoc.blocks?.length);
if (afterDoc.blocks?.length !== 4) throw new Error(`expected 4 blocks after ink gesture, got ${afterDoc.blocks?.length}`);

await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "note-play", command: "undo" }), viewState);
const afterUndo = (await handle.render(instanceId, "note.play.composite", viewState)) as { noteCanvas?: Record<string, unknown> };
const undoDoc = JSON.parse((afterUndo.noteCanvas?.documentJson as string) ?? "{}") as { blocks?: unknown[] };
console.log("[DEBUG] blocks after single undo:", undoDoc.blocks?.length);
if (undoDoc.blocks?.length !== 3) throw new Error(`expected undo to remove the whole ink gesture in one step, got ${undoDoc.blocks?.length} blocks`);

const engagements = await handle.windowEngagements(instanceId, viewState);
console.log("[DEBUG] composite engagement status:", engagements["note-composite"]?.status?.map((row: { text: string }) => row.text).join(" | "));
if (!engagements["note-composite"]?.input) throw new Error("composite window engagement missing input");

const measures = await handle.windowMeasures(instanceId, viewState);
console.log("[DEBUG] composite measure groups:", measures["note-composite"]?.map((row: { label?: string }) => row.label).join(", "));
if (!measures["note-composite"]?.length) throw new Error("composite window measures missing");

const tools = await handle.tools(instanceId, viewState);
console.log("[DEBUG] toolbar tool count:", tools.length);
if (tools.length < 10) throw new Error(`expected at least 10 toolbar tools, got ${tools.length}`);

const manifest = handle.manifest;
const examples = manifest.examples.filter((row: { appId?: string }) => row.appId === "note-play");
console.log("[DEBUG] note examples:", examples.map((row: { label: string }) => row.label).join(", "));

console.log("[DEBUG] wasm-verify passed");
