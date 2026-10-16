"""🏭 Neural engine carve generator: splits the monolith snapshot into model (M), registry (R), budgeted (B) producers and the evaluator facade (E). Deterministic from the snapshot; writes into the repo."""
import sys,os,re,collections,json
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from segs import parse
SNAP="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/snap/neural/"
DEST="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/"
PFX="semio_framework_os_kernel_neural_engine_"
CR={
 "M":dict(ident=PFX+"model",pkg="semio-framework-os-kernel-neural-engine-model",id="os-neural-engine-model",root="🧱️model/🦀️.rs",pkgdir="🧱️model/📦️packages/🦀️rust",lower=[]),
 "R":dict(ident=PFX+"registry",pkg="semio-framework-os-kernel-neural-engine-registry",id="os-neural-engine-registry",root="📔️registry/🦀️.rs",pkgdir="📔️registry/📦️packages/🦀️rust",lower=["M"]),
 "B":dict(ident=PFX+"budgeted",pkg="semio-framework-os-kernel-neural-engine-budgeted",id="os-neural-engine-budgeted",root="⏱️evaluation/🦀️.rs",pkgdir="⏱️evaluation/📦️packages/🦀️rust",lower=["M","R"]),
 "E":dict(ident="neural_engine",pkg="semio-framework-os-kernel-neural-engine",id="os-neural-engine",root="🦀️.rs",pkgdir="📦️packages/🦀️rust",lower=["M","R","B"]),
}
RANK={c:i for i,c in enumerate(["M","R","B","E"])}
FILES={"root":"🦀️.rs","retirement":"🧵️retirement/🦀️.rs","topology":"🧭️topology/🦀️.rs","input":"📥️input/🦀️.rs","evaluation":"⏱️evaluation/🦀️.rs","wave":"⏱️evaluation/🌊️wave/🦀️.rs","hash":"⏱️evaluation/🌊️wave/🔑️hash/🦀️.rs","cache_insert":"🌱️cache/🦀️.rs","tree_copy":"🌳️tree/📋️copy/🦀️.rs","cold":"🧊️cold/🦀️.rs","registry":"📔️registry/🦀️.rs","output":"📔️registry/✅️output/🦀️.rs","admission":"📔️registry/🏗️admission/🦀️.rs","lookup":"📖️lookup/🦀️.rs","projection":"📤️projection/🦀️.rs","native":"🚦️native/🦀️.rs","failure":"⚠️failure/🦀️.rs"}
FILE_CRATE={"topology":"B","input":"B","evaluation":"B","wave":"B","hash":"B","cache_insert":"B","tree_copy":"B","registry":"R","output":"R","admission":"R","lookup":"M","projection":"M","native":"M","failure":"M"}
TESTS={"M":["🧪️tests/🏷️type/🦀️.rs","🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs"],"R":["📔️registry/🧪️tests/📔️registry/🦀️.rs","📔️registry/✅️output/🧪️tests/🦀️.rs"],"B":["📥️input/🧪️tests/🦀️.rs","🌳️tree/📋️copy/🧪️tests/🦀️.rs","🧭️topology/🧪️tests/🦀️.rs","⏱️evaluation/🧪️tests/🦀️.rs","⏱️evaluation/🌊️wave/🧪️tests/🦀️.rs"],"E":["🧪️tests/🔬️unit/🦀️.rs","🧪️tests/🚦️owned-controls/🦀️.rs","🧪️tests/🌲️canonical/🦀️.rs"]}
ROOT_REGION={"🔖️Dictionary":"M","🔖️Schema":"M","⚠️ Errors":"M","🔖️SchemaComponent":"R","🔖️Tree":"M","🔖️Contract":"M","🔖️OperatorRecord":"M","🔖️Cache":"M","🔖️DirtyPropagation":"M","🔖️Evaluator":"E","🔖️Tests":"E"}
ROOT_NAME={"Registry":"R","OperatorImpl":"R","OperatorRecord":"R","OperatorPlanCursor":"R","OperatorPlanStep":"R",
 "NeuralCache":"B","NeuralCacheState":"B","NeuralCacheRetirement":"B","eval_error_dictionary":"B","evaluate_cached_output":"B",
 "budgeted_remaining":"B","waits_on_parked":"B","BudgetedEval":"B","EvalStepDeadline":"B","EvalStepBudget":"B","PendingExtensionEval":"B",
 "EvalChannels":"M","list_item_count":"M","validate_homogeneous_list":"M","validate_channel_value":"M","validate_neuron_inputs":"M","validate_operator_outputs":"M","channel_schema":"M","operator_signature":"M",
 **{n:"M" for n in ["VALUE_TYPE_NUMBER","VALUE_TYPE_TEXT","VALUE_TYPE_BOOLEAN","VALUE_TYPE_LIST","VALUE_TYPE_VECTOR","VALUE_TYPE_POINT","VALUE_TYPE_GEOMETRY"]}}
