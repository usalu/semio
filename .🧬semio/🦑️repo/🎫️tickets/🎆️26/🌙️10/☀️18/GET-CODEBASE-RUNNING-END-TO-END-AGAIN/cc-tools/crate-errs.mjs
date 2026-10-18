import {readFileSync} from "node:fs";
const [log,only,limit]=process.argv.slice(2);
const lines=readFileSync(log,"utf8").split("\n");
let buf=[];const out=new Map();
for(const l of lines){
 if(/^\[script\]/.test(l))continue;
 const m=l.match(/^error: could not compile `([^`]+)`/);
 if(m){out.set(m[1],buf);buf=[];continue;}
 if(/: error(\[E\d+\])?:/.test(l)||/^error(\[E\d+\])?:/.test(l)&&!l.startsWith("error: script")&&!l.startsWith("error: cargo"))buf.push(l.replace(/\/📦️packages\/🦀️rust\/\.\.\/\.\./,"/<pkg>/.."));
}
for(const [c,b] of out){if(only&&only!=="-"&&c!==only)continue;console.log("=== "+c+" ("+b.length+")");for(const x of b.slice(0,Number(limit??12)))console.log("  "+x.slice(0,420));}
