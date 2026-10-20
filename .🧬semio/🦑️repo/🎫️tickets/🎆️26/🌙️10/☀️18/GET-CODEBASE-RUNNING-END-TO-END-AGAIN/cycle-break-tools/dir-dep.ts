import { scan } from "../exports-stage-tools/scan.ts";
const [from, to, fromExcl, toExcl] = process.argv.slice(2);
const { edges, bare } = scan();
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
let run = 0, test = 0; const ex: string[] = [];
for (const e of edges) { if (!e.file.startsWith(from!) || (fromExcl && e.file.startsWith(fromExcl))) continue; if (!e.target.startsWith(to!) || (toExcl && e.target.startsWith(toExcl))) continue; if (isTest(e.file) || isTest(e.target)) test++; else { run++; if (ex.length < 40) ex.push(e.file.replace(from!, "F/") + " -> " + e.target.replace(to!, "T/")); } }
console.log("runtime", run, "test", test); console.log(ex.join("\n"));
