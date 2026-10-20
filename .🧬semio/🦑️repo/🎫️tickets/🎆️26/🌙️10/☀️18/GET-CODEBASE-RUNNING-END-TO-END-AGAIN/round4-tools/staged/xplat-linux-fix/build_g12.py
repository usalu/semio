import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, read_abs
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEP = L + "/⚡️caching/🚀️bootstrap/📦️dependencies"
GLUE, STORE = DEP + "/🚦️concurrent/💾️storage/🟦️.ts", DEP + "/💾️storage/🟦️.ts"
FACADE = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"
glue = read_abs(os.path.join(C, "conc-deps-tools/staged/g2/glue.ts"))
rel = os.path.relpath("/Users/ueli/Documents/semio/" + FACADE, os.path.dirname("/Users/ueli/Documents/semio/" + GLUE))
edits = []
imp = [l for l in glue.split("\n") if l.startswith("import {closeSync") and "statfsSync" in l][0]
edits.append((GLUE, imp, imp.replace("statfsSync,", "") + "\nimport {nativeStatfsV1} from \"" + rel + "\";"))
old_fraction = glue[glue.index("/** 🌡️ Reports the free fraction"):glue.index("/** 🔗️ Tells whether any binding")]
new_fraction = '''/** 🌡️ Reports the free fraction of the volume that holds the generations, measured now: the mounted image on macOS, the declared envelope minus the measured tree on a directory partition, or 1 when the platform cannot tell. */
export async function concurrentFreeFractionV1(path:string):Promise<number>{
 try{const facts=await nativeStatfsV1(path,{fresh:true});return facts.blocks>0?facts.bavail/facts.blocks:1;}catch{return 1;}
}
/** 🌡️ Reports the bytes a writer can still use on that volume, measured now, so a before-and-after difference is the size of what an install built; a platform that cannot tell reports no limit. */
export async function concurrentFreeBytesV1(path:string):Promise<number>{
 try{const facts=await nativeStatfsV1(path,{fresh:true});return facts.bavail*facts.bsize;}catch{return Number.MAX_SAFE_INTEGER;}
}
'''
edits.append((GLUE, old_fraction, new_fraction))
edits.append((STORE, 'freeBytes:concurrentFreeBytesV1(store.root)});', 'freeBytes:await concurrentFreeBytesV1(store.root)});', 2))
edits.append((STORE, 'freeFraction:concurrentFreeFractionV1(store.root),freeBytes:concurrentFreeBytesV1(store.root),neededBytes', 'freeFraction:await concurrentFreeFractionV1(store.root),freeBytes:await concurrentFreeBytesV1(store.root),neededBytes'))
edits.append((STORE, 'const freeBefore=concurrentFreeBytesV1(store.root);', 'const freeBefore=await concurrentFreeBytesV1(store.root);'))
edits.append((STORE, 'Math.max(0,freeBefore-concurrentFreeBytesV1(store.root))', 'Math.max(0,freeBefore-await concurrentFreeBytesV1(store.root))'))
emit("g12", "dependency generations measure free space through the virtual statfs, so the image-fullness admission and the built-bytes accounting work on directory partitions", [GLUE, STORE], edits, [], os.path.join(HERE, "groups"))
print("g12 written", len(edits))