RETIRE_TO={"super::OperatorImpl":"R","super::OperatorRecord":"R","super::Registry":"R"}
COLD_TO=[("for OperatorRecord","R"),("for Registry","R"),("for BudgetedEval","B"),("for NeuralCache","B")]
TOP_MODS={"retirement","topology","input","evaluation","cache_insert","tree_copy","cold","registry","output"}
def text(s):return ''.join(s['lines'])
def btext(s):return ''.join(s['body'])
def impl_target(body):
    m=re.search(r'(?m)^[ \t]*(?:unsafe\s+)?impl\b([^{]*)',body)
    if not m:return ''
    head='impl'+m.group(1)
    mm=re.match(r'(?:unsafe\s+)?impl(?:<[^>]*(?:<[^>]*>[^>]*)*>)?\s+(?:[\w:<>\', &]+?\s+for\s+)?([\w:]+)',head)
    return mm.group(1).split('::')[-1] if mm else ''
def regions(segs):
    stack=[];out=[]
    for s in segs:
        if s['kind']=='marker':
            if s['name']=='region':stack.append(s['label'])
            elif stack:stack.pop()
        out.append((tuple(stack),s))
    return out
def root_crate(s,stack):
    ik=s['ikind']
    if ik=='impl':
        t=impl_target(btext(s))
        if t in ROOT_NAME:return ROOT_NAME[t]
    elif s['name'] in ROOT_NAME:return ROOT_NAME[s['name']]
    if ik=='other' and 'OperatorPlanCursor' in btext(s):return "R"
    return ROOT_REGION.get(stack[0] if stack else "","M")
def load(name):
    L,segs,trail=parse(SNAP+FILES[name]);return L,segs
def strip_doc(lines):
    i=0
    while i<len(lines) and lines[i].startswith('//!'):i+=1
    return lines[:i],lines[i:]
# ----------------------------------------------------------------------------- inventory
class Inv:
    def __init__(s):
        s.items=[]   # dict(file,crate,seg,stack,where)  where: 'root'|'child'
        s.header_uses=[]
        L,segs=load("root")
        rs=regions(segs)
        first_marker=next(i for i,(st,sg) in enumerate(rs) if sg['kind']=='marker')
        s.root_doc,_=strip_doc(segs[0]['lines'])
        for i,(st,sg) in enumerate(rs):
            if i<first_marker:
                if sg['ikind']=='use' and not re.match(r'\s*(?:///[^\n]*\n\s*)*(?:#\[[^\n]*\n\s*)*pub use (?:input|evaluation|cache_insert|tree_copy|retirement|cold|registry|output)::',text(sg).lstrip('\n')):
                    lines=strip_doc(sg['lines'])[1]
                    s.header_uses.append(''.join(lines))
                continue
            if sg['kind']=='marker':s.items.append(dict(file="root",crate=None,seg=sg,stack=st));continue
            s.items.append(dict(file="root",crate=root_crate(sg,st),seg=sg,stack=st))
        s.children={}
        for f in FILES:
            if f=="root":continue
            L,segs=load(f);s.children[f]=segs
