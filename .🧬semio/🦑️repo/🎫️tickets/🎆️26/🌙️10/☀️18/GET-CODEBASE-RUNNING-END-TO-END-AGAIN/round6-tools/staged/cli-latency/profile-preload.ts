import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
/** Preload for SEMIO_REPO_PROFILE runs: wraps selected function declarations of the live sources in profile phases without editing them. Targets: {path suffix: {function name: phase}}. */
const root=process.env.SEMIO_PROFILE_ROOT??process.cwd();
const profileModule=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⏱️profile/🟨️.mjs");
(globalThis as any).__prof=(phase:string,body:()=>unknown)=>profileModule.profilePhaseV1(phase,body);
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",P="🧰️framework/🔨️modules/🏃️process";
const targets:Record<string,Record<string,string>>=JSON.parse(readFileSync(process.env.SEMIO_PROFILE_TARGETS!,"utf8"));
const suffixes=Object.entries(targets);
Bun.plugin({name:"repo-profile",setup(build){
 build.onLoad({filter:/\.(ts|mjs)$/},args=>{
  const hit=suffixes.find(([suffix])=>args.path.endsWith(suffix));
  let text=readFileSync(args.path,"utf8"),tail="";
  if(!hit)return {contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};
  for(const [name,phase] of Object.entries(hit[1])){
   const pattern=new RegExp("(^|\\n)(\\s*)(export\\s+)?(async\\s+)?function\\s+"+name+"\\b","u"),match=pattern.exec(text);
   if(!match){process.stderr.write(`[profile-preload] no declaration ${name} in ${hit[0]}\n`);continue;}
   text=text.replace(pattern,(_all,a,b,e,s)=>`${a}${b}${e??""}${s??""}function ${name}__raw`);
   if(args.path.endsWith(".ts"))text=text.replace(new RegExp(`function ${name}__raw(\\s*)<`,"u"),`function ${name}__raw$1<`);
   tail+=`\n${match[3]??""}function ${name}(...a${args.path.endsWith(".ts")?":any[]":""})${args.path.endsWith(".ts")?":any":""}{return globalThis.__prof(${JSON.stringify(phase)},()=>${name}__raw(...a));}\n`;
  }
  return {contents:text+tail,loader:args.path.endsWith(".ts")?"ts":"js"};
 });
}});
