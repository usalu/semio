import Parser from "web-tree-sitter";
import { dirname, join } from "node:path";
import { existsSync, readFileSync, appendFileSync } from "node:fs";

const ticket = dirname(dirname(import.meta.path));
const host = process.argv[2] === "host-current";
const reports = host ? ["native-host-integration-current.md"] : ["semantic-diff-extraction.md", "semantic-replay-extraction.md", "lowpoly-semantic-bytes.md", "semantic-diff-codec-coverage.md"];
const files = new Set(reports.flatMap(report => readFileSync(join(ticket, report), "utf8").split("\n").filter(line => line.startsWith("- ") && line.endsWith(".rs")).map(line => line.slice(2))).filter(file => existsSync(file)));
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
const nativeGrammarFiles: string[] = [];
for (const file of files) {
  const tree = parser.parse(readFileSync(file, "utf8"))!;
  if (tree.rootNode.hasError()) {
    const pending = [tree.rootNode];
    const errors = [];
    while (pending.length && errors.length < 12) {
      const node = pending.pop()!;
      if (node.type === "ERROR") errors.push({ start: node.startPosition, end: node.endPosition, source: node.text.slice(0, 180) });
      else if (node.hasError()) pending.push(...node.children.reverse());
    }
    console.error(`[DEBUG] Rust grammar diagnostic ${file}: ${JSON.stringify(errors)}`);
    const native = Bun.spawn(["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", file], { stdout: "ignore", stderr: "pipe" });
    const diagnostic = await new Response(native.stderr).text();
    if (await native.exited !== 0) throw Error(`Native Rust syntax invalid: ${file}\n${diagnostic}`);
    nativeGrammarFiles.push(file);
  }
  tree.delete();
}
console.log(`[DEBUG] semantic-final-rust-syntax files=${files.size} treeSitter=${files.size-nativeGrammarFiles.length} nativeRustfmt=${nativeGrammarFiles.length}`);
appendFileSync(join(ticket, host ? "native-host-integration-current.md" : "semantic-native-verification.md"), `\n## Final Source Audit\n\nParsed all ${files.size} existing Rust files listed in the selected changed-file reports: ${files.size-nativeGrammarFiles.length} passed tree-sitter; ${nativeGrammarFiles.length} required the installed native Rust parser because the tree-sitter grammar refused current syntax. Native rustfmt runs use --emit stdout and skip_children=true with discarded stdout, so they do not write source files. Native-parser files: ${nativeGrammarFiles.join(", ") || "none"}. This establishes source syntax only, not compilation or runtime correctness.\n`);