inv=Inv()

# ----------------------------------------------------------------------------- streams
def prefix_paths(t):
    def f(m):
        p=m.group(2)
        return m.group(1)+(p if p.startswith("../") else "../"+p)+m.group(3)
    t=re.sub(r'(#\[path\s*=\s*")([^"]+)(")',f,t)
    t=re.sub(r'(include(?:_str|_bytes)?!\(\s*")([^"]+)("\s*\))',f,t)
    return t
def keep_regions(entries,pred):
    """entries: list of (stack,seg); returns segs kept with balanced markers (only regions that hold a kept item)"""
    idx=[i for i,(st,sg) in enumerate(entries) if sg['kind']!='marker' and pred(st,sg)]
    keepset=set(idx)
    # pair markers
    pairs={};stack=[]
    for i,(st,sg) in enumerate(entries):
        if sg['kind']=='marker':
            if sg['name']=='region':stack.append(i)
            elif stack:j=stack.pop();pairs[j]=i;pairs[i]=j
    keepm=set()
    for j,i in list(pairs.items()):
        if j<i:
            if any(j<k<i for k in keepset):keepm.add(j);keepm.add(i)
    out=[]
    for i,(st,sg) in enumerate(entries):
        if i in keepset or i in keepm:out.append(sg)
    return out
def root_stream(crate):
    return keep_regions([(it['stack'],it['seg']) for it in inv.items if it['seg']['kind']=='marker' or True],lambda st,sg:False) # placeholder
def root_entries():
    return [(it['stack'],it['seg'],it['crate']) for it in inv.items]
def root_segments_for(crate):
    ents=[(st,sg) for st,sg,c in root_entries()]
    cm={id(sg):c for st,sg,c in root_entries()}
    return keep_regions(ents,lambda st,sg:cm[id(sg)]==crate)
def child_entries(name):
    return regions(inv.children[name])
def split_child(name,rule):
    """returns dict crate->list of segs for a child file where rule(seg)->crate (None = drop)"""
    ents=child_entries(name)
    cm={id(sg):rule(sg) for st,sg in ents if sg['kind']!='marker'}
    out={}
    for c in set(v for v in cm.values() if v):
        out[c]=keep_regions(ents,lambda st,sg,c=c:cm[id(sg)]==c)
    return out
def retire_rule(sg):
    t=btext(sg)
    for k,c in RETIRE_TO.items():
        if k in t.split('\n')[0] or (sg['ikind']=='other' and k in t):return c
    return "M"
def cold_rule(sg):
    head=btext(sg).split('{')[0]
    for k,c in COLD_TO:
        if k in head:return c
    return "M"
retire_parts=split_child("retirement",retire_rule)
cold_parts=split_child("cold",cold_rule)

# ----------------------------------------------------------------------------- crate content model
KINDS_NAMED={"struct","enum","fn","const","static","type","trait","union"}
def file_tokens(rel):
    L,segs,tr=parse(DEST_ORIG_TEST(rel))
    s=set()
    for sg in segs:
        if sg['kind']=='item':s|=sg['toks']
    return s
def DEST_ORIG_TEST(rel):return SNAP+rel
def file_segs(name):
    return [sg for sg in inv.children[name]]
def non_marker(segs):return [s for s in segs if s['kind']=='item']
def toks_of(segs):
    t=set()
    for s in segs:
        if s['kind']=='item':t|=s['toks']
    return t
STREAMS={}  # crate -> ordered list of ("seg", seg, origin) for root-file content
def reg_file_segments(name):
    """segments of an existing file that becomes a crate root: drops `use super/crate` lines, rewrites `super::`"""
    segs=inv.children[name]
    out=[]
    for sg in segs:
        if sg['kind']=='item' and sg['ikind']=='use':
            b=btext(sg).strip()
            m=re.match(r'(?:#\[[^\n]*\]\s*)*use\s+super::\{((?:\w+::)[^;]*)\};$',b)
            if m:
                inner=m.group(1)
                inner=re.sub(r'\w+::\{[^}]*\},?','',inner).strip().strip(',')
                if inner:
                    sg=dict(sg);sg['lines']=[f"use {{{inner}}};\n"];out.append(sg)
                continue
            if re.match(r'(?:#\[[^\n]*\]\s*)*use\s+(?:super|crate)::',b):continue
        out.append(sg)
    return out
