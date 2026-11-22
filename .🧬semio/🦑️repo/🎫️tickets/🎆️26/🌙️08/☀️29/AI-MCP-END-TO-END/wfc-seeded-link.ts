import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

const generated = join(import.meta.dirname, "🗑️generated");
const build = join(generated, "native-host", "debug", "build");
const folderOwner = "semio-os-store-sync-native";
const storageVariant = process.argv[2] ?? "9d0b44bf77df9a85";
const folders = new Set<string>();
const native = new Set<string>();
type Library = { readonly binary: string; readonly fingerprint: string };
const libraries = new Map<string, Library>();
const variants = new Map<string, Library>();
for (const pkg of readdirSync(build)) {
  for (const variant of readdirSync(join(build, pkg))) {
    const unit = join(build, pkg, variant);
    const out = join(unit, "out");
    if (!existsSync(out)) continue;
    for (const file of readdirSync(out)) {
      if (file.endsWith(".a")) native.add(out);
      const crate = /^lib(.+)-[a-f0-9]+\.(?:rlib|dylib|dll|so)$/u.exec(file)?.[1];
      if (!crate) continue;
      const fingerprint = join(unit, "fingerprint", `lib-${crate}`);
      if (!existsSync(fingerprint)) continue;
      const library = { binary: join(out, file), fingerprint };
      libraries.set(readFileSync(fingerprint, "utf8").trim(), library);
      variants.set(unit, library);
    }
  }
}
const dependencies = ({ fingerprint }: Library) => {
  const lossless = readFileSync(`${fingerprint}.json`, "utf8");
  return Array.from(lossless.matchAll(/"([^"]+)",false,(\d+)\]/gu), ([, name, number]) => {
    const hash = Buffer.alloc(8);
    hash.writeBigUInt64LE(BigInt(number));
    return [name, libraries.get(hash.toString("hex"))] as const;
  }).filter(([name]) => name !== "build_script_build");
};
const storage = variants.get(join(build, folderOwner, storageVariant));
assert(storage, "No full native folder owner");
const asyncLibrary = dependencies(storage).find(([name]) => name === "semio_framework_async")?.[1];
assert(asyncLibrary, "No matching async executor");
const visit = (library: Library) => {
  const directory = dirname(library.binary);
  if (folders.has(directory)) return;
  folders.add(directory);
  for (const [name, dependency] of dependencies(library)) {
    assert(dependency, `Missing exact ${name} for ${library.fingerprint}`);
    visit(dependency);
  }
};
visit(storage);
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
assert(channel);
const args = ["repo", "tools", "exec", "--light", "rustup", "--", "run", channel, "rustc", "--edition=2021", join(import.meta.dirname, "wfc-seeded-folder.rs"), "-o", join(generated, process.platform === "win32" ? "wfc-seeded-folder.exe" : "wfc-seeded-folder")];
for (const [name, library] of [["semio_os_store_sync_native", storage], ["semio_framework_async", asyncLibrary]] as const) {
  const metadata = library.binary.replace(/\.rlib$/u, ".rmeta");
  if (existsSync(metadata) && metadata !== library.binary) args.push("--extern", `${name}=${metadata}`);
  args.push("--extern", `${name}=${library.binary}`);
}
for (const dir of folders) args.push("-L", `dependency=${dir}`);
for (const dir of native) args.push("-L", `native=${dir}`);
console.log("[DEBUG] Link native folder owner", storageVariant, folders.size);
process.exit(Bun.spawnSync(args, { stdout: "inherit", stderr: "inherit" }).exitCode);
