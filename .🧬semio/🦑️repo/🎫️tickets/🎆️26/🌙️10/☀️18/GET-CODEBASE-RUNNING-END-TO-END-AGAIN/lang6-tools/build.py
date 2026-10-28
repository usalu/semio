#!/usr/bin/env python3
"""🗣️ Builds the staged batch `rest` (lane lang6) from the CURRENT bytes of a root: the 9 repository language-law findings that no other batch owns become machine tokens (internal refusals) or the module's own `both(english, german)` pairs (the bench acceptance command), one literal at a time, each anchored and unique.
usage: python3 -I build.py [--root DIR]   (default root: the live checkout; re-run right before the gate lands the batch: it re-records base/new hashes from the live bytes)
Writes `staged/rest/{manifest.json,files/,base/}`; `stage.py` applies it."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "rest")
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else "/Users/ueli/Documents/semio"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build/📏️incremental/🏎️bench/🛂️acceptance"
S = L + "/⚡️caching/💾️storage/🔒️enforcement"
def both(en, de): return ("both(" + en + "," + de + ")")
EDITS = {
    S + "/📦️budget/📥️opening/🏪️store/🟦️.ts": [
        ('throw Error("Exact issued cache owner required");', 'throw Error("exact-issued-cache-owner-required");'),
        ('throw Error("Original cache owner frame required");', 'throw Error("original-cache-owner-frame-required");'),
    ],
    S + "/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts": [
        ('throw Error("Exact optional graph target bytes required");', 'throw Error("exact-optional-graph-target-bytes-required");'),
    ],
    S + "/🔌️nx/📦️graph/🟦️.ts": [
        ('if(!parent)throw Error("Actual nx partition owner required");work=withRepositoryCacheStoreOwnerFrameV1', 'if(!parent)throw Error("actual-nx-partition-owner-required");work=withRepositoryCacheStoreOwnerFrameV1'),
    ],
    B + "/⌨️command/🟦️.ts": [
        ('throw Error(`Closed acceptance table required: ${issues.join("; ")}`);', 'throw Error(both(`Closed acceptance table required: ${issues.join("; ")}`,`Geschlossene Abnahmetabelle erforderlich: ${issues.join("; ")}`));'),
        ('const absolute=(path:string)=>{const target=join(root,path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);',
         'const absolute=(path:string)=>{const target=join(root,path);if(target.length>policy.maximumPathLength)throw Error(both(`Path exceeds ${policy.maximumPathLength} characters: ${target}`,`Pfad überschreitet ${policy.maximumPathLength} Zeichen: ${target}`));'),
        ('const target=resolve(path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);await mkdir',
         'const target=resolve(path);if(target.length>policy.maximumPathLength)throw Error(both(`Path exceeds ${policy.maximumPathLength} characters: ${target}`,`Pfad überschreitet ${policy.maximumPathLength} Zeichen: ${target}`));await mkdir'),
    ],
    B + "/🟦️.ts": [
        ('throw Error("Closed acceptance nonce required");', 'throw Error(both("Closed acceptance nonce required","Geschlossene Abnahme-Nonce erforderlich"));'),
    ],
    L + "/🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity/🟦️.ts": [
        ('refuse(Error("Bazel action left before it started"))', 'refuse(Error("bazel-action-left-before-start"))'),
    ],
    L + "/⚖️laws/🧫️test-store-isolation/🟦️.ts": [
        ('throw Error("Test store isolation rules invalid: "+JSON.stringify', 'throw Error("test-store-isolation-rules-invalid:"+JSON.stringify'),
    ],
    L + "/⚖️laws/🧫️test-store-isolation/🧪️tests/🟦️.ts": [
        ('.toThrow("Test store isolation rules invalid")', '.toThrow("test-store-isolation-rules-invalid")'),
    ],
}
digest = lambda data: hashlib.sha256(data).hexdigest()
for sub in ("files", "base"):
    shutil.rmtree(os.path.join(OUT, sub), ignore_errors=True); os.makedirs(os.path.join(OUT, sub))
manifest = {}
for index, (rel, edits) in enumerate(EDITS.items()):
    data = open(os.path.join(ROOT, rel), "rb").read(); text = data.decode("utf8")
    for old, new in edits:
        if text.count(old) != 1: sys.exit("anchor count %d != 1 in %s: %s" % (text.count(old), rel[-50:], old[:70]))
        text = text.replace(old, new)
    name = "%02d%s" % (index, os.path.splitext(rel)[1])
    open(os.path.join(OUT, "base", name), "wb").write(data)
    open(os.path.join(OUT, "files", name), "wb").write(text.encode("utf8"))
    manifest[rel] = {"file": name, "base": digest(data), "new": digest(text.encode("utf8"))}
open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8").write(json.dumps(manifest, ensure_ascii=False, indent=1))
print("built", len(manifest), "entries from", ROOT)
