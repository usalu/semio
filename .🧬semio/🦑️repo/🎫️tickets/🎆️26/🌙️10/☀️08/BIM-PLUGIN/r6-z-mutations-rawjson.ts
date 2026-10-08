/**
 * 🧰️ Raw-lexeme JSON for fixture migrations: parses a JSON document keeping every number and string exactly as written and prints it
 * in the `serde_json` pretty style (two spaces, `[]`/`{}` for empty containers, one trailing newline), so a migrated fixture only
 * differs where the migration touched it.
 */
export type Node = { k: "obj"; v: [string, Node][] } | { k: "arr"; v: Node[] } | { k: "raw"; v: string };

export function parse(text: string): Node {
  let i = 0;
  const ws = () => {
    while (i < text.length && /\s/.test(text[i])) i++;
  };
  const value = (): Node => {
    ws();
    const c = text[i];
    if (c === "{") {
      i++;
      const v: [string, Node][] = [];
      ws();
      if (text[i] === "}") {
        i++;
        return { k: "obj", v };
      }
      for (;;) {
        ws();
        const key = string();
        ws();
        i++;
        v.push([key, value()]);
        ws();
        if (text[i++] === "}") return { k: "obj", v };
      }
    }
    if (c === "[") {
      i++;
      const v: Node[] = [];
      ws();
      if (text[i] === "]") {
        i++;
        return { k: "arr", v };
      }
      for (;;) {
        v.push(value());
        ws();
        if (text[i++] === "]") return { k: "arr", v };
      }
    }
    if (c === '"') return { k: "raw", v: JSON.stringify(string()) };
    const start = i;
    while (i < text.length && !/[\s,\]}]/.test(text[i])) i++;
    return { k: "raw", v: text.slice(start, i) };
  };
  const string = (): string => {
    const start = i++;
    while (text[i] !== '"') i += text[i] === "\\" ? 2 : 1;
    i++;
    return JSON.parse(text.slice(start, i));
  };
  return value();
}

export function print(node: Node, depth = 0): string {
  const pad = "  ".repeat(depth + 1);
  const end = "  ".repeat(depth);
  switch (node.k) {
    case "raw":
      return node.v;
    case "arr":
      return node.v.length === 0 ? "[]" : `[\n${node.v.map((item) => pad + print(item, depth + 1)).join(",\n")}\n${end}]`;
    case "obj":
      return node.v.length === 0 ? "{}" : `{\n${node.v.map(([key, item]) => `${pad}${JSON.stringify(key)}: ${print(item, depth + 1)}`).join(",\n")}\n${end}}`;
  }
}

export const object = (...entries: [string, Node][]): Node => ({ k: "obj", v: entries });
export const raw = (v: string): Node => ({ k: "raw", v });
export const string_ = (v: string): Node => ({ k: "raw", v: JSON.stringify(v) });
export const field = (node: Node, key: string): Node | undefined => (node.k === "obj" ? node.v.find(([name]) => name === key)?.[1] : undefined);

export function walk(node: Node, visit: (node: Node) => Node | undefined): Node {
  const replaced = visit(node);
  if (replaced) return replaced;
  if (node.k === "obj") return { k: "obj", v: node.v.map(([key, item]) => [key, walk(item, visit)]) };
  if (node.k === "arr") return { k: "arr", v: node.v.map((item) => walk(item, visit)) };
  return node;
}
