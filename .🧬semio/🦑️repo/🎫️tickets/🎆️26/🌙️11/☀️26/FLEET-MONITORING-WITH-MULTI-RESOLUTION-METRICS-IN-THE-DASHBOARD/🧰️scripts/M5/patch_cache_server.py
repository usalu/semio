import os, sys, hashlib
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
path = B + "/🌐️remote-cache/🖥️server/🟦️.ts"
text = open(path, encoding="utf-8").read()
before = hashlib.sha256(text.encode()).hexdigest()[:12]
edits = [
 ('import {createMirrorHandlerV1} from "../🪞️mirror/🟦️.ts";',
  'import {createMirrorHandlerV1} from "../🪞️mirror/🟦️.ts";\nimport {openCounterLedgerV1,type CounterLedgerV1} from "../../📈️counter-ledger/🟦️.ts";'),
 ('/** 🧮️ Counts what one server did per keyspace since it started; `stale` counts action results that were dropped on a read because their blobs had left the store. */',
  '/** 🧮️ Counts what one server did per keyspace; `stale` counts action results that were dropped on a read because their blobs had left the store. In `GET /status` the `counters` are cumulative over every run of the server (the persisted ledger plus this run, see `📈️counter-ledger`), `session` is this run alone. */'),
 ('counters:RemoteCacheCountersV1;startedAt:string;fleet?:',
  'counters:RemoteCacheCountersV1;session?:RemoteCacheCountersV1;ledger?:Readonly<{sinceMs:number;restarts:number}>;startedAt:string;fleet?:'),
 ('onRequest?:(event:RemoteCacheRequestEventV1)=>void}>;',
  'onRequest?:(event:RemoteCacheRequestEventV1)=>void;ledger?:CounterLedgerV1}>;'),
 (' const publications=new Map<string,Promise<void>>();\n',
  ' const publications=new Map<string,Promise<void>>();\n options.ledger?.bind(()=>({ac:counters.ac,cas:counters.cas,nx:counters.nx,...(options.peers===undefined?{}:{peers:options.peers.counters()})}));\n const cumulative=():Readonly<{counters:RemoteCacheCountersV1;peers?:FleetPeerCountersV1}>=>{\n  if(options.ledger===undefined)return {counters,...(options.peers===undefined?{}:{peers:options.peers.counters()})};\n  const total=options.ledger.total(),kind=(name:RemoteCacheKindV1)=>({...row(),...total[name]});\n  return {counters:{ac:kind("ac"),cas:kind("cas"),nx:kind("nx")},...(options.peers===undefined?{}:{peers:{...options.peers.counters(),...total.peers}})};\n };\n'),
 ('reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters,startedAt,',
  'reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters:total.counters,...(options.ledger===undefined?{}:{session:counters,ledger:{sinceMs:options.ledger.sinceMs,restarts:options.ledger.restarts}}),startedAt,'),
 ('claims:claims.summary(),...(options.peers===undefined?{}:{peers:options.peers.counters()}),',
  'claims:claims.summary(),...(total.peers===undefined?{}:{peers:total.peers}),'),
 ('   const usage=options.store.usage(),status:RemoteCacheServerStatusV1=',
  '   const total=cumulative(),usage=options.store.usage(),status:RemoteCacheServerStatusV1='),
 (' const provenance=await openProvenanceV1(options.directory);\n',
  ' const provenance=await openProvenanceV1(options.directory);\n const ledger=openCounterLedgerV1({path:join(root,"counters.json"),name:"remote-cache"});\n'),
 ('const opened=store,handler=createRemoteCacheHandlerV1({store:opened,',
  'const opened=store,handler=createRemoteCacheHandlerV1({store:opened,ledger,'),
 ('await opened.close();await provenance.flush();await release();',
  'await opened.close();ledger.close();await provenance.flush();await release();'),
 ('  closed=true;await store?.close().catch(()=>{});await release();throw error;',
  '  closed=true;ledger.close();await store?.close().catch(()=>{});await release();throw error;'),
]
for old, new in edits:
    n = text.count(old)
    if n != 1:
        print("ANCHOR", n, old[:90]); sys.exit(1)
    text = text.replace(old, new)
if "--check" in sys.argv:
    print("anchors ok"); sys.exit(0)
temporary = path + ".m5.tmp"
open(temporary, "w", encoding="utf-8").write(text)
os.replace(temporary, path)
print("patched", before, "->", hashlib.sha256(text.encode()).hexdigest()[:12])
