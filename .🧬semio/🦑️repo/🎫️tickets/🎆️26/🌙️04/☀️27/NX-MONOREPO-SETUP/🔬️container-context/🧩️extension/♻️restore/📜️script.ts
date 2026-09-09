import assert from "node:assert/strict";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const workspace = process.cwd(), require = createRequire(join(workspace, "package.json"));
const packageRoot = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript";
if (process.argv[2] === "restore") {
  const root = process.argv[3], native = require("nx/src/native"), database = native.connectToNxDb(join(root, "database"));
  const manifest = JSON.parse(readFileSync(join(root, "contract.json"), "utf8"));
  const cache = new native.NxCache(join(root, "consumer"), join(root, "snapshot"), database);
  const files = (directory: string, prefix = ""): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? files(join(directory, entry.name), prefix + entry.name + "/") : [prefix + entry.name]).sort();
  const digest = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
  try {
    for (const row of manifest) {
      const snapshot = join(root, "snapshot", row.hash), expected = files(snapshot);
      for (let attempt = 0; attempt < 2; attempt++) {
        for (const output of row.outputs) rmSync(join(root, "consumer", output), { recursive: true, force: true });
        const copied = cache.copyFilesFromCache({ code: 0, outputsPath: snapshot }, row.outputs);
        for (const file of expected) assert.equal(digest(join(root, "consumer", file)), digest(join(snapshot, file)), file);
        console.log(`[DEBUG] Native Nx restored ${row.target} from actual task ${row.hash}, attempt ${attempt + 1}: ${copied} bytes, ${expected.length} files`);
      }
    }
    const output = join(root, "consumer", packageRoot, "out/extension.js"), checked = spawnSync("node", ["--check", output], { encoding: "utf8", timeout: 10000 });
    assert.equal(checked.status, 0, checked.stderr);
    const yauzl = createRequire(require.resolve("@vscode/vsce/package.json"))("yauzl");
    const archive = join(root, "consumer", packageRoot, "🧩️repo.vsix");
    const embedded = await new Promise<Buffer>((accept, reject) => yauzl.open(archive, { lazyEntries: true }, (error: Error | null, zip: any) => {
      if (error) return reject(error);
      zip.on("error", reject); zip.on("end", () => reject(new Error("Missing VSIX runtime bundle")));
      zip.on("entry", (entry: any) => {
        if (entry.fileName !== "extension/out/extension.js") return zip.readEntry();
        zip.openReadStream(entry, (error: Error | null, stream: any) => {
          if (error) return reject(error);
          const chunks: Buffer[] = []; stream.on("data", (chunk: Buffer) => chunks.push(chunk)); stream.on("error", reject);
          stream.on("end", () => { zip.close(); accept(Buffer.concat(chunks)); });
        });
      }); zip.readEntry();
    }));
    assert.deepEqual(embedded, readFileSync(output));
    console.log("[DEBUG] Restored VSIX opens through native ZIP reader and embeds the exact restored, Node-parseable host bundle PASS");
  } finally { native.closeDbConnection(database); }
} else {
  const { Database } = await import("bun:sqlite");
  const databasePath = readdirSync(join(workspace, ".nx/workspace-data")).find(name => name.endsWith(".db")); assert.ok(databasePath);
  const database = new Database(join(workspace, ".nx/workspace-data", databasePath), { readonly: true });
  const root = mkdtempSync(resolve(import.meta.dir, "../../../🗑️generated/extension-native-restore-"));
  const project = JSON.parse(readFileSync(join(workspace, packageRoot, "📋️project.json"), "utf8")), rows: any[] = [];
  try {
    for (const target of ["build", "build-vsix"]) {
      const row = database.query("SELECT d.hash FROM task_details d JOIN cache_outputs c USING(hash) JOIN task_history h USING(hash) WHERE d.project=? AND d.target=? AND h.status='local-cache' ORDER BY h.end DESC LIMIT 1").get("@semio-tech/repo-vscode", target) as { hash: string } | null; assert.ok(row);
      const source = join(workspace, ".nx/cache", row.hash); assert.ok(existsSync(source));
      cpSync(source, join(root, "snapshot", row.hash), { recursive: true });
      rows.push({ hash: row.hash, target, outputs: project.targets[target].outputs.map((path: string) => path.replace("{projectRoot}", packageRoot)) });
    }
  } finally { database.close(); }
  mkdirSync(join(root, "consumer")); mkdirSync(join(root, "database"));
  writeFileSync(join(root, "contract.json"), JSON.stringify(rows));
  const result = spawnSync("node", [import.meta.filename, "restore", root], { cwd: workspace, encoding: "utf8", timeout: 30000 });
  writeFileSync(join(root, "restore.log"), result.stdout + result.stderr); process.stdout.write(result.stdout); process.stderr.write(result.stderr); assert.equal(result.status, 0);
}
