import fs from "node:fs";
const tax = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const paths = process.argv.slice(3);
const fold = (s) => s.normalize("NFC").replaceAll("️", "").replaceAll("︎", "");
const seg = new Intl.Segmenter("und", { granularity: "grapheme" });
const isEmoji = (g) => /\p{Extended_Pictographic}|\p{Regional_Indicator}|[\u{1F3FB}-\u{1F3FF}]|^[0-9#*]️?⃣/u.test(g);
function split(v) { let sequence = "", first = ""; for (const { segment } of seg.segment(v.normalize("NFC"))) { if (!isEmoji(segment)) break; if (!first) first = segment; sequence += segment; } return { first, rest: v.normalize("NFC").slice(sequence.length) }; }
const arr = (x) => (Array.isArray(x) ? x : Object.entries(x).map(([id, v]) => ({ id, ...v })));
const kinds = arr(tax.semanticDirectoryKinds).map((s) => ({ ...s, re: new RegExp(`^(?:${s.slugPattern})$`, "u") }));
const byEmoji = new Map(), byId = new Map();
for (const k of kinds) { const e = fold(k.emoji); (byEmoji.get(e) ?? byEmoji.set(e, []).get(e)).push(k); const i = k.id.toLocaleLowerCase("und"); (byId.get(i) ?? byId.set(i, []).get(i)).push(k); }
const members = new Map();
for (const s of arr(tax.semanticDirectoryMemberKinds)) for (const n of new Set(s.memberNames.map(fold))) (members.get(n) ?? members.set(n, []).get(n)).push(s);
function match(name, parent, anc) {
  const id = split(name);
  const ctxAllows = (k) => (k.parentKindIds?.length ?? 0) === 0 || (parent !== undefined && k.parentKindIds.includes(parent));
  const global = (byEmoji.get(fold(id.first)) ?? []).filter((k) => (id.rest.length === 0 && k.allowEmojiOnly) || k.re.test(id.rest));
  const named = byId.get(id.rest.toLocaleLowerCase("und")) ?? [];
  const exact = global.filter((k) => ctxAllows(k) && named.includes(k));
  if (exact.length === 1) return { kind: exact[0].id, via: "exact" };
  if (exact.length > 1) return { kind: null, amb: exact.map((e) => e.id) };
  const contextual = parent === undefined ? [] : global.filter((k) => k.parentKindIds?.includes(parent));
  const ordinary = contextual.length > 0 ? contextual : global.filter((k) => (k.parentKindIds?.length ?? 0) === 0);
  if (ordinary.length === 1) return { kind: ordinary[0].id, via: "ordinary" };
  const ctxs = [parent, ...anc].filter((k, i, r) => Boolean(k) && r.indexOf(k) === i);
  const ov = (members.get(fold(`${id.first}${id.rest}`)) ?? []).map((s) => ({ id: s.id, d: ctxs.findIndex((c) => s.ownerKindIds.includes(c)) })).filter((e) => e.d >= 0).sort((a, b) => a.d - b.d || a.id.localeCompare(b.id));
  if (ov.length) { const near = ov.filter((e) => e.d === ov[0].d); if (near.length === 1) return { kind: near[0].id, via: "member" }; return { kind: null, amb: near.map((e) => e.id) }; }
  return { kind: null, amb: ordinary.map((e) => e.id), unresolved: true };
}

function walk(p) {
  const segs = p.split("/");
  let parent, anc = [];
  const out = [];
  for (const s of segs) {
    const r = match(s, parent, anc);
    out.push(`${s} => ${r.kind ?? (r.unresolved ? "UNRESOLVED" : "AMB:" + (r.amb ?? []).join("|"))}`);
    if (r.kind) { anc = [r.kind, ...anc.filter((k) => k !== r.kind)]; parent = r.kind; }
    else { parent = undefined; }
  }
  return out;
}
for (const p of paths) { console.log("== " + p); for (const l of walk(p)) console.log("   " + l); }
