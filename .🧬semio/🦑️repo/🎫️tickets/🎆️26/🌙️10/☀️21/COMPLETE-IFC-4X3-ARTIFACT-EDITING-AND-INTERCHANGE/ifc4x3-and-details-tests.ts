/** 🧪 Execute current registered IFC4x3 and shared details suites without the unavailable build router. */
import { readFile, writeFile, stat } from "node:fs/promises";
import { resolve } from "node:path";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";

const root = resolve(import.meta.dir, "../../../../../../..");
const artifact = resolve(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc");
const registration = await readFile(resolve(artifact, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
const paths = [...registration.matchAll(/"(🏅️standards\/🔖️4x3\/[^"\r\n]+|🔎️details\/[^"\r\n]+|📚️catalog\/🔎️query\/[^"\r\n]+|📐️geometry\/(?:🕳️rings|🧊️boolean|⭕️circle|🔲️convex)\/[^"\r\n]+)"/gu)].map(match => resolve(artifact, match[1]!));
if (paths.length < 50 || new Set(paths).size !== paths.length) throw Error("Expanded current registration was not found");
for (const path of paths) if (!(await stat(path)).isFile()) throw Error("Registered suite missing: " + path);
const hashes=async()=>Promise.all(paths.map(async path=>({path,sha256:createHash("sha256").update(await readFile(path)).digest("hex")})));
const before=await hashes();
console.log("[DEBUG] " + JSON.stringify({ registeredSuites: paths.length, scope: "IFC4x3, details and shared query/ring" }));
const child = spawn(process.execPath, ["test", ...paths], { cwd: root, windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
let output = "";
for (const stream of [child.stdout, child.stderr]) stream?.on("data", chunk => { output += String(chunk); });
const result = await new Promise<{ code: number | null; signal: string | null }>((accept, reject) => { child.once("error", reject); child.once("close", (code, signal) => accept({ code, signal })); });
await writeFile(resolve(import.meta.dir, "🗑️generated/ifc4x3-and-details-tests.txt"), output);
const after=await hashes(),changedTests=before.filter((source,index)=>source.sha256!==after[index]?.sha256);
await writeFile(resolve(import.meta.dir, "🗑️generated/ifc4x3-and-details-selection.json"), JSON.stringify({ suites: paths.length, paths, before,after,changedTests,...result }, null, 2));
console.log("[DEBUG] " + JSON.stringify({...result,changedTests:changedTests.length}));
console.log(output.trim().split(/\r?\n/u).slice(-8).join("\n"));
process.exitCode = result.code ?? 1;

