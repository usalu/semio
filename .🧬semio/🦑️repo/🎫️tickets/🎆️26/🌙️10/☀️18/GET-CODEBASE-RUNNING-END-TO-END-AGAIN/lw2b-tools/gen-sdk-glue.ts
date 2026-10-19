import ts from "typescript";
import { writeFileSync, mkdirSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const root = process.cwd();
const entry = resolve(root, process.argv[2]);
const out = resolve(root, process.argv[3]);
const key = process.argv[4];
const options: ts.CompilerOptions = {
  target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler,
  jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, strict: true, resolveJsonModule: true,
};
const program = ts.createProgram([entry], options);
const checker = program.getTypeChecker();
const source = program.getSourceFile(entry)!;
const moduleSymbol = checker.getSymbolAtLocation(source)!;
const values: string[] = [];
const types: string[] = [];
for (const exported of checker.getExportsOfModule(moduleSymbol)) {
  const target = exported.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(exported) : exported;
  const isValue = (target.flags & ts.SymbolFlags.Value) !== 0 && target.flags !== ts.SymbolFlags.Alias;
  (isValue ? values : types).push(exported.name);
}
values.sort(); types.sort();
const spec = relative(dirname(out), entry).replaceAll("\\", "/");
const lines = [`/** 🌍️ SDK entry \`${key}\`: explicit public surface of the framework module that plugins build on. */`];
if (values.length) lines.push(`export { ${values.join(", ")} } from "${spec}";`);
if (types.length) lines.push(`export type { ${types.join(", ")} } from "${spec}";`);
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, lines.join("\n") + "\n");
console.log(values.length, "values", types.length, "types", spec);
