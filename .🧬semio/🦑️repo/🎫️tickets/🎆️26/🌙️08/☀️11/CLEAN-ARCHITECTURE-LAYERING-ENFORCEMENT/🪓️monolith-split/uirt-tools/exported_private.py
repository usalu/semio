"""🔎️ Private fields and methods of the facade-exported core types that the laws/facade test files mention by name."""
import re,sys
sys.path.insert(0,".")
from widen import strip_comments,REPO,RT
R=f"{REPO}/{RT}"
EXPORTED=["SurfaceReconciler","SurfaceReconcileJob","SurfaceReconcileOutputReservation","SurfaceReconcileOutputs","SurfaceReconcileOutputTransfer","SurfaceReconcilePublishedAck","SurfaceReconcilePublishedPatch","SurfaceReconcileReadyPatch","SurfaceReconcileRejected","SurfaceReconcileReservation","SurfaceReconcileTerminal"]
if __name__=="__main__":
    core_files=[f"{R}/♻️reconcile/🦀️.rs",f"{R}/📤️output/🦀️.rs"]
    tests=[f"{R}/🧪️tests/🔬️reconcile-unit/🦀️.rs",f"{R}/📏️ownership/🧪️tests/📏️ownership/🦀️.rs",f"{R}/📃️document/🧪️tests/📃️document/🦀️.rs",f"{R}/📤️output/🧪️tests/📤️output/🦀️.rs",f"{R}/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs",f"{R}/🧪️tests/🔬️present-unit/🦀️.rs",f"{R}/🔄️transaction/🦀️.rs",f"{R}/🧪️tests/🔬️transaction-unit/🦀️.rs"]
    ttext="\n".join(strip_comments(open(p,encoding="utf-8").read()) for p in tests)
    for cf in core_files:
        lines=open(cf,encoding="utf-8").read().split("\n")
        cur=None;kind=None
        for i,l in enumerate(lines):
            m=re.match(r"^(?:pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_]\w*)",l)
            if m: cur=(m.group(1),"struct"); continue
            m=re.match(r"^impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)\s*(?:<[^{]*>)?\s*\{",l)
            if m: cur=(m.group(1),"impl"); continue
            if l.startswith("}"): cur=None; continue
            if cur and cur[0] in EXPORTED and l.startswith("    ") and not l.startswith("     "):
                if cur[1]=="struct":
                    mm=re.match(r"^    (pub(?:\([^)]*\))?\s+)?([a-z_]\w*)\s*:",l)
                    if mm and not (mm.group(1) or "").startswith("pub ") and re.search(r"\."+mm.group(2)+r"(?![A-Za-z0-9_(])|\b"+mm.group(2)+r"\s*:",ttext):
                        print(f"FIELD  {cur[0]}.{mm.group(2)}  ({cf.split('runtime/')[1]}:{i+1})")
                else:
                    mm=re.match(r"^    (pub(?:\([^)]*\))?\s+)?(?:const\s+)?fn\s+([a-z_]\w*)",l)
                    if mm and not (mm.group(1) or "").startswith("pub ") and re.search(r"[.:]"+mm.group(2)+r"\s*\(",ttext):
                        print(f"METHOD {cur[0]}::{mm.group(2)}  ({cf.split('runtime/')[1]}:{i+1})  {l.strip()[:80]}")
