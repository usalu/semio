import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { admitCargoSourceBoundariesV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import { readPhysicalCargoSourceV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import { readCargoCompilationGraphV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts";
import fixtures from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };

/** ⏱️ Times admitCargoSourceBoundariesV1 over every workspace crate entry with a plain fs port versus the physical no-follow port. Usage: native-source-admission-bench.ts <root> <dir>... */
const root = resolve(process.argv[2]!), dirs = process.argv.slice(3), manifests: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d, { withFileTypes: true })) { if (["node_modules", "target", "dist", ".cache", "🧪️tests", "🤖️generated"].includes(e.name) || e.name.startsWith(".")) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
for (const d of dirs) walk(join(root, d));
const copies = Number(process.env.COPIES ?? 1), crates0 = manifests.flatMap(m => { const t = readFileSync(m, "utf8"); if (!/^\[package\]/m.test(t)) return []; const lib = ["🦀️.rs", "src/lib.rs"].map(f => join(dirname(m), f)).find(existsSync); return lib ? [{ manifest: m, entry: lib }] : []; });
const crates = Array.from({ length: copies }, () => crates0).flat();
const raw = fixtures.cases[fixtures.identicalUnits.case]!.units[fixtures.identicalUnits.unit]!;
const make = () => { const graph = readCargoCompilationGraphV1({ version: 1, units: crates.map(() => ({ ...structuredClone(raw), dependencies: [] })), roots: crates.map((_, i) => i) } as any, { ...fixtures.metadata, packages: fixtures.metadata.packages.map(p => ({ ...p, metadata: {} })) } as any, "/repo"); graph.units.forEach((u, i) => { u.entry = crates[i]!.entry; u.manifest = crates[i]!.manifest; u.packageName = "bench-" + i; }); return graph; };
const policy = { version: 1 as const, maxSourceBytes: 1 << 30, maxSourceFiles: 100000, maxActiveLines: 100000000 };
const stats = { publishes: 0, reads: 0, cfgs: 0 };
const control: any = { signal: new AbortController().signal, remainingMilliseconds: () => 60000, publish() { stats.publishes++; }, async yieldContinuation() { } };
for (const mode of ["plain", "physical", "physical-distinct-cfg"] as const) {
  Object.assign(stats, { publishes: 0, reads: 0, cfgs: 0 }); const graph = make(), t0 = performance.now();
  await admitCargoSourceBoundariesV1(graph, root, policy, control, { async recheck() { }, async cfg(u: any) { stats.cfgs++; return mode === "physical-distinct-cfg" ? ["u" + u.id] : []; }, async read(path, c, request) { stats.reads++; return mode === "plain" ? (existsSync(path) && statSync(path).isFile() ? new Uint8Array(readFileSync(path)) : null) : readPhysicalCargoSourceV1(path, request, c); } });
  console.log(JSON.stringify({ mode, units: crates.length, ...stats, files: graph.units.reduce((s, u) => s + u.sourceInputs.length, 0), ms: Math.round(performance.now() - t0) }));
}
