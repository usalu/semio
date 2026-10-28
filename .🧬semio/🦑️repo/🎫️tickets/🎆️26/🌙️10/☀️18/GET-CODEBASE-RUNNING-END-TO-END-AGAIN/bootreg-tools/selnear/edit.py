#!/usr/bin/env python3
"""🧭 Anchored, idempotent edit of batch `selnear` (lane bootreg): repository Cargo tests and lint select their named packages through the declaring workspace of the caller's manifest instead of a synchronous whole-repository inventory.
usage: python3 -I edit.py <root> check|apply   (check: exit 0 when every anchor is found or already applied)."""
import os, sys
root, verb = sys.argv[1], sys.argv[2]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
WS = f"{L}/🗂️workspaces/🦀️cargo/🟦️.ts"
AD = f"{L}/🏃️process/🦀️cargo/🎟️admission/🟦️.ts"
LI = f"{L}/🏃️process/🦀️cargo/🧹️lint/🟦️.ts"

OLD_MEMBERS = r'''function* membersPlan(root:string,owner:CargoWorkspaceScope):CargoPhysicalPlanV1<readonly CargoWorkspacePackage[]> {
  const documents = new Map<string, Record<string, any>>(), scopes = new Map<string, CargoWorkspaceScope>();
  const readDocument=function*(path:string):CargoPhysicalPlanV1<Record<string,any>>{const current=documents.get(path);if(current)return current;const document=yield* readPlan(root,path);documents.set(path,document);return document;};
  const cwd = resolve(root, owner.directory), paths = new Set<string>();
  const patterns = owner.memberManifests.map(pattern => new GlobV1(pattern));
  const leaves = owner.memberManifests.map(pattern => new GlobV1(pattern.slice(0, -"/Cargo.toml".length)));
  const prefixes = owner.memberManifests.map(pattern => pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u, ""));
  const excluded = owner.contribution.exclude.map(pattern => new GlobV1(pattern));
  const excludedRoots = owner.contribution.exclude.map(pattern => pattern.endsWith("/**") ? new GlobV1(pattern.slice(0, -3)) : undefined);
  const opaque = new Set(["node_modules", "target", "dist", "build", "🤖️generated", "coverage", "🗑️generated"]);
  const walk=function*(directory:string):CargoPhysicalPlanV1<void>{
    if(collectionRoot(root,join(cwd,directory)))return;
    if (directory && !prefixes.some(prefix => !prefix || directory.startsWith(prefix + "/") || prefix === directory || prefix.startsWith(directory + "/"))) return;
    if (directory && leaves.some(pattern => pattern.match(directory))) {
      const manifest = join(cwd, directory, "Cargo.toml");
      if((yield {operation:"presence",path:manifest,root}) && (yield* readDocument(slash(relative(root,manifest)))).package!==undefined){yield* physicalPlan(root,slash(relative(root,manifest)));paths.add(slash(relative(root,manifest)));return;}
    }
    for(const entry of (yield {operation:"list",path:join(cwd,directory),root})){
      const name=entry.name;
      if(!entry.isDirectory() && (!entry.isFile() || name!=="Cargo.toml"))continue;
      const local=directory?`${directory}/${name}`:name;
      if(name.startsWith(".") || opaque.has(name) || excluded.some((pattern,index)=>pattern.match(local) || pattern.match(`${local}/`) || excludedRoots[index]?.match(local)))continue;
      if(entry.isDirectory())yield* walk(local);
      else if(local!=="Cargo.toml" && patterns.some(pattern=>pattern.match(local)) && !excluded.some(pattern=>pattern.match(slash(dirname(local)))))paths.add(slash(relative(root,join(cwd,local))));
    }
  };
  if (patterns.some(pattern => pattern.match("Cargo.toml")) && !excluded.some(pattern => pattern.match(".") || pattern.match("./") || pattern.match("Cargo.toml"))) {
    const manifest = slash(relative(root, join(cwd, "Cargo.toml")));
    if((yield* readDocument(manifest)).package!==undefined)paths.add(manifest);
  }
  yield* walk("");
'''

