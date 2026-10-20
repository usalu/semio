import json,re,collections
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G=C+"/🗑️generated/tst7/"
R=json.load(open(G+"classified.json",encoding="utf-8"))
OVR=[
("brep-intersect|brep-topology|brep-feature|geometry-service","needs-cargo-wasm-native","pinned meshes / fixtures are produced by Rust tests (tst3b: fixture data lost)"),
("capacity/🧪️tests/🔐️inheritance|capacity/🧪️tests/🪆️scope|🗿️artifact-source|native-build/🧪️tests|runtime/🧪️tests/🛬️decoder-absence|discovery/🕸️runtime","envelope-gated","artifact dir path budget (launcher provides a short in-repo dir)"),
("dashboard-control","envelope-gated","dashboard not installed (bun run dashboard:install)"),
("native-entrypoint/🧪️tests/📇️catalog","envelope-gated","live launcher catalog"),
("🪶️sqlite-snapshot|snapshot/🧪️tests","law-drift","sqlite snapshot receiving source moved (mount/needle)"),
]
REAL=[
("coordinator/🔮️oracle","import.meta in non-module (library, frozen)"),
("🧾️cargo-provenance","manifest.profile[name] undefined (library, frozen)"),
("storybook-discovery|owned-script-routes","admission.form undefined (library, frozen)"),
("cargo/🎯️exact/🧪️tests","test lost its imports/helpers in a split (runExactCargoLaws, originalInvocation, ...)"),
("cargo/🏃️execution/🧪️tests|build/🏃️execution/🧪️tests","runTestScript not defined (library, frozen)"),
("🏗️bazel/🧪️tests/📥️fingerprint","bazelTaskInputsV1 not defined (library, frozen)"),
("diff-only-law-gate","policyHostApplyFileBreaches export missing (library, frozen)"),
]
def owner(f):
    p=f.split("/")
    if p[0]=="✏️s": return "✏️s"
    if f.startswith("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"): return "repo/library"
    if f.startswith("🧰️framework/🛍️products/🦑️repo/🧪️tests"): return "repo/tests"
    if f.startswith("🧰️framework/🛍️products/🦑️repo"): return "repo/modules"
    if f.startswith("🧰️framework/🛍️products/💻️os"): return "framework/products/os"
    if f.startswith("🧰️framework/🛍️products"): return "framework/products/other"
    if f.startswith("🧰️framework/🔨️modules"): return "framework/modules"
    if p[0]=="🧰️framework": return "framework/other"
    if p[0]=="🌎️hub": return "hub"
    if p[0] in("🏢️semio-tech","♻️mit-bestand","🎓️teaching"): return p[0]
    return p[0]
def module(f):
    p=f.split("/")
    return p[2] if f.startswith("🧰️framework/🔨️modules") else None
out=[]
for r in R:
    f=r["file"];c=r["class"];note=""
    if c!="green":
        for pat,cl,n in OVR:
            if re.search(pat,f): c,note=cl,n;break
        else:
            for pat,n in REAL:
                if re.search(pat,f): c,note="real-defect",n;break
            else:
                if c=="unclassified": c="law-drift"
    r2=dict(file=f,rc=r["rc"],passed=r["pass_"],failed=r["fail"],secs=r["secs"],owner=owner(f),cls=c,note=note,firstError=(r.get("errs") or [""])[0][:200],fails=(r.get("fails") or [])[:3],log=r.get("log"))
    out.append(r2)
json.dump(dict(suites=out),open(G+"tst7-census-full.json","w",encoding="utf-8"),ensure_ascii=False)
def agg(rows):
    return dict(suites=len(rows),green=sum(x["rc"]==0 for x in rows),red=sum(x["rc"]!=0 for x in rows),testsPass=sum(x["passed"] for x in rows),testsFail=sum(x["failed"] for x in rows))
by_owner=collections.defaultdict(list);by_cls=collections.defaultdict(list);by_mod=collections.defaultdict(list)
for x in out:
    by_owner[x["owner"]].append(x)
    if x["rc"]!=0: by_cls[x["cls"]].append(x)
    if module(x["file"]): by_mod[module(x["file"])].append(x)
cens=dict(total=agg(out),perOwner={k:agg(v) for k,v in sorted(by_owner.items())},perFrameworkModule={k:agg(v) for k,v in sorted(by_mod.items())},perFailureClass={k:dict(suites=len(v),testsFail=sum(x["failed"] for x in v),owners=dict(collections.Counter(x["owner"] for x in v))) for k,v in sorted(by_cls.items())},
 reds=[dict(file=x["file"],owner=x["owner"],cls=x["cls"],pass_=x["passed"],fail=x["failed"],note=x["note"],firstError=x["firstError"]) for x in out if x["rc"]!=0])
json.dump(cens,open(G+"tst7-census.json","w",encoding="utf-8"),ensure_ascii=False,indent=1)
print(json.dumps(cens["total"]));
for k,v in cens["perOwner"].items():print(k,v)
for k,v in cens["perFailureClass"].items():print(k,v["suites"],v["testsFail"])
