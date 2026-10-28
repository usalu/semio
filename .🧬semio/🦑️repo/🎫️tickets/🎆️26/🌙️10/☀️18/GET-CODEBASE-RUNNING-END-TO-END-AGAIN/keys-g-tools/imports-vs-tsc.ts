// imports-vs-tsc: bun imports-vs-tsc.ts <tsconfig> <tsc-listFilesOnly.txt> : the Bun-runtime import closure of the tsconfig roots against the TypeScript program (third-party oracle).
import {readFileSync} from "node:fs";
import {dirname, relative} from "node:path";
import {realHost, W} from "./realhost.ts";
const [tsconfig, listing] = process.argv.slice(2);
const ts = (await import("typescript")).default;
const path = `${W}/${tsconfig}`, read = ts.readConfigFile(path, ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(read.config, ts.sys, dirname(path), undefined, path);
const roots = parsed.fileNames.filter((file) => file.startsWith(W + "/") && !file.includes("/node_modules/")).map((file) => relative(W, file));
const {host} = await realHost("");
const closure = new Set(await host.importClosure(roots));
const program = new Set(readFileSync(listing, "utf8").split("\n").filter((line) => line.startsWith(W + "/") && !line.includes("/node_modules/")).map((line) => relative(W, line)));
const onlyProgram = [...program].filter((file) => !closure.has(file)), onlyClosure = [...closure].filter((file) => !program.has(file));
console.log(JSON.stringify({roots: roots.length, program: program.size, closure: closure.size, onlyProgram: onlyProgram.length, onlyClosure: onlyClosure.length, onlyProgramSample: onlyProgram.slice(0, 25), onlyClosureSample: onlyClosure.slice(0, 15)}, null, 1));
