/**
 * 🚚 Moves one exported top-level function (with its leading docstring) from one module to another and rewires the named import.
 *
 *   bun move-fn.ts <fromFile> <toFile> <name> <fromSpecifierInToFile> <toSpecifierInFromFile>
 */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const [from, to, name, specInTo, specInFrom] = process.argv.slice(2) as [string, string, string, string, string];
const src = readFileSync(from, "utf8");
const sf = ts.createSourceFile(from, src, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const fn = sf.statements.find((s): s is ts.FunctionDeclaration => ts.isFunctionDeclaration(s) && s.name?.text === name)!;
const start = fn.getFullStart(), end = fn.getEnd();
const text = src.slice(start, end).replace(/^\n+/u, "");
let out = src.slice(0, start) + src.slice(end).replace(/^\n/u, "");
const imp = out.match(new RegExp(`import \\{([^}]*)\\} from "${specInFrom.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}";`, "u"));
if (imp) out = out.replace(imp[0], `import {${imp[1]!.replace(/\s+$/u, "")}, ${name} } from "${specInFrom}";`);
else throw Error("no import of " + specInFrom + " in " + from);
writeFileSync(from, out);
let dst = readFileSync(to, "utf8");
const back = new RegExp(`import \\{ ?${name} ?\\} from "${specInTo.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}";\\n`, "u");
if (!back.test(dst)) throw Error("no single-name import in " + to);
dst = dst.replace(back, "");
writeFileSync(to, dst.replace(/\n*$/u, "\n\n") + text + "\n");
console.log("moved", name, text.length);
