/** 📂️ Reopens an existing saved document and checks it survives a real browser reload. */
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {resolve,join} from 'node:path';
import {statSync,mkdirSync} from 'node:fs';
const {chromium}=createRequire(resolve('✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs'))('playwright');
const [folderArg,outArg,base='http://127.0.0.1:6064']=process.argv.slice(2);
if(!folderArg||!outArg)throw Error('Expected existing folder and output prefix');
const folder=resolve(folderArg),out=resolve(outArg),event=join(folder,'.semio/events.semio');
mkdirSync(folder,{recursive:true});let before=0;
const browser=await chromium.launch({headless:true,args:['--use-angle=swiftshader','--enable-unsafe-swiftshader','--enable-unsafe-webgpu']});
const page=await (await browser.newContext({locale:'en',viewport:{width:1600,height:1000}})).newPage();
const errors=[];page.on('pageerror',e=>errors.push(String(e)));page.on('console',m=>{if(m.type()==='error'&&!m.text().startsWith('[DEBUG] '))errors.push(m.text());});
const click=id=>page.locator(`[id="${id}"]`).first().click({timeout:30000});
const ready=async()=>{await page.waitForFunction(()=>document.documentElement.dataset.semioOsReady||document.documentElement.dataset.semioOsError,undefined,{timeout:120000});assert.equal(await page.evaluate(()=>document.documentElement.dataset.semioOsReady),'draw');};
const layers=async()=>{const toggle=page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first(),status=page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');if(!await status.isVisible())await toggle.click();await status.waitFor({state:'visible'});return (await status.innerText()).trim();};
try {
 await page.goto(`${base}/?plugin=draw&lang=en`);await ready();
 await click('s-sync-status');if(!await page.locator('[id="framework.sync.folder"]').isVisible())await click('ui.utilities.group.sync');await click('framework.sync.folder');
 await page.locator('[id="framework.sync.folder.path"]').fill(folder);await page.getByRole('button',{name:/^attach$/i}).click();
 await page.locator('[id="framework.sync.folder.path"]').waitFor({state:'hidden',timeout:30000});
 await page.keyboard.press('Escape');await page.waitForTimeout(5000);before=statSync(event).size;console.log('[DEBUG] attached bytes',before);await page.locator('[id^="framework.window."][id$=".utilityBar.unfold"]').first().click({timeout:30000});await page.locator('[id^="ui.utilities."][id$=".group.group:Drawing"]').first().click({timeout:30000});for(const pressed of [true,false,true]){await click('shapeRect');await page.waitForFunction(({pressed})=>document.getElementById('shapeRect')?.getAttribute('aria-pressed')===String(pressed),{pressed},{timeout:15000});}const box=await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first().boundingBox();assert(box);await page.mouse.move(box.x+box.width*.275,box.y+box.height*.1);await page.mouse.down();await page.mouse.move(box.x+box.width*.325,box.y+box.height*.2,{steps:8});await page.mouse.up();await layers();await page.waitForFunction(()=>/^2 layers? ·/.test(document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent||''),undefined,{timeout:60000});const attached=await layers();console.log('[DEBUG] drawn',attached);await click('s-sync-status');if(!await page.locator('[id="framework.sync.folder"]').isVisible())await click('ui.utilities.group.sync');await click('framework.sync.folder');await page.waitForFunction(()=>[...document.querySelectorAll('[data-semio-sync-status]')].some(e=>/(?:^|·)\s*saved(?:\s*·|$)/i.test(e.textContent||'')),undefined,{timeout:60000});assert(statSync(event).size>before);await page.keyboard.press('Escape');
 await page.screenshot({path:out+'-attached.png'});
 await page.reload({waitUntil:'domcontentloaded',timeout:60000});await ready();await click('s-folder-reconnect');await page.waitForTimeout(10000);await layers();for(let i=0;i<24;i++){await page.waitForTimeout(10000);const sample=await page.evaluate(()=>({status:document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent,load:document.body.innerText.match(/Document load[^\n]*/)?.[0],error:document.documentElement.dataset.semioOsError}));console.log('[DEBUG] replay',10*(i+1),JSON.stringify(sample),JSON.stringify(errors));if(/^2 layers? ·/.test(sample.status||'')&&!sample.load)break;}const reloaded=await layers();
 await page.screenshot({path:out+'-reloaded.png'});assert.match(reloaded,/^2 layers? ·/);assert.equal(errors.length,0,errors.join('\n'));
 console.log(JSON.stringify({folder,before,eventBytesAfter:statSync(event).size,attached,reloaded,errors}));
}catch(error){await page.screenshot({path:out+'-failure.png'});console.log('[DEBUG] errors',JSON.stringify(errors));throw error;}finally{await browser.close();}
