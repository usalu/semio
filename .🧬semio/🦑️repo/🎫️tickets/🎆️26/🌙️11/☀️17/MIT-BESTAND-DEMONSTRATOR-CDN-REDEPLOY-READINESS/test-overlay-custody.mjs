import {spawn} from 'node:child_process';
import {mkdtempSync,mkdirSync,writeFileSync,rmSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {join} from 'node:path';
import assert from 'node:assert/strict';
const root=mkdtempSync(fileURLToPath(new URL('./🗑️generated/overlay-custody-',import.meta.url))),site=join(root,'site'),modules=join(root,'modules'),probe=join(root,'probe'),port=19784;
for(const p of [site,join(modules,'guest'),probe])mkdirSync(p,{recursive:true});
writeFileSync(join(site,'index.html'),'site');writeFileSync(join(modules,'guest','x.core.wasm'),'old');writeFileSync(join(modules,'guest','metadata.json'),'meta');writeFileSync(join(probe,'x.js'),'new');
const child=spawn(process.execPath,[fileURLToPath(new URL('./overlay-serve.mjs',import.meta.url)),site,modules,String(port),'guest',probe]);
try {let ready=false;for(let i=0;i<50;i++){try{await fetch(`http://127.0.0.1:${port}/`);ready=true;break}catch{await new Promise(r=>setTimeout(r,50))}}assert(ready);const get=n=>fetch(`http://127.0.0.1:${port}/🔌️plugin-modules/guest/${n}`);assert.equal(await (await get('x.js')).text(),'new');assert.equal((await get('x.core.wasm')).status,404);assert.equal((await get('🌉️bridge.js')).status,404);assert.equal(await (await get('metadata.json')).text(),'meta');console.log('overlay custody passed')}finally{child.kill();rmSync(root,{recursive:true,force:true})}
