import assert from "node:assert/strict";
import { existsSync, mkdirSync, readdirSync, readFileSync, symlinkSync } from "node:fs";
import { join } from "node:path";

const output = join(import.meta.dir, "🗑️generated");
const build = join(output, "rust-build", "debug", "build");
const crate = "semio_s_artifact_remodel_remodeling_session";
const unit = join(build, crate.replaceAll("_", "-"), "9b42f60fda67eaec", "fingerprint", "test-lib-" + crate + ".json");
const record = readFileSync(unit, "utf8");
const fingerprint = JSON.parse(record);
assert.equal(fingerprint.features, "[]");
const requested = new Map<string, string>();
for (const [, name, value] of record.matchAll(/\[\d+,"([^"]+)",false,(\d+)\]/gu)) {
  const identity = Buffer.alloc(8);
  identity.writeBigUInt64LE(BigInt(value));
  requested.set(identity.toString("hex"), name);
}
assert.equal(requested.size, fingerprint.deps.length);

const links = join(output, "dsl-probe-dependencies");
mkdirSync(links, { recursive: true });
const externs = new Map<string, string[]>();
for (const pkg of readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
  const packagePath = join(build, pkg.name);
  for (const hash of readdirSync(packagePath, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
    const folder = join(packagePath, hash.name);
    const artifacts = join(folder, "out");
    if (!existsSync(artifacts)) continue;
    const libraryFiles = readdirSync(artifacts).filter(name => /\.(?:rmeta|rlib|so)$/u.test(name));
    for (const file of libraryFiles) {
      const link = join(links, file);
      if (!existsSync(link)) symlinkSync(join(artifacts, file), link);
    }
    const receipts = join(folder, "fingerprint");
    if (!existsSync(receipts)) continue;
    for (const receipt of readdirSync(receipts).filter(name => /^lib-[^.]+$/u.test(name))) {
      const alias = requested.get(readFileSync(join(receipts, receipt), "utf8"));
      if (!alias) continue;
      const prefix = "lib" + receipt.slice(4) + "-" + hash.name + ".";
      const files = libraryFiles.filter(name => name.startsWith(prefix)).map(name => join(artifacts, name));
      assert.ok(files.length, alias);
      externs.set(alias, files);
    }
  }
}
assert.deepEqual([...externs.keys()].sort(), [...requested.values()].sort());
const manifest = join(process.cwd(), "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/📦️packages/🦀️rust");
const argumentsByCrate = [...externs].flatMap(([name, files]) => files.flatMap(file => ["--extern", name + "=" + file]));
const rustc = process.env.REMODEL_PINNED_RUSTC;
assert.ok(rustc);
const binary = process.argv[2] ?? join(output, "reconstruction-session-standalone");
const child = Bun.spawn([rustc, "--test", "--edition=2021", "--crate-name", crate, "-C", "debuginfo=0", ...fingerprint.rustflags, "-L", "dependency=" + links, ...argumentsByCrate, join(manifest, "🦀️.rs"), "-o", binary], {
  env: { ...process.env, CARGO_MANIFEST_DIR: manifest },
  stdout: "inherit",
  stderr: "inherit",
});
process.exit(await child.exited);
