/** 🧪️ Loads the staged W14 files (`list.tsv`: staged name, checkout path under the build library or absolute) in place of the checkout's, by exact path, so a test file runs against the staged code while the checkout is unchanged. */
import{plugin}from"bun";
import{readFileSync}from"node:fs";
import{join}from"node:path";
const lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const map:Record<string,string>=Object.fromEntries(readFileSync(join(import.meta.dir,"list.tsv"),"utf8").trim().split("\n").filter(line=>line&&!line.startsWith("#")).map(line=>line.split("\t")).map(([staged,target])=>[target!.startsWith("/")?target!:lib+target,join(import.meta.dir,"staged",staged!)]));
const esc=(text:string)=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
plugin({name:"w14-staged",setup(build){build.onLoad({filter:new RegExp("^("+Object.keys(map).map(esc).join("|")+")$","u")},args=>{const staged=map[args.path]!,text=readFileSync(staged,"utf8");return staged.endsWith(".json")?{exports:{default:JSON.parse(text)},loader:"object"}:{contents:text,loader:staged.endsWith(".ts")?"ts":"js"};});}});
