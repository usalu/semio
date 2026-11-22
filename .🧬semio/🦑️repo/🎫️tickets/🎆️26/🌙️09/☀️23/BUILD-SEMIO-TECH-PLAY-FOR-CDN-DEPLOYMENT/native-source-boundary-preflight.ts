import { readFileSync, existsSync, readdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join, relative, resolve } from "node:path";
import { admitCargoSourceBoundariesV1, mergeCargoCfgFeaturesV1, type CargoSourceAdmissionPortV1, type CargoSourceBoundaryPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import { createPhysicalCargoSourceReaderV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import type { CargoCompilationGraphV1, CargoCompilationUnitV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts";
import { isRepoErrorV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";

const skipped = new Set(["node_modules", "target", "dist", ".cache", "🧪️tests", "🤖️generated"]);

/** 🔎️ Lists every Cargo manifest below the directories, skipping build output, tests and generated trees. */
export function listCargoManifestsV1(root: string, dirs: readonly string[]): string[] {
  const manifests: string[] = [];
  const walk = (d: string): void => { for (const e of readdirSync(d, { withFileTypes: true })) { if (skipped.has(e.name) || e.name.startsWith(".")) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (e.name === "Cargo.toml") manifests.push(p); } };
  for (const d of dirs) walk(join(root, d));
  return manifests;
}

/** 🧬️ The default feature closure of one manifest, the features Cargo enables for a plain selection of the crate. */
export function defaultFeaturesV1(features: Readonly<Record<string, readonly string[]>> | undefined): string[] {
  const enabled = new Set<string>(), visit = (name: string): void => {
    if (name.startsWith("dep:") || name.includes("/") || enabled.has(name)) return;
    enabled.add(name);
    for (const next of features?.[name] ?? []) visit(next);
  };
  visit("default");
  enabled.delete("default");
  return [...enabled].sort();
}

/** 🧱️ Builds the workspace compilation units the real admission would receive for one manifest: the library and, when present, the build script. Everything admission reads from a unit is set; Cargo's own planning is the only input not reproduced. */
export function unitsOfManifestV1(manifest: string, text: string): CargoCompilationUnitV1[] {
  const toml = Bun.TOML.parse(text) as any, name: string | undefined = toml.package?.name;
  if (!name) return [];
  const directory = dirname(manifest), boundary = toml.package?.metadata?.semio?.["rust-source-boundary"] ?? null;
  const lib = toml.lib?.path ?? (existsSync(join(directory, "src/lib.rs")) ? "src/lib.rs" : undefined), build = toml.package?.build === false ? undefined : toml.package?.build ?? (existsSync(join(directory, "build.rs")) ? "build.rs" : undefined);
  const features = defaultFeaturesV1(toml.features), common = { packageId: name, packageName: name, packageVersion: "0.0.0", manifest, provenance: { kind: "workspace", identity: name }, sourceInputs: [], compilerCfg: null, compilerEnvironment: null, platform: null, compilerArguments: [], compilerFlags: [], features, dependencies: [], boundary, support: { kind: "unsupported", reason: null } };
  return [
    ...lib ? [{ ...common, id: name + ":lib", name, kind: toml.lib?.["proc-macro"] ? "proc-macro" : "library", operation: "build", entry: join(directory, lib) }] : [],
    ...build ? [{ ...common, id: name + ":build", name: "build-script-build", kind: "build-script", operation: "build", entry: join(directory, build) }] : [],
  ] as unknown as CargoCompilationUnitV1[];
}

/** 🔌️ The admission port over the physical no-follow reader the release uses, with the compiler cfg printed once per profile. */
export function createPreflightPortV1(release: boolean, target?: string): CargoSourceAdmissionPortV1 {
  const reader = createPhysicalCargoSourceReaderV1(), args = ["--print", "cfg", ...release ? ["-C", "debug-assertions=off"] : [], ...target ? ["--target", target] : []];
  let printed: readonly string[] | undefined;
  return {
    async recheck(control) { await reader.sweep(control); },
    async cfg(unit) {
      if (!printed) { const run = spawnSync("rustc", args, { encoding: "utf8" }); if (run.status !== 0) throw Error("rustc --print cfg failed: " + run.stderr); printed = Object.freeze(run.stdout.trim().split("\n")); }
      return [...mergeCargoCfgFeaturesV1(printed, unit.features)];
    },
    read: (path, control, request) => reader.read(path, request, control),
  };
}

export type CrateVerdictV1 = Readonly<{ kind: "within" } | { kind: "over"; line: string } | { kind: "error"; line: string }>;

/** 🧮️ Runs the real `admitCargoSourceBoundariesV1` over the crate's units and renders the refusal with the numbers admission prints. */
export async function admitManifestV1(root: string, manifest: string, policy: CargoSourceBoundaryPolicyV1, port: CargoSourceAdmissionPortV1): Promise<CrateVerdictV1> {
  let units: CargoCompilationUnitV1[];
  try { units = unitsOfManifestV1(manifest, readFileSync(manifest, "utf8")); } catch (error) { return { kind: "error", line: `ERROR\t${relative(root, manifest)}\t${String((error as Error).message).slice(0, 300)}` }; }
  const control = { signal: new AbortController().signal, remainingMilliseconds: () => null, publish() { }, async yieldContinuation() { } };
  const graph: CargoCompilationGraphV1 = { version: 1, workspaceRoot: root, units, roots: units.map(unit => unit.id) };
  const where = relative(root, manifest);
  try { await admitCargoSourceBoundariesV1(graph, root, policy, control, port); return { kind: "within" }; } catch (error) {
    const crate = units[0]?.packageName ?? where, cause = (error as { cause?: unknown }).cause;
    if (isRepoErrorV1(error) && error.key === "process.source.boundary-exceeded") return { kind: "over", line: `OVER\t${crate}\t${error.key} ${JSON.stringify(error.values)}\t${isRepoErrorV1(cause) ? cause.key + " " + JSON.stringify(cause.values) : "-"}\t${where}` };
    if (error instanceof Error && error.message.startsWith("Rust crate Source boundary exceeded before compiler")) return { kind: "over", line: `OVER\t${crate}\t${error.message}\t-\t${where}` };
    return { kind: "error", line: `ERROR\t${where}\t${isRepoErrorV1(error) ? error.key + " " + JSON.stringify(error.values) : String((error as Error).message)}`.slice(0, 600) };
  }
}

/** 📐️ The repository admission policy the release passes to admission. */
export function readSourceBoundaryPolicyV1(root: string): CargoSourceBoundaryPolicyV1 {
  const policy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
  return (function find(o: any): any { if (o && typeof o === "object") { if (o.sourceBoundary?.maxSourceFiles) return o.sourceBoundary; for (const v of Object.values(o)) { const r = find(v); if (r) return r; } } })(policy);
}

/** 🧮️ Lists every workspace crate the real source admission refuses. Usage: native-source-boundary-preflight.ts <root> [dir...]; PREFLIGHT_PROFILE=debug and PREFLIGHT_TARGET=<triple> select the compiler cfg. */
if (import.meta.main) {
  const root = resolve(process.argv[2]!), manifests = listCargoManifestsV1(root, process.argv.slice(3)), policy = readSourceBoundaryPolicyV1(root), release = process.env.PREFLIGHT_PROFILE !== "debug";
  let next = 0;
  await Promise.all(Array.from({ length: 6 }, async () => {
    const port = createPreflightPortV1(release, process.env.PREFLIGHT_TARGET || undefined);
    for (let at = next++; at < manifests.length; at = next++) { const verdict = await admitManifestV1(root, manifests[at]!, policy, port); if (verdict.kind !== "within") console.log(verdict.line); }
  }));
}
