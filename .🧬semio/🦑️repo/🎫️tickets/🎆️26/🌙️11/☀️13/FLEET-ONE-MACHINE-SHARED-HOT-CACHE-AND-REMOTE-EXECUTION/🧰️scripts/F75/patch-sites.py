import os,sys
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
TARGET="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts"
R=lambda en,de:'{en:"%s",de:"%s"}'%(en,de)
S=[
(L+"🏗️builder/🟦️.ts",[("throw new Error(`Authoring target identity changed: ${path}`)","throw descriptorRefusalV1(\"immutable\",path,"+R("Authoring target","Schreibziel")+")")]),
(L+"⚡️caching/🌐️vite/📥️inputs/📸️capture/🟦️.ts",[('throw Error("Private Vite artifact descriptor changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Private Vite artifact","Privates Vite-Artefakt")+")")]),
(L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📐️layout/🟦️.ts",[('throw Error("Original cache charge receipt changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Cache charge receipt","Cache-Belastungsbeleg")+")")]),
(L+"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/📸️observation/🟦️.ts",[('throw Error("Pinned dependency receipt changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Pinned dependency receipt","Angeheftter Abhängigkeitsbeleg")+")"),('throw Error("Exact dependency receipt observation changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Dependency receipt","Abhängigkeitsbeleg")+")")]),
(L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts",[('throw Error("Original authorizer body changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Authorizer module","Autorisierungsmodul")+")")]),
(L+"🏃️process/🦀️cargo/🏃️execution/🟦️.ts",[('throw Error("Original Cargo execution Source file changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Cargo execution Source","Cargo-Ausführungsquelle")+")")]),
(L+"🏗️build/👁️observation/🟨️.mjs",[('throw Error("Observation descriptor identity changed")',"throw descriptorRefusalV1(\"immutable\",path,"+R("Observation leaf","Beobachtungsblatt")+")")]),
]
for F,reps in S:
    t=open(F,encoding="utf8").read()
    rel=os.path.relpath(TARGET,os.path.dirname(F)).replace(os.sep,"/")
    if not rel.startswith("."):rel="./"+rel
    for a,b in reps:
        assert t.count(a)>=1,(F[-40:],a)
        t=t.replace(a,b)
    imp='import{descriptorRefusalV1}from"%s";\n'%rel
    if imp not in t:t=imp+t
    tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F);print("ok",F[-50:])
