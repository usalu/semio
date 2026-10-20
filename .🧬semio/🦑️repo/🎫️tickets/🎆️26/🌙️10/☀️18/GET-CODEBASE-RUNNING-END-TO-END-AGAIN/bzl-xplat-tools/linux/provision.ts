import {createHash} from "node:crypto";
import {chmodSync,mkdirSync,readFileSync,writeFileSync,existsSync} from "node:fs";
import {join} from "node:path";
const [schemaPath,destination]=process.argv.slice(2);
const manifest=JSON.parse(readFileSync(schemaPath!,"utf8")).definitions.Release.const;
const row=manifest.distributions.find((row:{platform:string;arch:string})=>row.platform===process.platform&&row.arch===process.arch);
if(!row)throw new Error(`No pinned Bazel for ${process.platform}/${process.arch}`);
const target=join(destination!,row.sha256.slice(0,16),process.platform==="win32"?"bazel.exe":"bazel");
if(existsSync(target)){console.log(target);process.exit(0);}
mkdirSync(join(destination!,row.sha256.slice(0,16)),{recursive:true});
const started=performance.now(),response=await fetch(`${manifest.release}/${row.file}`);
if(!response.ok)throw new Error(`download failed ${response.status}`);
const bytes=new Uint8Array(await response.arrayBuffer()),digest=createHash("sha256").update(bytes).digest("hex");
if(bytes.length!==row.bytes||digest!==row.sha256)throw new Error(`pin mismatch bytes=${bytes.length} sha=${digest}`);
writeFileSync(target,bytes);chmodSync(target,0o755);
console.error(`downloaded ${row.file} ${bytes.length} bytes sha256 ok in ${Math.round(performance.now()-started)} ms`);
console.log(target);
