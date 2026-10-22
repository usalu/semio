import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf-8") as handle:
        return handle.read()


class Drift(Exception):
    pass


def replace_once(text, old, new, label):
    if new in text:
        return text
    count = text.count(old)
    if count != 1:
        raise Drift("%s: anchor found %d times" % (label, count))
    return text.replace(old, new)


def apply(text, rows, label):
    for index, (old, new) in enumerate(rows):
        text = replace_once(text, old, new, "%s[%d]" % (label, index))
    return text


CACHING_INDEX = [
    (
        'import { join } from "node:path";\n',
        'import { join } from "node:path";\nimport {requireDeclaredCacheSegmentsV1} from "./💾️storage/🧾️ledger/🟦️.ts";\n',
    ),
    (
        '/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`). */\n'
        'export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {\n'
        '  return join(',
        '/** ⚡️ The single repository cache root shared by every tool, agent and dev (`.🧬semio/🦑️repo/⚡️cache/…`); it refuses every home the cache ledger does not declare. */\n'
        'export function repoCacheDirectory(repoRoot: string, ...segments: string[]): string {\n'
        '  requireDeclaredCacheSegmentsV1(segments);\n'
        '  return join(',
    ),
]

CACHING_POLICY = [
    ('    "cargo": {\n      "budgetBytes": 85899345920,', '    "cargo": {\n      "budgetBytes": 13957595136,'),
    ('      "incremental": {\n        "budgetBytes": 25769803776,', '      "incremental": {\n        "budgetBytes": 6442450944,'),
]

NX_JSON = [
    (
        '  "cacheDirectory": ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts",\n  "analytics": false',
        '  "cacheDirectory": ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts",\n  "maxCacheSize": "3GB",\n  "analytics": false',
    ),
]

PROVIDERS_TEST = [
    (
        'expect(repoCacheDirectory("/repository", "a", "b")).toBe(join(repositoryMetaDirectory("/repository"), "⚡️cache", "a", "b"));',
        'expect(repoCacheDirectory("/repository", "go", "b")).toBe(join(repositoryMetaDirectory("/repository"), "⚡️cache", "go", "b"));',
    ),
]

BUILD_SCRIPT = [
    (
        '    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){',
        '    if(command==="cache"&&["status","gc"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")){\n'
        '      const {runCacheLedgerV1}=await import("./🧹️cache-gc/⌨️command/🟦️.ts");\n'
        '      await runCacheLedgerV1(root,resolved.args.slice(1),invocation,phaseControl);\n'
        '      return;\n'
        '    }\n'
        '    if(command==="cache"&&["serve","remote"].includes(resolved.args[1]??"")){',
    ),
]

OPERATIONS = [
    (
        ' if(args.length!==1||operation!=="status"&&operation!=="clear")throw Error("Expected repo cache status|clear");',
        ' if(operation!=="status"&&operation!=="clear"||args.length!==1&&!(operation==="status"&&args.length===2&&args[1]==="--tasks"))throw Error("Expected repo cache status|clear (status accepts --tasks)");',
    ),
]

