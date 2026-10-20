/** 🔎 Type diagnostics of every TypeScript file the series touches, for one tree. usage: bun check-types.ts <root> <out.json>; compare the output of the mirror before and after the series. */
import ts from "typescript";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const root = resolve(process.argv[2]!), out = process.argv[3]!;
const touched = JSON.parse(readFileSync(join(import.meta.dir, "touched.json"), "utf8")) as { files: string[]; added: string[] };
const files = [...touched.files, ...touched.added].filter(path => /\.tsx?$/u.test(path) && existsSync(join(root, path)));
const bundle = join(root, "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript");
const config = ts.readConfigFile(join(bundle, "tsconfig.json"), ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, bundle);
const program = ts.createProgram({ rootNames: files.map(path => join(root, path)), options: { ...parsed.options, noEmit: true, incremental: false } });
const wanted = new Set(files);
const rows = ts.getPreEmitDiagnostics(program).filter(diagnostic => diagnostic.file && wanted.has(relative(root, diagnostic.file.fileName))).map(diagnostic => `${relative(root, diagnostic.file!.fileName)} TS${diagnostic.code} ${ts.flattenDiagnosticMessageText(diagnostic.messageText, " ").slice(0, 200)}`);
writeFileSync(out, JSON.stringify({ files: files.length, diagnostics: [...new Set(rows)].sort() }, null, 1));
console.log(`checked ${files.length} touched files, ${new Set(rows).size} distinct diagnostics`);
