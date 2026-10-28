#!/usr/bin/env python3
"""🪪️ e8: the journal service watches the directory it started on: `root-gone` compares the identity (device, inode, birth time) of the root, not only its path."""
import sys
root = sys.argv[1]
J = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/"
def edit(path, pairs):
    text = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (path[-40:], text.count(old), old[:70])
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
edit(J + "🟦️.ts", [
 ('''const excludedBy=(exclude''', '''/** 🪪️ The identity of the directory behind a path (device, inode, birth time), or nothing where there is none: watches follow the directory they were set on, not the path. */
const directoryIdentityV1=(path:string):string|undefined=>{try{const facts=statSync(path,{bigint:true});return facts.dev+":"+facts.ino+":"+facts.birthtimeNs;}catch{return undefined;}};

const excludedBy=(exclude'''),
 ('''const epoch=randomBytes(8).toString("hex"),instance=randomBytes(4).toString("hex"),startedAt=now(),excluded=excludedBy(exclude);''', '''const epoch=randomBytes(8).toString("hex"),instance=randomBytes(4).toString("hex"),startedAt=now(),excluded=excludedBy(exclude),rootIdentity=directoryIdentityV1(root);'''),
 ('''  if(!existsSync(root)){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}
  if(!existsSync(directory)){void stop("directory-gone");return;}
  if(!existsSync(root)){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}''', '''  const here=directoryIdentityV1(root);
  if(here===undefined||here!==rootIdentity){void stop("root-gone").then(()=>rmSync(directory,{recursive:true,force:true}));return;}
  if(!existsSync(directory)){void stop("directory-gone");return;}'''),
])
edit(J + "🧪️tests/🟦️.ts", [
 ('''test("a service stops itself when its journal directory vanished''', '''test("a service whose workspace was swapped for another directory at the same path stops: its watches follow the old directory, not the path",async()=>{
 const{root,directory}=workspace(),tight={...policy,heartbeatMilliseconds:20};
 const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 await sleep(60);
 renameSync(root,root+".old");mkdirSync(root);
 expect(existsSync(root)).toBe(true);
 expect(await service.closed).toBe("root-gone");
 await sleep(100);
 expect(existsSync(directory)).toBe(false);
 rmSync(root+".old",{recursive:true,force:true});
});

test("a service stops itself when its journal directory vanished'''),
 ('''import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";''', '''import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";'''),
])
