/** 🩹️ Writes a changed plant into the text of its catalog file without touching the bytes of what did not change: the drawn shapes (`representations.*.geometry.*`, by id), the solids (`volumes.*.body|thermal|structure`, by id) and the declared `dimensions` are compared with the text, and only removed, replaced and inserted items are edited (an inserted or replaced item is written with two spaces per level at the indentation of its place). The result must parse to the changed plant, else it throws.
 * @see ./lib.ts */

const WS = new Set([" ", "\n", "\r", "\t"]);
const ws = (t: string, i: number): number => { while (WS.has(t[i]!)) i++; return i; };
const str = (t: string, i: number): number => { i++; while (t[i] !== "\"") i += t[i] === "\\" ? 2 : 1; return i + 1; };
/** The end of the JSON value that starts at `i`. */
const skip = (t: string, i: number): number => {
  const c = t[i]!;
  if (c === "\"") return str(t, i);
  if (c === "{" || c === "[") {
    let depth = 0;
    for (;;) {
      const d = t[i]!;
      if (d === "\"") { i = str(t, i); continue; }
      if (d === "{" || d === "[") depth++;
      else if (d === "}" || d === "]") { depth--; if (depth === 0) return i + 1; }
      i++;
    }
  }
  while (i < t.length && !WS.has(t[i]!) && t[i] !== "," && t[i] !== "}" && t[i] !== "]") i++;
  return i;
};
type Span = { s: number; e: number };
const pairs = (t: string, at: number): Map<string, Span> => {
  const out = new Map<string, Span>();
  let i = ws(t, at + 1);
  while (t[i] !== "}") {
    const ke = str(t, i), key = JSON.parse(t.slice(i, ke)) as string;
    i = ws(t, ws(t, ke) + 1);
    const e = skip(t, i);
    out.set(key, { s: i, e });
    i = ws(t, e);
    if (t[i] === ",") i = ws(t, i + 1);
  }
  return out;
};
const items = (t: string, at: number): Span[] => {
  const out: Span[] = [];
  let i = ws(t, at + 1);
  while (t[i] !== "]") {
    const e = skip(t, i);
    out.push({ s: i, e });
    i = ws(t, e);
    if (t[i] === ",") i = ws(t, i + 1);
  }
  return out;
};
const indentAt = (t: string, s: number): string => { const i = t.lastIndexOf("\n", s - 1) + 1; const lead = t.slice(i, s); return /^\s*$/u.test(lead) ? lead : (/^\s*/u.exec(t.slice(i))?.[0] ?? ""); };
const pretty = (value: unknown, indent: string): string => JSON.stringify(value, null, 2).replace(/\n/gu, `\n${indent}`);

/** 🩹️ The text of the file with the lists and the dimensions of `after` (the changed document) written into it. */
export function patched(text: string, after: any): string {
  const before = JSON.parse(text);
  const edits: { s: number; e: number; x: string }[] = [];
  const root = pairs(text, ws(text, 0));
  const asset = pairs(text, root.get("asset")!.s);
  const list = (span: Span, was: any[], now: any[]): void => {
    if (JSON.stringify(was) === JSON.stringify(now)) return;
    const spans = items(text, span.s);
    const index = new Map<string, number>(was.map((x, k) => [x.id as string, k]));
    if (index.size !== was.length) throw new Error("items without a unique id");
    const kept = now.filter((x) => index.has(x.id));
    if (kept.some((x, k) => k > 0 && index.get(x.id)! < index.get(kept[k - 1].id)!)) throw new Error("the order of kept items changed");
    const nowIds = new Set(now.map((x) => x.id));
    const survivors = was.filter((x) => nowIds.has(x.id));
    const base = spans.length > 0 ? indentAt(text, spans[0]!.s) : `${indentAt(text, span.s)}  `;
    was.forEach((x, k) => {
      if (nowIds.has(x.id)) return;
      const sp = spans[k]!;
      if (survivors.length === 0) return;
      const later = was.slice(k + 1).some((y) => nowIds.has(y.id));
      if (later) edits.push({ s: sp.s, e: spans[k + 1]!.s, x: "" });
      else { let p = k - 1; while (!nowIds.has(was[p].id)) p--; const from = Math.max(spans[p]!.e, ...edits.filter((d) => d.x === "" && d.e > spans[p]!.e && d.e <= sp.s).map((d) => d.e)); edits.push({ s: from, e: sp.e, x: "" }); }
    });
    if (survivors.length === 0 && was.length > 0) { edits.push({ s: span.s, e: span.e, x: now.length === 0 ? "[]" : `[\n${base}${now.map((x) => pretty(x, base)).join(`,\n${base}`)}\n${indentAt(text, span.s)}]` }); return; }
    let pending: any[] = [];
    let previous: number | undefined;
    const flush = (): void => {
      if (pending.length === 0) return;
      const body = pending.map((x) => pretty(x, base)).join(`,\n${base}`);
      if (previous !== undefined) edits.push({ s: spans[previous]!.e, e: spans[previous]!.e, x: `,\n${base}${body}` });
      else if (spans.length > 0) { const first = index.get(survivors[0].id)!; edits.push({ s: spans[first]!.s, e: spans[first]!.s, x: `${body},\n${base}` }); }
      else edits.push({ s: span.s, e: span.e, x: `[\n${base}${body}\n${indentAt(text, span.s)}]` });
      pending = [];
    };
    for (const x of now) {
      const k = index.get(x.id);
      if (k === undefined) { pending.push(x); continue; }
      flush();
      previous = k;
      if (JSON.stringify(was[k]) !== JSON.stringify(x)) edits.push({ s: spans[k]!.s, e: spans[k]!.e, x: pretty(x, indentAt(text, spans[k]!.s)) });
    }
    flush();
  };
  const reps = pairs(text, asset.get("representations")!.s);
  for (const [sc, span] of reps) {
    const g = pairs(text, span.s).get("geometry");
    if (g === undefined) continue;
    for (const [view, vspan] of pairs(text, g.s)) list(vspan, before.asset.representations[sc].geometry[view], after.asset.representations[sc].geometry[view]);
  }
  const vols = pairs(text, asset.get("volumes")!.s);
  for (const [sc, span] of vols) for (const [name, lspan] of pairs(text, span.s)) if (["body", "thermal", "structure"].includes(name) && Array.isArray(before.asset.volumes[sc][name])) list(lspan, before.asset.volumes[sc][name], after.asset.volumes[sc][name]);
  for (const [key, span] of pairs(text, asset.get("dimensions")!.s)) if (before.asset.dimensions[key] !== after.asset.dimensions[key]) edits.push({ s: span.s, e: span.e, x: JSON.stringify(after.asset.dimensions[key]) });
  edits.sort((a, b) => b.s - a.s || b.e - a.e);
  let out = text;
  for (const d of edits) out = out.slice(0, d.s) + d.x + out.slice(d.e);
  if (JSON.stringify(JSON.parse(out)) !== JSON.stringify(after)) throw new Error("the patched text does not parse to the changed plant");
  return out;
}
