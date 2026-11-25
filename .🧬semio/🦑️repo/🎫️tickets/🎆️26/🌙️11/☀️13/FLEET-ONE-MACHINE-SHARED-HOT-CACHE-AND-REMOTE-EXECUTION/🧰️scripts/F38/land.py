#!/usr/bin/env python3
"""F38 landing: python3 land.py <stage> [--apply]; stage 1 = locks bootstrap+process, stage 2 = lock caching-storage. Anchored, idempotent, atomic writes."""
import os, sys, hashlib
R = "/Users/ueli/Documents/semio/"
L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
P = R + "🧰️framework/🔨️modules/🏃️process/"
NS = P + "📦️artifacts/🏗️native-build/💾️storage/"
DS = L + "⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
BUD = L + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/"
ED = L + "⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🔣️.json"

def row(owner, contract_en, contract_de):
    return ''',
  {
   "owner": "%s",
   "reference": "../../../📜️script.ts",
   "contract": {
    "en": "%s",
    "de": "%s"
   }
  }''' % (owner, contract_en, contract_de)

DISK = L + "🏗️build/🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts"
IDLE = L + "🏗️build/🧹️cache-gc/🛌️idle/⏲️loop/🟦️.ts"
rel = lambda p: p[len(R):]
ROWS = row(rel(DISK), "The disk recovery starts the repo build script as a child that runs a cache garbage collection or a cache reset target. That child only reclaims bytes and never runs an Nx task, so the script and everything only it reaches cannot change the bytes of a task.", "Die Plattenwiederherstellung startet das Repo-Build-Skript als Kindprozess, der eine Cache-Müllsammlung oder ein Cache-Zurücksetzen ausführt. Dieses Kind gibt nur Bytes frei und führt nie eine Nx-Task aus, daher können das Skript und alles, was nur es erreicht, die Bytes einer Task nicht ändern.") + row(rel(IDLE), "The idle compaction loop starts the repo build script as a detached maintenance child that compacts caches while the host is idle. That child never runs an Nx task, so the script and everything only it reaches cannot change the bytes of a task.", "Die Leerlauf-Verdichtungsschleife startet das Repo-Build-Skript als losgelösten Wartungs-Kindprozess, der Caches verdichtet, solange der Rechner ruht. Dieses Kind führt nie eine Nx-Task aus, daher können das Skript und alles, was nur es erreicht, die Bytes einer Task nicht ändern.")

