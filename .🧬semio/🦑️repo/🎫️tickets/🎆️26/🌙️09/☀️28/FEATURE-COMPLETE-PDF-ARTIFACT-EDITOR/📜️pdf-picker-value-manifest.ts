/** 📦 Point the PDF document's value dependency at its actual internal package across workspaces. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const file = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/📄️document/📦️packages/🦀️rust/Cargo.toml";
const value = "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust";
const relativeValue = relative(resolve(dirname(file)), resolve(value)).split("\\").join("/");
const inherited = "semio-framework-value = { workspace = true }";
const local = `semio-framework-value = { path = "${relativeValue}" }`;
const current = readFileSync(file, "utf8");
const source = JSON.parse(readFileSync(join(generated, "raw-picker-mounted-overlay.json"), "utf8")) as { candidate: Record<string, string> };
const preimage = source.candidate[file]!;
const sha = (body: string) => createHash("sha256").update(body).digest("hex");
const rows = (JSON.parse(readFileSync(join(generated, "raw-picker-substrate-installed.json"), "utf8")) as { files: { path: string; after: string }[] }).files;
assert.equal(sha(preimage), rows.find(row => row.path === file)?.after);
assert.equal(preimage.split(inherited).length, 2);
const body = preimage.replace(inherited, local);
assert.ok(current === preimage || current === body);
const manifest = Bun.TOML.parse(body) as { dependencies: Record<string, { path?: string }> };
assert.equal(resolve(dirname(file), manifest.dependencies["semio-framework-value"]!.path!), resolve(value));
assert.equal((Bun.TOML.parse(readFileSync(join(value, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name, "semio-framework-value");
if (current === preimage) {
    const staged = join(generated, `pdf-value-manifest-${process.pid}.toml`);
    writeFileSync(staged, body, { flag: "wx", mode: statSync(file).mode & 0o777 });
    assert.equal(readFileSync(file, "utf8"), preimage);
    renameSync(staged, file);
}
assert.equal(readFileSync(file, "utf8"), body);
writeFileSync(join(generated, "pdf-value-manifest-receipt.json"), JSON.stringify({ file, before: sha(preimage), after: sha(body) }) + "\n");
console.log("[DEBUG] Verified the exact PDF document value dependency path");
