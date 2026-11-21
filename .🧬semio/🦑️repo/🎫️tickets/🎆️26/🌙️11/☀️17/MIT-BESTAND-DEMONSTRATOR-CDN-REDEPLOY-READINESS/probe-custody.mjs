import {copyFileSync,createReadStream,chmodSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {Readable} from 'node:stream';
const digest=async stream=>{const h=createHash('sha256');for await(const chunk of stream)h.update(chunk);return h.digest('hex')};
const [action,a,b]=process.argv.slice(2);
if(action==='copy'){const before=await digest(createReadStream(a));copyFileSync(a,b);const after=await digest(createReadStream(a)),copy=await digest(createReadStream(b));if(before!==after||copy!==before)throw Error('probe source changed during copy');chmodSync(b,0o444);console.log(`${copy} ${b}`)}
else if(action==='verify'){const expected=await digest(createReadStream(a));const response=await fetch(b);if(!response.ok||!response.body)throw Error(`probe response ${response.status}`);const actual=await digest(Readable.fromWeb(response.body));if(expected!==actual)throw Error('probe response digest differs');console.log(`${actual} ${b}`)}
else throw Error('expected copy or verify');