NEW_MEMBERS = r'''/** 📏️ Binds one owner's admitted member patterns once, for the whole-owner walk and the single-manifest check alike. */
function memberRules(root:string,owner:CargoWorkspaceScope){
  const cwd=resolve(root,owner.directory),patterns=owner.memberManifests.map(pattern=>new GlobV1(pattern)),leaves=owner.memberManifests.map(pattern=>new GlobV1(pattern.slice(0,-"/Cargo.toml".length)));
  const prefixes=owner.memberManifests.map(pattern=>pattern.split(/[*?\[{]/u)[0]!.replace(/\/$/u,"")),excluded=owner.contribution.exclude.map(pattern=>new GlobV1(pattern)),excludedRoots=owner.contribution.exclude.map(pattern=>pattern.endsWith("/**")?new GlobV1(pattern.slice(0,-3)):undefined);
  const opaque=new Set(["node_modules","target","dist","build","🤖️generated","coverage","🗑️generated"]);
  return Object.freeze({cwd,
   matches:(local:string)=>patterns.some(pattern=>pattern.match(local)),
   leaf:(directory:string)=>leaves.some(pattern=>pattern.match(directory)),
   reaches:(directory:string)=>!collectionRoot(root,join(cwd,directory))&&(!directory||prefixes.some(prefix=>!prefix||directory.startsWith(prefix+"/")||prefix===directory||prefix.startsWith(directory+"/"))),
   hidden:(local:string)=>{const name=local.slice(local.lastIndexOf("/")+1);return name.startsWith(".")||opaque.has(name)||excluded.some((pattern,index)=>pattern.match(local)||pattern.match(`${local}/`)||excludedRoots[index]?.match(local));},
   fileExcluded:(local:string)=>excluded.some(pattern=>pattern.match(slash(dirname(local)))),
   rootAdmitted:patterns.some(pattern=>pattern.match("Cargo.toml"))&&!excluded.some(pattern=>pattern.match(".")||pattern.match("./")||pattern.match("Cargo.toml"))});
}
type CargoMemberRulesV1=ReturnType<typeof memberRules>;
function* membersPlan(root:string,owner:CargoWorkspaceScope):CargoPhysicalPlanV1<readonly CargoWorkspacePackage[]> {
  const documents = new Map<string, Record<string, any>>(), scopes = new Map<string, CargoWorkspaceScope>();
  const readDocument=function*(path:string):CargoPhysicalPlanV1<Record<string,any>>{const current=documents.get(path);if(current)return current;const document=yield* readPlan(root,path);documents.set(path,document);return document;};
  const rules=memberRules(root,owner),cwd=rules.cwd,paths=new Set<string>();
  const walk=function*(directory:string):CargoPhysicalPlanV1<void>{
    if(!rules.reaches(directory))return;
    if (directory && rules.leaf(directory)) {
      const manifest = join(cwd, directory, "Cargo.toml");
      if((yield {operation:"presence",path:manifest,root}) && (yield* readDocument(slash(relative(root,manifest)))).package!==undefined){yield* physicalPlan(root,slash(relative(root,manifest)));paths.add(slash(relative(root,manifest)));return;}
    }
    for(const entry of (yield {operation:"list",path:join(cwd,directory),root})){
      const name=entry.name;
      if(!entry.isDirectory() && (!entry.isFile() || name!=="Cargo.toml"))continue;
      const local=directory?`${directory}/${name}`:name;
      if(rules.hidden(local))continue;
      if(entry.isDirectory())yield* walk(local);
      else if(local!=="Cargo.toml" && rules.matches(local) && !rules.fileExcluded(local))paths.add(slash(relative(root,join(cwd,local))));
    }
  };
  if (rules.rootAdmitted) {
    const manifest = slash(relative(root, join(cwd, "Cargo.toml")));
    if((yield* readDocument(manifest)).package!==undefined)paths.add(manifest);
  }
  yield* walk("");
'''

