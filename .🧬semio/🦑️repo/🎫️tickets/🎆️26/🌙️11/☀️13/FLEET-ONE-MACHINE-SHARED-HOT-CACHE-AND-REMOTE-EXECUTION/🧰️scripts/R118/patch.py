import sys,re
kind,path=sys.argv[1],sys.argv[2]
s=open(path,encoding="utf8").read()
def once(old,new):
    global s
    assert s.count(old)==1,(kind,old[:80],s.count(old))
    s=s.replace(old,new)
if kind=="observation":
    once('import { sha256HexV1, utf8TextV1 } from "../🧂️text-bytes/🟨️.mjs";\n','import { sha256HexV1, utf8TextV1 } from "../🧂️text-bytes/🟨️.mjs";\nimport { sourceTreeFilterV1 } from "./📂️source-names/🟨️.mjs";\n')
    old=re.search(r'/\*\* 🆔️ Fingerprints exactly the names or typed entries a directory read returned\. \*/\nfunction directoryFactV1\(entries,typed=false\)\{[^\n]*\n',s)
    assert old
    new='''/** 🆔️ Fingerprints the names or typed entries of a directory read that belong to the source tree as the build sees it (`📂️source-names`): machine-local output and operating-system noise are not part of the fact, so a record made on one machine holds on every other and after a build. A name of the generated-directory list leaves only where the entry is a directory or a link (`directory` is where the untyped names were listed; none means every name is a directory). */
function directoryFactV1(entries,typed=false,directory=null){
 const kept=sourceTreeFilterV1(entries,entry=>String(typed?entry.name:entry),entry=>typed?entry.isDirectory()||entry.isSymbolicLink():directory===null||directoryLinkOrDirectoryV1(join(directory,String(entry))));
 return sha256HexV1(JSON.stringify(kept.map(entry=>typed?[String(entry.name),nodeType(entry)]:String(entry)).sort()));
}
function directoryLinkOrDirectoryV1(path){try{const stat=world.lstatSync(path);return stat.isDirectory()||stat.isSymbolicLink();}catch{return false;}}
'''
    s=s.replace(old.group(0),new)
    once('readdirNames: (path) => {probeCountsV1.readdir++;try{return directoryFactV1(world.readdirSync(path));}','readdirNames: (path) => {probeCountsV1.readdir++;try{return directoryFactV1(world.readdirSync(path),false,path);}')
    once('value=>directoryFactV1(value,typed));},','value=>directoryFactV1(value,typed,path));},')
    once('/** 🔗️ The seam of the shared command closure entries','''/** 🔦️ The directory fact of `path` for one of the kinds that read a directory (`readdir`, `readdirNames`, `directories`), as a record row would hold it. */
export function directoryFactOfV1(kind,path){if(kind!=="readdir"&&kind!=="readdirNames"&&kind!=="directories")throw Error("Directory fact kind required");return facts[kind](path);}

/** 🔗️ The seam of the shared command closure entries''')
elif kind=="declarations":
    s=s.rstrip("\n")+'\n/** 🔦️ The directory fact of `path` for one of the kinds that read a directory (`readdir`, `readdirNames`, `directories`), as a record row would hold it. */\nexport function directoryFactOfV1(kind:"readdir"|"readdirNames"|"directories",path:string):string;\n'
elif kind=="assembly":
    once('import { observedTypeScriptHost } from "../../../../🏗️build/👁️observation/🟨️.mjs";\n','import { observedTypeScriptHost } from "../../../../🏗️build/👁️observation/🟨️.mjs";\nimport { sourceTreeFilterV1 } from "../../../../🏗️build/👁️observation/📂️source-names/🟨️.mjs";\n')
    once('record("directories", path, JSON.stringify(value));','record("directories", path, JSON.stringify(sourceTreeFilterV1(value, name => name, () => true)));')
    once('if (kind === "directories") return JSON.stringify(sys.getDirectories(path));','if (kind === "directories") return JSON.stringify(sourceTreeFilterV1(sys.getDirectories(path), name => name, () => true));')
elif kind=="script-inputs":
    once('/** 📁️ Directories a declared glob never descends into. */\nconst DECLARED_SKIPPED_DIRECTORIES_V1 = new Set(["node_modules", ".git", "target"]);\n','')
    once('      for (const entry of readdir(join(workspaceRoot, relative)) ?? []) {','      for (const entry of sourceTreeFilterV1(readdir(join(workspaceRoot, relative)) ?? [], row => row.name, row => row.directory || row.link === true)) {')
    once('if (entry.directory) { if (!DECLARED_SKIPPED_DIRECTORIES_V1.has(entry.name)) walk(path); }','if (entry.directory) walk(path);')
    once('list = directory => { try { return observedFs.readdirSync(local(directory), { withFileTypes: true }).map(','list = directory => { try { return sourceTreeFilterV1(observedFs.readdirSync(local(directory), { withFileTypes: true }), entry => entry.name, entry => entry.isDirectory() || entry.isSymbolicLink()).map(')
    m=re.search(r'^import [^\n]*observation/🟨️\.mjs";\n',s,re.M)
    assert m
    s=s.replace(m.group(0),m.group(0)+'import { sourceTreeFilterV1 } from "../../../../🏗️build/👁️observation/📂️source-names/🟨️.mjs";\n',1)
elif kind=="corpus":
    assert s.count("pkg")>0 and "pak" not in s
    s=s.replace("pkg","pak")
open(path,"w",encoding="utf8").write(s)
print("patched",kind)
