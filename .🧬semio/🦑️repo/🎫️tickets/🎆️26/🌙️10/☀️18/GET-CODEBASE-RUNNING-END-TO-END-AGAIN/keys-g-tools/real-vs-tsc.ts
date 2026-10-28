// real-vs-tsc: bun real-vs-tsc.ts <tsconfig> <tsc-listFilesOnly.txt> [learned.json] : production import closure of the tsconfig roots against the TypeScript program and the oracle's learned reads.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, relative} from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const [tsconfig, listing, learnedPath] = process.argv.slice(2);
const ts = (await import("typescript")).default;
const path = `${W}/${tsconfig}`, read = ts.readConfigFile(path, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(read.config, ts.sys, dirname(path), undefined, path);
const roots = parsed.fileNames.filter((file) => file.startsWith(W + "/") && !file.includes("/node_modules/") && /\.[cm]?[jt]sx?$/u.test(file));
const inference: any = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const found = await inference.cacheInternals.relativeScriptInputs(roots, W, scripts);
const closure = new Set<string>(found.map((p: string) => p.replace(/^\{workspaceRoot\}\//u, "")));
const program = new Set(readFileSync(listing, "utf8").split("\n").filter((line) => line.startsWith(W + "/") && !line.includes("/node_modules/")).map((line) => relative(W, line)));
const learned: string[] = learnedPath && existsSync(learnedPath) ? JSON.parse(readFileSync(learnedPath, "utf8")).learned : [];
const miss = (set: Set<string>) => learned.filter((file) => !set.has(file));
console.log(JSON.stringify({roots: roots.length, program: program.size, closure: closure.size, onlyProgram: [...program].filter((f) => !closure.has(f)).length, onlyClosure: [...closure].filter((f) => !program.has(f)).length, learned: learned.length, learnedMissingFromClosure: miss(closure), learnedMissingFromProgram: miss(program).length}, null, 1));
