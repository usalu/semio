"""⏱️ Locate response intake, dispatch and retirement in the actual browser worker sample timeline."""
import collections
from pathlib import Path
import re
import sys
t=Path(__file__).resolve().parent
p=t/"profile-requester-symbols.py"
exec(p.read_text().split("own=collections.Counter()")[0])
groups={
"retainCompleted":r"retain_completed",
"responseStep":r"RetainedResponse>::step",
"responseBody":r"CompletedInput>::(step|pack|close)",
"responseDispatch":r"plugin_dispatch_response_action|OriginalResponseActionSource>::",
"flowReply":r"FlowEvaluationReplyReceiver|PreviewEvalReceive",
"tick":r"Generation3dPreviewWindowCommandWork|PreviewEvalTick|Generation3dPreviewAndFlow",
"ingress":r"exchange_generic",
"catalogue":r"FlowOperatorCatalogue",
}
groups={k:re.compile(v) for k,v in groups.items()}
group_ids={k:{i for i in nodes if pat.search(label(i))} for k,pat in groups.items()}
for k,v in group_ids.items():print(k,len(v))
hits={}
def hit(i):
 if i not in hits:hits[i]={k for k,v in group_ids.items() if i in v}|(hit(parent[i]) if i in parent else set())
 return hits[i]
hist=collections.defaultdict(collections.Counter)
total=0
for i,dt in zip(d['samples'],d['timeDeltas']):
 total+=dt
 for k in hit(i):hist[total//10000000][k]+=dt/1000
for slot,counts in sorted(hist.items()):print(slot*10,{k:round(v) for k,v in counts.items()})

paths=collections.Counter()
for i,dt in zip(d['samples'],d['timeDeltas']):
 if "responseStep" in hit(i):
    stack=[]
    while i is not None:
        name=label(i);stack.append(name);i=parent.get(i)
        if groups["responseStep"].search(name):break
    paths[tuple(stack)]+=dt
for frames,dt in paths.most_common(25):
 print(round(dt/1000,1), " <- ".join(x[:190] for x in frames[:10]))
