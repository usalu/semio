// 📏️ Crate source-gate re-measure at HEAD with the canonical reader (readRustSourceClosureV1), read-only.
import { execFileSync } from "node:child_process";
import { writeFileSync, readFileSync } from "node:fs";
import { dirname, join, posix } from "node:path";
import { fileURLToPath } from "node:url";
import { readRustSourceClosureV1, type RustSourceInputV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { crateTreeHostCfg } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const SHARD = Number(process.env.SHARD ?? 0), NSHARD = Number(process.env.NSHARD ?? 1);
const OUT = join(dirname(fileURLToPath(import.meta.url)), `measure.${process.env.TAG ?? "x"}.json`);
const ONLY = (process.env.ONLY ?? "").split(",").filter(Boolean);
const POLICY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
const BULK = ["8f91299152c", "bd9e4e02bd0"];

const git = (...args: string[]) => execFileSync("git", ["-C", ROOT, ...args], { maxBuffer: 1 << 30 });
const trackedList = git("ls-files", "-z", "--cached", "--others", "--exclude-standard").toString("utf8").split("\0").filter(Boolean);
const tracked = new Set(trackedList);

const excluded = (path: string): boolean => {
  const segs = path.split("/");
  return path.startsWith(".🧬semio/") || segs.some((s) => s === "node_modules" || s === "dist" || s === "target" || (s.includes("🤖") && s.endsWith("generated")) || (s.includes("🪞") && s.includes("vendor")) || (s.includes("♻") && s.includes("mit-bestand")));
};

const policy = JSON.parse(git("show", "HEAD:" + POLICY).toString("utf8"));
const gate = policy.storage.sourceBoundary as { maxSourceBytes: number; maxSourceFiles: number; maxActiveLines: number };

const cache = new Map<string, Uint8Array>();
const disk = (abs: string): Uint8Array | null => { try { return readFileSync(abs); } catch { return null; } };
const read = async (abs: string): Promise<Uint8Array | null> => {
  return disk(abs);
};

type Target = { manifestDir: string; entry: string; kind: "lib" | "bin"; manifest: string; name: string };
const manifests = trackedList.filter((p) => p.endsWith("Cargo.toml") && !excluded(p));
const targets: Target[] = [];
const skipped: string[] = [];
for (const manifest of manifests) {
  const text = disk(join(ROOT, manifest))?.toString("utf8") ?? "";
  if (!/^\[package\]/mu.test(text)) continue;
  const dir = posix.dirname(manifest) === "." ? "" : posix.dirname(manifest);
  const name = /^\[package\][\s\S]*?^name\s*=\s*"([^"]+)"/mu.exec(text)?.[1] ?? manifest;
  const sections: { header: string; body: string }[] = [];
  const parts = text.split(/^(\[\[?[^\]]+\]\]?)\s*$/mu);
  for (let i = 1; i < parts.length; i += 2) sections.push({ header: parts[i]!, body: parts[i + 1] ?? "" });
  const pathOf = (h: string) => sections.filter((s) => s.header === h).map((s) => /^path\s*=\s*"([^"]+)"/mu.exec(s.body)?.[1]).find((x) => x !== undefined);
  const libHasSection = sections.some((s) => s.header === "[lib]");
  const libPath = pathOf("[lib]") ?? "src/lib.rs";
  const libRel = posix.join(dir, libPath);
  if (libHasSection || tracked.has(libRel)) {
    if (tracked.has(libRel)) targets.push({ manifestDir: dir, entry: libRel, kind: "lib", manifest, name });
    else skipped.push(manifest + " lib " + libRel);
  }
  const binRels = new Set<string>();
  for (const s of sections.filter((x) => x.header === "[[bin]]")) {
    const p = /^path\s*=\s*"([^"]+)"/mu.exec(s.body)?.[1];
    if (p) binRels.add(posix.join(dir, p));
  }
  const mainRel = posix.join(dir, "src/main.rs");
  if (tracked.has(mainRel)) binRels.add(mainRel);
  const binPrefix = posix.join(dir, "src/bin") + "/";
  for (const t of trackedList) {
    if (!t.startsWith(binPrefix)) continue;
    const rest = t.slice(binPrefix.length);
    if (/^[^/]+\.rs$/u.test(rest) || /^[^/]+\/main\.rs$/u.test(rest)) binRels.add(t);
  }
  for (const b of binRels) if (tracked.has(b)) targets.push({ manifestDir: dir, entry: b, kind: "bin", manifest, name });
}

