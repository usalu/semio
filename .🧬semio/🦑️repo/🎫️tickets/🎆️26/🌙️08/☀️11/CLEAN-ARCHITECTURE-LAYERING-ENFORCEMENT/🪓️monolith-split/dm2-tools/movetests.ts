// 🚚️ Moves the [[test]] targets whose source closure mounts a shared file from the non-owner packages to the owner package, aligning dev-dependencies and lock edges.
import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname, relative, resolve} from "node:path";
import {DRY, ROOT, abs, log, read, put, packageNameOf, propagateEdges} from "./lib.ts";
const TSV = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/dm2-modowners-0.tsv";
const closureFiles = (entry: string): string[] => {
  const out = execFileSync("bun", [require("node:url").fileURLToPath(new URL("./closure.ts", import.meta.url)), entry, "--tests", "--json"], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28});
  return out.split("\n").filter(l => l.startsWith("{")).flatMap(l => JSON.parse(l).files as string[]);
};
type TestBlock = {name: string; path: string; text: string};
const testBlocks = (manifestText: string): TestBlock[] => [...manifestText.matchAll(/\[\[test\]\]\nname = "([^"]+)"\npath = "([^"]+)"\n(\n?)/gu)].map(m => ({name: m[1]!, path: m[2]!, text: m[0]}));
const entryLine = (text: string, key: string): string | undefined => new RegExp(`^${key.replace(/\./gu, "\\.")}\\s*=.*$`, "mu").exec(text)?.[0];
const snake = (k: string) => k.replaceAll("-", "_");
export type MoveSpec = {shared: string; owner: string};
export function moveTests(spec: MoveSpec): {moved: string[]; edges: {consumer: string; dep: string; dev: boolean}[]} {
  const row = readFileSync(TSV, "utf8").split("\n").filter(Boolean).map(l => l.split("\t")).find(r => r[0] === spec.shared);
  if (!row) throw new Error(`no scan row for ${spec.shared}`);
  const packages = row[2]!.split(" ; ");
  if (!packages.includes(spec.owner)) throw new Error(`owner ${spec.owner} does not mount ${spec.shared}`);
  const ownerManifest = `${spec.owner}/Cargo.toml`;
  const edges: {consumer: string; dep: string; dev: boolean}[] = [];
  const moved: string[] = [];
  for (const dir of packages.filter(p => p !== spec.owner)) {
    const sourceText = read(`${dir}/Cargo.toml`);
    const sourceParsed = Bun.TOML.parse(sourceText) as any;
    const chosen: TestBlock[] = [];
    for (const block of testBlocks(sourceText)) {
      const target = relative(abs(""), resolve(abs(dir), block.path));
      if (closureFiles(target).includes(spec.shared)) chosen.push(block);
    }
    if (!chosen.length) { log(`  !! ${dir}: no [[test]] mounts ${spec.shared} (non-test mount?)`); continue; }
    let nextSource = sourceText;
    for (const block of chosen) nextSource = nextSource.replace(block.text, "");
    put(`${dir}/Cargo.toml`, nextSource);
    let ownerText = read(ownerManifest);
    const ownerParsed = Bun.TOML.parse(ownerText) as any;
    const existing = new Set(testBlocks(ownerText).map(b => b.name));
    const files: string[] = [];
    let blocks = "";
    for (const block of chosen) {
      if (existing.has(block.name)) throw new Error(`test name clash ${block.name}`);
      const target = resolve(abs(dir), block.path);
      const path = relative(abs(spec.owner), target).replaceAll("\\", "/");
      blocks += `[[test]]\nname = "${block.name}"\npath = "${path}"\n\n`;
      files.push(...closureFiles(relative(abs(""), target)));
      moved.push(`${block.name}: ${dir} -> ${spec.owner}`);
    }
    ownerText = ownerText.replace(/\n\[dependencies\]\n/u, `\n${blocks}[dependencies]\n`);
    const corpus = [...new Set(files)].filter(f => f.endsWith(".rs")).map(f => read(f)).join("\n");
    const sourceKeys = [...Object.keys(sourceParsed.dependencies ?? {}), ...Object.keys(sourceParsed["dev-dependencies"] ?? {})];
    const have = new Set([...Object.keys(ownerParsed.dependencies ?? {}), ...Object.keys(ownerParsed["dev-dependencies"] ?? {})]);
    const ownerName = ownerParsed.package.name as string;
    const additions: string[] = [];
    for (const key of [...new Set(sourceKeys)]) {
      const s = snake(key);
      if (s === snake(ownerName)) continue;
      const used = new RegExp(`(?<![A-Za-z0-9_])${s}(?=::)|extern crate ${s}\\b|(?<![A-Za-z0-9_])use ${s}\\b`, "u").test(corpus);
      if (!used) continue;
      const sourceSpec = (sourceParsed["dev-dependencies"] ?? {})[key] ?? (sourceParsed.dependencies ?? {})[key];
      const ownerSpec = (ownerParsed["dev-dependencies"] ?? {})[key] ?? (ownerParsed.dependencies ?? {})[key];
      const wanted: string[] = typeof sourceSpec === "object" ? (sourceSpec.features ?? []) : [];
      const ownerFeatures: string[] = ownerSpec && typeof ownerSpec === "object" ? (ownerSpec.features ?? []) : [];
      if (ownerSpec && wanted.every(f => ownerFeatures.includes(f))) continue;
      const devLine = entryLine(sourceText, key);
      if (!devLine) { log(`  !! cannot find manifest line for ${key}`); continue; }
      let line = devLine;
      const pathMatch = /path\s*=\s*"([^"]+)"/u.exec(devLine);
      if (pathMatch) line = devLine.replace(pathMatch[1]!, relative(abs(spec.owner), resolve(abs(dir), pathMatch[1]!)).replaceAll("\\", "/"));
      additions.push(line);
      if (!ownerSpec) edges.push({consumer: ownerName, dep: typeof sourceSpec === "object" && sourceSpec.package ? sourceSpec.package : key, dev: true});
      have.add(key);
    }
    if (additions.length) {
      if (/\n\[dev-dependencies\]\n/u.test(ownerText)) ownerText = ownerText.replace(/\n\[dev-dependencies\]\n/u, `\n[dev-dependencies]\n${additions.join("\n")}\n`);
      else ownerText = ownerText.trimEnd() + `\n\n[dev-dependencies]\n${additions.join("\n")}\n`;
      log(`  dev-dependencies added to ${spec.owner}: ${additions.map(a => a.split("=")[0]!.trim()).join(", ")}`);
    }
    put(ownerManifest, ownerText);
  }
  return {moved, edges};
}