CATALOG = [
    (
        '      "name": "--dry-run",\n      "value": "none",\n      "operations": ["run", "run-many", "affected", "exec"],\n      "description": {\n'
        '        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool.",\n'
        '        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich."\n      }',
        '      "name": "--dry-run",\n      "value": "none",\n      "operations": ["run", "run-many", "affected", "exec", "cache"],\n      "description": {\n'
        '        "en": "Print the task graph as a DryRunPlan on stdout and execute nothing; the same for every tool. With cache gc: list what would be removed and remove nothing.",\n'
        '        "de": "Gibt den Task-Graphen als DryRunPlan auf stdout aus und führt nichts aus; für jedes Werkzeug gleich. Mit cache gc: listet auf, was entfernt würde, und entfernt nichts."\n      }',
    ),
    (
        '      "name": "--json",\n      "value": "none",\n      "operations": ["graph", "show", "incremental", "capacity"],\n      "description": { "en": "Print lists as JSON instead of one entry per line.", "de": "Gibt Listen als JSON statt eines Eintrags je Zeile aus." }\n    },\n',
        '      "name": "--json",\n      "value": "none",\n      "operations": ["graph", "show", "incremental", "capacity", "cache"],\n      "description": { "en": "Print lists as JSON instead of one entry per line.", "de": "Gibt Listen als JSON statt eines Eintrags je Zeile aus." }\n    },\n'
        '    {\n      "name": "--area",\n      "value": "required",\n      "valueName": "id[,id]",\n      "operations": ["cache"],\n'
        '      "description": { "en": "cache status|gc: limit the report or the collection to these ledger areas (or retired, undeclared).", "de": "cache status|gc: beschränkt Bericht oder Einsammeln auf diese Bereiche des Verzeichnisses (oder retired, undeclared)." }\n    },\n'
        '    {\n      "name": "--tasks",\n      "value": "none",\n      "operations": ["cache"],\n'
        '      "description": { "en": "cache status: print the selected tool\'s task cache (a CacheStatus) instead of the cache ledger.", "de": "cache status: gibt den Task-Cache des gewählten Werkzeugs (einen CacheStatus) statt der Cache-Bilanz aus." }\n    },\n',
    ),
    (
        '      "synopsis": "cache status|clear|remote status|serve",\n      "summary": {\n'
        '        "en": "Show or clear the selected tool\'s local task cache, show the remote cache contract, or serve a remote cache.",\n'
        '        "de": "Zeigt oder leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."\n      },',
        '      "synopsis": "cache status|gc|clear|remote status|serve",\n      "summary": {\n'
        '        "en": "Show and collect every declared cache home with its budget, clear the selected tool\'s local task cache, show the remote cache contract, or serve a remote cache.",\n'
        '        "de": "Zeigt und sammelt jede deklarierte Cache-Heimat mit ihrem Budget, leert den lokalen Task-Cache des gewählten Werkzeugs, zeigt den Remote-Cache-Vertrag oder stellt einen Remote-Cache bereit."\n      },',
    ),
    (
        '        {\n          "name": "status",\n          "synopsis": "cache status",\n          "summary": {\n'
        '            "en": "Print a CacheStatus: entries, bytes and, for Bazel, the recorded build activity.",\n'
        '            "de": "Gibt einen CacheStatus aus: Einträge, Bytes und, für Bazel, die aufgezeichnete Build-Aktivität."\n          }\n        },\n',
        '        {\n          "name": "status",\n          "synopsis": "cache status [--tasks]",\n          "summary": {\n'
        '            "en": "Print the cache ledger: every partition and area with its use against its budget, the retired roots and undeclared entries still on disk; with --tasks, a CacheStatus of the selected tool instead.",\n'
        '            "de": "Gibt die Cache-Bilanz aus: jede Partition und jeden Bereich mit Nutzung gegen Budget, die noch vorhandenen stillgelegten Wurzeln und nicht deklarierten Einträge; mit --tasks stattdessen einen CacheStatus des gewählten Werkzeugs."\n          }\n        },\n'
        '        {\n          "name": "gc",\n          "synopsis": "cache gc [--dry-run] [--area=<id>]",\n          "summary": {\n'
        '            "en": "Collect the cache: remove retired roots, units unused longer than their area allows and the oldest units beyond an area\'s budget, and entries nobody declared; it never removes a unit that is held by a lease or used recently.",\n'
        '            "de": "Sammelt den Cache ein: entfernt stillgelegte Wurzeln, länger als erlaubt unbenutzte Einheiten, die ältesten Einheiten über dem Budget eines Bereichs und nicht deklarierte Einträge; eine durch Lease gehaltene oder kürzlich benutzte Einheit wird nie entfernt."\n          }\n        },\n',
    ),
    (
        '      "flags": ["--port", "--host", "--directory", "--max-bytes"],',
        '      "flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--tasks"],',
    ),
    (
        '      "examples": [\n        { "command": "repo cache status --tool=bazel",',
        '      "examples": [\n'
        '        { "command": "repo cache status", "description": { "en": "Show every cache partition and area against its budget.", "de": "Zeigt jede Cache-Partition und jeden Bereich gegen sein Budget." } },\n'
        '        { "command": "repo cache gc --dry-run", "description": { "en": "List what the collector would remove.", "de": "Listet auf, was der Einsammler entfernen würde." } },\n'
        '        { "command": "repo cache status --tasks --tool=bazel",',
    ),
    (
        '        "status prints a CacheStatus, clear prints a CacheClear; both stop at the next checkpoint when the caller cancels and publish progress while walking or removing entries.",',
        '        "status prints the cache ledger of schema cache-ledger: every fixed partition, every area inside a partition, every area of the repository and native cache roots with its declared budget, and what is retired or undeclared; gc collects it. Both publish progress, stop at the next checkpoint when the caller cancels, and take no lease for the report; gc takes an area\'s lease exclusively without waiting and never removes a unit whose SQLite lease file is locked, whose queue ticket belongs to a living process or that was used within the quiet time of its area. Areas kept by an owner (Nx at maxCacheSize, Cargo, Bazel, test evidence, the dashboard) are reported, not collected here.",\n'
        '        "status --tasks prints a CacheStatus, clear prints a CacheClear; both stop at the next checkpoint when the caller cancels and publish progress while walking or removing entries.",',
    ),
]

