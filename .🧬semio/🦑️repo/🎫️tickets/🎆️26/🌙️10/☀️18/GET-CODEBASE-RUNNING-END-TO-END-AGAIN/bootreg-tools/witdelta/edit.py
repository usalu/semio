#!/usr/bin/env python3
"""🕳 Anchored, idempotent edit of batch `witdelta` (lane bootreg): the Cargo preparation witness keeps each listing's entries and its replay tolerates appeared directories no listing plan can find a manifest in, and vanished directories it read no manifest from; usage: python3 -I edit.py <root> check|apply."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness"
TS, SC = f"{D}/🟦️.ts", f"{D}/🧬️schema/🔣️.json"
EDITS = {TS: [
 ('export type CargoPhysicalObservationV1=Readonly<{operation:CargoPhysicalOperationKindV1;path:string;stamp:string;stat?:string}>;',
  'export type CargoPhysicalObservationV1=Readonly<{operation:CargoPhysicalOperationKindV1;path:string;stamp:string;stat?:string;entries?:readonly string[]}>;'),
 (''' if(operation==="list")return sha256(JSON.stringify((value as readonly Dirent[]).filter(entry=>entry.isDirectory()||entry.isFile()&&entry.name===protocol.listedFile).map(entry=>entry.name).sort()));
 return sha256(String(value));
}
''', ''' if(operation==="list")return sha256(JSON.stringify(cargoListedEntriesV1(value as readonly Dirent[])));
 return sha256(String(value));
}
/** 📇️ Names the entries of one listing a Cargo workspace plan can act on: its sub-directories and its manifest, sorted. */
export function cargoListedEntriesV1(entries:readonly Dirent[]):string[]{return entries.filter(entry=>entry.isDirectory()||entry.isFile()&&entry.name===protocol.listedFile).map(entry=>entry.name).sort();}
const skippedV1=(name:string):boolean=>name.startsWith(".")||protocol.skippedNames.includes(name);
/** 🕳️ Holds when no listing plan can find a manifest below a directory that appeared: every plan skips its name, or its plan-visible subtree holds no manifest within the protocol's entry bound. */
function manifestFreeV1(path:string,name:string):boolean{
 if(name===protocol.listedFile)return false;if(skippedV1(name))return true;
 let budget=protocol.maximumAppearedEntries;const stack=[path];
 while(stack.length){const directory=stack.pop()!;for(const entry of readdirSync(directory,{withFileTypes:true})){if(--budget<0)return false;if(skippedV1(entry.name))continue;if(entry.isFile()&&entry.name===protocol.listedFile)return false;if(entry.isDirectory())stack.push(join(directory,entry.name));}}
 return true;
}
'''),
 (''' const value=answer(),stamp=cargoPhysicalStampV1(operation,value),key=operation+"\\0"+path,previous=observations.get(key),agreed=!previous||previous.stamp===stamp;
 observations.set(key,{operation,path,stamp:agreed?stamp:"",...(agreed&&stat&&(!previous||previous.stat===stat)?{stat}:{})});return value;''',
  ''' const value=answer(),entries=operation==="list"?cargoListedEntriesV1(value as unknown as readonly Dirent[]):undefined,stamp=entries?sha256(JSON.stringify(entries)):cargoPhysicalStampV1(operation,value),key=operation+"\\0"+path,previous=observations.get(key),agreed=!previous||previous.stamp===stamp;
 observations.set(key,{operation,path,stamp:agreed?stamp:"",...(agreed&&stat&&(!previous||previous.stat===stat)?{stat}:{}),...(agreed&&entries?{entries}:{})});return value;'''),
 ('''/** 🔁️ Asks every recorded physical question again under the plan's own directory and symlink guards and holds only when every answer is unchanged; an unchanged settled stat answers a listing or text question without reading it.''',
  '''/** 🔁️ Asks every recorded physical question again under the plan's own directory and symlink guards and holds only when every answer is unchanged or cannot change what a plan derives: an unchanged settled stat answers a listing or text question without reading it; a listing may differ only by appeared directories no plan can find a manifest in and by vanished directories below which no manifest, file kind or present path was read (their recorded questions are then not asked).'''),
 (''' const settled=(row:CargoPhysicalObservationV1):boolean=>row.stat!==undefined&&row.stat===statText(stat(row.path)!);
 try{
  for(const row of observations){
   if(!row.stamp)return false;let stamp:string;
   if(row.operation==="list"){if(!directory(row.path))return false;stamp=settled(row)?row.stamp:cargoPhysicalStampV1("list",readdirSync(row.path,{withFileTypes:true}));}''',
  ''' const settled=(row:CargoPhysicalObservationV1):boolean=>row.stat!==undefined&&row.stat===statText(stat(row.path)!);
 const vanished:string[]=[],within=(path:string):boolean=>vanished.some(prefix=>path===prefix||path.startsWith(prefix+sep));
 try{
  for(const row of observations){
   if(!row.stamp)return false;let stamp:string;
   if(within(row.path)){if(row.operation==="text"||row.operation==="presence"&&row.stamp==="true"||row.operation==="state"&&row.stamp==="file")return false;continue;}
   if(row.operation==="list"){
    if(!directory(row.path))return false;if(settled(row))continue;
    const entries=cargoListedEntriesV1(readdirSync(row.path,{withFileTypes:true}));stamp=sha256(JSON.stringify(entries));
    if(stamp!==row.stamp){if(!row.entries||sha256(JSON.stringify(row.entries))!==row.stamp)return false;const now=new Set(entries),then=new Set(row.entries);for(const name of row.entries)if(!now.has(name))vanished.push(join(row.path,name));if(entries.some(name=>!then.has(name)&&!manifestFreeV1(join(row.path,name),name)))return false;continue;}
   }'''),
], SC: [
 ('''          "stat": {"type": "string", "pattern": "^[0-9]+(?::[0-9]+){4}$"}
        }''', '''          "stat": {"type": "string", "pattern": "^[0-9]+(?::[0-9]+){4}$"},
          "entries": {"type": "array", "maxItems": 65536, "items": {"type": "string", "minLength": 1}, "description": "A listing's sub-directory names and its manifest, in the order its stamp hashed them, so a replay can tell appeared and vanished entries apart. / Die Namen der Unterverzeichnisse und des Manifests einer Auflistung in der gehashten Reihenfolge, damit eine Wiederholung erschienene und verschwundene Einträge unterscheiden kann."}
        }'''),
 ('''        "listedFile": "Cargo.toml",''', '''        "listedFile": "Cargo.toml",
        "skippedNames": ["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "🗑️generated"],
        "maximumAppearedEntries": 65536,'''),
]}
bad = 0
for rel, pairs in EDITS.items():
    path = os.path.join(root, rel); text = open(path, encoding="utf8").read(); out = text
    for old, new in pairs:
        if new in out: continue
        if out.count(old) != 1: print("ANCHOR", out.count(old), rel[-40:], old[:80].replace("\n", " ")); bad += 1; continue
        out = out.replace(old, new)
    if verb == "apply" and out != text:
        st = os.stat(path); open(path, "w", encoding="utf8").write(out); os.chmod(path, st.st_mode & 0o7777); print("edited", rel[-60:])
sys.exit(1 if bad else 0)
