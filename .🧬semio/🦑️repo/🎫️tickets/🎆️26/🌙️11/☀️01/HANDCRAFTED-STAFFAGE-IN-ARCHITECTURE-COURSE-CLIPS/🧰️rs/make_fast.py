import sys
from pathlib import Path
hops = sys.argv[3]
src = Path(sys.argv[1]).read_text(encoding="utf-8")
m = 'await page.clock.pauseAt(new Date(1000));'
assert m in src
patch = '''await page.evaluate(hops=>{const c=globalThis.__pwClock.controller,channel=new MessageChannel(),queue=[];channel.port1.onmessage=()=>queue.shift()();c._embedder.setTimeout=task=>{let left=hops;const step=()=>{if(--left<=0)task();else{queue.push(step);channel.port2.postMessage(0)}};queue.push(step);channel.port2.postMessage(0);return()=>{}}},''' + hops + ''');'''
Path(sys.argv[2]).write_text(src.replace(m, m + patch, 1), encoding="utf-8")
