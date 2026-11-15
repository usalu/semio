import {cpSync,existsSync,mkdirSync,readFileSync,writeFileSync,copyFileSync} from "node:fs";
import {dirname,join,normalize} from "node:path";

const SRC="/Users/Shared/semio",DST=process.argv[2]!,CRATE=process.argv[3]??"🧰️framework/🛍️products/🐾️pets/🔨️modules/📜️contract/📦️packages/🦀️rust";
const workspace=(Bun.TOML.parse(readFileSync(`${SRC}/Cargo.toml`,"utf8")) as any).workspace.dependencies as Record<string,any>;
const EXTRA=(process.argv[4]??"🧰️framework/🛍️products/🐾️pets").split(",");const members:string[]=[],seen=new Set<string>(),todo=[CRATE];
while(todo.length){
 const rel=todo.pop()!;if(seen.has(rel))continue;seen.add(rel);members.push(rel);
 const manifest=Bun.TOML.parse(readFileSync(`${SRC}/${rel}/Cargo.toml`,"utf8")) as any;
 for(const section of["dependencies","dev-dependencies","build-dependencies"])for(const [name,raw] of Object.entries<any>(manifest[section]??{})){
  const spec=raw?.workspace?workspace[name]:raw;
  if(spec&&typeof spec==="object"&&spec.path)todo.push(normalize(join(rel,spec.path)));
 }
}
for(const extra of EXTRA)if(!existsSync(`${DST}/${extra}`))cpSync(`${SRC}/${extra}`,`${DST}/${extra}`,{recursive:true,filter:path=>!/\/(node_modules|target|\.cache)(\/|$)/u.test(path)});
for(const rel of members){
 const module=rel.endsWith("📦️packages/🦀️rust")?dirname(dirname(rel)):rel,target=`${DST}/${module}`;
 if(!existsSync(target))cpSync(`${SRC}/${module}`,target,{recursive:true,filter:path=>!/\/(node_modules|target|\.cache)(\/|$)/u.test(path)});
 mkdirSync(`${DST}/${rel}`,{recursive:true});writeFileSync(`${DST}/${rel}/Cargo.toml`,readFileSync(`${SRC}/${rel}/Cargo.toml`,"utf8").replace(/^workspace = ".*"$/mu,`workspace = "${"../".repeat(rel.split("/").length).slice(0,-1)}"`));
}
const paths=Object.entries(workspace).filter(([,s])=>s&&typeof s==="object"&&s.path&&existsSync(`${DST}/${s.path}`)).map(([n,s])=>`${n} = { path = "${s.path}" }`);
writeFileSync(`${DST}/Cargo.toml`,`[workspace]\nresolver = "2"\nmembers = [${members.map(m=>`"${m}"`).join(",")}]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n\n[workspace.dependencies]\nserde = { version = "1.0.228", features = ["derive"] }\nserde_json = { version = "1.0.149", features = ["raw_value"] }\n${paths.join("\n")}\n\n[workspace.lints.rust]\nfuture_incompatible = { level = "warn", priority = -1 }\nrust_2018_idioms = { level = "warn", priority = -1 }\n\n[workspace.lints.clippy]\nall = { level = "warn", priority = -1 }\n`);
mkdirSync(`${DST}/.cargo`,{recursive:true});copyFileSync(`${SRC}/rust-toolchain.toml`,`${DST}/rust-toolchain.toml`);
writeFileSync(`${DST}/.cargo/config.toml`,`[unstable]\nembed-metadata = "DoNotEmbed"\nfine-grain-locking = true\nbuild-dir-new-layout = true\nchecksum-freshness = true\n\n[build]\nrustflags = ["-Z", "threads=0"]\n`);
console.log(members.length,"members");
