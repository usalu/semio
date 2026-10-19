import { existsSync, readFileSync, readdirSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { Database } from "bun:sqlite";
import glob from "fast-glob";
import Parser from "web-tree-sitter";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import fixture from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json";

const plugins = "/Users/ueli/Documents/semio/✏️s/🔌️plugins";
const parts = (file: string): string[] => file.split(/[\\/]/u);
const opaque = new Set(["node_modules", "target", "dist", ".git", ".nx", "🗑️generated", "🤖️generated"]);
const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() && !opaque.has(entry.name) ? walk(join(directory, entry.name)) : entry.isFile() ? [join(directory, entry.name)] : []);
const files = fixture.owners.flatMap(owner => walk(join(plugins, owner)));
const accepted = (file: string): boolean => {
  const path = parts(file), tests = path.indexOf("🧪️tests"), examples = path.indexOf("🧫️fixtures");
  return tests >= 0 && examples > tests && !path.slice(0, examples).some(part => ["🧬️schema", "🖼️assets"].includes(part));
};

const cfgWithoutTests = (expression: string): boolean | undefined => {
  if (expression === "test") return false;
  const group = /^(all|any|not)\((.*)\)$/u.exec(expression);
  if (!group) return undefined;
  const args: string[] = []; let depth = 0, quoted = false, start = 0;
  for (let i = 0; i < group[2]!.length; i++) {
    const character = group[2]![i];
    if (character === '"') quoted = !quoted;
    if (!quoted) {
      if (character === "(") depth++;
      if (character === ")") depth--;
      if (character === "," && depth === 0) { args.push(group[2]!.slice(start, i)); start = i + 1; }
    }
  }
  if (group[2]!.slice(start)) args.push(group[2]!.slice(start));
  const values = args.map(cfgWithoutTests);
  if (group[1] === "not") return values.length === 1 && values[0] !== undefined ? !values[0] : undefined;
  if (group[1] === "all") return values.includes(false) ? false : values.includes(undefined) ? undefined : true;
  return values.includes(true) ? true : values.includes(undefined) ? undefined : false;
};


const bad: string[] = [];
await Parser.init(); const parser = new Parser();
parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", "/Users/ueli/Documents/semio/✏️s")), "out/tree-sitter-rust.wasm")));
for (const file of files.filter(file => file.endsWith(".rs"))) {
  const source = readFileSync(file, "utf8");
  for (const match of source.matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
    const target = resolve(dirname(file), match[1]!);
    if (!existsSync(target)) { bad.push("MISSING " + file + " => " + match[1]); continue; }
    if (!parts(file).includes("🧪️tests")) {
      const tree = parser.parse(source)!;
      let node: Parser.SyntaxNode | null = tree.rootNode.descendantForIndex(match.index!);
      let testing = false;
      while (node) {
        let previous = node.previousNamedSibling;
        while (previous?.type === "attribute_item") {
          const cfg = /^#\[cfg\((.*)\)\]$/u.exec(previous.text.replace(/\s/gu, ""));
          testing ||= Boolean(cfg && cfgWithoutTests(cfg[1]!) === false);
          previous = previous.previousNamedSibling;
        }
        node = node.parent;
      }
      if (!testing) bad.push("NOTEST " + file.replace(plugins + "/", "") + ":" + (source.slice(0, match.index!).split("\n").length));
      tree.delete();
    }
  }
}
console.log(bad.join("\n")); console.log(bad.length, "offenders");
