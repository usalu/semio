export type Node =
  | { type: "object"; start: number; end: number; props: Prop[] }
  | { type: "array"; start: number; end: number; items: Node[] }
  | { type: "string" | "number" | "literal"; start: number; end: number; value: unknown };
export type Prop = { key: string; keyStart: number; keyEnd: number; value: Node };

export function parse(text: string): Node {
  let i = 0;
  const ws = () => { while (i < text.length && /\s/.test(text[i]!)) i++; };
  const value = (): Node => {
    ws();
    const start = i;
    const c = text[i];
    if (c === "{") {
      i++;
      const props: Prop[] = [];
      ws();
      if (text[i] === "}") { i++; return { type: "object", start, end: i, props }; }
      for (;;) {
        ws();
        const keyStart = i;
        const key = str();
        const keyEnd = i;
        ws();
        if (text[i] !== ":") throw Error(`expected : at ${i}`);
        i++;
        const v = value();
        props.push({ key, keyStart, keyEnd, value: v });
        ws();
        if (text[i] === ",") { i++; continue; }
        if (text[i] === "}") { i++; break; }
        throw Error(`expected , or } at ${i}`);
      }
      return { type: "object", start, end: i, props };
    }
    if (c === "[") {
      i++;
      const items: Node[] = [];
      ws();
      if (text[i] === "]") { i++; return { type: "array", start, end: i, items }; }
      for (;;) {
        items.push(value());
        ws();
        if (text[i] === ",") { i++; continue; }
        if (text[i] === "]") { i++; break; }
        throw Error(`expected , or ] at ${i}`);
      }
      return { type: "array", start, end: i, items };
    }
    if (c === '"') { const v = str(); return { type: "string", start, end: i, value: v }; }
    const m = /^-?\d+(\.\d+)?([eE][+-]?\d+)?/.exec(text.slice(i));
    if (m) { i += m[0].length; return { type: "number", start, end: i, value: Number(m[0]) }; }
    for (const lit of ["true", "false", "null"]) if (text.startsWith(lit, i)) { i += lit.length; return { type: "literal", start, end: i, value: lit === "true" ? true : lit === "false" ? false : null }; }
    throw Error(`unexpected token at ${i}`);
  };
  const str = (): string => {
    const s = i;
    if (text[i] !== '"') throw Error(`expected string at ${i}`);
    i++;
    while (text[i] !== '"') { if (text[i] === "\\") i++; i++; }
    i++;
    return JSON.parse(text.slice(s, i));
  };
  const root = value();
  ws();
  if (i !== text.length) throw Error("trailing content");
  return root;
}

export const prop = (node: Node, key: string): Prop | undefined => node.type === "object" ? node.props.find((p) => p.key === key) : undefined;

export type Edit = { start: number; end: number; text: string };

const lineStart = (text: string, pos: number) => text.lastIndexOf("\n", pos - 1) + 1;
const indentOf = (text: string, pos: number) => { const ls = lineStart(text, pos); return /^[ \t]*/.exec(text.slice(ls, pos))![0]; };
export const isMultiline = (text: string, node: Node) => text.slice(node.start, node.end).includes("\n");

const scalar = (v: unknown) => v === null || typeof v !== "object";
export function render(value: unknown, indent: string, multiline: boolean): string {
  if (!multiline || scalar(value)) return inline(value);
  if (Array.isArray(value)) return value.length ? "[\n" + value.map((v) => indent + "  " + render(v, indent + "  ", true)).join(",\n") + "\n" + indent + "]" : "[]";
  const entries = Object.entries(value as object);
  if (!entries.length || entries.every(([, v]) => scalar(v))) return inline(value);
  return "{\n" + entries.map(([k, v]) => indent + "  " + JSON.stringify(k) + ": " + render(v, indent + "  ", true)).join(",\n") + "\n" + indent + "}";
}

export function inline(value: unknown): string {
  if (Array.isArray(value)) return "[" + value.map(inline).join(", ") + "]";
  if (value && typeof value === "object") return "{ " + Object.entries(value).map(([k, v]) => JSON.stringify(k) + ": " + inline(v)).join(", ") + " }";
  return JSON.stringify(value);
}

export function setProps(text: string, obj: Node, entries: [string, unknown][], after: string[] = []): Edit[] {
  if (obj.type !== "object") throw Error("not an object");
  const multi = isMultiline(text, obj);
  const edits: Edit[] = [];
  const fresh: [string, unknown][] = [];
  for (const [key, value] of entries) {
    const existing = prop(obj, key);
    if (!existing) { fresh.push([key, value]); continue; }
    const nonScalar = typeof value === "object" && value !== null;
    const scalarBefore = existing.value.type !== "array" && existing.value.type !== "object";
    const useMulti = nonScalar && (scalarBefore ? multi : isMultiline(text, existing.value) || (multi && Array.isArray(value) && value.length > 3));
    edits.push({ start: existing.value.start, end: existing.value.end, text: render(value, indentOf(text, existing.keyStart), useMulti) });
  }
  if (!fresh.length) return edits;
  const entry = (key: string, value: unknown, indent: string) => `${JSON.stringify(key)}: ${render(value, indent, multi && typeof value === "object" && value !== null)}`;
  if (!obj.props.length) {
    const base = indentOf(text, obj.start);
    const inner = base + "  ";
    edits.push({ start: obj.start + 1, end: obj.end - 1, text: multi ? "\n" + fresh.map(([k, v]) => inner + entry(k, v, inner)).join(",\n") + "\n" + base : " " + fresh.map(([k, v]) => entry(k, v, "")).join(", ") + " " });
    return edits;
  }
  const anchor = after.map((k) => prop(obj, k)).find(Boolean) ?? obj.props[obj.props.length - 1]!;
  const at = anchor.value.end;
  if (multi) {
    const indent = indentOf(text, obj.props[0]!.keyStart);
    edits.push({ start: at, end: at, text: fresh.map(([k, v]) => `,\n${indent}${entry(k, v, indent)}`).join("") });
  } else edits.push({ start: at, end: at, text: fresh.map(([k, v]) => `, ${entry(k, v, "")}`).join("") });
  return edits;
}

export function removeProp(text: string, obj: Node, key: string): Edit | undefined {
  if (obj.type !== "object") return undefined;
  const idx = obj.props.findIndex((p) => p.key === key);
  if (idx < 0) return undefined;
  const p = obj.props[idx]!;
  if (idx > 0) return { start: obj.props[idx - 1]!.value.end, end: p.value.end, text: "" };
  if (obj.props.length > 1) return { start: p.keyStart, end: obj.props[1]!.keyStart, text: "" };
  return { start: obj.start + 1, end: obj.end - 1, text: "" };
}

export function apply(text: string, edits: Edit[]): string {
  const sorted = [...edits].sort((a, b) => b.start - a.start || b.end - a.end);
  for (let k = 1; k < sorted.length; k++) if (sorted[k]!.end > sorted[k - 1]!.start) throw Error("overlapping edits");
  let out = text;
  for (const e of sorted) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  return out;
}