const measure = async (t: Target, cfg: readonly string[]) => {
  try {
    const inputs: readonly RustSourceInputV1[] = await readRustSourceClosureV1([join(ROOT, t.entry)], ROOT, cfg, read, { manifestDirectory: join(ROOT, t.manifestDir), generated: () => undefined });
    const files = inputs.length, bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
    const ratios = { files: files / gate.maxSourceFiles, bytes: bytes / gate.maxSourceBytes, lines: lines / gate.maxActiveLines };
    const dim = (Object.keys(ratios) as (keyof typeof ratios)[]).reduce((a, b) => (ratios[b] > ratios[a] ? b : a));
    return { ok: true as const, files, bytes, lines, ratio: ratios[dim], dim };
  } catch (e) {
    return { ok: false as const, error: String((e as Error).message ?? e).slice(0, 300) };
  }
};

const hostCfg = [...crateTreeHostCfg()];
const testCfg = [...hostCfg, "test"];
const mine = targets.filter((t) => ONLY.includes(t.name));
const t0 = Date.now();
const results: Record<string, unknown>[] = [];
let n = 0;
for (const t of mine) {
  const prod = await measure(t, hostCfg);
  const test = await measure(t, testCfg);
  results.push({ name: t.name, manifest: t.manifest, kind: t.kind, entry: t.entry, prod, test });
  n++;
  if (n % 200 === 0) console.log(`[DEBUG] shard ${SHARD} measured ${n}/${mine.length} in ${(Date.now() - t0) / 1000}s`);
}

const touched = new Set<string>();
for (const rev of BULK) {
  const names = git("diff-tree", "--no-commit-id", "--name-only", "-r", "-z", "--no-renames", rev + "^", rev).toString("utf8").split("\0").filter((x) => x.endsWith(".rs"));
  for (const f of names) {
    const owners = manifests.filter((m) => { const d = posix.dirname(m); return d === "." || f.startsWith(d + "/"); }).sort((a, b) => b.length - a.length);
    if (owners[0]) touched.add(owners[0]);
  }
}

writeFileSync(OUT, JSON.stringify({ gate, hostCfgCount: hostCfg.length, crates: results, touchedManifests: [...touched], skipped }, null, 1));
const over = (r: any, k: "prod" | "test") => r[k].ok && r[k].ratio >= 0.9;
const prodOver = results.filter((r: any) => over(r, "prod") || over(r, "test") || !(r as any).prod.ok || !(r as any).test.ok);
console.log(JSON.stringify({ gate, crates: results.length, manifests: manifests.length, skipped: skipped.length, refusedProd: results.filter((r: any) => !r.prod.ok).length, refusedTest: results.filter((r: any) => !r.test.ok).length, atOrOver090Prod: results.filter((r: any) => over(r, "prod")).length, atOrOver090Test: results.filter((r: any) => over(r, "test")).length, over100Prod: results.filter((r: any) => r.prod.ok && r.prod.ratio > 1).length, touched: touched.size, seconds: (Date.now() - t0) / 1000, flagged: prodOver.length }));

for (const r of results as any[]) console.log(r.name, r.kind, "prod", r.prod.ok ? `${r.prod.files}f ${r.prod.bytes}b ${r.prod.lines}l ${r.prod.ratio.toFixed(3)} ${r.prod.dim}` : r.prod.error, "| test", r.test.ok ? `${r.test.files}f ${r.test.bytes}b ${r.test.lines}l ${r.test.ratio.toFixed(3)} ${r.test.dim}` : r.test.error);
