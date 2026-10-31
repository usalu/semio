/** 🔎 Read exact process facts through the independent existing portable doctor port. */
import {existsSync,writeFileSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const ticket=dirname(fileURLToPath(import.meta.url));let root=ticket;
while(!existsSync(join(root,'🧰️framework'))){const parent=dirname(root);if(parent===root)throw Error('Root missing');root=parent;}
const mod=await import(pathToFileURL(join(root,'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts')).href);
const rows=await mod.doctorPortsV1(root,{signal:AbortSignal.timeout(15000)}).processes();
writeFileSync(process.argv[2]!,JSON.stringify({at:new Date().toISOString(),rows},null,2));
console.log('[DEBUG] independent process snapshot '+rows.length);
