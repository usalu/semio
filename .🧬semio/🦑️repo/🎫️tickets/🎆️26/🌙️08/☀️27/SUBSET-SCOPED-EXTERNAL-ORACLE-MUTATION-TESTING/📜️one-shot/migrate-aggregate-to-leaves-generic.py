import io,json,os,re,sys

VERBS=set("add append apply bind change clear commit connect create delete disconnect drag duplicate edit extract finish fix flatten group inline insert merge move remove rename reorder replace resize restore rotate scale seal set split start switch toggle unbind unflatten ungroup".split())
# 🧩️One emoji per leaf directory. Written as an explicit LIST: `"🔧🔩…".split()` returns the whole
# string as a single element, which silently produced 14 directories all prefixed with the entire pool.
POOL=["🔧","🔩","⚙","🧩","🔖","🏷","📐","📏","🧮","🔢","🔤","🧷","📎","📌","🗂","🗃","🗄","📤","📥","📦",
      "🎯","🎚","🎛","🔭","🔬","🧪","🧫","🧬","🪛","🪚","🪝","🪟","🪞","🪣","🧰","🧱","🪵","🪶","🧶","🧵"]

def kebab(v): return re.sub(r'(?<!^)(?=[A-Z])','-',v).lower()
def snake(v): return kebab(v).replace("-","_")

def brace_match(s,i):
    d=0
    while i<len(s):
        if s[i]=="{": d+=1
        elif s[i]=="}":
            d-=1
            if d==0: return i
        i+=1
    raise ValueError("unbalanced")

def parse_variants(body):
    """[(name, 'unit'|'struct', fields_src)] in declaration order."""
    out=[]; i=0
    while i<len(body):
        m=re.compile(r"^\s{4}([A-Z]\w*)\s*(\{|\(|,)",re.M).search(body,i)
        if not m: break
        name,kind=m.group(1),m.group(2)
        if kind=="{":
            j=brace_match(body,m.end(1)+body[m.end(1):].index("{"))
            out.append((name,"struct",body[body.index("{",m.end(1))+1:j])); i=j+1
        elif kind=="(":
            out.append((name,"newtype",None)); i=m.end()
        else:
            out.append((name,"unit",None)); i=m.end()
    return out

def newtype_rewrite(s,agg,mapping):
    for var,(mod,ty) in mapping.items():
        needle=f"{agg}::{var} {{"; out=[]; i=0
        while True:
            j=s.find(needle,i)
            if j==-1: out.append(s[i:]); break
            k=brace_match(s,s.index("{",j))
            out.append(s[i:j]); out.append(f"{agg}::{var}({mod}::{ty} {s[s.index('{',j):k+1]})"); i=k+1
        s="".join(out)
        s=s.replace(f"{agg}::{var}({mod}::{ty} {{ .. }})",f"{agg}::{var}(_)")
    return s

