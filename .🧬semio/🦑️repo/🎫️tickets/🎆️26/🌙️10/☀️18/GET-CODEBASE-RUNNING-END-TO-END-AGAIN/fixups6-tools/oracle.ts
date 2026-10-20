import {readFileSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const ROOT="/Users/ueli/Documents/semio",HERE=import.meta.dir;
const map=JSON.parse(readFileSync(join(HERE,"mapping.json"),"utf8")) as {base:string;tests:{title:string;group:string;file:string}[];support:string;groups:string[]};
const live=process.env.SPLIT_TREE&&existsSync(join(process.env.SPLIT_TREE,map.base))?join(process.env.SPLIT_TREE,map.base):join(ROOT,map.base);
const text=(p:string)=>readFileSync(p,"utf8");
const IMPORT=/^import\s*(?:[\w${}*,\s]+?\s*from\s*)?["'][^"']+["'](?:\s+with\s*\{[^}]*\})?;?$/u;
const REL=/(["'`])((?:\.\.?\/)+)/gu;
const strip=(l:string)=>l.replace(REL,"$1");
const relLits=(l:string)=>[...l.matchAll(/["'`](\.\.?(?:\/[^"'`]*)?)["'`]/gu)].map(m=>m[1]!);
const REVIEWED=new Set([".","..","../","../../"]);
const origLines=text(live).split("\n");
const origDir=dirname(join(ROOT,map.base));
const REQ=origLines.find(l=>l.startsWith("const require=createRequire(import.meta.url)"))!;
const isImport=(l:string)=>IMPORT.test(l.trim())&&!/^\s/u.test(l);
const origBody=origLines.filter(l=>l.trim()!==""&&!isImport(l)&&l!==REQ);
const origCount=new Map<string,number>(),origByStrip=new Map<string,string[]>();
for(const l of origBody){const k=strip(l);origCount.set(k,(origCount.get(k)??0)+1);(origByStrip.get(k)??origByStrip.set(k,[]).get(k)!).push(l);}
const files=[map.support,...map.groups];
const newCount=new Map<string,number>();
let problems=0;const fail=(m:string)=>{problems++;console.log("FAIL",m);};
let literalPairs=0,rewritten=0,kept=0;
for(const f of files){
  const p=join(ROOT,"/dev/null");
  const staged=join(HERE,"staged",f),newDir=dirname(join(HERE,"staged",f)).replace(join(HERE,"staged"),ROOT);
  const lines=text(staged).split("\n");
  for(let i=0;i<lines.length;i++){
    let l=lines[i]!;
    if(l.trim()==="")continue;
    if(i===0&&l.startsWith("/** ")&&l.endsWith(" */"))continue;
    if(isImport(l))continue;
    if(l===REQ)continue;
    if(f===map.support&&l.startsWith("export "))l=l.slice(7);
    const k=strip(l);
    newCount.set(k,(newCount.get(k)??0)+1);
    const candidates=origByStrip.get(k);
    if(!candidates){fail(`${f}:${i+1} line not in original: ${l.slice(0,100)}`);continue;}
    const B=relLits(l);
    const ok=candidates.some(orig=>{
      const A=relLits(orig);
      if(A.length!==B.length)return false;
      return A.every((a,j)=>{
        const b=B[j]!;
        if(a===b)return REVIEWED.has(a)||!existsSync(resolve(origDir,a))||resolve(origDir,a)===resolve(newDir,b);
        return resolve(newDir,b)===resolve(origDir,a);
      });
    });
    if(!ok)fail(`${f}:${i+1} relative literals not equivalent: ${B.join(" | ").slice(0,160)}`);
    else for(const b of B){literalPairs++;}
  }
}
for(const [k,c] of origCount){const n=newCount.get(k)??0;if(n!==c)fail(`multiset mismatch (orig ${c}, new ${n}): ${k.slice(0,100)}`);}
for(const [k,n] of newCount)if(!origCount.has(k))fail(`new-only line: ${k.slice(0,100)}`);
const titles=map.tests.map(t=>t.title);
const origTitleCount=(text(live).match(/^(?:test(?:\.skipIf\([^\n]*?\)|\.each\([^\n]*?\))?|registerBuildTestLawV1)\(?"/gmu)??[]).length;
console.log(JSON.stringify({origNonImportLines:origBody.length,newNonImportLines:[...newCount.values()].reduce((a,b)=>a+b,0),tests:titles.length,topLevelRegistrationsByRegex:origTitleCount,relativeLiteralsChecked:literalPairs,problems}));
process.exit(problems?1:0);
