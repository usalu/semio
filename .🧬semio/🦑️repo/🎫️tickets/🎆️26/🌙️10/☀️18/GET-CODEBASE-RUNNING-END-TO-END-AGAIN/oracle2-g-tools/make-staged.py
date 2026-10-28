#!/usr/bin/env python3
"""📦️ usage: make-staged.py -> (re)builds staged/files (the new version of each changed read-oracle file) and staged/base (the live bytes it started from, verified against base-hashes.txt) from src/."""
import filecmp, os, shutil, hashlib
W = "/Users/ueli/Documents/semio"
H = os.path.dirname(os.path.abspath(__file__))
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle"
base_hash = {line.split()[1]: line.split()[0] for line in open(H + "/base-hashes.txt", encoding="utf-8") if line.strip()}
shutil.rmtree(H + "/staged", ignore_errors=True)
changed = 0
for d, _, names in os.walk(H + "/src/🔭️read-oracle"):
    for name in names:
        if name == ".DS_Store": continue
        src = os.path.join(d, name); sub = os.path.relpath(src, H + "/src/🔭️read-oracle"); live = f"{W}/{REL}/{sub}"
        if os.path.exists(live) and filecmp.cmp(src, live, shallow=False): continue
        key = "./" + sub
        if os.path.exists(live):
            assert hashlib.sha256(open(live, "rb").read()).hexdigest() == base_hash[key], "live file moved since the base: " + sub
            os.makedirs(os.path.dirname(f"{H}/staged/base/{REL}/{sub}"), exist_ok=True); shutil.copy2(live, f"{H}/staged/base/{REL}/{sub}")
        os.makedirs(os.path.dirname(f"{H}/staged/files/{REL}/{sub}"), exist_ok=True); shutil.copy2(src, f"{H}/staged/files/{REL}/{sub}")
        changed += 1
print("staged", changed, "files")

README = "🧰️framework/🛍️products/🦑️repo/README.md"
EN_OLD = "Repeated rounds learn every missing input; the oracle runs and their results are kept with the ticket."
EN_NEW = EN_OLD + " A huge declared list stays exact: the Seatbelt profile is split into several runs that each grant everything declared and refuse a share of the undeclared files (the union is as strict as the whole plan; `--max-profiles` bounds the runs), a plan that still does not fit hands over to the mirror and says so, directories are listed through one rule, and the unified log is read only after a marker denial proves it has flushed, so a loaded host cannot turn a missed denial into a clean run."
DE_OLD = "Wiederholte Runden lernen jeden fehlenden Input; die Orakel-Läufe und ihre Ergebnisse liegen beim Ticket."
DE_NEW = DE_OLD + " Eine riesige deklarierte Liste bleibt exakt: das Seatbelt-Profil wird auf mehrere Läufe verteilt, die je alles Deklarierte freigeben und einen Teil der nicht deklarierten Dateien verweigern (die Vereinigung ist so streng wie der ganze Plan; `--max-profiles` begrenzt die Läufe), ein Plan, der trotzdem nicht passt, geht mit Hinweis an den Spiegel über, Verzeichnisse werden über eine einzige Regel gelistet, und das Unified Log wird erst gelesen, wenn eine Marker-Verweigerung zeigt, dass es geschrieben wurde, sodass ein ausgelasteter Rechner eine verpasste Verweigerung nicht zu einem sauberen Lauf macht."
live = f"{W}/{README}"
text = open(live, encoding="utf-8").read()
assert text.count(EN_OLD) == 1 and text.count(DE_OLD) == 1
os.makedirs(os.path.dirname(f"{H}/staged/base/{README}"), exist_ok=True); shutil.copy2(live, f"{H}/staged/base/{README}")
os.makedirs(os.path.dirname(f"{H}/staged/files/{README}"), exist_ok=True)
open(f"{H}/staged/files/{README}", "w", encoding="utf-8").write(text.replace(EN_OLD, EN_NEW).replace(DE_OLD, DE_NEW))
print("README staged")
