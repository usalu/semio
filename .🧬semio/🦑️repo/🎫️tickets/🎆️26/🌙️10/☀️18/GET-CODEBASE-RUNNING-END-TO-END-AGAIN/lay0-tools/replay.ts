import { readFileSync, writeFileSync } from "node:fs";
import { builtinModules } from "node:module";
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";
import { loadDependencyDirectionPolicy } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";

const [, , reportPath, outPath] = process.argv;
const root = resolve(import.meta.dir, "../../../../../../../..");
const { validateDependency } = await import(new URL("../validate/index.mjs", pathToFileURL(Bun.resolveSync("dependency-cruiser", root))).href);
const { default: normalizeRules } = await import(new URL("./rule-set/normalize.mjs", pathToFileURL(Bun.resolveSync("dependency-cruiser", root))).href);
const snapshot = loadDependencyDirectionPolicy(root);
const stems = snapshot.taxonomy.bannedNameStems as readonly string[];
const names = ["no-core-path", "renderer-hosts-only-ui"];
const rules = normalizeRules({ forbidden: snapshot.policy.forbidden.filter((rule) => names.includes(rule.name)) });
const oldCoreFrom = new RegExp("^(✏️s/|🧰️framework/|🌎️hub/|♻️mit-bestand/)", "u");
const oldCoreTo = new RegExp(`(^|/)([^/]*?)(${stems.join("|")})(/|$)`, "u");
const oldCoreNot = /node_modules|target|\/pkg\//u;
const rendererRoot = "^🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/";
const builtin = `^(?:${[...new Set(builtinModules.map((name) => name.replace(/^node:/, "")))].join("|")})(?:$|/)`;
const oldRenderer = [rendererRoot, "^🧰️framework/🔨️modules/🖱️ui/", "^🧰️framework/📦️packages/", "^node_modules/react(?:-dom)?/", "^(?:node:|vitest/)", "^node_modules/(?:vite|vitest)/", builtin].map((pattern) => new RegExp(pattern, "u"));
const rendererFrom = new RegExp(rendererRoot, "u");
const strip = (value: string): string => value.replace(/[^\x00-\x7f]/gu, "");
const stemSegment = (path: string): string => path.split("/").map((segment) => strip(segment).replace(/\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$/u, "").toLowerCase()).find((stem) => stems.includes(stem)) ?? "";

type Counter = Record<string, number>;
const bump = (counter: Counter, key: string): void => { counter[key] = (counter[key] ?? 0) + 1; };
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const out = { edges: 0, oldCore: 0, newCore: 0, oldRenderer: 0, newRenderer: 0, oldCoreNotNew: {} as Counter, newCoreNotOld: {} as Counter, oldRendererNotNew: {} as Counter, newRendererNotOld: {} as Counter, remainingCore: {} as Counter, remainingRendererTargets: {} as Counter };
for (const module of report.modules) {
  for (const dependency of module.dependencies) {
    out.edges++;
    const from: string = module.source, to: string = dependency.resolved, types: string[] = dependency.dependencyTypes;
    const oldCore = oldCoreFrom.test(from) && oldCoreTo.test(to) && !oldCoreNot.test(to);
    const verdict = validateDependency(rules, { source: from }, { resolved: to, dependencyTypes: types });
    const hits = new Set((verdict.rules ?? []).map((rule: { name: string }) => rule.name));
    const newCore = hits.has("no-core-path"), newRenderer = hits.has("renderer-hosts-only-ui");
    const oldRendererHit = rendererFrom.test(from) && !oldRenderer.some((pattern) => pattern.test(to));
    if (oldCore) out.oldCore++;
    if (newCore) { out.newCore++; bump(out.remainingCore, stemSegment(to)); }
    if (oldRendererHit) out.oldRenderer++;
    if (newRenderer) { out.newRenderer++; bump(out.remainingRendererTargets, /node_modules\/\.bun\/([^/]+)/u.exec(to)?.[1] ?? strip(to.split("/").slice(0, 3).join("/"))); }
    if (oldCore && !newCore) {
      const kind = types.includes("core") ? "node-builtin" : types.includes("unknown") ? "unresolved-specifier" : stemSegment(to) === "" ? "compound-or-prefixed-name" : /(^|\/)[^/]*(?:🧱️|)base(\/|$)/u.test(to) && /🪆️subsets\/[^/]*base\//u.test(to) ? "subset-vocabulary-base" : "other";
      bump(out.oldCoreNotNew, kind);
    }
    if (newCore && !oldCore) bump(out.newCoreNotOld, /target/u.test(to) ? "path-contains-substring-target" : "other");
    if (oldRendererHit && !newRenderer) bump(out.oldRendererNotNew, /\.cache\//u.test(to) ? "bun-store-package" : to.startsWith("bun:") ? "bun-test-runner" : "other");
    if (newRenderer && !oldRendererHit) bump(out.newRendererNotOld, "other");
  }
}
writeFileSync(outPath!, JSON.stringify(out, null, 1));
console.log(JSON.stringify(out, null, 1));
