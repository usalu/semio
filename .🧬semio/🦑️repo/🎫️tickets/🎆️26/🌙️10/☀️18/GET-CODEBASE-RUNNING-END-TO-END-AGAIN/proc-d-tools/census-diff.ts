import ts from "typescript";
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
const fx=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🧫️fixtures/🔣️.json","utf8"));
const names=["withCompilerCapacityScopeV1","acquireCompilerCapacityLeaseV1","withCompilerCapacityConsumptionV1"];
const rows:any[]=[];
for(const path of fx.callers){const full=join("/Users/ueli/Documents/semio",path);if(!existsSync(full)){console.log("MISSING",path);continue;}const text=readFileSync(full,"utf8"),source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true),forwarded:any[]=[];let literalCalls=0;const walk=(node:ts.Node)=>{if(ts.isCallExpression(node)&&names.includes(node.expression.getText(source))){const o=node.arguments[0];if(o&&ts.isObjectLiteralExpression(o))literalCalls++;else forwarded.push({callee:node.expression.getText(source),argument:o?.getText(source)??""});}ts.forEachChild(node,walk);};walk(source);rows.push({path,literalCalls,forwarded});}
const old=new Map(fx.census.rows.map((r:any)=>[r.path,r]));
for(const r of rows){const o:any=old.get(r.path);if(!o||o.literalCalls!==r.literalCalls||JSON.stringify(o.forwarded)!==JSON.stringify(r.forwarded))console.log("DIFF",r.path.split("/").slice(-4).join("/"),"old",o?.literalCalls,"new",r.literalCalls,JSON.stringify(r.forwarded).slice(0,120));}
console.log("total",rows.reduce((s,r)=>s+r.literalCalls,0),"files",rows.length);
