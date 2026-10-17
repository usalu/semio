"""✂️ D3 spill valve of PLAN-OS-KERNEL-CRATES.md: moves `advance_apply_batch` (+ `spend_item_close_grant`), `ingest_remote_merge` and the pure `edits_for_ids` out of `os-store-engine` into `os-store-engine-bundle` as functions over the `StoreParts` bundle. The composer keeps one-line delegates with the old signatures.
Every `self.helper(..)` of the moved bodies becomes a call of a private bundle helper that has the body of the composer's delegate; `self.<component>` becomes `store.<component>`. `protocol` and `pack` stay: they are the library names of `semio-framework-replication` and `semio-framework-pack`.
usage: d3_split.py dry|write   (dry writes the candidate files under 🗑️generated/rsl-d3/out/)"""
import json, os, re, sys, shutil
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
S="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
ENG=S+"/⚙️engine/📥️lifted/🏪️store/🦀️.rs"
GROUP=S+"/🧩️composition/📬️publication/🤝️group/🦀️.rs"
NEW=S+"/⚙️engine/🧳️bundle"
PRE="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rsl-pre/"
write=sys.argv[1]=="write"
OUT=G+"/rsl-d3/out/"
BOUND_P="Clone + ToValue + FromValue + Send + Sync + 'static"
BOUND_M="Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static"

t=open(ENG,encoding="utf8").read()
orig=t
def block_end(text,i):
    depth=0; started=False
    while True:
        c=text[i]
        if c=='{': depth+=1; started=True
        elif c=='}':
            depth-=1
            if started and depth==0: return i+1
        i+=1
def doc_start(text,fn_pos):
    ls=text.rfind("\n",0,fn_pos)+1
    cur=ls
    while True:
        prev=text.rfind("\n",0,cur-1)+1
        line=text[prev:cur-1]
        if line.startswith("    ///") or line.startswith("    #["): cur=prev
        else: break
    return cur
def span(name,kw):
    m=re.search(r'\n(    (?:pub )?(?:async )?fn '+name+r'\b)',t)
    assert m and len(re.findall(r'\n    (?:pub )?(?:async )?fn '+name+r'\b',t))==1,name
    fn_pos=m.start()+1
    s=doc_start(t,fn_pos)
    e=block_end(t,fn_pos)
    # include trailing newline
    if t[e]=="\n": e+=1
    return s,e,fn_pos
# --- spans
s_adv,e_adv,p_adv=span("advance_apply_batch",None)
s_sp,e_sp,p_sp=span("spend_item_close_grant",None)
s_ing,e_ing,p_ing=span("ingest_remote_merge",None)
s_edi,e_edi,p_edi=span("edits_for_ids",None)
assert e_adv<=s_sp+1 and s_adv<s_sp<s_ing<s_edi or True
spans=sorted([(s_adv,e_adv,"adv"),(s_sp,e_sp,"spend"),(s_ing,e_ing,"ing"),(s_edi,e_edi,"edi")])
for a,b in zip(spans,spans[1:]): assert a[1]<=b[0],(a,b)
texts={k:t[s:e] for s,e,k in spans}
# --- helpers from the delegates
def parse_delegate(name):
    m=re.search(r'\n    (#\[inline\] )?(?:pub )?(async )?fn '+name+r'(<[^>]*>)?\(',t[e_edi:] if False else t)
    cands=[x for x in re.finditer(r'\n    #\[inline\] (?:pub )?(async )?fn '+name+r'(<[^>]*>)?\(',t)]
    assert len(cands)==1,(name,len(cands))
    c=cands[0]; start=c.end()-1
    depth=0;i=start
    while True:
        if t[i]=='(': depth+=1
        elif t[i]==')':
            depth-=1
            if depth==0: break
        i+=1
    params=t[start+1:i]
    line_end=t.index("\n",i)
    rest=t[i+1:line_end]
    m2=re.match(r'\s*(?:->\s*(.*?)\s*)?(?:where P: Sync,\s*)?\{ (.*) \}\s*$',rest)
    assert m2,(name,rest)
    ret,body=m2.group(1),m2.group(2)
    is_async=bool(c.group(1)); generics=c.group(2)
    recv=re.match(r'\s*(&mut self|&self)\s*(?:,\s*(.*))?$',params,re.S)
    assert recv,(name,params)
    return dict(name=name,is_async=is_async,generics=generics,recv=recv.group(1),params=(recv.group(2) or "").strip(),ret=ret,body=body)
