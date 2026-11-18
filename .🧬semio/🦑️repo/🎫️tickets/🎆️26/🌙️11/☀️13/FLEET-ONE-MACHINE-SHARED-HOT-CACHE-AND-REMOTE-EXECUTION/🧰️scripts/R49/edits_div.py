L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
G=B+"🪪️graph-state/📦️project-records/🧳️portable/"
RC=B+"🌐️remote-cache/"
E=[
 (G+"🟦️.ts",
  "const neutralWay=(host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1)",
  '''const zeroStamps=(fact:Record<string,unknown>,policy:PortableRowsPolicyV1):Record<string,unknown>=>Object.fromEntries(Object.entries(fact).map(([name,value])=>[name,policy.stampFields.includes(name)&&typeof value==="number"?policy.stampValue:value]));
/** 🕰️ A row without the stamps of the machine that recorded it: the modification time, the change time and the inode of a file are hints that let the machine that recorded the row skip hashing the file, they differ on every machine for the same content, and a taker decides by the size and the SHA-256 when they do not match. The neutral form carries the declared value instead, so two machines that infer one project write the same bytes; the `file` rows and the sources of `analysis` rows (and their nested rows) are covered. */
export function unstampRowV1(row:ObservationRowV1,policy:PortableRowsPolicyV1=portableRowsPolicyV1):ObservationRowV1{
 const fact=row.fact;
 if(fact===null||typeof fact!=="object"||Array.isArray(fact))return row;
 const body=fact as Record<string,unknown>;
 if(row.kind==="file"&&body.error===undefined)return{...row,fact:zeroStamps(body,policy)};
 if(row.kind!=="analysis")return row;
 const source=body.source!==null&&typeof body.source==="object"&&!Array.isArray(body.source)?zeroStamps(body.source as Record<string,unknown>,policy):body.source,nested=Array.isArray(body.nestedObservations)?(body.nestedObservations as ObservationRowV1[]).map(item=>unstampRowV1(item,policy)):body.nestedObservations;
 return{...row,fact:{...body,...source===undefined?{}:{source},...nested===undefined?{}:{nestedObservations:nested}}};
}
const neutralWay=(host:HostV1,keys:KeyMapV1,policy:PortableRowsPolicyV1)''',1),
 (G+"🟦️.ts",
  "withoutLocationRowsV1(rows,host.root,policy).map(row=>convertRow(row,way,policy))",
  "withoutLocationRowsV1(rows,host.root,policy).map(row=>unstampRowV1(convertRow(row,way,policy),policy))",1),
 (G+"🧬️schema/🔣️.json",
  '''        "factPathKinds",
        "residue"
      ],''',
  '''        "factPathKinds",
        "residue",
        "stampFields",
        "stampValue"
      ],''',1),
 (G+"🧬️schema/🔣️.json",
  '''        "residue": {
          "type": "string"
        }
      },''',
  '''        "residue": {
          "type": "string"
        },
        "stampFields": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "stampValue": {
          "type": "number"
        }
      },''',1),
 (G+"🧬️schema/🔣️.json",
  '''        "residue": "⟦semio-"
      }''',
  '''        "residue": "⟦semio-",
        "stampFields": ["mtimeMs", "ctimeMs", "ino"],
        "stampValue": 0
      }''',1),
 (RC+"🔬️action-result/🟦️.ts",
  "stable:readonly RemoteCacheDigestV1[];manifest:boolean}>;",
  "stable:readonly RemoteCacheDigestV1[];manifest:boolean;paths:readonly string[]}>;",1),
 (RC+"🔬️action-result/🟦️.ts",
  "const references:RemoteCacheDigestV1[]=[],trees:RemoteCacheDigestV1[]=[],stable:RemoteCacheDigestV1[]=[];let manifest=false;",
  "const references:RemoteCacheDigestV1[]=[],trees:RemoteCacheDigestV1[]=[],stable:RemoteCacheDigestV1[]=[],paths:string[]=[];let manifest=false;",1),
 (RC+"🔬️action-result/🟦️.ts",
  '   if(path==="")fail("output_files.path is required");',
  '   if(path==="")fail("output_files.path is required");\n   paths.push(path);',1),
 (RC+"🔬️action-result/🟦️.ts",
  "return Object.freeze({references:Object.freeze(references),trees:Object.freeze(trees),stable:Object.freeze(stable),manifest});",
  "return Object.freeze({references:Object.freeze(references),trees:Object.freeze(trees),stable:Object.freeze(stable),manifest,paths:Object.freeze(paths)});",1),
 (RC+"🧾️provenance/🟦️.ts",
  "diverged(row:Omit<DivergenceV1,\"atMs\">):void;",
  "diverged(row:Omit<DivergenceV1,\"atMs\">,sort?:string):void;",1),
 (RC+"🧾️provenance/🟦️.ts",
  "summary():Readonly<{count:number;recent:readonly DivergenceV1[]}>;",
  "summary():Readonly<{count:number;recent:readonly DivergenceV1[];since:number;byClass:Readonly<Record<string,number>>}>;",1),
 (RC+"🧾️provenance/🟦️.ts",
  "  diverged(row){\n",
  "  diverged(row,sort=\"other\"){\n   classes.set(sort,(classes.get(sort)??0)+1);\n",1),
 (RC+"🧾️provenance/🟦️.ts",
  "summary:()=>({count:divergences.length,recent:divergences.slice(-RECENT)}),",
  "summary:()=>({count:divergences.length,recent:divergences.slice(-RECENT),since:opened,byClass:Object.fromEntries([...classes].sort(([a],[b])=>a<b?-1:1))}),",1),
 (RC+"🧾️provenance/🟦️.ts",
  "let divergences:DivergenceV1[]=[],pending:string[]=[],",
  "const classes=new Map<string,number>(),opened=clock();\n let divergences:DivergenceV1[]=[],pending:string[]=[],",1),
 (RC+"🖥️server/🟦️.ts",
  'options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint});',
  'options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint},parsed.paths.includes("record.br")?"project-inference":parsed.manifest?"unit":parsed.paths.some(path=>path.startsWith("closure-"))?"closure":"other");',1),
 (RC+"🖥️server/🟦️.ts",
  "divergences?:Readonly<{count:number;recent:readonly DivergenceV1[]}>;",
  "divergences?:Readonly<{count:number;recent:readonly DivergenceV1[];since?:number;byClass?:Readonly<Record<string,number>>}>;",1),
 (RC+"🖥️server/🟦️.ts",
  "divergences:options.provenance?.summary()??{count:0,recent:[]},",
  "divergences:options.provenance?.summary()??{count:0,recent:[],since:0,byClass:{}},",1),
 (B+"🏃️execution/🧬️schema/🌐️remote-cache.json",
  '''            "count": { "type": "integer", "minimum": 0 },
            "recent": {
              "type": "array",
              "maxItems": 20,''',
  '''            "count": { "type": "integer", "minimum": 0 },
            "since": { "type": "integer", "minimum": 0 },
            "byClass": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },
            "recent": {
              "type": "array",
              "maxItems": 20,''',1),
]
