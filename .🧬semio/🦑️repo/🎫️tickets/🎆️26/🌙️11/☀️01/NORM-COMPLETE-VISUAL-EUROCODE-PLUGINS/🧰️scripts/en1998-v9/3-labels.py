"""en1998 v9 step 3: label text of the retyped fields (liquefiable layer, wall storey id, member kind) across the artifact and the leaf taxonomy. python -I"""
import os
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ROOTS = [NORM + "/🗿️artifacts/🫨️en1998", NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1"]
PAIRS = [
 ("Thickness of liquefiable or sensitive soil deposits (S2 indicator, 0 = none)", "Liquefiable or sensitive soil deposits (ground type S2)"),
 ("Mächtigkeit verflüssigungsfähiger oder empfindlicher Bodenablagerungen (S2-Kennzeichen, 0 = keine)", "Verflüssigungsfähige oder empfindliche Böden (Baugrundklasse S2)"),
 ("Storey number of the wall, counted from 1 at the base (0 = every storey)", "Storey of the wall (empty = every storey)"),
 ("Geschossnummer der Wand, ab 1 am Fußpunkt gezählt (0 = jedes Geschoss)", "Geschoss der Wand (leer = jedes Geschoss)"),
 ("Change Member role", "Change Member kind"), ("Bauteilrolle ändern", "Bauteilart ändern"),
 ('"Member role", "Bauteilrolle"', '"Member kind", "Bauteilart"'),
]
counts = {a: 0 for a, _ in PAIRS}
for root in ROOTS:
    for dp, dn, fs in os.walk(root):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fs:
            p = os.path.join(dp, f)
            try: t = open(p, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError): continue
            n = t
            for a, b in PAIRS:
                c = n.count(a)
                if c: counts[a] += c; n = n.replace(a, b)
            if n != t: open(p, "w", encoding="utf-8").write(n)
for a, c in counts.items(): print(c, a[:70])
