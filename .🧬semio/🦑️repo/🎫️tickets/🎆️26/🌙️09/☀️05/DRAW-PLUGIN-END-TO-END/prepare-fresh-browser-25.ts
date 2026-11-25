/** 🧩️ Materializes one already linked Draw component and verifies its native and JS descriptors. */
import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { mkdir } from 'node:fs/promises';
import { resolve, join } from 'node:path';
const [componentArg, emitterArg, outputArg] = process.argv.slice(2);
if (!componentArg || !emitterArg || !outputArg) throw Error('Expected linked component, installed emitter and output directory');
const root=process.cwd(), component=resolve(componentArg), emitter=resolve(emitterArg), out=resolve(outputArg);
const ticket=resolve(root,'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END');
const digest=async()=>{const hash=createHash('sha256');for await(const chunk of createReadStream(component))hash.update(chunk);return hash.digest('hex');};
await mkdir(out,{recursive:true});
const before=await digest();
for (const args of [
  ['tools','exec','bun','--',join(ticket,'describe-linked-draw.ts'),component,emitter,join(out,'descriptor')],
  ['tools','exec','bun','--',join(ticket,'probe-linked-draw-browser.ts'),component,join(out,'descriptor'),out],
]) {
  const child=Bun.spawn(['repo',...args],{cwd:root,stdin:'ignore',stdout:'inherit',stderr:'inherit'});
  const code=await child.exited;
  if(code)throw Error(`Materialization stage exited ${code}`);
  if(await digest()!==before)throw Error('Linked component changed during materialization');
}
const browser=join(root,'🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle');
const {finalizePluginDescriptor}=await import(join(browser,'🛂️descriptor/🟦️.ts'));
const {monolithicComponentTable,pluginComponentBridgeSource}=await import(join(browser,'🏗️materialization/🌉️component-bridge-source/🟦️.ts'));
const finalized=finalizePluginDescriptor(await Bun.file(join(out,'semantic.bin')).bytes(),'draw');
await Bun.write(join(out,'module','🔣️.json'),finalized.json);
await Bun.write(join(out,'module','🛂️.descriptor.semio'),finalized.pack);
await Bun.write(join(out,'module','🌉️bridge.js'),pluginComponentBridgeSource('semio_hub_draw_component',monolithicComponentTable(JSON.parse(finalized.json),'semio_hub_draw_component')));
console.log(JSON.stringify({component,sha256:before,descriptor:join(out,'descriptor'),module:join(out,'module')}));
