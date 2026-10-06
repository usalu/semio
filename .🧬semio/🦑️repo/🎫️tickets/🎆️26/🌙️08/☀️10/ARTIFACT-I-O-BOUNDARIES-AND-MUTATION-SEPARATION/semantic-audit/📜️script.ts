import Parser from "web-tree-sitter";
import { dirname, join } from "node:path";
import { existsSync, readFileSync, appendFileSync } from "node:fs";

const ticket = dirname(dirname(import.meta.path));
const reports = ["semantic-diff-extraction.md", "semantic-replay-extraction.md", "lowpoly-semantic-bytes.md", "semantic-diff-codec-coverage.md"];
const files = new Set(reports.flatMap(report => readFileSync(join(ticket, report), "utf8").split("\n").filter(line => line.startsWith("- ") && line.endsWith(".rs")).map(line => line.slice(2))).filter(file => existsSync(file)));
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
for (const file of files) {
  const tree = parser.parse(readFileSync(file, "utf8"))!;
  if (tree.rootNode.hasError()) throw Error(`Rust syntax invalid: ${file}`);
  tree.delete();
}
console.log(`[DEBUG] semantic-final-rust-syntax files=${files.size} oracle=tree-sitter`);
appendFileSync(join(ticket, "semantic-native-verification.md"), `\n## Final Source Audit\n\nTree-sitter parsed all ${files.size} existing Rust files listed in this agent's four changed-file reports after the final codec and media edits, with no syntax errors. This establishes source syntax only, not compilation.\n\nBoth native owners have left Cargo preparation and report owner commands running. Energy session 24793 last reports elapsedMs=150044; Lowpoly session 95288 last reports elapsedMs=70019. The parent task retains these session IDs for final compiler and runtime monitoring.\n`);