def migrate(aggfile, dry=False):
    root=os.path.dirname(aggfile)
    s=io.open(aggfile,encoding="utf8").read()
    m=re.search(r"pub enum (\w+) \{",s)
    if not m: return f"SKIP {root}: no enum"
    agg=m.group(1)
    end=brace_match(s,s.index("{",m.start()))
    variants=parse_variants(s[s.index("{",m.start())+1:end])
    if any(k=="newtype" for _,k,_ in variants): return f"SKIP {root}: already newtype"
    snap=re.search(r"impl Mutation<(\w+)> for "+agg,s)
    if not snap: return f"SKIP {root}: no hand-written impl Mutation"
    snap=snap.group(1)
    diffty=re.search(r"type Diff = (\w+);",s)
    if not diffty: return f"SKIP {root}: no type Diff"
    diffty=diffty.group(1)
    leaves=[]; bad=[]
    for name,kind,fields in variants:
        if name=="NoMutation": continue
        k=kebab(name); verb=k.split("-")[0]
        if verb not in VERBS: bad.append((name,verb)); continue
        if "-" not in k: bad.append((name,"single-word-kind")); continue
        leaves.append((name,k,verb,fields or ""))
    if bad: return f"SKIP {root}: unmappable {bad}"
    if not leaves: return f"SKIP {root}: nothing to migrate"
    # ── the hand-written impl becomes free functions over the aggregate ──────────────────────────
    ms=s.index(f"impl Mutation<{snap}> for {agg} {{"); me=brace_match(s,s.index("{",ms))
    block=s[ms:me+1]
    if block.count("fn ")>3: return f"SKIP {root}: impl carries more than diff/inverse"
    def lift(sig, newname):
        i=block.index(sig); j=brace_match(block,block.index("{",i))
        # 🐛️`Self::` MUST be spelled out. The body is being lifted OUT of an `impl`, where `Self` is
        # legal, into a free function, where it is not — and the newtype rewrite keys on `<Agg>::`, so
        # any `Self::Variant { .. }` left behind is invisible to it too. Missing this produced 181
        # `E0433` plus a tail of E0532/E0559/E0308 across 11 aggregates in one pass.
        body=block[block.index("{",i):j+1].replace("self","this").replace("Self::",f"{agg}::")
        return body, block[i:j+1]
    try:
        dsig="fn diff(&self"; isig="fn inverse(&self"
        dbody,_=lift(dsig,"agg_diff"); ibody,_=lift(isig,"agg_inverse")
    except ValueError: return f"SKIP {root}: cannot lift diff/inverse"
    if dry: return f"OK {root}: {agg} {len(leaves)} leaves"
    used=set(); mapping={}
    for idx,(name,k,verb,fields) in enumerate(leaves):
        emoji=POOL[idx%len(POOL)]
        while emoji+k in used: emoji=POOL[(idx+len(used))%len(POOL)]
        used.add(emoji+k)
        if len(emoji)>2 or not (emoji+k).endswith(k) or len(emoji+k)>len(k)+2:
            raise SystemExit(f"refusing malformed leaf directory {emoji+k!r} — emoji must be one grapheme")
        d=os.path.join(root,emoji+k); os.makedirs(d,exist_ok=True)
        mapping[name]=(snake(name),name)
        io.open(os.path.join(d,"🦀️.rs"),"w",encoding="utf8").write(f'''//! {emoji}️ `{k}` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct {name} {{{fields}}}

impl protocol::MutationKind<{snap}, {agg}> for {name} {{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {{ verb: "{verb}", entity: "{'-'.join(k.split('-')[1:]) or 'document'}", kind: "{k}", record: "{name}" }};

    fn diff(&self, base: &{snap}) -> protocol::MutationOutcome<<{agg} as protocol::Mutation<{snap}>>::Diff> {{
        agg_diff(&{agg}::{name}(self.clone()), base)
    }}
    fn inverse(&self, base: &{snap}) -> Vec<{agg}> {{
        agg_inverse(&{agg}::{name}(self.clone()), base)
    }}
    fn label(&self) -> String {{
        "{k}".to_string()
    }}
    fn target(&self) -> Vec<String> {{
        Vec::new()
    }}
}}
//#endregion 🔖️Payload
''')
        io.open(os.path.join(d,"🔣️.json"),"w",encoding="utf8").write(json.dumps({
            "schemaVersion":1,"owner":f"{root}/{emoji+k}","semanticKind":k,
            "displayName":" ".join(w.capitalize() for w in k.split("-")),"emoji":emoji,
            "aggregateVariant":name,"payloadSchema":"🔣️payload.schema.json","textOpcode":None,"binaryTag":None,
            "invertibility":"explicit-mutation","diffParticipation":"detect","outcomeClasses":["applied"],
            "composition":"atomic","requiredLanguageSurfaces":["rust","json-schema"]},ensure_ascii=False,indent=2)+"\n")
    # ── rewrite the aggregate ──────────────────────────────────────────────────────────────────
    decls="\n".join(f'#[path = "{e}/🦀️.rs"]\npub mod {snake(n)};' for e,(n,_,_,_) in zip(sorted(used,key=lambda x:[l[1] for l in leaves].index(x[1:]) if x[1:] in [l[1] for l in leaves] else 0),leaves)) if False else \
          "\n".join(f'#[path = "{sorted(used)[i] if False else ""}"]' for i in [])
    decls="\n".join(f'#[path = "{d}/🦀️.rs"]\npub mod {snake(n)};' for d,(n,k,v,f) in zip([x for x in [e for e in used]],leaves)) if False else None
    # deterministic: recompute dir names the same way
    dirs=[]; used2=set()
    for idx,(name,k,verb,fields) in enumerate(leaves):
        emoji=POOL[idx%len(POOL)]
        while emoji+k in used2: emoji=POOL[(idx+len(used2))%len(POOL)]
        used2.add(emoji+k); dirs.append(emoji+k)
    decls="\n".join(f'#[path = "{d}/🦀️.rs"]\npub mod {snake(n[0])};' for d,n in zip(dirs,leaves))
    vars_="\n".join(f"    {n[0]}({snake(n[0])}::{n[0]})," for n in leaves)
    newenum=f'''//#region 🔖️Leaves
{decls}
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = {snap}, diff = {diffty}, schema = "{agg}")]
pub enum {agg} {{
{vars_}
}}'''
    ds=s.rindex("#[derive",0,m.start())
    s=s[:ds]+newenum+s[end+1:]
    ms=s.index(f"impl Mutation<{snap}> for {agg} {{"); me=brace_match(s,s.index("{",ms))
    s=s[:ms]+f'''// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &{agg}, base: &{snap}) -> protocol::MutationOutcome<{diffty}> {dbody}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &{agg}, base: &{snap}) -> Vec<{agg}> {ibody}'''+s[me+1:]
    s=newtype_rewrite(s,agg,mapping)
    s=re.sub(r"\n\s*"+agg+r"::NoMutation\s*=>[^,\n]*,","",s)
    s=s.replace(f"vec![{agg}::NoMutation]","Vec::new()")
    s=re.sub(r"\n\s*"+agg+r"::NoMutation,","",s)
    io.open(aggfile,"w",encoding="utf8").write(s)
    return f"MIGRATED {root}: {agg} {len(leaves)} leaves"

