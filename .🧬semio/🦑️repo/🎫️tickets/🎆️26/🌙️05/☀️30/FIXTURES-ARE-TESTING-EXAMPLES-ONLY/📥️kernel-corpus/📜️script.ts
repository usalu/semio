import { readFileSync, writeFileSync, unlinkSync, mkdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
const ts = (await import(join(root, "node_modules/typescript/lib/typescript.js"))).default;
const ticket = dirname(import.meta.dir);
const target = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts";
const source = readFileSync(join(root, target), "utf8");
const tree = ts.createSourceFile(target, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const changes: { start: number; end: number; text: string }[] = [];
const removed: string[] = [];
const drop = (node: import("typescript").Node) => { changes.push({ start: node.getStart(tree), end: node.end, text: "" }); removed.push(node.getText(tree)); };
const corpusSchemas = new Set(["writerDeferredWakeSchema", "mountedPoolUseSchema", "mapSchema", "preparationSchema", "pagedSchema", "pagedOwnerSchema"]);
const corpusValidators = new Set(["validateWriterDeferredWake", "validateMountedPoolUse", "validateMap", "validatePreparation", "validatePaged", "validatePagedOwner"]);
const visit = (node: import("typescript").Node) => {
  if (ts.isBlock(node)) {
    const validators = new Set<string>();
    for (const statement of node.statements) if (ts.isVariableStatement(statement)) {
      for (const variable of statement.declarationList.declarations) if (ts.isIdentifier(variable.name) && variable.initializer) {
        const name = variable.name.text, init = variable.initializer.getText(tree);
        if (/^ownedExport\(/u.test(init) || corpusSchemas.has(name) || corpusValidators.has(name) || (node.parent && ts.isMethodDeclaration(node.parent) && ts.isClassDeclaration(node.parent.parent) && ["PagedHistoryStackScript", "RetainedCloneCheckScript"].includes(node.parent.parent.name?.text ?? "") && ["schema", "admit", "validate"].includes(name))) {
          drop(statement);
          if (/^ownedExport\(/u.test(init) || corpusValidators.has(name) || ["admit", "validate"].includes(name)) validators.add(name);
        }
      }
    }
    for (const statement of node.statements) if (ts.isExpressionStatement(statement) && ts.isCallExpression(statement.expression) && statement.expression.expression.getText(tree) === "assert") {
      const first = statement.expression.arguments[0];
      if (first && ts.isCallExpression(first) && validators.has(first.expression.getText(tree))) drop(statement);
    }
  }
  ts.forEachChild(node, visit);
};
visit(tree);
for (const edit of [...changes].sort((a, b) => b.start - a.start)) if (changes.some(other => other !== edit && other.start === edit.start)) throw Error("Duplicate edit");
let result = source;
for (const edit of changes.sort((a, b) => b.start - a.start)) result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);
result = result.replace(/\/\/#region 🧬️OwnedSchemaExports[\s\S]*?\/\/#endregion 🧬️OwnedSchemaExports\n/u, "");
result = result.replace('import Ajv2020 from "ajv/dist/2020.js";\n', "");
result = result.replace('const branchSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🌿️branch-provenance/🔣️.json"), "utf8"));', 'const editSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json"), "utf8"));');
result = result.replace('assert(branchAjv.compile(branchSchema)(branchFixture));\n    const editCheck = branchAjv.compile(branchSchema.$defs.Edit);', 'const editCheck = branchAjv.compile({ $defs: editSchema.$defs, $ref: "#/$defs/Edit" });');
result = result.replaceAll("AJV=1", "behavior-oracles=retained").replace("throughput fixture valid (ThroughputV1, AJV):", "throughput example:");
result = result.replaceAll("its schema admits", "describes").replace("One `choice` arm of `🧬️retained-clone/🧫️fixtures/📦️nested/🧬️schema/🔣️.json` (`#/$defs/choice`).", "One example payload for retained-clone choice behavior.").replace("Validates the retained-clone resource contract and neutral corpus with Ajv and the platform structured-clone oracle.", "Verifies retained-clone behavior against the platform structured-clone oracle.");
if (ts.createSourceFile(target, result, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS).parseDiagnostics.length) throw Error("Edited source cannot parse");
writeFileSync(join(root, target), result);
const db = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/";
const removedDocs: string[] = [];
for (const owner of ["⚙️engine", "📝️wal", "🗄️storage", "🗜️compact", "🗿️artifact"]) { const path = db + owner + "/🧬️schema/🔣️.json"; unlinkSync(join(root, path)); removedDocs.push(path); }
const writer = db + "🗄️storage/🔐️writer/🧬️schema/🔣️.json";
const writerDoc = JSON.parse(readFileSync(join(root, writer), "utf8"));
writerDoc.$defs = { WalWriterFenceMutationV1: writerDoc.$defs.WalWriterFenceMutationV1 };
writeFileSync(join(root, writer), JSON.stringify(writerDoc, null, 2) + "\n");
const vcs = "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🧬️schema/🔣️.json";
unlinkSync(join(root, vcs)); removedDocs.push(vcs);
const input = JSON.parse(readFileSync(join(ticket, "📥️paged-history-schema-input.json"), "utf8"));
const edit = input["🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🧬️schema/🌿️branch-provenance/🔣️.json"].$defs.Edit;
edit.properties.sequenceNumber = { type: "integer", minimum: -2147483648, maximum: 2147483647 };
for (const key of ["actor", "verb", "finishedAt"]) edit.properties[key] = { type: ["string", "null"] };
edit.properties.mutationMeta = { type: "array", items: { type: "object" } };
const canonical = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json";
mkdirSync(dirname(join(root, canonical)), { recursive: true });
writeFileSync(join(root, canonical), JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", $id: "https://json.schemas.assets.semio-tech.com/replication/mutation/schema.json", title: "Replication Mutation Contracts", $defs: { Edit: edit } }, null, 2) + "\n");
writeFileSync(join(ticket, "📥️kernel-corpus-closure-ledger.json"), JSON.stringify([target, writer, canonical, ...removedDocs], null, 2) + "\n");
writeFileSync(join(ticket, "📓️kernel-corpus-closure-2026-10-06.md"), "# Kernel Corpus Closure\n\nBefore editing, actual isolated Nx paged-history command failed ENOENT on the retired corpus schema. The six DB owners and VCS history visibility published fixed test envelopes, not runtime schemas. Removed only whole-corpus admission; all SQLite, SHA256, array/deque, structuredClone, byte/grant/lifecycle/native predicates remain. Preserved writer operation vocabulary. Extracted the real generic Edit wire contract into neutral replication/mutation, where Edit<Op> lives; branch vectors validate actual edits with Ajv and native FromValue. The DB artifact record was a test summary rather than the actual DurableOwnedGroupJournalRecordV1 wire contract, so it was not promoted as a fake domain schema.\n\nExact removed admission statements:\n\n" + removed.map(row => "```ts\n" + row + "\n```\n").join("\n"));
console.log("[DEBUG] kernel corpus closure removed-admissions=" + removed.length + " files=" + (removedDocs.length + 3));
