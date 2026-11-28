exec(open('actions-lib.py').read())
wait()
CANVAS="(async()=>{const c=document.querySelector('canvas');const r=c.getBoundingClientRect();return JSON.stringify([r.x+r.width/2,r.y+r.height/2])})()"
step("PL0 demo loaded","await h.actions();const s=await h.stepsNow(s=>s.n===4,150000,'demo steps');window.__p0=s;return h.json(s)",timeoutMs=170000)
step("PL1 Catalogue Place capability arms the Generic Saw","h.click('Catalogue');await h.until(()=>h.find('Generic Saw — Cut'),30000,'catalogue rows');const row=h.find('Generic Saw — Cut').closest('[data-slot=tree-row-content]');const btn=[...row.querySelectorAll('button')].find(b=>b.innerText.trim()==='Place capability');h.ok(btn,'no place button');btn.click();await h.wait(2000);return h.json(h.steps())")
step("PL2 click the viewport to commit the armed placement","return 'c'",pointer=CANVAS,settleMs=4000)
step("PL3 placement added exactly one step","const s=await h.stepsNow(s=>s.n===window.__p0.n+1,60000,'placed step');return h.json(s)")
save("process-actions-place.json")
