import {existsSync,readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
const worktree=process.argv[2]!,images=join(worktree,".cache","b");
const boot=await import(join(worktree,"🧰️framework","🔨️modules","🏃️process","📦️artifacts","🏗️native-build","💾️storage","🌐️platform","🥾️boot","🟦️.ts"));
const rows:{partition:string;bound:boolean;reason?:string}[]=[];
for(const partition of readdirSync(images).sort()){
 const directory=join(images,partition);
 if(!existsSync(join(directory,"image.json")))continue;
 boot.admitNativeImageRelocationV1(directory);
 try{boot.requireNativeImageAncestryBindingV1({published:JSON.parse(readFileSync(join(directory,"image.json"),"utf8")).ancestry,current:boot.readNativeImageAncestryV1(directory)});rows.push({partition,bound:true});}
 catch(error){let reason=String((error as Error).message);try{const hint=JSON.parse(readFileSync(join(images,"image-relocation.json"),"utf8"));boot.rebindNativeImageAncestryV1({directory,movedFrom:hint.movedFrom,movedTo:hint.movedTo});}catch(inner){reason=String((inner as Error).message);}rows.push({partition,bound:false,reason});}
}
for(const row of rows)console.log(`${row.bound?"bound":"unbound"}\t${row.partition}${row.reason?`\t${row.reason}`:""}`);
process.exit(rows.every(row=>row.bound)?0:1);
