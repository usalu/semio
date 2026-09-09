import assert from "node:assert/strict";
import { cpSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { runInNewContext } from "node:vm";

const workspace = process.cwd(), require = createRequire(join(workspace, "package.json"));
const packageRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust";
if (process.argv[2] === "restore") {
  const root = process.argv[3], native = require("nx/src/native"), database = native.connectToNxDb(join(root, "database"));
  const contract = JSON.parse(readFileSync(join(root, "contract.json"), "utf8")), cache = new native.NxCache(join(root, "consumer"), join(root, "snapshot"), database);
  const sourcePath = join(root, "snapshot", contract.hash, contract.output), restoredPath = join(root, "consumer", contract.output), expected = readFileSync(sourcePath);
  try {
    for (let attempt = 0; attempt < 2; attempt++) {
      rmSync(restoredPath, { force: true });
      const bytes = cache.copyFilesFromCache({ code: 0, outputsPath: join(root, "snapshot", contract.hash) }, [contract.output]);
      assert.deepEqual(readFileSync(restoredPath), expected); console.log(`[DEBUG] Actual boot task ${contract.hash} restored ${bytes} bytes on attempt ${attempt + 1}`);
    }
    const text = readFileSync(restoredPath, "utf8"), ts = require("typescript"), source = ts.createSourceFile(restoredPath, text, ts.ScriptTarget.Latest, true);
    const names = new Set(["DEFAULT_HOST_VARIANT", "BOOT_FIELD_CAPACITY", "LOCATION_SEARCH_CAPACITY", "bounded", "bootDescriptor"]);
    const nodes = source.statements.filter((node: any) => ts.isFunctionDeclaration(node) ? names.has(node.name?.text) : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration: any) => names.has(declaration.name?.text)));
    assert.equal(nodes.length, names.size);
    const program = nodes.map((node: any) => node.getText(source)).join("\n") + "\nJSON.stringify(bootDescriptor())";
    for (const row of contract.cases) assert.deepEqual(JSON.parse(runInNewContext(program, { URLSearchParams, window: { location: { search: row.search } } })), row.expected);
    const syntax = spawnSync("node", ["--input-type=module", "--check"], { input: text, encoding: "utf8", timeout: 10000 }); assert.equal(syntax.status, 0, syntax.stderr);
    console.log("[DEBUG] Restored production boot bytes execute all selection fixtures in native Node and parse as a complete ES module PASS");
  } finally { native.closeDbConnection(database); }
} else {
  const { Database } = await import("bun:sqlite"), databasePath = readdirSync(join(workspace, ".nx/workspace-data")).find(name => name.endsWith(".db")); assert.ok(databasePath);
  const database = new Database(join(workspace, ".nx/workspace-data", databasePath), { readonly: true });
  const root = mkdtempSync(resolve(import.meta.dir, "../../🗑️generated/wgpu-boot-native-restore-"));
  try {
    const row = database.query("SELECT d.hash FROM task_details d JOIN cache_outputs c USING(hash) JOIN task_history h USING(hash) WHERE d.project=? AND d.target=? AND h.code=0 ORDER BY h.end DESC LIMIT 1").get("@semio-tech/framework-renderer-wgpu", "generate-browser-boot") as { hash: string } | null; assert.ok(row);
    cpSync(join(workspace, ".nx/cache", row.hash), join(root, "snapshot", row.hash), { recursive: true });
    const fixture = JSON.parse(readFileSync(resolve(workspace, packageRoot, "../../🚀️browser-boot/🧪️tests/⚡️cache-inputs/🔣️.json"), "utf8"));
    writeFileSync(join(root, "contract.json"), JSON.stringify({ hash: row.hash, output: fixture.generator.output.replace("{projectRoot}", packageRoot), cases: fixture.cases }));
  } finally { database.close(); }
  mkdirSync(join(root, "consumer")); mkdirSync(join(root, "database"));
  const result = spawnSync("node", [import.meta.filename, "restore", root], { cwd: workspace, encoding: "utf8", timeout: 30000 });
  writeFileSync(join(root, "restore.log"), result.stdout + result.stderr); process.stdout.write(result.stdout); process.stderr.write(result.stderr); assert.equal(result.status, 0);
}
