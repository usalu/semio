#!/usr/bin/env python3
"""Anchored edits of the journal for the compaction landing (a publisher touches a blob that exists and stores it again when the touch finds it gone). Usage: patch-journal.py <module root>."""
import sys, os
path = os.path.join(sys.argv[1], "📒️journal/🟦️.ts")
s = open(path, encoding="utf8").read()
def rep(old, new):
    global s
    assert s.count(old) == 1, f"anchor occurs {s.count(old)} times: {old[:80]!r}"
    s = s.replace(old, new)
rep('''  if(await exists(target)&&await intact(target,known.sha256)){const now=new Date();await utimes(target,now,now).catch(()=>{});return {...known,stable:true,created:false};}''',
    '''  if(await exists(target)&&await intact(target,known.sha256)&&await touchBlobV1(target))return {...known,stable:true,created:false};''')
rep('''  if((present??await exists(target))&&await intact(target,copied.sha256)){await unlink(temporary).catch(()=>{});const now=new Date();await utimes(target,now,now).catch(()=>{});return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:false};}''',
    '''  if((present??await exists(target))&&await intact(target,copied.sha256)&&await touchBlobV1(target)){await unlink(temporary).catch(()=>{});return {sha256:copied.sha256,bytes:copied.bytes,stable:true,created:false};}
  if(present===true){const handle=await open(temporary,"r+");try{await handle.sync();}finally{await handle.close();}}''')
rep('/** 🛢️ Stores a file as an immutable blob in one pass', '''/** 🤚️ Marks an existing blob as in use by setting its modification time to now and tells whether it was still there: a blob reclaim moves unreferenced blobs aside and puts one back when its modification time changed, so a publisher that finds its blob already stored must touch it before it appends the event, and must store the blob again when the touch finds it gone. */
export async function touchBlobV1(target:string):Promise<boolean>{
 const now=new Date();
 try{await utimes(target,now,now);return true;}catch(error){return !missing(error);}
}

/** 🛢️ Stores a file as an immutable blob in one pass''')
open(path, "w", encoding="utf8").write(s)
print("journal patched")
