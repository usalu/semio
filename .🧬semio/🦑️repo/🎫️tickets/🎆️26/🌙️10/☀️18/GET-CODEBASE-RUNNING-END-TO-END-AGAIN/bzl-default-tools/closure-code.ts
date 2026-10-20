

/** 🎛️ Reads the JavaScript install mode: the environment wins over the build policy; anything but `all` and `closure` is refused. */
export function javascriptInstallModeV1(environment:Readonly<Record<string,string|undefined>>):"all"|"closure"{const mode=environment.SEMIO_JS_INSTALL??buildPolicy.javascriptInstall??"all";if(mode!=="all"&&mode!=="closure")throw new Error(`Unknown JavaScript install mode: ${mode}`);return mode;}

/** 📦️ Names the parser package, the only dependency scope the graph inference needs, as a workspace-relative path derived from this module's own location. */
export const buildParserScopeV1=(root:string):string=>relative(root,fileURLToPath(new URL("./📦️parsers",import.meta.url))).split(sep).join("/");

/** 🏗️ Builds the dependency installer of one selection, with the same exclusive-hold recheck as the complete installer. */
export function javascriptInstallerV1(selection:JavascriptDependencyStoreSelectionV1):BuildEnvironmentInstallerV1{
 return async(root,original,phase,recheck)=>{
  const {prepareJavascriptDependencies,withJavascriptDependencyStorageV1}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");
  checkResourceLeaseControl(phase);await withJavascriptDependencyStorageV1(root,original,phase,(store,control)=>installUnlessReadyV1(store,control,recheck,()=>prepareJavascriptDependencies("sync",root,store,control,selection)).then(()=>undefined));checkResourceLeaseControl(phase);
 };
}

/** 🧭️ Plans the first JavaScript install of one run: a Bazel run in closure mode needs only the parser package before the graph exists; every other run keeps the complete install. */
export type BuildEnvironmentPlanV1=Readonly<{closure:boolean;scope?:string;install?:BuildEnvironmentInstallerV1}>;

/** 🗺️ Chooses the bootstrap scope and installer for a tool and an install mode. */
export function buildEnvironmentPlanV1(root:string,tool:"nx"|"bazel",environment:Readonly<Record<string,string|undefined>>):BuildEnvironmentPlanV1{
 if(tool!=="bazel"||javascriptInstallModeV1(environment)!=="closure")return Object.freeze({closure:false});
 const scope=buildParserScopeV1(root);return Object.freeze({closure:true,scope,install:javascriptInstallerV1({kind:"scope",owner:scope})});
}

/** 🧮️ Maps project roots onto the deepest declared dependency scope that contains each of them; the workspace root and roots no member contains belong to the root scope `.`. */
export function javascriptScopeOwnersV1(scopes:readonly string[],roots:readonly string[]):string[]{
 const declared=[...new Set(scopes)].filter(scope=>scope!==".").sort((left,right)=>right.length-left.length||(left<right?-1:1)),owners=new Set<string>();
 for(const raw of roots){const path=raw.replaceAll("\\","/").replace(/^(?:\.\/)+/u,"").replace(/\/+$/u,"");owners.add(declared.find(scope=>path===scope||path.startsWith(scope+"/"))??".");}
 return [...owners].sort();
}

/** 🪜️ Installs the owners of a planned task closure after the graph exists; a no-op unless closure mode is selected. */
export async function ensureBuildClosureScopesV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,projectRoots:()=>readonly string[],environment:Readonly<Record<string,string|undefined>>):Promise<readonly string[]>{
 if(javascriptInstallModeV1(environment)!=="closure")return [];
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 const manifest=JSON.parse(readFileSync(join(root,"package.json"),"utf8")) as {workspaces?:string[]},owners=javascriptScopeOwnersV1([".",...(manifest.workspaces??[])],[...projectRoots(),...buildPolicy.javascriptInstallRoots]);
 if(!owners.length)return owners;
 const {prepareJavascriptDependencies,withJavascriptDependencyStorageV1}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");
 await withJavascriptDependencyStorageV1(root,invocation,phaseControl,(store,control)=>prepareJavascriptDependencies("sync",root,store,control,{kind:"scopes",owners}));checkResourceLeaseControl(phaseControl);return owners;
}
