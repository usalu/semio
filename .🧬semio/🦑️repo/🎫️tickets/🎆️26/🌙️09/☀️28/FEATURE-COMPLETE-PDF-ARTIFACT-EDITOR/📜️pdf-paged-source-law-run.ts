/** 🔬 Links the exact staged natural PDF source owner and settlement against the real native Value library. */
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const ticket=import.meta.dir;
const output=join(ticket,"🗑️generated","paged-decoder-source");
await mkdir(output,{recursive:true});
const cache=resolve(process.argv[2]??"");
assert(cache.endsWith("/prototype/build")&&!cache.includes("/wasm32-"),"Pass a native repository dependency cohort.");
const edits: [string,string,string][]=await Bun.file(join(ticket,"🗑️generated","pdf-paged-decoder-overlay.json")).json();
const [[path,before,source]]=edits;
assert.equal(new Bun.CryptoHasher("sha256").update(await Bun.file(path).bytes()).digest("hex"),before);
const start=source.indexOf("enum PdfNaturalSource {"), stop=source.indexOf("\nstruct NaturalPdfDecoder",start);
assert(start>=0&&stop>start);
const valueOwner="🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs";
const settled=await Bun.file(valueOwner).text();
const closeStart=settled.indexOf("pub fn failure_value_error("),closeStop=settled.indexOf("\n/// ⏳️",closeStart);
assert(closeStart>=0&&closeStop>closeStart);
const harness=await Bun.file(join(ticket,"📜️pdf-paged-source-law.rs")).text();
const entry=join(output,"lib.rs");
await writeFile(entry,[source.slice(start,stop),settled.slice(closeStart,closeStop),harness].join("\n"));
const libs = await Array.fromAsync(new Bun.Glob("semio-framework-value/*/out/libsemio_framework_value-*.rlib").scan({cwd:cache,onlyFiles:true}));
assert.equal(libs.length,1);
const target=join(output,process.platform==="win32"?"law.exe":"law");
const args=["--edition","2024","--crate-name","pdf_natural_paged_source_law",entry,"-o",target,
 "--extern","semio_framework_value="+join(cache,libs[0]!.replace(/\.rlib$/u,".rmeta")),
 "--extern","semio_framework_value="+join(cache,libs[0]!)];
for await(const dir of new Bun.Glob("*/*/out").scan({cwd:cache,onlyFiles:false}))args.push("-Ldependency="+join(cache,dir));
const response=join(output,"law.args");
await writeFile(response,args.join("\n")+"\n");
const compile=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+response],
 {stdout:"inherit",stderr:"inherit"});
assert.equal(await compile.exited,0);
const check=Bun.spawn([target,join(ticket,"🗑️generated","pdf-segmented-source.pdf")],{stdout:"inherit",stderr:"inherit"});
assert.equal(await check.exited,0);
