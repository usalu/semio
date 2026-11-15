import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {homedir} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import componentSchema from "./🧬️schema/🔣️.json" with {type:"json"};

const {Action:actionSchema,Manifest:manifestSchema}=componentSchema.definitions,policy=componentSchema.definitions.Policy.const,hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");

/** 🦀️ One typed Cargo invocation Bazel can key by declared inputs: the package, its selection, profile and the toolchain identity; the repository ingress decides how it compiles. */
export interface BazelCargoActionV1{version:1;id:string;subcommand:"check"|"build"|"test";manifest:string;package:string;selection:"--lib"|"--bins"|"--tests"|"--all-targets";target?:string;configuration:"prototype"|"release";features?:string[];toolchain:string;artifacts:"none"|"files";suffixes?:string[]}
/** 📜️ The closed manifest the action runner reads: ingress argv, the package whose artifacts are published and their suffixes. */
export interface BazelCargoManifestV1{version:1;id:string;package:string;toolchain:string;argv:string[];artifacts:"none"|"files";suffixes:string[]}
/** 🧭️ The Cargo subcommand and manifest a native build task runs through the repository's Cargo script. */
export interface BazelCargoClassV1{subcommand:"check"|"build"|"test";manifest:string}
/** 🔁️ The local CPU request that makes one Cargo action own the machine: Bazel clamps it to the host, so Cargo actions run one at a time and waiting actions hold no process, like the compiler lease the ingress takes. */
export const BAZEL_CARGO_EXCLUSIVE_V1=policy.exclusive;

/** 🧮️ Names the Cargo subcommand and manifest of a native task from its authored command, or the `build` of a component task (its driver compiles the one crate); every other task is not a Cargo task. */
export function bazelCargoClassV1(task:{definition:{execution:{command?:string};metadata?:unknown}}):BazelCargoClassV1|undefined{
 const kind=(task.definition.metadata as {semio?:{execution?:{kind?:string}}}|undefined)?.semio?.execution?.kind,command=task.definition.execution.command;
 if(typeof command!=="string")return undefined;
 if(kind==="orchestration"){const match=new RegExp(policy.componentCommand,"u").exec(command);return match?{subcommand:"build",manifest:match[1]!.replace(/^"|"$/gu,"")}:undefined;}
 if(kind!=="native")return undefined;
 const match=new RegExp(policy.nativeCommand,"u").exec(command);
 return match?{subcommand:match[1] as BazelCargoClassV1["subcommand"],manifest:match[2]!.replace(/^"|"$/gu,"")}:undefined;
}

/** 🚪️ Renders the exact argument vector after `repo`: launcher options first, the Cargo subcommand next, the machine-readable message format last, so Bazel keys what the ingress receives. */
export function bazelCargoArgumentsV1(action:BazelCargoActionV1):string[]{
 if(validateJsonSchemaSubset(actionSchema,action,componentSchema).length)throw new Error(`Closed Bazel Cargo action required: ${action.id}`);
 return[policy.launcher.command,"--store="+policy.launcher.store,"--configuration="+action.configuration,"--parallelism="+policy.launcher.parallelism,policy.launcher.delimiter,action.subcommand,"--manifest-path",action.manifest,"-p",action.package,action.selection,...(action.target?["--target",action.target]:[]),...(action.features?.length?["--features",[...action.features].sort().join(",")]:[]),policy.messageFormat];
}

/** 📕️ Builds the closed runner manifest: it holds no workspace path, so the same action key is valid on every host. */
export function bazelCargoManifestV1(action:BazelCargoActionV1):BazelCargoManifestV1{
 const manifest:BazelCargoManifestV1={version:1,id:action.id,package:action.package,toolchain:action.toolchain,argv:bazelCargoArgumentsV1(action),artifacts:action.artifacts,suffixes:[...(action.suffixes??[])].sort()};
 if(validateJsonSchemaSubset(manifestSchema,manifest,componentSchema).length)throw new Error(`Closed Bazel Cargo manifest required: ${action.id}`);
 return manifest;
}

