import json
S=[]
def step(name,js,**kw):
    d={"name":name,"js":"(async()=>{const h=window.__h;"+js+"})()"};d.update(kw);S.append(d)
DUMP="const chrome=new Set(['Artifact','Catalogue','Workshop','Inspection','Chat','Fullscreen','Editor|⌘️⌥️E','Viewer|⌘️⌥️V','Display','Remote: detached','Command','Settings','Marketplace','History','Tasks','Workpiece','Projection','Window Options','Search','Utilities','⋯','Close','Example']);const el=[...document.querySelectorAll('button,input,select,textarea,[role=switch],[role=checkbox],[role=combobox],[role=slider],[role=option],[role=menuitem]')].map(e=>(e.tagName[0]+(e.getAttribute('role')?'/'+e.getAttribute('role'):'')+(e.type&&e.tagName=='INPUT'?'/'+e.type:'')+':'+(e.getAttribute('aria-label')||'')+'~'+(e.value!==undefined&&e.tagName!='BUTTON'?'='+e.value+'~':'')+e.innerText.trim().replace(/\\n/g,'/'))).filter(s=>!chrome.has(s.split(':')[1]?.split('~').pop()));"
step("wait","return 'w'",until="(async()=>String(document.querySelectorAll('canvas').length>=1&&!/Loading document/.test(document.body.innerText)&&!!document.querySelector('[role=combobox]')))()",timeoutMs=420000,settleMs=2000)
for p in ["Artifact","Catalogue","Workshop","Inspection"]:
    step("dump "+p,"h.click('%s');await h.wait(2000);"%p+DUMP+"return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,2500)+'\\n@@'+el.join(' ; ')",settleMs=300)
    step("close "+p,"h.click('%s');await h.wait(500);return 'c'"%p,settleMs=300)
step("open Actions","h.click('Actions');await h.wait(1500);return 'a'",settleMs=500)
for a in ["Add Step…","Set Stock…","Set Active Example…","Export Model…","Set Selection Mode…","Set Granularity…","Paste…","Switch Alternative…","Import Document…"]:
    step("form "+a,"h.click('%s');await h.wait(1000);"%a+DUMP+"return el.join(' ; ')",settleMs=300)
step("select all","h.click('Select All');await h.wait(1500);return h.steps()&&JSON.stringify(h.steps())+document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,800)",settleMs=300)
json.dump(S,open("process-explore2.json","w"),ensure_ascii=False,indent=1)
