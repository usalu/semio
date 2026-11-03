import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
inject = '''  await page.evaluate(()=>{const f=document.createElement("iframe");f.style.display="none";document.body.appendChild(f);const real=()=>f.contentWindow.performance.now();window.__diag={n:{raf:0,timeout:0,interval:0},t:{raf:0,timeout:0,interval:0},names:{},real};
    const wrap=(name,kind)=>{const orig=window[name];window[name]=function(cb,...rest){const w=(...a)=>{const s=real();try{return cb(...a)}finally{window.__diag.n[kind]++;const k=cb.toString().slice(0,90);window.__diag.names[k]=(window.__diag.names[k]||0)+1;window.__diag.t[kind]+=real()-s}};return orig.call(window,w,...rest)}};
    wrap("requestAnimationFrame","raf");wrap("setTimeout","timeout");wrap("setInterval","interval")});
'''
marker = '  const outputs=[],metadata=[];'
assert marker in src
src = src.replace(marker, inject + marker, 1)
marker2 = 'await page.clock.runFor(frame.transition_elapsed??1000);'
assert marker2 in src
src = src.replace(marker2, '    const __s=Date.now();' + marker2 + 'console.error("[DIAG] f"+index+" wall="+(Date.now()-__s)+" "+JSON.stringify(await page.evaluate(()=>{const d=window.__diag;const r=JSON.stringify({n:d.n,names:d.names});d.names={};d.n={raf:0,timeout:0,interval:0};d.t={raf:0,timeout:0,interval:0};return JSON.parse(r)})));', 1)
Path(sys.argv[2]).write_text(src, encoding="utf-8")
