/**
 * Glue files of the packages other than repo-lib, derived from exports-final.json and the live import edges:
 * one explicit re-export module per proposed entry (JSON documents get a TS wrapper), plus the names an existing
 * glue file still lacks. Dry by default; the deps executor decides whether to materialize them.
 *
 *   bun repoapi-tools/glue-others.ts [--write] [--only=<package>]
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { GENERATED, ROOT, posix, readJson, writeJson, type Edge } from "./lib.ts";
import { exportsOf } from "./exports.ts";
import { packageIndex } from "./packages.ts";

export type OtherGlue = { file: string; package: string; subpath: string; content: string; kind: "module" | "json" };
export type GlueExtension = { file: string; package: string; subpath: string; add: { values: string[]; types: string[] }; star?: boolean };

const relTo = (from: string, to: string): string => { const r = posix(relative(dirname(from), to)); return r.startsWith(".") ? r : "./" + r; };

export function extensionText(x: GlueExtension, final: any): string {
  const abs = resolve(ROOT, x.file);
  const entry = Object.values<any>(final.packages[x.package].entries).find((e: any) => e.glueFile === x.file);
  const spec = relTo(abs, resolve(ROOT, entry.targetFile));
  const lines = [...(x.star ? [`export * from ${JSON.stringify(spec)};`] : []), ...(x.add.values.length ? [`export { ${x.add.values.join(", ")} } from ${JSON.stringify(spec)};`] : []), ...(x.add.types.length ? [`export type { ${x.add.types.join(", ")} } from ${JSON.stringify(spec)};`] : [])];
  return readFileSync(abs, "utf8").replace(/\n?$/u, "\n") + lines.join("\n") + "\n";
}

export function buildOtherGlue(only?: string, force = false): { create: OtherGlue[]; extend: GlueExtension[]; unnamed: string[] } {
  const final = readJson<any>(resolve(import.meta.dir, "exports-final.json"));
  const scanFile = resolve(GENERATED, "apply-edges.json");
  const edges: Edge[] = existsSync(scanFile) ? readJson<{ edges: Edge[] }>(scanFile).edges : [];
  const byTarget = new Map<string, Edge[]>();
  for (const e of edges) (byTarget.get(e.target) ?? byTarget.set(e.target, []).get(e.target)!).push(e);
  const index = packageIndex();
  const create: OtherGlue[] = [], extend: GlueExtension[] = [], unnamed: string[] = [];
  for (const [name, p] of Object.entries<any>(final.packages)) {
    if (only && name !== only) continue;
    const pkg = index.byName.get(name);
    for (const e of Object.values<any>(p.entries)) {
      if (e.existing || !e.glueFile) continue;
      const abs = resolve(ROOT, e.glueFile), target = resolve(ROOT, e.targetFile);
      if (!existsSync(target)) { unnamed.push(`${name}${e.subpath.slice(1)} (target vanished: ${e.targetFile})`); continue; }
      const owner = pkg ? pkg.sourceOwner : "";
      const used = (byTarget.get(e.targetFile) ?? []).filter(x => !owner || !(x.file === owner || x.file.startsWith(owner + "/")));
      const spec = relTo(abs, target);
      if (e.kind === "json") {
        if (force || !existsSync(abs)) create.push({ file: e.glueFile, package: name, subpath: e.subpath, kind: "json", content: `/** 🧩️ Public \`${name}${e.subpath.slice(1)}\` entry: the JSON document, re-exported unchanged. */\nimport document from ${JSON.stringify(spec)} with { type: "json" };\nexport default document;\n` });
        continue;
      }
      const ex = exportsOf(target);
      const star = e.symbols === "*" || used.some(x => x.star);
      const wanted = new Set<string>(star ? [] : [...(e.symbols as string[]), ...used.flatMap(x => x.names)].filter(n => n !== "*" && n !== "default"));
      const noName = !star && wanted.size === 0;
      if (noName) unnamed.push(`${name}${e.subpath.slice(1)} (${e.targetFile})`);
      const values = [...wanted].filter(n => ex.values.has(n)).sort(), types = [...wanted].filter(n => !ex.values.has(n) && ex.types.has(n)).sort();
      const missing = [...wanted].filter(n => !ex.values.has(n) && !ex.types.has(n));
      if (existsSync(abs) && !(force && readFileSync(abs, "utf8").startsWith("/** 🧩️ Public `"))) {
        const have = exportsOf(abs);
        const addValues = values.filter(n => !have.values.has(n)), addTypes = types.filter(n => !have.types.has(n) && !have.values.has(n));
        if (addValues.length || addTypes.length) extend.push({ file: e.glueFile, package: name, subpath: e.subpath, add: { values: addValues, types: addTypes } });
        continue;
      }
      const javascript = /\.[cm]?js$/u.test(e.glueFile);
      const lines = star || noName || (javascript && types.length) ? [`export * from ${JSON.stringify(spec)};`] : [
        ...(values.length ? [`export { ${values.join(", ")} } from ${JSON.stringify(spec)};`] : []),
        ...(types.length ? [`export type { ${types.join(", ")} } from ${JSON.stringify(spec)};`] : []),
      ];
      const content = `/** 🧩️ Public \`${name}${e.subpath.slice(1)}\` entry: explicit re-exports of the source module that owns each member. */\n${lines.join("\n")}\n`;
      create.push({ file: e.glueFile, package: name, subpath: e.subpath, kind: "module", content });
      if (javascript && /\.mjs$/u.test(e.glueFile) && existsSync(target.replace(/\.mjs$/u, ".d.mts"))) create.push({ file: e.glueFile.replace(/\.mjs$/u, ".d.mts"), package: name, subpath: e.subpath, kind: "module", content });
    }
  }
  return { create, extend, unnamed };
}

if (import.meta.main) {
  const write = process.argv.includes("--write");
  const only = process.argv.find(a => a.startsWith("--only="))?.slice(7);
  const force = process.argv.includes("--force");
  const { create, extend, unnamed } = buildOtherGlue(only, force);
  console.log(`glue files to create ${create.length} (${create.filter(c => c.kind === "json").length} JSON wrappers), existing glue to extend ${extend.length}, entries without a used symbol ${unnamed.length}`);
  writeJson(resolve(GENERATED, "glue-others.json"), { create, extend, unnamed });
  if (write) {
    for (const g of create) { const abs = resolve(ROOT, g.file); if (!force && existsSync(abs)) continue; mkdirSync(dirname(abs), { recursive: true }); writeFileSync(abs, g.content); }
    const final = readJson<any>(resolve(import.meta.dir, "exports-final.json"));
    for (const x of extend) writeFileSync(resolve(ROOT, x.file), extensionText(x, final));
  }
}
