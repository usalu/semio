"""📦️ Accounts for every GIS-concrete line removed from the hub: found verbatim (after crate-alias normalization) in the GIS binding, or adapted."""
import re,sys
R="/Users/ueli/Documents/semio/"
B=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/relo-hubgis-baseline/tree/"
files=["🌎️hub/💡️inference/🏃️runtime/🌍️gis-map-binding/🦀️.rs","🌎️hub/💡️inference/🏃️runtime/🔐️committer/🦀️.rs","🌎️hub/💡️inference/🏃️runtime/🧱️document-preparation/🦀️.rs","🌎️hub/💡️inference/🏃️runtime/🏭️inference-runtime/🦀️.rs","🌎️hub/💡️inference/🏃️runtime/🧭️job-types-and-port/🦀️.rs","🌎️hub/💡️inference/🧾️wal/🦀️.rs","🌎️hub/💡️inference/📇️catalog/🦀️.rs"]
gis=open(R+"🌎️hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs",encoding='utf-8').read()
hubnow={f:open(R+f,encoding='utf-8').read() if not f.endswith("gis-map-binding/🦀️.rs") else "" for f in files}
binding=open(R+"🌎️hub/💡️inference/🏃️runtime/🔗️binding/🦀️.rs",encoding='utf-8').read()
def n(s):
    s=s.strip()
    s=s.replace("directory::os_store::","semio_framework_os_kernel::os_store::").replace("directory::pack_rt::","semio_framework_os_kernel::os_pack::pack_rt::")
    s=re.sub(r"InferenceRouteErrorV1::\w+|GisMapApprovalCommitErrorV1::\w+","E",s)
    s=re.sub(r"MapBindingRefusalV1::\w+","E",s)
    return s
G=set(n(l) for l in gis.splitlines())
H=set(n(l) for t in list(hubnow.values())+[binding] for l in t.splitlines())
tot=found_gis=found_hub=0; adapted=[]
for f in files:
    old=open(B+f,encoding='utf-8').read().splitlines()
    now=set(n(l) for l in hubnow[f].splitlines())
    for l in old:
        k=n(l)
        if len(k)<3 or k in now or k in ("{","}","};","});",")?;"): continue
        tot+=1
        if k in G: found_gis+=1
        elif k in H: found_hub+=1
        else: adapted.append((f.split("/")[-2],l.strip()[:150]))
print("removed non-trivial lines",tot,"verbatim in GIS binding",found_gis,"elsewhere in hub",found_hub,"adapted/dropped",len(adapted))
for a in adapted: print("  ADAPTED",a)
