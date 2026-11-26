/** 📎️ Seeds the saved device binding through its public mutation and measures real reconnect. */
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {resolve,join} from 'node:path';
import {statSync} from 'node:fs';
const {chromium}=createRequire(resolve('✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs'))('playwright');
const [folderArg,outArg,base='http://127.0.0.1:6064']=process.argv.slice(2);
if(!folderArg||!outArg)throw Error('Expected copied canonical folder and output prefix');
const folder=resolve(folderArg),out=resolve(outArg),event=join(folder,'.semio/events.semio');assert.equal(statSync(event).size,8307);
const browser=await chromium.launch({headless:true,args:['--use-angle=swiftshader','--enable-unsafe-swiftshader','--enable-unsafe-webgpu']});
const page=await (await browser.newContext({locale:'en',viewport:{width:1600,height:1000}})).newPage();
await page.route(url=>decodeURIComponent(url.pathname).includes('/🔌️PluginRuntime/📬️typed-result/🟦️.ts'),async route=>{const response=await route.fetch();let source=await response.text();const anchor='fault(call, token, message) {';if(!source.includes(anchor))throw Error('Missing typed fault instrumentation anchor');source=source.replace(anchor,anchor+` console.log('[TRACE] fault', JSON.stringify({label:call.label,operation:String(token.operation),sequence:token.sequence,receiver:token.receiver,owner:this.owners.get(typedOperationOwnerKey(token))?.label,message}),new Error().stack);`);await route.fulfill({response,body:source});});
await page.route(url=>decodeURIComponent(url.pathname).includes('/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts'),async route=>{const response=await route.fetch();let source=await response.text();for(const name of ['load','bind']){const old=`await ports.${name}();`;if(!source.includes(old))throw Error('Missing instrumentation anchor '+name);source=source.replace(old,`try { console.log('[TRACE] ${name} begin'); await ports.${name}(); console.log('[TRACE] ${name} success'); } catch (error) { console.log('[TRACE] ${name} failed', String(error), error?.stack); throw error; }`);}await route.fulfill({response,body:source});});
const errors=[];page.on('pageerror',e=>errors.push(String(e)));page.on('console',m=>{if(m.text().startsWith('[TRACE]'))console.log(m.text());if(m.type()==='error'&&!m.text().startsWith('[DEBUG] '))errors.push(m.text());});
const click=id=>page.locator(`[id="${id}"]`).first().click({timeout:30000});
const ready=async()=>{await page.waitForFunction(()=>document.documentElement.dataset.semioOsReady||document.documentElement.dataset.semioOsError,undefined,{timeout:120000});assert.equal(await page.evaluate(()=>document.documentElement.dataset.semioOsReady),'draw');};
try{
 await page.goto(`${base}/?plugin=draw&lang=en`);await ready();
 const path=resolve('🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📎️local-folders/🟦️.tsx');
 const result=await page.evaluate(async({path,folder})=>{const mod=await import('/@fs/'+path);const storage={get:k=>localStorage.getItem(k),set:(k,v)=>localStorage.setItem(k,v),remove:k=>localStorage.removeItem(k)};return mod.commitLocalFoldersConfigMutationV1(storage,{mutation:'attachLocalFolder',documentId:'01a21454-66e2-7da6-9e99-b6e3f5316b90',pluginId:'draw',appId:'s.draw.drawing@1/*#editor',folder:{kind:'path',path:folder}});},{path,folder});
 console.log('[DEBUG] seeded',JSON.stringify(result));
 await page.reload({waitUntil:'domcontentloaded',timeout:60000});await ready();await click('s-folder-reconnect');
 const toggle=page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first(),status=page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');if(!await status.isVisible())await toggle.click();
 let observed=false;for(let i=0;i<24;i++){await page.waitForTimeout(10000);const sample=await page.evaluate(()=>({status:document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent,load:document.body.innerText.match(/Document load[^\n]*/)?.[0],error:document.documentElement.dataset.semioOsError}));console.log('[DEBUG] replay',10*(i+1),JSON.stringify(sample),JSON.stringify(errors));if(/^2 layers? ·/.test(sample.status||'')&&!sample.load){observed=true;break;}}
 await page.screenshot({path:out+'.png'});assert(observed,'Bound canonical document did not finish with two layers within 240s');assert.equal(errors.length,0,errors.join('\n'));console.log(JSON.stringify({eventBytes:statSync(event).size,observed,errors}));
}catch(error){await page.screenshot({path:out+'-failure.png'}).catch(()=>{});throw error;}finally{await browser.close();}
