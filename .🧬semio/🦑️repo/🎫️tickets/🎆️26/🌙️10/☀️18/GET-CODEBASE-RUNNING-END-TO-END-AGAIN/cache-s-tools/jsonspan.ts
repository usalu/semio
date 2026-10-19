/** 🔬 Minimal JSON scanner that keeps source offsets so manifests are edited in place and keep their formatting. */
export type Node = { kind: "object" | "array" | "string" | "number" | "literal"; start: number; end: number; props?: Prop[]; items?: Node[] };
export type Prop = { key: string; keyStart: number; keyEnd: number; value: Node };

export function scan(text: string): Node {
  let i = 0;
  const ws = () => { while (i < text.length && /\s/u.test(text[i]!)) i++; };
  const str = (): Node => { const start = i; i++; while (text[i] !== '"') { if (text[i] === "\\") i++; i++; } i++; return { kind: "string", start, end: i }; };
  const value = (): Node => {
    ws();
    const c = text[i]!;
    if (c === "{") {
      const start = i; i++; const props: Prop[] = []; ws();
      while (text[i] !== "}") {
        ws(); const k = str(); ws(); i++; const v = value(); props.push({ key: JSON.parse(text.slice(k.start, k.end)), keyStart: k.start, keyEnd: k.end, value: v }); ws();
        if (text[i] === ",") i++; ws();
      }
      i++; return { kind: "object", start, end: i, props };
    }
    if (c === "[") {
      const start = i; i++; const items: Node[] = []; ws();
      while (text[i] !== "]") { items.push(value()); ws(); if (text[i] === ",") i++; ws(); }
      i++; return { kind: "array", start, end: i, items };
    }
    if (c === '"') return str();
    const start = i; while (i < text.length && !/[\s,\]}]/u.test(text[i]!)) i++;
    return { kind: /^-?\d/u.test(text.slice(start, i)) ? "number" : "literal", start, end: i };
  };
  const root = value(); ws();
  if (i !== text.length) throw Error("trailing JSON content");
  return root;
}

export const prop = (node: Node, key: string): Prop | undefined => node.props?.find((p) => p.key === key);

export type Edit = { start: number; end: number; text: string };

/** 🧩 Replaces present properties of an object node and inserts the missing ones as one block after the first present anchor, copying the object's own indentation style. */
export function setProps(text: string, node: Node, entries: readonly (readonly [string, unknown])[], after: readonly string[]): Edit[] {
  const edits: Edit[] = [];
  const first = node.props![0];
  const multiline = first ? text.slice(node.start, first.keyStart).includes("\n") : false;
  const lineStart = first ? text.lastIndexOf("\n", first.keyStart) + 1 : 0;
  const indent = first && multiline ? /^\s*/u.exec(text.slice(lineStart, first.keyStart))![0] : "";
  const rendered = (json: unknown): string => Array.isArray(json) && json.length ? "[\n" + json.map((item) => `${indent}  ${JSON.stringify(item)}`).join(",\n") + `\n${indent}]` : JSON.stringify(json);
  const missing: (readonly [string, unknown])[] = [];
  for (const [key, json] of entries) {
    const existing = prop(node, key);
    if (existing) edits.push({ start: existing.value.start, end: existing.value.end, text: multiline ? rendered(json) : JSON.stringify(json) });
    else missing.push([key, json]);
  }
  if (!missing.length) return edits;
  const anchor = after.map((name) => prop(node, name)).find(Boolean);
  const block = missing.map(([key, json]) => (multiline ? `\n${indent}${JSON.stringify(key)}: ${rendered(json)}` : `${JSON.stringify(key)}:${JSON.stringify(json)}`)).join(",");
  if (anchor) edits.push({ start: anchor.value.end, end: anchor.value.end, text: "," + block });
  else edits.push({ start: node.start + 1, end: node.start + 1, text: block + "," });
  return edits;
}

export function apply(text: string, edits: Edit[]): string {
  for (const e of [...edits].sort((a, b) => b.start - a.start || b.end - a.end)) text = text.slice(0, e.start) + e.text + text.slice(e.end);
  return text;
}

/** 🧱 Adds `values` to `namedInputs[group]`, creating the object or the group when absent and keeping the manifest's indentation. */
export function mergeNamedInput(text: string, root: Node, group: string, values: readonly string[]): Edit[] {
  const named = prop(root, "namedInputs");
  const targets = prop(root, "targets");
  const lineStart = (offset: number) => text.lastIndexOf("\n", offset) + 1;
  const indentOf = (offset: number) => /^\s*/u.exec(text.slice(lineStart(offset), offset))![0];
  const multiline = (node: Node, firstOffset: number) => text.slice(node.start, firstOffset).includes("\n");
  const list = (items: readonly string[], indent: string) => "[\n" + items.map((item) => `${indent}  ${JSON.stringify(item)}`).join(",\n") + `\n${indent}]`;
  if (named) {
    const existing = prop(named.value, group);
    if (existing) {
      const present = (existing.value.items ?? []).map((item) => (item.kind === "string" ? (JSON.parse(text.slice(item.start, item.end)) as string) : undefined));
      const add = values.filter((value) => !present.includes(value));
      if (!add.length) return [];
      const all = (existing.value.items ?? []).map((item) => text.slice(item.start, item.end)).concat(add.map((value) => JSON.stringify(value)));
      const indent = indentOf(existing.keyStart);
      const body = multiline(existing.value, existing.value.items?.[0]?.start ?? existing.value.end) || !existing.value.items?.length ? "[\n" + all.map((item) => `${indent}  ${item}`).join(",\n") + `\n${indent}]` : "[" + all.join(", ") + "]";
      return [{ start: existing.value.start, end: existing.value.end, text: body }];
    }
    return setProps(text, named.value, [[group, values]], [named.value.props!.at(-1)!.key]);
  }
  if (!targets) throw Error("manifest has no targets");
  const indent = indentOf(targets.keyStart);
  const isMulti = text.slice(root.start, targets.keyStart).includes("\n");
  const block = isMulti ? `"namedInputs": {\n${indent}  ${JSON.stringify(group)}: ${list(values, indent + "  ")}\n${indent}},\n${indent}` : `"namedInputs":{${JSON.stringify(group)}:${JSON.stringify(values)}},`;
  return [{ start: targets.keyStart, end: targets.keyStart, text: block }];
}
