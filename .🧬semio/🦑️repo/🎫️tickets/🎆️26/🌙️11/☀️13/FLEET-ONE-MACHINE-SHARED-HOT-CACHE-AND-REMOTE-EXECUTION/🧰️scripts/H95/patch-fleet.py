import re,sys
B=sys.argv[1]
def rd(p): return open(B+'/'+p,encoding='utf8').read()
def wr(p,s): open(B+'/'+p,'w',encoding='utf8').write(s)
def sub1(s,old,new,count=1):
    n=s.count(old)
    if n!=count: raise SystemExit(f'anchor count {n}!={count}: {old[:70]!r}')
    return s.replace(old,new)

# schema
p='🚢️fleet-status/🧬️schema/🔣️.json'
s=rd(p)
s,n=re.subn(r'("rates",\n)(\s+)("findings",\n\s+"sources")',lambda m:m.group(1)+m.group(2)+'"broken",\n'+m.group(2)+m.group(3),s)
assert n==2,n
s,n=re.subn(r'("cas",\n\s+"offload",\n(\s+)"rates")\n',lambda m:m.group(1)+',\n'+m.group(2)+'"broken"\n',s)
assert n==2,n
s=sub1(s,'''    "cas": {
     "$ref": "#/definitions/CasFact"
    },''','''    "cas": {
     "$ref": "#/definitions/CasFact"
    },
    "broken": {
     "$ref": "#/definitions/BrokenSection"
    },''')
s=sub1(s,'''    "cas": {
     "$ref": "#/definitions/CasSection"
    },''','''    "cas": {
     "$ref": "#/definitions/CasSection"
    },
    "broken": {
     "$ref": "#/definitions/BrokenSection"
    },''')
s,n=re.subn(r'(\n(\s+)"cas": \{\n\s+"\$ref": "#/definitions/Report/properties/cas"\n\s+\},)',lambda m:m.group(1)+'\n'+m.group(2)+'"broken": {\n'+m.group(2)+' "$ref": "#/definitions/Report/properties/broken"\n'+m.group(2)+'},',s)
assert n==2,n
defs='''  "BrokenRow": {
   "description": "One crate of the broken crates board: status, first error and last editor (see the broken crates module).",
   "type": "object",
   "additionalProperties": false,
   "required": ["crate", "status", "checkedAtMs"],
   "properties": {
    "crate": {"type": "string", "minLength": 1},
    "dir": {"type": "string"},
    "status": {"enum": ["ok", "broken", "blocked", "unchecked"]},
    "firstError": {"type": "object", "additionalProperties": false, "required": ["message"], "properties": {"code": {"type": "string"}, "message": {"type": "string"}, "file": {"type": "string"}, "line": {"type": "integer", "minimum": 0}}},
    "lastEditor": {"type": "object", "additionalProperties": false, "required": ["origin", "atMs", "path"], "properties": {"origin": {"type": "string", "minLength": 1}, "atMs": {"type": "integer", "minimum": 0}, "path": {"type": "string"}}},
    "checkedAtMs": {"type": "integer", "minimum": 0},
    "brokenSinceMs": {"type": "integer", "minimum": 0},
    "digest": {"type": "string"},
    "origin": {"type": "string"}
   }
  },
  "BrokenSection": {
   "description": "The broken crates of the fleet: the freshest check of every crate over the boards of all machines, with the counts and the age of every board.",
   "type": "object",
   "additionalProperties": false,
   "required": ["atMs", "origins", "counts", "rows"],
   "properties": {
    "atMs": {"type": "integer", "minimum": 0},
    "origins": {"type": "array", "items": {"type": "object", "additionalProperties": false, "required": ["origin", "ageSeconds"], "properties": {"origin": {"type": "string"}, "ageSeconds": {"type": "integer", "minimum": 0}, "crates": {"type": "integer", "minimum": 0}}}},
    "counts": {"type": "object", "additionalProperties": false, "required": ["ok", "broken", "blocked", "unchecked"], "properties": {"ok": {"type": "integer", "minimum": 0}, "broken": {"type": "integer", "minimum": 0}, "blocked": {"type": "integer", "minimum": 0}, "unchecked": {"type": "integer", "minimum": 0}}},
    "rows": {"type": "array", "items": {"$ref": "#/definitions/BrokenRow"}}
   }
  },
'''
s=sub1(s,'  "CasFact": {\n',defs+'  "CasFact": {\n')
wr(p,s)

# aggregation
p='🚢️fleet-status/🟦️.ts'
s=rd(p)
s=sub1(s,'"memory"|"cas"|"offload"|"rates";ok:boolean','"memory"|"cas"|"offload"|"rates"|"broken";ok:boolean')
s=sub1(s,' cas?:FleetCasFactV1;\n',' cas?:FleetCasFactV1;\n broken?:BrokenSectionV1;\n')
s=sub1(s,'cas?:Row;retention?:Row|null;','cas?:Row;broken?:Row;retention?:Row|null;')
s=sub1(s,'...facts.cas===undefined?{}:{cas:casSection(facts.cas,atMs)},','...facts.cas===undefined?{}:{cas:casSection(facts.cas,atMs)},...facts.broken===undefined?{}:{broken:facts.broken as unknown as Row},')
s=sub1(s,' if(report.cas!==undefined)result.cas=report.cas;\n',' if(report.cas!==undefined)result.cas=report.cas;\n if(report.broken!==undefined){const broken=report.broken as Row;result.broken={...broken,rows:(broken.rows as Row[]).filter(row=>row.status==="broken").slice(0,10)};}\n')
s=sub1(s,' const retention=report.retention;\n if(!want("retention")){}',' if(want("broken")&&report.broken!==undefined)for(const row of brokenFormatV1(report.broken as never,language,limit))lines.push(row);\n const retention=report.retention;\n if(!want("retention")){}')
# import
i=s.index('\n',s.index('import '))
s=s[:i+1]+'import {brokenFormatV1,type BrokenSectionV1} from "../💥️broken/🟦️.ts";\n'+s[i+1:]
wr(p,s)

