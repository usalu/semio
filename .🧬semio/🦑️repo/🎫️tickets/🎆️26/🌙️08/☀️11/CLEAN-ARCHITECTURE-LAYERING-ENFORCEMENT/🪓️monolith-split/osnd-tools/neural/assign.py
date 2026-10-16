"""🧭 Item-level crate assignment of the neural engine (M model, R registry, B budgeted, E evaluator facade) and upward-reference check."""
import sys,os,re,collections,json
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from items import parse
SNAP="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/snap/neural/"
ORDER=["M","R","B","E"]
FILES={"root":"🦀️.rs","retirement":"🧵️retirement/🦀️.rs","topology":"🧭️topology/🦀️.rs","input":"📥️input/🦀️.rs","evaluation":"⏱️evaluation/🦀️.rs","wave":"⏱️evaluation/🌊️wave/🦀️.rs","hash":"⏱️evaluation/🌊️wave/🔑️hash/🦀️.rs","cache_insert":"🌱️cache/🦀️.rs","tree_copy":"🌳️tree/📋️copy/🦀️.rs","cold":"🧊️cold/🦀️.rs","registry":"📔️registry/🦀️.rs","output":"📔️registry/✅️output/🦀️.rs","admission":"📔️registry/🏗️admission/🦀️.rs","lookup":"📖️lookup/🦀️.rs","projection":"📤️projection/🦀️.rs","native":"🚦️native/🦀️.rs","failure":"⚠️failure/🦀️.rs"}
FILE_CRATE={"topology":"B","input":"B","evaluation":"B","wave":"B","hash":"B","cache_insert":"B","tree_copy":"B","registry":"R","output":"R","admission":"R","lookup":"M","projection":"M","native":"M","failure":"M"}
ROOT_REGION={"🔖️Dictionary":"M","🔖️Schema":"M","⚠️ Errors":"M","🔖️SchemaComponent":"R","🔖️Tree":"M","🔖️Contract":"M","🔖️OperatorRecord":"M","🔖️Cache":"M","🔖️DirtyPropagation":"M","🔖️Evaluator":"E","🔖️Tests":"E","":"M"}
ROOT_NAME={ # overrides by item name
 "Registry":"R","OperatorImpl":"R","OperatorRecord":"R",
 "NeuralCache":"B","NeuralCacheState":"B","NeuralCacheRetirement":"B","eval_error_dictionary":"B","evaluate_cached_output":"B",
 "budgeted_remaining":"B","waits_on_parked":"B","EvalChannels":"B","BudgetedEval":"B","EvalStepDeadline":"B","EvalStepBudget":"B","PendingExtensionEval":"B",
 "list_item_count":"M","validate_homogeneous_list":"M","validate_channel_value":"M","validate_neuron_inputs":"M","validate_operator_outputs":"M","channel_schema":"M","operator_signature":"M",
}
def load(name):
    L,items=parse(SNAP+FILES[name])
    return L,items
def impl_target(head):
    m=re.match(r'(?:unsafe\s+)?impl(?:<[^>]*(?:<[^>]*>[^>]*)*>)?\s+(?:[\w:<>\', &]+?\s+for\s+)?([\w:]+)',head)
    return m.group(1).split('::')[-1] if m else ''
def root_assign(it,L):
    nm=it['name']
    if it['kind']=='impl':
        head=L[it['head']].strip()
        tgt=impl_target(head)
        if tgt in ROOT_NAME:return ROOT_NAME[tgt]
        # `impl X for NeuralCache` etc handled by tgt
    if nm in ROOT_NAME:return ROOT_NAME[nm]
    return ROOT_REGION.get(it['region'].split('/')[0],"M")
if __name__=="__main__":
    L,items=load("root")
    rows=[]
    for it in items:
        c=root_assign(it,L)
        rows.append((it['start']+1,it['kind'],it['name'][:50],c,it['bytes']))
    tot=collections.Counter()
    for r in rows:tot[r[3]]+=r[4]
    print(tot)
    for r in rows:
        if r[1] not in('use',):print(r)

# ---- item-level assignment of every file ---------------------------------------------------------
ROOT_NAME.update({"EvalChannels":"M","OperatorPlanCursor":"R","OperatorPlanStep":"R",**{n:"M" for n in ["VALUE_TYPE_NUMBER","VALUE_TYPE_TEXT","VALUE_TYPE_BOOLEAN","VALUE_TYPE_LIST","VALUE_TYPE_VECTOR","VALUE_TYPE_POINT","VALUE_TYPE_GEOMETRY"]}})
RETIRE_ASSIGN_BY_TEXT=[("super::OperatorImpl","R"),("super::OperatorRecord","R"),("super::Registry","R")]
COLD_ASSIGN_BY_TEXT=[("for OperatorRecord","R"),("for Registry","R"),("for BudgetedEval","B"),("for NeuralCache","B")]
def assign_all():
    out=[]  # (file,crate,item_index,item)
    for f in FILES:
        L,items=load(f)
        for idx,it in enumerate(items):
            if f=="root":
                c=root_assign(it,L)
                if it['kind']=='other' and 'OperatorPlanCursor' in it['text']:c="R"
            elif f=="retirement":
                text=it['text']
                c="M"
                for key,cc in RETIRE_ASSIGN_BY_TEXT:
                    if key in text.split('\n')[0] or (it['kind']=='other' and key in text):c=cc
            elif f=="cold":
                head=L[it['head']].strip()
                c="M"
                for key,cc in COLD_ASSIGN_BY_TEXT:
                    if key in head:c=cc
            else:c=FILE_CRATE[f]
            out.append((f,c,idx,it,L))
    return out
def check():
    rows=assign_all()
    defs=collections.defaultdict(lambda:collections.defaultdict(set))  # crate -> name -> kinds
    for f,c,idx,it,L in rows:
        if it['kind'] in('struct','enum','fn','const','static','type','trait','union','macro') and it['name']:defs[c][it['name']].add(f)
    rank={c:i for i,c in enumerate(ORDER)}
    owners=collections.defaultdict(set)
    for c in defs:
        for n in defs[c]:owners[n].add(c)
    bad=collections.defaultdict(list)
    for f,c,idx,it,L in rows:
        for n in it['toks']:
            if n not in owners or c in owners[n]:continue
            # defined only in other crates; flag if all definers are higher
            if all(rank[o]>rank[c] for o in owners[n]):bad[(c,n)].append((f,it['start']+1,it['kind'],it['name'][:30]))
    return rows,bad
if __name__=="__main__" and len(sys.argv)>1 and sys.argv[1]=="check":
    rows,bad=check()
    tot=collections.Counter()
    for f,c,idx,it,L in rows:tot[c]+=it['bytes']
    print(tot)
    for (c,n),where in sorted(bad.items()):print(f"UP-REF {c} -> {n}: {where[:3]}")
