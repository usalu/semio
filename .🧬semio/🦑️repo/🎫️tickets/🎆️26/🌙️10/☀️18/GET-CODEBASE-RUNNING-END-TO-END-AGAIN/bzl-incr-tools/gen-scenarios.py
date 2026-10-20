#!/usr/bin/env python3
"""Generates src/scenarios.json: authored texts, the effect table and the scenarios. Paths are resolved against the checkout ignoring U+FE0F so they can be typed plainly."""
import json,os,sys
R="/Users/ueli/Documents/semio"
strip=lambda s:s.replace("️","")
def real(path):
    cur=R;out=[]
    for part in path.split("/"):
        hit=[e for e in os.listdir(cur) if strip(e)==strip(part)]
        if len(hit)!=1: raise SystemExit(f"cannot resolve {part!r} under {cur}: {hit}")
        out.append(hit[0]);cur=os.path.join(cur,hit[0])
    return "/".join(out)
T=lambda en,de:{"en":en,"de":de}
labels={
 "scenario":T("scenario","Szenario"),"tool":T("tool","Werkzeug"),"phase":T("phase","Phase"),"must":T("must","muss"),"may":T("may","darf"),
 "executed":T("executed","ausgeführt"),"restored":T("restored","wiederhergestellt"),"cutoff":T("cutoff","abgeschnitten"),"verdict":T("verdict","Urteil"),"milliseconds":T("time (ms)","Zeit (ms)"),
 "stale":T("stale","veraltet"),"excess":T("excess","überzählig"),"pass":T("pass","bestanden"),"failed":T("failed","fehlgeschlagen"),"skipped":T("skipped","übersprungen"),
 "summary":T("summary","Zusammenfassung"),"area":T("area","Bereich"),"kind":T("edit","Änderung"),"target":T("target","Ziel"),"progress":T("ready","bereit"),
 "cancelled":T("cancelled","abgebrochen"),"title":T("Incremental build conformance","Konformität inkrementeller Builds"),"exact":T("exact","exakt"),"bounded":T("bounded","begrenzt"),
 "comparison":T("comparison of the implementations","Vergleich der Implementierungen"),"agree":T("same tasks","gleiche Tasks"),"differ":T("different tasks","verschiedene Tasks"),"only":T("only","nur"),
 "warm":T("warm run","Warmlauf"),"edit":T("edit","Änderung"),"revert":T("revert","Rücknahme"),
}
labels_schema_order=["scenario","tool","phase","must","may","executed","restored","cutoff","verdict","milliseconds","stale","excess","pass","failed","skipped","summary","area","kind","target","progress","cancelled","title","exact","bounded"]+["comparison","agree","differ","only"]
labels={k:labels[k] for k in labels_schema_order}|{k:labels[k] for k in ("warm","edit","revert")}
fam=["bun-script","tsc","vitest","bun-test","vite","cargo","wasm","aggregate"]
effects={
 "comment-only":{"tsc":"identical","vitest":"identical","bun-test":"identical","cargo":"identical","wasm":"identical","aggregate":"identical","bun-script":"unknown","vite":"unknown"},
 "body-change":{"tsc":"identical","vitest":"identical","bun-test":"identical","cargo":"unknown","wasm":"unknown","aggregate":"identical","bun-script":"unknown","vite":"unknown"},
 "signature-change":{"tsc":"changed","vitest":"identical","bun-test":"identical","cargo":"changed","wasm":"changed","aggregate":"identical","bun-script":"unknown","vite":"unknown"},
 "lockfile":{f:"identical" for f in fam},"config":{f:"identical" for f in fam},"unrelated-sibling":{f:"identical" for f in fam},
}
sens=[{"family":f,"kind":k,"effect":effects[k][f]} for k in effects for f in fam]
G=real("🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust")
P=real("🎓️teaching/🛂️proctor/📦️packages/🦀️rust")
S=real("🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust")
CO=real("🎓️teaching/🏛️architecture/📚️course")
CP=real("🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript")
D=real("♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime")
def f(d,name): return real(d+"/"+name)
def sc(id,en,de,area,target,kind,op,lang,file,tools=None):
    m={"op":op,"language":lang,"file":file}
    r={"id":id,"title":T(en,de),"area":area,"target":target,"kind":kind,"mutation":m}
    if tools: r["tools"]=tools
    return r
