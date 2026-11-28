import json
S=[]
def step(name,js,**kw):
    d={"name":name,"js":"(async()=>{const h=window.__h;"+js+"})()"};d.update(kw);S.append(d)
step("wait","return 'w'",until="(async()=>String(document.querySelectorAll('canvas').length>=1&&!/Loading document/.test(document.body.innerText)&&!!document.querySelector('[role=combobox]')))()",timeoutMs=420000,settleMs=2000)
for p in ["Artifact","Catalogue","Workshop","Inspection"]:
    step("dump "+p,"h.click('%s');await h.wait(2500);return document.body.innerText.replace(/\\n/g,'|').slice(0,1800)"%p,settleMs=500)
    step("html "+p,"const els=[...document.querySelectorAll('[data-slot]')].map(e=>e.getAttribute('data-slot')).filter((v,i,a)=>a.indexOf(v)===i);return els.join(',')",settleMs=300)
    step("close "+p,"h.click('%s');await h.wait(800);return 'c'"%p,settleMs=300)
step("open Actions","h.click('Actions');await h.wait(1500);return 'a'")
step("set stock","h.click('Set Stock…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("set example","h.click('Set Active Example…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("export model","h.click('Export Model…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("sel mode","h.click('Set Selection Mode…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("granularity","h.click('Set Granularity…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("paste","h.click('Paste…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("switch alt","h.click('Switch Alternative…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)")
step("import doc","h.click('Import Document…');await h.wait(1500);return document.body.innerText.replace(/\\n/g,'|').slice(-500)+JSON.stringify(window.__fileReq)")
step("export doc","h.click('Export Document');await h.wait(2500);return JSON.stringify(window.__dl.map(d=>[d.name,d.size,d.type,d.head]))")
step("load model","h.click('Load Model…');await h.wait(1500);return JSON.stringify(window.__fileReq)")
json.dump(S,open("process-explore.json","w"),ensure_ascii=False,indent=1)
