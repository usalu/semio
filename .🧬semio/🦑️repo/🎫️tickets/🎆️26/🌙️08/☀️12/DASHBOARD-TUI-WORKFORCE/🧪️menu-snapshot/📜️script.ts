import {writeFileSync,readFileSync,copyFileSync,existsSync} from "node:fs";
import {resolve,join} from "node:path";
const generated=resolve(import.meta.dir,"../🗑️generated"),binary=process.argv[2]!,file=join(generated,"command-tree.json");
const child=Bun.spawn([binary,"command-tree","--dump-tree"],{cwd:process.cwd(),stdout:"pipe",stderr:"pipe"});
const [output,error,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);if(status!==0)throw Error(error);
const tree=JSON.parse(output),targets:string[]=[];
function visit(node:any):void {if(node.leaf?.args?.[0]==="nx"&&node.leaf?.args?.[1]==="run")targets.push(node.leaf.args[2]);for(const child of node.children??[])visit(child);}
visit(tree);if(!targets.includes("@semio-tech/draw-plugin:materialize-dev"))throw Error("Actual menu omitted Draw materialization");
if(!targets.includes("@semio-tech/repo-lib:test-cargo-library-search-path"))throw Error("Actual menu omitted the registered Cargo test");
if(existsSync(file)&&!existsSync(join(generated,"command-tree-original.json")))copyFileSync(file,join(generated,"command-tree-original.json"));
writeFileSync(file,output);console.log(`[DEBUG] actual menu targets=${targets.length} materialization=${targets.filter(value=>value.includes(":materialize-")).length}`);