def rewrite_super(t):
    t=re.sub(r'(?<![\w:])super::super::','',t) if False else t
    return re.sub(r'(?<![\w:])super::(?=[A-Za-z_])','crate::',t)
M_CHILD={"lookup","projection","native","failure"}
def stream_segments(crate):
    """ordered segment lists composing the root file body of a crate, per origin"""
    if crate=="M":
        return [("root",root_segments_for("M"))]
    if crate=="R":
        return [("file",reg_file_segments("registry")),("root",root_segments_for("R")),("retire",retire_parts.get("R",[])),("cold",cold_parts.get("R",[]))]
    if crate=="B":
        return [("file",reg_file_segments("evaluation")),("root",root_segments_for("B")),("cold",cold_parts.get("B",[]))]
    if crate=="E":
        return [("root",root_segments_for("E"))]
def module_files(crate):
    if crate=="M":return [("retirement",retire_parts["M"]),("cold",cold_parts["M"]),("lookup",inv.children["lookup"]),("projection",inv.children["projection"]),("native",inv.children["native"]),("failure",inv.children["failure"])]
    if crate=="R":return [("output",inv.children["output"]),("admission",inv.children["admission"])]
    if crate=="B":return [("topology",inv.children["topology"]),("input",inv.children["input"]),("cache_insert",inv.children["cache_insert"]),("tree_copy",inv.children["tree_copy"]),("wave",inv.children["wave"]),("hash",inv.children["hash"])]
    return []
def crate_lib_tokens(crate):
    t=set()
    for origin,segs in stream_segments(crate):t|=toks_of(segs)
    for name,segs in module_files(crate):t|=toks_of(segs)
    return t
def crate_tokens(crate):
    t=crate_lib_tokens(crate)
    for rel in TESTS[crate]:
        t|=file_tokens(rel)
    return t
def seg_vis(sg):
    h=sg['lines'][sg['head']-sg['start']] if 'head' in sg else ''
    m=re.match(r'\s*(pub(?:\([^)]*\))?)\s',h)
    return m.group(1) if m else ''
def named_defs(crate):
    """name -> (seg, origin_vis) for named root-level items of the crate (streams only)"""
    d={}
    for origin,segs in stream_segments(crate):
        for sg in segs:
            if sg['kind']=='item' and sg['ikind'] in KINDS_NAMED and sg['name']:
                d[sg['name']]=(sg,seg_vis(sg))
    return d
def parse_use_names(stmt):
    """'use a::b::{X, Y as Z};' -> (pre, vis, prefix, [(name,alias)]) ; returns None if not parseable"""
    pre=[];lines=stmt.split('\n');k=0
    while k<len(lines) and (not lines[k].strip() or lines[k].strip().startswith('//') or lines[k].strip().startswith('#[')):pre.append(lines[k]);k+=1
    body='\n'.join(lines[k:]).strip()
    m=re.match(r'^(pub\s+)?use\s+(.*?);$',body,re.S)
    if not m:return None
    vis=(m.group(1) or '').strip();path=m.group(2).strip()
    mm=re.match(r'^(.*?)::\{(.*)\}$',path,re.S)
    if mm:
        prefix=mm.group(1);items=[x.strip() for x in mm.group(2).split(',') if x.strip()]
    else:
        prefix,_,last=path.rpartition('::');items=[last]
    names=[]
    for it in items:
        a=re.match(r'^(\w+)\s+as\s+(\w+)$',it)
        names.append((a.group(1),a.group(2)) if a else (it,it))
    return '\n'.join(pre).strip('\n'),vis,prefix,names

