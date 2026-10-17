// 🔬 Prints the checker's type of every identifier/property-access that starts at the given line (1-based) of a file inside a generated tsconfig. usage: node tt5_type.mjs <tsconfig> <repo-relative file> <line> [name-filter]
import { createRequire } from "node:module";
import { resolve, dirname } from "node:path";
const root = "/Users/ueli/Documents/semio";
const ts = createRequire(root + "/package.json")("typescript");
const [project, file, lineText, filter] = process.argv.slice(2);
const parsed = ts.getParsedCommandLineOfConfigFile(resolve(project), { noEmit: true }, { ...ts.sys, onUnRecoverableConfigFileDiagnostic: d => { throw new Error(ts.flattenDiagnosticMessageText(d.messageText, "\n")); } });
const target = resolve(root, file);
const program = ts.createProgram({ rootNames: [target, ...parsed.fileNames.filter(f => f !== target)].slice(0, 1), options: { ...parsed.options, noEmit: true }, projectReferences: parsed.projectReferences });
const checker = program.getTypeChecker();
const source = program.getSourceFile(target);
const line = Number(lineText) - 1;
const visit = node => {
  if ((ts.isIdentifier(node) || ts.isPropertyAccessExpression(node) || ts.isCallExpression(node)) && source.getLineAndCharacterOfPosition(node.getStart(source)).line === line) {
    const text = node.getText(source);
    if (text.length < 120 && (!filter || text.includes(filter))) console.log(text.padEnd(60), "::", checker.typeToString(checker.getTypeAtLocation(node), undefined, ts.TypeFormatFlags.NoTruncation).slice(0, 300));
  }
  ts.forEachChild(node, visit);
};
visit(source);
