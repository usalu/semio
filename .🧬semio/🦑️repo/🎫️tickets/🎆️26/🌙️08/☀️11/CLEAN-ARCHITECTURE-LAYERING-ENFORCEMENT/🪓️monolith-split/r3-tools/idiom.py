# Converts the testkit idiom produced by r4-testkit.ts to the canonical one: owner `#[cfg(any(test, feature = "testkit"))]`, no self dev-dependency.
import os,re,sys
P="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
for s in sys.argv[1:]:
    for dp,dn,fn in os.walk(f"{P}/{s}"):
        dn[:]=[d for d in dn if d not in("node_modules","target")]
        for f in fn:
            p=os.path.join(dp,f)
            if f.endswith(".rs"):
                t=open(p,encoding='utf8').read()
                if '#[cfg(feature = "testkit")]' in t: open(p,'w',encoding='utf8').write(t.replace('#[cfg(feature = "testkit")]','#[cfg(any(test, feature = "testkit"))]')); print("rs",p[-40:])
            elif f=="Cargo.toml":
                t=open(p,encoding='utf8').read()
                n=re.sub(r'^(semio-s-artifact-stdio-semio-[\w-]+) = \{ path = "\.", features = \["testkit"\] \}\n','',t,flags=re.M)
                n=re.sub(r'\n\[dev-dependencies\]\n(?=\n|\Z)','\n',n)
                if n!=t: open(p,'w',encoding='utf8').write(n); print("toml",p[-50:])