EDITORS = {
    "caching-index": lambda text: apply(text, CACHING_INDEX, "caching-index"),
    "caching-policy": lambda text: apply(text, CACHING_POLICY, "caching-policy"),
    "nx-json": lambda text: apply(text, NX_JSON, "nx-json"),
    "providers-test": lambda text: apply(text, PROVIDERS_TEST, "providers-test"),
    "build-script": lambda text: apply(text, BUILD_SCRIPT, "build-script"),
    "operations": lambda text: apply(text, OPERATIONS, "operations"),
    "catalog": lambda text: apply(text, CATALOG, "catalog"),
}


CATALOG2 = [
    (
        '    {\n      "name": "--tasks",\n      "value": "none",\n      "operations": ["cache"],\n',
        '    {\n      "name": "--deep",\n      "value": "none",\n      "operations": ["cache"],\n'
        '      "description": { "en": "cache status: walk every cache home now (with progress, cancellable) and record the sizes, instead of reading statfs and the size ledger.", "de": "cache status: läuft jetzt jede Cache-Heimat ab (mit Fortschritt, abbrechbar) und erfasst die Größen, statt statfs und das Größenverzeichnis zu lesen." }\n    },\n'
        '    {\n      "name": "--tasks",\n      "value": "none",\n      "operations": ["cache"],\n',
    ),
    (
        '"flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--tasks"],',
        '"flags": ["--port", "--host", "--directory", "--max-bytes", "--json", "--dry-run", "--area", "--deep", "--tasks"],',
    ),
    (
        '"synopsis": "cache status [--tasks]",',
        '"synopsis": "cache status [--deep|--tasks]",',
    ),
    (
        'Print the cache ledger: every partition and area with its use against its budget,',
        'Print the cache ledger without walking any tree (statfs of each partition plus the recorded sizes; --deep walks everything): every partition and area with its use against its budget,',
    ),
    (
        'Gibt die Cache-Bilanz aus: jede Partition',
        'Gibt die Cache-Bilanz ohne Ablaufen eines Baums aus (statfs jeder Partition plus erfasste Größen; --deep läuft alles ab): jede Partition',
    ),
    (
        'take no lease for the report;',
        'take no lease for the report, which reads statfs of each partition and the size ledger that gc and --deep write and walks no tree unless --deep is given;',
    ),
]

