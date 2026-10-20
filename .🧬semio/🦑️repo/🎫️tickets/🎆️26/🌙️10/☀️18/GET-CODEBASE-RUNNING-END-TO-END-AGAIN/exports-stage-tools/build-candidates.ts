/**
 * 🧱 Turns the uncovered violation groups into export candidates: package, key, value, glue (create | extend | existing), conflicts.
 *
 *   bun build-candidates.ts   (reads generated/groups.json, writes generated/candidates.json)
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { GENERATED, ROOT, posix } from "./lib.ts";
import { abs, entriesOf, lane, model, originsOf, rel, siblingsOf, type Entry } from "./model.ts";
import { proposals } from "./inputs.ts";
import { loadPolicy, otherViolations } from "./rules.ts";
import { reexportsOf } from "./glue-ops.ts";
import type { Group } from "./groups.ts";

const LIB_ROOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const ascii = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "");
const stripExt = (s: string): string => s.replace(/\.(ts|tsx|mts|mjs|cjs|js)$/u, "");
const jsonKey = (segs: string[]): string[] => (segs.at(-1) === ".json" && segs.length > 1 ? [...segs.slice(0, -2), segs.at(-2)! + ".json"] : segs);

const DOMAINS: Record<string, { key: string; dir: string }> = {
  laws: { key: "./laws", dir: "⚖️laws" }, inventory: { key: "./inventory", dir: "📇️inventory" }, authoring: { key: "./authoring", dir: "🏗️authoring" }, testing: { key: "./testing", dir: "🧪️testing" },
  process: { key: "./process", dir: "🏃️process" }, cargo: { key: "./cargo", dir: "🦀️cargo" }, artifacts: { key: "./artifacts", dir: "📦️artifacts" }, storage: { key: "./storage", dir: "💾️storage" },
  build: { key: "./build", dir: "🏗️build" }, dev: { key: "./dev", dir: "🎮️playground" }, policy: { key: "./policy", dir: "🚨️policy" },
};
const PREFIX: [string, string][] = [["⚖️laws/", "laws"], ["🚨️policy/", "policy"], ["🧬️schema/", "inventory"], ["🧾️serialization/", "authoring"], ["🏭️generator/", "authoring"], ["🏗️authoring/", "authoring"], ["🏗️builder/", "authoring"], ["📽️projection/", "authoring"], ["🗂️workspaces/🦀️cargo/", "cargo"], ["🏃️process/🦀️cargo/", "cargo"], ["🏃️process/🕸️wasm/", "cargo"], ["⚡️caching/🦀️cargo/", "cargo"], ["⚡️caching/📦️artifacts/🏗️native-build/", "cargo"], ["⚡️caching/📦️artifacts/🦀️rust/", "cargo"], ["⚡️caching/📦️artifacts/🕸️wasm/", "cargo"], ["🏗️build/🕸️graph/🦀️rust/", "cargo"], ["🏗️build/🎚️mode/🦀️profile/", "cargo"], ["🏗️build/🎯️host-platform/", "cargo"], ["🏗️build/🕸️graph/📥️inference/🧪️tests/", "testing"], ["⚡️caching/🧪️tests/", "testing"], ["🧪️testing/", "testing"], ["⚡️caching/📦️artifacts/", "artifacts"], ["⚡️caching/📦️bundle/", "artifacts"], ["⚡️caching/💾️storage/", "storage"], ["⚡️caching/🚀️bootstrap/", "storage"], ["💾️development/", "storage"], ["⚡️caching/🌐️vite/", "dev"], ["🎮️playground/", "dev"], ["🌐️network/", "dev"], ["🏃️process/", "process"], ["⚡️caching/🟦️.ts", "process"], ["🏗️build/", "build"], ["🟨️.mjs", "build"], ["🔍️discovery/", "inventory"], ["🧹️normalization/", "inventory"], ["📇️catalog/", "inventory"], ["🤖️agent-clients/", "inventory"], ["🕸️dependencies/", "inventory"], ["🗂️workspaces/", "inventory"], ["🎛️commands/", "inventory"]];
const domainOf = (libRel: string): string | undefined => PREFIX.find(([p]) => libRel === p || libRel.startsWith(p))?.[1];

export type Candidate = {
  id: string; pkg: string; packageJson: string; packageDir: string; owner: string; target: string; key: string; value: string;
  glue: { path: string; mode: "create" | "extend" | "existing" | "none"; content?: string; add?: { values: string[]; types: string[] } };
  names: string[] | "*"; importers: string[]; edges: number; flags: string[]; proposals: { source: string; subpath?: string; pkg?: string }[]; conflicts: string[]; carriedRules: string[];
};

function repoLibPlacement(target: string): { key: string; glueRel: string } | null {
  if (!target.startsWith(LIB_ROOT + "/")) return null;
  const libRel = target.slice(LIB_ROOT.length + 1);
  const domain = domainOf(libRel);
  if (!domain) return null;
  const d = DOMAINS[domain]!;
  const segs = libRel.split("/");
  const at = segs.indexOf(d.dir);
  const stripped = at >= 0 ? segs.slice(at + 1) : segs;
  const tail = stripped.length === 1 && /^🟦️\.(ts|tsx)$/u.test(stripped[0]!) && at >= 0 ? segs : stripped;
  const leafIdx = tail.findIndex(x => /^🟦️\.(ts|tsx)$/u.test(x));
  const dirs = tail.filter((_, i) => i !== leafIdx);
  const isIndex = leafIdx >= 0;
  const json = target.endsWith(".json");
  const stem = json ? ascii(dirs[dirs.length - 1]!.replace(/\.json$/u, "")) : "";
  const base = json ? [...dirs.slice(0, -1)] : isIndex ? dirs : [...dirs.slice(0, -1), stripExt(dirs[dirs.length - 1]!)];
  const ext = /\.(mjs|cjs)$/u.test(target) ? target.slice(target.lastIndexOf(".")) : ".ts";
  const file = ext === ".ts" ? "🟦️.ts" : "🟨️" + ext;
  const renamed = base.map(x => (x === "🧪️tests" ? "🧪️testing" : x));
  const keyTail = jsonKey([...renamed.map(ascii).filter(Boolean), ...(json ? [stem ? stem + ".json" : ".json"] : [])]);
  const key = d.key + (ext === ".ts" ? "" : ext === ".mjs" ? "/tooling" : "/commonjs") + (keyTail.length ? "/" + keyTail.join("/") : "");
  return { key, glueRel: [d.dir, ...renamed, ...(json ? ["🔣️json", ...(stem ? [stem] : [])] : []), file].join("/") };
}

export function placement(pkg: ReturnType<typeof lane> & {}, target: string): { key: string; glueRel: string | null; valueRel: string } {
  const isJson = target.endsWith(".json");
  if (pkg.name === "@semio-tech/repo-lib") {
    const p = repoLibPlacement(target);
    if (p) return { key: p.key, glueRel: p.glueRel, valueRel: "./" + p.glueRel };
  }
  if (pkg.dir === pkg.sourceOwner || target.startsWith(pkg.dir + "/")) {
    const r = posix(target.slice(pkg.dir.length + 1));
    const segs = jsonKey(r.split("/").map(s => ascii(stripExt(s))).filter(Boolean));
    return { key: segs.length ? "./" + segs.join("/") : ".", glueRel: null, valueRel: "./" + r };
  }
  let ownerRel = target.startsWith(pkg.sourceOwner + "/") ? target.slice(pkg.sourceOwner.length + 1) : target;
  if (pkg.name === "@semio-tech/framework") ownerRel = ownerRel.replace(/^🔨️modules\//u, "");
  let keyRel: string;
  ownerRel = ownerRel.replace(/(^|\/)🧪️tests(?=\/|$)/gu, "$1🧪️testing");
  if (isJson) {
    const parts = ownerRel.split("/");
    const stem = ascii(parts[parts.length - 1]!.replace(/\.json$/u, ""));
    const dirs = parts.slice(0, -1);
    ownerRel = [...dirs, "🔣️json", ...(stem ? [stem] : []), "🟦️.ts"].join("/");
    keyRel = [...dirs.map(ascii).filter(Boolean), ...(stem ? [stem] : [])].join("/") + ".json";
  } else {
    ownerRel = ownerRel.replace(/(^|\/)🧪️tests(?=\/|$)/gu, "$1🧪️testing");
    const segs = ownerRel.split("/").map(s => ascii(stripExt(s))).filter(Boolean);
    const js = /\.(mjs|cjs)$/u.exec(target)?.[1];
    if (js && !ascii(stripExt(ownerRel.split("/").pop()!))) segs.push(js === "mjs" ? "tooling" : "commonjs");
    keyRel = segs.join("/");
  }
  return { key: keyRel ? "./" + keyRel : ".", glueRel: ownerRel, valueRel: "./" + ownerRel };
}

const relSpec = (from: string, to: string): string => { const p = posix(relative(dirname(abs(from)), abs(to))); return p.startsWith(".") ? p : "./" + p; };

export function glueContent(pkgName: string, key: string, glueAbsRel: string, target: string, names: string[] | "*"): { text: string; values: string[]; types: string[]; missing: string[] } {
  const spec = relSpec(glueAbsRel, target);
  const doc = `/** 🧩️ Public \`${pkgName}${key.slice(1)}\` entry: explicit re-exports of the source module that owns each member. */`;
  if (target.endsWith(".json")) return { text: `/** 🧩️ Public \`${pkgName}${key.slice(1)}\` entry: the JSON document, re-exported unchanged. */\nimport document from ${JSON.stringify(spec)} with { type: "json" };\nexport default document;\n`, values: ["default"], types: [], missing: [] };
  const origins = originsOf(target);
  if (!origins) return { text: "", values: [], types: [], missing: ["target unreadable"] };
  const all = [...origins.keys()].filter(k => !k.startsWith("*bare:"));
  const want = names === "*" ? all : names;
  const missing = want.filter(n => !origins.has(n));
  const have = want.filter(n => origins.has(n));
  const isType = (n: string): boolean => { const o = origins.get(n)!; return o.typeKind || o.via; };
  const values = have.filter(n => !isType(n) && n !== "default").sort(), types = have.filter(n => isType(n)).sort();
  const lines = [doc];
  if (have.includes("default") && !isType("default")) lines.push(`export { default } from ${JSON.stringify(spec)};`);
  if (values.length) lines.push(`export { ${values.join(", ")} } from ${JSON.stringify(spec)};`);
  if (types.length) lines.push(`export type { ${types.join(", ")} } from ${JSON.stringify(spec)};`);
  return { text: lines.join("\n") + "\n", values: have.includes("default") && !isType("default") ? ["default", ...values] : values, types, missing };
}

if (import.meta.main) {
  const groups: Group[] = JSON.parse(readFileSync(resolve(GENERATED, "groups.json"), "utf8"));
  const policy = loadPolicy();
  const ignoreRule = (name: string): boolean => name === "no-circular" || name === "not-to-unlisted" || name === "no-escaping-relative-imports" || name === "no-core-path" || name === "no-impl-segment";
  type Item = { g: Group; P: NonNullable<ReturnType<typeof lane>>; pl: ReturnType<typeof placement> };
  const merged = new Map<string, Item[]>();
  const skipped: { target: string; pkg: string | null; reason: string; edges: number; importers: string[] }[] = [];
  for (const g of groups.filter(x => !x.covering.length).sort((a, b) => a.target.localeCompare(b.target))) {
    const importers = [...new Set(g.importers.map(i => i.file))];
    const P = lane(g.owner);
    if (!P) { skipped.push({ target: g.target, pkg: g.pkg, reason: "owner has no TypeScript package lane", edges: g.importers.length, importers }); continue; }
    if (!/\.(ts|tsx|mts|js|mjs|cjs|json)$/u.test(g.target)) { skipped.push({ target: g.target, pkg: g.pkg, reason: "asset: a package export cannot point outside its directory", edges: g.importers.length, importers }); continue; }
    const pl = placement(P, g.target);
    const k = P.name + "\0" + pl.valueRel + "\0" + pl.key;
    (merged.get(k) ?? merged.set(k, []).get(k)!).push({ g, P, pl });
  }
  const takenKey = new Map<string, string>();
  const out: Candidate[] = [];
  for (const items of [...merged.values()].sort((a, b) => (a[0]!.P.name + a[0]!.pl.key).localeCompare(b[0]!.P.name + b[0]!.pl.key))) {
    const { P, pl } = items[0]!;
    const sources = items.map(i => i.g).filter(g => !(g.target.startsWith(P.dir + "/")));
    const viaGlue = items.map(i => i.g).filter(g => g.target.startsWith(P.dir + "/"));
    const all = items.map(i => i.g);
    const importers = [...new Set(all.flatMap(g => g.importers.map(i => i.file)))];
    const edges = all.reduce((n, g) => n + g.importers.length, 0);
    const props = [...new Map(all.flatMap(g => proposals.get(g.target) ?? []).map(p => [p.source + p.subpath + p.pkg, { source: p.source, subpath: p.subpath, pkg: p.pkg }])).values()];
    const flags: string[] = [];
    const conflicts: string[] = [];
    const carried = new Set<string>(), glueRules = new Set<string>();
    let key = pl.key;
    let n = 2;
    while ((P.exports[key] !== undefined && (P.exports[key] as unknown) !== pl.valueRel) || (takenKey.get(P.name + key) && takenKey.get(P.name + key) !== pl.valueRel)) { flags.push("key-collision:" + key); key = pl.key.replace(/(\.json)?$/u, m => `-${n++}${m}`); }
    takenKey.set(P.name + key, pl.valueRel);
    const gluePath = pl.glueRel ? posix(`${P.dir}/${pl.glueRel}`) : null;
    let glue: Candidate["glue"] = { path: gluePath ?? sources[0]?.target ?? viaGlue[0]!.target, mode: "none" };
    if (gluePath) {
      const exists = existsSync(abs(gluePath));
      const src = sources[0];
      const namesOf = (g: Group): string[] | "*" => g.names;
      const merge = (gs: Group[]): string[] | "*" => (gs.some(g => namesOf(g) === "*") ? "*" : [...new Set(gs.flatMap(g => namesOf(g) as string[]))].sort());
      if (src && sources.length > 1) flags.push("several-sources-for-one-glue:" + sources.map(g => g.target).join("|"));
      if (!src) glue = { path: gluePath, mode: exists ? "existing" : "none" };
      else if (exists) {
        const have = originsOf(gluePath);
        const origins = src.target.endsWith(".json") ? null : originsOf(src.target);
        const names = merge(sources);
        const want = names === "*" ? [...(origins?.keys() ?? [])].filter(k => !k.startsWith("*bare:")) : names;
        const lacking = src.target.endsWith(".json") ? [] : want.filter(nm => !have?.has(nm));
        const otherKey = Object.entries(P.exports).find(([k2, v]) => k2 !== key && typeof v === "string" && posix(resolve(abs(P.dir), v)) === abs(gluePath))?.[0];
        const serves = reexportsOf(gluePath).map(r => r.target).filter(Boolean);
        if (serves.length && !serves.includes(src.target) && !src.target.endsWith(".json")) flags.push("glue-serves-other-target:" + serves.slice(0, 2).join("|"));
        if (otherKey) flags.push("glue-already-exported-as:" + otherKey);
        const miss = lacking.filter(nm => !origins?.has(nm));
        if (miss.length) flags.push("names-missing-in-target:" + miss.join(","));
        const isT = (nm: string): boolean => { const o = origins?.get(nm); return !!o && (o.typeKind || o.via); };
        const lackV = lacking.filter(nm => origins?.has(nm) && !isT(nm)), lackT = lacking.filter(nm => origins?.has(nm) && isT(nm));
        glue = lacking.length ? { path: gluePath, mode: "extend", add: { values: lackV, types: lackT } } : { path: gluePath, mode: "existing" };
      } else {
        const names = merge(sources);
        const gc = glueContent(P.name, key, gluePath, src.target, names);
        if (gc.missing.length) flags.push("names-missing-in-target:" + gc.missing.join(","));
        glue = { path: gluePath, mode: "create", content: gc.text };
      }
    }
    const aliasPath = gluePath ?? sources[0]?.target ?? viaGlue[0]!.target;
    for (const g of sources) for (const imp of new Set(g.importers.map(i => i.file))) {
      const before = new Set(otherViolations(policy, imp, g.target, "local", ignoreRule));
      const after = otherViolations(policy, imp, aliasPath, "aliased-workspace", ignoreRule);
      for (const r of after) if (!before.has(r)) conflicts.push(`${r}: ${imp}`); else carried.add(r);
    }
    if ((glue.mode === "create" || glue.mode === "extend") && sources[0]) for (const r of otherViolations(policy, glue.path, sources[0].target, "local", ignoreRule)) glueRules.add(r);
    const names = all.some(g => g.names === "*") ? "*" : [...new Set(all.flatMap(g => g.names as string[]))].sort();
    out.push({ id: P.name + key, pkg: P.name, packageJson: P.packageJson, packageDir: P.dir, owner: items[0]!.g.owner, target: sources[0]?.target ?? viaGlue[0]!.target, key, value: pl.valueRel, glue, names, importers, edges, flags: [...flags, ...[...glueRules].map(r => "glue-violates:" + r)], proposals: props, conflicts: [...new Set(conflicts)], carriedRules: [...carried] });
  }
  writeFileSync(resolve(GENERATED, "candidates.json"), JSON.stringify({ candidates: out, skipped }));
  const by = (f: (c: Candidate) => boolean) => out.filter(f).length;
  console.log(`candidates ${out.length} (edges ${out.reduce((s2, c) => s2 + c.edges, 0)}) skipped ${skipped.length} (edges ${skipped.reduce((s2, c) => s2 + c.edges, 0)})`);
  console.log(`glue create ${by(c => c.glue.mode === "create")} extend ${by(c => c.glue.mode === "extend")} existing ${by(c => c.glue.mode === "existing")} none ${by(c => c.glue.mode === "none")}; flagged ${by(c => c.flags.length > 0)}; conflicts ${by(c => c.conflicts.length > 0)}`);
}
void ROOT;
