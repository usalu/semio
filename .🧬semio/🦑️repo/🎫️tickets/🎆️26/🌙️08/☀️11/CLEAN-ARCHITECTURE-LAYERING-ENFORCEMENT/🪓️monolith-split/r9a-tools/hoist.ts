// 🪜️ Hoists the pure `#[path = "."]` mount wrappers of component files back into the inline wrapper of the artifact root nest that mounts the component (inverse of the bim mount re-homing for the named files).
// usage: bun hoist.ts <root.rs> <component-file-substring>[,..] [--apply]
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { parseRustSourceTree, type RustModuleNode } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [rootArg, onlyArg, ...flags] = process.argv.slice(2);
const apply = flags.includes("--apply");
const wrapperize = (flags.find((flag) => flag.startsWith("--wrapperize=")) ?? "").slice("--wrapperize=".length).split(",").filter(Boolean);
const root = resolve(rootArg!);
const rootDir = dirname(root);
const only = (onlyArg ?? "").split(",").filter(Boolean);
const texts = new Map<string, string>();
const read = (path: string): string => { const t = texts.get(path) ?? readFileSync(path, "utf8"); texts.set(path, t); return t; };
const isDotWrapper = (node: RustModuleNode): boolean => node.inline && node.pathAttribute === ".";
const leadingComments = (text: string, start: number): number => {
  let cursor = start;
  for (;;) {
    const lineStart = text.lastIndexOf("\n", cursor - 2) + 1;
    const line = text.slice(lineStart, cursor).trim();
    if (cursor > 0 && line.startsWith("//") && !line.startsWith("//#")) { cursor = lineStart; continue; }
    return cursor;
  }
};
const rebase = (wrapper: string, fromDir: string): string => wrapper.replace(/#\[\s*path\s*=\s*"([^"]*)"\s*\]/gu, (whole, p: string) => (p === "." ? whole : `#[path = "${relative(rootDir, resolve(fromDir, p))}"]`));
const log: string[] = [];
const indentOf = (text: string, at: number): string => { const lineStart = text.lastIndexOf("\n", at - 1) + 1; return /^[ \t]*/u.exec(text.slice(lineStart))![0]; };
const reindent = (block: string, indent: string): string => block.split("\n").map((line, index) => (index === 0 || !line.length ? line : indent + line)).join("\n");
let moved = 0;
const visit = (file: string, parseOf: () => ReturnType<typeof parseRustSourceTree>): boolean => {
  const text = read(file);
  const tree = parseRustSourceTree(text);
  const edits: { start: number; end: number; text: string }[] = [];
  const walk = (nodes: readonly RustModuleNode[], level: number): void => {
    for (const node of nodes) {
      if (!node.inline) continue;
      const component = node.children.find((child) => !child.inline && child.pathAttribute !== null && !child.conditionalTest && /component$/u.test(child.name) && node.items.some((item) => item.kind === "use"));
      if (component && only.some((needle) => component.pathAttribute!.includes(needle))) {
        const cfile = resolve(rootDir, component.pathAttribute!);
        const ctext = read(cfile);
        const ctree = parseRustSourceTree(ctext);
        const wrappers = ctree.modules.filter(isDotWrapper);
        const moduleCuts: { start: number; end: number }[] = [];
        for (const mod of ctree.modules) {
          if (mod.inline || mod.conditionalTest || mod.pathAttribute === null || !wrapperize.some((needle) => mod.pathAttribute!.includes(needle))) continue;
          const nfile = resolve(dirname(cfile), mod.pathAttribute);
          const ntext = read(nfile);
          if (!parseRustSourceTree(ntext).modules.some(isDotWrapper)) continue;
          const rebased = relative(rootDir, nfile);
          const block = `#[path = "."]\n${mod.visibility ? mod.visibility + " " : ""}mod ${mod.name} {\n    #[path = "${rebased}"]\n    mod component;\n    pub use component::*;\n}`;
          const start = leadingComments(ctext, mod.start);
          moduleCuts.push({ start, end: mod.end });
          const closeAt0 = text.lastIndexOf("\n", node.bodyEnd - 1) + 1;
          const indent0 = indentOf(text, closeAt0) + "    ";
          edits.push({ start: closeAt0, end: closeAt0, text: indent0 + reindent(block, indent0) + "\n" });
          moved += 1;
          log.push(`${component.pathAttribute}: wrapperized ${mod.name}`);
        }
        if (moduleCuts.length && !wrappers.length) {
          let next0 = texts.get(cfile) ?? ctext;
          for (const cut of moduleCuts.sort((a, b) => b.start - a.start)) next0 = next0.slice(0, cut.start) + next0.slice(cut.end);
          texts.set(cfile, next0);
        }
        if (wrappers.length) {
          const cuts: { start: number; end: number }[] = [];
          const blocks: string[] = [];
          for (const wrapper of wrappers) {
            const start = leadingComments(ctext, wrapper.start);
            blocks.push(rebase(ctext.slice(start, wrapper.end), dirname(cfile)));
            cuts.push({ start, end: wrapper.end });
          }
          cuts.push(...moduleCuts);
          let next = ctext;
          for (const cut of cuts.sort((a, b) => b.start - a.start)) next = next.slice(0, cut.start) + next.slice(cut.end);
          next = next.replace(/\n{3,}/gu, "\n\n").replace(/\/\/#region [^\n]*\n\s*\/\/#endregion [^\n]*\n?/gu, "");
          texts.set(cfile, next);
          const names = new Set(wrappers.map((wrapper) => wrapper.name));
          const closeAt = text.lastIndexOf("\n", node.bodyEnd - 1) + 1;
          const indent = indentOf(text, closeAt) + "    ";
          const body = blocks.map((block) => indent + reindent(block, indent)).join("\n");
          edits.push({ start: closeAt, end: closeAt, text: body + "\n" });
          moved += wrappers.length;
          log.push(`${component.pathAttribute}: hoisted ${[...names].join(", ")}`);
        }
      }
      walk(node.children, level + 1);
    }
  };
  walk(tree.modules, 0);
  if (!edits.length) return false;
  let out = text;
  for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  texts.set(file, out);
  return true;
};
for (let round = 0; round < 8; round += 1) { if (!visit(root, () => parseRustSourceTree(read(root)))) break; }
console.log(log.join("\n"));
console.log("moved wrappers", moved, "files", [...texts.keys()].filter((p) => p !== root).length);
if (apply) for (const [path, text] of texts) writeFileSync(path, text);
else { writeFileSync("/Users/ueli/.cache/r9a/hoist.preview.root.rs", texts.get(root)!); }
