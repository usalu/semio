import re,sys,collections
def rows(p):
    out=[]
    for line in open(p,encoding="utf8"):
        m=re.match(r"\s{2}(testing/\w+)\s{2}(\S+)\s{2}(.*)",line.rstrip("\n"))
        if m: out.append(m.groups())
    return out
def cls(r):
    rule,path,detail=r
    pats=[("fixture-unresolved",r"^Fixture .* does not resolve"),("catalog-unclaimed",r"is claimed by no feature"),("no-runtime-inventory",r"No runtime inventory"),("binary-protocol-drift",r"protocol"),("oracle-requirement-unmet",r"requirement|qualif"),("mutation-without-fixture",r"no mutate scenario|without a fixture|has no vector|declares no vector"),("vocabulary-without-catalog",r"no catalog registers"),("catalog-defers",r"defers \d+ kind"),("test-source-depth",r"direct children of"),("vitest-wiring",r"import.meta.vitest"),("test-registration",r"Test registration is outside"),("self-test",r"Self-test declaration"),("fixture-dependency",r"resolves a dependency on fixture"),("oracle-capability",r"does not declare capability"),("unknown-oracle",r"unknown oracle|Unknown oracle"),("stub-serializer",r"serializer|stub")]
    for name,p in pats:
        if re.search(p,detail,re.I): return name
    return rule+":"+detail[:60]
a=collections.Counter(cls(r) for r in rows(sys.argv[1])); b=collections.Counter(cls(r) for r in rows(sys.argv[2]))
print("total",sum(a.values()),"->",sum(b.values()))
for k in sorted(set(a)|set(b), key=lambda k:-(a[k]+b[k])):
    print(f"{a[k]:6} -> {b[k]:6} {b[k]-a[k]:+6}  {k}")
