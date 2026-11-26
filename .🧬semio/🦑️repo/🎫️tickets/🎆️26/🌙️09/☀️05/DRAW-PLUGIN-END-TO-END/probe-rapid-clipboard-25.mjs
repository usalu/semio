/** 📋️ Repeats immediate copy/paste followed by cut in one Draw session. */
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {resolve} from 'node:path';
const {chromium}=createRequire(resolve('✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs'))('playwright');
const browser=await chromium.launch({headless:true,args:['--use-angle=swiftshader','--enable-unsafe-swiftshader','--enable-unsafe-webgpu']});
const page=await (await browser.newContext({locale:'en',viewport:{width:1600,height:1000}})).newPage();
const errors=[];page.on('pageerror',e=>errors.push(String(e)));page.on('console',m=>{if(m.type()==='error')errors.push(m.text());});
const mod=process.platform==='darwin'?'Meta':'Control',status=page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');
const wait=async n=>page.waitForFunction(n=>{const s=document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent||'';return new RegExp(`^${n} layers? ·`).test(s);},n,{timeout:60000});
try{
 await page.goto('http://127.0.0.1:6064/?plugin=draw&lang=en');await page.waitForFunction(()=>document.documentElement.dataset.semioOsReady==='draw',undefined,{timeout:120000});
 await page.getByText('Artifact',{exact:true}).first().click();await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').waitFor();
 if(!await status.isVisible())await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click();await wait(1);
 const canvas=page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
 for(let round=1;round<=4;round++){
  await page.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();await page.waitForFunction(()=>document.querySelector('[id="drawing-composite"] [data-slot="engagement-status"]')?.textContent?.includes('1 selected'),undefined,{timeout:30000});
  await canvas.focus();await page.keyboard.press(`${mod}+c`);await page.keyboard.press(`${mod}+v`);await wait(2);
  await canvas.focus();await page.keyboard.press(`${mod}+x`);await wait(1);console.log('[DEBUG] rapid round',round,'passed');
 }
 assert.equal(errors.length,0,errors.join('\n'));await page.screenshot({path:resolve(process.argv[2])});
}finally{await browser.close();}
