import {plugin} from "bun";
const subs:[RegExp,string,string][]=[
 [/project-nodes\//,"export async function inferProjectBodyV1(environment, job, admission) {","export async function inferProjectBodyV1(environment, job, admission) { (globalThis.__body??=[]).push(Math.round(performance.now())); return __ib(environment, job, admission); }\nasync function __ib(environment, job, admission) {"],
 [/script-inputs\//,"async function closurePackagesV1(closure, workspaceRoot, graph, checkpoint) {","async function closurePackagesV1(closure, workspaceRoot, graph, checkpoint) { const __t=performance.now(),__s=new Error().stack.split(String.fromCharCode(10)).slice(2,9).map(x=>x.trim().split(' ')[1]+x.slice(x.lastIndexOf(':')-5)).join(' < ');try{return await __cpInner(closure, workspaceRoot, graph, checkpoint);}finally{(globalThis.__cp??=[]).push([closure.length,Math.round(performance.now()-__t),Math.round(__t)]);if(closure.length>1000)(globalThis.__st??=[]).push(__s);} }\nasync function __cpInner(closure, workspaceRoot, graph, checkpoint) {"],
];
plugin({name:"r200-count",setup(b){b.onLoad({filter:/\.mjs$/},async({path})=>{
 let t=await Bun.file(path).text();
 for(const [re,a,r] of subs) if(re.test(path)&&t.includes(a)&&!path.includes("/tests/")) t=t.replace(a,r);
 return {contents:t,loader:"js"};});}});