EDITORS2 = {"catalog": lambda text: apply(text, CATALOG2, "catalog2")}


BUDGET_POLICY3 = [
    ('  "maximumBytes": 68719476736,', '  "maximumBytes": 70866960384,'),
    (
        '    "graph": {\n      "directory": ".cache/b/g",\n      "capacityBytes": 4294967296,\n      "maximumLogicalBytes": 4293918720\n    },',
        '    "graph": {\n      "directory": ".cache/b/g",\n      "capacityBytes": 6442450944,\n      "maximumLogicalBytes": 6441402368\n    },',
    ),
]
BUDGET_SCHEMA3 = [
    (
        '    "maximumBytes": {\n      "type": "integer",\n      "minimum": 1048576,\n      "maximum": 68719476736\n    },',
        '    "maximumBytes": {\n      "type": "integer",\n      "minimum": 1048576,\n      "maximum": 70866960384\n    },',
    ),
]
BUDGET_FIXTURE3 = [('"expectedPartitionBytes": 68249714688,', '"expectedPartitionBytes": 70397198336,')]
GRAPH_STORAGE3 = [
    (
        'again=await maintainBuildGraphOptionalCachesV1(image,leasePhase);check();}else{await image.recheck();check();}',
        'again=await maintainBuildGraphOptionalCachesV1(image,leasePhase);check();}else{{const{launchGraphOptionalCollectorV1}=await import("../../🧹️cache-gc/🚀️launch/🟦️.ts");await launchGraphOptionalCollectorV1(workspace,space.bavail*space.bsize);}await image.recheck();check();}',
    ),
]

EDITORS3 = {
    "budget-policy": lambda text: apply(text, BUDGET_POLICY3, "budget-policy"),
    "budget-schema": lambda text: apply(text, BUDGET_SCHEMA3, "budget-schema"),
    "budget-fixture": lambda text: apply(text, BUDGET_FIXTURE3, "budget-fixture"),
    "graph-storage": lambda text: apply(text, GRAPH_STORAGE3, "graph-storage"),
}


SPOT_MARK = "ports.platform??process.platform"


def _spots(text):
    lines = text.split("\n")
    return lines, 1, next((index for index, line in enumerate(lines) if SPOT_MARK in line), None)


def runner_normalize(text):
    lines, head, backing = _spots(text)
    lines[head] = "@import"
    if backing is not None:
        lines[backing] = "@backing"
    return "\n".join(lines)


def runner_merge(staged, live):
    """Keeps the two lines a peer set (storage-seam) repointed in the live runner: the platform-contract import and the backing/mounted line."""
    staged_lines, head, backing = _spots(staged)
    live_lines, live_head, live_backing = _spots(live)
    staged_lines[head] = live_lines[live_head]
    if backing is not None and live_backing is not None:
        staged_lines[backing] = live_lines[live_backing]
    return "\n".join(staged_lines)


MERGES = {"gc-runner.ts": ("base-text/gc-runner.ts", runner_normalize, runner_merge)}


def json_edit(text, edit, label):
    document = json.loads(text)
    edit(document)
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def _schema_members(document):
    plan = document["definitions"]["RetirementPlan"]["properties"]["members"]["items"]["properties"]["kind"]["enum"]
    if plan.count("member-index") == 0:
        plan.append("member-index")


PIN = ".semioPhysicalSourceURL)),new URL(\"../../../../../../../🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json\""
ORIGIN = "physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL)"

