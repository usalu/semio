L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
C=L+"⚡️caching/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)
U=C+'🦀️cargo/🗃️unit-cache/🛰️fleet/'
edit(U+'🟦️.ts',[
('import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "./📡️client/🟦️.ts";','import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "./📡️client/🟦️.ts";\nimport {fleetCacheRootV1,remoteCacheFetchV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";'),
('const client=world.client??createFleetClientV1(settings);if(!client.usable())return undefined;\n  const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined;','const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined,identityRoot=workspace??fleetCacheRootV1(input.environment);\n  const client=world.client??createFleetClientV1(settings,identityRoot===undefined?{}:{fetch:remoteCacheFetchV1(input.environment,{root:identityRoot})});if(!client.usable())return undefined;'),
])
edit(U+'📤️uploader/🟦️.ts',[
('import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../📡️client/🟦️.ts";','import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../📡️client/🟦️.ts";\nimport {remoteCacheFetchV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";'),
(' const client=createFleetClientV1(settings),idle=',' const client=createFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||root})}),idle='),
])
edit(B+'🪪️graph-state/🛰️exchange/🟦️.ts',[('export function graphFleetClientV1(settings:GraphFleetSettingsV1,world?:Readonly<{fetch?:typeof fetch}>):GraphFleetClientV1{return createGraphFleetClientV1(settings,world);}','export function graphFleetClientV1(settings:GraphFleetSettingsV1,world?:Readonly<{fetch?:typeof fetch}>):GraphFleetClientV1{return createGraphFleetClientV1(settings,world);}')])
print("ok3")
