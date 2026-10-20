import ts from "typescript";
import { join, relative } from "node:path";

const root = process.argv[2]!;
const course = join(root, "🎓️teaching/🏛️architecture/📚️course");
const bundle = join(course, "📦️packages/🟦️typescript");
const config = ts.readConfigFile(join(bundle, "tsconfig.json"), ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, bundle);
const wanted = process.argv.slice(3).map(path => join(course, path));
const program = ts.createProgram({ rootNames: wanted, options: { ...parsed.options, noEmit: true, incremental: false } });
const own = (file: string | undefined): boolean => file !== undefined && file.startsWith(course) && !file.includes("node_modules");
const diagnostics = ts.getPreEmitDiagnostics(program).filter(diagnostic => own(diagnostic.file?.fileName));
for (const diagnostic of diagnostics) console.log(`${relative(root, diagnostic.file!.fileName)}:${diagnostic.file!.getLineAndCharacterOfPosition(diagnostic.start!).line + 1} ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`);
console.log(`checked ${program.getSourceFiles().filter(file => own(file.fileName)).length} course files, ${diagnostics.length} diagnostics`);
process.exit(diagnostics.length ? 1 : 0);
