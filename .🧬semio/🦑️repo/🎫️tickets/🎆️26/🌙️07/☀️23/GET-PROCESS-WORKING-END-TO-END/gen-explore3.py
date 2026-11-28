import json
exec(open('gen-explore2.py').read().split('step("wait"')[0])
step("wait","return 'w'",until="(async()=>String(document.querySelectorAll('canvas').length>=1&&!/Loading document/.test(document.body.innerText)&&!!document.querySelector('[role=combobox]')))()",timeoutMs=420000,settleMs=2000)
step("workshop groups","h.click('Workshop');await h.wait(1500);const out=[];for(const g of ['METAL','CONCRETE','ROBOTIC']){const b=h.find(g);out.push(g+':'+(b?b.outerHTML.slice(0,300):'none'));b&&b.click();await h.wait(800)}return out.join('\\n')+'\\n'+document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,2000)",settleMs=300)
step("workshop machine html","const b=h.find('Table Saw');return b.outerHTML.slice(0,800)+'\\nPARENT '+b.parentElement.outerHTML.slice(0,1200)",settleMs=300)
step("catalogue after","h.click('Catalogue');await h.wait(1500);return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,2500)",settleMs=300)
step("artifact row html","h.click('Artifact');await h.wait(1500);const b=h.find('Cut Lap Joint');return b.outerHTML.slice(0,600)+'\\nPARENT '+b.parentElement.outerHTML.replace(/style=\"[^\"]*\"/g,'').slice(0,2500)",settleMs=300)
step("select row + inspection","h.click('Cut Lap Joint');await h.wait(1500);h.click('Inspection');await h.wait(2000);"+DUMP+"return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,2500)+'\\n@@'+el.join(' ; ')",settleMs=300)
json.dump(S,open("process-explore3.json","w"),ensure_ascii=False,indent=1)
