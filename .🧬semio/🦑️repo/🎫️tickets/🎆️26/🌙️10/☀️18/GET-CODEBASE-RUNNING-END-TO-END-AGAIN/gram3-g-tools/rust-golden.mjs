// [gram3-g] writes the fixture of law `rust-literals`: Rust sources and the references the BASE module derives from them (golden, taken before the batch). usage: bun rust-golden.mjs <out.json>
import{mkdtempSync,writeFileSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inf=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inf.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");
const cases=[
 ["plain modules","mod a;\nmod b;\npub mod c;\n"],
 ["path attribute with escapes","#[path = \"sub\\\\dir/x.rs\"]\nmod x;\n"],
 ["path attribute plain","#[path = \"sub/dir/y.rs\"]\nmod y;\n"],
 ["include macro plain","const A: &str = include_str!(\"data/a.txt\");\nconst B: &[u8] = include_bytes!(\"data/b.bin\");\n"],
 ["include macro with escape","const A: &str = include_str!(\"data/\\u{61}\\\".txt\");\n"],
 ["unterminated or raw newline in a string","mod z;\nconst S: &str = \"line one\nline two\";\nmod after;\n"],
 ["raw strings","const R: &str = r#\"mod hidden;\"#;\nmod shown;\n"],
 ["byte and c strings","const B: &[u8] = b\"mod nope;\";\nconst C: &CStr = c\"mod nope2;\";\nmod yes;\n"],
 ["characters and lifetimes","fn f<'a>(x: &'a str) -> char { 'm' }\nmod real;\n"],
 ["comments","// mod commented;\n/* mod blocked; */\nmod live;\n"],
 ["nested modules","mod outer {\n    mod inner;\n    #[path = \"deep/q.rs\"] mod q;\n}\n"],
 ["test only modules","#[cfg(test)]\nmod tests;\nmod prod;\n"],
 ["unicode strings and whitespace","const S: &str = \"é東 x\";　mod uni;\n"],
 ["raw identifiers","mod r#type;\nmod r#match;\n"],
];
const dir=mkdtempSync(join(tmpdir(),"rust-golden-")),rust=inf.prepareSourceAnalysesV1().rust,out=[];
for(const[name,source]of cases){const file=join(dir,"x.rs");writeFileSync(file,source);let references;try{references=obs.observedSourceAnalysisV1(rust,file);}catch(error){references={thrown:String(error.message).slice(0,80)};}out.push({name,source,references});}
rmSync(dir,{recursive:true,force:true});
writeFileSync(process.argv[2],JSON.stringify({version:1,cases:out},null,1));
console.log(out.map(c=>c.name+": "+JSON.stringify(c.references).slice(0,100)).join("\n"));