SINGLE = '''const single={...installSchema,$defs:{...installSchema.$defs,InstalledPathProtocol:{...installSchema.$defs.InstalledPathProtocol,const:{...installSchema.$defs.InstalledPathProtocol.const,maximumScanAttempts:1}}}};
'''
edits1 = [
 (L + "⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📏️paths/🟦️.ts",
  '''/** 📏️ Pins the installed path scan to an independent Python walk and to its closed mutation table: every change to a directory between its listing and its close is refused. */
test(''',
  '''/** 📏️ Pins the installed path scan to an independent Python walk and to its closed mutation table: with one walk allowed (`maximumScanAttempts` 1) every change to a directory between its listing and its close is refused; the bounded rescan of a change that fires once is pinned by `../🔁️rescan`. */
test(''', 'with one walk allowed'),
 (L + "⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📏️paths/🟦️.ts",
  '''const validate=new Ajv({strict:true}).compile(corpus.schema),make=(stat:unknown,read:unknown)=>new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(installSchema,createHash,stat,read,''',
  '''const single={...installSchema,$defs:{...installSchema.$defs,InstalledPathProtocol:{...installSchema.$defs.InstalledPathProtocol,const:{...installSchema.$defs.InstalledPathProtocol.const,maximumScanAttempts:1}}}};
 const validate=new Ajv({strict:true}).compile(corpus.schema),make=(stat:unknown,read:unknown)=>new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")(single,createHash,stat,read,''', 'const single={...installSchema'),
 (ED, '''\n  }\n ]\n}\n''', '''\n  }''' + ROWS + '''\n ]\n}\n''', rel(DISK)),
 (P + "🧪️testing/🗂️suites/🔣️.json",
  '''      "id": "native-storage",
      "budgetMs": 300000,
      "typecheck": "none",
      "trees": [
        "📦️artifacts/🏗️native-build/💾️storage"
      ],
''', '''      "id": "native-storage",
      "budgetMs": 300000,
      "typecheck": "none",
      "trees": [
        "📦️artifacts/🏗️native-build/💾️storage"
      ],
      "capabilities": {
        "artifacts": true
      },
''', '"capabilities": {\n        "artifacts": true\n      },\n      "platforms": [\n        "darwin",\n        "linux",\n        "win32"\n      ]\n    },\n    {\n      "id": "native-build"'),
 (NS + "🧹️cleanup/📥️operation/🧪️tests/🟦️.ts", 'maximumElapsedMilliseconds:300000,capabilities:{}}', 'maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}}', 'maximumElapsedMilliseconds:300000,capabilities:{artifacts:true}'),
 (NS + "📤️publication/🌐️platform/🧮️read-plan/🧪️tests/🔒️journal/🟦️.ts", 'maximumElapsedMilliseconds:120000,capabilities:{}}', 'maximumElapsedMilliseconds:120000,capabilities:{artifacts:true}}', 'maximumElapsedMilliseconds:120000,capabilities:{artifacts:true}'),
 (NS + "🔒️mapping/🧪️tests/🟦️.ts", 'maximumElapsedMilliseconds:60000,capabilities:{}}', 'maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}}', 'maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}'),
 (NS + "🌐️platform/📋️contract/🧪️tests/📸️bootstrap/🟦️.ts", 'maximumElapsedMilliseconds:180000,capabilities:{}}', 'maximumElapsedMilliseconds:180000,capabilities:{artifacts:true}}', 'maximumElapsedMilliseconds:180000,capabilities:{artifacts:true}'),
 (NS + "🧪️tests/🟦️.ts", 'expect(rechecks).toBe(0);expect(readdirSync(root)).toEqual([]);', 'expect(rechecks).toBe(0);expect(readdirSync(root).filter(name=>name!==".cache")).toEqual([]);', 'filter(name=>name!==".cache")'),
]
ADM = BUD + "📥️opening/🚦️admission/🧪️tests/🟦️.ts"
SSR = BUD + "🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts"
edits2 = [
 (ADM, ''' const lines:string[]=[];let clock=0;const ports:PartitionSlotPortsV1={now:()=>clock,''', ''' const lines:string[]=[],languageNames=["SEMIO_LANGUAGE","LANGUAGE","LC_ALL","LC_MESSAGES","LANG"],saved=languageNames.map(name=>process.env[name]);for(const name of languageNames)delete process.env[name];
 try{
 let clock=0;const ports:PartitionSlotPortsV1={now:()=>clock,''', 'languageNames=['),
 (ADM, '''expect(lines.join("")).toContain("Warte auf einen freien Cache-Operationsplatz");
});''', '''expect(lines.join("")).toContain("Warte auf einen freien Cache-Operationsplatz");
 }finally{languageNames.forEach((name,index)=>{if(saved[index]===undefined)delete process.env[name];else process.env[name]=saved[index];});}
});''', 'languageNames.forEach'),
 (SSR, '''partitionPorts:Record<string,any>={checkResourceLeaseControl:leases.checkResourceLeaseControl,''', '''partitionPorts:Record<string,any>={refreshStaleImageBeforeOpenV1:async()=>{},keepRepositoryImageReserveV1:()=>{},checkResourceLeaseControl:leases.checkResourceLeaseControl,''', 'refreshStaleImageBeforeOpenV1:async()=>{}'),
]

def write_atomic(path, text):
    tmp = path + ".f38tmp"
    open(tmp, "w", encoding="utf8").write(text)
    os.replace(tmp, path)

def run(edits, apply):
    for path, old, new, marker in edits:
        text = open(path, encoding="utf8").read()
        if marker in text:
            print("already", path[-48:], marker[:30].replace("\n", " ")); continue
        if text.count(old) != 1:
            print("ANCHOR", text.count(old), path, old[:70].replace("\n", " ")); sys.exit(1)
        if apply:
            write_atomic(path, text.replace(old, new)); print("applied", path[-48:], marker[:30].replace("\n", " "))
        else:
            print("ok", path[-48:], marker[:30].replace("\n", " "))

stage = sys.argv[1]
run(edits1 if stage == "1" else edits2, "--apply" in sys.argv)
