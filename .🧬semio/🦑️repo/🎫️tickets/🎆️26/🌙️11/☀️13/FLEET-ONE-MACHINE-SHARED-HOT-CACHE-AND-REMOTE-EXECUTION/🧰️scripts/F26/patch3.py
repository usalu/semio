import hashlib
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/"
def edit(path,pairs):
    t=open(D+path,encoding="utf8").read();b=hashlib.sha256(t.encode()).hexdigest()[:12]
    for old,new in pairs:
        assert t.count(old)==1,(path,old[:50],t.count(old));t=t.replace(old,new)
    open(D+path,"w",encoding="utf8").write(t);print(path,b,"->",hashlib.sha256(t.encode()).hexdigest()[:12],len(t.encode()))
helper='''/** 🔎️ Names the entries that appeared and disappeared in a directory (at most ten of each, then a count), in English and German.
 * @see ./🧪️tests/⏳️in-flight-temporary/🟦️.ts */
export function directoryChangeV1(before:readonly string[],after:readonly string[]):{en:string;de:string}{
 const was=new Set(before),is=new Set(after),list=(names:string[])=>names.slice(0,10).join(", ")+(names.length>10?" +"+(names.length-10):""),added=after.filter(name=>!was.has(name)),gone=before.filter(name=>!is.has(name));
 return added.length||gone.length?{en:"appeared "+added.length+": "+list(added)+"; disappeared "+gone.length+": "+list(gone),de:"neu "+added.length+": "+list(added)+"; verschwunden "+gone.length+": "+list(gone)}:{en:"no entry name changed",de:"kein Eintragsname geändert"};
}
export function closureReadWorkerScopeV1('''
edit("📡️session/🟦️.ts",[("export function closureReadWorkerScopeV1(",helper)])
old='throw Error(`Closure reference directory changed: ${directory.path} (${directory.names.length} entries before) / Verzeichnis der Abschlussreferenz geändert: ${directory.path} (${directory.names.length} Einträge zuvor)`);'
new='{const change=directoryChangeV1(directory.names,row.isDirectory()?namesAt(proof,directory.path):[]);throw Error(`Closure reference directory changed: ${directory.path} (${directory.names.length} entries before; ${change.en}) / Verzeichnis der Abschlussreferenz geändert: ${directory.path} (${directory.names.length} Einträge zuvor; ${change.de})`);}'
edit("🟦️.ts",[(old,new),('import{runClosureReadCohortsV1}from"./📡️session/🟦️.ts";','import{directoryChangeV1,runClosureReadCohortsV1}from"./📡️session/🟦️.ts";')])

import re
q=D+"🟦️.ts";s=open(q,encoding="utf8").read()
s=re.sub(r"/\*\* ⏳️ The known publication[^\n]*\n \* @see[^\n]*\n","/** ⏳️ The known publication an in-flight temporary belongs to, else undefined: its writer's, only retired as an orphan. */\n",s,count=1)
s=re.sub(r"(/\*\* 🪦️ [^\n]*)\n \* @see[^\n]*\n",lambda m:m.group(1)[:-3]+" */\n",s,count=1)
open(q,"w",encoding="utf8").write(s);print(len(s.encode()))
assert len(s.encode())<65536
