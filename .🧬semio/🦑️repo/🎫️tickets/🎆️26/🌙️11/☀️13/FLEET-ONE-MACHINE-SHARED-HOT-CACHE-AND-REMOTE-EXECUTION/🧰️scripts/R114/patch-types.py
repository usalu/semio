import sys
p="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.d.mts"
s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:60],s.count(old))
    s=s.replace(old,new)
rep('prepare?:()=>boolean|PromiseLike<boolean>;fleet?:string}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;','prepare?:()=>boolean|PromiseLike<boolean>;fleet?:string;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>}>,compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;\n/** 🤝️ The verdict of one process on one stored project record: `holds`, `stale`, `other` (a record of another key) or `missing`, with the digest of the header line it judged. */\nexport function verifyStoredProjectRecordV1(options:Readonly<{cachePath:string;key:unknown}>):Promise<{readonly verdict:"holds"|"stale"|"other"|"missing";readonly headerDigest?:string;readonly rows?:number;readonly verifier:number}>;')
rep('export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>}>','export function observeInference<T>(options:Readonly<{cachePath:string;key:unknown;reusable?:(value:unknown)=>boolean|PromiseLike<boolean>;verdict?:(asked:{readonly headerDigest:string})=>PromiseLike<{readonly verdict?:string;readonly headerDigest?:string}|undefined>}>')
open(sys.argv[1],"w").write(s)
