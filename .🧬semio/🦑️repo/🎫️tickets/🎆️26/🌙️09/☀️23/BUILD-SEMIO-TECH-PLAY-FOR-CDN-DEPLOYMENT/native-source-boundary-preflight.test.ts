import { expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { admitManifestV1, createPreflightPortV1, defaultFeaturesV1, unitsOfManifestV1 } from "./native-source-boundary-preflight.ts";

const policy = { version: 1 as const, maxSourceBytes: 262144, maxSourceFiles: 32, maxActiveLines: 4000 };
const filler = (bytes: number): string => ("// " + "x".repeat(96) + "\n").repeat(Math.floor(bytes / 100));
const sizeOf = (text: string): number => Buffer.byteLength(text);

/** 🧪️ Writes one synthetic crate whose cfg-mounted modules exist only under the real compiler cfg: the lib mounts them behind `target_pointer_width` and a default feature. */
async function crate(root: string, name: string, lib: string, modules: Record<string, string>, features = ""): Promise<string> {
  const directory = join(root, name);
  await mkdir(directory, { recursive: true });
  await writeFile(join(directory, "Cargo.toml"), `[package]\nname = "${name}"\nversion = "0.0.0"\nedition = "2021"\n\n[lib]\npath = "lib.rs"\n${features}`);
  await writeFile(join(directory, "lib.rs"), lib);
  for (const [file, text] of Object.entries(modules)) await writeFile(join(directory, file), text);
  return join(directory, "Cargo.toml");
}

/** 🧮️ What the former probe counted: the lib closure under an empty cfg. */
async function formerProbe(manifest: string, root: string): Promise<{ files: number; bytes: number }> {
  const files = await readRustSourceClosureV1([join(dirname(manifest), "lib.rs")], root, [], async path => { try { return new Uint8Array(await Bun.file(path).arrayBuffer()); } catch { return null; } });
  return { files: files.length, bytes: files.reduce((sum, file) => sum + file.bytes, 0) };
}

test("a read that crosses the remaining byte budget reports admission's boundary breach with its read cause", async () => {
  const root = await mkdtemp(join(import.meta.dir, "🗑️generated/probe-parity-"));
  await mkdir(join(root), { recursive: true });
  try {
    const lib = filler(200000) + '#[cfg(target_pointer_width = "64")]\nmod big;\n', big = filler(70000);
    const manifest = await crate(root, "bytes", lib, { "big.rs": big });
    const former = await formerProbe(manifest, root);
    expect(former.bytes).toBeLessThan(policy.maxSourceBytes);
    const verdict = await admitManifestV1(root, manifest, policy, createPreflightPortV1(true));
    expect(verdict.kind).toBe("over");
    const total = sizeOf(lib) + sizeOf(big);
    expect(verdict.kind === "over" && verdict.line).toBe(`OVER\tbytes\tprocess.source.boundary-exceeded ${JSON.stringify({ crate: "bytes", files: 2, maxFiles: 32, bytes: total, maxBytes: 262144 })}\tprocess.source.read-bytes-exceeded ${JSON.stringify({ file: join(root, "bytes/big.rs"), observedBytes: String(sizeOf(big)), maximumBytes: 262144 - sizeOf(lib) })}\tbytes/Cargo.toml`);
  } finally { await rm(root, { recursive: true, force: true }); }
});

test("default-feature modules past the file ceiling are counted although the former empty-cfg probe skipped them", async () => {
  const root = await mkdtemp(join(import.meta.dir, "🗑️generated/probe-parity-"));
  try {
    const names = Array.from({ length: 32 }, (_, index) => "m" + index), lib = names.map(name => `#[cfg(feature = "wide")]\nmod ${name};\n`).join("");
    const manifest = await crate(root, "files", lib, Object.fromEntries(names.map(name => [name + ".rs", "pub fn f() {}\n"])), '\n[features]\ndefault = ["wide"]\nwide = []\n');
    expect(await formerProbe(manifest, root)).toEqual({ files: 1, bytes: sizeOf(lib) });
    expect(defaultFeaturesV1({ default: ["wide"], wide: [] })).toEqual(["wide"]);
    const verdict = await admitManifestV1(root, manifest, policy, createPreflightPortV1(true));
    expect(verdict.kind === "over" && verdict.line).toBe(`OVER\tfiles\tprocess.source.boundary-exceeded ${JSON.stringify({ crate: "files", files: 33, maxFiles: 32, bytes: sizeOf(lib) + 31 * sizeOf("pub fn f() {}\n"), maxBytes: 262144 })}\tprocess.source.read-files-exhausted ${JSON.stringify({ file: join(root, "files/m31.rs"), remainingFiles: 0 })}\tfiles/Cargo.toml`);
  } finally { await rm(root, { recursive: true, force: true }); }
});

test("a crate within every limit passes and its units mirror admission's inputs", async () => {
  const root = await mkdtemp(join(import.meta.dir, "🗑️generated/probe-parity-"));
  try {
    const manifest = await crate(root, "small", "pub fn f() {}\n", {}, '\n[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 8\nmaxSourceFiles = 1\nmaxActiveLines = 1\n');
    const [unit] = unitsOfManifestV1(manifest, await Bun.file(manifest).text());
    expect(unit!.boundary).toEqual({ maxSourceBytes: 8, maxSourceFiles: 1, maxActiveLines: 1 });
    const verdict = await admitManifestV1(root, manifest, policy, createPreflightPortV1(true));
    expect(verdict.kind === "over" && verdict.line).toContain("process.source.boundary-exceeded");
    expect(verdict.kind === "over" && verdict.line).toContain('"maxBytes":8');
  } finally { await rm(root, { recursive: true, force: true }); }
});
