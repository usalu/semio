#!/usr/bin/env bun
/** @emoji 🧪️ Round-trip verification of the writer wasm plugin (WRITER-REACT-PARITY). Run: bun .repo/🎫️/26/07/10/WRITER-REACT-PARITY/wasm-verify.ts */
import { loadPluginModule } from "@semio-tech/framework-core";

const MODULE_URL = "file:///Users/ueli/Documents/semio/framework/product/os/dev/plugin-modules/writer/writer_plugin.js";

type Json = Record<string, unknown>;

function cmd(command: string, args?: Json) {
  return JSON.stringify({ controllerId: "writer-play", command, args });
}

async function main() {
  const handle = await loadPluginModule("writer", MODULE_URL);
  console.log(
    "[DEBUG] manifest apps:",
    handle.manifest.apps.map((a) => a.id),
  );
  console.log(
    "[DEBUG] manifest examples:",
    handle.manifest.examples.map((e) => e.id),
  );
  if (!handle.manifest.examples.some((e) => e.id === "dag.jack")) throw new Error("missing dag.jack example");
  const jackExample = handle.manifest.examples.find((e) => e.id === "jack");
  if (!jackExample) throw new Error("missing jack example");

  const instanceId = await handle.createApp("writer-play");

  // Reproduces the reported bug: the host's example dropdown dispatches "setActiveExample", not "setDocumentJson".
  const emptyScene = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  if (((emptyScene.textEditor as Json).buffer as string) !== "") throw new Error("expected a fresh instance to start empty");
  await handle.handleCommand(instanceId, cmd("setActiveExample", { exampleId: "jack" }), {});
  const afterExampleSelect = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  const bufferAfterSelect = (afterExampleSelect.textEditor as Json).buffer as string;
  if (!bufferAfterSelect.includes("MATCH")) throw new Error(`setActiveExample("jack") did not load the fixture; buffer: ${JSON.stringify(bufferAfterSelect)}`);
  console.log("[DEBUG] setActiveExample(jack) loaded buffer:", bufferAfterSelect);
  await handle.handleCommand(instanceId, cmd("setActiveExample", { exampleId: "empty" }), {});
  const afterEmptySelect = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  if (((afterEmptySelect.textEditor as Json).buffer as string) !== "") throw new Error("setActiveExample(empty) did not clear the buffer");
  console.log("[DEBUG] setActiveExample(empty) round-trip OK");

  await handle.handleCommand(instanceId, cmd("setDocumentJson", { json: jackExample.documentJson }), {});
  const scene1 = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  const textEditor1 = scene1.textEditor as Json;
  for (const field of ["placeholdersJson", "selectableSpansJson", "newlineGatesJson", "completionsJson"]) {
    if (!(field in textEditor1)) throw new Error(`missing scene field: ${field}`);
  }
  console.log("[DEBUG] initial scene field keys:", Object.keys(textEditor1));

  const text = textEditor1.buffer as string;
  const varOffset = text.indexOf("a:Piece");
  if (varOffset < 0) throw new Error("fixture missing expected variable");

  await handle.handleCommand(instanceId, cmd("textSelect", { start: varOffset, end: varOffset }), {});
  const scene2 = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  const textEditor2 = scene2.textEditor as Json;
  const occurrences = JSON.parse(textEditor2.occurrencesJson as string) as { selection: string; hover: string };
  const selectionOccurrences = JSON.parse(occurrences.selection) as unknown[];
  if (selectionOccurrences.length !== 3) throw new Error(`expected 3 occurrences, got ${selectionOccurrences.length}`);
  if (!textEditor2.extraCaretsJson) throw new Error("missing extraCaretsJson at collapsed variable caret");
  if (!textEditor2.renameJson) throw new Error("missing renameJson at collapsed variable caret");
  console.log("[DEBUG] renameJson:", textEditor2.renameJson);

  const renameInfo = JSON.parse(textEditor2.renameJson as string) as { name: string; occurrences: { start: number; end: number }[] };
  await handle.handleCommand(instanceId, cmd("commitRename", { occurrences: renameInfo.occurrences, text: "piece" }), {});
  const scene3 = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  const buffer3 = (scene3.textEditor as Json).buffer as string;
  const pieceCount = (buffer3.match(/piece/g) ?? []).length;
  if (pieceCount !== 3) throw new Error(`expected 3 occurrences of "piece" after rename, got ${pieceCount}`);
  console.log("[DEBUG] renamed buffer:", buffer3);

  await handle.handleCommand(instanceId, cmd("engagementSubmit", { value: "font 16" }), {});
  const measures = await handle.windowMeasures(instanceId, {});
  const mainMeasures = measures["writer-main"] as Json[];
  const fontMeasure = mainMeasures?.find((m) => m.id === "writer-font-size-measure");
  if (fontMeasure?.value !== 16) throw new Error(`expected font size 16, got ${JSON.stringify(fontMeasure)}`);
  console.log("[DEBUG] font measure after engagementSubmit:", fontMeasure);

  const engagements = await handle.windowEngagements(instanceId, {});
  const mainEngagement = engagements["writer-main"] as Json;
  const placeholder = (mainEngagement?.input as Json | undefined)?.placeholder;
  if (typeof placeholder !== "string" || !placeholder.includes("Format")) throw new Error(`unexpected engagement input placeholder: ${placeholder}`);
  console.log("[DEBUG] engagement input placeholder:", placeholder);

  const tools = await handle.tools(instanceId, {});
  const toolsJson = JSON.stringify(tools);
  if (!toolsJson.includes("writer-format") || !toolsJson.includes("writer-lint")) throw new Error("missing format/lint tools");
  console.log("[DEBUG] tools:", toolsJson);

  const treeNode = (await handle.render(instanceId, "writer.play.document", {})) as Json;
  const sections = treeNode.sections as Json[] | undefined;
  const rootId = (sections?.[0]?.items as Json[] | undefined)?.[0]?.id as string | undefined;
  if (!rootId) throw new Error("no root ast id found in document tree");

  await handle.handleCommand(instanceId, cmd("setAstHover", { id: rootId }), {});
  const scene4 = (await handle.render(instanceId, "writer.play.main", {})) as Json;
  const hoverJson4 = JSON.parse((scene4.textEditor as Json).hoverJson as string) as { start: number; end: number };
  console.log("[DEBUG] hoverJson after setAstHover:", hoverJson4);
  if (hoverJson4.start !== 0) throw new Error(`expected hover span to start at 0 for root node, got ${JSON.stringify(hoverJson4)}`);

  const treeNode2 = (await handle.render(instanceId, "writer.play.document", {})) as Json;
  if (JSON.stringify(treeNode2.highlightedIds ?? []) !== JSON.stringify([rootId])) {
    throw new Error(`expected tree highlightedIds to include hovered root, got ${JSON.stringify(treeNode2.highlightedIds)}`);
  }

  console.log("[DEBUG] ALL CHECKS PASSED");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