CLOSURES_TS4 = [
    (
        'import{parseRecordStoreIndex}from "../../../../../../🗃️records/🟦️.ts";\n',
        'import{parseRecordStoreIndex}from "../../../../../../🗃️records/🟦️.ts";\nimport{memberGroupOfPathV1,memberStampOfRowStampV1,parseMemberIndexV1,proveGroupV1}from"./🗂️members/🟨️.mjs";\n',
    ),
    (
        PIN,
        '.semioPhysicalSourceURL)),new URL("./🗂️members/🟨️.mjs",' + ORIGIN + '),new URL("./🗂️members/🧬️schema/🔣️.json",' + ORIGIN + '),new URL("../../../../../../../🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json"',
    ),
    ("index?:Row;expectedHash?:string};", "index?:Row;expectedHash?:string;trusted?:boolean};"),
    ("retainAll:boolean;work:{operations:number;bytes:number}};", "retainAll:boolean;unproven:number;work:{operations:number;bytes:number}};"),
    ("retainAll:false,work:{operations:0,bytes:0}};try{ancestry(proof);", "retainAll:false,unproven:0,work:{operations:0,bytes:0}};try{ancestry(proof);"),
    (
        'function role(name:string):string{\n if(/^build-input-closures',
        'function role(name:string):string{\n if(/^(?:inference\\/(?:(?:selections|contexts)\\/[a-f0-9]{64}\\/)?|generations\\/)members\\.jsonl$/u.test(name))return"member-index";\n if(/^build-input-closures',
    ),
    (
        'if(row.kind==="index"){const pack=proof.rows.get(row.path.slice(0,-4)+".pack");const bytes=smallBytes(proof,row)',
        'if(row.kind==="index"){const pack=proof.rows.get(row.path.slice(0,-4)+".pack");if(row.trusted){if(!pack){missingLeaf(proof,row.path.slice(0,-4)+".pack");proof.retainAll=true;}continue;}const bytes=smallBytes(proof,row)',
    ),
    (
        '/** 🧾️ Captures a complete authenticated reference read set under one actual image-exclusive lifetime. */\n',
        '''/** 🔗️ Admits the closure references one leaf contributes. */
function acceptReferences(proof:Proof,references:readonly string[]){for(const reference of references){proof.references.add(reference);if(proof.references.size>P.maximumReferences)throw Error("Bounded complete closure references required");}}
/** 🔗️ Admits the links one leaf requires: a link to a missing leaf is only tolerated for an inference block, and then the whole retained set is kept. */
function acceptLinks(proof:Proof,links:readonly{path:string;missing:string}[]){for(const link of links)if(!proof.rows.has(link.path)){if(link.missing!=="block"||role(relative(proof.image.root,link.path).split(sep).join("/"))!=="observation")throw Error("Complete retained closure publication required");missingLeaf(proof,link.path);proof.retainAll=true;}}
/** 🗂️ Vouches for rows without reading one leaf byte: a group member by its group's append-only member index plus the lstat stamp, a closure inventory by its content-addressed name, a member pinned by a retirement plan by that plan; a member the index cannot vouch for keeps its group unproven (never retired, references unknown so every inventory stays retained). Only what remains is opened by the read child. */
function resolveIncrementalRows(proof:Proof){
 const root=proof.image.root,groups=new Map<string,{directory:string;index?:Row;rows:Row[]}>();
 for(const row of proof.rows.values()){const group=row.kind==="retired"?undefined:memberGroupOfPathV1(relative(root,row.path).split(sep).join("/"));if(!group)continue;const entry=groups.get(group.group)??{directory:join(root,...group.directory.split("/")),rows:[]};groups.set(group.group,entry);if(row.kind==="member-index")entry.index=row;else entry.rows.push(row);}
 for(const[name,entry]of groups){
  const directory=entry.directory;let text="";
  if(entry.index&&entry.index.bytes<=P.maximumIndexBytes){const bytes=smallBytes(proof,entry.index);entry.index.hash=hash(bytes);entry.index.trusted=true;text=bytes.toString("utf8");}
  const proven=proveGroupV1(parseMemberIndexV1(text,name),entry.rows.map(row=>({name:relative(directory,row.path).split(sep).join("/"),kind:row.kind,bytes:row.bytes,stamp:memberStampOfRowStampV1(row.stamp)})));
  entry.rows.forEach((row,at)=>{const member=proven.members[at]!;if(member.state==="full"||member.state==="light"){row.hash=member.hash;row.trusted=true;if(member.state==="light")proof.retainAll=true;}else{proof.retainAll=true;proof.unproven++;}});
  acceptReferences(proof,proven.references);acceptLinks(proof,proven.links.map(link=>({path:join(directory,...link.p.split("/")),missing:link.m??"forbidden"})));
 }
 for(const row of proof.rows.values()){if(row.hash!==undefined||row.trusted)continue;if(row.expectedHash!==undefined){row.hash=row.expectedHash;row.trusted=true;}else if(row.kind==="source"||row.kind==="lock"){row.hash=row.path.slice(-69,-5);row.trusted=true;proof.candidates.set(row.kind+":"+row.hash,row);}}
}
/** 🧾️ Captures a complete authenticated reference read set under one actual image-exclusive lifetime. */
''',
    ),
    ("retirementCatalog(proof);pairs(proof);", "retirementCatalog(proof);resolveIncrementalRows(proof);pairs(proof);"),
    (
        'const rows=[...proof.rows.values()].filter(row=>row.kind!=="index"||!proof.rows.has(row.path.slice(0,-4)+".pack"));',
        'const rows=[...proof.rows.values()].filter(row=>!row.trusted&&(row.kind!=="index"||!proof.rows.has(row.path.slice(0,-4)+".pack")));',
    ),
    (
        "const groups=[...candidates.values()].filter(group=>group.rows.length).sort(",
        "const planned=new Set<string>(schema.definitions.RetirementPlan.properties.members.items.properties.kind.enum),groups=[...candidates.values()].filter(group=>group.rows.length&&group.rows.every(row=>row.hash!==undefined&&planned.has(row.kind))).sort(",
    ),
]

