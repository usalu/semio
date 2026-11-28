/** 🧭️ H99 proof without the cluster: runs the patched `build-script` tool text (read from the policy schema, no cluster, no revision gate) the way the worker does (private root with `in/run`, `in/manifest`, `in/tools/rustc` + identity, `OUT_DIR` below `out/`), for a script that asks `rustc --version`, and compares stdout and OUT_DIR with a local run against the real compiler. Usage: bun sim-worker.ts <buildscript dir of the patched overlay>. */
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const base=process.argv[2]!;
const toolText=(JSON.parse(readFileSync(join(base,"☁️offload/🧬️schema/🔣️.json"),"utf8")).policy.tool.lines as string[]).join("\n")+"\n";
const {rustcStandInTextV1}:any=await import(join(base,"🧭️probe/🟦️.ts"));

const compiler=join(spawnSync("rustc",["--print","sysroot"],{encoding:"utf8"}).stdout.trim(),"bin","rustc"),identity=spawnSync(compiler,["-vV"],{encoding:"utf8"}).stdout;
const directory=mkdtempSync(join(tmpdir(),"h99-sim-")),source=`use std::{env,fs,process::Command};
fn main(){
 let out=env::var("OUT_DIR").unwrap();
 let text=String::from_utf8(Command::new(env::var_os("RUSTC").unwrap()).arg("--version").output().unwrap().stdout).unwrap();
 let verbose=String::from_utf8(Command::new(env::var_os("RUSTC").unwrap()).arg("-vV").output().unwrap().stdout).unwrap();
 fs::write(format!("{}/minor.txt",out),format!("{}{}",text,verbose.lines().count())).unwrap();
 println!("cargo:rustc-cfg=probe_{}",text.split('.').nth(1).unwrap());
}
`;
try{
 const script=join(directory,"build.rs"),binary=join(directory,"script");
 writeFileSync(script,source);
 const built=spawnSync(compiler,["--edition","2021",script,"-o",binary],{encoding:"utf8"});if(built.status!==0)throw Error(built.stderr);
 const localOut=join(directory,"local-out");mkdirSync(localOut);
 const local=spawnSync(binary,[],{cwd:directory,env:{PATH:process.env.PATH!,OUT_DIR:localOut,RUSTC:compiler},encoding:"utf8"});
 const root=join(directory,"worker");mkdirSync(join(root,"in/manifest"),{recursive:true});mkdirSync(join(root,"in/tools"),{recursive:true});
 writeFileSync(join(root,"in/run"),readFileSync(binary));chmodSync(join(root,"in/run"),0o644);
 writeFileSync(join(root,"in/tools/rustc"),rustcStandInTextV1());writeFileSync(join(root,"in/tools/rustc.identity"),identity);
 const tool=join(directory,"tool");writeFileSync(tool,toolText);chmodSync(tool,0o755);
 const outDir=join(root,"out/out-dir");
 const remote=spawnSync(tool,[],{cwd:root,env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",OUT_DIR:outDir},encoding:"utf8"});
 const same=remote.status===0&&remote.stdout===local.stdout&&readFileSync(join(outDir,"minor.txt"),"utf8")===readFileSync(join(localOut,"minor.txt"),"utf8");
 console.log(JSON.stringify({status:remote.status,same,stdout:remote.stdout.trim(),refused:existsSync(join(outDir,".semio-stand-in-refused")),stderr:remote.stderr.slice(0,200)}));
 process.exitCode=same?0:1;
}finally{rmSync(directory,{recursive:true,force:true});}
