import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "@semio-tech/process/routing";

const REPO = process.env.AUDIT_FIX_REPO ?? "/Users/ueli/Documents/semio";
const OVERLAY = process.env.AUDIT_FIX_OVERLAY ?? "/private/tmp/audit-fix/overlay";
const INF = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference";
const argv = process.argv.slice(2), option = (name: string): string | undefined => { const at = argv.indexOf(name); return at < 0 ? undefined : argv[at + 1]; };
const roots = (option("--roots") ?? "").split(",").filter(Boolean), out = option("--out"), limit = Number(option("--limit") ?? "0"), manifests = option("--manifests"), swept = option("--swept");
if (roots.length === 0 || out === undefined) throw new Error("refused: --roots (explicit root list) and --out are required");
const listed = roots.flatMap(root => execFileSync("git", ["-C", REPO, "ls-files", "-z", "--", ...(root === "." ? [":(top)📋️project.json"] : [`${root}/**/📋️project.json`, `${root}/📋️project.json`])], { encoding: "utf8", maxBuffer: 1 << 28 }).split("\0").filter(Boolean));
const files = [...new Set(listed)].sort();
if (files.length === 0) throw new Error("refused: empty scope");
const offset = Number(option("--offset") ?? "0"), selected = manifests !== undefined ? JSON.parse(readFileSync(manifests, "utf8")) as string[] : limit > 0 ? files.slice(offset, offset + limit) : files.slice(offset);
const plugin = await import(`${REPO}/${INF}/🟨️.mjs`);
await plugin.libraryBootstrap;
const live = async (name: string) => import(`${REPO}/${INF}/${name}/🟨️.mjs`);
const registry = { bootstrap: await live("🔁️bootstrap"), nativeManifests: await live("🦀️native-manifests"), coordinates: await live("📍️coordinates"), sourceAnalyses: await live("🧬️source-analyses"), scriptInputs: await live("🧭️script-inputs"), generatorOutputs: await live("🏭️generator-outputs"), targetPolicy: await live("🎯️target-policy"), componentTargets: await live("🧩️component-targets") };
const taxonomy = join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const contracts = existsSync(taxonomy) ? registry.sourceAnalyses.readTaxonomyProjectionV1(taxonomy).generatorContracts : {};
const assembly = await import(`${OVERLAY}/${INF}/🧱️project-assembly/🟨️.mjs`);
assembly.wire(registry);
(registry.bootstrap as any).loaded.cargoLockInput = (manifest: string, workspaceRoot: string) => `{workspaceRoot}/${manifest}.lock-probe`;
const rows: unknown[] = [], failures: { path: string; error: string }[] = [];
await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "audit-fix-probe", maximumElapsedMilliseconds: 3_600_000 }, {}, Date.now()), async invocation => {
  const control = createIssuedResourceControlV1(invocation, "graph:project-source", 0);
  const scripts = plugin.createScriptInputCache({ workspaceRoot: REPO, invocation, phaseControl: control });
  for (const path of selected) {
    const text = readFileSync(swept !== undefined && existsSync(join(swept, path)) ? join(swept, path) : join(REPO, path), "utf8"), json = JSON.parse(text), root = json.root ?? dirname(path);
    (globalThis as any).__execClass = []; (globalThis as any).__classifyOnly = true;
    try {
      const project = await assembly.projectWithDefaults(json, root, join(REPO, root), REPO, contracts, new Map(), undefined, scripts, undefined, "source");
      rows.push({ path, sha256: createHash("sha256").update(readFileSync(join(REPO, path))).digest("hex"), name: json.name, classes: (globalThis as any).__execClass, output: createHash("sha256").update(JSON.stringify(project, (key, value) => key === "execution" && value?.version === 1 && typeof value?.kind === "string" ? undefined : value)).digest("hex") });
    } catch (error) { failures.push({ path, error: String((error as Error)?.stack ?? error).slice(0, 1800) }); }
  }
});
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, JSON.stringify({ roots, total: files.length, offset, count: selected.length, rows, failures }, null, 1));
console.log(JSON.stringify({ projects: selected.length, rows: rows.length, failures: failures.length }));
