"""Builds replace/<manifest module> from the live module: the sharded, hash-chained persisted form, the vouches and the entry bookkeeping. usage: make_manifest.py [--root R]"""
import os,sys,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
ROOT=sys.argv[sys.argv.index("--root")+1] if "--root" in sys.argv else "/Users/Shared/semio"
rel=edits.MANIFEST
raw=open(ROOT+"/"+rel,"rb").read()
if hashlib.sha256(raw).hexdigest()!=edits.MANIFEST_BEFORE_SHA:sys.exit("the live manifest module is not the one this was built from: re-merge")
text=raw.decode("utf8")
def rep(old,new):
    global text
    if text.count(old)!=1:sys.exit("anchor %d times: %r"%(text.count(old),old[:70]))
    text=text.replace(old,new)
rep('import{rowKeyV1}from"../../../🕰️generations/🧮️fact-rows/🟨️.mjs";\n','import{createHash}from"node:crypto";\nimport{rowKeyV1}from"../../../🕰️generations/🧮️fact-rows/🟨️.mjs";\n')
rep('outside:new Map()};}','outside:new Map(),revs:new Map(),saved:new Map(),fullAt:0};}')
rep('const addTo=(map,key,number)=>{','''const SHARDS_V1=128;
const shardOfV1=text=>{let hash=2166136261;for(let at=0;at<text.length;at++)hash=Math.imul(hash^text.charCodeAt(at),16777619);return(hash>>>0)%SHARDS_V1;};
const bump=(book,key)=>{book.revs.set(key,(book.revs.get(key)??0)+1);};
const sha=value=>createHash("sha256").update(value).digest("hex");
const addTo=(map,key,number)=>{''')
rep(' for(const path of meta.paths)addTo(book.paths,path,number);\n for(const dir of meta.dirs)addTo(book.dirs,dir,number);\n for(const dir of meta.bare)addTo(book.bare,dir,number);',' for(const path of meta.paths){addTo(book.paths,path,number);bump(book,"i"+shardOfV1(path));}\n for(const dir of meta.dirs){addTo(book.dirs,dir,number);bump(book,"i"+shardOfV1(dir));}\n for(const dir of meta.bare){addTo(book.bare,dir,number);bump(book,"i"+shardOfV1(dir));}')
rep('export function recordEntryV1(book,name,{digest,fact,length,ids}){book.entries.set(name,{digest,fact,length,bits:bitsOfV1(ids.map(id=>partNumberV1(book,id)))});}','''export function recordEntryV1(book,name,{digest,fact,length,ids,vouch}){book.entries.set(name,{digest,fact,length,vouch,bits:bitsOfV1(ids.map(id=>partNumberV1(book,id)))});bump(book,"e"+shardOfV1(name));}

/** 🗑️ Forgets the answer of one closure; whether there was one. */
export function forgetEntryV1(book,name){if(!book.entries.delete(name))return false;bump(book,"e"+shardOfV1(name));return true;}''')
cut=text.index("const pairs=map=>[...map];")
head=text[:cut]
tail=open(HERE+"/manifest-tail.mjs",encoding="utf8").read()
out=head+tail
os.makedirs(HERE+"/replace/"+os.path.dirname(rel),exist_ok=True)
open(HERE+"/replace/"+rel,"w",encoding="utf8").write(out)
print("replace/ built:",len(out),"bytes")