# reader
p='🚢️fleet-status/📥️readers/🟦️.ts'
s=rd(p)
s=sub1(s,'export const FLEET_READERS_V1:Required<FleetReadersV1>=Object.freeze({','''/** 💥️ The broken crates of the fleet: the boards every sweeping machine publishes in the fleet store, merged to the freshest check per crate; nothing when no machine publishes one. @see ../../💥️broken/🟦️.ts */
export const readFleetBrokenV1:FleetReaderV1=async context=>{
 const [{brokenReadBoardsV1,brokenSectionV1},{fleetStoreRootV1}]=await Promise.all([import("../../💥️broken/🟦️.ts"),import("../../🌐️remote-cache/🛰️fleet/🟦️.ts")]);
 const store=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root);
 if(store===undefined||store==="")return {};
 const boards=brokenReadBoardsV1(store);
 return boards.length===0?{}:{broken:brokenSectionV1(boards,context.now())};
};

export const FLEET_READERS_V1:Required<FleetReadersV1>=Object.freeze({broken:readFleetBrokenV1,''')
wr(p,s)

# command
p='🚢️fleet-status/⌨️command/🟦️.ts'
s=rd(p)
s=sub1(s,'  if(args[0]==="hostname"){','  if(args[0]==="broken"){const {runBrokenCliV1}=await import("../../💥️broken/⌨️command/🟦️.ts");return runBrokenCliV1(root,args.slice(1),environment,write);}\n  if(args[0]==="hostname"){')
wr(p,s)

# catalogue: subcommand + synopsis
p='⚙️operations/🔣️catalog.json'
s=rd(p)
s=sub1(s,'"synopsis": "fleet status|hostname",','"synopsis": "fleet status|hostname|broken",')
m=re.search(r'(\n( +)\{\n +"name": "hostname",.*?\n\2\})',s,re.S)
assert m,'hostname subcommand'
ind=m.group(2)
entry=f''',
{ind}{{
{ind}  "name": "broken",
{ind}  "synopsis": "fleet broken sweep [--once|--watch] [--full]",
{ind}  "summary": {{
{ind}    "en": "Sweep the workspace for broken crates: one low-priority cargo check on a consistent source snapshot (only crates changed in the replica journal and their dependents), published per crate with status, first error and last editor; `repo fleet status --section broken` shows the fleet's board. --watch keeps sweeping until cancelled.",
{ind}    "de": "Prüft den Arbeitsbereich auf kaputte Crates: ein cargo check mit niedrigster Priorität auf einer konsistenten Quelltext-Momentaufnahme (nur im Replika-Journal geänderte Crates und ihre Abhängigen), je Crate mit Status, erstem Fehler und letztem Bearbeiter veröffentlicht; `repo fleet status --section broken` zeigt die Tafel der Flotte. --watch prüft bis zum Abbruch weiter."
{ind}  }}
{ind}}}'''
s=s[:m.end()]+entry+s[m.end():]
wr(p,s)

# door: operation helper + main hook
p='🚪️door/🟦️.ts'
s=rd(p)
s=sub1(s,'/** 💓️ Says every few seconds what the front door works on','''/** 🧭️ The operation words of one command line (global selections consumed, the opaque tail kept), empty when the line is refused; the snapshot hook of the front door reads the command from it. */
export function buildDoorOperationV1(typed:readonly string[],environment:NodeJS.ProcessEnv,source:BuildDoorCatalogueV1=catalogue as BuildDoorCatalogueV1,rules:BuildDoorPolicyV1=policy as BuildDoorPolicyV1):readonly string[]{
 const selection=selectBuildDoorArgumentsV1(normalizeBuildOptionsV1(typed,source).slice(typed[0]==="repo"?1:0),environment,source,rules);
 return "refused" in selection?[]:selection.args;
}

/** 💓️ Says every few seconds what the front door works on''')
wr(p,s)
p='🚪️door/🏃️main/🟦️.ts'
s=rd(p)
s=sub1(s,'import {buildDoorStaysQuietV1,runBuildDoorV1,startBuildDoorLivenessV1} from "../🟦️.ts";','import {buildDoorOperationV1,buildDoorStaysQuietV1,runBuildDoorV1,startBuildDoorLivenessV1} from "../🟦️.ts";')
s=sub1(s,' profileMarkV1("boot.main");\n',' profileMarkV1("boot.main");\n if(environment.SEMIO_SOURCE_SNAPSHOT===undefined&&environment.SEMIO_SOURCE_SNAPSHOT_MODE!=="off"){const {snapshotDoorV1}=await import("../../📸️snapshot/🚪️door/🟦️.ts"),code=await snapshotDoorV1(root,argv,buildDoorOperationV1(argv,environment),environment);if(code!==undefined){process.exitCode=code;return;}}\n')
wr(p,s)
print('patched')
