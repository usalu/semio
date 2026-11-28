/** Regenerates the register of cache writers that rename without fsync: live files, with the staged copy used where one exists.
usage: bun regenerate-allowlist.ts <live library root> <stage library root> <output json> */
import {existsSync,lstatSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
const [live,stage,output]=process.argv.slice(2) as [string,string,string];
const rename=/\brename(?:Sync)?\(/u,durable=/fsync|writeDurable|writeSynced|\.sync\(\)|datasync/iu,skipped=/(^|\/)(node_modules|🧪️tests|🧰️scripts|🗑️generated|🤖️generated)(\/|$)/u;
const files:string[]=[];
for(const top of ["🏗️build","⚡️caching"])for(const file of new Bun.Glob("**/*.{ts,mjs}").scanSync({cwd:join(live,top),followSymlinks:false})){
 const path=top+"/"+file;if(skipped.test(path)||path.endsWith(".d.mts"))continue;
 const staged=join(stage,path),use=existsSync(staged)&&!lstatSync(staged).isSymbolicLink()?staged:join(live,path),text=readFileSync(use,"utf8");
 if(rename.test(text)&&!durable.test(text))files.push(path);
}
files.sort();
writeFileSync(output,JSON.stringify({version:1,description:{en:"Cache writers that publish by temporary file and rename but do not fsync yet (control records, stamps, journals and cache records alike). The register may only shrink: a new writer must use writeDurableSyncV1, writeDurableV1 or writeSyncedSyncV1 of the cache-repair member, or fsync itself; a file that became durable must leave it.",de:"Cache-Schreiber, die über temporäre Datei und rename veröffentlichen, aber noch kein fsync tun (Kontrollsätze, Stempel, Journale und Cache-Einträge). Das Register darf nur schrumpfen: Ein neuer Schreiber nutzt writeDurableSyncV1, writeDurableV1 oder writeSyncedSyncV1 des Mitglieds cache-repair oder ruft selbst fsync auf; eine Datei, die dauerhaft schreibt, muss es verlassen."},files},null,1)+"\n");
console.log(files.length);
