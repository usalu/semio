C="⚡️caching/"
TL=C+"🚀️bootstrap/🛠️tools/"
def plan(p,L):
    t=L+TL+"📥️use/🧪️tests/📬️task-child/🟦️.ts"
    p.sub(t,'JSON.stringify(join(dirname(sdkPackage),"semio-authorizers.mjs"))','JSON.stringify(join(process.env.SEMIO_NX_AUTHORIZER_ROOT!,"semio-authorizers.mjs"))')
    p.sub(t,"join(toolkit,'semio-authorizers.mjs')","join(process.env.SEMIO_NX_AUTHORIZER_ROOT,'semio-authorizers.mjs')")
    t=L+C+"🧪️tests/🌍️caller-environment/🟦️.ts"
    p.sub(t,"path.join(path.dirname(process.env.SEMIO_SCOPE_SDK),'semio-authorizers.mjs')","path.join(process.env.SEMIO_NX_AUTHORIZER_ROOT,'semio-authorizers.mjs')")
    t=L+TL+"📦️sdk/🧪️tests/📥️canonical/🟦️.ts"
    p.sub(t,'import{nxAuthorizerReceiptHoldsV1}from"../../../🔏️authorizers/📤️receipt/🟦️.ts";','import{newestNxAuthorizerBundleV1,nxAuthorizerBundleHoldsV1}from"../../../🔏️authorizers/📦️bundle/🟦️.ts";')
    p.sub(t,'assert.equal(nxAuthorizerReceiptHoldsV1(directory,marker.authorizers),true);\n  await withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot:directory,invocation:original},actualPhase,async()=>{\n   const frozen=await import(pathToFileURL(join(directory,"semio-authorizers.mjs")).href)','const bundle=newestNxAuthorizerBundleV1(dirname(dirname(directory)),identity);assert.ok(bundle!==undefined&&nxAuthorizerBundleHoldsV1(bundle,identity)!==null);\n  await withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot:directory,authorizer:basename(bundle),invocation:original},actualPhase,async()=>{\n   const frozen=await import(pathToFileURL(join(bundle,"semio-authorizers.mjs")).href)')
    t=L+C+"🧪️tests/🔒️nx/🟦️.ts"
    p.sub(t,'SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:identity,','SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_AUTHORIZER_ROOT:process.env.SEMIO_NX_AUTHORIZER_ROOT,SEMIO_NX_TOOLING_IDENTITY:identity,')
    t=L+"🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️purpose/🟦️.ts"
    p.sub(t,'import {join,basename} from "node:path";','import {join,basename,dirname} from "node:path";')
    p.sub(t,'import{withNxToolingUseV1,nxToolingUseStorageV1}from"@semio-tech/nx-bootstrap-tools/use";','import{withNxToolingUseV1,nxToolingUseStorageV1}from"@semio-tech/nx-bootstrap-tools/use";\nimport{newestNxAuthorizerBundleV1}from"@semio-tech/nx-bootstrap-tools/authorizers/bundle";')
    p.sub(t,'async function loadBazelNxParityAuthorizersV1(toolkitRoot:string):Promise<ParityAuthorizersV1>{const value=await import(join(toolkitRoot,"semio-authorizers.mjs"));','async function loadBazelNxParityAuthorizersV1(bundle:string):Promise<ParityAuthorizersV1>{const value=await import(join(bundle,"semio-authorizers.mjs"));')
    p.sub(t,'const identity=basename(toolkitRoot);\n return withRepositoryNxStorageV1(root,original,phaseControl,(bind,actualPhase)=>withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,invocation:original},actualPhase,async()=>{','const identity=basename(toolkitRoot),bundle=newestNxAuthorizerBundleV1(dirname(dirname(toolkitRoot)),identity);if(bundle===undefined)throw Error("Installed Nx authorizer bundle required");\n return withRepositoryNxStorageV1(root,original,phaseControl,(bind,actualPhase)=>withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,authorizer:basename(bundle),invocation:original},actualPhase,async()=>{')
    p.sub(t,'SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:identity,','SEMIO_NX_TOOLING_ROOT:toolkitRoot,SEMIO_NX_AUTHORIZER_ROOT:bundle,SEMIO_NX_TOOLING_IDENTITY:identity,')
    p.sub(t,'const frozen=await loadBazelNxParityAuthorizersV1(toolkitRoot);','const frozen=await loadBazelNxParityAuthorizersV1(bundle);')
    p.sub(L+TL+"package.json",'    "./authorizers/receipt": "./🔏️authorizers/📤️receipt/🟦️.ts",','    "./authorizers/receipt": "./🔏️authorizers/📤️receipt/🟦️.ts",\n    "./authorizers/bundle": "./🔏️authorizers/📦️bundle/🟦️.ts",')
    p.sub(L+"🏗️build/🔧️tools/🏗️bazel/🌿️environment/🏃️action/🧪️tests/🧫️fixtures/🔣️.json",'   "SEMIO_NX_COMMAND_DIRECTORY",\n','   "SEMIO_NX_AUTHORIZER_ROOT",\n   "SEMIO_NX_COMMAND_DIRECTORY",\n')
