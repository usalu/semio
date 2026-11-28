import sys
exec(open('actions-lib.py').read())
mode=sys.argv[1]
wait()
CANVAS="(async()=>{const c=document.querySelector('canvas');const r=c.getBoundingClientRect();return JSON.stringify([r.x+r.width/2,r.y+r.height/2])})()"
step("loaded","await h.actions();const s=await h.stepsNow(s=>s.n===4,150000,'demo');return h.json(s)",timeoutMs=170000)
if mode=="arm":
    step("arm","h.click('Catalogue');await h.until(()=>h.find('Generic Saw — Cut'),30000,'cat');const row=h.find('Generic Saw — Cut').closest('[data-slot=tree-row-content]');[...row.querySelectorAll('button')].find(b=>b.innerText.trim()==='Place capability').click();await h.wait(3000);return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,300)+h.json(h.steps())")
step("canvas click",  "return 'c'",pointer=CANVAS,settleMs=4000)
step("after","return h.json(h.steps())")
save("process-diag2-%s.json"%mode)