OLD_TAIL = r'''   names.add(name);result.push({directory:slash(dirname(manifest)),manifest,name,workspace:owner.directory});
  }
  return result;
}
'''
NEW_TAIL = OLD_TAIL + r'''/** 🔭️ Holds when the whole-owner walk of `membersPlan` would admit exactly this manifest, asking only its own ancestry instead of listing the owner. */
function* memberPlan(root:string,rules:CargoMemberRulesV1,manifest:string,readDocument:(path:string)=>CargoPhysicalPlanV1<Record<string,any>>):CargoPhysicalPlanV1<boolean> {
  const local=slash(relative(rules.cwd,resolve(root,manifest)));
  if(local==="Cargo.toml")return rules.rootAdmitted;
  if(local.startsWith("../")||isAbsolute(local)||!local.endsWith("/Cargo.toml")||!rules.reaches(""))return false;
  const parts=local.split("/").slice(0,-1);
  for(let index=1;index<=parts.length;index++){
    const directory=parts.slice(0,index).join("/");
    if(rules.hidden(directory)||!(yield {operation:"state",path:join(rules.cwd,directory),root}).isDirectory()||!rules.reaches(directory))return false;
    if(rules.leaf(directory)&&(yield {operation:"presence",path:join(rules.cwd,directory,"Cargo.toml"),root})&&(yield* readDocument(slash(relative(root,join(rules.cwd,directory,"Cargo.toml"))))).package!==undefined)return index===parts.length;
  }
  return !rules.hidden(local)&&rules.matches(local)&&!rules.fileExcluded(local)&&(yield {operation:"state",path:resolve(root,manifest),root}).isFile();
}
function* declaredPlan(root:string,manifest:string,names:readonly string[]):CargoPhysicalPlanV1<readonly CargoWorkspacePackage[]|undefined> {
  const documents=new Map<string,Record<string,any>>(),scopes=new Map<string,CargoWorkspaceScope>();
  const readDocument=function*(path:string):CargoPhysicalPlanV1<Record<string,any>>{const current=documents.get(path);if(current)return current;const document=yield* readPlan(root,path);documents.set(path,document);return document;};
  const owner=yield* selectPlan(root,manifest,readDocument,scopes),rules=memberRules(root,owner),declared=(yield* readDocument(owner.manifest)).workspace?.members;
  if(!Array.isArray(declared))return undefined;
  const near=slash(dirname(resolve(root,manifest))).split("/"),shared=(path:string):number=>{const parts=path.split("/");let count=0;while(count<parts.length&&parts[count]===near[count])count++;return count;};
  const candidates=[...new Set(declared.filter((member):member is string=>pathPattern(member)&&!/[*?[\]{}!]/u.test(member)).map(member=>slash(join(rules.cwd,member))))].sort((a,b)=>shared(b)-shared(a)||(a<b?-1:a>b?1:0));
  const wanted=new Set(names),found=new Map<string,CargoWorkspacePackage>();
  for(const directory of candidates){
    if(found.size===wanted.size)break;
    const path=slash(relative(root,join(directory,"Cargo.toml")));
    if(!(yield {operation:"presence",path:join(directory,"Cargo.toml"),root}))continue;
    const name=(yield* readDocument(path)).package?.name;
    if(typeof name!=="string"||!wanted.has(name))continue;
    if(found.has(name)||!(yield* memberPlan(root,rules,path,readDocument))||(yield* selectPlan(root,path,readDocument,scopes)).directory!==owner.directory)return undefined;
    found.set(name,{directory:slash(dirname(path)),manifest:path,name,workspace:owner.directory});
  }
  return found.size===wanted.size?names.map(name=>found.get(name)!):undefined;
}
/** 🪧️ Selects named packages among the literal members Cargo reads from the workspace owning `manifest`, nearest first, each admitted exactly as the whole-owner walk would admit it, without listing a directory; undefined when that workspace does not declare them all as current members or a physical answer is not plain.
 * @see https://doc.rust-lang.org/cargo/reference/workspaces.html#the-members-and-exclude-fields */
export function cargoDeclaredPackageSelectionsV1(root:string,manifest:string,names:readonly string[],observations?:CargoPhysicalObservationsV1):readonly CargoWorkspacePackage[]|undefined {
  if(!names.length)return [];try{return drain(declaredPlan(root,manifest,names),observations);}catch{return undefined;}
}
/** 🛰️ Binds named packages through the workspace owning `manifest` and inventories the whole repository only when that workspace does not declare them all; a duplicate name outside that workspace is left to the repository inventory's own consumers. */
export function cargoPackageSelectionsNearV1(root:string,manifest:string,names:readonly string[]):readonly CargoWorkspacePackage[] {
  return cargoDeclaredPackageSelectionsV1(root,manifest,names)??cargoRepositoryPackageSelections(root,names);
}
'''

EDITS = {
  WS: [(OLD_MEMBERS, NEW_MEMBERS), (OLD_TAIL, NEW_TAIL)],
  AD: [('import {cargoRepositoryPackageSelections,cargoWorkspaceForManifest,prepareCargoWorkspaceInvocation} from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";',
        'import {cargoPackageSelectionsNearV1,cargoWorkspaceForManifest,prepareCargoWorkspaceInvocation} from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";'),
       ('const repository=getWorkspaceRoot(), rows=cargoRepositoryPackageSelections(repository,packages);',
        'const repository=getWorkspaceRoot(), rows=cargoPackageSelectionsNearV1(repository,join(cwd,"Cargo.toml"),packages);')],
  LI: [('import { cargoRepositoryPackageSelections } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";',
        'import { cargoPackageSelectionsNearV1 } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";'),
       ('const repository = getWorkspaceRoot(), rows = cargoRepositoryPackageSelections(repository, resolvedPackages);',
        'const repository = getWorkspaceRoot(), rows = cargoPackageSelectionsNearV1(repository, join(cwd, "Cargo.toml"), resolvedPackages);')],
}
bad = 0
for rel, pairs in EDITS.items():
    path = os.path.join(root, rel); text = open(path, encoding="utf8").read(); out = text
    for old, new in pairs:
        if new in out and (old not in out or old in new): continue
        if out.count(old) != 1: print("ANCHOR", out.count(old), rel[-60:], old[:70].replace("\n", " ")); bad += 1; continue
        out = out.replace(old, new)
    if verb == "apply" and out != text:
        st = os.stat(path); open(path, "w", encoding="utf8").write(out); os.chmod(path, st.st_mode & 0o7777); print("edited", rel[-70:])
sys.exit(1 if bad else 0)