if __name__=="__main__":
    dry="--dry" in sys.argv
    for a in [x for x in sys.argv[1:] if not x.startswith("--")]:
        print(migrate(a,dry))


# ─────────────────────────────────────────────────────────────────────────────────────────────────
# 🧾️WHAT THIS SCRIPT DOES NOT DO — measured, in one 11-aggregate pass, at a cost of ~215 errors.
#
# The transformation itself is safe: the `diff`/`inverse` bodies move VERBATIM and no semantics are
# re-derived. What varies per artifact is the SURFACE the rewrite has to reach. Four shapes it missed,
# none of which the `📐️step/🔖️ap214` family happened to contain:
#
#   1. `Self::` — the bodies are lifted OUT of an `impl`, where `Self` is legal, into free functions,
#      where it is not; and a rewrite keyed on `<Agg>::` cannot see `Self::Variant { .. }` either.
#      Fixed above. Cost: 181 × E0433 + a tail of E0532/E0559/E0308.
#   2. UNIT variants become newtype over an EMPTY payload (`pub struct RemoveTileTags {}`). A bare
#      `Agg::Variant` in a PATTERN needs `(_)`; in a VALUE it needs `(mod::Ty {})`. Getting only the
#      first gives `error: in expressions, `_` can only be used on the left-hand side of an assignment`.
#      Cost: 16 × E0532, then 9 × that error.
#   3. Construction sites live OUTSIDE `🧪️tests` — `✏️editor/🦀️component.rs` had them. Sweep every
#      `.rs` in the artifact, not just the aggregate and its tests. Cost: 2 × E0559.
#   4. A leaf module is in scope INSIDE the aggregate only. Anywhere else it needs the full crate path,
#      which is best read off that file's own existing import of the aggregate rather than assumed.
#
# Run order that works: migrate → spell out `Self::` → newtype-rewrite → dropped-variant shapes →
# empty-payload value positions → qualify paths outside the aggregate → sweep for stale references →
# VALIDATE DESCRIPTORS → build. The descriptor validation is the cheap one; do it before the 10-minute
# build, not after.
