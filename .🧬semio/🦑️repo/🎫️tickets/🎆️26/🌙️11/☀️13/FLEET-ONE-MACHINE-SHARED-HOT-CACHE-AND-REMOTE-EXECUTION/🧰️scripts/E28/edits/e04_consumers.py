C="⚡️caching/"
TL=C+"🚀️bootstrap/🛠️tools/"
def plan(p,L):
    c=L+C;tl=L+TL
    r=tl+"📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts"
    p.sub(r,'import{nxAuthorizerReceiptHoldsV1}from"../../../../🔏️authorizers/📤️receipt/🟦️.ts";\n','')
    p.sub(r,'for(const name of[".semio-authorizer-source.json",".semio-authorizer-compiler.json",".semio-sdk-material.json"])files.set(','for(const name of[".semio-sdk-material.json"])files.set(')
    p.sub(r,'||nxToolingInstallIdentityV1(environment,files)!==identity||!nxAuthorizerReceiptHoldsV1(generation,marker.authorizers))throw Error("Authenticated derivative material reference required")','||nxToolingInstallIdentityV1(environment,files)!==identity)throw Error("Authenticated derivative material reference required")')
    r=tl+"📦️sdk/📥️source/🧊️pristine/🟦️.ts"
    p.sub(r,'import{nxAuthorizerReceiptHoldsV1}from"../../../🔏️authorizers/📤️receipt/🟦️.ts";\n','')
    p.sub(r,'for(const name of[".semio-authorizer-source.json",".semio-authorizer-compiler.json",".semio-sdk-material.json"])generationInputs.set(','for(const name of[".semio-sdk-material.json"])generationInputs.set(')
    p.sub(r,'!==marker.digest||!nxAuthorizerReceiptHoldsV1(current,marker.authorizers))throw Error("Authenticated current SDK artifact required")','!==marker.digest)return false')
    r=tl+"📦️sdk/📊️material/🟦️.ts"
    p.sub(r,'import{nxAuthorizerDigestManifestPolicyV1 as authorizerDigestManifestPolicy,nxAuthorizerReceiptHoldsV1}from"../../🔏️authorizers/📤️receipt/🟦️.ts";\n','')
    p.sub(r,'const member=basename(path),maximumBytes=Math.min(remainingBytes,authorizerDigestManifestPolicy.names.includes(member)?authorizerDigestManifestPolicy.maximumBytes:protocol.maximumIdentityInputFileBytes);','const maximumBytes=Math.min(remainingBytes,protocol.maximumIdentityInputFileBytes);')
    p.sub(r,'for(const name of[".semio-authorizer-source.json",".semio-authorizer-compiler.json",".semio-sdk-material.json"])files.set(','for(const name of[".semio-sdk-material.json"])files.set(')
    p.sub(r,'if(marker.version!==1||marker.digest!==digest||!nxAuthorizerReceiptHoldsV1(generation,marker.authorizers))throw Error("Authenticated SDK generation material reference required")','if(marker.version!==1||marker.digest!==digest)throw Error("Authenticated SDK generation material reference required")')
    p.sub(tl+"📦️sdk/🎟️capacity/🧬️schema.json",'[.]semio-authorizer-source[.]json|[.]semio-authorizer-compiler[.]json|','')
    r=tl+"🧹️retention/📦️generation/🧬️schema/🔣️.json"
    p.sub(r,'          ".semio-authorizer-compiler.json",\n          ".semio-authorizer-source.json",\n','')
    p.sub(r,'          "package.json",\n          "patches",\n          "semio-authorizers.mjs"\n','          "package.json",\n          "patches"\n')
    r=c+"🕸️graph/🔏️seal/📥️membership/📥️journal/🌐️rpc/🟦️.ts"
    p.sub(r,'import {nxAuthorizerReceiptHoldsV1} from "../../../../../🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts";\n','')
    p.sub(r,'      if(!nxAuthorizerReceiptHoldsV1(directory,marker.authorizers))return null;\n','')
    r=tl+"📥️use/🟦️.ts"
    p.sub(r,'import{withNxSdkDerivativeReaderV1}from"../📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts";','import{withNxSdkDerivativeReaderV1}from"../📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts";\nimport{nxAuthorizerBundlePolicyV1,nxAuthorizerBundleToolkitV1}from"../🔏️authorizers/📦️bundle/🟦️.ts";')
    p.sub(r,'export type NxToolingUseAuthorityV1={readonly identity:string;readonly toolkitRoot:string;readonly invocation:ScriptInvocation;','export type NxToolingUseAuthorityV1={readonly identity:string;readonly toolkitRoot:string;readonly authorizer?:string;readonly invocation:ScriptInvocation;')
    p.sub(r,'/** 🎟️ Keeps one exact immutable toolkit in use through its original owned operation. */','/** 🎟️ Keeps one exact immutable toolkit, and the authorizer bundle that runs beside it, in use through its original owned operation. */')
    p.sub(r,'if(!/^[a-f0-9]{64}$/u.test(identity)||!isAbsolute(authority.toolkitRoot)','if(!/^[a-f0-9]{64}$/u.test(identity)||authority.authorizer!==undefined&&!/^[a-f0-9]{64}$/u.test(authority.authorizer)||!isAbsolute(authority.toolkitRoot)')
    p.sub(r,'resources:[{resource:"nx-tooling:"+identity,mode:"shared"}]},async()=>{const root=callerRepositoryRootV1(invocation.capabilities);','resources:[{resource:"nx-tooling:"+identity,mode:"shared"},...(authority.authorizer===undefined?[]:[{resource:nxAuthorizerBundlePolicyV1.lease+authority.authorizer,mode:"shared" as const}])]},async()=>{const root=callerRepositoryRootV1(invocation.capabilities);')
    p.sub(r,'/** 🧷️ The actual immutable module owns its independent process use until the original task body closes. */','/** 🧷️ The actual immutable module owns its independent process use, of its own bundle directory and of the dependency generation beside it, until the original task body closes. */')
    p.sub(r,' const toolkitRoot=dirname(fileURLToPath(import.meta.url)),identity=basename(toolkitRoot);',' const{identity,toolkitRoot,authorizer}=nxAuthorizerBundleToolkitV1(dirname(fileURLToPath(import.meta.url)));')
    p.sub(r,'withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,invocation:original},actualPhase,async()=>{','withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot,authorizer,invocation:original},actualPhase,async()=>{')
    r=C+"🚀️bootstrap/📜️script.ts"
    p.sub(L+r,'{identity:toolingIdentity,toolkitRoot:dirname(tooling.modulePath),invocation:this.invocation}','{identity:toolingIdentity,toolkitRoot:dirname(tooling.modulePath),authorizer:basename(dirname(tooling.authorizer)),invocation:this.invocation}')
    p.sub(L+r,'SEMIO_NX_TOOLING_ROOT: dirname(tooling.modulePath), ','SEMIO_NX_TOOLING_ROOT: dirname(tooling.modulePath), SEMIO_NX_AUTHORIZER_ROOT: dirname(tooling.authorizer), ')
    r=c+"🕸️graph/🏁️startup/🚦️service/🟦️.ts"
    p.sub(r,'withNxToolingUseV1(nxToolingUseStorageV1(root),{identity:toolingIdentity,toolkitRoot,invocation:original}','withNxToolingUseV1(nxToolingUseStorageV1(root),{identity:toolingIdentity,toolkitRoot,...(environment.SEMIO_NX_AUTHORIZER_ROOT?{authorizer:basename(environment.SEMIO_NX_AUTHORIZER_ROOT)}:{}),invocation:original}')
    p.sub(r,'{identity:birth.toolingIdentity,toolkitRoot:birth.toolkitRoot,invocation:original,reader:','{identity:birth.toolingIdentity,toolkitRoot:birth.toolkitRoot,...(process.env.SEMIO_NX_AUTHORIZER_ROOT?{authorizer:basename(process.env.SEMIO_NX_AUTHORIZER_ROOT)}:{}),invocation:original,reader:')
    p.sub(r,'SEMIO_NX_TOOLING_ROOT:birth.toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:birth.toolingIdentity,','SEMIO_NX_TOOLING_ROOT:birth.toolkitRoot,SEMIO_NX_AUTHORIZER_ROOT:process.env.SEMIO_NX_AUTHORIZER_ROOT,SEMIO_NX_TOOLING_IDENTITY:birth.toolingIdentity,')
    r="🧪️testing/🌱️genuine-birth/🟦️.ts"
    p.sub(L+r,'import { basename, isAbsolute, join, relative, resolve } from "node:path";','import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";')
    p.sub(L+r,'''      const toolkitRoot = await realpath(machinePathV1(root, ".nx/installation")), identity = basename(toolkitRoot);
      return withNxToolingUseV1(nxToolingUseStorageV1(root), { identity, toolkitRoot, invocation: original }, phase, () => start(original, { SEMIO_NX_TOOLING_ROOT: toolkitRoot, SEMIO_NX_TOOLING_IDENTITY: identity }));''','''      const { newestNxAuthorizerBundleV1 } = await import("../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️bundle/🟦️.ts");
      const toolkitRoot = await realpath(machinePathV1(root, ".nx/installation")), identity = basename(toolkitRoot), bundle = newestNxAuthorizerBundleV1(dirname(dirname(toolkitRoot)), identity);
      if (bundle === undefined) throw Error("Installed Nx authorizer bundle required");
      return withNxToolingUseV1(nxToolingUseStorageV1(root), { identity, toolkitRoot, authorizer: basename(bundle), invocation: original }, phase, () => start(original, { SEMIO_NX_TOOLING_ROOT: toolkitRoot, SEMIO_NX_TOOLING_IDENTITY: identity, SEMIO_NX_AUTHORIZER_ROOT: bundle }));''')
