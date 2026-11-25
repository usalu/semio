import json,re,os
os.chdir("/Users/ueli/Documents/semio")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/"
N="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/"
PK="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts"
def edit(p,pairs):
    t=open(p,encoding="utf8").read()
    for a,b in pairs:
        assert t.count(a)==1,(p,a[:60],t.count(a));t=t.replace(a,b)
    open(p,"w",encoding="utf8").write(t)
edit(N+"🧹️cleanup/📋️state/🟦️.ts",[
 ("export function nativeImageOperationAdmissionV1(","export const nativeImageOperationSlotsV1:number=operationSchema.definitions.Protocol.const.maximumPendingOperations;\nexport function nativeImageOperationAdmissionV1("),
 ("row.pending<operationSchema.definitions.Protocol.const.maximumPendingOperations","row.pending<nativeImageOperationSlotsV1")])
edit(PK,[("export { withNativeImageTerminalCloseV1 }","export { nativeImageOperationAdmissionV1, nativeImageOperationSlotsV1, withNativeImageTerminalCloseV1 }")])
edit(N+"📥️borrower/🟦️.ts",[
 ("const maximum=schema.definitions.Protocol.const.maximumPendingOperations,","const "),
 ("!current&&scope.open&&scope.top.size>=maximum","!current&&nativeImageOperationAdmissionV1({pending:scope.top.size,topOpen:scope.open,nestedLive:false}).waits"),
 ("current&&current.live&&current.pending.size>=maximum&&nativeImageOperationAdmissionV1(","current&&current.live&&nativeImageOperationAdmissionV1("),
 ("(!scope.open||scope.top.size<maximum)","(!scope.open||nativeImageOperationAdmissionV1({pending:scope.top.size,topOpen:true,nestedLive:false}).accepts)"),
 ("frame.pending.size<maximum)","nativeImageOperationAdmissionV1({pending:frame.pending.size,topOpen:false,nestedLive:true}).accepts)")])
A=L+"📥️opening/🚦️admission/"
edit(A+"🟦️.ts",[
 ('import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import {nativeImageOperationAdmissionV1,nativeImageOperationSlotsV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts";\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};'),
 ("export const partitionOperationSlotsV1:number=policy.maximumPending;","export const partitionOperationSlotsV1:number=nativeImageOperationSlotsV1;\n/** 🚦️ Whether one more top-level operation fits beside `busy` others; the only decision, taken by the Native operation admission shared with the borrower. */\nexport function partitionHasRoomV1(busy:number):boolean{return nativeImageOperationAdmissionV1({pending:busy,topOpen:true,nestedLive:false}).accepts;}"),
 ("busy():number;limit?:number;","busy():number;"),
 ("const limit=wait.limit??partitionOperationSlotsV1,started","const limit=partitionOperationSlotsV1,started"),
 ("while(wait.busy()>=limit){","while(!partitionHasRoomV1(wait.busy())){")])
edit(L+"📥️opening/🟦️.ts",[
 ("load(admitted)<partitionOperationSlotsV1","partitionHasRoomV1(load(admitted))"),
 ("import {awaitPartitionSlotV1,partitionFailureTextV1,partitionOperationSlotsV1,partitionRefusalV1}","import {awaitPartitionSlotV1,partitionFailureTextV1,partitionHasRoomV1,partitionRefusalV1}")])
sp=A+"🧬️schema/🔣️.json";d=json.load(open(sp,encoding="utf8"))
d["definitions"]["Policy"]["required"].remove("maximumPending");del d["definitions"]["Policy"]["properties"]["maximumPending"];del d["definitions"]["Protocol"]["const"]["maximumPending"]
open(sp,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
print('product ok')

LB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
for f in [L+"🕸️graph/📏️headroom/🧪️tests/🟦️.ts",L+"🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts",LB+"🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/⚠️scoped-retirement-and-bridge/🟦️.ts"]:
    edit(f,[("partitionOperationSlotsV1:admission.partitionOperationSlotsV1,","partitionOperationSlotsV1:admission.partitionOperationSlotsV1,partitionHasRoomV1:admission.partitionHasRoomV1,")])
edit(A+"🧪️tests/🟦️.ts",[("import{awaitPartitionSlotV1,partitionFailureTextV1,partitionOperationSlotsV1,","import{awaitPartitionSlotV1,partitionFailureTextV1,partitionHasRoomV1,partitionOperationSlotsV1,"),("awaitPartitionSlotV1,partitionFailureTextV1,partitionOperationSlotsV1,partitionRefusalV1};","awaitPartitionSlotV1,partitionFailureTextV1,partitionHasRoomV1,partitionOperationSlotsV1,partitionRefusalV1};")])
print("tests ok")