/** 📦️ Selects the published files of one package from Cargo's JSON messages: only `compiler-artifact` rows of that package, filtered by suffix, deduplicated and sorted by file name. */
export function bazelCargoArtifactsV1(lines:readonly string[],pack:string,suffixes:readonly string[]):string[]{
 const found=new Map<string,string>();
 for(const line of lines){
  if(!line.startsWith('{"reason":"compiler-artifact"'))continue;
  const message=JSON.parse(line) as {package_id:string;filenames:string[]};
  if(!message.package_id.includes("#"+pack+"@"))continue;
  for(const file of message.filenames)if(suffixes.some(suffix=>file.endsWith(suffix)))found.set(file.slice(file.lastIndexOf("/")+1),file);
 }
 return[...found.entries()].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([,file])=>file);
}

/** 🧊️ Hashes the toolchain without spawning a compiler: the pinned `rust-toolchain.toml` and, when installed, the exact channel manifest of the resolved toolchain, so a reinstalled or changed toolchain re-keys every Cargo action. */
export async function bazelCargoToolchainDigestV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform+"-"+process.arch):Promise<string>{
 const read=async(path:string)=>{try{return await readFile(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}};
 const pinned=await read(join(root,policy.toolchain.file)),channel=/^\s*channel\s*=\s*"([^"]+)"/mu.exec(pinned?.toString("utf8")??"")?.[1],host=(policy.toolchain.hosts as Record<string,string>)[platform],home=environment.RUSTUP_HOME??join(homedir(),".rustup");
 const installed=channel&&host?await read(join(home,"toolchains",channel+"-"+host,policy.toolchain.manifest)):undefined;
 return hash(JSON.stringify({file:pinned?hash(pinned):null,channel:channel??null,host:host??null,installed:installed?hash(installed):null}));
}

/** 📑 The Starlark rule of one typed Cargo action: it runs the action runner with the closed manifest, declares exactly its source files and publishes one directory holding the receipt and artifacts. */
export const BAZEL_CARGO_RULE_V1=`def cargo_resources(os, inputs):
    return {"cpu": ${BAZEL_CARGO_EXCLUSIVE_V1.cpu}}

def _cargo_impl(ctx):
    output = ctx.actions.declare_directory(ctx.label.name + ".cargo")
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.runner.path, ctx.file.manifest.path, output.path], inputs = depset([ctx.file.manifest, ctx.file.runner], transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "RepoCargo", progress_message = ctx.attr.identity, execution_requirements = {"no-sandbox": "1", "no-remote-exec": "1"}, resource_set = cargo_resources, use_default_shell_env = True)
    return [DefaultInfo(files = depset([output]))]

repo_cargo = rule(implementation = _cargo_impl, attrs = {"manifest": attr.label(allow_single_file = True), "runner": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True)})
`;

/** 🏗️ Renders the BUILD text of typed Cargo actions: one repo_cargo rule per action in label order, each naming its manifest, runner and source labels in sorted order. */
export function renderBazelCargoBuildV1(input:{load:string;bun:string;runner:string;actions:readonly {label:string;id:string;manifest:string;srcs:readonly string[]}[]}):string{
 const quote=(value:string)=>JSON.stringify(value),order=(left:string,right:string)=>left<right?-1:left>right?1:0,lines=[`load(${quote(input.load)}, "repo_cargo")`,""];
 for(const action of [...input.actions].sort((left,right)=>order(left.label,right.label)))lines.push("repo_cargo(",`    name = ${quote(action.label)},`,`    manifest = ${quote(action.manifest)},`,`    runner = ${quote(input.runner)},`,"    srcs = ["+[...new Set(action.srcs)].sort(order).map(value=>`\n        ${quote(value)},`).join("")+"\n    ],",`    bun = ${quote(input.bun)},`,`    identity = ${quote(action.id)},`,")","");
 return lines.join("\n");
}