GR="@semio-tech/framework-graph:test"
scen=[
 sc("fw-ts-comment","Comment appended to a framework TypeScript generator","Kommentar am Ende eines TypeScript-Generators des Frameworks","framework-ts",GR,"comment-only","comment","ts",f(G,"📜️script.ts")),
 sc("fw-ts-body","Dead statement added to a framework TypeScript generator","Wirkungslose Anweisung in einem TypeScript-Generator des Frameworks","framework-ts",GR,"body-change","private-body","ts",f(G,"📜️script.ts")),
 sc("fw-ts-signature","Public export added to a framework TypeScript generator","Öffentlicher Export in einem TypeScript-Generator des Frameworks","framework-ts",GR,"signature-change","public-api","ts",f(G,"📜️script.ts")),
 sc("fw-ts-config","Whitespace in the project manifest of a framework library","Leerraum im Projektmanifest einer Framework-Bibliothek","framework-ts",GR,"config","whitespace","text",f(G,"📋️project.json")),
 sc("fw-ts-unrelated","Comment in a sibling project outside the closure","Kommentar in einem Geschwisterprojekt außerhalb des Abschlusses","framework-ts",GR,"unrelated-sibling","comment","ts",f(P,"📜️script.ts")),
 sc("rust-crate-comment","Comment appended to a Rust crate root","Kommentar am Ende einer Rust-Crate-Wurzel","rust-crate","@teaching/proctor:build","comment-only","comment","rs",f(P,"🦀️.rs")),
 sc("rust-crate-signature","Public constant added to a Rust crate root","Öffentliche Konstante in einer Rust-Crate-Wurzel","rust-crate","@teaching/proctor:test","signature-change","public-api","rs",f(P,"🦀️.rs")),
 sc("wasm-comment","Comment appended to a wasm component crate root","Kommentar am Ende einer Wasm-Komponenten-Crate","wasm-component","@semio-tech/framework-surface-rs:wasm","comment-only","comment","rs",f(S,"🦀️.rs")),
 sc("wasm-body","Private constant added to a wasm component crate root","Private Konstante in einer Wasm-Komponenten-Crate","wasm-component","@semio-tech/framework-surface-rs:wasm","body-change","private-body","rs",f(S,"🦀️.rs")),
 sc("vite-comment","Comment appended to a Vite builder module","Kommentar am Ende eines Vite-Builder-Moduls","vite-app","@teaching/architecture-course:build","comment-only","comment","ts",real(CO+"/🏗️builder/🌐️vite/🟦️.ts")),
 sc("vite-config","Whitespace in the TypeScript configuration of a Vite app","Leerraum in der TypeScript-Konfiguration einer Vite-App","vite-app","@teaching/architecture-course:build","config","whitespace","text",f(CP,"tsconfig.json")),
 sc("test-body","Dead statement added to a course module under test","Wirkungslose Anweisung in einem getesteten Kursmodul","test-target","@teaching/architecture-course:test","body-change","private-body","ts",real(CO+"/📚️course/🟦️.ts")),
 sc("test-comment","Comment appended to a course module under test","Kommentar am Ende eines getesteten Kursmoduls","test-target","@teaching/architecture-course:test","comment-only","comment","ts",real(CO+"/📚️course/🟦️.ts")),
 sc("lockfile-typecheck","Whitespace in the dependency lock before a type check","Leerraum in der Abhängigkeitssperre vor einer Typprüfung","test-target","@teaching/architecture-course:typecheck","lockfile","whitespace","text","bun.lock"),
 sc("activation-comment","Comment appended to a product activation script","Kommentar am Ende eines Aktivierungsskripts eines Produkts","product-activation","@semio-tech/mit-bestand-demonstrator:activate-prototype","comment-only","comment","ts",f(D,"📜️script.ts")),
 sc("activation-body","Dead statement added to a product activation script","Wirkungslose Anweisung in einem Aktivierungsskript eines Produkts","product-activation","@semio-tech/mit-bestand-demonstrator:activate-prototype","body-change","private-body","ts",f(D,"📜️script.ts")),
 sc("activation-unrelated","Comment in a sibling project outside the activation closure","Kommentar in einem Geschwisterprojekt außerhalb des Aktivierungsabschlusses","product-activation","@semio-tech/mit-bestand-demonstrator:activate-prototype","unrelated-sibling","comment","ts",f(P,"📜️script.ts")),
]
for s in scen:
    assert os.path.isfile(os.path.join(R,s["mutation"]["file"])),s["mutation"]["file"]
out={"version":1,"labels":labels,"sensitivity":sens,"scenarios":scen}
dst=sys.argv[1]
json.dump(out,open(dst,"w",encoding="utf8"),ensure_ascii=False,indent=1)
print("scenarios",len(scen),"sensitivity",len(sens))
