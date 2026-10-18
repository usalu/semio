import ts from "typescript";
import { readFileSync } from "node:fs";
const A="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/";
const fx=JSON.parse(readFileSync(A+"📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️tests/🧫️fixtures/🔣️.json","utf8"));
const path=A+"📮️shard-client/🟦️.ts";
const shard=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
for(const row of fx.layouts){
  const decl=shard.statements.find((n:any)=>(ts.isClassDeclaration(n)||ts.isTypeAliasDeclaration(n))&&n.name?.text===row.declaration) as any;
  const members=decl&&ts.isClassDeclaration(decl)?decl.members.filter(ts.isPropertyDeclaration):decl&&ts.isTypeAliasDeclaration(decl)&&ts.isTypeLiteralNode(decl.type)?decl.type.members:null;
  const fields=members.map((m:any)=>m.name?.getText(shard).replace(/^#/,""));
  const add=fields.filter((f:string)=>!row.fields.includes(f)), del=row.fields.filter((f:string)=>!fields.includes(f));
  console.log(row.declaration,row.group,"actual",fields.length,"fixture",row.fields.length,"added",add,"removed",del, JSON.stringify(fields)===JSON.stringify(row.fields)?"SAME":"DIFF");
}
const variants=(name:string)=>{const d=shard.statements.find((n:any)=>ts.isTypeAliasDeclaration(n)&&n.name.text===name) as any;return d.type.types.map((type:any)=>{const props=type.members.filter(ts.isPropertySignature);const kind=props.find((i:any)=>i.name.getText(shard)==="kind")?.type;const ok=props.find((i:any)=>i.name.getText(shard)==="ok")?.type;return {kind:kind.literal.text,ok:ok&&ts.isLiteralTypeNode(ok)?ok.literal.kind===ts.SyntaxKind.TrueKeyword:null,fields:props.map((i:any)=>i.name.getText(shard))}})};
for(const [n,k] of [["OutboundMessage","outbound"],["InboundMessage","inbound"]] as const){const a=variants(n);console.log(n,JSON.stringify(a)===JSON.stringify(fx[k])?"SAME":"DIFF");if(JSON.stringify(a)!==JSON.stringify(fx[k])){console.log(JSON.stringify(a));console.log(JSON.stringify(fx[k]))}}
if (process.argv[2]==="--apply"){
  const out=JSON.parse(JSON.stringify(fx));
  for(const row of out.layouts){
    const decl=shard.statements.find((n:any)=>(ts.isClassDeclaration(n)||ts.isTypeAliasDeclaration(n))&&n.name?.text===row.declaration) as any;
    const members=ts.isClassDeclaration(decl)?decl.members.filter(ts.isPropertyDeclaration):decl.type.members;
    row.fields=members.map((m:any)=>m.name?.getText(shard).replace(/^#/,""));
  }
  const totals:any={};
  for(const row of out.layouts){const t=totals[row.group]??={bytes:0,slots:0,owners:0};t.bytes+=out.model.recordBytes+row.fields.length*out.model.fieldBytes;t.slots+=1;t.owners+=1;}
  for(const g of Object.keys(out.minimumShells)) out.minimumShells[g]=totals[g];
  out.outbound=variants("OutboundMessage"); out.inbound=variants("InboundMessage");
  const fs=await import("node:fs");
  fs.writeFileSync(A+"📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧪️tests/🧫️fixtures/🔣️.json",JSON.stringify(out,null,2)+"\n");
}