# ----------------------------------------------------------------------------- exports, widening, imports
def pub_use_names(crate):
    names={}
    if crate=="M":
        for n in ["ColdDictionaryBuilder","ColdValueOwner","ValueRetirement","ColdOwner","ColdRetire","RetainedCloneGrant","RetainedCloneProgress","RetainedCloneStep","OriginalDictionaryLookup"]:names[n]=True
    if crate=="R":
        for n in ["OperatorFinishCursor","SharedRegistryIssuer","SharedRegistryAdmissionStep"]:names[n]=True
    if crate=="B":
        for n in ["BudgetedInputMerge","BudgetedOwnedEntry","NeuralCacheInsert","BudgetedTreeCopy"]:names[n]=True
    return names
EXPORTS={}
for c in CR:
    d={n:(sg,vis) for n,(sg,vis) in named_defs(c).items()}
    EXPORTS[c]={"defs":d,"pubuse":pub_use_names(c)}
def export_names(c):return set(EXPORTS[c]["defs"])|set(EXPORTS[c]["pubuse"])
TOK={c:crate_tokens(c) for c in CR}
WIDEN={c:set() for c in CR}
for y in CR:
    for n in EXPORTS[y]["defs"]:
        if any(RANK[x]>RANK[y] and n in TOK[x] for x in CR):WIDEN[y].add(n)
def make_pub(sg):
    k=sg['head']-sg['start']
    line=sg['lines'][k]
    m=re.match(r'^(\s*)(pub(?:\([^)]*\))?\s+)?(.*)$',line,re.S)
    if m.group(2) and m.group(2).strip()=='pub':return
    sg['lines'][k]=m.group(1)+'pub '+m.group(3)
widened=collections.defaultdict(list)
for y in CR:
    for n in sorted(WIDEN[y]):
        sg,vis=EXPORTS[y]["defs"][n]
        if vis!='pub':
            make_pub(sg);widened[y].append((n,vis or 'private'))
def lower_map(crate):
    m={}
    for y in reversed(CR[crate]["lower"]):  # nearer crates override
        pass
    for y in CR[crate]["lower"]:
        for n in export_names(y):m.setdefault(n,y)
    return m
def bound_names(tree):
    tree=tree.strip()
    if '{' in tree:
        pre,_,rest=tree.partition('{');inner=rest.rsplit('}',1)[0]
        out=set();depth=0;cur=''
        for ch in inner+',':
            if ch=='{':depth+=1
            if ch=='}':depth-=1
            if ch==',' and depth==0:
                if cur.strip():out|=bound_names(cur)
                cur=''
            else:cur+=ch
        return out
    m=re.search(r'\bas\s+(\w+)$',tree)
    if m:return {m.group(1)}
    return {tree.rsplit('::',1)[-1].strip()}
def file_imported_names(crate):
    names=set()
    for origin,segs in stream_segments(crate):
        if origin!="file":continue
        for sg in segs:
            if sg['kind']=='item' and sg['ikind']=='use':
                b=btext(sg).strip()
                m=re.match(r'(?:#\[[^\n]*\]\s*)*(?:pub\s+)?use\s+(.*);$',b,re.S)
                if m and not re.match(r'(?:super|crate)::',m.group(1)):names|=bound_names(m.group(1))
    return names
