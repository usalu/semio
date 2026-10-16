"""🧭 Per-method field/method touch analysis of `impl FlowHost` (analysis only)."""
import os,re,sys,collections
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
H="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/"
FIELDS="""geometry_port operator_registry host_snapshot dag current_channels export_payloads last_eval_json host_catalogue_json kind_infos neural_cache previous_snapshot previous_channels pending_baseline_publication baseline_retirement baseline_publication_progress pending_evaluation evaluation_revision evaluation_progress baseline_registry_generation next_widget_serial next_synapse_serial viewport_w viewport_h viewport_dpr pan_anchor ghost_node history_store history_store_factory pending_history_baseline pending_change pending_leaves history_fault recorded journal_mark edited_note gesture_active pending_extension_evals pending_remaining interaction_revision interaction_projection displaced""".split()
files=["✏️host-authoring/🦀️.rs","🌳️host-evaluation-history/🦀️.rs"]
methods={}
order=[]
for f in files:
    t=open(H+f,encoding="utf-8").read();L=[l+"\n" for l in t.split("\n")][:-1];I=lex2(L)
    cur=None
    for i,info in enumerate(I):
        m=re.match(r'^    (pub(?:\([^)]*\))?\s+)?(?:(?:unsafe|async|const)\s+)*fn\s+(\w+)',L[i])
        if m and info['depth']==1 and info['state']=='code':
            cur=m.group(2);methods[cur]=dict(file=f,line=i+1,pub=bool(m.group(1)),toks=[],lines=0,sig=L[i]);order.append(cur)
        if cur and info['depth']>=1:
            methods[cur]['toks'].append((L[i],info))
            methods[cur]['lines']+=1
names=set(methods)
for n,m in methods.items():
    fields=collections.Counter();calls=collections.Counter()
    for line,info in m['toks']:
        for tok,col in info['toks']:
            pre=line[:col].rstrip()
            if pre.endswith('self.') or pre.endswith('self .'):
                if tok in FIELDS:fields[tok]+=1
                elif tok in names:calls[tok]+=1
            if pre.endswith('Self::') and tok in names:calls[tok]+=1
            if tok in FIELDS and (pre.endswith('host.') or pre.endswith('previous.') or pre.endswith('other.')):fields[tok+'*']+=1
    m['fields']=fields;m['calls']=calls
if __name__=="__main__":
    tot=0
    for n in order:
        m=methods[n]
        print(f"{n:42s} {m['lines']:4d} {'pub' if m['pub'] else '   '} F={','.join(sorted(m['fields']))} C={','.join(sorted(m['calls']))}"[:300])
        tot+=m['lines']
    print(len(order),"methods",tot,"lines")
