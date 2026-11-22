"""📝 Pigeon: states the feather-tract wing in the decisions of every scale (rework author 08)."""
import json

SOURCE = 'volumes/animals/pigeon.json'
NOTE = {
    'en': ' The wing is built from its feather tracts: ten primaries on the hand and ten secondaries on the forearm, each a lens-shaped vane on its own bone as thick as the 7.5 mm feature, the hand with its coverts as one small mass, the forearm coverts ending behind the elbow and the scapulars lying on the trunk. Folded, the remiges stack along the flank under the coverts with the tips over the tail; in every opened pose they fan out into the wing surface. The same feathers stand at every scale, so the wing folds and spreads alike.',
    'de': ' Der Flügel besteht aus seinen Federfluren: zehn Handschwingen an der Hand und zehn Armschwingen am Unterarm, jede als linsenförmige Fahne an einem eigenen Knochen in der Stärke des 7,5-mm-Merkmals, die Hand mit ihren Decken als eine kleine Masse, die Armdecken enden hinter dem Ellbogen und die Schulterfedern liegen am Rumpf. Gefaltet stapeln sich die Schwingen entlang der Flanke unter den Decken mit den Spitzen über dem Schwanz; in jeder geöffneten Pose fächern sie zur Flügelfläche auf. Dieselben Federn stehen in jedem Maßstab, sodass der Flügel überall gleich faltet und spreizt.',
}
RACHIS = {
    'en': ' Each remex carries one rachis bar from its base to its tip as its structural member.',
    'de': ' Jede Schwinge trägt einen Schaftstab von der Basis bis zur Spitze als Tragglied.',
}
doc = json.load(open(SOURCE))
for v in doc['volumes'].values():
    for index, note in ((0, NOTE), (2, RACHIS)):
        target = v['decisions'][min(index, len(v['decisions']) - 1)]
        for lang in ('en', 'de'):
            if note[lang] not in target[lang]:
                target[lang] += note[lang]
json.dump(doc, open(SOURCE, 'w'), ensure_ascii=False)