EXCLUDE_IMPORTS={"E":{"Ordering"}}
def build_imports(crate):
    file_imported=file_imported_names(crate)
    toks=TOK[crate];lib=crate_lib_tokens(crate);lines=[]
    local=set(EXPORTS[crate]["defs"])
    def emit(pre,kw,prefix,used,cfgtest):
        items=[n if n==a else f"{n} as {a}" for n,a in used]
        if pre and not cfgtest:lines.append(pre)
        elif pre and cfgtest:lines.append(pre)
        if cfgtest and not (pre and '#[cfg(test)]' in pre):lines.append("#[cfg(test)]")
        lines.append(f"{kw} {prefix}::{{{', '.join(items)}}};" if len(items)>1 or '{' in prefix else f"{kw} {prefix}::{items[0]};")
    for u in inv.header_uses:
        pre,vis,prefix,names=parse_use_names(u)
        kw="pub use" if vis=="pub" else "use"
        if vis=='pub' and crate!="M":continue
        used=[(n,a) for n,a in names if (a in toks or n in toks) and a not in EXCLUDE_IMPORTS.get(crate,())]
        if vis=='pub':used=names
        if vis!="pub":used=[(n,a) for n,a in used if a not in file_imported]
        if not used:continue
        libu=[(n,a) for n,a in used if vis=='pub' or a in lib or n in lib]
        testu=[(n,a) for n,a in used if (n,a) not in libu]
        if libu:emit(pre,kw,prefix,libu,False)
        if testu:emit(pre if not libu else '',kw,prefix,testu,True)
    lm=lower_map(crate)
    per=collections.defaultdict(list);pert=collections.defaultdict(list)
    faced=facade_names() if crate=="E" else set()
    for n in sorted(toks):
        if n in lm and n not in local and not n.startswith('_') and n not in faced:
            (per if n in lib else pert)[lm[n]].append(n)
    for y in CR[crate]["lower"]:
        if per[y]:lines.append(f"use {CR[y]['ident']}::{{{', '.join(per[y])}}};")
        if pert[y]:lines.append("#[cfg(test)]");lines.append(f"use {CR[y]['ident']}::{{{', '.join(pert[y])}}};")
    return lines,per
# ----------------------------------------------------------------------------- assembly
def seg_out(sg,origin,crate):
    t=''.join(sg['lines'])
    if origin=="root" and crate in("M","R","B"):t=prefix_paths(t)
    if origin=="file":t=rewrite_super(t)
    if origin=="retire":t=re.sub(r'\((?:super::)',"(",t)
    if crate=="B":t=t.replace(".pairs.",".pairs().")
    if not t.endswith('\n'):t+='\n'
    return t
def needs_allocator(crate):
    return bool({"Allocator","AllocError"}&TOK[crate])
def mount_lines(crate):
    out=[]
    if crate=="M":
        out+=['#[path = "../🧵️retirement/🦀️.rs"]','pub mod retirement;','pub use retirement::{ColdDictionaryBuilder,ColdValueOwner,ValueRetirement};',
              '#[path = "../🧊️cold/🦀️.rs"]','pub mod cold;','pub use cold::{ColdOwner, ColdRetire};',
              '#[cfg(any(test, feature = "testkit"))]','#[path = "🧪️tests/🧰️testkit/🦀️.rs"]','pub mod testkit;']
    if crate=="R":
        out+=['#[path = "✅️output/🦀️.rs"]','mod output;','pub use output::OperatorFinishCursor;']
    if crate=="B":
        out+=['#[path = "../📥️input/🦀️.rs"]','pub mod input;','pub use input::{BudgetedInputMerge,BudgetedOwnedEntry};',
              '#[path = "../🧭️topology/🦀️.rs"]','pub mod topology;',
              '#[path = "../🌱️cache/🦀️.rs"]','pub mod cache_insert;','pub use cache_insert::NeuralCacheInsert;',
              '#[path = "../🌳️tree/📋️copy/🦀️.rs"]','pub mod tree_copy;','pub use tree_copy::BudgetedTreeCopy;']
    return out
def facade_items(y):
    names=[]
    for n,(sg,vis) in sorted(EXPORTS[y]["defs"].items()):
        if ORIG_PUB.get(n):names.append(n)
    names+=[n for n in sorted(EXPORTS[y]["pubuse"])]
    mods={"M":["cold","retirement"],"R":[],"B":["cache_insert","input","topology","tree_copy"]}[y]
    return sorted(set(names+mods))
def facade_names():
    s=set()
    for y in ["M","R","B"]:s|=set(facade_items(y))
    return s
def facade_lines():
    out=[]
    for y in ["M","R","B"]:
        items=facade_items(y)
        out.append(f"pub use {CR[y]['ident']}::{{{', '.join(items)}}};" if items else '')
    out.append(f"pub use {CR['R']['ident']} as registry;")
    out.append(f"pub use {CR['B']['ident']} as evaluation;")
    return [l for l in out if l]
