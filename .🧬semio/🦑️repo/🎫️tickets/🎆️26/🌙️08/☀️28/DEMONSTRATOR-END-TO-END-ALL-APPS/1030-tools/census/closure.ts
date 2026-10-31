// 🧮️ Read-only runtime component closure of the mit-Bestand demonstrator (wave 1030 census lane).
// Mirrors runtimeComponentClosure (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs)
// over the [package.metadata.semio] manifests. Source of truth: Cargo manifests, no build.
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = process.argv[2];
const files = readFileSync(join(TICKET, "🗑️generated/1030/manifests.txt"), "utf8").split("\n").filter(Boolean);
const panes = JSON.parse(readFileSync(join(ROOT, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json"), "utf8")).panes as { variant: string; runtimeVariant: string }[];
const variants = new Set(panes.flatMap(p => [p.variant, p.runtimeVariant]));

type Row = { id: string; file: string; crate: string; pkg: string; role: string; extendsId?: string; dependsOn: string[]; consumes: string[]; contributes: string[]; host: boolean };
const rows = new Map<string, Row>();
const playgrounds: { variant: string; id: string; file: string }[] = [];

const clean = (s: string) => s.replace(/^[^A-Za-z0-9]+/u, "");
for (const f of files) {
  const t = Bun.TOML.parse(readFileSync(join(ROOT, f), "utf8")) as any;
  const s = t.package?.metadata?.semio ?? {};
  const id = clean(String(s["deployment-directory"] ?? ""));
  for (const p of s.playground ?? []) playgrounds.push({ variant: p.variant, id, file: f });
  if (!s["component-kind"]) continue;
  rows.set(id, {
    id, file: f, crate: dirname(f), pkg: t.package?.name ?? "?", role: s["component-kind"],
    extendsId: s.extends ? String(s.extends) : undefined,
    dependsOn: [...(s.extends ? [String(s.extends)] : []), ...(s["depends-on"] ?? [])],
    consumes: s.consumes ?? [], contributes: s.contributes ?? [], host: s.host === true,
  });
}

const contributors = new Map<string, string[]>();
for (const r of rows.values()) for (const topic of r.contributes) contributors.set(topic, [...(contributors.get(topic) ?? []), r.id]);

function closure(roots: string[]): string[] {
  const selected = new Set<string>();
  const pending = roots.map(id => ({ id, shallow: false, appScoped: false }));
  while (pending.length) {
    const { id, shallow, appScoped } = pending.pop()!;
    if (selected.has(id)) continue;
    const c = rows.get(id);
    if (!c) throw new Error(`Unknown runtime component ${id}`);
    selected.add(id);
    if (shallow) continue;
    for (const dep of c.dependsOn) pending.push({ id: dep, shallow: c.extendsId === dep, appScoped: false });
    for (const topic of c.consumes) for (const k of contributors.get(topic) ?? []) pending.push({ id: k, shallow: false, appScoped: false });
    if (c.host && !appScoped) for (const h of rows.keys()) pending.push({ id: h, shallow: false, appScoped: false });
  }
  return [...selected].sort();
}

const rootsFor = (vs: Iterable<string>) => {
  const out = new Set<string>();
  for (const pg of playgrounds) if (vs && [...vs].includes(pg.variant)) out.add(pg.id);
  return [...out].sort();
};
const variantsFor = (names: string[]) => new Set(names);

const lines: string[] = [];
const emit = (s = "") => lines.push(s);

const allRoots = rootsFor(variants);
emit(`ROOTS (all pane variants+runtimeVariants -> playground pluginIds): ${allRoots.join(", ")}`);
const lanes: Record<string, string[]> = {
  "generator(demonstrator)": ["demonstrator"],
  "generation3d": ["procedural"],
  energy: ["energy"],
  fem3d: ["fem"],
};
for (const [name, ids] of Object.entries(lanes)) emit(`LANE ${name} roots=${ids.join(",")} -> closure ${closure(ids).length}`);
emit(`PLAYGROUND MAP ${playgrounds.filter(p => variants.has(p.variant)).map(p => `${p.variant}=>${p.id}`).join(" ")}`);
emit(`UNRESOLVED playground ids (no component-kind row): ${[...new Set(playgrounds.map(p => p.id))].filter(id => !rows.has(id)).join(", ") || "none"}`);
const union = closure(allRoots);
emit(`UNION CLOSURE size=${union.length}`);
emit("");
emit("id | role | cargo package | crate dir | dependsOn | consumes | contributes | host");
for (const id of union) {
  const r = rows.get(id)!;
  emit(`${id} | ${r.role} | ${r.pkg} | ${r.crate} | ${r.dependsOn.join(",")} | ${r.consumes.join(",")} | ${r.contributes.join(",")} | ${r.host}`);
}
emit("");
emit("ALL COMPONENT ROWS NOT IN UNION (for unowned check):");
for (const id of [...rows.keys()].filter(id => !union.includes(id)).sort()) emit(`${id} | ${rows.get(id)!.pkg} | ${rows.get(id)!.crate}`);
console.log(lines.join("\n"));
