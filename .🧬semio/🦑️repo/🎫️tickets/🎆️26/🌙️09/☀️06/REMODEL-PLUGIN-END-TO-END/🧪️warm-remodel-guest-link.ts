import assert from "node:assert/strict";
import { existsSync, mkdirSync, readdirSync, readFileSync, symlinkSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const output = join(import.meta.dir, "🗑️generated");
const units = join(output, "wasm-build", "wasm32-wasip2", "debug", "build");
const component = "semio_hub_remodel_component";
const fingerprintPath = join(units, component.replaceAll("_", "-"), "d9807bae90662a24", "fingerprint", "lib-" + component + ".json");
const fingerprintText = readFileSync(fingerprintPath, "utf8");
const requested = new Map<string, string>();
for (const [, name, value] of fingerprintText.matchAll(/\[\d+,"([^"]+)",false,(\d+)\]/gu)) {
  const receipt = Buffer.alloc(8);
  receipt.writeBigUInt64LE(BigInt(value));
  requested.set(receipt.toString("hex"), name);
}
assert.equal(requested.size, 3);
const dependencies = join(output, "guest-link-deps");
mkdirSync(dependencies, { recursive: true });
const externs = new Map<string, string[]>();
const nativeDirectories = new Set<string>();
for (const pkg of readdirSync(units, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
  const packagePath = join(units, pkg.name);
  for (const hash of readdirSync(packagePath, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
    const unit = join(packagePath, hash.name);
    const artifacts = join(unit, "out");
    if (!existsSync(artifacts)) continue;
    const files = readdirSync(artifacts);
    if (files.some(name => name.endsWith(".a"))) nativeDirectories.add(artifacts);
    for (const file of files.filter(name => /\.(?:rmeta|rlib)$/u.test(name))) {
      const link = join(dependencies, file);
      if (!existsSync(link)) symlinkSync(join(artifacts, file), link);
    }
    const receipts = join(unit, "fingerprint");
    if (!existsSync(receipts)) continue;
    for (const name of readdirSync(receipts).filter(name => /^lib-[^.]+$/u.test(name))) {
      const alias = requested.get(readFileSync(join(receipts, name), "utf8").trim());
      if (!alias) continue;
      const prefix = "lib" + name.slice(4) + "-" + hash.name + ".";
      const pair = files.filter(file => file.startsWith(prefix) && /\.(?:rmeta|rlib)$/u.test(file)).map(file => join(artifacts, file));
      if (pair.some(file => file.endsWith(".rlib"))) externs.set(alias, pair);
    }
  }
}
assert.equal(externs.size, requested.size, JSON.stringify([...externs]));
for (const pkg of readdirSync(join(output, "wasm-build", "debug", "build"), { withFileTypes: true }).filter(entry => entry.isDirectory())) {
  const packagePath = join(output, "wasm-build", "debug", "build", pkg.name);
  for (const hash of readdirSync(packagePath, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
    const artifacts = join(packagePath, hash.name, "out");
    if (!existsSync(artifacts)) continue;
    for (const file of readdirSync(artifacts).filter(name => name.endsWith(".so"))) {
      const link = join(dependencies, file);
      if (!existsSync(link)) symlinkSync(join(artifacts, file), link);
    }
  }
}
const argumentsByCrate = [...externs].flatMap(([name, files]) => files.flatMap(file => ["--extern", name + "=" + file]));
const [stack, tag] = process.argv.slice(2);
assert.match(stack ?? "", /^[0-9]+$/u);
assert.match(tag ?? "", /^[a-z][a-z0-9-]+$/u);
const rustc = process.env.REMODEL_PINNED_RUSTC;
assert(rustc);
const directory = join(root, "🌎️hub", "🧩️compositions", "📸️remodel", "🧩️component");
const source = join(directory, "🦀️.rs");
const env = { ...process.env, CARGO_MANIFEST_DIR: join(directory, "📦️packages", "🦀️rust"), CARGO_PKG_VERSION: "0.1.0" };
const pass = Bun.spawn([rustc, "--crate-name", component, "--edition=2021", "--crate-type=cdylib", "--target=wasm32-wasip2", "--emit=link", "-C", "debuginfo=0", "-C", "strip=symbols", "-C", "link-arg=--max-memory=536870912", "-C", "link-arg=-zstack-size=" + stack, "-C", "link-arg=-Map=" + join(output, tag + ".map"), "-L", "dependency=" + dependencies, ...[...nativeDirectories].flatMap(path => ["-L", "native=" + path]), ...argumentsByCrate, "-o", join(output, tag + ".wasm"), source], { env, stdout: "inherit", stderr: "inherit" });
process.exitCode = await pass.exited;
