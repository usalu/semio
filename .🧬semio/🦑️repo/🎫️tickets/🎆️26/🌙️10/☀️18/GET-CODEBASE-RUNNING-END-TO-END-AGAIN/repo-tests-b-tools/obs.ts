import ts from "typescript";import{readFileSync}from"node:fs";import{join}from"node:path";import Ajv from "ajv";
const root=process.cwd(),path=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher"),text=readFileSync(join(path,"🟦️.ts"),"utf8"),source=ts.createSourceFile("l.ts",text,ts.ScriptTarget.Latest,true);
let a=0,c=0,g=false;const args:string[]=[];
const walk=(n:ts.Node)=>{if(ts.isCallExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text==="readNativeStorageReservationPolicyV1"){a++;args.push(n.arguments.map(x=>x.getText(source)).join(","));g=args.at(-1)==="store.storage,store.control";}if(ts.isStringLiteral(n)&&n.text===".cache/b/l/m/data")c++;ts.forEachChild(n,walk);};walk(source);
const policy=JSON.parse(readFileSync(join(path,"🧬️schema/🔣️.json"),"utf8")).definitions.Configuration.const;
console.log(JSON.stringify({a,c,g,args,cap:Object.hasOwn(policy,"capacityLeaseDirectory")}));
const schema=JSON.parse(readFileSync(process.argv[2],"utf8"));const v=new Ajv({strict:true}).compile(schema.definitions.NamespaceObservation);console.log(v({version:1,authenticatedPolicyReads:a,constantNamespaceUses:c,independentCapacityDirectory:Object.hasOwn(policy,"capacityLeaseDirectory"),actualGuardControl:g}),JSON.stringify(v.errors));