used_helpers=sorted(set(re.findall(r'self\.([a-z_][a-z0-9_]*)\(',texts["adv"]+texts["ing"])))
helpers={n:parse_delegate(n) for n in used_helpers}
def conv_body(b):
    b=re.sub(r'&mut self\.(log|owners|wire|cursor|live|replay)\b',r'&mut *store.\1',b)
    b=re.sub(r'&self\.(log|owners|wire|cursor|live|replay)\b',r'&*store.\1',b)
    b=re.sub(r'\bself\.(log|owners|wire|cursor|live|replay)\b',r'store.\1',b)
    assert "self" not in re.sub(r'self::','',b),b
    return b
H=[]
for n in used_helpers:
    h=helpers[n]
    params=re.sub(r'\bmut (\w+):',r'\1:',h["params"])
    gen=h["generics"] or ""
    extra=gen[1:-1] if gen else ""
    gparams=("<"+extra+", P, Mutation>") if extra else "<P, Mutation>"
    store_ty="&mut StoreParts<'_, P, Mutation>" if h["recv"]=="&mut self" else "&StoreParts<'_, P, Mutation>"
    ret=h["ret"]
    if n=="batch_amend_target_ref":
        gparams="<'s, P, Mutation>"; store_ty="&'s StoreParts<'_, P, Mutation>"; ret=ret.replace("Option<&str>","Option<&'s str>")
    sig=f"fn {n}{gparams}(store: {store_ty}{', '+params if params else ''})"+(f" -> {ret}" if ret else "")
    if h["is_async"]: sig="async "+sig
    H.append(f"#[inline]\n{sig}\nwhere\n    P: {BOUND_P},\n    Mutation: {BOUND_M},\n{{ {conv_body(h['body'])} }}\n")
# --- transform the moved bodies
def dedent(s): return "\n".join((l[4:] if l.startswith("    ") else l) for l in s.split("\n"))
def to_free(text,name,is_async):
    # signature
    m=re.search(r'((?:pub )?(?:async )?)fn '+name+r'\(&mut self,? ?',text)
    assert m
    pre=text[:m.start()]
    rest=text[m.end():]
    # params end: matching paren
    depth=1;i=0
    while depth:
        if rest[i]=='(': depth+=1
        elif rest[i]==')': depth-=1
        i+=1
    params=rest[:i-1]
    after=rest[i:]
    mb=re.match(r'\s*->\s*(.*?)\s*(?:where\s*P: Sync,\s*)?\{',after,re.S)
    ret=mb.group(1).strip(); body=after[mb.end():]
    vis=m.group(1) if 'pub' in m.group(1) else 'pub '+m.group(1)
    sig=f"{vis}fn {name}<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>{', '+params if params.strip() else ''}) -> {ret}\nwhere\n        P: {BOUND_P},\n        Mutation: {BOUND_M},\n    {{"
    return pre+sig+body
def conv_calls(code):
    for n in used_helpers:
        code=re.sub(r'\bself\.'+n+r'\(\s*\)',n+'(store)',code)
        code=re.sub(r'\bself\.'+n+r'\(',n+'(store, ',code)
    code=re.sub(r'\bSelf::(spend_item_close_grant|edits_for_ids|fold_history)\(',r'\1(',code)
    code=re.sub(r'\bself\.(log|owners|wire|cursor|live|replay)\b',r'store.\1',code)
    return code
adv=dedent(to_free(texts["adv"],"advance_apply_batch",False))
ing=dedent(to_free(texts["ing"],"ingest_remote_merge",True))
sp=dedent(texts["spend"])
edi=dedent(texts["edi"])
adv=conv_calls(adv); ing=conv_calls(ing)
# spend_item_close_grant: static fn of the impl
sp=re.sub(r'fn spend_item_close_grant\(',"fn spend_item_close_grant<P, Mutation>(",sp)
sp=sp.replace("-> Result<RetainedCloneStep, ValueError> {","-> Result<RetainedCloneStep, ValueError>\nwhere\n    P: "+BOUND_P+",\n    Mutation: "+BOUND_M+",\n{",1)
edi=edi.replace("pub fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {","pub fn edits_for_ids<Mutation: Clone>(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {")
assert "edits_for_ids<Mutation: Clone>" in edi
for name,code in (("adv",adv),("ing",ing)):
    left=[m.group(0) for m in re.finditer(r'(?<![A-Za-z0-9_])(?:self|Self)\b(?!::Mutation)',re.sub(r'(?m)^\s*//.*$','',code))]
    assert not left,(name,left[:5])
# --- imports
head_lines=t.split("\n")
uses=[]
for i,l in enumerate(head_lines):
    if l.startswith("use ") and l.rstrip().endswith(";"): uses.append(l)
