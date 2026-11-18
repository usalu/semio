import{plugin}from"bun";
import{existsSync,readFileSync}from"node:fs";
import{dirname,resolve}from"node:path";
const map=JSON.parse(readFileSync(process.env.R109_OVERLAY!,"utf8")) as Record<string,string>;
const paths=Object.keys(map).map(path=>path.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"));
plugin({name:"r109-overlay",setup(build){
 build.onResolve({filter:/(^\.\.?\/|^\/)/u},({path,importer})=>{const absolute=resolve(dirname(importer),path);return map[absolute]!==undefined&&!existsSync(absolute)?{path:absolute}:undefined;});
 build.onLoad({filter:new RegExp("("+paths.join("|")+")$","u")},({path})=>{
  const staged=map[path];if(staged===undefined)return undefined;
  const contents=readFileSync(staged,"utf8");
  return path.endsWith(".json")?{exports:{default:JSON.parse(contents)},loader:"object"}:{contents,loader:path.endsWith(".ts")?"ts":"js"};
 });
}});
