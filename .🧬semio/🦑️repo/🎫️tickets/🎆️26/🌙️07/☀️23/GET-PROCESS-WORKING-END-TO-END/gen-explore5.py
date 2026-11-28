import json
exec(open('gen-explore2.py').read().split('step("wait"')[0])
CLEAN="const clean=(e)=>e.outerHTML.replace(/style=\\\"[^\\\"]*\\\"/g,'').replace(/<svg.*?<\\/svg>/g,'<svg/>').replace(/class=\\\"[^\\\"]*\\\"/g,'').replace(/\\s+/g,' ');"
TXT="document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,1800)"
step("wait","return 'w'",until="(async()=>String(document.querySelectorAll('canvas').length>=1&&!/Loading document/.test(document.body.innerText)&&!!document.querySelector('[role=combobox]')))()",timeoutMs=420000,settleMs=2000)
step("open artifact","h.click('Artifact');await h.until(()=>h.find('Cut Lap Joint'),20000,'rows');return document.body.innerText.slice(0,300)",settleMs=300)
step("hover row kebab",
 "return 'x'",
 pointer="(async()=>{const b=window.__h.find('Cut Lap Joint');const r=b.getBoundingClientRect();return JSON.stringify([r.x+r.width/2,r.y+r.height/2])})()",settleMs=1000)
step("row html after hover",CLEAN+"const row=h.find('Cut Lap Joint').closest('[data-slot=tree-row-content]');const out=[...row.querySelectorAll('[data-slot=tree-header-actions] *')].map(e=>e.tagName+':'+(e.getAttribute('data-slot')||'')+':'+(e.getAttribute('aria-label')||'')+':'+Math.round(e.getBoundingClientRect().x)+','+Math.round(e.getBoundingClientRect().width));return out.join(' ; ')+'\\nCTX '+document.querySelectorAll('[role=menu]').length",settleMs=300)
step("contextmenu",
 "const b=h.find('Cut Lap Joint');b.dispatchEvent(new MouseEvent('contextmenu',{bubbles:true,cancelable:true,clientX:b.getBoundingClientRect().x+10,clientY:b.getBoundingClientRect().y+5,button:2}));await h.wait(1200);return [...document.querySelectorAll('[role=menu],[role=menuitem]')].map(e=>e.getAttribute('role')+':'+e.innerText.replace(/\\n/g,'/')).join(' ; ')+'||'+"+TXT,settleMs=300)
json.dump(S,open("process-explore5.json","w"),ensure_ascii=False,indent=1)
