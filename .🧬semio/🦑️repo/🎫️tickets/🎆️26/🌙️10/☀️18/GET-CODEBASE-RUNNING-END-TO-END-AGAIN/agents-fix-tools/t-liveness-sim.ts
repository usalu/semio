import fs from "node:fs";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { requireHostCapacityV1, resolveHostCapacityLanguageV1, readHostCapacityPolicyV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
const manifest = JSON.parse(fs.readFileSync("staged/manifest.json", "utf8"));
const schemaRow = manifest.files.find((r: any) => r.path.endsWith("host-capacity/🧬️schema/🔣️.json"));
const schema = JSON.parse(fs.readFileSync("staged/" + schemaRow.after, "utf8"));
const row = manifest.files.find((r: any) => r.path.endsWith("host-capacity/🎫️gate/🟦️.ts"));
const source = fs.readFileSync("staged/" + row.after, "utf8");
const tree = ts.createSourceFile("host-gate.ts", source, ts.ScriptTarget.Latest, true);
const selected = tree.statements.filter((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "withHostCapacityV1");
const code = ts.transpileModule(selected[0]!.getText(tree).replace(/^export /u, "") + "\nreturn withHostCapacityV1;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const protocol = schema.definitions.Protocol.const;
const ports = { protocol, schema, requireHostCapacityV1, resolveHostCapacityLanguageV1, prepare: () => { throw Error("must not prepare"); }, readJson: () => { throw Error("must not read"); } };
const run = new Function(...Object.keys(ports), code)(...Object.values(ports));
for (const environment of [{ [protocol.grantEnvironment]: "x" }, { [protocol.grantEnvironment]: "x", SEMIO_LANGUAGE: "de" }]) {
  try { await run({ directory: "/model", policy: { enabled: true }, request: { id: "aaaaaaaa1", operation: "run", priority: "batch", minimumJobs: 1, preferredJobs: 1, fixedMegabytes: 1, perJobMegabytes: 1, cpuPerJob: 1 }, control: {}, environment, now: () => 1, pid: 1, write: () => {} }, async () => {}); console.log("no throw"); }
  catch (error) { console.log(String((error as Error).message)); }
}
