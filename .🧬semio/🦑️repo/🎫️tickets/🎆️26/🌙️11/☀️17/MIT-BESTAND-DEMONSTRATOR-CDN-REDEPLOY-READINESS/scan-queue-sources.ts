import { readFile } from "node:fs/promises";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync } from "node:child_process";

const root = process.cwd();
const framework = "🧰️framework/🔨️modules";
const native = join(root, framework, "🏃️process/📦️artifacts/🏗️native-build");
const { readRustSourceClosureV1, createRustSourceAnalysisV1 } = await import(join(native, "📥️source/🟦️.ts"));
const { cargoSourceBoundaryExcessV1, effectiveCargoSourceBoundaryV1, mergeCargoCfgFeaturesV1 } = await import(join(native, "📥️source/🎟️admission/🟦️.ts"));
const { validateJsonSchemaSubset } = await import(join(root, framework, "🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"));
const unitSchema = JSON.parse(await readFile(join(native, "🕸️units/🧬️schema/🔣️.json"), "utf8"));
const boundarySchema = unitSchema.properties.units.items.properties.boundary;
const policy = JSON.parse(await readFile(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8")).storage.sourceBoundary;
const admissionSchema = JSON.parse(await readFile(join(native, "📥️source/🎟️admission/🧬️schema/🔣️.json"), "utf8"));
if (validateJsonSchemaSubset(admissionSchema, policy).length) throw new Error("Invalid repository Cargo Source boundary");

type Unit = {
  pkg_id: string;
  target: {name: string; src_path: string; kind: string[]; edition?: string};
  profile: {opt_level: string; debug_assertions: boolean | null; panic?: string};
  mode?: string;
  features: string[];
  platform: string | null;
};
type Boundary = {maxSourceFiles: number; maxSourceBytes: number; maxActiveLines: number};

const analysis = createRustSourceAnalysisV1();
const measured = new Set<string>();
const configurations = new Map<string, string[]>();
const boundaries = new Map<string, Boundary | null>();
const warnings = [];
const refused = [];
const graphs = process.argv.slice(2);
if (!graphs.length) throw new Error("Retained Cargo unit graphs required");

for (const graph of graphs) {
  const { units } = JSON.parse(await readFile(graph, "utf8")) as {units: Unit[]};
  for (const unit of units) {
    if (!unit.pkg_id.startsWith("path+file:")) continue;
    const directory = fileURLToPath(unit.pkg_id.slice(5).split("#")[0]!);
    const location = relative(root, directory);
    if (location === ".." || location.startsWith(".." + sep) || isAbsolute(location) || location === ".cache" || location.startsWith(".cache" + sep)) continue;
    const args = ["--print", "cfg", "-C", "opt-level=" + unit.profile.opt_level];
    if (unit.profile.debug_assertions !== null) args.push("-C", "debug-assertions=" + (unit.profile.debug_assertions ? "yes" : "no"));
    if (unit.profile.panic) args.push("-C", "panic=" + unit.profile.panic);
    if (unit.target.edition) args.push("--edition=" + unit.target.edition);
    if (unit.platform) args.push("--target", unit.platform);
    if (unit.target.kind.includes("proc-macro")) args.push("--crate-type=proc-macro");
    else if (unit.mode === "test" || unit.target.kind.includes("test") || unit.target.kind.includes("bench")) args.push("--test");
    const configKey = JSON.stringify(args);
    if (!configurations.has(configKey)) configurations.set(configKey, execFileSync(process.env.RUSTC || "rustc", args, { encoding: "utf8" }).trim().split(/\r?\n/u));
    const cfg = mergeCargoCfgFeaturesV1(configurations.get(configKey)!, unit.features);
    const entry = resolve(unit.target.src_path);
    const key = JSON.stringify([entry, cfg, directory]);
    if (measured.has(key)) continue;
    measured.add(key);
    const identity = { crate: unit.target.name, entry, manifest: join(directory, "Cargo.toml"), target: unit.platform, graph };
    try {
      if (!boundaries.has(directory)) {
        const manifest = Bun.TOML.parse(await readFile(identity.manifest, "utf8")) as {package?: {metadata?: {semio?: {"rust-source-boundary"?: Boundary}}}};
        const authored = manifest.package?.metadata?.semio?.["rust-source-boundary"] ?? null;
        if (validateJsonSchemaSubset(boundarySchema, authored).length) throw new Error("Invalid per-crate Cargo Source boundary");
        boundaries.set(directory, authored);
      }
      const limits = effectiveCargoSourceBoundaryV1(policy, boundaries.get(directory)!);
      const physical = new Map<string, number>();
      const files = await readRustSourceClosureV1([entry], root, cfg, async (path: string) => {
        try {
          const body = await readFile(path);
          physical.set(path, body.byteLength);
          return body;
        } catch (error) {
          if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
          throw error;
        }
      }, { manifestDirectory: directory, generated: () => undefined }, analysis);
      const bytes = Math.max(files.reduce((sum, file) => sum + file.bytes, 0), [...physical.values()].reduce((sum, length) => sum + length, 0));
      const observed = { ...identity, files: Math.max(files.length, physical.size), bytes, lines: files.reduce((sum, file) => sum + file.activeLines, 0), limits };
      const limitsExceeded = cargoSourceBoundaryExcessV1(limits, observed);
      if (limitsExceeded.length) refused.push({ ...observed, limitsExceeded });
      else if (observed.files >= 0.9375 * limits.maxSourceFiles || bytes >= 0.9375 * limits.maxSourceBytes || observed.lines >= 0.925 * limits.maxActiveLines) warnings.push(observed);
    } catch (error) {
      refused.push({ ...identity, error: String(error) });
    }
  }
}
console.log(JSON.stringify({ checked: measured.size, warnings, refused }, null, 2));
if (refused.length) process.exitCode = 1;