ORIG_PUB={}
for it in inv.items:
    sg=it['seg']
    if sg['kind']=='item' and sg['ikind'] in KINDS_NAMED and sg['name']:
        ORIG_PUB[sg['name']]=ORIG_PUB.get(sg['name'],False) or bool(re.match(r'pub\s',(sg['lines'][sg['head']-sg['start']]).strip()+' '))
for f in ["registry","evaluation"]:
    for sg in inv.children[f]:
        if sg['kind']=='item' and sg['ikind'] in KINDS_NAMED and sg['name']:
            ORIG_PUB[sg['name']]=ORIG_PUB.get(sg['name'],False) or bool(re.match(r'pub\s',(sg['lines'][sg['head']-sg['start']]).strip()+' '))
def crate_doc(crate):
    if crate=="M":return ["//! 🧱️ Neural engine model: dictionary, schema, tree and operator vocabulary with their retirement, hashing and validation laws.\n"]
    if crate=="E":return list(inv.root_doc)
    f={"R":"registry","B":"evaluation"}[crate]
    d,_=strip_doc(inv.children[f][0]['lines'])
    return d
def assemble(crate):
    out=[]
    out+=crate_doc(crate)
    if crate!="E" and needs_allocator(crate):out.append("#![feature(allocator_api)]\n")
    out.append("\n")
    imp,per=build_imports(crate)
    out+=[l+"\n" for l in imp]
    ml=mount_lines(crate)
    if ml:out.append("\n");out+=[l+"\n" for l in ml]
    if crate=="E":
        out.append("\n");out+=[l+"\n" for l in facade_lines()]
    for origin,segs in stream_segments(crate):
        if not segs:continue
        out.append("\n")
        for i,sg in enumerate(segs):
            if origin=="file" and i==0:
                d,rest=strip_doc(sg['lines']);sg=dict(sg);sg['lines']=rest
            out.append(seg_out(sg,origin,crate))
    res=''.join(out)
    if crate=="M":
        anchor="    pub fn len(&self) -> usize {\n        self.pairs.len()\n    }\n"
        assert res.count(anchor)==1
        res=res.replace(anchor,"    /// 🧬️ Borrows the ordered owner so the budgeted merge hands shared roots over without cloning values.\n    pub fn pairs(&self) -> &OrderedMap<Value> {\n        &self.pairs\n    }\n\n"+anchor)
    return res
def write_children():
    rel=lambda f:DEST+FILES[f]
    def join(segs):
        o=''.join(''.join(sg['lines']) if ''.join(sg['lines']).endswith('\n') else ''.join(sg['lines'])+'\n' for sg in segs)
        return o
    inp=''.join(inv_src("input"))
    assert inp.count(".pairs.")==3,inp.count(".pairs.")
    open(rel("input"),"w",encoding="utf-8").write(inp.replace(".pairs.",".pairs()."))
    open(rel("retirement"),"w",encoding="utf-8").write(join(retire_parts["M"]))
    open(rel("cold"),"w",encoding="utf-8").write(join(cold_parts["M"]))
    w=''.join(inv_src("wave"))
    assert w.count("use super::super::{PendingExtensionEval,OperatorPendingPlan};")==1
    open(rel("wave"),"w",encoding="utf-8").write(w.replace("use super::super::{PendingExtensionEval,OperatorPendingPlan};","use super::{PendingExtensionEval,OperatorPendingPlan};"))
def inv_src(name):
    return open(SNAP+FILES[name],encoding="utf-8").read().splitlines(True)
if __name__=="__main__":
    for c in "MRBE":
        t=assemble(c)
        p=DEST+CR[c]["root"]
        os.makedirs(os.path.dirname(p),exist_ok=True)
        open(p,"w",encoding="utf-8").write(t)
        print(c,p[-60:],len(t.encode()),t.count('\n'))
    write_children()
    print("widened",dict(widened))
