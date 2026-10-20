"""Edit tables of apply.py: new files, anchored text edits and structural JSON edits.

Paths are repository relative. Text edits are (label, old, new) triples whose old text must occur exactly once.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from spec import ALIAS, PATH_BUDGET, budget

BZ = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
BS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel"

NEW_FILES = [
    ("paths.test.ts", BZ + "/🧪️tests/📥️paths/🟦️.ts"),
    ("paths.fixture.json", BZ + "/🧪️tests/📥️paths/🔣️.json"),
    ("paths.schema.json", BZ + "/🧪️tests/📥️paths/🧬️schema.json"),
]

PLATFORM = [
    (
        "closure in the platform input type",
        r'''readonly bun:string;readonly longPaths:"enabled"|"disabled"|"unprobed"};''',
        r'''readonly bun:string;readonly longPaths:"enabled"|"disabled"|"unprobed";readonly closure:BazelPathClosureV1};''',
    ),
    (
        "path budget report in the profile type",
        r'''readonly pathBudget:{readonly outputBase:number;readonly execroot:number;readonly tracked:number;readonly total:number;readonly limit:number;readonly fits:boolean};''',
        r'''readonly pathBudget:BazelPathBudgetV1;''',
    ),
    (
        "path budget model",
        r'''/** 🚏️ Selects the authored per-platform Bazel rules; an unlisted platform is refused instead of guessed. */''',
        r'''const paths=schema.definitions.PathBudget.const;
export type BazelPathSizeV1={readonly units:number;readonly bytes:number};
export type BazelPathClosureV1={readonly alias:BazelPathSizeV1;readonly real:BazelPathSizeV1};
export type BazelPathBudgetV1={readonly outputBase:BazelPathSizeV1;readonly families:readonly (BazelPathSizeV1&{readonly name:string})[];readonly limitUnits:number;readonly limitBytes:number|null;readonly slackUnits:number;readonly slackBytes:number|null;readonly fits:boolean};
export type BazelPathBudgetInputV1=Pick<BazelPlatformInputV1,"platform"|"arch"|"outputBase"|"longPaths"|"closure">;

/** 📏️ Measures the longest in-package source labels of the aliased and of the repository-layout view in UTF-16 units and UTF-8 bytes, the two lengths a host path limit is stated in. */
export function bazelPathClosureV1(alias:Iterable<string>,real:Iterable<string>):BazelPathClosureV1{
 const encoder=new TextEncoder(),size=(labels:Iterable<string>)=>{let units=0,bytes=0;for(const text of labels){units=Math.max(units,text.length);bytes=Math.max(bytes,encoder.encode(text).length);}return{units,bytes};};
 return{alias:size(alias),real:size(real)};
}

/** 📐️ Computes every path family Bazel creates beneath an output base from the measured source labels: the repository rule of 256 UTF-16 units applies everywhere, and each host adds its own limit (PATH_MAX in UTF-8 bytes on macOS and Linux, MAX_PATH in UTF-16 units on Windows unless long paths are enabled); `slackUnits` and `slackBytes` are what the longest family leaves free, negative when it does not fit. */
export function bazelPathBudgetV1(input:BazelPathBudgetInputV1):BazelPathBudgetV1{
 const row=paths.platforms[input.platform],runfiles=!bazelPlatformRulesV1(input.platform).command.includes("--noenable_runfiles"),encoder=new TextEncoder(),base={units:input.outputBase.length,bytes:encoder.encode(input.outputBase).length},families:(BazelPathSizeV1&{name:string})[]=[];
 const filler:Readonly<Record<string,string>>={configuration:row.configuration[input.arch],task:paths.task,file:paths.file,sandbox:row.sandboxes.reduce((longest,name)=>name.length>longest.length?name:longest,""),sandboxId:"0".repeat(paths.sandboxDigits)};
 for(const family of paths.families as readonly {name:string;template:string;view?:"alias"|"real";needs?:"runfiles"|"sandbox"}[]){
  if((family.needs==="runfiles"&&!runfiles)||(family.needs==="sandbox"&&!row.sandboxes.length))continue;
  const source=family.view===undefined?{units:0,bytes:0}:input.closure[family.view];if(family.view!==undefined&&source.units===0)continue;
  const prefix=family.template.replace("{source}","").replace(/\{(\w+)\}/gu,(_match,key:string)=>filler[key]!).length;
  families.push({name:family.name,units:base.units+1+prefix+source.units,bytes:base.bytes+1+prefix+source.bytes});
 }
 const operating=input.platform==="win32"?(input.longPaths==="enabled"?paths.platforms.win32.extendedMax:row.pathMax)-1:Infinity,limitUnits=Math.min(paths.repositoryUnits,operating),limitBytes=row.unit==="utf8"?row.pathMax-1:null;
 const slackUnits=limitUnits-Math.max(...families.map(family=>family.units)),slackBytes=limitBytes===null?null:limitBytes-Math.max(...families.map(family=>family.bytes));
 return Object.freeze({outputBase:base,families,limitUnits,limitBytes,slackUnits,slackBytes,fits:slackUnits>=0&&(slackBytes===null||slackBytes>=0)});
}

/** 🚧️ Names the family that exceeds the budget most and how many characters shorter the checkout must be. */
function bazelPathRefusalV1(input:BazelPathBudgetInputV1,budget:BazelPathBudgetV1):string{
 const excess=(family:BazelPathSizeV1)=>Math.max(family.units-budget.limitUnits,budget.limitBytes===null?-Infinity:family.bytes-budget.limitBytes),worst=[...budget.families].sort((left,right)=>excess(right)-excess(left))[0]!,windows=input.platform==="win32";
 return `The Bazel path budget is exceeded: ${worst.name} reaches ${worst.units} UTF-16 units (${worst.bytes} bytes) beneath the output base ${input.outputBase}, beyond the limit of ${budget.limitUnits} units${budget.limitBytes===null?"":` and ${budget.limitBytes} bytes`} (repository rule ${paths.repositoryUnits}${windows?`, Windows MAX_PATH ${paths.platforms.win32.pathMax-1} without long paths`:""}); move the checkout to a directory at least ${excess(worst)} characters shorter${windows?", for example C:\\semio":""}.`;
}

/** 🚏️ Selects the authored per-platform Bazel rules; an unlisted platform is refused instead of guessed. */''',
    ),
    (
        "budget refusal from the real closure",
        r''' const execroot=(windows?rules.execroot.win32:rules.execroot.posix).length,total=actual.outputBase.length+execroot+rules.trackedPathUnits,limit=windows?(actual.longPaths==="enabled"?rules.limits.windowsExtended:rules.limits.windows):rules.limits.posix,fits=total<=limit;
 if(!fits)refusals.push(`The Bazel output base ${actual.outputBase} (${actual.outputBase.length}) plus the execroot (${execroot}) plus a tracked path of ${rules.trackedPathUnits} units is ${total}, beyond the ${limit} unit path limit${windows?"; enable Windows long paths machine-wide or move the checkout to a shorter directory such as C:\\semio":""}.`);
''',
        r''' const pathBudget=bazelPathBudgetV1(actual);
 if(!pathBudget.fits)refusals.push(bazelPathRefusalV1(actual,pathBudget));
''',
    ),
    (
        "budget in the rendered profile",
        r'''pathBudget:{outputBase:actual.outputBase.length,execroot,tracked:rules.trackedPathUnits,total,limit,fits},refusals,warnings});''',
        r'''pathBudget,refusals,warnings});''',
    ),
]

SCRIPT = [
    (
        "platform module imports",
        r'''import {bazelPlatformProfileV1,bazelPlatformRulesV1} from "./🌐️platform/🟦️.ts";''',
        r'''import {bazelPathClosureV1,bazelPlatformProfileV1,bazelPlatformRulesV1,type BazelPathClosureV1} from "./🌐️platform/🟦️.ts";''',
    ),
    (
        "alias rule constant",
        r'''const actions=componentSchema.definitions.Actions.const,sourceLabel=new RegExp(actions.sources.label);''',
        r'''const actions=componentSchema.definitions.Actions.const,sourceLabel=new RegExp(actions.sources.label),aliasRule=actions.sources.alias;''',
    ),
    (
        "repository-layout label doc",
        r'''only package names must be ASCII. */
export function bazelSourceLabelV1(path:string):string{return actions.sources.link+"/"+path;}''',
        r'''only package names must be ASCII. It keeps the repository layout and serves hermetic actions, which execute inside it; every other action names its sources through `bazelSourceAliasLabelV1`. */
export function bazelSourceLabelV1(path:string):string{return actions.sources.link+"/"+path;}

/** 🔖 Names a checkout directory by the first hex digits of the SHA-256 of its slash-separated repository path, so an aliased source label is short, deterministic and independent of the checkout location and the operating system. */
export function bazelSourceAliasV1(directory:string,length:number=aliasRule.hexLength):string{
 if(!Number.isInteger(length)||length<1||length>64)throw new Error("Bazel source alias length of 1 to 64 hex digits required");
 return hash(directory).slice(0,length);
}

/** 🏷️ Names a checkout file by its short in-package label `a/<alias of its directory>/<name>`, which every host path limit admits however deep the repository path is; the real path stays in the checkout and in the workspace's `sources.json`. */
export function bazelSourceAliasLabelV1(path:string,length:number=aliasRule.hexLength):string{const cut=path.lastIndexOf("/");return aliasRule.link+"/"+bazelSourceAliasV1(cut<0?"":path.slice(0,cut),length)+"/"+path.slice(cut+1);}''',
    ),
    (
        "view tag on source groups",
        r'''export interface BazelSourceGroupV1{groups:readonly string[];files:readonly string[]}''',
        r'''export interface BazelSourceGroupV1{groups:readonly string[];files:readonly string[];view?:"alias"}''',
    ),
    (
        "cover view parameter",
        r'''export function bazelSourceCoverV1(universe:readonly string[]):{groups:Map<string,BazelSourceGroupV1>;''',
        r'''export function bazelSourceCoverV1(universe:readonly string[],view:"alias"|"real"="real"):{groups:Map<string,BazelSourceGroupV1>;''',
    ),
    (
        "cover view tag",
        r'''prefix=new Int32Array(universe.length+1),order=(left:string,right:string)''',
        r'''prefix=new Int32Array(universe.length+1),tag=view==="alias"?{view:"alias" as const}:{},order=(left:string,right:string)''',
    ),
    (
        "cover directory group names",
        r'''const owner=representative(row),name="d_"+hash(owner.path).slice(0,16);
  if(!groups.has(name))groups.set(name,{groups:owner.children.map(child=>define(directories.get(child)!)),files:owner.files.map(index=>universe[index]!)});''',
        r'''const owner=representative(row),name=(view==="alias"?"a_":"d_")+hash(owner.path).slice(0,16);
  if(!groups.has(name))groups.set(name,{groups:owner.children.map(child=>define(directories.get(child)!)),files:owner.files.map(index=>universe[index]!),...tag});''',
    ),
    (
        "cover result tag",
        r'''return{groups:sorted(names),files:sorted(files)};''',
        r'''return{groups:sorted(names),files:sorted(files),...tag};''',
    ),
    (
        "merge result tag",
        r'''merge(covers){return{groups:sorted(covers.flatMap(cover=>cover.groups)),files:sorted(covers.flatMap(cover=>cover.files))};},''',
        r'''merge(covers){return{groups:sorted(covers.flatMap(cover=>cover.groups)),files:sorted(covers.flatMap(cover=>cover.files)),...tag};},''',
    ),
    (
        "compose view names",
        r'''const name=kind+"_"+hash(cover.groups.join("\0")+"\u0001"+cover.files.join("\0")).slice(0,16);groups.set(name,cover);return name;''',
        r'''const name=kind+"_"+hash((view==="alias"?"a\u0002":"")+cover.groups.join("\0")+"\u0001"+cover.files.join("\0")).slice(0,16);groups.set(name,{...cover,...tag});return name;''',
    ),
    (
        "render labels per view",
        r'''...sources.files.map(bazelSourceLabelV1)])}''',
        r'''...sources.files.map(path=>sources.view==="alias"?bazelSourceAliasLabelV1(path):bazelSourceLabelV1(path))])}''',
    ),
    (
        "alias farm",
        r''' await symlink(root,link,bazelPlatformRulesV1(process.platform).sourceLink);
}
''',
        r''' await symlink(root,link,bazelPlatformRulesV1(process.platform).sourceLink);
}

/** 🪢 Reconciles the alias farm `a/<alias>` of a generated workspace with the directories that declare sources: one directory link per alias (a junction on Windows), written only when absent or different so Bazel's directory state stays stable, stale aliases pruned, and two directories that share an alias refused before any link changes. Returns the alias to directory map that `sources.json` records. */
export async function linkBazelSourceAliasesV1(root:string,workspace:string,directories:Iterable<string>,length:number=aliasRule.hexLength):Promise<ReadonlyMap<string,string>>{
 const wanted=new Map<string,string>();
 for(const directory of directories){const alias=bazelSourceAliasV1(directory,length),known=wanted.get(alias);if(known!==undefined&&known!==directory)throw new Error(`Bazel source alias collision: ${alias}: ${known} and ${directory}`);wanted.set(alias,directory);}
 const farm=join(workspace,aliasRule.link),kind=bazelPlatformRulesV1(process.platform).sourceLink,names=[...wanted.keys()];
 await mkdir(farm,{recursive:true});
 for(const name of await readdir(farm))if(!wanted.has(name))await rm(join(farm,name),{recursive:true,force:true});
 for(let offset=0;offset<names.length;offset+=256)await Promise.all(names.slice(offset,offset+256).map(async alias=>{
  const target=join(root,wanted.get(alias)!),link=join(farm,alias);let current:string|undefined;
  try{current=await readlink(link);}catch(error){if(!["ENOENT","EINVAL"].includes((error as NodeJS.ErrnoException).code??""))throw error;}
  if(current!==undefined&&resolve(current)===resolve(target))return;
  await rm(link,{recursive:true,force:true});await symlink(target,link,kind);
 }));
 return wanted;
}
''',
    ),
    (
        "per-view source covers",
        r''' const tree=bazelSourceCoverV1(universe),shared=new Map<string,{cover:BazelSourceGroupV1;hashed:string[]}>(),composed=new Map<string,{source?:string;hashed:string[]}>();''',
        r''' const views=new Map<"alias"|"real",{tree:ReturnType<typeof bazelSourceCoverV1>;shared:Map<string,{cover:BazelSourceGroupV1;hashed:string[]}>;composed:Map<string,{source?:string;hashed:string[]}>}>(),view=(name:"alias"|"real")=>{let known=views.get(name);if(!known){known={tree:bazelSourceCoverV1(universe,name),shared:new Map(),composed:new Map()};views.set(name,known);}return known;};''',
    ),
    (
        "unit cover per view",
        r''' const unitCover=(unit:BazelSourceKeyV1)=>{const text=JSON.stringify(unit);let row=shared.get(text);''',
        r''' const unitCover=(unit:BazelSourceKeyV1,{tree,shared}:ReturnType<typeof view>)=>{const text=JSON.stringify(unit);let row=shared.get(text);''',
    ),
    (
        "action cover per view",
        r'''checkScriptInvocation(invocation);const text=JSON.stringify(inputs.get(action.id)!);let row=composed.get(text);''',
        r'''checkScriptInvocation(invocation);const scope=view(action.kind==="hermetic"?"real":"alias"),{tree,composed}=scope,text=JSON.stringify(inputs.get(action.id)!);let row=composed.get(text);''',
    ),
    (
        "unit covers of the action view",
        r'''parts=units.map(unitCover);''',
        r'''parts=units.map(unit=>unitCover(unit,scope));''',
    ),
    (
        "alias farm, sources record and path measurement",
        r''' let authorized:Set<string>|undefined;''',
        r''' const directories=new Set(universe.map(path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):"")),aliases=await linkBazelSourceAliasesV1(root,workspace,directories),repositoryLabels=new Set<string>();
 await writeChanged(join(workspace,"sources.json"),JSON.stringify({version:1,link:aliasRule.link,hexLength:aliasRule.hexLength,directories:Object.fromEntries([...aliases].sort(([left],[right])=>left<right?-1:left>right?1:0))}));
 for(const action of declared)if(action.kind==="hermetic"){const{marks,extra}=index.selectAll(inputs.get(action.id)!);for(let position=0;position<marks.length;position++)if(marks[position]&&declarable.has(index.files[position]!))repositoryLabels.add(bazelSourceLabelV1(index.files[position]!));for(const path of extra)if(declarable.has(path))repositoryLabels.add(bazelSourceLabelV1(path));}
 await writeChanged(join(workspace,"paths.json"),JSON.stringify({version:1,closure:bazelPathClosureV1(universe.map(path=>bazelSourceAliasLabelV1(path)),repositoryLabels)}));
 let authorized:Set<string>|undefined;''',
    ),
    (
        "build groups of every view",
        r'''groups:tree.groups,bun:process.execPath''',
        r'''groups:new Map([...views.values()].flatMap(scope=>[...scope.tree.groups])),bun:process.execPath''',
    ),
    (
        "path measurement reader",
        r'''/** 📜 Hands the Starlark rules of a generated workspace to tests and tools that render or inspect them. */''',
        r'''/** 📏️ Reads the source label sizes the workspace generator measured for this exact selection; a workspace without the record is refused because the path budget cannot be computed from a guess. */
async function readBazelPathsV1(workspace:string):Promise<BazelPathClosureV1>{
 const value=JSON.parse(await readFile(join(workspace,"paths.json"),"utf8"));
 if(validateJsonSchemaSubset(componentSchema.definitions.PathMeasurement,value,componentSchema).length)throw new Error("Closed Bazel path measurement required");
 return value.closure;
}

/** 📜 Hands the Starlark rules of a generated workspace to tests and tools that render or inspect them. */''',
    ),
    (
        "profile requires the closure",
        r'''function requireBazelPlatformProfileV1(cache:{readonly startup:readonly string[]}){''',
        r'''function requireBazelPlatformProfileV1(cache:{readonly startup:readonly string[]},closure:BazelPathClosureV1){''',
    ),
    (
        "profile input carries the closure",
        r'''longPaths:process.platform==="win32"?(readNativeWindowsLongPathsEnabledV1()?"enabled":"disabled"):"unprobed"});''',
        r'''longPaths:process.platform==="win32"?(readNativeWindowsLongPathsEnabledV1()?"enabled":"disabled"):"unprobed",closure});''',
    ),
    (
        "task profile reads the measurement",
        r'''skipRemoteCache:request.skipRemoteCache}),profile=requireBazelPlatformProfileV1(cache),''',
        r'''skipRemoteCache:request.skipRemoteCache}),profile=requireBazelPlatformProfileV1(cache,await readBazelPathsV1(workspace)),''',
    ),
    (
        "affected profile has no sources",
        r'''{environment:process.env}),profile=requireBazelPlatformProfileV1(cache),''',
        r'''{environment:process.env}),profile=requireBazelPlatformProfileV1(cache,bazelPathClosureV1([],[])),''',
    ),
]

def insert_after(table, after, items):
    keys = list(table)
    rebuilt = {}
    for key in keys:
        rebuilt[key] = table[key]
        if key == after:
            for name, value in items.items():
                rebuilt.setdefault(name, value)
    table.clear()
    table.update(rebuilt)


SIZE = {
    "type": "object",
    "additionalProperties": False,
    "required": ["units", "bytes"],
    "properties": {"units": {"type": "integer", "minimum": 0, "maximum": 1048576}, "bytes": {"type": "integer", "minimum": 0, "maximum": 4194304}},
}
NEW_DEFINITIONS = {
    "PathBudget": {"const": PATH_BUDGET},
    "PathSize": SIZE,
    "PathClosure": {
        "type": "object",
        "additionalProperties": False,
        "required": ["alias", "real"],
        "properties": {"alias": {"$ref": "#/definitions/PathSize"}, "real": {"$ref": "#/definitions/PathSize"}},
    },
    "PathMeasurement": {
        "type": "object",
        "additionalProperties": False,
        "required": ["version", "closure"],
        "properties": {"version": {"const": 1}, "closure": {"$ref": "#/definitions/PathClosure"}},
    },
    "PathBudgetReport": {
        "type": "object",
        "additionalProperties": False,
        "required": ["outputBase", "families", "limitUnits", "limitBytes", "slackUnits", "slackBytes", "fits"],
        "properties": {
            "outputBase": {"$ref": "#/definitions/PathSize"},
            "families": {
                "type": "array",
                "minItems": 1,
                "items": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["name", "units", "bytes"],
                    "properties": {
                        "name": {"type": "string", "minLength": 1},
                        "units": {"type": "integer", "minimum": 0},
                        "bytes": {"type": "integer", "minimum": 0},
                    },
                },
            },
            "limitUnits": {"type": "integer", "minimum": 1},
            "limitBytes": {"anyOf": [{"type": "integer", "minimum": 1}, {"type": "null"}]},
            "slackUnits": {"type": "integer"},
            "slackBytes": {"anyOf": [{"type": "integer"}, {"type": "null"}]},
            "fits": {"type": "boolean"},
        },
    },
}


def schema_edit(document):
    definitions = document["definitions"]
    changed = False
    sources = definitions["Actions"]["const"]["sources"]
    if sources.get("alias") != ALIAS:
        sources["alias"] = dict(ALIAS)
        changed = True
    policy = definitions["ActionPolicy"]["properties"]["sources"]
    if "alias" not in policy["required"]:
        policy["required"].append("alias")
        policy["properties"]["alias"] = {
            "type": "object",
            "additionalProperties": False,
            "required": ["link", "hexLength"],
            "properties": {"link": {"type": "string", "pattern": "^[a-z][a-z0-9_]*$"}, "hexLength": {"type": "integer", "minimum": 8, "maximum": 32}},
        }
        changed = True
    rules = definitions["PlatformRules"]["const"]
    for key in ("limits", "trackedPathUnits", "execroot"):
        if key in rules:
            del rules[key]
            changed = True
    missing = {name: value for name, value in NEW_DEFINITIONS.items() if definitions.get(name) != value}
    if missing:
        for name in missing:
            definitions.pop(name, None)
        insert_after(definitions, "PlatformRules", missing)
        changed = True
    platform_input = definitions["PlatformInput"]
    if "closure" not in platform_input["required"]:
        platform_input["required"].append("closure")
        platform_input["properties"]["closure"] = {"$ref": "#/definitions/PathClosure"}
        changed = True
    profile = definitions["PlatformProfile"]["properties"]
    if profile["pathBudget"] != {"$ref": "#/definitions/PathBudgetReport"}:
        profile["pathBudget"] = {"$ref": "#/definitions/PathBudgetReport"}
        changed = True
    return changed


PLATFORM_FIXTURE = BZ + "/🧪️tests/📥️platform/🔣️.json"
PLATFORM_TEST = BZ + "/🧪️tests/📥️platform/🟦️.ts"
CLOSURE = {"alias": {"units": 47, "bytes": 51}, "real": {"units": 0, "bytes": 0}}
OVERLONG = {"units": 220, "bytes": 260}
RENAMED = {
    "Linux longest output base": "Linux longest output base refuses the 256 unit rule",
    "Windows x64 longest output base with long paths": "Windows x64 longest output base with long paths refuses the 256 unit rule",
    "Windows x64 without long paths refuses the MAX_PATH budget": "Windows x64 without long paths refuses an overlong closure",
    "Windows x64 unprobed long paths refuses the MAX_PATH budget": "Windows x64 unprobed long paths refuses an overlong closure",
}


def platform_fixture_edit(document):
    changed = False
    for row in document["profiles"]:
        name = RENAMED.get(row["name"], row["name"])
        overlong = name.endswith("refuses an overlong closure")
        closure = {"alias": OVERLONG, "real": CLOSURE["real"]} if overlong else CLOSURE
        if row["name"] != name:
            row["name"] = name
            changed = True
        if row["input"].get("closure") != closure:
            row["input"]["closure"] = closure
            changed = True
        source = row["input"]
        expected = row["expect"]
        result = budget(source["platform"], source["arch"], source["outputBase"], source["longPaths"], closure)
        other = expected["refusals"] - (0 if expected["fits"] else 1)
        refusals = other + (0 if result["fits"] else 1)
        if expected["fits"] != result["fits"] or expected["refusals"] != refusals:
            expected["fits"] = result["fits"]
            expected["refusals"] = refusals
            changed = True
    return changed


READINESS_FIXTURE = BS + "/🩺️readiness/🧪️tests/🧫️fixtures/🔣️.json"
READINESS_SCHEMA = BS + "/🧬️schema/🔣️.json"


def readiness_fixture_edit(document):
    changed = False
    if "longPaths" in document["facts"]:
        del document["facts"]["longPaths"]
        changed = True
    for row in document["cases"]:
        if "longPaths" in row["patch"]:
            del row["patch"]["longPaths"]
            changed = True
    kept = [row for row in document["cases"] if row["id"] != "windows-long-paths-disabled"]
    if len(kept) != len(document["cases"]):
        document["cases"] = kept
        changed = True
    invalid = [row for row in document["invalid"] if "longPaths" not in row]
    if len(invalid) != len(document["invalid"]):
        document["invalid"] = invalid
        changed = True
    return changed


def readiness_schema_edit(document):
    facts = document["definitions"]["PlatformFacts"]
    changed = False
    if "longPaths" in facts["required"]:
        facts["required"].remove("longPaths")
        changed = True
    if "longPaths" in facts["properties"]:
        del facts["properties"]["longPaths"]
        changed = True
    return changed


READINESS = [
    (
        "no long paths fact in the type",
        r'''readonly imageTool:boolean;readonly longPaths:"enabled"|"disabled"|"unprobed";readonly paths:''',
        r'''readonly imageTool:boolean;readonly paths:''',
    ),
    (
        "no long paths probe state",
        r''',imageTool=true,longPaths:BazelPlatformFactsV1["longPaths"]="unprobed";''',
        r''',imageTool=true;''',
    ),
    (
        "no long paths fact in the result",
        r'''exchange,caseSensitive,imageTool,longPaths,paths};''',
        r'''exchange,caseSensitive,imageTool,paths};''',
    ),
]

LINE_EDITS = [
    (
        BS + "/🩺️readiness/🟦️.ts",
        [
            ("drop the registry import", "line", "import {readNativeWindowsLongPathsEnabledV1} from", None, None),
            ("drop the unconditional long path refusal", "line", '  if(actual.longPaths==="disabled")refusals.push(', None, None),
            ("drop the long path probe", "line", ' if(process.platform==="win32")longPaths=readNativeWindowsLongPathsEnabledV1()', None, None),
        ],
    ),
    (
        PLATFORM_TEST,
        [
            (
                "drop constants of the replaced MAX_PATH test",
                "line",
                'const rules=component.definitions.PlatformRules.const,python=Bun.which("python3"),binary=',
                None,
                'const binary=process.env.SEMIO_TEST_BAZEL_BINARY,adapter=readFileSync(join(import.meta.dir,"../../📜️script.ts"),"utf8");',
            ),
            ("drop the constant MAX_PATH oracle test", "block", 'test.skipIf(!python)("the MAX_PATH budget counts UTF-16 units', "});", None),
        ],
    ),
]

PLATFORM_TEST_TEXT = [
    (
        "real Bazel profile carries an empty closure",
        r'''bun:process.execPath,longPaths:process.platform==="win32"?"enabled":"unprobed"});''',
        r'''bun:process.execPath,longPaths:process.platform==="win32"?"enabled":"unprobed",closure:{alias:{units:0,bytes:0},real:{units:0,bytes:0}}});''',
    ),
]

AGGREGATOR = [
    (
        "register the path tests",
        r'''import "./📥️platform/🟦️.ts";''',
        r'''import "./📥️platform/🟦️.ts";
import "./📥️paths/🟦️.ts";''',
    ),
]

TEXT_EDITS = [
    (BZ + "/🧪️tests/🟦️.ts", AGGREGATOR),
    (BZ + "/🌐️platform/🟦️.ts", PLATFORM),
    (BZ + "/📜️script.ts", SCRIPT),
    (PLATFORM_TEST, PLATFORM_TEST_TEXT),
    (BS + "/🩺️readiness/🟦️.ts", READINESS),
]

JSON_EDITS = [
    (BZ + "/🧬️schema/🔣️.json", "alias rule, path budget, closure input and budget report", schema_edit),
    (PLATFORM_FIXTURE, "profile inputs carry the measured closure and the budget outcome", platform_fixture_edit),
    (READINESS_FIXTURE, "no long paths fact", readiness_fixture_edit),
    (READINESS_SCHEMA, "no long paths fact", readiness_schema_edit),
]
