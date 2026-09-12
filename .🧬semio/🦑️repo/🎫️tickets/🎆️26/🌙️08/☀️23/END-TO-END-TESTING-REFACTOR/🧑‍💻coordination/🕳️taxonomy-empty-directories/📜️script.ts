import { existsSync, lstatSync, readdirSync, rmdirSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve, relative, dirname, join } from "node:path";
const root = process.env.SEMIO_TAXONOMY_REPO_ROOT ?? process.cwd();
const ticket = resolve(import.meta.dir, "../..");
const residual = process.argv[2] === "residual";
const census = JSON.parse(readFileSync(join(ticket, residual ? "🗑️generated/testing-taxonomy/census/findings.json" : "🗑️generated/taxonomy-guard/physical-census.json"), "utf8")) as {code:string;path:string}[];
const eligible = new Set(["legacy-fixture-directory", "obsolete-testing-category", "fixture-owner-delivery-scope", "fixture-in-test-case"]);
const candidates = [...new Set(census.filter(f=>eligible.has(f.code)).map(f=>f.path))].filter(p=>existsSync(join(root,p)) && lstatSync(join(root,p)).isDirectory());
const removed: string[] = [], preserved: string[] = [];
function prune(path: string): void {
  if (!existsSync(path) || !lstatSync(path).isDirectory()) return;
  for (const entry of readdirSync(path, {withFileTypes:true})) if (entry.isDirectory() && !entry.isSymbolicLink()) prune(join(path,entry.name));
  if (readdirSync(path).length) return;
  try { rmdirSync(path); removed.push(relative(root,path)); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOTEMPTY") throw error; }
}
for (const path of candidates.sort((a,b)=>b.length-a.length)) { prune(join(root,path)); if(existsSync(join(root,path))) preserved.push(path); }
const report = join(ticket,"📓️testing-taxonomy-empty-directories-2026-09-12.md");
writeFileSync(report, (residual && existsSync(report) ? readFileSync(report,"utf8")+"\n## Residual Empty Directories\n\n" : "# Testing Taxonomy Empty Directory Removal — 2026-09-12\n\n")+"Only empty directories from the guard census were removed with the filesystem empty-directory operation. Populated directories and all files were preserved. Each directory was checked again immediately before removal; concurrent nonempty directories remain untouched.\n\nRemoved "+removed.length+" directories.\n\n"+removed.map(p=>"- `"+p+"`").join("\n")+"\n\n## Populated Candidates Retained for Source Migration\n\n"+preserved.map(p=>"- `"+p+"`").join("\n")+"\n\nAuthored runner: `"+relative(root,import.meta.filename)+"`.\n");
console.log(JSON.stringify({removed:removed.length,preserved:preserved.length,report}));
