import sys,re
w=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,"w",encoding="utf8").write(s)
edit(w+"/🌿️git/🟦️.ts",[
 ('''export async function trackedEntriesV1(worktree:string,signal?:AbortSignal):Promise<readonly GitTrackedV1[]|null>{''','''export async function trackedEntriesV1(worktree:string,signal?:AbortSignal):Promise<readonly GitTrackedV1[]|null>{'''),
 ('''/** 🔗 Lists the paths among the given symbolic-link entries''','''/** 🐚 Lists the tracked shell scripts (`*.sh`) of the worktree that contain a carriage return: the replica copies bytes and never applies `eol=lf`, so a script saved with CRLF on a Windows machine would break bash on Linux and macOS. `null` without git or a repository. */
export async function crlfScriptsV1(worktree:string,signal?:AbortSignal):Promise<readonly string[]|null>{
 const tracked=await trackedEntriesV1(worktree,signal);
 if(tracked===null)return null;
 const scripts=tracked.filter(entry=>entry.path.endsWith(".sh")&&(entry.mode==="100644"||entry.mode==="100755")),found:string[]=[];
 for(let start=0;start<scripts.length;start+=64){
  signal?.throwIfAborted();
  const verdicts=await Promise.all(scripts.slice(start,start+64).map(async entry=>{const bytes=await readFile(join(worktree,...entry.path.split("/"))).catch(()=>undefined);return bytes!==undefined&&bytes.includes(13)?entry.path:null;}));
  for(const path of verdicts)if(path!==null)found.push(path);
 }
 return found;
}

/** 🔗 Lists the paths among the given symbolic-link entries'''),
])
edit(w+"/🗺️topology/⌨️command/🟦️.ts",[
 ('import {gitGuardsV1,resolveGitV1,type GitGuardsV1}','import {crlfScriptsV1,gitGuardsV1,resolveGitV1,type GitGuardsV1}'),
 (' guards:(worktree:string,platform:NodeJS.Platform,longPaths:boolean|null,signal:AbortSignal)=>Promise<GitGuardsV1|null>;\n',' guards:(worktree:string,platform:NodeJS.Platform,longPaths:boolean|null,signal:AbortSignal)=>Promise<GitGuardsV1|null>;\n scripts:(worktree:string,signal:AbortSignal)=>Promise<readonly string[]|null>;\n'),
 (' guards:(worktree,platform,longPaths,signal)=>gitGuardsV1(worktree,{signal,platform,longPaths}),\n',' guards:(worktree,platform,longPaths,signal)=>gitGuardsV1(worktree,{signal,platform,longPaths}),\n scripts:(worktree,signal)=>crlfScriptsV1(worktree,signal),\n'),
])
s=open(w+"/🗺️topology/⌨️command/🟦️.ts",encoding="utf8").read()
i=s.index(' guarded("guards.symlinks"')
j=s.index("\n",i)
s=s[:j+1]+' add("guards.scripts",Math.max(timeoutMilliseconds,120000),async()=>{const found=await probes.scripts(worktree,signal);return found===null?skip("guards.skip"):found.length>0?warn("guards.scripts.crlf",{count:found.length,paths:found.slice(0,5).join(", ")}):ok("guards.scripts.ok");});\n'+s[j+1:]
open(w+"/🗺️topology/⌨️command/🟦️.ts","w",encoding="utf8").write(s)
edit(w+"/🗺️topology/📖️texts/🔣️.json",[
 ('''    "preflight.guards.skip": {''','''    "preflight.name.guards.scripts": {
      "en": "shell script line endings",
      "de": "Zeilenenden der Shell-Skripte"
    },
    "preflight.guards.scripts.ok": {
      "en": "no tracked shell script contains a carriage return",
      "de": "kein versioniertes Shell-Skript enthält ein Wagenrücklaufzeichen"
    },
    "preflight.guards.scripts.crlf": {
      "en": "{count} tracked shell scripts contain a carriage return (CRLF), which breaks bash on Linux and macOS: {paths}; save them with LF line endings",
      "de": "{count} versionierte Shell-Skripte enthalten ein Wagenrücklaufzeichen (CRLF), das bash unter Linux und macOS bricht: {paths}; mit LF-Zeilenenden speichern"
    },
    "preflight.guards.skip": {'''),
])
edit(w+"/🗺️topology/🧪️tests/🟦️.ts",[
 ('git:async()=>"/usr/bin/git",guards:async()=>GUARDS,...extra','git:async()=>"/usr/bin/git",guards:async()=>GUARDS,scripts:async()=>[],...extra'),
 ('"guards.symlinks":"ok:guards.symlinks.ok","service.replica"','"guards.symlinks":"ok:guards.symlinks.ok","guards.scripts":"ok:guards.scripts.ok","service.replica"'),
 ('expect(report.text).toContain("All 21 checks passed.");','expect(report.text).toContain("All 22 checks passed.");'),
 ('expect(good.text).toContain("Alle 21 Prüfungen bestanden.");\n expect(good.text).not.toContain("All 21 checks passed.");','expect(good.text).toContain("Alle 22 Prüfungen bestanden.");\n expect(good.text).not.toContain("All 22 checks passed.");'),
 ('expect(english.text).toContain("All 21 checks passed.");','expect(english.text).toContain("All 22 checks passed.");'),
 ('"guards.paths","guards.symlinks"].map(id=>[id,"skip:guards.sk','"guards.paths","guards.symlinks","guards.scripts"].map(id=>[id,"skip:guards.sk'),
])
