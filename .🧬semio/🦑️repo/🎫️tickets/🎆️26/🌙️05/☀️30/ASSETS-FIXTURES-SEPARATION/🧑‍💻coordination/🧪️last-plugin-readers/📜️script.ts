import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const out = join(ticket, "🗑️generated/coordinator/last-plugin-readers");
mkdirSync(out, { recursive: true });
const results: unknown[] = [];
try {
 const module = await import(pathToFileURL(join(root, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts")).href);
 results.push({ name: "CAD presence retirement", passed: true, result: module.cadPresenceRetirementSelfTests() });
} catch(error) { results.push({ name: "CAD presence retirement", passed: false, error: String(error) }); }
for (const [name,args] of [
 ["Note canonical cohort", ["test", join(root, "✏️s/🔌️plugins/🗒️note/🧪️tests/🧭️action-cohort/🟦️.ts")]],
 ["Flow and Note package cohort", [join(root, "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts"),"action-cohort-audit"]]
] as [string,string[]][]) {
 if (process.argv[2] === "cad") continue;
 const child = Bun.spawn([process.execPath,...args], {cwd:root, env:process.env, stdout:"pipe", stderr:"pipe"});
 const [stdout,stderr,status] = await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 writeFileSync(join(out,name+".log"),stdout+stderr);
 results.push({name,passed:status===0,status});
}
writeFileSync(join(out,"results.json"),JSON.stringify(results,null,2)+"\n");
console.log("[DEBUG] "+JSON.stringify(results));
process.exitCode=results.every((row:any)=>row.passed)?0:1;