GRAPH_STORAGE4 = [
    (
        'import {statSync,lstatSync,mkdirSync,rmSync} from "node:fs";',
        'import {statSync,lstatSync,mkdirSync,rmSync,openSync,fstatSync,readSync,writeSync,closeSync,readFileSync} from "node:fs";\nimport {recordMemberLeafV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🟨️.mjs";',
    ),
    (
        "if(!scope.ledger.publish(plan,write))return decline();reservation.commit();return true;}finally{reservation.release();}",
        "if(!scope.ledger.publish(plan,write))return decline();reservation.commit();recordBuildGraphOptionalMemberV1(scope.store.root,resolve(path));return true;}finally{reservation.release();}",
    ),
    (
        "/** 🔌️ Closes retained cache descriptors before releasing original image custody. */\n",
        '/** 📇️ Seals a freshly published group leaf into its group\'s member index (name, size, kind, stamp and the hash of its bytes, taken once, now); a leaf the index cannot take stays unindexed for the background collector and never fails the publication. */\nfunction recordBuildGraphOptionalMemberV1(root,path){try{recordMemberLeafV1({openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync},root,path);}catch{}}\n\n/** 🔌️ Closes retained cache descriptors before releasing original image custody. */\n',
    ),
]

EDITORS4 = {
    "closures-ts": lambda text: apply(text, CLOSURES_TS4, "closures-ts"),
    "closures-schema": lambda text: json_edit(text, _schema_members, "closures-schema"),
    "graph-storage": lambda text: apply(text, GRAPH_STORAGE4, "graph-storage"),
}


