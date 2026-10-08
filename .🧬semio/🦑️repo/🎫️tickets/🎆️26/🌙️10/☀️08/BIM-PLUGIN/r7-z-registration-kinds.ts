import { readFileSync, writeFileSync } from "node:fs";
import { loadTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts";
import { SEGMENTER, isEmojiGrapheme, emojiFold } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🛣️path/🟦️.ts";

const [, , dirsFile, outFile, mode, taxonomyFile] = process.argv as string[];
const taxonomy = loadTaxonomy({ repoRoot: process.cwd() });
const rows: { path: string; codes: string[] }[] = JSON.parse(readFileSync(dirsFile!, "utf8"));
const memberKinds = (JSON.parse(readFileSync(taxonomyFile!, "utf8")) as { semanticDirectoryMemberKinds: Record<string, { ownerKindIds: string[]; memberNames: string[] }> }).semanticDirectoryMemberKinds;

function split(value: string): { first: string; rest: string } {
  let sequence = "", first = "";
  for (const { segment } of SEGMENTER.segment(value.normalize("NFC"))) {
    if (!isEmojiGrapheme(segment)) break;
    if (!first) first = segment;
    sequence += segment;
  }
  return { first, rest: value.slice(sequence.length) };
}

function resolve(name: string, parent: string | undefined, ancestors: string[]): { kind: string | null; candidates: string[] } {
  const id = split(name.normalize("NFC"));
  const allows = (k: (typeof taxonomy.directoryKinds)[number]): boolean => (k.parentKindIds?.length ?? 0) === 0 || (parent !== undefined && k.parentKindIds?.includes(parent) === true);
  const global = taxonomy.directoryKinds.filter((k) => emojiFold(k.emoji) === emojiFold(id.first) && ((id.rest.length === 0 && k.allowEmojiOnly) || k.slugRegex.test(id.rest)));
  const exact = global.filter((k) => allows(k) && k.id.normalize("NFC").toLocaleLowerCase("und") === id.rest.toLocaleLowerCase("und"));
  if (exact.length === 1) return { kind: exact[0]!.id, candidates: [] };
  const contextual = parent === undefined ? [] : global.filter((k) => k.parentKindIds?.includes(parent) === true);
  const ordinary = contextual.length > 0 ? contextual : global.filter((k) => (k.parentKindIds?.length ?? 0) === 0);
  if (ordinary.length === 1) return { kind: ordinary[0]!.id, candidates: [] };
  const contexts = [parent, ...ancestors].filter((k, i, a): k is string => Boolean(k) && a.indexOf(k) === i);
  const overlays = Object.entries(taxonomy.schema.semanticDirectoryMemberKinds)
    .filter(([, spec]) => spec.memberNames.some((m) => emojiFold(m) === emojiFold(`${id.first}${id.rest}`)))
    .map(([k, spec]) => ({ k, d: contexts.findIndex((c) => spec.ownerKindIds.includes(c)) }))
    .filter((e) => e.d >= 0).sort((a, b) => a.d - b.d || a.k.localeCompare(b.k));
  if (overlays.length > 0 && overlays.filter((e) => e.d === overlays[0]!.d).length === 1) return { kind: overlays[0]!.k, candidates: [] };
  return { kind: null, candidates: ordinary.map((k) => k.id) };
}

const kinds = new Map<string, string>();
const missing: { path: string; name: string; parent: string | undefined; ancestors: string[]; guess: string[] }[] = [];
const sorted = [...rows].sort((a, b) => a.path.split("/").length - b.path.split("/").length);
for (const row of sorted) {
  const parentPath = row.path.split("/").slice(0, -1).join("/");
  const parent = kinds.get(parentPath);
  const ancestors: string[] = [];
  for (let p = parentPath.split("/").slice(0, -1).join("/"); p; p = p.split("/").slice(0, -1).join("/")) { const k = kinds.get(p); if (k) ancestors.push(k); }
  const name = row.path.split("/").at(-1)!;
  const r = resolve(name, parent, ancestors);
  const leaf = /\/🧬️schema\/🧬️mutations\/[^/]+$/u.test(row.path);
  if (leaf) kinds.set(row.path, "members-of-schema");
  else if (r.kind) kinds.set(row.path, r.kind);
  else if (!row.codes.includes("directory-kind-unresolved")) {
    const slug = split(name).rest;
    if (taxonomy.directoryKinds.some((k) => k.id === slug)) kinds.set(row.path, slug);
    else kinds.set(row.path, `?${slug}`);
  } else missing.push({ path: row.path, name, parent, ancestors, guess: r.candidates });
}
const plan = new Map<string, Set<string>>();
const problems: unknown[] = [];
const creates = new Map<string, string>();
for (const m of missing) {
  if (m.parent === undefined) continue;
  const mirror = /\/🧫️fixtures\/🧬️mutations\/[^/]+\/[^/]+$/u.test(m.path);
  const target = mirror ? "members-of-tests" : undefined;
  if (target) { const set = plan.get(target) ?? new Set<string>(); set.add(m.name.normalize("NFC")); plan.set(target, set); continue; }
  const owners = Object.entries(memberKinds).filter(([, s]) => m.parent !== undefined && s.ownerKindIds.includes(m.parent));
  const preferred = owners.find(([k]) => k === `members-of-${m.parent}`) ?? (owners.length === 1 ? owners[0] : undefined);
  if (!preferred) {
    if (m.parent.startsWith("?")) { problems.push({ ...m, owners: [] }); continue; }
    const created = `members-of-${m.parent}`;
    const set = plan.get(created) ?? new Set<string>();
    set.add(m.name.normalize("NFC"));
    plan.set(created, set);
    creates.set(created, m.parent);
    continue;
  }
  const set = plan.get(preferred[0]) ?? new Set<string>();
  set.add(m.name.normalize("NFC"));
  plan.set(preferred[0], set);
}
const result = { missing: missing.length, plan: Object.fromEntries([...plan].map(([k, v]) => [k, [...v]])), problems };
writeFileSync(outFile!, JSON.stringify(result, null, 1));
console.error(`unresolved ${missing.length}; lists ${plan.size}; problems ${problems.length}; mode ${mode}`);
if (mode === "apply") {
  const raw = readFileSync(taxonomyFile!, "utf8");
  const json = JSON.parse(raw);
  for (const [list, names] of plan) {
    if (!json.semanticDirectoryMemberKinds[list]) json.semanticDirectoryMemberKinds[list] = { ownerKindIds: [creates.get(list)], memberNames: [], source: "registry" };
    const members: string[] = json.semanticDirectoryMemberKinds[list].memberNames;
    for (const name of names) if (!members.some((m) => m.normalize("NFC") === name)) members.push(name);
  }
  writeFileSync(taxonomyFile!, JSON.stringify(json, null, 2) + "\n");
  console.error("applied");
}
