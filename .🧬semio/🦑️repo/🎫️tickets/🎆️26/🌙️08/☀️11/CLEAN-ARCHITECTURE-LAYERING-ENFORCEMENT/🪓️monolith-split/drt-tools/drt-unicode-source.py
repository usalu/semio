p="🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🟦️.ts"
t=open(p,encoding="utf8").read()
def rep(old,new):
    global t
    assert t.count(old)==1,(t.count(old),old[:60])
    t=t.replace(old,new)
rep('export const unicodeSourceUrl=new URL("./📦️data/🔢️properties.bin",import.meta.url);',
'/** 🧩️ The original Unicode record in byte order: the header and ranges (`📦️data/🏷️properties`), then the decompositions, compositions, scalars, mirrors and brackets (`📦️data/🔁️mappings`); the Rust crates join the same two parts. */\nexport const unicodeSourceUrls=[new URL("./📦️data/🏷️properties/🔢️properties.bin",import.meta.url),new URL("./📦️data/🔁️mappings/🔢️mappings.bin",import.meta.url)] as const;')
old=t.split("\n")
line=[l for l in old if l.startswith("export async function loadBuiltinUnicodeSource")]
assert len(line)==1
new='export async function loadBuiltinUnicodeSource(provider:FontByteProvider=fetchFontBytes,signal?:AbortSignal):Promise<Uint8Array>{const cancelled=()=>new DOMException("Unicode source loading cancelled","AbortError"),parts:Uint8Array[]=[];let total=0;if(signal?.aborted)throw cancelled();for(const[index,url]of unicodeSourceUrls.entries()){const part=await provider({family:"Unicode",id:"16.0.0/"+index,url},signal);if(signal?.aborted)throw cancelled();total+=part.length;if(total>16777216)throw Error("Unicode source byte authority exceeded");parts.push(part);}const bytes=new Uint8Array(total);let at=0;for(const part of parts){bytes.set(part,at);at+=part.length;}return bytes;}'
rep(line[0],new)
open(p,"w",encoding="utf8").write(t)
