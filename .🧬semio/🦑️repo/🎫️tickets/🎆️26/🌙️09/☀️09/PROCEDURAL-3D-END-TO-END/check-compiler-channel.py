"""🧵 Exercise nested Bun Cargo stream relays against a slow consumer and a Python byte oracle."""
import subprocess
import sys
import threading
import time
from pathlib import Path
bun = str(next(Path(".cache/b/t/m/bun").glob("*/bun")).resolve())
program = r'''
import {spawn} from 'node:child_process';
import {createInterface} from 'node:readline';
const producer = process.argv[1]==='produce';
const output = producer ? [process.argv[0],['-e',"const fs=require('node:fs');const row=JSON.stringify({reason:'compiler-artifact',value:'z'.repeat(996)})+'\\n';for(let i=0;i<4096;i++){fs.writeSync(1,row);fs.writeSync(2,'e'.repeat(1000)+'\\n')}"]] : [process.argv[0],['-e',process.argv[2],'produce']];
const child=spawn(output[0],output[1],{stdio:['ignore','pipe','pipe']});
const observation=producer?(async()=>{for await(const line of createInterface({input:child.stdout,crlfDelay:Infinity})){JSON.parse(line)}})():Promise.resolve();
child.stdout.pipe(process.stdout,{end:false});child.stderr.pipe(process.stderr,{end:false});
const status=await new Promise(resolve=>child.on('close',resolve));await observation;process.exitCode=status;
'''
child = subprocess.Popen([bun,'-e',program,'outer',program], stdout=subprocess.PIPE,stderr=subprocess.PIPE)
counts = [0,0]
def read(i, f):
    while True:
        chunk = f.read(16384)
        if not chunk: break
        counts[i] += len(chunk)
        time.sleep(.002)
workers=[threading.Thread(target=read,args=(i,f),daemon=True) for i,f in enumerate([child.stdout,child.stderr])]
for worker in workers: worker.start()
try: code=child.wait(timeout=45)
except subprocess.TimeoutExpired:
    child.kill()
    raise
for worker in workers: worker.join(6)
print(code,counts, flush=True)
assert code==0 and min(counts)>4_000_000
