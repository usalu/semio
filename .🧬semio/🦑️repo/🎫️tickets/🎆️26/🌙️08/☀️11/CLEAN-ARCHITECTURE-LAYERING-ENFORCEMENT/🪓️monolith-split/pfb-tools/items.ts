// 🧭️ Lists the top-level items of a Rust file with 1-based line ranges, and the members of inherent impls.
import {readFileSync} from "node:fs";
import {rustTokenPairs, rustTokens} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";

export type Span = {kind: string; name: string | null; vis: string; from: number; to: number; members?: Span[]};

export function listItems(source: string): Span[] {
  const tree = parseRustSourceTree(source);
  const lineOf = (offset: number) => source.slice(0, offset).split("\n").length;
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const out: Span[] = [];
  for (const item of [...tree.items, ...tree.modules.map(m => ({kind: "mod", name: m.name, visibility: m.visibility, start: m.start, declStart: m.declStart, end: m.end}))].sort((a, b) => a.start - b.start)) {
    const span: Span = {kind: item.kind, name: item.name, vis: item.visibility, from: lineOf(item.start), to: lineOf(item.end - 1)};
    if (item.kind === "impl") {
      const first = tokens.findIndex(t => t.start >= item.declStart);
      let open = first;
      while (open < tokens.length && tokens[open]!.text !== "{") { open += 1; }
      const close = pairs.get(open);
      const members: Span[] = [];
      if (close !== undefined) {
        for (let k = open + 1; k < close; k += 1) {
          const startToken = k;
          let m = k;
          while (tokens[m]!.text === "#" && tokens[m + 1]?.text === "[") m = (pairs.get(m + 1) ?? m + 1) + 1;
          let head = m;
          while (["pub", "async", "unsafe", "const", "extern", "default"].includes(tokens[head]!.text) && tokens[head + 1]?.text !== "(") head += 1;
          if (tokens[head]!.text === "pub" && tokens[head + 1]?.text === "(") head = (pairs.get(head + 1) ?? head + 1) + 1;
          while (["async", "unsafe", "const", "extern", "default"].includes(tokens[head]!.text) && tokens[head + 1]?.text !== "(" ) head += 1;
          const name = tokens[head + 1]?.text ?? null;
          let end = head;
          while (end < close && !(tokens[end]!.text === ";" || tokens[end]!.text === "{")) { const p = pairs.get(end); end = p !== undefined && p > end && ["(", "[", "<"].includes(tokens[end]!.text) && tokens[end]!.text !== "<" ? p + 1 : end + 1; }
          if (tokens[end]?.text === "{") end = pairs.get(end) ?? end;
          const text = source.slice(tokens[startToken]!.start, tokens[end]!.end);
          void text;
          // include doc comments directly above: extend start to the previous line start that holds comments
          let startOffset = tokens[startToken]!.start;
          const lines = source.slice(0, startOffset).split("\n");
          let ln = lines.length;
          while (ln > 1 && /^\s*(\/\/\/|#\[)/u.test(source.split("\n")[ln - 2]!)) ln -= 1;
          members.push({kind: tokens[head]!.text, name, vis: "", from: ln, to: lineOf(tokens[end]!.end - 1)});
          k = end;
        }
      }
      span.members = members;
    }
    out.push(span);
  }
  return out;
}

if (import.meta.main) {
  const file = process.argv[2]!;
  const source = readFileSync(file, "utf8");
  for (const span of listItems(source)) {
    console.log(`${span.from}-${span.to} ${span.kind} ${span.name ?? ""} [${span.vis}]`);
    for (const m of span.members ?? []) console.log(`    ${m.from}-${m.to} ${m.kind} ${m.name}`);
  }
}
