import {spawnSync} from "node:child_process";
import {realpathSync,statSync,copyFileSync,existsSync,mkdirSync,readdirSync,readFileSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";

/** 🧪️ Generates the scratch Bazel workspace of typed Cargo actions over the scratch Cargo workspace; mode `glob` declares owner directory trees, `exact` declares rustc dep-info files. */
const root=process.cwd(),mode=process.argv[2]??"glob",base="/private/tmp/bzlrust",bw=join(base,"bw");
const overlay=join(import.meta.dir,"overlay"),mod=await import(join(overlay,readdirSync(overlay).find(name=>name.endsWith(".ts")&&name!=="adapter.ts")!));
const ws=process.env.BZLRUST_WS!,wsRel=relative(root,resolve(ws));
const walk=(dir:string):string[]=>readdirSync(dir,{withFileTypes:true}).flatMap(entry=>entry.name==="node_modules"?[]:entry.isDirectory()?walk(join(dir,entry.name)):[join(dir,entry.name)]);
const crates=[
 {label:"c_primitives",package:"bzlrust-primitives",dir:"geometry/"+readdirSync(join(ws,"geometry")).find(name=>name.startsWith("⚙"))!,deps:[] as string[],subcommand:"check"},
 {label:"c_vector",package:"bzlrust-vector",dir:"geometry/"+readdirSync(join(ws,"geometry")).find(name=>name.startsWith("➗"))!,deps:["c_primitives"],subcommand:"check"},
 {label:"c_bulge",package:"bzlrust-bulge",dir:"geometry/"+readdirSync(join(ws,"geometry")).find(name=>name.startsWith("🌙"))!,deps:["c_primitives","c_vector"],subcommand:"check"},
 {label:"b_wasm",package:"bzlrust-wasm",dir:"wasm-demo",deps:[] as string[],subcommand:"build"},
];
const closure=(crate:typeof crates[number])=>{
 const own=[crate.dir,...crate.deps.map(label=>crates.find(row=>row.label===label)!.dir)];
 if(mode==="exact"){const exact=JSON.parse(readFileSync(join(import.meta.dir,"exact-inputs.json"),"utf8"))[crate.label] as string[];return exact;}
 return own.flatMap(dir=>walk(join(ws,dir)).map(path=>relative(root,path)));
};
const common=[join(wsRel,"Cargo.toml"),join(wsRel,"Cargo.lock"),"rust-toolchain.toml",".cargo/config.toml"];
rmSync(join(bw),{recursive:true,force:true});mkdirSync(bw,{recursive:true});
writeFileSync(join(bw,"MODULE.bazel"),'module(name = "bzlrust", version = "0.0.0")\n');
writeFileSync(join(bw,"tasks.bzl"),mod.BAZEL_CARGO_RULE_V1);
const actionDirectory=join(overlay,readdirSync(overlay).find(name=>statSync(join(overlay,name)).isDirectory()&&!name.includes("schema")&&!name.includes("tests"))!);copyFileSync(join(actionDirectory,readdirSync(actionDirectory)[0]!),join(bw,"runner.ts"));
symlinkSync(root,join(bw,"s"),"dir");
const toolchain=await mod.bazelCargoToolchainDigestV1(root);
const actions=[];
for(const crate of crates){
 const build=crate.subcommand==="build",action={version:1,id:crate.label,subcommand:crate.subcommand,manifest:relative(root,walk(join(ws,crate.dir)).find(path=>path.endsWith("/Cargo.toml"))!),package:crate.package,selection:"--lib",configuration:"prototype",toolchain,artifacts:build?"files":"none",...(build?{target:"wasm32-wasip2",suffixes:[".wasm"]}:{})};
 writeFileSync(join(bw,crate.label+".json"),JSON.stringify(mod.bazelCargoManifestV1(action)));
 actions.push({label:crate.label,id:crate.label,manifest:crate.label+".json",srcs:[...closure(crate),...common].map(path=>"s/"+path)});
}
writeFileSync(join(bw,"BUILD.bazel"),mod.renderBazelCargoBuildV1({load:":tasks.bzl",bun:process.execPath,runner:"runner.ts",actions})+`
genrule(name = "all", srcs = [${actions.map(action=>JSON.stringify(":"+action.label)).join(", ")}], outs = ["all.txt"], cmd = "echo ok > $@")
`);
const jco=realpathSync(join(root,"node_modules/.bin/jco")),opt=realpathSync(join(root,"node_modules/.bin/wasm-opt")),node=process.env.BZLRUST_NODE!,version=(program:string,args:string[])=>spawnSync(program,args,{encoding:"utf8"}).stdout.trim();
writeFileSync(join(bw,"stage.bzl"),`def _impl(ctx):
    out = ctx.actions.declare_directory(ctx.label.name + ".stage")
    ctx.actions.run_shell(inputs = [d[DefaultInfo].files.to_list()[0] for d in ctx.attr.deps], outputs = [out], command = ctx.attr.command, env = {"IN": ctx.attr.deps[0][DefaultInfo].files.to_list()[0].path, "OUT": out.path, "TOOL": ctx.attr.identity}, execution_requirements = {"no-sandbox": "1", "no-remote-exec": "1"})
    return [DefaultInfo(files = depset([out]))]
repo_stage = rule(implementation = _impl, attrs = {"deps": attr.label_list(), "command": attr.string(), "identity": attr.string()})
`);
writeFileSync(join(bw,"BUILD.bazel"),readFileSync(join(bw,"BUILD.bazel"),"utf8")+`
load(":stage.bzl", "repo_stage")
repo_stage(name = "t_jco", deps = [":b_wasm"], identity = ${JSON.stringify("jco "+version(node,[jco,"--version"])+" node "+process.version)}, command = ${JSON.stringify(`export PATH=${dirname(node)}:/usr/bin:/bin; ${node} ${jco} transpile $IN/artifacts/bzlrust_wasm.wasm -o $OUT --no-typescript >/dev/null`)})
repo_stage(name = "t_opt", deps = [":t_jco"], identity = ${JSON.stringify("wasm-opt "+version(node,[opt,"--version"]))}, command = ${JSON.stringify(`export PATH=${dirname(node)}:/usr/bin:/bin; for f in $IN/*.core.wasm; do ${node} ${opt} -Os --all-features "$f" -o $OUT/$(basename "$f"); done`)})
`);
const environment=Object.fromEntries(Object.entries(process.env).filter((entry):entry is [string,string]=>entry[1]!==undefined));
writeFileSync(join(bw,"runtime.json"),JSON.stringify({workspace:root,environment,journal:join(base,"spawns.log")}));
console.log(mode,"toolchain",toolchain.slice(0,12),"actions",actions.map(action=>action.label+":"+action.srcs.length).join(" "));
