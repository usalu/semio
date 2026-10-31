"""🧮 Derive every number of the tower narration from the model with f-strings."""
import os
import sys

root = "C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower"
path = os.path.join(root, next(n for n in os.listdir(root) if n.endswith(".py")))
text = open(path, "rb").read().decode("utf-8")


def sub(old, new):
    global text
    if text.count(old) != 1:
        sys.exit(f"not unique: {old[:60]}")
    text = text.replace(old, new)


sub('KBASE = np.array([VX, VY - 1.9, 0.0])\n', '''KBASE = np.array([VX, VY - 1.9, 0.0])
_A, _B = _nak.towers()
_PER = _nak.totals()["per_tower"]
_TOTAL = _nak.totals()["count"]


def _m(value, language):
    """📏 Metres with two decimals and the decimal sign of the language."""
    rendered = f"{value:.2f}"
    return rendered.replace(".", ",") if language == "de" else rendered
''')
sub('"The Nakagin Capsule Tower has 140 capsules. How do they become two towers?"', 'f"The Nakagin Capsule Tower has {_TOTAL} capsules. How do they become two towers?"')
sub('"Der Nakagin Capsule Tower hat 140 Kapseln. Wie werden sie zu zwei Türmen?"', 'f"Der Nakagin Capsule Tower hat {_TOTAL} Kapseln. Wie werden sie zu zwei Türmen?"')
sub('"A Series starts at the provisional 7.72 metre podium roof, with the sourced 2.80 metre pitch. Authored phase offsets place the capsules."',
    'f"A Series starts at the provisional {_m(_nak.podium_height(), \'en\')} metre podium roof, with the sourced {_m(_A.storey_height, \'en\')} metre pitch. Authored phase offsets place the capsules."')
sub('"Eine Series beginnt am vorläufigen Sockeldach bei 7,72 m mit belegten 2,80 m Schritt. Verfasste Phasenversätze platzieren die Kapseln."',
    'f"Eine Series beginnt am vorläufigen Sockeldach bei {_m(_nak.podium_height(), \'de\')} m mit belegten {_m(_A.storey_height, \'de\')} m Schritt. Verfasste Phasenversätze platzieren die Kapseln."')
sub('The documented result is 76 capsules in A and 64 in B."', 'The documented result is {_PER[\'A\'][\'count\']} capsules in A and {_PER[\'B\'][\'count\']} in B."')
sub('"Cull Index removes the', 'f"Cull Index removes the')
sub('"Cull Index entfernt die unbelegten Positionen auf jeder Ebene. Das belegte Ergebnis sind 76 Kapseln in A und 64 in B."',
    'f"Cull Index entfernt die unbelegten Positionen auf jeder Ebene. Das belegte Ergebnis sind {_PER[\'A\'][\'count\']} Kapseln in A und {_PER[\'B\'][\'count\']} in B."')
sub('"The machinery head is a separate part. A has a sourced height of 11.36 metres; B uses a provisional derived 10.83 metres."',
    'f"The machinery head is a separate part. A has a sourced height of {_m(_A.capital_height, \'en\')} metres; B uses a provisional derived {_m(_B.capital_height, \'en\')} metres."')
sub('"Der Maschinenkopf ist ein eigenes Bauteil. Für A sind 11,36 Meter belegt; B nutzt vorläufig abgeleitete 10,83 Meter."',
    'f"Der Maschinenkopf ist ein eigenes Bauteil. Für A sind {_m(_A.capital_height, \'de\')} Meter belegt; B nutzt vorläufig abgeleitete {_m(_B.capital_height, \'de\')} Meter."')
sub('"The head starts at the core roof: 42.14 m in A, a provisional 36.62 m in B; each tower keeps its own clearance."',
    'f"The head starts at the core roof: {_m(_A.height, \'en\')} m in A, a provisional {_m(_B.height, \'en\')} m in B; each tower keeps its own clearance."')
sub('"Der Kopf beginnt am Kerndach: 42,14 m in A, vorläufig 36,62 m in B; jeder Turm behält seinen Dachfreiraum."',
    'f"Der Kopf beginnt am Kerndach: {_m(_A.height, \'de\')} m in A, vorläufig {_m(_B.height, \'de\')} m in B; jeder Turm behält seinen Dachfreiraum."')
sub('"The same definition reads the authored floor masks for both towers: 76 capsules in A, 64 in B."',
    'f"The same definition reads the authored floor masks for both towers: {_PER[\'A\'][\'count\']} capsules in A, {_PER[\'B\'][\'count\']} in B."')
sub('"Dieselbe Definition liest die verfassten Geschossmasken beider Türme: 76 Kapseln in A, 64 in B."',
    'f"Dieselbe Definition liest die verfassten Geschossmasken beider Türme: {_PER[\'A\'][\'count\']} Kapseln in A, {_PER[\'B\'][\'count\']} in B."')
sub('"Together they give the historical count, 140 capsules, read from the definition and not typed in."',
    'f"Together they give the historical count, {_TOTAL} capsules, read from the definition and not typed in."')
sub('"Zusammen ergeben sie die historische Zahl von 140 Kapseln, aus der Definition gelesen und nicht eingetippt."',
    'f"Zusammen ergeben sie die historische Zahl von {_TOTAL} Kapseln, aus der Definition gelesen und nicht eingetippt."')
data = text.encode("utf-8")
with open(path, "r+b") as handle:
    handle.write(data)
    handle.truncate()
print("ok")
