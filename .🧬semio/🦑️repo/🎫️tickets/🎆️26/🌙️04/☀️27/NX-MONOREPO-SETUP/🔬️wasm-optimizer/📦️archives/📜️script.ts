import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { open } from "node:fs/promises";
import { join, resolve } from "node:path";
const workspace = process.cwd(), root = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm"), output = resolve(import.meta.dir, "../../🗑️generated/binaryen-archives");
const manifest = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")), { binaryenMembers } = await import(join(root, "📜️script.ts"));
mkdirSync(output, { recursive: true });
const results = [];
for (const row of manifest.platforms) {
  const archive = join(output, row.archive), response = await fetch(`${manifest.release}/${row.archive}`, { signal: AbortSignal.timeout(120000) });
  assert.equal(response.ok, true); const file = await open(archive, "w"), hash = createHash("sha256"); let bytes = 0;
  try { for await (const chunk of response.body!) { bytes += chunk.byteLength; assert.ok(bytes <= row.bytes); hash.update(chunk); await file.writeFile(chunk); } } finally { await file.close(); }
  assert.equal(bytes, row.bytes); assert.equal(hash.digest("hex"), row.sha256);
  const native = Bun.spawnSync(["tar", "-tzf", archive], { stdout: "pipe", stderr: "pipe", timeout: 30000 }); assert.equal(native.exitCode, 0, native.stderr.toString());
  const selected = binaryenMembers(native.stdout.toString().trim().split(/\r?\n/), row.platform);
  assert.ok(selected.includes(`bin/wasm-opt${row.platform === "win32" ? ".exe" : ""}`));
  const oracle = Bun.spawnSync(["python3", "-c", "import tarfile,json,sys; t=tarfile.open(sys.argv[1]); names=json.loads(sys.argv[2]); print(json.dumps([{'path':m.name,'file':m.isfile(),'link':m.linkname,'size':m.size} for m in t if m.name in names]))", archive, JSON.stringify(selected.map((name: string) => `binaryen-version_${manifest.version}/${name}`))], { stdout: "pipe", stderr: "pipe", timeout: 30000 });
  assert.equal(oracle.exitCode, 0, oracle.stderr.toString()); const members = JSON.parse(oracle.stdout.toString());
  assert.equal(members.length, selected.length); assert.ok(members.every((member: any) => member.file));
  results.push({ platform: row.platform, architecture: row.architecture, archiveSha256: row.sha256, archiveBytes: bytes, members });
  rmSync(archive); console.log("[DEBUG] Published archive selection and Python tarfile parity PASS", row.platform, row.architecture, JSON.stringify(members));
}
writeFileSync(join(output, "verified-members.json"), JSON.stringify(results, null, 2) + "\n");
