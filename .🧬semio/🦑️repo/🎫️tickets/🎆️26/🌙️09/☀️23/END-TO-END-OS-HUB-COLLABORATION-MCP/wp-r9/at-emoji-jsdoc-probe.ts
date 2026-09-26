import * as ts from "typescript";
const source = `/** @emoji 🧹️ Tagged summary. */\nexport const a = 1;\n/** 🧹️ Plain summary. */\nexport const b = 2;\n`;
const host = ts.createCompilerHost({});
const file = ts.createSourceFile("probe.ts", source, ts.ScriptTarget.Latest, true);
host.getSourceFile = (name) => (name === "probe.ts" ? file : undefined);
const program = ts.createProgram(["probe.ts"], { noLib: true }, host);
const checker = program.getTypeChecker();
for (const symbol of checker.getExportsOfModule(checker.getSymbolAtLocation(file)!)) {
  console.log(symbol.name, JSON.stringify({ documentation: ts.displayPartsToString(symbol.getDocumentationComment(checker)), tags: symbol.getJsDocTags(checker).map((tag) => ({ name: tag.name, text: ts.displayPartsToString(tag.text) })) }));
}
