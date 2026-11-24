/** 🧬️ Writes a changed JSON document into the text of the document it came from: only the values that differ are rewritten, each in the layout of what it replaces or stands beside (on one line with `, ` and `: `, or one member per line at the indentation found there), so a file keeps its own mixed formatting and its size. Lists of objects with an `id` are matched by id (kept elements keep their order; removed ones are cut out, new ones are set in after the element that precedes them).
 * @see ./apply.ts */
type Node = { start: number; end: number; members?: { key: string; start: number; value: Node }[]; items?: Node[] };
type Edit = { start: number; end: number; text: string };

/** 🌳️ The value spans of a JSON text. */
export function spans(text: string): Node {
  let i = 0;
  const ws = (): void => { while (i < text.length && (text.charCodeAt(i) === 32 || text.charCodeAt(i) === 10 || text.charCodeAt(i) === 13 || text.charCodeAt(i) === 9)) i++; };
  const str = (): string => { const s = i; i++; while (text[i] !== '"') i += text[i] === "\\" ? 2 : 1; i++; return JSON.parse(text.slice(s, i)); };
  const value = (): Node => {
    ws();
    const start = i;
    if (text[i] === "{") {
      i++;
      const members: NonNullable<Node["members"]> = [];
      ws();
      if (text[i] === "}") { i++; return { start, end: i, members }; }
      for (;;) {
        ws();
        const ks = i;
        const key = str();
        ws();
        i++;
        members.push({ key, start: ks, value: value() });
        ws();
        if (text[i++] === "}") return { start, end: i, members };
      }
    }
    if (text[i] === "[") {
      i++;
      const items: Node[] = [];
      ws();
      if (text[i] === "]") { i++; return { start, end: i, items }; }
      for (;;) {
        items.push(value());
        ws();
        if (text[i++] === "]") return { start, end: i, items };
      }
    }
    if (text[i] === '"') { str(); return { start, end: i }; }
    while (i < text.length && !",]} \n\r\t".includes(text[i]!)) i++;
    return { start, end: i };
  };
  return value();
}

const inline = (v: unknown): string => (Array.isArray(v) ? `[${v.map(inline).join(", ")}]` : v !== null && typeof v === "object" ? `{${Object.entries(v).filter(([, x]) => x !== undefined).map(([k, x]) => `${JSON.stringify(k)}: ${inline(x)}`).join(", ")}}` : JSON.stringify(v));
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const sorted = (v: unknown): unknown => (Array.isArray(v) ? v.map(sorted) : v !== null && typeof v === "object" ? Object.fromEntries(Object.entries(v).filter(([, x]) => x !== undefined).sort(([p], [q]) => (p < q ? -1 : p > q ? 1 : 0)).map(([k, x]) => [k, sorted(x)])) : v);

/** ✍️ The text of `next` in the layout of `text`, whose parsed value is `previous`. */
export function rewrite(text: string, previous: unknown, next: unknown): string {
  const edits: Edit[] = [];
  const lineIndent = (at: number): string => { const s = text.lastIndexOf("\n", at - 1) + 1; return /^[ \t]*/u.exec(text.slice(s, at))![0]; };
  const ownLine = (at: number): boolean => text.slice(text.lastIndexOf("\n", at - 1) + 1, at).trim() === "";
  const step = /\n([ \t]+)\S/u.exec(text)?.[1] ?? "  ";
  const format = (v: unknown, like: { multiline: boolean; indent: string }): string => (like.multiline ? JSON.stringify(v, null, step).replace(/\n/gu, `\n${like.indent}`) : inline(v));
  const styleOf = (n: Node): { multiline: boolean; indent: string } => ({ multiline: text.slice(n.start, n.end).includes("\n"), indent: lineIndent(n.start) });
  const cut = (all: { start: number; end: number }[], k: number, close: number): void => { edits.push(k > 0 ? { start: all[k - 1]!.end, end: all[k]!.end, text: "" } : all.length > 1 ? { start: all[0]!.start, end: all[1]!.start, text: "" } : { start: all[0]!.start, end: close, text: "" }); };
  const walk = (n: Node, a: any, b: any): void => {
    if (same(a, b)) return;
    if (n.members !== undefined && b !== null && typeof b === "object" && !Array.isArray(b)) {
      const whole = n.members.map((m) => ({ start: m.start, end: m.value.end }));
      n.members.forEach((m, k) => { if (b[m.key] === undefined) cut(whole, k, n.end - 1); else walk(m.value, a[m.key], b[m.key]); });
      const fresh = Object.keys(b).filter((k) => b[k] !== undefined && !(k in a));
      if (fresh.length > 0) {
        const last = n.members[n.members.length - 1];
        if (last === undefined) { edits.push({ start: n.start, end: n.end, text: inline(b) }); return; }
        const lines = ownLine(last.start);
        const indent = lineIndent(last.start);
        const like = { multiline: lines && fresh.some((k) => typeof b[k] === "object") && n.members.some((m) => text.slice(m.value.start, m.value.end).includes("\n")), indent };
        edits.push({ start: last.value.end, end: last.value.end, text: fresh.map((k) => `${lines ? `,\n${indent}` : ", "}${JSON.stringify(k)}: ${format(b[k], like)}`).join("") });
      }
      return;
    }
    if (n.items !== undefined && Array.isArray(b)) {
      const ids = (l: any[]): boolean => l.length > 0 && l.every((x) => x !== null && typeof x === "object" && typeof x.id === "string");
      if (ids(a) && ids(b) && new Set(a.map((x: any) => x.id)).size === a.length) {
        const kept = new Set(b.map((x: any) => x.id));
        const old = new Map<string, number>(a.map((x: any, k: number) => [x.id, k]));
        const order = b.filter((x: any) => old.has(x.id)).map((x: any) => old.get(x.id)!);
        if (order.every((k: number, i2: number) => i2 === 0 || k > order[i2 - 1]!)) {
          a.forEach((x: any, k: number) => { if (!kept.has(x.id)) cut(n.items!, k, n.end - 1); });
          const first = n.items[0]!;
          const lines = ownLine(first.start);
          const like = styleOf(first);
          let anchor = -1;
          const pending = new Map<number, string[]>();
          for (const x of b) {
            if (old.has(x.id)) { anchor = old.get(x.id)!; walk(n.items[anchor]!, a[anchor], x); continue; }
            const l = pending.get(anchor) ?? [];
            l.push(format(x, like));
            pending.set(anchor, l);
          }
          for (const [k, list] of pending) {
            if (k >= 0) edits.push({ start: n.items[k]!.end, end: n.items[k]!.end, text: list.map((t) => `${lines ? `,\n${like.indent}` : ", "}${t}`).join("") });
            else edits.push({ start: first.start, end: first.start, text: list.map((t) => `${t}${lines ? `,\n${like.indent}` : ", "}`).join("") });
          }
          return;
        }
      }
      if (a.length === b.length) { n.items.forEach((item, k) => walk(item, a[k], b[k])); return; }
    }
    edits.push({ start: n.start, end: n.end, text: format(b, styleOf(n)) });
  };
  walk(spans(text), previous, next);
  let out = text;
  for (const e of edits.sort((p, q) => q.start - p.start || q.end - p.end)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  if (!same(sorted(JSON.parse(out)), sorted(JSON.parse(JSON.stringify(next))))) throw new Error("the rewritten text does not hold the new document");
  return out;
}
