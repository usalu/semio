#!/usr/bin/env python3
"""🍞️ Applies the loader edits of the seal2-g batch to <root>'s 🍞️bun/🟦️.ts (every anchor must occur exactly once; idempotent: refuses when already patched).
usage: python3 -I patch-loader.py <root>"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
root = sys.argv[1]
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts"
path = os.path.join(root, rel)
text = open(path, encoding="utf-8").read()
if "admitCapturedImagesV1" in text:
    sys.exit("already patched")
functions = open(os.path.join(HERE, "src", "loader-functions.ts"), encoding="utf-8").read().rstrip("\n") + "\n\n"
def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:80])
    text = text.replace(old, new)
once("export /** 🏃️ Runs only checksum-bound captured modules at their original Source URLs. */", functions + "export /** 🏃️ Runs only checksum-bound captured modules at their original Source URLs. */")
old_read = "const chunks=[],chunk=Buffer.alloc(${policy.readChunkBytes});let size=0,used=0;for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,${policy.maximumSnapshotBytes}-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>${policy.maximumSnapshotBytes}-size)throw Error('Bounded immutable implementation snapshot required');size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}const encoded=Buffer.concat(chunks,size);check();"
new_read = "const facts=fs.fstatSync(3);let encoded;if(facts.isFile()){if(facts.size>${policy.maximumSnapshotBytes})throw Error('Bounded immutable implementation snapshot required');encoded=Buffer.allocUnsafe(facts.size);for(let used=0;used<encoded.length;){check();const count=fs.readSync(3,encoded,used,encoded.length-used,null);if(!count)throw Error('Immutable implementation snapshot changed');used+=count;}if(fs.readSync(3,Buffer.alloc(1),0,1,null))throw Error('Bounded immutable implementation snapshot required');}else{const chunks=[],chunk=Buffer.alloc(${policy.readChunkBytes});let size=0,used=0;for(;;){check();const count=fs.readSync(3,chunk,used,Math.min(chunk.length-used,${policy.maximumSnapshotBytes}-size+1),null);check();if(!count){if(used)chunks.push(Buffer.from(chunk.subarray(0,used)));break;}if(count>${policy.maximumSnapshotBytes}-size)throw Error('Bounded immutable implementation snapshot required');size+=count;used+=count;if(used===chunk.length){chunks.push(Buffer.from(chunk));used=0;}}encoded=Buffer.concat(chunks,size);}check();"
once(old_read, new_read)
once("const value=JSON.parse(encoded.toString('utf8'));${overlay?overlaySource(overlay):''}",
     "const redirectCapturedBuiltinReadsV1=(${redirectCapturedBuiltinReadsV1.toString()}),splitCapturedSnapshotV1=(${splitCapturedSnapshotV1.toString()}),admitCapturedImagesV1=(${admitCapturedImagesV1.toString()});\nconst split=${auxiliary?'splitCapturedSnapshotV1(encoded)':'undefined'},value=JSON.parse(split?split.text:encoded.toString('utf8')),spans=split&&Array.isArray(value?.files)&&split.spans.length===value.files.length?split.spans:undefined;${overlay?overlaySource(overlay):''}")
start = text.index("const images=new Map(),resolutions=new Map();let total=0;for(const row of value.files){")
end_marker = "images.set(name,bytes);resolutions.set(name,row.imports);row.base64='';}"
end = text.index(end_marker, start) + len(end_marker)
text = text[:start] + "const{images,resolutions}=admitCapturedImagesV1(value,encoded,spans,{maximumFileBytes:${policy.maximumFileBytes},maximumSourceBytes:${policy.maximumSourceBytes}},hash,path,check);" + text[end:]
start = text.index("const redirect=(source,commonJs=false)=>{const parser=new Bun.Transpiler({loader:'js',target:'bun'}),targets=new Map([")
end_marker = "return result;};"
end = text.index(end_marker, start) + len(end_marker)
block = text[start:end]
targets = block[block.index("targets=new Map(["):block.index("]),expected=")+2]
text = text[:start] + "const redirectParser=new Bun.Transpiler({loader:'js',target:'bun'}),redirectTargets=" + targets.replace("targets=", "") + ",redirect=(source,commonJs=false)=>redirectCapturedBuiltinReadsV1(source,commonJs,redirectParser,redirectTargets);" + text[end:]
open(path, "w", encoding="utf-8").write(text)
print("patched", path)
