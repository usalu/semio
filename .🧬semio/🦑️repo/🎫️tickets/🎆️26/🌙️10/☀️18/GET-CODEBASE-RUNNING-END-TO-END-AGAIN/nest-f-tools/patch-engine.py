import os, sys, shutil
G = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/nest-f-tools"
def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    base = os.path.join(T, os.path.basename(os.path.dirname(path)) + "-" + "x.orig")
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit("anchor count %d for %r in %s" % (text.count(old), old[:60], path))
        text = text.replace(old, new)
    tmp = path + ".nest-f.tmp"
    open(tmp, "w", encoding="utf-8").write(text)
    os.rename(tmp, path)

mjs = G + "/🟨️.mjs"
edit(mjs, [
 (" async function readIndex(digest){\n",
  " /** 📌️ Reads one sealed generation by the digest of its manifest although a newer pointer replaced it; a retired manifest answers undefined. */\n async function readManifestAt(digest,signal){\n  aborted(signal);requireHex(digest,\"manifest\");const bytes=await fs.read(manifestPath(digest));if(!bytes)return undefined;\n  if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail(\"Generation manifest content differs from its name\",\"GENERATION_CORRUPT\");\n  try{return readGenerationManifestV1(bytes);}catch{return undefined;}\n }\n async function readIndex(digest){\n"),
 ("return Object.freeze({dropIndexes:()=>indexCache.clear(),read,readIndex,", "return Object.freeze({dropIndexes:()=>indexCache.clear(),read,readManifestAt,readIndex,"),
])
print("engine patched")
