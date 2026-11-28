import sys,os;sys.path.insert(0,os.path.dirname(__file__))
from lib import edit
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
C=L+"⚡️caching/"
S=C+"🚀️bootstrap/🛠️tools/📦️sdk/"
for f,ups in [("🎟️capacity/📥️physical","../../../../../../"),("📊️material","../../../../../"),("📥️source/🔗️derivative/📥️reader","../../../../../../../"),("📥️source/🔗️derivative","../../../../../../")]:
    old='import {linuxHostLibcV1} from "'+ups+'🎛️commands/🔧️setup/🟦️.ts";'
    edit(S+f+"/🟦️.ts",[(old,old.replace("🔧️setup/🟦️.ts","🔧️setup/🐧️libc/🟦️.ts"))])
G=C+"💾️storage/🔒️enforcement/🛠️tools/🐹️go/🟦️.ts"
edit(G,[
("import { checkResourceLeaseControl } from","import { checkResourceLeaseControl, type ResourceLeaseControl } from"),
('import { systemNativeToolPortV1 } from "../../../../🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts";','import type { NativeToolPortV1 } from "../../../../🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";'),
("overlay?: () => Readonly<Record<string, string>> } = {}): Promise<void> {","overlay?: () => Readonly<Record<string, string>> }, port: (actual: ResourceLeaseControl, offline: boolean) => NativeToolPortV1): Promise<void> {"),
("systemNativeToolPortV1(workspace, source.CARGO_NET_OFFLINE === \"true\"), actual.signal);","port(actual, source.CARGO_NET_OFFLINE === \"true\"), actual.signal);"),
])
edit(L+"🧪️testing/🐹️go-plan/🟦️.ts",[("{...opts,overlay:()=>canonicalGoPlan(moduleRoot).replacements});","{...opts,overlay:()=>canonicalGoPlan(moduleRoot).replacements},(_actual,offline)=>systemNativeToolPortV1(workspace,offline));")])
edit(C+"💾️storage/🔒️enforcement/🏗️compiler/🛠️native-compiler/🪟️windows/🟦️.ts",[
('import{runRepositoryToolingGoBuildV1}from"../../../🛠️tools/🐹️go/🟦️.ts";','import{runRepositoryToolingGoBuildV1}from"../../../🛠️tools/🐹️go/🟦️.ts";\nimport{ownedNativeToolPortV1}from"../../../../🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🎟️owned/🟦️.ts";'),
('SEMIO_TEST_ARTIFACT_DIR:captureDir}});if(await source()','SEMIO_TEST_ARTIFACT_DIR:captureDir}},control=>ownedNativeToolPortV1(store.environment,control));if(await source()'),
])
