// 🩹 Format-preserving JSON property editor: replaces or inserts task properties by byte offset so unrelated lines of a shared manifest never change.
export type Span = { start: number; end: number };
export type Prop = { key: string; keyStart: number; value: Span };
export type Node = { span: Span; kind: "object" | "array" | "scalar"; props?: Prop[]; items?: Span[] };

class Scanner {
  i = 0;
  constructor(readonly s: string) {}
  ws() { while (this.i < this.s.length && /\s/.test(this.s[this.i]!)) this.i++; }
  value(): Node {
    this.ws();
    const start = this.i, c = this.s[this.i];
    if (c === "{") return this.object(start);
    if (c === "[") return this.array(start);
    if (c === '"') { this.string(); return { span: { start, end: this.i }, kind: "scalar" }; }
    while (this.i < this.s.length && !/[,\]}\s]/.test(this.s[this.i]!)) this.i++;
    return { span: { start, end: this.i }, kind: "scalar" };
  }
  string(): string {
    const start = this.i++;
    while (this.s[this.i] !== '"') { if (this.s[this.i] === "\\") this.i++; this.i++; }
    this.i++;
    return JSON.parse(this.s.slice(start, this.i));
  }
  object(start: number): Node {
    this.i++;
    const props: Prop[] = [];
    this.ws();
    if (this.s[this.i] === "}") { this.i++; return { span: { start, end: this.i }, kind: "object", props }; }
    for (;;) {
      this.ws();
      const keyStart = this.i, key = this.string();
      this.ws(); this.i++;
      const node = this.value();
      props.push({ key, keyStart, value: node.span });
      (props[props.length - 1] as any).node = node;
      this.ws();
      if (this.s[this.i] === ",") { this.i++; continue; }
      this.i++;
      break;
    }
    return { span: { start, end: this.i }, kind: "object", props };
  }
  array(start: number): Node {
    this.i++;
    const items: Span[] = [];
    this.ws();
    if (this.s[this.i] === "]") { this.i++; return { span: { start, end: this.i }, kind: "array", items }; }
    for (;;) {
      items.push(this.value().span);
      this.ws();
      if (this.s[this.i] === ",") { this.i++; continue; }
      this.i++;
      break;
    }
    return { span: { start, end: this.i }, kind: "array", items };
  }
}

export function parseNode(text: string): Node { return new Scanner(text).value(); }
export function child(node: Node, key: string): (Prop & { node: Node }) | undefined { return node.props?.find(p => p.key === key) as any; }

const lineIndent = (text: string, offset: number) => { const lineStart = text.lastIndexOf("\n", offset - 1) + 1; return /^[ \t]*/.exec(text.slice(lineStart))![0]; };

export function render(value: unknown, indent: string, multiline: boolean): string {
  if (!multiline) return inline(value);
  return JSON.stringify(value, null, 2).split("\n").map((line, index) => index === 0 ? line : indent + line).join("\n");
}
function inline(value: unknown): string {
  if (Array.isArray(value)) return value.length ? `[${value.map(inline).join(", ")}]` : "[]";
  if (value && typeof value === "object") { const entries = Object.entries(value); return entries.length ? `{ ${entries.map(([k, v]) => `${JSON.stringify(k)}: ${inline(v)}`).join(", ")} }` : "{}"; }
  return JSON.stringify(value);
}

export type Edit = { start: number; end: number; text: string };

const rich = (value: unknown) => typeof value === "object" && value !== null && (Array.isArray(value) ? value.length > 0 : Object.keys(value).length > 0);

export function setProperty(text: string, object: Node, key: string, value: unknown): Edit {
  const existing = child(object, key);
  const multiline = text.slice(object.span.start, object.span.end).includes("\n");
  if (existing) {
    const indent = lineIndent(text, existing.keyStart);
    return { start: existing.value.start, end: existing.value.end, text: render(value, indent, multiline && rich(value)) };
  }
  const props = object.props!;
  if (!props.length) return { start: object.span.start, end: object.span.end, text: `{ ${JSON.stringify(key)}: ${inline(value)} }` };
  const last = props[props.length - 1]!;
  if (multiline) {
    const indent = lineIndent(text, props[0]!.keyStart);
    return { start: last.value.end, end: last.value.end, text: `,\n${indent}${JSON.stringify(key)}: ${render(value, indent, rich(value))}` };
  }
  return { start: last.value.end, end: last.value.end, text: `, ${JSON.stringify(key)}: ${inline(value)}` };
}

export function apply(text: string, edits: Edit[]): string {
  let out = text;
  for (const edit of [...edits].sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}
