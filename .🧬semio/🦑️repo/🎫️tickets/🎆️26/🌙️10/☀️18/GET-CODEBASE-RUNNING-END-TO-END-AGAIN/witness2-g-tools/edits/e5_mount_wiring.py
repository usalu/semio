#!/usr/bin/env python3
"""🗻️ e5: the change journal refuses on remote file systems: refusal `remote-file-system` (schema, EN/DE texts, type), client gate with an injected file-system question, library glue with the real host ports."""
import sys
root = sys.argv[1]
N = root + "/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/"
J = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/"
def edit(path, pairs):
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (path[-40:], text.count(old), old[:70])
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
edit(N + "🧬️schema/🔣️.json", [
 ('"fence-timeout", "overflow", "cancelled"],', '"fence-timeout", "overflow", "remote-file-system", "cancelled"],'),
 ('overflow: more changes than a consumer can use; cancelled."', 'overflow: more changes than a consumer can use; remote-file-system: the workspace lies on a network, virtual or FUSE file system that does not report edits made on its other side (see the mount module); cancelled."'),
 ('''            "overflow": "More than {count} files changed since the last run; the tree is walked.",
''', '''            "overflow": "More than {count} files changed since the last run; the tree is walked.",
            "remote-file-system": "Cannot vouch: {path} lies on a {detail} file system that does not report edits made on its other side; the tree is hashed.",
'''),
 ('''            "overflow": "Mehr als {count} Dateien haben sich seit dem letzten Lauf geändert; der Baum wird durchlaufen.",
''', '''            "overflow": "Mehr als {count} Dateien haben sich seit dem letzten Lauf geändert; der Baum wird durchlaufen.",
            "remote-file-system": "Kann nicht einstehen: {path} liegt auf einem Dateisystem vom Typ {detail}, das Änderungen von seiner anderen Seite nicht meldet; der Baum wird per Hash geprüft.",
'''),
])
edit(N + "🟦️.ts", [('"fence-timeout"|"overflow"|"cancelled";', '"fence-timeout"|"overflow"|"remote-file-system"|"cancelled";')])
edit(N + "🧭️client/🟦️.ts", [
 ('''import {randomBytes} from "node:crypto";''', '''import {randomBytes} from "node:crypto";'''),
 (''' start?:()=>Promise<void>;now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;
}>;''', ''' start?:()=>Promise<void>;now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;
 /** The question "does the workspace lie on a file system that does not report far-side edits?", asked once per client; the answer refuses every call with `remote-file-system`. */
 fileSystem?:(root:string)=>Promise<Readonly<{type:string;mount:string}>|undefined>;
}>;'''),
 (''' let counter=0,touched=0;
''', ''' let counter=0,touched=0,remote:Promise<Readonly<{type:string;mount:string}>|undefined>|undefined;
'''),
 ('''  signal?.throwIfAborted();
  let state=live();
  if(!state){''', '''  signal?.throwIfAborted();
  if(options.fileSystem){const found=await(remote??=options.fileSystem(root).catch(()=>undefined));signal?.throwIfAborted();if(found)return refuse("remote-file-system",{path:found.mount,detail:found.type});}
  let state=live();
  if(!state){'''),
])
edit(J + "🟦️.ts", [
 ('''import {spawn} from "node:child_process";''', '''import {execFile,spawn} from "node:child_process";'''),
 ('''import {existsSync,mkdirSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";''', '''import {existsSync,mkdirSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";'''),
 ('''import {createChangeJournalClientV1,type ChangeJournalClientV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";''', '''import {createChangeJournalClientV1,type ChangeJournalClientV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {CHANGE_FILE_SYSTEM_POLICY_V1,inspectChangeFileSystemV1,type ChangeFileSystemPortsV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🗻️mount/🟦️.ts";'''),
 ('''/** 🔌️ The client every consumer uses''', '''/** 🗻️ The host ports of the file-system question: the platform, a text file and one program's output (absent where either fails or takes longer than the declared bound). */
export const CHANGE_HOST_PORTS_V1:ChangeFileSystemPortsV1=Object.freeze({
 platform:process.platform,
 readText:async path=>{try{return await readFile(path,"utf8");}catch{return undefined;}},
 run:(program,args)=>new Promise<string|undefined>(resolve=>{try{execFile(program,[...args],{encoding:"utf8",timeout:CHANGE_FILE_SYSTEM_POLICY_V1.probeTimeoutMilliseconds,windowsHide:true,maxBuffer:16*1024*1024},(error,stdout)=>resolve(error?undefined:stdout));}catch{resolve(undefined);}}),
});

/** 🔌️ The client every consumer uses'''),
 ('''return createChangeJournalClientV1({directory,root:canonical,environment,start:()=>startChangeJournalServiceV1({directory,root:canonical,environment})});''', '''return createChangeJournalClientV1({directory,root:canonical,environment,start:()=>startChangeJournalServiceV1({directory,root:canonical,environment}),fileSystem:root=>inspectChangeFileSystemV1((()=>{try{return realpathSync(root);}catch{return root;}})(),CHANGE_HOST_PORTS_V1)});'''),
])
