import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const generated = join(import.meta.dir, "🗑️generated");
const git = (...args: string[]) => { const p = spawnSync("git", args, { encoding: "utf8", windowsHide: true, maxBuffer: 128 * 1024 * 1024 }); if (p.status !== 0) throw new Error(JSON.stringify({error:p.error,status:p.status,stderr:p.stderr})); return p.stdout; };
const entries = () => Object.fromEntries(git("ls-files", "--stage", "-z").split("\0").filter(Boolean).map(line => { const tab = line.indexOf("\t"); return [line.slice(tab + 1), line.slice(0, tab)]; }));
if (process.argv[2] === "before") {
 const index = entries(); writeFileSync(join(generated, "index-before.json"), JSON.stringify(index));
 const staged = git("diff", "--cached", "--name-only", "-z").split("\0").filter(Boolean); writeFileSync(join(generated,"staged-before.json"),JSON.stringify(staged));
 console.log(JSON.stringify({ indexEntries: Object.keys(index).length, alreadyStaged: staged.length }));
} else {
 const before = JSON.parse(readFileSync(join(generated,"index-before.json"),"utf8")); const after = entries();
 const changed = Object.keys(after).filter(p => after[p] !== before[p]); const removed = Object.keys(before).filter(p=>!(p in after));
 writeFileSync(join(generated,"index-after.json"),JSON.stringify(after));
 console.log(JSON.stringify({ changed, removed },null,2));
}