CATALOG4 = [
    ('"synopsis": "cache status|gc|clear|remote status|serve",', '"synopsis": "cache status|gc|verify|clear|remote status|serve",'),
    (
        '        { "name": "clear", "synopsis": "cache clear",',
        '        {\n          "name": "verify",\n          "synopsis": "cache verify --deep [--json]",\n          "flags": ["--deep", "--json"],\n          "summary": {\n'
        '            "en": "Read every indexed optional graph leaf and every content-addressed closure inventory again, compare each with its recorded hash or name and print what differs; the only command that re-reads leaf bytes after they were written.",\n'
        '            "de": "Liest jedes indexierte optionale Graph-Blatt und jedes inhaltsadressierte Closure-Inventar erneut, vergleicht es mit seinem erfassten Hash oder Namen und gibt aus, was abweicht; der einzige Befehl, der Blattbytes nach dem Schreiben erneut liest."\n          }\n        },\n'
        '        { "name": "clear", "synopsis": "cache clear",',
    ),
    (
        '"en": "cache status: walk every cache home now',
        '"en": "cache verify (required): read every indexed optional graph leaf and every closure inventory again and compare it with its recorded hash. cache status: walk every cache home now',
    ),
    (
        '"de": "cache status: läuft jetzt jede Cache-Heimat ab',
        '"de": "cache verify (erforderlich): liest jedes indexierte optionale Graph-Blatt und jedes Closure-Inventar erneut und vergleicht es mit seinem erfassten Hash. cache status: läuft jetzt jede Cache-Heimat ab',
    ),
    (
        '        { "command": "repo cache status --tasks --tool=bazel",',
        '        { "command": "repo cache verify --deep", "description": { "en": "Re-read and re-hash every indexed optional graph leaf and closure inventory.", "de": "Liest und hasht jedes indexierte optionale Graph-Blatt und Closure-Inventar erneut." } },\n'
        '        { "command": "repo cache status --tasks --tool=bazel",',
    ),
]

KERNEL_CACHE4 = [
    ('(["status","gc"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")))throw Error("Cache command kernel', '(["status","gc","verify"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")))throw Error("Cache command kernel'),
]
KERNEL_LEDGER4 = [
    ('!["status","gc"].includes(resolved.args[1]??"")||resolved.args.includes("--tasks"))throw Error("Cache ledger command kernel', '!["status","gc","verify"].includes(resolved.args[1]??"")||resolved.args.includes("--tasks"))throw Error("Cache ledger command kernel'),
]
KERNEL_SELECTION4 = [
    ('command==="cache"?(["status","gc"].includes(selected[1]??"")&&!selected.includes("--tasks")?"cache-ledger":"cache")', 'command==="cache"?(["status","gc","verify"].includes(selected[1]??"")&&!selected.includes("--tasks")?"cache-ledger":"cache")'),
]
SELECTION_TEST4 = [
    ("when ?1='cache' and ?3 in ('status','gc') and not ?4 then 'cache-ledger'", "when ?1='cache' and ?3 in ('status','gc','verify') and not ?4 then 'cache-ledger'"),
]
SELECTION_FIXTURE4 = [
    (
        '        "--tasks=true"\n      ],\n      "kind": "cache-ledger"\n    }\n  ]',
        '        "--tasks=true"\n      ],\n      "kind": "cache-ledger"\n    },\n'
        '    {\n      "args": [\n        "cache",\n        "verify"\n      ],\n      "kind": "cache-ledger"\n    },\n'
        '    {\n      "args": [\n        "cache",\n        "verify",\n        "--tasks"\n      ],\n      "kind": "cache"\n    }\n  ]',
    ),
]
EDITORS4["kernel-cache"] = lambda text: apply(text, KERNEL_CACHE4, "kernel-cache4")
EDITORS4["kernel-ledger"] = lambda text: apply(text, KERNEL_LEDGER4, "kernel-ledger4")
EDITORS4["kernel-selection"] = lambda text: apply(text, KERNEL_SELECTION4, "kernel-selection4")
EDITORS4["selection-test"] = lambda text: apply(text, SELECTION_TEST4, "selection-test4")
EDITORS4["selection-fixture"] = lambda text: apply(text, SELECTION_FIXTURE4, "selection-fixture4")


