import os
R="/Users/ueli/Documents/semio/🧰️framework/"
S=os.path.dirname(os.path.abspath(__file__))+"/"
def load(p):return open(p,encoding="utf8").read()
def save(p,t):
    tmp=p+".f60.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
def once(t,old,new):
    assert t.count(old)==1,(old[:80],t.count(old));return t.replace(old,new)
lib=R+"🛍️products/🦑️repo/🔨️modules/📚️library/"
# product host-capacity
p=lib+"🏗️build/🌡️host-capacity/🟦️.ts";t=load(p)
if "createGrantGrowthPortV1" not in t:
    imports='''import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {totalmem} from "node:os";
import {hostAvailableBytesV1,hostInteractiveReserveMegabytesV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import type {GrantGrowthPortV1,GrowthHostV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";
import admissionPolicy from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json" with {type:"json"};
const GROWTH_POLICY_V1=admissionPolicy.definitions.Policy.const.growth;
'''
    a="import type {HostUpgradeOutcomeV1} from"
    i=t.index(a);j=t.index("\n",i)+1
    t=t[:j]+imports+t[j:]
    add=load(S+"growth_product.ts").replace("ADMISSION_POLICY_V1.growth.","GROWTH_POLICY_V1.")
    anchor="/** 🔭️ The megabytes a process tree holds right now"
    t=once(t,anchor,add+anchor)
    save(p,t)
# launcher
p=lib+"🏃️process/🦀️cargo/🚪️launcher/🟦️.ts";t=load(p)
if "createGrantGrowthPortV1" not in t:
    t=once(t,"const broker=await startAdmissionBrokerV1({physicalBytes:admissionMemoryShareV1(physical,local,availableParallelism()),reserveBytes:admissionReserveBytesV1(physical),parentMemoryAllowanceBytes,history,","const broker=await startAdmissionBrokerV1({physicalBytes:admissionMemoryShareV1(physical,local,availableParallelism()),reserveBytes:admissionReserveBytesV1(physical),parentMemoryAllowanceBytes,growth:(await import(\"../../../🏗️build/🌡️host-capacity/🟦️.ts\")).createGrantGrowthPortV1({environment}),history,")
    save(p,t)
# unit-cache script
p=lib+"⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";t=load(p)
if "localHoldableMegabytesV1" not in t:
    t=once(t,"learned.length>0?Math.max(...learned):undefined,Math.round(totalmem()/1048576));}","learned.length>0?Math.max(...learned):undefined,Math.round(totalmem()/1048576),admission.localHoldableMegabytesV1(Math.round(totalmem()/1048576)));}")
    save(p,t)
print("applied product")
