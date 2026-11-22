F="⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts"
def plan(p,L):
    f=L+F
    p.sub(f,'import {join} from "node:path";\nimport {openNxSdkV1} from "../🔌️sdk/🟦️.ts";','import {basename,dirname,join} from "node:path";\nimport {openNxSdkV1} from "../🔌️sdk/🟦️.ts";\nimport {newestNxAuthorizerBundleV1} from "../🔏️authorizers/📦️bundle/🟦️.ts";')
    p.sub(f,'   delete nx.NX_FORCE_REUSE_CACHED_GRAPH;\n','   delete nx.NX_FORCE_REUSE_CACHED_GRAPH;\n   const generation=dirname(dirname(sdk.packageDirectory())),bundle=nx.SEMIO_NX_AUTHORIZER_ROOT??newestNxAuthorizerBundleV1(dirname(dirname(generation)),basename(generation));if(bundle!==undefined)nx.SEMIO_NX_AUTHORIZER_ROOT=bundle;\n')
