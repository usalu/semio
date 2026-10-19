import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { LIB, LIB_REL, PKG_DIR, PKG_NAME, ROOT, posix, writeJson, GENERATED } from "./lib.ts";
import { classify, type ClassifyResult } from "./classify.ts";
import { DOMAINS, SUB_PURPOSE, entryOf, domainOf, type DomainId, type Glue } from "./rules.ts";
import { exportsOf } from "./exports.ts";

const EMOJI: Record<DomainId, string> = { laws: "⚖️", inventory: "📇️", authoring: "🏭️", testing: "🧪️", process: "🏃️", cargo: "🦀️", artifacts: "📦️", storage: "💾️", build: "🏗️", dev: "🎮️", policy: "🚨️" };

export type GlueFile = { declaration?: string; domain: DomainId; glue: Glue; key: string; abs: string; content: string; members: { target: string; names: string[]; types: string[]; json?: string }[]; conflicts: string[]; missing: string[] };

const relTo = (from: string, to: string): string => { const r = posix(relative(dirname(from), to)); return r.startsWith(".") ? r : "./" + r; };

export function buildGlue(r: ClassifyResult): GlueFile[] {
  const byGlue = new Map<string, { domain: DomainId; glue: Glue; key: string; abs: string; targets: string[] }>();
  for (const t of [...r.bTargets.keys()].sort()) {
    const e = entryOf(t);
    const row = byGlue.get(e.glueAbs) ?? { domain: e.domain, glue: e.glue, key: e.key, abs: e.glueAbs, targets: [] };
    byGlue.set(e.glueAbs, row);
    row.targets.push(t);
  }
  const wanted = (t: string): string[] => {
    const row = r.bTargets.get(t)!;
    if (row.json || t.endsWith(".cjs")) return [];
    const ex = exportsOf(resolve(LIB, t));
    let want = [...row.names].filter(n => n !== "default");
    if (row.star && !t.endsWith(".mjs")) want = [...new Set([...want, ...ex.values, ...ex.types])].filter(n => n !== "default");
    return want.sort();
  };
  const out: GlueFile[] = [];
  for (const g of [...byGlue.values()].sort((a, b) => a.key.localeCompare(b.key))) {
    const owner = new Map<string, string>();
    const conflicts: string[] = [], missing: string[] = [];
    if (g.glue === "ts" || g.glue === "mjs") {
      for (const t of g.targets) {
        for (const n of wanted(t)) {
          const previous = owner.get(n);
          if (previous === undefined) { owner.set(n, t); continue; }
          const declares = (x: string): boolean => exportsOf(resolve(LIB, x)).declared.has(n);
          if (declares(t) && !declares(previous)) owner.set(n, t);
          else if (declares(t) === declares(previous) && !(g.glue === "mjs")) conflicts.push(`${n}: ${previous} vs ${t}`);
        }
      }
      for (const t of g.targets) {
        const row = r.bTargets.get(t)!;
        if (row.json) {
          const name = r.jsonNames.get(t)!;
          if (owner.has(name)) conflicts.push(`${name}: ${owner.get(name)} vs ${t}`);
          owner.set(name, t);
        }
      }
    }
    const lines: string[] = [];
    const declarations: string[] = [];
    let declarable = g.glue === "mjs";
    const members: GlueFile["members"] = [];
    for (const t of g.targets) {
      const row = r.bTargets.get(t)!;
      const abs = resolve(LIB, t);
      const spec = relTo(g.abs, abs);
      if (g.glue === "cjs") { lines.push(`Object.assign(module.exports, require(${JSON.stringify(spec)}));`); members.push({ target: t, names: ["*"], types: [] }); continue; }
      if (row.json) {
        const name = r.jsonNames.get(t)!;
        lines.push(`export { default as ${name} } from ${JSON.stringify(spec)} with { type: "json" };`);
        members.push({ target: t, names: [name], types: [], json: name });
        continue;
      }
      if (g.glue === "mjs") {
        const runtime = exportsOf(abs).values;
        const typed = existsSync(abs.replace(/\.mjs$/u, ".d.mts"));
        if (row.star || row.names.size === 0 || wanted(t).some(n => !runtime.has(n))) {
          lines.push(`export * from ${JSON.stringify(spec)};`);
          if (typed) declarations.push(`export * from ${JSON.stringify(spec)};`); else declarable = false;
          members.push({ target: t, names: ["*"], types: [] });
          continue;
        }
        const names = wanted(t).filter(n => owner.get(n) === t);
        if (names.length) {
          lines.push(`export { ${names.join(", ")} } from ${JSON.stringify(spec)};`);
          declarations.push(typed ? `export { ${names.join(", ")} } from ${JSON.stringify(spec)};` : names.map(n => `export declare const ${n}: any;`).join("\n"));
        }
        members.push({ target: t, names, types: [] });
        continue;
      }
      const ex = exportsOf(abs);
      const values: string[] = [], types: string[] = [];
      for (const n of wanted(t)) {
        if (owner.get(n) !== t) continue;
        if (ex.values.has(n)) values.push(n);
        else if (ex.types.has(n)) types.push(n);
        else { values.push(n); missing.push(`${t}#${n}`); }
      }
      if (values.length) lines.push(`export { ${values.join(", ")} } from ${JSON.stringify(spec)};`);
      if (types.length) lines.push(`export type { ${types.join(", ")} } from ${JSON.stringify(spec)};`);
      members.push({ target: t, names: values, types });
    }
    const d = DOMAINS[g.domain];
    const doc = `${EMOJI[g.domain]} Public \`${PKG_NAME}${g.key.slice(1)}\` entry (${SUB_PURPOSE[g.key] ?? d.purpose}); explicit re-exports of the library paths that own each member, nothing is defined here.`;
    out.push({ domain: g.domain, glue: g.glue, key: g.key, abs: g.abs, content: `/** ${doc} */\n${lines.join("\n")}\n`, ...(declarable ? { declaration: `/** ${doc} */\n${declarations.join("\n")}\n` } : {}), members, conflicts, missing });
  }
  return out;
}

if (import.meta.main) {
  const write = process.argv.includes("--write");
  const r = classify();
  const glue = buildGlue(r);
  let bad = 0;
  for (const g of glue) {
    const rel = posix(relative(ROOT, g.abs));
    console.log(`${g.key.padEnd(24)} ${g.members.reduce((n, m) => n + m.names.length + m.types.length, 0).toString().padStart(4)} names in ${g.members.length} files  ${rel.slice(LIB_REL.length)}`);
    for (const c of g.conflicts) { console.log("   CONFLICT", c); bad++; }
    for (const m of g.missing) console.log("   MISSING", m);
    if (write) { mkdirSync(dirname(g.abs), { recursive: true }); writeFileSync(g.abs, g.content); if (g.glue === "mjs") { const d = g.abs.replace(/\.mjs$/u, ".d.mts"); if (g.declaration) writeFileSync(d, g.declaration); else if (existsSync(d)) rmSync(d); } }
  }
  writeJson(resolve(GENERATED, "glue.json"), glue.map(g => ({ key: g.key, domain: g.domain, glue: g.glue, file: posix(relative(ROOT, g.abs)), members: g.members })));
  console.log(write ? "written" : "dry-run", glue.length, "glue files,", bad, "conflicts");
  void existsSync; void readFileSync; void PKG_DIR; void domainOf;
}
