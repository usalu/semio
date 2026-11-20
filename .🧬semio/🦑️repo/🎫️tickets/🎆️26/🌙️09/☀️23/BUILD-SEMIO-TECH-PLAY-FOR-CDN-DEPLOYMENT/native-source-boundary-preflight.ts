import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

/** 🧮️ Lists every workspace crate whose non-test Rust source closure exceeds its declared or default source boundary. Usage: native-source-boundary-preflight.ts <root> [dir...] */
const root = resolve(process.argv[2]!), dirs = process.argv.slice(3), policy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const fallback = (function find(o: any): any { if (o && typeof o === "object") { if (o.sourceBoundary?.maxSourceFiles) return o.sourceBoundary; for (const v of Object.values(o)) { const r = find(v); if (r) return r; } } })(policy);
const manifests: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d, { withFileTypes: true })) { if (["node_modules", "target", "dist", ".cache", "🧪️tests", "🤖️generated"].includes(e.name) || e.name.startsWith(".")) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
for (const d of dirs) walk(join(root, d));
for (const manifest of manifests) {
  const text = readFileSync(manifest, "utf8"); if (!/^\[package\]/m.test(text)) continue;
  const lib = /\[lib\][^\[]*?path\s*=\s*"([^"]+)"/s.exec(text)?.[1] ?? (existsSync(join(dirname(manifest), "src/lib.rs")) ? "src/lib.rs" : undefined); if (!lib) continue;
  const block = /rust-source-boundary\]\s*([^\[]*)/s.exec(text)?.[1] ?? /rust-source-boundary\s*=\s*\{([^}]*)\}/.exec(text)?.[1] ?? "";
  const num = (k: string): number => Math.min(fallback[k], Number(new RegExp(`${k}\\s*=\\s*(\\d+)`).exec(block)?.[1] ?? fallback[k]));
  let files: readonly { path?: string }[] = [];
  try { files = await readRustSourceClosureV1([join(dirname(manifest), lib)], root, [], async path => existsSync(path) && statSync(path).isFile() ? new Uint8Array(readFileSync(path)) : null); } catch (error) { console.log(`ERROR\t${manifest.slice(root.length + 1)}\t${String((error as Error).message).slice(0, 120)}`); continue; }
  const bytes = files.reduce((s, f: any) => s + (f.bytes ?? f.content?.byteLength ?? 0), 0);
  if (files.length > num("maxSourceFiles") || bytes > num("maxSourceBytes")) console.log(`OVER\t${files.length}/${num("maxSourceFiles")}\t${bytes}/${num("maxSourceBytes")}\t${manifest.slice(root.length + 1)}`);
}
