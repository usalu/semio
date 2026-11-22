F="⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"
def plan(p,L):
    f=L+F
    t=p.load(f)
    s=t.index('    const captured=Buffer.from(JSON.stringify(snapshotNxAuthorizerSourcesV1(authorizer))')
    e=t.index('rmSync(program,{force:true});}\n',s)+len('rmSync(program,{force:true});}\n')
    block=t[s:e]
    assert block.count("\n")==2,block.count("\n")
    p.sub(f,block,"")
    p.sub(f,'import {nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";\n','import {nxAuthorizerBundleIdentityV1,nxToolingInstallIdentityV1} from "./📦️sdk/🪪️identity/🟦️.ts";\n')
    p.sub(f,'import {nxAuthorizerDigestManifestPolicyV1,nxAuthorizerReceiptHoldsV1} from "./🔏️authorizers/📤️receipt/🟦️.ts";','import {nxAuthorizerDigestManifestPolicyV1,nxAuthorizerReceiptHoldsV1} from "./🔏️authorizers/📤️receipt/🟦️.ts";\nimport {nxAuthorizerBundleHoldsV1,nxAuthorizerBundlePolicyV1,touchNxAuthorizerBundleV1} from "./🔏️authorizers/📦️bundle/🟦️.ts";')
    p.sub(f,'export type NxTooling = { readonly cli: string; readonly modulePath: string; readonly bun: string; readonly authorizer:string };','export type NxTooling = { readonly cli: string; readonly modulePath: string; readonly bun: string; readonly authorizer:string };\n/** 🧱️ The installed dependency generation alone: the authorizer bundle is a separate directory beside it. */\ntype NxToolingGenerationV1 = Omit<NxTooling,"authorizer">;')
    p.sub(f,'if(marker.version!==1||marker.digest!==basename(selected)||!nxAuthorizerReceiptHoldsV1(selected,marker.authorizers))throw Error("Original immutable activation receipt required");await preflightNxAuthorizersV1(selected,control,store.environment);await store.recheck();check();','if(marker.version!==1||marker.digest!==basename(selected))throw Error("Original immutable activation receipt required");const bundle=dirname(tooling.authorizer);if(nxAuthorizerBundleHoldsV1(bundle,marker.digest)===null)throw Error("Original immutable authorizer bundle receipt required");await preflightNxAuthorizersV1(bundle,control,store.environment);await store.recheck();check();')
    p.sub(f,'||previousMarker.digest!==basename(previous)||!nxAuthorizerReceiptHoldsV1(previous,previousMarker.authorizers))throw Error("The Nx installation link has another owner")','||previousMarker.digest!==basename(previous))throw Error("The Nx installation link has another owner")')
    p.sub(f,'  const authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);','  const authorizerFiles=new Map<string,Buffer>(),authorizer=await captureNxAuthorizerSourcesV1(root,RECIPE+"/🔏️authorizers/📦️runtime/🟦️.ts",signal);')
    p.sub(f,'  files.set(".semio-authorizer-source.json",Buffer.from(','  authorizerFiles.set(".semio-authorizer-source.json",Buffer.from(')
    p.sub(f,'  files.set(".semio-authorizer-compiler.json",Buffer.from(','  authorizerFiles.set(".semio-authorizer-compiler.json",Buffer.from(')
    p.sub(f,'if(files.get(name)!.byteLength>nxAuthorizerDigestManifestPolicyV1.maximumBytes)','if(authorizerFiles.get(name)!.byteLength>nxAuthorizerDigestManifestPolicyV1.maximumBytes)')
    p.sub(f,'return{root,files,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest};','return{root,files,authorizerFiles,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest};')
    p.sub(f,'const{root,files,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest}=await captureNxToolingInputsV1(workspace,signal),bun=','const{root,files,authorizerFiles,manifest,dependencyFiles,authorizer,compiler,environment,sdkDigest}=await captureNxToolingInputsV1(workspace,signal),bun=')
    p.sub(f,'const installed = async(directory: string): Promise<NxTooling> =>','const installed = async(directory: string): Promise<NxToolingGenerationV1> =>')
    p.sub(f,'    if(!nxAuthorizerReceiptHoldsV1(directory,marker.authorizers)||marker.authorizers.sourceSha256!==authorizer.digest)throw new Error("Nx immutable authorizer receipt changed");\n','')
    p.sub(f,'    await preflightNxAuthorizersV1(directory,control,store.environment);await store.recheck();\n    return { cli: installedBinaryV1(directory, "nx", "nx"), modulePath, bun,authorizer:join(directory,"semio-authorizers.mjs") };','    await store.recheck();\n    return { cli: installedBinaryV1(directory, "nx", "nx"), modulePath, bun };')
    p.sub(f,'  if (present) return withResourceLeases({directory:machinePathV1(root,".cache/b/l/m/data"),resources:[{resource:"nx-tooling:"+digest,mode:"shared"}],control,signal},()=>installed(destination));','''  const ready=async(generation:NxToolingGenerationV1|ToolingHeadroomRefusalV1):Promise<NxTooling|ToolingHeadroomRefusalV1>=>{if(isToolingHeadroomRefusalV1(generation))return generation;const bundle=await ensureNxAuthorizerBundleV1(root,store,publisher,control,bun,digest,authorizer,compiler,authorizerFiles);return isToolingHeadroomRefusalV1(bundle)?bundle:{...generation,authorizer:bundle};};
  if (present) return ready(await withResourceLeases({directory:machinePathV1(root,".cache/b/l/m/data"),resources:[{resource:"nx-tooling:"+digest,mode:"shared"}],control,signal},()=>installed(destination)));''')
    p.sub(f,'const publish=async(quote:NxSdkClonePublicationV1):Promise<NxTooling|ToolingHeadroomRefusalV1>=>{','const publish=async(quote:NxSdkClonePublicationV1):Promise<NxToolingGenerationV1|ToolingHeadroomRefusalV1>=>{')
    p.sub(f,'    const authorizers={version:1,mode:"release",sourceSha256:authorizer.digest,entry:"semio-authorizers.mjs",outputs:[{path:"semio-authorizers.mjs",sha256:createHash("sha256").update(readFileSync(join(staging,"semio-authorizers.mjs"))).digest("hex")}]};\n','')
    p.sub(f,'JSON.stringify({version:1,digest,patches,authorizers,binaries:{','JSON.stringify({version:1,digest,patches,binaries:{')
    p.sub(f,'  return withNxSdkDerivativePublicationV1(root,store,projection,capturedInputs,control,quote=>publish(quote),{requireNxSdkDerivativeProjectionV1,copyNxSdkDerivativeV1});','  return ready(await withNxSdkDerivativePublicationV1(root,store,projection,capturedInputs,control,quote=>publish(quote),{requireNxSdkDerivativeProjectionV1,copyNxSdkDerivativeV1}));')
    p.sub(f,'{resource:"nx-tooling:"+basename(dirname(tooling.modulePath)),mode:"shared"}],control,signal:control.signal},()=>withNxSdkDerivativeReaderV1(','{resource:"nx-tooling:"+basename(dirname(tooling.modulePath)),mode:"shared"},{resource:nxAuthorizerBundlePolicyV1.lease+basename(dirname(tooling.authorizer)),mode:"shared"}],control,signal:control.signal},()=>withNxSdkDerivativeReaderV1(')
    p.sub(f,'publicationRequired=!(marker.version===1&&marker.digest===identity&&marker.authorizers?.sourceSha256===captured.authorizer.digest&&nxAuthorizerReceiptHoldsV1(generation,marker.authorizers));','publicationRequired=!(marker.version===1&&marker.digest===identity);')
    fn='''/** 🔖️ Provisions the compiled authorizer bundle beside one dependency generation as its own content-addressed directory, verified and published separately from the generation: a library edit costs one compile of the bundle and never a new installation. */
async function ensureNxAuthorizerBundleV1(root:string,store:RepositoryToolingStoreV1,publisher:ManagedToolingPublisherV1,control:ResourceLeaseControl,bun:string,dependencies:string,authorizer:Awaited<ReturnType<typeof captureNxAuthorizerSourcesV1>>,compiler:Awaited<ReturnType<typeof captureNxAuthorizerSourcesV1>>,files:ReadonlyMap<string,Buffer>):Promise<string|ToolingHeadroomRefusalV1>{
  const signal=control.signal,identity=nxAuthorizerBundleIdentityV1(dependencies,files),policy=nxAuthorizerBundlePolicyV1,bundles=requireNativeStoragePathV1(store.root,join(store.root,policy.directory)),destination=join(bundles,identity),leases=machinePathV1(root,".cache/b/l/m/data");
  const verified=async(directory:string):Promise<string>=>{requireNativeStoragePathV1(store.root,directory);const marker=nxAuthorizerBundleHoldsV1(directory,dependencies);if(marker===null||marker.digest!==identity||marker.authorizers.sourceSha256!==authorizer.digest)throw new Error("Nx immutable authorizer bundle changed");touchNxAuthorizerBundleV1(directory);await store.recheck();return join(realpathSync(directory),policy.bundle);};
  let present=true;
  try { lstatSync(destination); }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
  if (present) return withResourceLeases({directory:leases,resources:[{resource:policy.lease+identity,mode:"shared"}],control,signal},()=>verified(destination));
  return publisher(async()=>{
  const base=requireNativeStoragePathV1(store.root,machinePathV1(root,STORE));regularDirectory(base);regularDirectory(bundles);
  const staging = mkdtempSync(join(base, ".stage-"));let stagingFailed=false;
  try {
    signal.throwIfAborted();
'''+block+'''    for(const[name,bytes]of files)writeFileSync(join(staging,name),bytes,{flag:"wx",mode:0o444});
    const receipt={version:1,mode:"release",sourceSha256:authorizer.digest,entry:policy.bundle,outputs:[{path:policy.bundle,sha256:createHash("sha256").update(readFileSync(join(staging,policy.bundle))).digest("hex")}]};
    writeFileSync(join(staging,policy.marker),JSON.stringify({version:1,digest:identity,deps:dependencies,authorizers:receipt}),{flag:"wx",mode:0o444});
    if(!nxAuthorizerReceiptHoldsV1(staging,receipt))throw new Error("Nx immutable authorizer receipt changed");
    await preflightNxAuthorizersV1(staging,control,store.environment);await store.recheck();
    try { renameSync(staging, destination); }
    catch (error) { if (!["EEXIST", "ENOTEMPTY", "EPERM"].includes((error as NodeJS.ErrnoException).code ?? "")) throw error; }
    return await withResourceLeases({directory:leases,resources:[{resource:policy.lease+identity,mode:"shared"}],control,signal},()=>verified(destination));
  } catch(error){stagingFailed=true;throw error;}
  finally {try{rmSync(staging,{recursive:true,force:true});}catch(error){if(!stagingFailed)throw error;try{console.error("[nx] Authorizer staging cleanup failed after publication refusal",error);}catch{}}}
  });
}

'''
    anchor='/** 🎟️ Keeps the actual loader inside the same Original registry and generation custody used by retirement. */'
    p.sub(f,anchor,fn+anchor)