NOTE_MARK = "Optional graph caches are retired by an append-only member index"
NOTE_STRINGS = [
    (
        "status prints the cache ledger",
        "status gibt die Cache-Bilanz des Schemas cache-ledger aus: jede feste Partition, jeden Bereich innerhalb einer Partition, jeden Bereich der Repository- und nativen Cache-Wurzeln mit seinem deklarierten Budget sowie das, was stillgelegt oder nicht deklariert ist; gc sammelt es ein. Beide melden Fortschritt, halten am nächsten Prüfpunkt an, wenn der Aufrufer abbricht, und nehmen für den Bericht keine Lease, der statfs jeder Partition und das Größenverzeichnis liest, das gc und --deep schreiben, und keinen Baum abläuft, solange --deep nicht gegeben ist; gc nimmt die Lease eines Bereichs exklusiv ohne Warten und entfernt nie eine Einheit, deren SQLite-Lease-Datei gesperrt ist, deren Warteschlangenticket einem lebenden Prozess gehört oder die innerhalb der Ruhezeit ihres Bereichs benutzt wurde. Bereiche, die ein Verwalter hält (Nx bei maxCacheSize, Cargo, Bazel, Testnachweise, das Dashboard), werden gemeldet, nicht hier eingesammelt.",
    ),
    (
        "status --tasks prints a CacheStatus",
        "status --tasks gibt einen CacheStatus aus, clear einen CacheClear; beide halten am nächsten Prüfpunkt an, wenn der Aufrufer abbricht, und melden Fortschritt beim Durchlaufen oder Entfernen von Einträgen.",
    ),
]
NOTE_NEW = (
    '        {\n          "en": "Optional graph caches are retired by an append-only member index per group (name, size, kind, stamp and the sha256 of every leaf, taken once when the leaf is written; the shared namespace and the generation documents keep one too) plus an lstat of each member, never by reading leaf bytes; gc indexes a group the index cannot vouch for before it may be retired, holding that group\'s non-blocking writer lease only while it appends rows, never while it reads leaves. verify --deep reads and hashes every indexed leaf and every closure inventory again and prints what differs, publishes progress and stops at the next checkpoint when the caller cancels.",\n'
    '          "de": "Optionale Graph-Caches werden über einen anhängenden Mitgliederindex je Gruppe (Name, Größe, Art, Stempel und sha256 jedes Blatts, einmalig beim Schreiben erfasst; der gemeinsame Namensraum und die Generationsdokumente führen ebenfalls einen) und ein lstat jedes Mitglieds stillgelegt, nie durch Lesen von Blattbytes; gc indexiert eine Gruppe, für die der Index nicht bürgen kann, bevor sie stillgelegt werden darf, und hält deren nichtblockierende Schreiber-Lease nur beim Anhängen von Zeilen, nie beim Lesen von Blättern. verify --deep liest und hasht jedes indexierte Blatt und jedes Closure-Inventar erneut und gibt aus, was abweicht, meldet Fortschritt und hält am nächsten Prüfpunkt an, wenn der Aufrufer abbricht."\n        },\n'
)


def localize_notes(text):
    lines = text.split("\n")
    for start, german in NOTE_STRINGS:
        marker = '        "' + start
        hits = [index for index, line in enumerate(lines) if line.startswith(marker)]
        if len(hits) > 1:
            raise Drift("catalog4: note %s found %d times" % (start, len(hits)))
        if hits:
            literal = lines[hits[0]].strip().rstrip(",")
            json.loads(literal)
            lines[hits[0]] = '        {\n          "en": %s,\n          "de": %s\n        },' % (literal, json.dumps(german, ensure_ascii=False))
    text = "\n".join(lines)
    if NOTE_MARK not in text:
        anchor = '        {\n          "en": "status --tasks prints a CacheStatus'
        if text.count(anchor) != 1:
            raise Drift("catalog4: notes anchor found %d times" % text.count(anchor))
        text = text.replace(anchor, NOTE_NEW + anchor)
    return text


EDITORS4["catalog"] = lambda text: localize_notes(apply(text, CATALOG4, "catalog4"))
