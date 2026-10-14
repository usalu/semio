import assert from "node:assert/strict";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {join,relative,resolve} from "node:path";
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated"),library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),dashboard=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard");
const {buildGraphPathV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts")),expected=relative(root,buildGraphPathV1(root)).replaceAll("\\","/");
if(process.argv[2]==="red"){
 const source=await readFile(join(dashboard,"🎮️registry/🦀️.rs"),"utf8"),actual=/const GRAPH_FILE: &str = "([^"]+)"/.exec(source)?.[1];console.log("[DEBUG] Dashboard publication path witness "+JSON.stringify({actual,expected}));assert.equal(actual,expected);
}else{
 const scratch=join(output,"dp");await mkdir(scratch,{recursive:true});
 await writeFile(join(scratch,"Cargo.toml"),'[package]\nname="dashboard-publication-witness"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[dependencies]\nserde_json="1"\n[[bin]]\nname="publication"\npath="main.rs"\n');
 await writeFile(join(scratch,"main.rs"),`#[path=${JSON.stringify(join(library,"🏗️build/🕸️graph/🦀️rust/🦀️.rs"))}] mod publication;\nfn main(){println!("{}",publication::graph_file());}\n`);
 const child=Bun.spawn(["cargo","run","--offline","--quiet","--manifest-path",join(scratch,"Cargo.toml")],{cwd:root,env:{...process.env,CARGO_TARGET_DIR:join(output,"dc"),CARGO_NET_OFFLINE:"true"},stdout:"pipe",stderr:"pipe"}),timer=setTimeout(()=>child.kill("SIGTERM"),60000);try{const[stdout,stderr,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);assert.equal(code,0,stderr);assert.equal(stdout.trim(),expected);console.log("[DEBUG] Actual freshly compiled Rust publication helper agrees with neutral TypeScript "+expected);}finally{clearTimeout(timer);}
}
