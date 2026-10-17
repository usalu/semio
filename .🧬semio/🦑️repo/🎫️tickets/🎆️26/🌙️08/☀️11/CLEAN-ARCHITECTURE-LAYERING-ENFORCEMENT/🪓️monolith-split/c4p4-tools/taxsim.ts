import { readFileSync, writeFileSync } from "node:fs";
const [, , taxPath, dirsPath, realPath, outPath] = process.argv;
const tax = JSON.parse(readFileSync(taxPath!, "utf8"));
const fold = (s: string) => s.normalize("NFC").replaceAll("️", "").replaceAll("︎", "");
const seg = new Intl.Segmenter("und", { granularity: "grapheme" });
const isEmoji = (g: string) => /\p{Extended_Pictographic}|\p{Regional_Indicator}|[\u{1F3FB}-\u{1F3FF}]|^[0-9#*]️?⃣/u.test(g);
function split(v: string) { let sequence = "", first = ""; for (const { segment } of seg.segment(v.normalize("NFC"))) { if (!isEmoji(segment)) break; if (!first) first = segment; sequence += segment; } return { first, rest: v.normalize("NFC").slice(sequence.length) }; }
const kinds = Object.entries<any>(tax.semanticDirectoryKinds).map(([id, s]) => ({ id, ...s, re: new RegExp(`^(?:${s.slugPattern})$`, "u") }));
const byEmoji = new Map<string, any[]>(), byId = new Map<string, any[]>();
for (const k of kinds) { const e = fold(k.emoji); (byEmoji.get(e) ?? byEmoji.set(e, []).get(e)!).push(k); const i = k.id.toLocaleLowerCase("und"); (byId.get(i) ?? byId.set(i, []).get(i)!).push(k); }
const members = new Map<string, [string, any][]>();
for (const [id, s] of Object.entries<any>(tax.semanticDirectoryMemberKinds)) for (const n of new Set(s.memberNames.map(fold))) (members.get(n as string) ?? members.set(n as string, []).get(n as string)!).push([id, s]);
function match(name: string, parent: string | undefined, anc: string[]) {
  const id = split(name);
  const ctxAllows = (k: any) => (k.parentKindIds?.length ?? 0) === 0 || (parent !== undefined && k.parentKindIds.includes(parent));
  if (id.first) {
    const global = (byEmoji.get(fold(id.first)) ?? []).filter((k) => (id.rest.length === 0 && k.allowEmojiOnly) || k.re.test(id.rest));
    const named = byId.get(id.rest.toLocaleLowerCase("und")) ?? [];
    const exact = global.filter((k) => ctxAllows(k) && named.includes(k));
    if (exact.length === 1) return { kind: exact[0].id };
    if (exact.length > 1) return { kind: null, amb: exact.map((e) => e.id) };
    const contextual = parent === undefined ? [] : global.filter((k) => k.parentKindIds?.includes(parent));
    const ordinary = contextual.length > 0 ? contextual : global.filter((k) => (k.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1) return { kind: ordinary[0].id };
    const ctxs = [parent, ...anc].filter((k, i, r): k is string => Boolean(k) && r.indexOf(k) === i);
    const ov = (members.get(fold(`${id.first}${id.rest}`)) ?? []).map(([mid, s]) => ({ id: mid, d: ctxs.findIndex((c) => s.ownerKindIds.includes(c)) })).filter((e) => e.d >= 0).sort((a, b) => a.d - b.d || a.id.localeCompare(b.id));
    if (ov.length) { const near = ov.filter((e) => e.d === ov[0].d); if (near.length === 1) return { kind: near[0].id }; return { kind: null, amb: near.map((e) => e.id) }; }
    return { kind: null, amb: ordinary.map((e) => e.id) };
  }
  return { kind: null, amb: [] as string[] };
}
const dirs = readFileSync(dirsPath!, "utf8").split("\n").filter(Boolean).sort((a, b) => a.split("/").length - b.split("/").length || (a < b ? -1 : 1));
const kindOf = new Map<string, string>();
const unres: string[] = [], ambig: string[] = [];
const chain = (p: string) => { const rows: string[] = []; let c = p.includes("/") ? p.slice(0, p.lastIndexOf("/")) : ""; while (c) { const k = kindOf.get(c); if (k) rows.push(k); c = c.includes("/") ? c.slice(0, c.lastIndexOf("/")) : ""; } return rows; };
const info: Record<string, { parent?: string; anc: string[] }> = {};
for (const p of dirs) {
  const name = p.slice(p.lastIndexOf("/") + 1);
  const parent = kindOf.get(p.includes("/") ? p.slice(0, p.lastIndexOf("/")) : "");
  const anc = chain(p);
  const r: any = match(name, parent, anc);
  if (r.kind) kindOf.set(p, r.kind); else { (r.amb?.length > 1 ? ambig : unres).push(p); info[p] = { parent, anc }; }
}
const real = new Set<string>(JSON.parse(readFileSync(realPath!, "utf8")));
const simU = new Set(unres);
const both = [...real].filter((p) => simU.has(p)).length;
console.log(`dirs=${dirs.length} simUnresolved=${unres.length} simAmbiguous=${ambig.length} real=${real.size} overlap=${both} realNotSim=${[...real].filter((p) => !simU.has(p) && !ambig.includes(p)).length} simNotReal=${unres.filter((p) => !real.has(p)).length}`);
writeFileSync(outPath!, JSON.stringify({ unres, ambig, info, kinds: Object.fromEntries(kindOf) }));
