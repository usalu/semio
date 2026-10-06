import {mkdirSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawn} from "node:child_process";
const root=process.cwd(),generated=resolve(import.meta.dir,"../🗑️generated"),output=join(generated,"native-inspect"),sdk="C:/Program Files (x86)/Windows Kits/10",sdkVersion="10.0.26100.0",msvc="C:/Program Files/Microsoft Visual Studio/18/Community/VC/Tools/MSVC/14.51.36231",native="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
mkdirSync(output,{recursive:true});const binary=join(output,"native-inspect.exe");
if(process.argv[2]==="build"){
  const result=Bun.spawnSync([join(msvc,"bin/Hostx64/x64/cl.exe"),"/nologo","/O2","/Oi","/GS-","/Zl","/Gs99999999","/Tc"+join(import.meta.dir,"📓️probe.c.md"),"/Fo"+join(output,"probe.obj"),"/Fe"+binary,"/I"+join(msvc,"include"),... ["shared","um","ucrt"].map(path=>"/I"+join(sdk,"Include",sdkVersion,path)),"/link","/NODEFAULTLIB","/ENTRY:ProbeMain","/SUBSYSTEM:CONSOLE",... ["kernel32.lib","dbghelp.lib"].map(path=>join(sdk,"Lib",sdkVersion,"um/x64",path))],{stdout:"inherit",stderr:"inherit"});if(result.exitCode)process.exit(result.exitCode);
}else{
  const {nativeRunnerEnvironment}=await import(join(root,native,"⌨️native-entrypoint/📜️script.ts"));
  const target=process.argv[2]==="original"?join(root,native,"📦️packages/🦀️rust/dist/native-dev/semio-wgpu-native.exe"):join(generated,"native-stack-diagnostic.exe");
  const child=spawn(binary,[target,"--plugin","draw","--app","s.draw.drawing@1/*#editor","--example","🎬️demo","--smoke"],{cwd:root,env:nativeRunnerEnvironment({...process.env,SEMIO_PLUGIN_MODULES:join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/dev/draw"),SEMIO_LOCKED_LOCALE:"en",SEMIO_LOCKED_TERMINOLOGY:"native"}),stdio:["ignore","pipe","pipe"],windowsHide:true});
  let log="";for(const stream of [child.stdout,child.stderr])stream.on("data",chunk=>{log+=chunk;process.stdout.write(chunk);});const status=await new Promise<number|null>((accept,reject)=>{child.once("error",reject);child.once("close",accept);});writeFileSync(join(output,`${process.argv[2]}.log`),log);process.exit(status??1);
}
