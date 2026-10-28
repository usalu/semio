"""[lane diskproof] anchored edits of every staged batch: `edits.py <batch> <root>` prints the new bytes' digests, `--write` applies them to <root>; exit 2 when an anchor is gone or occurs twice (never guesses). `make.py` calls `build(batch, root)` against the LIVE root, so a rebase is a rebuild."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "src")
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
LEDGER = LIB + "⚡️caching/💾️storage/🧾️ledger/"
GC = LIB + "🏗️build/🧹️cache-gc/"
MEMBERS = LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/"
ACCEPT = LIB + "🏗️build/📏️incremental/🏎️bench/🛂️acceptance/"
REPO = "🧰️framework/🛍️products/🦑️repo/"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"

class Drift(Exception):
    pass

def read(root, rel):
    with open(os.path.join(root, rel), encoding="utf8", newline="") as handle:
        return handle.read()

def sub(text, old, new, rel):
    if text.count(old) != 1:
        raise Drift(f"anchor {'missing' if text.count(old) == 0 else 'ambiguous'} in {rel[-70:]}: {old[:70]!r}")
    return text.replace(old, new)

def src(name):
    with open(os.path.join(SRC, name), encoding="utf8", newline="") as handle:
        return handle.read()

def closurefix(root):
    out = {}
    rel = MEMBERS + "🔎️scan/🟦️.ts"
    out[rel] = sub(read(root, rel), '''/** 💀️ Whether a read refusal says the leaf can never be decoded: its observation blocks are gone, so no reader can follow it to a closure. */
const deadLeafV1=(error:unknown)=>(error as {closureReadFailure?:{worker?:{failure?:{message?:unknown}}}}|undefined)?.closureReadFailure?.worker?.failure?.message==="Authenticated observation closure block required";''', '''/** 💀️ Whether a read refusal says the leaf can never be decoded, so no reader can follow it to a closure: its observation blocks are gone, or its payload is no complete compressed stream (the child's inflation check failed with a truncated or corrupt stream error, which stays so; a memory or limit error says nothing about the bytes; a leaf is only judged once it is older than the grace). */
const deadLeafV1=(error:unknown)=>{const worker=(error as {closureReadFailure?:{worker?:{failure?:{message?:unknown;code?:unknown};closurePayloadRefusal?:{check?:unknown}|null}}}|undefined)?.closureReadFailure?.worker,code=worker?.failure?.code;return worker?.failure?.message==="Authenticated observation closure block required"||((code==="Z_BUF_ERROR"||code==="Z_DATA_ERROR")&&worker?.closurePayloadRefusal?.check==="inflation");};''', rel)
    rel = MEMBERS + "🧪️tests/🧹️supersede/🟦️.ts"
    text = read(root, rel)
    text = sub(text, 'import{mkdirSync,mkdtempSync,readdirSync,', 'import{lstatSync,mkdirSync,mkdtempSync,readdirSync,', rel)
    text = sub(text, 'import{fileURLToPath}from"node:url";', 'import{fileURLToPath,pathToFileURL}from"node:url";', rel)
    text = sub(text, 'import{brotliCompressSync}from"node:zlib";', 'import{spawnSync}from"node:child_process";\nimport{brotliCompressSync}from"node:zlib";', rel)
    anchor = 'test("an observation block is never read on its own, its record header carries the references",async()=>{'
    out[rel] = sub(text, anchor, src("closurefix.test-block.ts") + anchor, rel)
    return out

def futurestamp(root):
    out = {}
    rel = LEDGER + "🧬️schema/🔣️.json"
    out[rel] = sub(read(root, rel), '"overBudgetIntervalMs": 1800000\n', '"overBudgetIntervalMs": 1800000,\n        "futureSkewMs": 86400000\n', rel)
    rel = LEDGER + "🟦️.ts"
    out[rel] = sub(read(root, rel), '/** 🧱️ One independently removable child of an area,', '''/** ⏳️ When a file was last used: its newest access or modification time that is not dated beyond `futureSkewMs` ahead of now (Bazel dates its installed files ten years ahead, and a skewed clock does the same), and the inode change time, which no tool can set, when every other time lies in the future; a future time never makes a file look recently used for ever. */
export function cacheRecencyMsV1(times:Readonly<{mtimeMs:number;atimeMs:number;ctimeMs:number}>,nowMs:number,skewMs:number=CACHE_RUNTIME_V1.futureSkewMs):number{
 const limit=nowMs+skewMs,past=[times.mtimeMs,times.atimeMs].filter(time=>time<=limit);
 return past.length?Math.max(...past):Math.min(times.ctimeMs,nowMs);
}

/** 🧱️ One independently removable child of an area,''', rel)
    rel = GC + "🏃️runner/🟦️.ts"
    text = read(root, rel)
    text = sub(text, 'formatCacheReasonV1,planCacheCollectionV1,', 'cacheRecencyMsV1,formatCacheReasonV1,planCacheCollectionV1,', rel)
    text = sub(text, 'recencyMs:Math.max(first.mtimeMs,first.atimeMs),entries:1', 'recencyMs:cacheRecencyMsV1(first,ports.now()),entries:1', rel)
    out[rel] = sub(text, 'files++;recency=Math.max(recency,info.mtimeMs,info.atimeMs);', 'files++;recency=Math.max(recency,cacheRecencyMsV1(info,ports.now()));', rel)
    rel = GC + "👯️dedupe/🟦️.ts"
    text = read(root, rel)
    text = sub(text, 'import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1}', 'import {cacheRecencyMsV1,cacheTextV1,formatCacheBytesV1,type CacheLanguageV1}', rel)
    out[rel] = sub(text, 'recencyMs:Math.max(info.mtimeMs,info.atimeMs),mtimeMs', 'recencyMs:cacheRecencyMsV1(info,ports.now()),mtimeMs', rel)
    out[GC + "🧪️tests/⏳️future/🟦️.ts"] = src("future.test.ts")
    return out

def lawregex(root):
    out = {}
    rel = LEDGER + "🧪️tests/⚖️law/🟦️.ts"
    out[rel] = sub(read(root, rel), r'''(?:\/([^"'`\s,)}*{<\\$\[]+)|''', r'''(?:\/([^"'`\s,)}*{<\\$\[(|]+)|''', rel)
    rel = LEDGER + "🧪️tests/⚖️law/🌐️native/🟦️.ts"
    out[rel] = sub(read(root, rel), '''expect(files.flatMap(file=>scan(file,readFileSync(join(root,file),"utf8")))).toEqual([]);
});

test("the scan itself flags undeclared and retired native homes''', '''expect(files.flatMap(file=>scan(file,readFileSync(join(root,file),"utf8")))).toEqual([]);
},120000);

test("the scan itself flags undeclared and retired native homes''', rel)
    return out

def acceptjournal(root):
    out = {}
    rel = LEDGER + "🔣️policy.json"
    out[rel] = sub(read(root, rel), '''        "en": "Benchmark outputs.",
        "de": "Benchmark-Ausgaben."
      }
    },
''', '''        "en": "Benchmark outputs.",
        "de": "Benchmark-Ausgaben."
      }
    },
    {
      "id": "acceptance",
      "root": "repository",
      "kind": "tree",
      "directory": "acceptance",
      "budget": {
        "bytes": 16777216
      },
      "collect": {
        "unusedAgeMs": null,
        "quietMs": 3600000,
        "custody": "quiet"
      },
      "writers": [
        "''' + ACCEPT + '''⌨️command/🟦️.ts"
      ],
      "purpose": {
        "en": "Edit journal of the build acceptance run: the original bytes of every file it edited, until a run has restored them.",
        "de": "Änderungsjournal der Build-Abnahme: die Originalbytes jeder von ihr geänderten Datei, bis ein Lauf sie zurückgesetzt hat."
      }
    },
''', rel)
    rel = ACCEPT + "🧬️schema/🔣️.json"
    out[rel] = sub(read(root, rel), '"journalDirectory": ".cache/acceptance",', '"journalDirectory": ".🧬semio/🦑️repo/⚡️cache/acceptance",', rel)
    rel = REPO + "README.md"
    text = read(root, rel)
    text = sub(text, "edits are journaled under `.cache/acceptance` and a killed run", "edits are journaled under `.🧬semio/🦑️repo/⚡️cache/acceptance` and a killed run", rel)
    out[rel] = sub(text, "Änderungen werden unter `.cache/acceptance` protokolliert", "Änderungen werden unter `.🧬semio/🦑️repo/⚡️cache/acceptance` protokolliert", rel)
    return out

def mirrordrop(root):
    rel = TICKET + "mirror-tools/mirror.py"
    text = read(root, rel)
    text = sub(text, '''def do_drop(name, wait=False):
    root = root_of(name)
    if os.path.islink(root):
        os.unlink(root)
    elif os.path.isdir(root):
''', src("mirrordrop.py.block") + '''def do_drop(name, wait=False, force=False):
    root = root_of(name)
    if os.path.islink(root):
        os.unlink(root)
    elif os.path.isdir(root):
        detach_below(root, force)
''', rel)
    text = sub(text, "        do_drop(name, args.sync)\n", "        do_drop(name, args.sync, args.force)\n", rel)
    text = sub(text, '''    p.add_argument("--prefix", help="drop every mirror whose name starts with this lane prefix")
    p.add_argument("--force", action="store_true")''', '''    p.add_argument("--prefix", help="drop every mirror whose name starts with this lane prefix")
    p.add_argument("--force", action="store_true", help="detach images below the mirror even when they are busy; with --nm: also while other mirrors exist")''', rel)
    return {rel: text}

BATCHES = {"closurefix": closurefix, "futurestamp": futurestamp, "lawregex": lawregex, "acceptjournal": acceptjournal, "mirrordrop": mirrordrop}

def build(batch, root):
    return BATCHES[batch](root)

if __name__ == "__main__":
    batch, root = sys.argv[1], os.path.abspath(sys.argv[2])
    try:
        result = build(batch, root)
    except Drift as error:
        print("DRIFT", error)
        sys.exit(2)
    for rel, text in result.items():
        if "--write" in sys.argv:
            os.makedirs(os.path.dirname(os.path.join(root, rel)), exist_ok=True)
            with open(os.path.join(root, rel), "w", encoding="utf8", newline="") as handle:
                handle.write(text)
        print("ok", rel[-80:], len(text))
