"""✂️ Splits the monolith's white-box suites along the component lines: cursor crate, authority crate and primitives."""
import re,sys
sys.path.insert(0,".")
from items import items
from widen import strip_comments
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/uirt/orig"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
UPPER_WORDS=["SurfaceReconcileJob","SurfaceReconcileReadyPatch","SurfaceReconcilePublishedPatch","SurfaceReconcilePublishedAck","SurfaceReconcileReservation","SurfaceReconcileTerminal","SurfaceReconcileRejected","SurfaceReconcileRetained","SURFACE_RECONCILE_HANDBACKS","reserve_surface_reconcile","register_surface_reconcile_backing","pending_surface_patch","SurfaceReconcileOutputs","take_ready","close_surface_reconcile_handback_one","drain_surface_reconcile_registry_until_idle","SurfaceReconcileHandback","reserve_surface_reconcile_handback","release_surface_reconcile","SurfaceDocumentProducer","take_surface_reconcile_terminal","surface_reconcile_registry_census","SURFACE_RECONCILE_ADMISSION_SLOTS","SURFACE_RECONCILE_AGGREGATE","SurfaceReconcileOutput"]
PRIM_TESTS={"retained_map_page_advances_each_key_once_without_rewalking_prior_entries","allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation"}
def parse(rel):
    from items import items as it
    import items as im
    lines=open(f"{S}/{RT}/{rel}",encoding="utf-8").read().split("\n")
    # reuse the block parser on a text file
    im_lines,full=im.items(f"{S}/{RT}/{rel}")
    blocks=[]
    for k,s,e,t in full:
        txt="\n".join(im_lines[s-1:e])
        name=None;kindw=None
        if k=="item":
            m=re.search(r"\b(fn|const|static|struct|enum)\s+([A-Za-z_0-9]+)",t)
            mi=re.match(r"impl(?:<[^>]*>)?\s+(?:[A-Za-z_:<>]+\s+for\s+)?([A-Za-z_0-9]+)",t)
            if t.startswith("impl") and mi: name="impl:"+mi.group(1); kindw="impl"
            elif m and not t.startswith(("use ","mod ","#[cfg(test)]")): name=m.group(2); kindw=m.group(1)
        blocks.append(dict(kind=k,start=s,end=e,text=txt,name=name,head=t,kw=kindw))
    return im_lines,blocks
def classify(blocks):
    helpers={b["name"]:b for b in blocks if b["name"] and "#[test]" not in b["text"].split("\n")[0:3].__str__()}
    side={}
    for b in blocks:
        if b["kind"]!="item" or not b["name"]: continue
        body=strip_comments(b["text"])
        up=any(re.search(r"(?<![A-Za-z0-9_])"+re.escape(w),body) for w in UPPER_WORDS)
        side[b["name"]]="UP" if up else "CUR"
    changed=True
    while changed:
        changed=False
        for b in blocks:
            if b["kind"]!="item" or not b["name"]: continue
            if side[b["name"]]=="UP": continue
            body=strip_comments(b["text"])
            for n,s in side.items():
                if s=="UP" and n!=b["name"] and re.search(r"(?<![A-Za-z0-9_])"+re.escape(n)+r"\(",body):
                    side[b["name"]]="UP"; changed=True; break
    for n in PRIM_TESTS: 
        if n in side: side[n]="PRIM"
    return side
if __name__=="__main__":
    for rel in ["🧪️tests/🔬️reconcile-unit/🦀️.rs","📏️ownership/🧪️tests/📏️ownership/🦀️.rs","📃️document/🧪️tests/📃️document/🦀️.rs","📤️output/🧪️tests/📤️output/🦀️.rs","♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs"]:
        lines,blocks=parse(rel); side=classify(blocks)
        print("==",rel.split("/")[-2])
        tot={}
        for b in blocks:
            if b["name"] in side:
                n=b["end"]-b["start"]+1
                tot[side[b["name"]]]=tot.get(side[b["name"]],0)+n
                print(f"  {side[b['name']]:4} {n:4} {b['name'][:80]}")
        print("  totals",tot)
