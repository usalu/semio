"""🔎️ Per test function: which upper-only (authority) identifiers it mentions."""
import re,sys
sys.path.insert(0,".")
from widen import strip_comments,REPO,RT
R=f"{REPO}/{RT}"
UPPER=["SurfaceReconcileJob","SurfaceReconcileReadyPatch","SurfaceReconcilePublishedPatch","SurfaceReconcilePublishedAck","SurfaceReconcileReservation","SurfaceReconcileTerminal","SurfaceReconcileRejected","SurfaceReconcileRetained","SURFACE_RECONCILE_HANDBACKS","reserve_surface_reconcile","register_surface_reconcile_backing","pending_surface_patch","SurfaceReconcileOutputs","take_ready","close_surface_reconcile_handback_one","drain_surface_reconcile_registry_until_idle","SurfaceReconcileHandback","reserve_surface_reconcile_handback","release_surface_reconcile","SurfaceDocumentProducer","take_surface_reconcile_terminal","surface_reconcile_registry_census","SURFACE_RECONCILE_FLOOR","SURFACE_RECONCILE_ADMISSION_SLOTS","SURFACE_RECONCILE_AGGREGATE","SurfaceReconcileOutput"]
files={"unit":"🧪️tests/🔬️reconcile-unit/🦀️.rs","own":"📏️ownership/🧪️tests/📏️ownership/🦀️.rs","doc":"📃️document/🧪️tests/📃️document/🦀️.rs","outt":"📤️output/🧪️tests/📤️output/🦀️.rs","tree":"♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs","patch":"🩹️patch/🧪️tests/🩹️patch/🦀️.rs","hand":"🚪️handback/🧪️tests/🚪️handback/🦀️.rs"}
tot={"cursor-side":0,"upper":0}
for k,p in files.items():
    src=open(f"{R}/{p}",encoding="utf-8").read().split("\n")
    # split into top-level fns (col 0 `fn`/`#[test]`)
    blocks=[];start=None;name=None
    for i,l in enumerate(src):
        m=re.match(r"^(?:#\[test\]\s*)?fn\s+([a-z_0-9A-Z]+)",l)
        if m:
            if name: blocks.append((name,start,i))
            name=m.group(1);start=i
        if l.startswith("//#"):
            if name: blocks.append((name,start,i)); name=None
    if name: blocks.append((name,start,len(src)))
    for name,s,e in blocks:
        body=strip_comments("\n".join(src[s:e]))
        hits=sorted({u for u in UPPER if u in body})
        side="UPPER" if hits else "cursor"
        lines=e-s
        tot["upper" if hits else "cursor-side"]+=lines
        print(f"{k:5} {side:6} {lines:4} {name[:70]:70} {','.join(h[:28] for h in hits[:4])}")
print(tot)
