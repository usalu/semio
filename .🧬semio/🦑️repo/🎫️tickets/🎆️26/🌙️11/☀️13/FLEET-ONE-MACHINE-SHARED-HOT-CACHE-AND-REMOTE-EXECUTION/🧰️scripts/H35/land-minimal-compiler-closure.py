import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from lib import edit,R
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
T=L+"🚀️bootstrap/🛠️tools/🧰️native/"
def importers():
    edit(T+"📜️script.ts",[('import {ownedNativeToolPortV1} from "./🔌️port/🟦️.ts";','import {ownedNativeToolPortV1} from "./🔌️port/🎟️owned/🟦️.ts";')])
    edit(L+"🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts",[('import {ownedNativeToolPortV1} from "../../🧰️native/🔌️port/🟦️.ts";','import {ownedNativeToolPortV1} from "../../🧰️native/🔌️port/🎟️owned/🟦️.ts";')])
    edit(L+"🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts",[('import { ownedNativeToolPortV1, systemNativeToolPortV1 } from "../../🛠️tools/🧰️native/🔌️port/🟦️.ts";','import { systemNativeToolPortV1 } from "../../🛠️tools/🧰️native/🔌️port/🟦️.ts";\nimport { ownedNativeToolPortV1 } from "../../🛠️tools/🧰️native/🔌️port/🎟️owned/🟦️.ts";')])
port=R+T+"🔌️port/🟦️.ts"
s=open(port,encoding="utf8").read()
marker="/** 🎟️ Runs archive extraction and pin probes under the exact held Original control and caller network authority. */"
new=R+T+"🔌️port/🎟️owned/🟦️.ts"
if marker in s:
    head,tail=s.split(marker)
    up9="../../../../../../../../../";up10="../"+up9
    owned=("""import {captureOwnedCommandV1,runOwnedCommand} from \""""+up10+"""🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import{fleetFetchContextV1,pinnedFetchV1}from"../../../../🛰️fleet-fetch/🟦️.ts";
import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1} from \""""+up10+"""🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ResourceLeaseControl} from \""""+up10+"""🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import type {NativeToolPortV1} from "../../🟦️.ts";

"""+marker.replace(" */"," Lives apart from the system port so the compiler store never reaches the JavaScript dependency bootstrap that the system port runs tools through.\n * @see ../🟦️.ts — `systemNativeToolPortV1`, the system half */")+"\n"+tail.lstrip("\n"))
    # keep one docstring block: merge
    keep=head.replace('import {captureOwnedCommandV1,runOwnedCommand} from "'+up9+'🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";\n',"").replace('import {readIssuedResourceControlCapabilitiesV1,requireIssuedResourceControlV1} from "'+up9+'🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";\n',"").replace('import type {ResourceLeaseControl} from "'+up9+'🔨️modules/🏃️process/🔒️leases/🟦️.ts";\n',"").replace('import {readNativeCallerAuthorityV1} from "../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";\n',"")
    assert keep.count("import")==len([l for l in keep.split("\n") if l.startswith("import")]) and "owned-execution" not in keep and "readNativeCallerAuthorityV1" not in keep,keep
    keep=keep.rstrip("\n")+"\n"
    if os.environ.get("DRY"):
        print(owned[:1500]);print("----");print(keep)
    else:
        os.makedirs(os.path.dirname(new),exist_ok=True)
        open(new+".tmp","w",encoding="utf8").write(owned);os.replace(new+".tmp",new)
        importers()
        edit(T+"🔌️port/🟦️.ts",[(s,keep)])
else:
    print("port already split")
    importers()
M=L+"🚀️bootstrap/📦️dependencies/🔌️managers/"
dcts=R+M+"🟨️.d.cts"
if not os.path.exists(dcts):
    body='''/** 🔌️ Types the plain-JavaScript manager accessor for TypeScript consumers that must not import the registry (and with it the whole dependency bootstrap). */
export interface ManagerSummaryV1{readonly id:string;readonly label:string;readonly ecosystem:string;readonly primaryLockName:string;readonly lockFileNames:readonly string[];readonly manifestFileNames:readonly string[];readonly scopeConfigName:string|null;configurationNames(role:"scope"|"home"|"config"):readonly string[];readonly generationDirectory:string;readonly hostEnvironment:readonly string[];readonly graphComponent:string;readonly ownedModules:readonly string[]}
declare const accessor:{managerV1(id:string):ManagerSummaryV1;selectedManagerV1(ecosystem:string,environment?:Readonly<Record<string,string|undefined>>):ManagerSummaryV1;allManagersV1():readonly ManagerSummaryV1[];sourceFilesV1():readonly string[]};
export=accessor;
'''
    if not os.environ.get("DRY"):
        open(dcts+".tmp","w",encoding="utf8").write(body);os.replace(dcts+".tmp",dcts)
edit(L+"🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts",[
('import {selectPackageManagerV1,packageManagerRegistryV1} from "../../../../../📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";\nconst javascriptManagerV1=selectPackageManagerV1("javascript"),javascriptLockV1=javascriptManagerV1.primaryLockName();',
 'import managersV1 from "../../../📦️dependencies/🔌️managers/🟨️.cjs";\nconst javascriptLockV1=managersV1.selectedManagerV1("javascript").primaryLockName;'),
])
