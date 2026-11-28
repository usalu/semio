T = OFF + "/🧪️tests/🟦️.ts"
edit(T, 'import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";', 'import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";')
edit(T, 'import {decodeProtobufV1,REAPI_V1} from', 'import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from')
edit(T, 'import {offloadCheckoutTestV1} from "../🔌️client/🟦️.ts";', 'import {followCheckoutTestChunksV1,offloadCheckoutTestV1} from "../🔌️client/🟦️.ts";')
edit(T, 'import {checkoutTestGateV1} from "../🚦️gate/🟦️.ts";', 'import {checkoutTestGateV1,checkoutTestLoadPerCoreV1,checkoutTestLoadVerdictV1,readCheckoutTestMeasuresV1,recordCheckoutTestMeasureV1} from "../🚦️gate/🟦️.ts";')
edit(T, 'checkoutTestTextV1,validateCheckoutTestActionV1,type CheckoutTestCutV1} from "../📐️policy/🟦️.ts";', 'checkoutTestTextV1,validateCheckoutTestActionV1,checkoutTestChunkKeyV1,decodeCheckoutTestChunkV1,encodeCheckoutTestChunkV1,type CheckoutTestChunkV1,type CheckoutTestCutV1} from "../📐️policy/🟦️.ts";')
edit(T, 'import {createCheckoutTestsV1} from "../👷️worker/🟦️.ts";', 'import {createCheckoutTestChunkWriterV1,createCheckoutTestsV1} from "../👷️worker/🟦️.ts";')
edit(T, 'path:boolean[];gate:{offload:boolean;reason:string;asked:string[]}[]}=>{', 'path:boolean[];gate:{offload:boolean;reason:string;asked:string[]}[];load:string[]}=>{')
APPEND = {T: "tests-checkout.ts"}
V = OFF + "/🧪️tests/🔣️.json"
edit(V, '  "gate": [\n', open(os.path.join(HERE, "staged", "vectors-load.json"), encoding="utf-8").read() + '  "gate": [\n')
Y = OFF + "/🧪️tests/🐍️.py"
edit(Y, '''    if row["mode"] == "auto":
        asked.append("host")
        if row["host"] != "wait":
            return {"offload": False, "reason": "local-slot", "asked": asked}
        reason = "host-wait"''', '''    if row["mode"] == "auto":
        asked.append("host")
        if row["host"] != "wait":
            if row.get("load") is None:
                return {"offload": False, "reason": "local-slot", "asked": asked}
            asked.append("load")
            if row["load"] != "offload":
                return {"offload": False, "reason": "local-slot", "asked": asked}
            reason = "host-load"
        else:
            reason = "host-wait"''')
edit(Y, 'request = json.load(sys.stdin)', '''def load_verdict(row, policy):
    load = row["loadPerCore"]
    if load >= policy["perCoreAlways"]:
        return "offload"
    if load < policy["perCoreMeasured"]:
        return "keep"
    def fresh(timing):
        return timing is not None and timing["ageSeconds"] <= policy["maximumAgeSeconds"]
    local, remote = row["local"], row["remote"]
    if fresh(local) and fresh(remote) and remote["seconds"] >= local["seconds"] * policy["remoteWorthRatio"]:
        return "keep"
    return "offload"

request = json.load(sys.stdin)''')
edit(Y, '''    "cover": [covers(row["need"], row["have"]) for row in request["cover"]],
    "path": [valid_path(row["path"]) for row in request["path"]],
    "gate": [gate(row) for row in request["gate"]],''', '''    "cover": [covers(row["need"], row["have"]) for row in request.get("cover", [])],
    "path": [valid_path(row["path"]) for row in request.get("path", [])],
    "gate": [gate(row) for row in request.get("gate", [])],
    "load": [load_verdict(row, request["policy"]) for row in request.get("load", [])],''')
RT = TF + "/🧪️tests/🌐️remote/🟦️.ts"
edit(RT, 'import {expect,test} from "bun:test";\nimport {existsSync} from "node:fs";', 'import {afterAll,expect,test} from "bun:test";\nimport {existsSync,mkdtempSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";')
edit(RT, 'type Offload=typeof offloadCheckoutTestV1;', 'type Offload=typeof offloadCheckoutTestV1;\nconst sharedMemo=mkdtempSync(join(tmpdir(),"semio-h44-"));\nafterAll(()=>{rmSync(sharedMemo,{recursive:true,force:true});});')
edit(RT, '{host:async()=>"wait",idle:async()=>2,offload}))!;', '{memoRoot:sharedMemo,host:async()=>"wait",idle:async()=>2,offload}))!;')
APPEND[RT] = "tests-remote.ts"
ST = OFF + "/🧪️tests/🌐️scheduler/🟦️.ts"
edit(ST, 'const ask=(file:string,signal?:AbortSignal)=>offloadCheckoutTestV1({root:temporary(),memoRoot:temporary(),environment:{},file,budgetSeconds:30,language:"en",retryMilliseconds:0,...signal?{signal}:{},ports',
         'const ask=(file:string,signal?:AbortSignal,onOutput?:(text:string)=>void)=>offloadCheckoutTestV1({root:temporary(),memoRoot:temporary(),environment:{},file,budgetSeconds:30,language:"en",retryMilliseconds:0,...signal?{signal}:{},...onOutput?{onOutput}:{},ports')
edit(ST, '  expect(cluster.events.some(line=>line.startsWith("refused")&&line.includes("not-ready"))',
         '''  const live:string[]=[],streamed=await ask("a/ok.ts",undefined,text=>{live.push(text);});
  expect(streamed.kind).toBe("done");
  if(streamed.kind==="done"){expect(streamed.streamed,"the pieces of the output went through the real cache server").toBe(true);expect(live.join("")).toContain("ran a/ok.ts");expect(streamed.stderr).toBe("");}
  expect(cluster.events.some(line=>line.startsWith("refused")&&line.includes("not-ready"))''')
