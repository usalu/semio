#!/usr/bin/env python3
"""🔁 Rebases the audit11 copy of `warmderive` on the CURRENT live tree: python3 -I rebuild.py <work root>   (a fresh mirror of live; nothing live is written)
Runs the lane's anchored edit scripts on the work root, adds audit11's anchored edits (option-keyed parse reuse, `warm.checkMilliseconds`), copies the new files of this batch, then rewrites files/, base/ (live bytes) and manifest.json. A moved anchor stops the rebuild."""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LANE = os.path.join(HERE, "..", "..", "..", "edit30-tools", "staged", "warmderive")
LIVE = HERE
while not os.path.isdir(os.path.join(LIVE, "🧰️framework")): LIVE = os.path.dirname(LIVE)
work = os.path.abspath(sys.argv[1])
manifest = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
for script in ("edit_si.py", "edit_memo.py"):
    if subprocess.run([sys.executable, "-I", os.path.join(LANE, script), work, "apply"]).returncode: sys.exit("lane anchor moved: " + script)
EDITS = {
 "00.mjs": [("if (reuse) compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile);", "if (reuse) compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, JSON.stringify(options));")],
 "03.ts": [("maximumMessageBytes:number;maximumResidentBytes:number}>;", "maximumMessageBytes:number;maximumResidentBytes:number;checkMilliseconds:number}>;")],
 "04.json": [('"derive": "The environment variable that makes a child process answer a derivation request instead of planning.",', '"derive": "The environment variable that makes a child process answer a derivation request instead of planning.",\n          "warm": "The warm deriver: the endpoint name prefix, the idle time, the connect and answer deadlines, the largest request line, the resident size limit and the closure check interval of the detached process that answers derivation requests.",'),
             ('"derive": "Die Umgebungsvariable, die ein Kindprozess dazu bringt, eine Ableitungsanfrage zu beantworten statt zu planen.",', '"derive": "Die Umgebungsvariable, die ein Kindprozess dazu bringt, eine Ableitungsanfrage zu beantworten statt zu planen.",\n          "warm": "Der warme Ableiter: das Präfix des Endpunktnamens, die Leerlaufzeit, die Verbindungs- und Antwortfristen, die längste Anfragezeile, die Grenze der residenten Größe und das Prüfintervall des Abschlusses des abgelösten Prozesses, der Ableitungsanfragen beantwortet.",'),
             ('"maximumResidentBytes": 2147483648}', '"maximumResidentBytes": 2147483648, "checkMilliseconds": 15000}'),
             ("and the resident size after which it stops after its answer (it keeps every program it built). DE:", "the resident size after which it stops after its answer (it keeps every program it built), and how often an idle deriver re-reads the size and modification time of every file of its captured closure and its own resident size: it stops at once when a file moved (its implementation was superseded, no run asks it any more) or it outgrew the resident size. DE:"),
             ("und die residente Größe, nach der er nach seiner Antwort aufhört (er behält jedes gebaute Programm).", "die residente Größe, nach der er nach seiner Antwort aufhört (er behält jedes gebaute Programm), und wie oft ein untätiger Ableiter Größe und Änderungszeit jeder Datei seines erfassten Abschlusses und seine eigene residente Größe neu liest: er hört sofort auf, wenn sich eine Datei bewegt hat (seine Implementierung wurde abgelöst, kein Lauf fragt ihn mehr) oder er über die residente Größe hinausgewachsen ist.")],
}
NEW = {"01.mjs", "02.mts", "06.ts", "07.ts", "08.ts", "09.json"}
staged = {}
for rel, row in manifest.items():
    path = os.path.join(work, rel)
    if row["file"] in NEW: data = open(os.path.join(HERE, "files", row["file"]), "rb").read()
    else:
        text = open(path, encoding="utf8").read()
        for old, new in EDITS.get(row["file"], []):
            if text.count(new) == 1: continue
            if text.count(old) != 1: sys.exit("audit11 anchor moved in %s: %r" % (rel[-50:], old[:50]))
            text = text.replace(old, new)
        data = text.encode("utf8")
    staged[rel] = data
for rel, row in manifest.items():
    live = os.path.join(LIVE, rel)
    base = open(live, "rb").read() if os.path.exists(live) else None
    open(os.path.join(HERE, "files", row["file"]), "wb").write(staged[rel])
    if base is None:
        if os.path.exists(os.path.join(HERE, "base", row["file"])): os.remove(os.path.join(HERE, "base", row["file"]))
    else: open(os.path.join(HERE, "base", row["file"]), "wb").write(base)
    row["base"] = None if base is None else hashlib.sha256(base).hexdigest(); row["new"] = hashlib.sha256(staged[rel]).hexdigest()
json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("rebuilt", len(manifest), "rows on live of now")
