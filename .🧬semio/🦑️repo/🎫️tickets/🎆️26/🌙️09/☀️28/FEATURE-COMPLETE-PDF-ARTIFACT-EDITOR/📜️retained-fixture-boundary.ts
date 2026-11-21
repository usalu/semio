/** 🧱 Reuse the mounted-operation fixture so its test crate fits the repository source limit. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command";
const file = resolve(root, "🪟️mounted/♻️frontier/🦀️.rs");
const before = readFileSync(file, "utf8");
const sha = (body: string) => createHash("sha256").update(body).digest("hex");
const lines = before.split("\n");
const blank = lines.filter(line => line.startsWith(" MountedTypedCommandFullOperation::<A>{"));
const committed = lines.filter(line => line.startsWith("  let mut mounted=MountedTypedCommandFullOperation::<A>{"));
assert.equal(blank.length, 1);
assert.equal(committed.length, 1);
const literal = blank[0]!.trim();
const committedLiteral = committed[0]!.slice("  let mut mounted=".length, -1);
assert.equal(committedLiteral, literal.replace("terminal_seen:false", "terminal_seen:true")
    .replace("pending_child_publication:None", "pending_child_publication:Some(child)"));
const reused = literal.replace("terminal_seen:false", "terminal_seen:child.is_some()")
    .replace("pending_child_publication:None", "pending_child_publication:child");
const after = before.replace(blank[0]!, ` blank_mounted_operation(binding,None)
}
#[cfg(any(test, feature = "artifact-app-testing"))]
fn blank_mounted_operation<A:ArtifactApp>(binding:semio_framework_job::OriginalSourceBinding,child:Option<PendingChildGroupPublication<A>>)->MountedTypedCommandFullOperation<A>{
 ${reused}`).replace(committed[0]!, "  let mut mounted=blank_mounted_operation(binding,Some(child));")
    .replaceAll("semio_framework_value::ValueRefusalKind::", "PublicationRefusalKind::")
    .replaceAll("semio_framework_job::", "job::");
const imported = "use semio_framework_value::ValueRefusalKind as PublicationRefusalKind;\nuse semio_framework_job as job;\n\n" + after;
assert.ok(after.length < before.length);

const scope = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts");
const { measureRustCrate } = await import(scope);
const manifest = resolve(root, "📦️packages/🦀️rust/Cargo.toml");
const entry = resolve(dirname(manifest), (Bun.TOML.parse(readFileSync(manifest, "utf8")) as { lib: { path: string } }).lib.path);
const candidate = new Map([[file, imported]]);
const checks = [];
for (const os of ["macos", "linux", "windows", "wasi"]) {
    const cfg = [`target_os=${os}`, os === "wasi" ? "target_arch=wasm32" : os === "macos" ? "target_arch=aarch64" : "target_arch=x86_64",
        os === "wasi" ? "target_family=wasm" : os === "windows" ? "target_family=windows" : "target_family=unix"];
    for (const test of [false, true]) {
        const { files, lines, bytes } = await measureRustCrate(process.cwd(), entry, test, candidate, dirname(manifest), cfg);
        assert.ok(files <= 32 && lines <= 4000 && bytes <= 262144, JSON.stringify({ os, test, files, lines, bytes }));
        checks.push({ os, test, files, lines, bytes });
    }
}
const generated = join(import.meta.dirname, "🗑️generated");
mkdirSync(generated, { recursive: true });
const receipt = { file, preimage: sha(before), postimage: sha(imported), body: imported, checks };
writeFileSync(join(generated, "retained-fixture-bounds.json"), JSON.stringify(receipt, null, 2) + "\n");
if (process.argv.includes("--apply")) {
    const scratch = join(generated, `retained-fixture-${process.pid}.rs`);
    writeFileSync(scratch, imported, { flag: "wx", mode: statSync(file).mode & 0o777 });
    assert.equal(sha(readFileSync(file, "utf8")), receipt.preimage);
    renameSync(scratch, file);
    assert.equal(sha(readFileSync(file, "utf8")), receipt.postimage);
}
console.log("[DEBUG] Reused exact mounted fixture", JSON.stringify(checks));
