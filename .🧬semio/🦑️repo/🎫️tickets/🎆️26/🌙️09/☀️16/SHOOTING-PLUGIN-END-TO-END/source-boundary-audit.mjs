import {readFile} from "node:fs/promises";
import {dirname, resolve} from "node:path";
import {parse} from "smol-toml";
import {readRustSourceClosureV1, createRustSourceAnalysisV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = process.cwd();
const args = process.argv.slice(2);
const target = args.find(arg => arg.startsWith("--target="))?.slice("--target=".length);
const architecture = {arm64: "aarch64", x64: "x86_64"}[process.arch] ?? process.arch;
const host = {darwin: `${architecture}-apple-darwin`, win32: `${architecture}-pc-windows-msvc`, linux: `${architecture}-unknown-linux-gnu`}[process.platform];
const platform = /^(aarch64|x86_64|wasm32)-(apple-darwin|pc-windows-msvc|unknown-linux-gnu|unknown-unknown|wasip2)$/.exec(target ?? host ?? "");
if (!platform || (platform[1] === "wasm32") !== ["unknown-unknown", "wasip2"].includes(platform[2])) {
  throw new Error(`Unsupported audit target: ${target ?? host ?? process.platform}`);
}
const [, arch, suffix] = platform;
const [system, family, vendor, environment] = {
  "apple-darwin": ["macos", "unix", "apple", ""],
  "pc-windows-msvc": ["windows", "windows", "pc", "msvc"],
  "unknown-linux-gnu": ["linux", "unix", "unknown", "gnu"],
  "unknown-unknown": ["unknown", "wasm", "unknown", ""],
  "wasip2": ["wasi", "wasm", "unknown", "p2"],
}[suffix];
const cfg = [`target_arch=${arch}`, `target_os=${system}`, `target_family=${family}`, `target_vendor=${vendor}`, `target_env=${environment}`, "target_endian=little", `target_pointer_width=${arch === "wasm32" ? 32 : 64}`, "debug_assertions"];
if (family !== "wasm") cfg.push(family);
const analysis = createRustSourceAnalysisV1();
const read = path => readFile(path).catch(error => {
  if (error.code !== "ENOENT") throw error;
  return null;
});
for (const file of args.filter(arg => !arg.startsWith("--target="))) {
  const manifest = resolve(root, file);
  const manifestDirectory = dirname(manifest);
  const spec = parse(await readFile(manifest, "utf8"));
  const entry = resolve(manifestDirectory, spec.lib?.path ?? spec.bin?.[0]?.path ?? "src/lib.rs");
  for (const test of [false, true]) {
    const inputs = await readRustSourceClosureV1([entry], root, test ? [...cfg, "test"] : cfg, read, {manifestDirectory, generated: () => {}}, analysis);
    console.log(spec.package.name, test ? "test" : "lib", inputs.length, "files,", inputs.reduce((sum, input) => sum + input.activeLines, 0), "active lines,", inputs.reduce((sum, input) => sum + input.bytes, 0), "bytes");
  }
}
