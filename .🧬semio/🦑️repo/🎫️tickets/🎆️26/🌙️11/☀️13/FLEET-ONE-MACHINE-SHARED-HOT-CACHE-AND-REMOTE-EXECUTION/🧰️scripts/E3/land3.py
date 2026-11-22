import sys,json,os,shutil
B=sys.argv[1].rstrip('/')+'/'
S=sys.argv[2].rstrip('/')+'/'
MCP=sys.argv[3].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:80],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
RC=B+'🌐️remote-cache/'
os.makedirs(RC+'🧾️diff/🧪️tests/🧫️fixtures',exist_ok=True)
for rel in ['🟦️.ts','🧪️tests/🟦️.ts','🧪️tests/🐍️.py','🧪️tests/🧫️fixtures/🔣️.json']:
    shutil.copy(S+'🧾️diff/'+rel,RC+'🧾️diff/'+rel)
edit(RC+'⌨️command/🟦️.ts',[
('import texts from "../📖️texts/🔣️.json" with {type:"json"};','import {diffEvidenceV1,type EvidenceDiffV1} from "../🧾️diff/🟦️.ts";\nimport texts from "../📖️texts/🔣️.json" with {type:"json"};'),
('type Operation="serve"|"status"|"gc"|"rotate";','type Operation="serve"|"status"|"gc"|"rotate"|"evidence";'),
('  rotate:{valued:["--lang"],plain:[]}\n','  rotate:{valued:["--lang"],plain:[]},\n  evidence:{valued:["--lang"],plain:[]}\n'),
('export type RemoteCacheArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>}>;','export type RemoteCacheArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>;words:readonly string[]}>;'),
('/** 🧭️ Reads `serve|status|gc|rotate` with its options','/** 🧭️ Reads `serve|status|gc|rotate|evidence` with its options (`evidence` also takes the words `list` or `show <id>`)'),
('if(operation!=="serve"&&operation!=="status"&&operation!=="gc"&&operation!=="rotate")throw','if(operation!=="serve"&&operation!=="status"&&operation!=="gc"&&operation!=="rotate"&&operation!=="evidence")throw'),
(' const allowed=OPTIONS[operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>();',' const allowed=OPTIONS[operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),words:string[]=[];'),
('  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);\n','  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);\n  if(operation==="evidence"&&!argument.startsWith("--")){words.push(argument);continue;}\n'),
(' return Object.freeze({operation,values:Object.freeze(values),flags});',' return Object.freeze({operation,values:Object.freeze(values),flags,words:Object.freeze(words)});'),
('/** 🎬️ Routes `remote-cache serve|status|gc|rotate`;','''/** 🗂️ Reads the evidence the running cache server keeps of the writes it refused as divergences (`repo remote-cache evidence list`: the entries, newest first; `show <id>`: one entry with, for every output that differs, both digests and sizes, the first differing byte, the number of differing bytes and the lines only one side has, Brotli and JSON unpacked). The entries are fetched with the same signed client as `status`; a server that does not answer is a fault, never a hang. */
export async function remoteCacheEvidenceV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<unknown>{
 const parsed=parseRemoteCacheArgumentsV1(["evidence",...args]),words=parsed.words,action=words[0]??"list";
 if(action!=="list"&&action!=="show"||action==="list"&&words.length>1||action==="show"&&words.length!==2)throw Error(both("evidence.syntax"));
 const found=await resolveAuthorisedRemoteCacheV1({...context.environment,SEMIO_WORKSPACE_ROOT:context.environment.SEMIO_WORKSPACE_ROOT??context.root},{root:context.root}),resolved=found.resolution;
 if(resolved===undefined)throw Error(both("evidence.off"));
 const call=remoteCacheFetchV1(resolved.environment,{root:context.root}),endpoint=resolved.contract.endpoint,limit=resolved.contract.timeoutSeconds*1000;
 const get=async(path:string):Promise<Response>=>{
  let response:Response;
  try{response=await call(endpoint+path,{signal:AbortSignal.any([context.signal,AbortSignal.timeout(limit)])});}catch(error){if(context.signal.aborted)throw error;throw Error(both("evidence.fault",{status:0}));}
  if(response.status!==200){await response.body?.cancel();throw Error(both("evidence.fault",{status:response.status}));}
  return response;
 };
 await context.progress("remote-cache:evidence");
 const listed=(await (await get("/evidence")).json()) as {evidence:readonly EvidenceRowV1[]};
 if(action==="list"){const result={version:1,action:"list",evidence:listed.evidence};requireRemoteCacheShapeV1("EvidenceList",result,"Closed remote cache evidence list required");return result;}
 const matches=listed.evidence.filter(row=>row.id===words[1]||row.id.startsWith(words[1]!)),row=matches.find(candidate=>candidate.id===words[1])??(matches.length===1?matches[0]:undefined);
 if(row===undefined)throw Error(both("evidence.unknown",{id:words[1]!}));
 const diffs:EvidenceDiffV1[]=[];
 for(const path of row.differing){
  const sides=row.files.filter(file=>file.path===path),pick=async(side:"kept"|"refused")=>{
   const file=sides.find(candidate=>candidate.side===side);
   if(file===undefined)return undefined;
   const bytes=file.stored?new Uint8Array(await (await get(`/evidence/${row.id}/${file.name}`)).arrayBuffer()):undefined;
   return{sha256:file.sha256,bytes,size:file.bytes};
  };
  diffs.push(diffEvidenceV1(path,await pick("kept"),await pick("refused")));
 }
 const result={version:1,action:"show",evidence:row,diffs};requireRemoteCacheShapeV1("EvidenceShow",result,"Closed remote cache evidence required");return result;
}

type EvidenceRowV1=Readonly<{id:string;atMs:number;sort:string;namespace:string;key:string;keptOrigin:string;refusedOrigin:string;differing:readonly string[];files:readonly Readonly<{name:string;side:"kept"|"refused";path:string;sha256:string;bytes:number;stored:boolean}>[]}>;

/** 🎬️ Routes `remote-cache serve|status|gc|rotate|evidence`;'''),
(' if(parsed.operation==="rotate")return rotateRemoteCacheV1(args.slice(1),context);\n',' if(parsed.operation==="rotate")return rotateRemoteCacheV1(args.slice(1),context);\n if(parsed.operation==="evidence")return {exitCode:0,result:await remoteCacheEvidenceV1(args.slice(1),context)};\n')])
t=json.load(open(RC+'📖️texts/🔣️.json'))
m=t['messages']
m['error.syntax']={"en":"Expected repo remote-cache serve|status|gc|rotate|evidence.","de":"Erwartet wird repo remote-cache serve|status|gc|rotate|evidence."}
m['evidence.syntax']={"en":"Expected repo remote-cache evidence [list|show <id>].","de":"Erwartet wird repo remote-cache evidence [list|show <id>]."}
m['evidence.off']={"en":"No remote cache endpoint is configured, so there is no evidence to read.","de":"Es ist kein Remote-Cache-Endpunkt konfiguriert, daher gibt es keine Belege zu lesen."}
m['evidence.fault']={"en":"The cache server did not deliver its evidence (status {status}; 0 means it did not answer).","de":"Der Cache-Server hat seine Belege nicht geliefert (Status {status}; 0 bedeutet, er hat nicht geantwortet)."}
m['evidence.unknown']={"en":"No evidence {id}; repo remote-cache evidence list shows the ids.","de":"Kein Beleg {id}; repo remote-cache evidence list zeigt die Ids."}
raw=open(RC+'📖️texts/🔣️.json').read()
indent=2 if raw.startswith('{\n  "') else 1
json.dump(t,open(RC+'📖️texts/🔣️.json','w'),indent=indent,ensure_ascii=False);open(RC+'📖️texts/🔣️.json','a').write("\n")
edit(RC+'🧪️tests/🟦️.ts',[('toThrow("Expected repo remote-cache serve|status|gc|rotate.")','toThrow("Expected repo remote-cache serve|status|gc|rotate|evidence.")')])
# schema
p=B+'🏃️execution/🧬️schema/🌐️remote-cache.json'
edit(p,[
('    { "$ref": "#/definitions/GcResult" },\n','    { "$ref": "#/definitions/GcResult" },\n    { "$ref": "#/definitions/EvidenceList" },\n    { "$ref": "#/definitions/EvidenceShow" },\n'),
('(?!(?:ac|cas|v1|healthz|status)$)','(?!(?:ac|cas|v1|healthz|status|evidence)$)'),
('    "TextCatalogue": {','''    "EvidenceFile": {
      "description": "One file of the evidence of a refused write: the action result of one side (kept.ac, refused.ac) or one output that differs; stored is false when the blob was gone or larger than the evidence limit.",
      "type": "object",
      "additionalProperties": false,
      "required": ["name", "side", "path", "sha256", "bytes", "stored"],
      "properties": {
        "name": { "type": "string", "pattern": "^[0-9A-Za-z._-]{1,160}$" },
        "side": { "enum": ["kept", "refused"] },
        "path": { "type": "string" },
        "sha256": { "type": "string", "pattern": "^([0-9a-f]{64})?$" },
        "bytes": { "type": "integer", "minimum": 0 },
        "stored": { "type": "boolean" }
      }
    },
    "EvidenceRow": {
      "description": "What the cache server keeps of one write it refused as a divergence: who, what class of result, which outputs differ, and the files that were kept (GET /evidence).",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "atMs", "sort", "namespace", "key", "keptOrigin", "refusedOrigin", "differing", "files"],
      "properties": {
        "id": { "type": "string", "pattern": "^[0-9A-Za-z._-]{1,160}$" },
        "atMs": { "type": "integer", "minimum": 0 },
        "sort": { "type": "string" },
        "namespace": { "type": "string" },
        "key": { "type": "string" },
        "keptOrigin": { "type": "string" },
        "refusedOrigin": { "type": "string" },
        "differing": { "type": "array", "maxItems": 16, "items": { "type": "string" } },
        "files": { "type": "array", "items": { "$ref": "#/definitions/EvidenceFile" } }
      }
    },
    "EvidenceList": {
      "description": "What repo remote-cache evidence list prints: the evidence entries of the running server, newest first.",
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "action", "evidence"],
      "properties": {
        "version": { "const": 1 },
        "action": { "const": "list" },
        "evidence": { "type": "array", "items": { "$ref": "#/definitions/EvidenceRow" } }
      }
    },
    "EvidenceSide": {
      "type": "object",
      "additionalProperties": false,
      "required": ["sha256", "bytes", "stored"],
      "properties": {
        "sha256": { "type": "string", "pattern": "^([0-9a-f]{64})?$" },
        "bytes": { "type": "integer", "minimum": 0 },
        "stored": { "type": "boolean" }
      }
    },
    "EvidenceDiff": {
      "description": "The comparison of one output that differs: both sides (null when a side does not name the path), the first differing byte and the number of differing bytes when both sides were kept, and the lines only one side has (- kept, + refused; Brotli and JSON are unpacked), at most 20.",
      "type": "object",
      "additionalProperties": false,
      "required": ["path", "kept", "refused", "firstDifferenceByte", "differingBytes", "lines"],
      "properties": {
        "path": { "type": "string" },
        "kept": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/EvidenceSide" }] },
        "refused": { "oneOf": [{ "const": null }, { "$ref": "#/definitions/EvidenceSide" }] },
        "firstDifferenceByte": { "oneOf": [{ "const": null }, { "type": "integer", "minimum": 0 }] },
        "differingBytes": { "oneOf": [{ "const": null }, { "type": "integer", "minimum": 0 }] },
        "lines": { "type": "array", "maxItems": 20, "items": { "type": "string", "maxLength": 201 } }
      }
    },
    "EvidenceShow": {
      "description": "What repo remote-cache evidence show <id> prints: one evidence entry with the comparison of every output that differs.",
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "action", "evidence", "diffs"],
      "properties": {
        "version": { "const": 1 },
        "action": { "const": "show" },
        "evidence": { "$ref": "#/definitions/EvidenceRow" },
        "diffs": { "type": "array", "maxItems": 16, "items": { "$ref": "#/definitions/EvidenceDiff" } }
      }
    },
    "TextCatalogue": {''')])
