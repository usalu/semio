# Preparation Actual Semantic Baseline

One approved normal successor existing-API diagnostic executed, direct terminal chunk171f4e, outer1. All125 captured inputs matched prelaunch, terminal and current verification. The raw caller ledger is an existing-API diagnostic, not an issued Runtime loan or Host Source authority. The first fixture-decoding failure stays immutable. No Source body edit or second attempt.

## Actual complete output

[tools] bun waits for machine capacity.
[capacity] admitted command: 1 job(s), 2048 MB reserved (20126 MB available, 4096 MB floor)
bun test v1.4.2 (744846f84)

.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/drawing-scene-preparation-zero-2.test.ts:
 8 | import {RetainedWorkLedger} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
 9 | import rows from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json";
10 | import law from "./📐️.json";
11 | import schema from "./🔣️.json";
12 | function lift(value:unknown):unknown{return typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;}
13 | test("actual Drawing preparation zero authority preserves public semantic progress before any new API",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const row=rows.find(value=>value.name===law.zeroAuthority.source);if(row===undefined)throw Error("Original preparation source missing");const source=parseDrawingArtifact({...row.document,layers:lift(row.document.layers)}),layers=source.layers,assets=source.assets,before=structuredClone(row.document),job=new DocumentSceneJob(source,row.limits),work=new RetainedWorkLedger(law.zeroAuthority.grant),db=new Database(":memory:"),errors:unknown[]=[];try{const actual=Reflect.apply(job.advance,job,[law.zeroAuthority.items,work]);const expected=db.query("SELECT ? AS work").get(law.zeroAuthority.expectedWork);expect({work:actual.work}).toEqual(expected);expect(work.receipt).toEqual(law.zeroAuthority.expectedReceipt);}catch(error){errors.push(error);}finally{try{job.cancel();expect(source.layers).toBe(layers);expect(source.assets).toBe(assets);expect(row

error: expect(received).toEqual(expected)

  {
-   "work": 0,
+   "work": 1,
  }

- Expected  - 1
+ Received  + 1

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🎟️owned/🧪️tests/zero-authority.test.ts:13:1243)
(fail) actual Drawing preparation zero authority preserves public semantic progress before any new API [22.04ms]

 0 pass
 1 fail
 5 expect() calls
Ran 1 test across 1 file. [116.00ms]


## Actual terminal receipt

{
  "session": null,
  "exitCode": 1,
  "terminalChunk": "171f4e",
  "log": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/🗑️generated/drawing-scene-preparation-zero-2/test/test.log",
  "bytes": 2509,
  "sha256": "d4587947c87bcc30420bf50434adf376e2a3198436c70b317620181658e89397"
}
