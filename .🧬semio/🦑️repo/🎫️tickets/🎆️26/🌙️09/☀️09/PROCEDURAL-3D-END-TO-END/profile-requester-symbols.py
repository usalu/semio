"""🔬 Resolve sampled WASM frames to the original linked Rust owners and report inclusive stacks."""
from pathlib import Path
import collections
import json
import mmap
import re
import runpy
import subprocess
import sys
t=Path(__file__).resolve().parent
g=t/"🗑️generated/bridge-current"
core=runpy.run_path(str(t/"requester-function-names.py"))
d=json.loads(Path(sys.argv[1]).read_text())
d=d.get('profile',d)
nodes={n['id']:n for n in d['nodes']}
parent={child:n['id'] for n in nodes.values() for child in n.get('children',[])}
wasm={}
for i,n in nodes.items():
    f=n['callFrame']
    m=re.fullmatch(r'wasm-function\[(\d+)\]',f['functionName'])
    if m:wasm[i]=int(m[1])
hot=sorted(set(wasm.values()))
old=core['original_index'](hot)
with (g/"requester-final-objects/procedural.core.wasm").open('rb') as f,mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ) as data:
    names=dict(core['names'](data,set(old)))
symbols=''.join(f'{n}\t{k}\t{names.get(k,"unnamed")}\n' for n,k in zip(hot,old))
result=subprocess.run([g/'demangle-requester'],input=symbols,text=True,capture_output=True,check=True)
table={int(l.split('\t',1)[0]):l.split('\t',2)[2] for l in result.stdout.splitlines()}
def label(i):return table[wasm[i]] if i in wasm else nodes[i]['callFrame']['functionName']
own=collections.Counter();inclusive=collections.Counter();stack_counts=collections.Counter()
for hit,us in zip(d['samples'],d['timeDeltas']):
    own[label(hit)]+=us
    stack=[];i=hit
    while i is not None:
        stack.append(label(i));i=parent.get(i)
    for n in set(stack):inclusive[n]+=us
    stack_counts[tuple(reversed(stack))]+=us
for title,counts in [('inclusive',inclusive),('own',own)]:
    print(title)
    for n,total in counts.most_common(100):
        print(f'{total/1000:.1f}\t{n}')
print("stacks")
for stack,total in stack_counts.most_common(32):
    print(f'{total/1000:.1f}', ' -> '.join(stack))
