import json
exec(open('gen-explore2.py').read().split('step("wait"')[0])
CLEAN="const clean=(e)=>e.outerHTML.replace(/style=\\\"[^\\\"]*\\\"/g,'').replace(/<svg.*?<\\/svg>/g,'<svg/>').replace(/class=\\\"[^\\\"]*\\\"/g,'').replace(/\\s+/g,' ');"
step("wait","return 'w'",until="(async()=>String(document.querySelectorAll('canvas').length>=1&&!/Loading document/.test(document.body.innerText)&&!!document.querySelector('[role=combobox]')))()",timeoutMs=420000,settleMs=2000)
step("workshop rows","h.click('Workshop');await h.wait(1500);"+CLEAN+"const row=(l)=>h.find(l).closest('[data-slot*=row]');return clean(row('Table Saw')).slice(0,1500)+'\\n---\\n'+(h.find('Waterjet')?clean(row('Waterjet')).slice(0,1500):'no waterjet')",settleMs=300)
step("click waterjet","const b=h.find('Waterjet');if(b){b.click()}else{h.click('ROBOTIC');await h.wait(800);h.click('Waterjet')}await h.wait(2000);return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,1500)",settleMs=300)
step("catalogue","h.click('Catalogue');await h.wait(2000);return document.body.innerText.split('Tasks').pop().replace(/\\n/g,'|').slice(0,2500)",settleMs=300)
step("artifact rows","h.click('Artifact');await h.wait(1500);"+CLEAN+"const row=h.find('Cut Lap Joint').closest('[data-slot*=row]');return row.getAttribute('data-slot')+clean(row).slice(0,2500)+'\\nBTNS '+[...row.querySelectorAll('button')].map(b=>b.getAttribute('aria-label')+'/'+b.innerText).join(';')",settleMs=300)
json.dump(S,open("process-explore4.json","w"),ensure_ascii=False,indent=1)
