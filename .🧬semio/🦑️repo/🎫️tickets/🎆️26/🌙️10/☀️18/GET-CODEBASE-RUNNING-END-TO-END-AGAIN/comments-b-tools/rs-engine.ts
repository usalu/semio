import Parser from "/Users/ueli/Documents/semio/node_modules/web-tree-sitter/tree-sitter.js";
import { keepText, type Removal } from "./keep.ts";

export type Plan = { removals: Removal[]; kept: number; error?: string };

let parser: any = null;

export async function initRust(wasm: string): Promise<void> {
  await Parser.init();
  const lang = await Parser.Language.load(wasm);
  parser = new Parser();
  parser.setLanguage(lang);
}

function parse(text: string): any | undefined {
  const tree = parser.parse(text);
  if (tree.rootNode.hasError()) {
    tree.delete();
    return undefined;
  }
  return tree;
}

function definition(node: any): boolean {
  let p = node.parent;
  while (p) {
    if (p.type === "source_file") return false;
    if (p.type === "mod_item") p = p.parent;
    else if (p.type === "declaration_list" && p.parent?.type === "mod_item") p = p.parent;
    else return true;
  }
  return false;
}

function trimEnd(text: string, pos: number, end: number): number {
  let e = end;
  while (e > pos && (text[e - 1] === "\n" || text[e - 1] === "\r")) e--;
  return e;
}

/** 🔎 Plans the removal of every non-doc comment inside definitions; module blocks count as top level and SAFETY runs are kept whole. */
export function planRs(text: string): Plan {
  const tree = parse(text);
  if (!tree) return { removals: [], kept: 0, error: "parse" };
  const nodes: any[] = tree.rootNode.descendantsOfType(["line_comment", "block_comment"]);
  const items = nodes.map((n) => ({ pos: n.startIndex as number, end: trimEnd(text, n.startIndex, n.endIndex), def: definition(n) }));
  tree.delete();
  items.sort((a, b) => a.pos - b.pos);
  const safetyRun = new Set<number>();
  for (let i = 0; i < items.length; i++) {
    const raw = text.slice(items[i].pos, items[i].end);
    if (!/^\/\/\s*SAFETY\b/.test(raw)) continue;
    safetyRun.add(i);
    for (let j = i + 1; j < items.length; j++) {
      const gap = text.slice(items[j - 1].end, items[j].pos);
      if (!/^\s*\n\s*$/.test(gap) || gap.split("\n").length > 2) break;
      if (!text.slice(items[j].pos, items[j].end).startsWith("//")) break;
      safetyRun.add(j);
    }
  }
  let kept = 0;
  const removals: Removal[] = [];
  items.forEach((it, i) => {
    if (!it.def) return;
    if (safetyRun.has(i) || keepText(text.slice(it.pos, it.end))) {
      kept++;
      return;
    }
    removals.push({ pos: it.pos, end: it.end });
  });
  return { removals, kept };
}

/** 🧾 Comment-free token signature (doc comments included) used to prove that only comments changed. */
export function rsSignature(text: string): string[] | undefined {
  const tree = parse(text);
  if (!tree) return undefined;
  const out: string[] = [];
  const walk = (n: any) => {
    if (n.type === "line_comment" || n.type === "block_comment") {
      const raw = n.text as string;
      if (/^(\/\/\/(?!\/)|\/\/!|\/\*[*!](?![*\/]))/.test(raw) || raw.includes("🔖") || /^\/\/\s*SAFETY/.test(raw)) out.push(`D:${raw.trimEnd()}`);
      return;
    }
    if (n.childCount === 0) {
      out.push(`${n.type}:${n.text}`);
      return;
    }
    out.push(`N${n.type}`);
    for (let i = 0; i < n.childCount; i++) walk(n.child(i));
  };
  walk(tree.rootNode);
  tree.delete();
  return out;
}