# catalog
p=B+'⚙️operations/🔣️catalog.json'
edit(p,[
('"synopsis": "remote-cache serve|status|gc|rotate",','"synopsis": "remote-cache serve|status|gc|rotate|evidence",'),
('''          }
        }
      ],
      "flags": ["--port", "--host", "--directory", "--max-bytes", "--read-only", "--tls-certificate", "--tls-key", "--json", "--dry-run"],''','''          }
        },
        {
          "name": "evidence",
          "synopsis": "remote-cache evidence [list|show <id>]", "flags": [],
          "summary": {
            "en": "Read what the running cache server keeps of the writes it refused because another machine had stored a different result for the same key: list the entries, or show one with both digests, the first differing byte and the differing lines of every output that differs.",
            "de": "Liest, was der laufende Cache-Server von den Schreibvorgängen aufbewahrt, die er abgelehnt hat, weil ein anderer Rechner für denselben Schlüssel ein anderes Ergebnis gespeichert hatte: listet die Einträge oder zeigt einen mit beiden Digests, dem ersten abweichenden Byte und den abweichenden Zeilen jeder abweichenden Ausgabe."
          }
        }
      ],
      "flags": ["--port", "--host", "--directory", "--max-bytes", "--read-only", "--tls-certificate", "--tls-key", "--json", "--dry-run"],'''),
('''and publish the registration.", "de": "Ersetzt die Flotten-Signaturschlüssel dieses Rechners und veröffentlicht die Registrierung." } }
      ],''','''and publish the registration.", "de": "Ersetzt die Flotten-Signaturschlüssel dieses Rechners und veröffentlicht die Registrierung." } },
        { "command": "repo remote-cache evidence show <id>", "description": { "en": "Compare the kept and the refused result of one divergence, output by output.", "de": "Vergleicht das behaltene und das abgelehnte Ergebnis einer Abweichung, Ausgabe für Ausgabe." } }
      ],''')])
# mcp
edit(MCP+'🖼️assets/🏗️build-surface.json',[
('    { "operation": "remote-cache", "subcommand": "status", "kind": "query", "flags": [] },\n','''    { "operation": "remote-cache", "subcommand": "status", "kind": "query", "flags": [] },
    {
      "operation": "remote-cache",
      "subcommand": "evidence",
      "kind": "query",
      "flags": [],
      "positional": [
        { "name": "action", "optional": true, "enum": ["list", "show"], "description": { "en": "list (default) names the evidence entries, show compares the kept and the refused result of one.", "de": "list (Standard) nennt die Belege, show vergleicht das behaltene und das abgelehnte Ergebnis eines Belegs." } },
        { "name": "id", "optional": true, "pattern": "^[0-9A-Za-z._-]{1,160}$", "description": { "en": "Evidence id from list; required for show.", "de": "Beleg-Id aus list; für show erforderlich." } }
      ]
    },
''')])
edit(MCP+'🧪️tests/🧫️fixtures/🏗️build-surface.json',[('    "remote_cache_status",\n','    "remote_cache_evidence",\n    "remote_cache_status",\n')])
with open(RC+'🧪️tests/🟦️.ts','a') as f:f.write(open(S+'command-test.ts').read())
print("ok")
