"""📏️ Emits the lib and lib-test jobs of the job-context facade and its step custody core for real-closure-probe.mjs."""
import json
m="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/⏱️context"
out=[]
for crate,pkg in (("semio-framework-job-context-step",m+"/🧱️step/📦️packages/🦀️rust"),("semio-framework-job-context",m+"/📦️packages/🦀️rust")):
    for unit,test,feats in (("lib",False,[]),("lib-testkit",False,["testkit"]),("test",True,["testkit"])):
        out.append({"crate":crate,"unit":unit,"kind":"lib","test":test,"entry":pkg+"/🦀️.rs","manifestDirectory":pkg,"features":feats})
print(json.dumps(out,ensure_ascii=False))
