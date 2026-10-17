// 📏️ c4p15 census: closure.ts of dm2-tools with --base reading the CENSUS4-PKG-15 snapshots (orig/) and hiding the new children (newfiles.json)
// 🔬️ Lists the canonical source closure of one crate target (same counter and cfg as census.ts), so a gate overrun can be attributed file by file.
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {basename, dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const rustc = (() => { for (let attempt = 0; attempt < 5; attempt++) { try { const out = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim(); if (out) return out; } catch {} } return "rustc"; })();
const TARGET = process.argv.includes("--target") ? process.argv[process.argv.indexOf("--target") + 1] : undefined;
const hostCfg = execFileSync(rustc, ["--print", "cfg", ...(TARGET ? ["--target", TARGET] : [])], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const BASE = process.argv.includes("--base"), JSON_OUT = process.argv.includes("--json");
const TOOLS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p15-tools";
const SNAP = new Map<string, string>(readFileSync(TOOLS + "/orig/SHA256", "utf8").split("\n").filter(Boolean).map(line => { const [, file, ...rest] = line.split(" "); return [rest.join(" "), TOOLS + "/orig/" + file] as [string, string]; }));
const NEW_DIRS: string[] = JSON.parse(readFileSync(TOOLS + "/newfiles.json", "utf8"));
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try {
    if (BASE) {
      const rel = path.startsWith(ROOT + "/") ? path.slice(ROOT.length + 1) : path;
      if (NEW_DIRS.includes(rel)) return null;
      const snap = SNAP.get(rel);
      if (snap) return readFileSync(snap);
    }
    return readFileSync(path);
  } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return process.env.C4P15_TOLERATE && path.includes(process.env.C4P15_TOLERATE) ? new Uint8Array() : null; throw error; }
};
const featureSet = (parsed: any): Set<string> => {
  const table: Record<string, string[]> = parsed.features ?? {};
  const out = new Set<string>(["default"]), queue = [...(table.default ?? [])];
  while (queue.length) {
    const token = queue.pop()!;
    if (out.has(token)) continue;
    out.add(token);
    if (token.startsWith("dep:")) continue;
    const [head, rest] = token.split("/") as [string, string | undefined];
    const name = head.replace(/\?$/u, "");
    if (table[name]) queue.push(...table[name]);
    if (rest !== undefined && !head.endsWith("?")) out.add(name);
  }
  return out;
};
const args = process.argv.slice(2);
const tests = args.includes("--tests");
const quiet = args.includes("--summary");
for (const arg of args.filter((value, index) => !value.startsWith("--") && args[index - 1] !== "--target" && args[index - 1] !== "--cfg")) {
  if (arg.endsWith(".rs")) {
    const entry = resolve(ROOT, arg), dir = dirname(entry);
    const cfg = [...hostCfg, "debug_assertions", ...(tests ? ["test"] : []), ...process.argv.flatMap((a, i) => a === "--cfg" ? [process.argv[i + 1]!.replace(/"/gu, "")] : [])];
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
    if (JSON_OUT) console.log(JSON.stringify({crate: "oracle-entry", kind: "lib", entry: arg, files: inputs.map(i => String((i as any).path ?? (i as any).file ?? (i as any).relativePath).replace(ROOT + "/", ""))}));
    else console.log(`## ${arg} files=${inputs.length}`);
    continue;
  }
  const manifest = arg.endsWith("Cargo.toml") ? resolve(ROOT, arg) : join(resolve(ROOT, arg), "Cargo.toml");
  const dir = dirname(manifest);
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const features = featureSet(parsed);
  const cfg = [...hostCfg, "debug_assertions", ...(tests ? ["test"] : []), ...process.argv.flatMap((a, i) => a === "--cfg" ? [process.argv[i + 1]!.replace(/"/gu, "")] : []), ...[...features].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f)];
  const entries: {kind: string; name: string; entry: string}[] = [];
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  if (existsSync(libEntry)) entries.push({kind: "lib", name: parsed.package.name, entry: libEntry});
  for (const bin of parsed.bin ?? []) entries.push({kind: "bin", name: bin.name, entry: resolve(dir, bin.path ?? `src/bin/${bin.name}.rs`)});
  if (tests) for (const test of parsed.test ?? []) entries.push({kind: "test", name: test.name, entry: resolve(dir, test.path ?? `tests/${test.name}.rs`)});
  for (const target of entries) {
    const inputs = await readRustSourceClosureV1([target.entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
    const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
    if (JSON_OUT) { console.log(JSON.stringify({crate: parsed.package.name, kind: target.kind, entry: target.entry.replace(ROOT + "/", ""), files: inputs.map(i => String((i as any).path ?? (i as any).file ?? (i as any).relativePath).replace(ROOT + "/", ""))})); continue; }
    console.log(`## ${parsed.package.name} ${target.kind}:${target.name} files=${inputs.length} bytes=${bytes} lines=${lines} embedded=${inputs.filter(i => i.kind === "embedded").length}`);
    if (!quiet) for (const input of inputs) console.log(`${input.kind}\t${input.bytes}\t${input.activeLines}\t${String((input as any).path ?? (input as any).file ?? (input as any).relativePath ?? JSON.stringify(input)).replace(ROOT + "/", "")}`);
  }
}