def parse_use(line):
    body=line[4:].rstrip(";").strip()
    out=[]
    def rec(prefix,s):
        s=s.strip()
        if "{" in s:
            pre,rest=s.split("{",1); inner=rest.rsplit("}",1)[0]
            depth=0;cur="";parts=[]
            for ch in inner:
                if ch=="{": depth+=1
                if ch=="}": depth-=1
                if ch=="," and depth==0: parts.append(cur);cur=""
                else: cur+=ch
            if cur.strip(): parts.append(cur)
            for p in parts: rec(prefix+pre.strip(),p)
        else:
            m=re.match(r'^(.*?)(?:\s+as\s+(\w+))?$',s)
            path=(prefix+m.group(1)).strip(); alias=m.group(2)
            if path.endswith("::self"): path=path[:-6]
            out.append((path,alias))
    rec("",body)
    return out
leaves=[]
for l in uses:
    for p,a in parse_use(l):
        name=a or p.split("::")[-1]
        leaves.append((name,p,a))
code_all=adv+ing+sp+edi+"".join(H)
tokens=set(re.findall(r'(?<![.\w])(?<!::)[A-Za-z_][A-Za-z0-9_]*',re.sub(r'"(?:\\.|[^"\\])*"','""',re.sub(r'(?m)^\s*//.*$','',code_all))))
need={}
for name,p,a in leaves:
    if name in tokens and name!="_" and not p.endswith("::*"): need[(p,a)]=name
needed_uses=sorted(need)
globs=[p for (name,p,a) in leaves if p.endswith("::*")]
extra_uses=["semio_framework_os_store_engine_live::lifted_store::StoreParts","semio_framework_os_store_engine_core::lifted_store::fold_history"]
for u in extra_uses:
    if (u,None) not in need: needed_uses.append((u,None))
header_doc="""//! 🧳️ Use cases of the store composer that need the whole `StoreParts` bundle: the batched one-item publication advance, the remote-merge ingest and the pure edit-id resolution.
//!
//! 🧭️ Spill valve D3 of `PLAN-OS-KERNEL-CRATES.md` (sections 5.3, 7.5 and 13): the bodies are the composer's, over the six component borrows; `ArtifactStore` in `os-store-engine` keeps delegates with the old signatures.

"""
use_text="\n".join(f"use {p}{(' as '+a) if a else ''};" for p,a in sorted(set(needed_uses)))
glob_text="\n".join(f"use {g};" for g in globs if any(True for _ in [0]) and False)
new_src=header_doc+use_text+"\n\n//#region 📬️OneItemPublication\n"+adv+"\n"+sp+"//#endregion 📬️OneItemPublication\n\n"+ing+"\n"+edi+"\n//#region 🪞️BundleHelpers\n"+"\n".join(H)+"//#endregion 🪞️BundleHelpers\n"
# --- the composer file after the cut
delegates={
 "adv": """    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::advance_apply_batch`]: advances at most one preparation, fold, cursor, preflight or atomic move-publication unit of the batched publication.
    #[inline]
    pub fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError>
    where
        P: Sync,
    {
        semio_framework_os_store_engine_bundle::advance_apply_batch(&mut self.parts(), publication, grant)
    }
""",
 "ing": """    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::ingest_remote_merge`]: feeds one remote envelope through the causal DAG and merges it into the edit timeline.
    #[inline]
    async fn ingest_remote_merge(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError> {
        semio_framework_os_store_engine_bundle::ingest_remote_merge(&mut self.parts(), envelope).await
    }
""",
 "edi": """    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::edits_for_ids`]: the strict id-to-edit resolution of the merge.
    #[inline]
    pub fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
        semio_framework_os_store_engine_bundle::edits_for_ids(ids, edits)
    }
""",
 "spend": "",
}
out=t
for s,e,k in sorted(spans,reverse=True):
    out=out[:s]+delegates[k]+out[e:]
if "\n\n\n" in out and False: pass
# unbound kernel alias heads in the remaining composer text
os.makedirs(OUT,exist_ok=True)
open(OUT+"bundle.rs","w",encoding="utf8").write(new_src)
open(OUT+"engine.rs","w",encoding="utf8").write(out)
print("bundle bytes",len(new_src.encode()),"lines",new_src.count("\n"),"| engine bytes",len(orig.encode()),"->",len(out.encode()))
print("helpers",len(used_helpers),"uses",len(set(needed_uses)))
print("unused-check: tokens of globs?",globs)
json.dump(dict(uses=[list(x) for x in sorted(set(needed_uses))],globs=globs),open(OUT+"imports.json","w"),ensure_ascii=False)
if write:
    cur=open(ENG,encoding="utf8").read()
    assert cur==orig,"concurrent edit of the composer file; re-run"
    d=PRE+ENG
    if not os.path.exists(d): os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(ENG,d)
    os.makedirs(NEW,exist_ok=True)
    assert not os.path.exists(NEW+"/🦀️.rs"),"new crate root exists"
    open(NEW+"/🦀️.rs","w",encoding="utf8").write(new_src)
    open(ENG,"w",encoding="utf8").write(out)
    print("WROTE")
