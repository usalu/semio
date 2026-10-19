/** 🧭 Per-target cache decision for ✏️s: flip a deterministic target on, pin a hazardous one off, or leave it. */
import { readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { ROOT, classify, effectiveCache, nativeName, type ProjectFacts } from "./lib.ts";
import { keepReason } from "./rules.ts";

export type Decision = { cache?: boolean; outputs?: string[]; inputs?: unknown[]; note: string };

const FORCE_OFF = new Set(["fixture", "regenerate-example-assets", "test-bestest-regenerate", "tiles-prefetch", "🧪️witness"]);
const strip = (s: string) => s.replace(/️/gu, "");
const GIS_CHECKS = new Set(["inference-check", "inference-presentation-check", "inference-bridge-source-check", "inference-bridge-process-check", "inference-service-receiving-check", "cold-document-pair-check"]);

const lockInput = (p: ProjectFacts): string => {
  const workspace = /^workspace\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(ROOT, p.root, "Cargo.toml"), "utf8"))?.[1];
  const owner = workspace ? relative(ROOT, resolve(ROOT, p.root, workspace)) : p.root;
  return `{workspaceRoot}/${owner ? owner + "/" : ""}Cargo.lock`;
};

export function decide(p: ProjectFacts, name: string, t: any): Decision | undefined {
  const cls = classify(name, t);
  const eff = effectiveCache(name, t).cache;
  const reason = keepReason(name);
  const key = `${p.file}#${strip(name)}`;
  if (p.file.includes("📐️cad/📦️packages/🟦️typescript") && name === "generate") return eff ? { cache: false, note: "keep: regenerates a checked-in example through the cad engine closure" } : undefined;
  if (reason || FORCE_OFF.has(name)) return eff ? { cache: false, note: `pin off: ${reason ?? "regenerates checked-in files or touches the network"}` } : undefined;
  const flip = cls === "deterministic-but-cache-off" || cls === "unclassified-cache-off";
  if (!flip && cls !== "deterministic-cacheable") return undefined;
  if (!flip) return cargoLockOnly(p, name, t);
  if (name === "wasm") return undefined;
  if (p.artifactTypeScript && (name === "build" || name === "test")) return undefined;
  const d: Decision = { cache: true, note: `flip: ${cls}` };
  if (name === "build" && p.rust && t.outputs === undefined) d.outputs = ["{projectRoot}/dist/build"];
  const generic = !(p.native && nativeName(name)) && !(p.artifactTypeScript && /^(?:build|check|test)$/u.test(name));
  if (generic && t.inputs === undefined) {
    if (name === "typecheck" && p.file.startsWith("✏️s/🧑‍💻dev/📋️project.json")) d.inputs = ["default", "^default", "{workspaceRoot}/✏️s/**/*.{ts,tsx,json}", "{workspaceRoot}/🧰️framework/**/*.{ts,tsx,json}", "{workspaceRoot}/tsconfig.json", { externalDependencies: ["typescript"] }];
    else if (GIS_CHECKS.has(name)) d.inputs = ["default", "^default", "{workspaceRoot}/✏️s/🔌️plugins/🌍️gis/**/*", "{workspaceRoot}/✏️s/🧑‍💻dev/🎭️variants/🌍️gis/**/*", { externalDependencies: ["vitest"] }];
    else if (name === "carrier-reproduce") { const base = p.file.slice(0, p.file.indexOf("/🏅️standards/") + "/🏅️standards".length); d.inputs = ["default", "^default", `{workspaceRoot}/${base}/**/*.json`, { externalDependencies: ["ajv"] }]; }
  }
  if (generic && p.native && t.inputs === undefined && d.inputs === undefined) d.inputs = ["default", "^default", lockInput(p)];
  void key;
  return d;
}

const cargoLockOnly = (p: ProjectFacts, name: string, t: any): Decision | undefined => {
  const generic = !(p.native && nativeName(name)) && !(p.artifactTypeScript && /^(?:build|check|test)$/u.test(name));
  if (!generic || !p.native || t.inputs !== undefined || !/^[^{]/u.test(name) || /generator/u.test(name)) return undefined;
  if (!t.options?.command || /^bun \.\/📜️script\.ts (graph-generate|preview-generated)/u.test(t.options.command)) return undefined;
  return { inputs: ["default", "^default", lockInput(p)], note: "inputs: cargo lock of the owning workspace" };
};
