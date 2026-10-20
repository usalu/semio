import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,writeFileSync} from "node:fs";
const SRC=process.argv[2];
const src=readFileSync(SRC,"utf8");
const sf=ts.createSourceFile("x.ts",src,ts.ScriptTarget.Latest,true);
const G:Record<string,number[]>={
 "🛂️admission":[81,93,98,103,112,124,129,138,143,499,506,519,525,681,696,746,759,772,783,865],
 "🧭️portable-cli":[157,267,353,373,165,177,199,364,408,553],
 "📐️portable-model":[72,148,171,234,397,493,512],
 "📌️pinned-publication":[240,257,283,288,308,317,442,447,575,686,719,739],
 "🔭️observed-facts":[303,414,422,433,461,887,908],
 "🔌️nx-engine":[193,206,215,563,590,606,616,629,641,675,920,930,940],
 "🚀️dependency-bootstrap":[650,659,666],
 "🧊️optional-cache":[63,797,813,822,836,843,850,856,876],
 "🌐️source-discovery":[327,342,471,483,706,731],
 "🎯️source-demand":[953,957,963,968,973,978,983,988,992,999,1004,1010,1015,1019,1028,1036],
};
const byLine=new Map<number,string>();
for(const [g,ls] of Object.entries(G))for(const l of ls){if(byLine.has(l))throw Error("dup "+l);byLine.set(l,g);}
const out:Record<string,string>={};const seen=new Set<number>();
for(const st of sf.statements){
  if(!ts.isExpressionStatement(st))continue;
  const call=st.expression;if(!ts.isCallExpression(call))continue;
  const callee=call.expression.getText(sf);
  if(!(callee==="test"||callee==="registerBuildTestLawV1"||callee.startsWith("test.skipIf(")||callee.startsWith("test.each("))) continue;
  const line=sf.getLineAndCharacterOfPosition(st.getStart()).line+1;
  const title=(call.arguments[0] as ts.StringLiteral).text;
  const g=byLine.get(line);
  if(!g){console.error("UNASSIGNED",line,title);continue;}
  seen.add(line);out[title]=g;
}
for(const l of byLine.keys())if(!seen.has(l))console.error("NO TEST AT",l);
writeFileSync(process.argv[3],JSON.stringify(out,null,1));
console.log(Object.keys(out).length,"titles");
