import os
ROOT = "/Users/ueli/Documents/semio"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
S = L + "/🏗️build/📥️inputs/💾️storage/🟨️.mjs"
D = L + "/🏗️build/📥️inputs/💾️storage/🟨️.d.mts"
text = open(os.path.join(ROOT, S), encoding="utf-8").read()
start = text.index("/** 📦️ Admits optional cache bytes only within the current original serial writer. */\nexport function withBuildGraphOptionalPublicationV1(path,plan,write){")
end = text.index("/** 📇️ Seals a freshly published group leaf")
old = text[start:end]
body = old.split("\n", 2)[2]
assert body.startswith(" const scope=writers.getStore();requireRepositoryBuildGraphObservationPathV1(path,\"write\");if(!scope){write();return true;}\n requireRepositoryBuildGraphWriterV1(scope.workspace);"), body[:200]
validate_old = 'requireNativeStoragePathV1(scope.store.root,resolve(path));const name=relative(join(scope.store.root,"inference"),resolve(path)).split(sep).join("/");if(!isBuildGraphOptionalCachePathV1(name))throw Error("Explicit optional graph cache leaf required");const isolated=isolatedGraphWriterScopeV1(scope);if(isolated&&!buildGraphWriterOptionalPathAdmittedV1(isolated,name))throw Error("Original isolated graph optional namespace required");'
assert validate_old in old
validate_new = 'const isolated=isolatedGraphWriterScopeV1(scope);for(const path of paths){requireNativeStoragePathV1(scope.store.root,resolve(path));const name=relative(join(scope.store.root,"inference"),resolve(path)).split(sep).join("/");if(!isBuildGraphOptionalCachePathV1(name))throw Error("Explicit optional graph cache leaf required");if(isolated&&!buildGraphWriterOptionalPathAdmittedV1(isolated,name))throw Error("Original isolated graph optional namespace required");}'
member_old = "recordBuildGraphOptionalMemberV1(scope.store.root,resolve(path));return true;"
assert member_old in old
new = '''/** 📦️ Admits optional cache bytes only within the current original serial writer. */
export function withBuildGraphOptionalPublicationV1(path,plan,write){return withBuildGraphOptionalPublicationsV1([path],plan,write);}

/** 📦️ Admits the optional cache leaves of one batch as one publication: one exact peak, one durable intent and one clean accounting for all of them, so a writer that adopts many leaves pays the ledger once. */
export function withBuildGraphOptionalPublicationsV1(paths,plan,write){
 if(!Array.isArray(paths)||!paths.length)throw Error("Explicit optional graph cache leaves required");
 const staged=stagings.getStore();if(staged){for(const path of paths)requireBuildGraphStagedPathV1(staged,path);write();for(const path of paths)staged.published?.(resolve(path));return true;}
 const scope=writers.getStore();for(const path of paths)requireRepositoryBuildGraphObservationPathV1(path,"write");if(!scope){write();return true;}
 requireRepositoryBuildGraphWriterV1(scope.workspace);'''
rest = body.split("requireRepositoryBuildGraphWriterV1(scope.workspace);", 1)[1]
rest = rest.replace(validate_old, validate_new).replace(member_old, "for(const path of paths)recordBuildGraphOptionalMemberV1(scope.store.root,resolve(path));return true;")
new += rest
helper = '''/** 🛰️ Runs a body whose graph fact directories are private staging directories: a worker process publishes nothing into the graph backing, its parent adopts the files it staged under the original writer custody. `published` is told the absolute path of every optional leaf the body publishes. */
export function withBuildGraphStagingV1(staging,body){
 if(typeof staging?.root!=="string"||!isAbsolute(staging.root))throw Error("Absolute private graph staging root required");
 return stagings.run(Object.freeze({root:resolve(staging.root),published:staging.published}),body);
}

/** 📌️ Tells the staging of the running body that a leaf outside the optional publication gate was written. */
export function noteBuildGraphStagedLeafV1(path){stagings.getStore()?.published?.(resolve(path));}

/** 🧷️ Admits only paths inside the private staging root. */
function requireBuildGraphStagedPathV1(staged,path){
 const local=relative(staged.root,resolve(path));if(local===""||local.startsWith("..")||isAbsolute(local))throw Error("Private graph staging path required");
}

'''
EDITS = [
 (S, "const writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),", "const stagings=new AsyncLocalStorage(),writers=new AsyncLocalStorage(),datasets=new AsyncLocalStorage(),"),
 (S, old, helper + new),
 (S, "export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){\n requireRepositoryBuildGraphWriterV1(workspace);", """export function repositoryBuildGraphWriterDirectoryV1(workspace,namespace){
 const staged=stagings.getStore();if(staged){if(!schema.enum.includes(namespace)||namespace==="admission")throw Error("Declared staged graph fact namespace required");const path=join(staged.root,namespace);mkdirSync(path,{recursive:true});return path;}
 requireRepositoryBuildGraphWriterV1(workspace);"""),
 (S, 'export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){\n if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");', 'export function repositoryBuildGraphDataDirectoryV1(workspace,namespace){\n if(!schema.enum.includes(namespace))throw Error("Declared bounded graph fact namespace required");const staged=stagings.getStore();if(staged)return join(staged.root,namespace);'),
 (D, "/** 📦️ Reserves optional peak bytes under the original serial writer. */\nexport function withBuildGraphOptionalPublicationV1(path:string,plan:BuildGraphOptionalPublicationV1,write:()=>void):boolean;",
  "/** 📦️ Reserves optional peak bytes under the original serial writer. */\nexport function withBuildGraphOptionalPublicationV1(path:string,plan:BuildGraphOptionalPublicationV1,write:()=>void):boolean;\n/** 📦️ Reserves the summed peak of several optional leaves as one publication. */\nexport function withBuildGraphOptionalPublicationsV1(paths:readonly string[],plan:BuildGraphOptionalPublicationV1,write:()=>void):boolean;\n/** 🛰️ Runs a body whose graph fact directories are private staging directories adopted later by the parent that holds the writer custody. */\nexport function withBuildGraphStagingV1<T>(staging:Readonly<{root:string;published?:(path:string)=>void}>,body:()=>Promise<T>|T):Promise<T>|T;\n/** 📌️ Tells the running staged body that a leaf outside the optional publication gate was written. */\nexport function noteBuildGraphStagedLeafV1(path:string):void;"),
]
