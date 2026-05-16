#!/usr/bin/env bun
/**
 * @emoji 🧷 Inlines generated Neo4j fragments into `migrations.cypher` (operation `Command` relabel/merge/dedupe/reparent + imperative renames + `Data` argument kit from golden `*Input`).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = import.meta.dir;
const migPath = join(dir, "migrations.cypher");
const relabelPath = join(dir, "relabel-rename-operation-commands.cypher.fragment");
const mergePath = join(dir, "merge-operation-classes.cypher.fragment");
const inputSurfacePath = join(dir, "merge-command-input-surfaces.cypher.fragment");
const dedupePath = join(dir, "dedupe-duplicate-operation-commands.cypher.fragment");
const reparentPath = join(dir, "reparent-operation-ownership.cypher.fragment");

const mig = readFileSync(migPath, "utf8");
const relabel = readFileSync(relabelPath, "utf8").trimEnd();
const merge = readFileSync(mergePath, "utf8").trimEnd();
const inputFrag = readFileSync(inputSurfacePath, "utf8").trimEnd();
const reparent = readFileSync(reparentPath, "utf8").trimEnd();

const mergeBody = merge
  .split("\n")
  .filter((l) => !l.startsWith("// Generated"))
  .join("\n")
  .trim();

const inputBody = inputFrag
  .split("\n")
  .filter((l) => !l.startsWith("// Generated"))
  .join("\n")
  .trim();

const reparentBody = reparent
  .split("\n")
  .filter((l) => !l.startsWith("// Generated"))
  .join("\n")
  .trim();

const dedupeBlock = readFileSync(dedupePath, "utf8").trimEnd();

const relabelBlock = [
  "//#region RelabelRenameOperationCommands",
  relabel,
  "//#endregion RelabelRenameOperationCommands",
  "",
  "//#region MergeOperationConcreteCommands",
  mergeBody,
  "//#endregion MergeOperationConcreteCommands",
  "",
  dedupeBlock,
  "",
  "//#region MergeCommandInputSurfaces",
  inputBody,
  "//#endregion MergeCommandInputSurfaces",
].join("\n");

const reCombinedWithInput = /\/\/#region RelabelRenameOperationCommands[\s\S]*?\/\/#endregion MergeCommandInputSurfaces/;
const reCombinedMergeOnly = /\/\/#region RelabelRenameOperationCommands[\s\S]*?\/\/#endregion MergeOperationConcreteCommands/;
const reMergeLegacy = /\/\/#region MergeOperationConcreteClasses[\s\S]*?\/\/#endregion MergeOperationConcreteClasses/;

let out = mig;
if (reCombinedWithInput.test(out)) {
  out = out.replace(reCombinedWithInput, relabelBlock);
} else if (reCombinedMergeOnly.test(out)) {
  out = out.replace(reCombinedMergeOnly, relabelBlock);
} else if (reMergeLegacy.test(out)) {
  out = out.replace(reMergeLegacy, relabelBlock);
} else {
  throw new Error("[stitch-migrations] No stitchable RelabelRename / Merge / Input block found");
}

const newReparent = [
  "//#region ReparentOperationCommandsUnderOwnerOperationModules",
  "// Each domain `Class` / `Interface` (Piece, Quality, …) `OWNS` `Module(operation)` which `OWNS` concrete operation `Command` nodes (golden `Operation` subtypes). `Module`→`Class`/`Interface`/`Scalar` shell uses `PART_OF` (see migrations + Neo4j SDL).",
  reparentBody,
  "//#endregion ReparentOperationCommandsUnderOwnerOperationModules",
].join("\n");

const reReparentNew = /\/\/#region ReparentOperationCommandsUnderOwnerOperationModules[\s\S]*?\/\/#endregion ReparentOperationCommandsUnderOwnerOperationModules/;
const reReparentLegacy = /\/\/#region ReparentOperationClassesUnderOwnerOperationModules[\s\S]*?\/\/#endregion ReparentOperationClassesUnderOwnerOperationModules/;

if (reReparentNew.test(out)) {
  out = out.replace(reReparentNew, newReparent);
} else if (reReparentLegacy.test(out)) {
  out = out.replace(reReparentLegacy, newReparent);
} else {
  throw new Error("[stitch-migrations] Reparent operation region not found");
}

writeFileSync(migPath, out, "utf8");
console.log(`[stitch-migrations] updated ${migPath}`);
