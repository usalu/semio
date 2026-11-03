import sys
R="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/"
L="semio-framework-value = { workspace = true }\n"
for p in ("⏱️context/📦️packages/🦀️rust/Cargo.toml","📦️packages/🦀️rust/Cargo.toml"):
    f=R+p;s=open(f).read()
    d=s.index("[dev-dependencies]");dep=s.index("[dependencies]")
    assert dep<d and s[d:].count(L)==1 and s[:d].count(L)==0
    s=s[:d].rstrip("\n")+"\n"+L+"\n"+s[d:].replace(L,"",1)
    open(f,"w").write(s)
    print("patched",p)
