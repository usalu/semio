"""🔩 Gives the cap ends of the stick and bucket cylinders and the rod ends of the boom cylinders something to hold on at 1:200 and 1:500 (lug blocks and the rod boss at their 1:100 size, all above the 75 mm feature) and writes the decisions of the structure rework."""
import json
P = 'volumes/machinery/excavator.json'
doc = json.load(open(P))
V = doc['volumes']
LUGS = {
    'stick-cyl-lug': {'kind': 'extrude', 'bone': 'boom', 'material': 'paint-steel', 'role': 'body', 'tone': 0.5, 'profile': [[2.74, 0.8], [3.26, 0.72], [3.07, 0.99], [2.93, 0.99]], 'plane': 'xy', 'from': -0.18, 'to': 0.02},
    'bucket-cyl-lug': {'kind': 'extrude', 'bone': 'stick', 'material': 'paint-steel', 'role': 'body', 'tone': 0.5, 'profile': [[0.02, 0.46], [0.5, 0.4], [0.32, 0.63], [0.18, 0.63]], 'plane': 'xy', 'from': -0.18, 'to': 0.02},
    'boom-rod-boss': {'kind': 'revolve', 'bone': 'boom', 'material': 'paint-steel', 'role': 'body', 'tone': 0.4, 'profile': [[0, 0], [0, 0.07], [0.88, 0.07], [0.88, 0]], 'axis': [0, 0, 1], 'origin': [2.45, 0.66, -0.52]},
}
for d in ('200', '500'):
    have = {x['id'] for x in V[d]['body']}
    for i, s in LUGS.items():
        if i in have:
            continue
        V[d]['body'].append({'id': i, **s})
        V[d]['thermal'].append({'id': 'th-' + i, 'of': i, 'region': 'attachment'})
        V[d]['structure'].append({'id': 'member-' + i, 'of': i, 'member': s['bone'], 'part': 'member'})
STRUCT = {
    '50': ("The load path is closed by hand: the two main girders stand under the boom foot brackets and run back into the counterweight carriers, two transverse bearers carry the engine across them, the slew bearing reaches from the carbody ring into the frame ring, the cab stands on a platform of two cross members and a sill beside the left girder and its ROPS is one frame of four posts, two cant rails and two roof bows; every rod carries its piston inside the hollow barrel, every barrel its cap eye on the pin, the pins sit in their lugs, and the lower grouser run takes the shoes down to the eight ground patches.",
           "Der Lastpfad ist von Hand geschlossen: Die zwei Hauptträger stehen unter den Auslegerfußblechen und laufen hinten in die Gegengewichtsträger, zwei Querlager tragen den Motor über beiden, das Drehkranzlager reicht vom Unterwagenring in den Rahmenring, die Kabine steht auf einer Plattform aus zwei Querträgern und einem Schweller neben dem linken Hauptträger, und ihr ROPS ist ein Rahmen aus vier Pfosten, zwei Dachholmen und zwei Dachspriegeln; jede Kolbenstange trägt ihren Kolben im hohlen Zylinderrohr, jedes Rohr sein Bodenauge auf dem Bolzen, die Bolzen sitzen in ihren Laschen, und das untere Stegband führt die Bodenplatten auf die acht Standflächen."),
    '100': ("The load path is closed by hand: main girders under the boom foot brackets and into the counterweight carriers, the engine bed on both girders, the cab platform (two cross members, one sill) under a closed ROPS frame with two roof bows, a piston on every rod inside its hollow barrel, the cylinder lugs and the rod boss as members, and the lower grouser run between shoes and ground patches.",
            "Der Lastpfad ist von Hand geschlossen: Hauptträger unter den Auslegerfußblechen und in die Gegengewichtsträger, das Motorlager auf beiden Trägern, die Kabinenplattform (zwei Querträger, ein Schweller) unter einem geschlossenen ROPS-Rahmen mit zwei Dachspriegeln, ein Kolben auf jeder Stange im hohlen Zylinderrohr, die Zylinderlaschen und das Stangenauge als Stäbe und das untere Stegband zwischen Bodenplatten und Standflächen."),
    '200': ("The load path is closed by hand: main girders under the boom foot brackets and into the counterweight carriers, the engine bed on both girders, the cab platform (two cross members, two sills) under a closed ROPS frame with two roof bows, the three cylinder lugs and the rod boss, the whole bucket (shell, cheeks, lip, ears, teeth), the idler hub in its yokes and the rollers between the two frame flanges.",
            "Der Lastpfad ist von Hand geschlossen: Hauptträger unter den Auslegerfußblechen und in die Gegengewichtsträger, das Motorlager auf beiden Trägern, die Kabinenplattform (zwei Querträger, zwei Schweller) unter einem geschlossenen ROPS-Rahmen mit zwei Dachspriegeln, die drei Zylinderlaschen und das Stangenauge, der ganze Löffel (Schale, Wangen, Schneide, Laschen, Zähne), die Leitradnabe in ihren Gabeln und die Laufrollen zwischen den zwei Rahmenflanschen."),
    '500': ("The load path is closed by hand: the two frame rails stand under the boom foot plates and run into the counterweight carriers, the engine bed lies on both, the cab platform (two cross members, two sills) carries a closed ROPS frame with two roof bows, the cylinder lugs and the rod boss hold the cylinder ends, and the tooth bar hangs on the bucket shell, cheeks and ears.",
            "Der Lastpfad ist von Hand geschlossen: Die zwei Rahmenholme stehen unter den Auslegerfußblechen und laufen in die Gegengewichtsträger, das Motorlager liegt auf beiden, die Kabinenplattform (zwei Querträger, zwei Schweller) trägt einen geschlossenen ROPS-Rahmen mit zwei Dachspriegeln, die Zylinderlaschen und das Stangenauge halten die Zylinderenden, und die Zahnleiste hängt an Löffelschale, Wangen und Laschen."),
}
BODY = {
    '200': ("The bucket cylinder has its lug block on the stick (as the stick cylinder has on the boom), so no barrel end hangs free; the track rollers reach from flange to flange of the track frame with their end collars.",
            "Der Löffelzylinder hat seinen Laschenblock auf dem Stiel (wie der Stielzylinder auf dem Ausleger), kein Rohrende hängt frei; die Laufrollen reichen mit ihren Endbunden von Flansch zu Flansch des Laufwerksrahmens."),
    '500': ("The two cylinder lug blocks (0.2 m wide, 0.14 to 0.23 m high) and the rod boss of the boom cylinders (0.14 m) are kept at this scale after all: without them the barrel and rod ends hang free beside boom and stick.",
            "Die zwei Zylinder-Laschenblöcke (0,2 m breit, 0,14 bis 0,23 m hoch) und das Stangenauge der Auslegerzylinder (0,14 m) bleiben in diesem Maßstab doch erhalten: Ohne sie hängen Rohr- und Stangenenden frei neben Ausleger und Stiel."),
}
for d, (en, de) in STRUCT.items():
    t = next(x for x in V[d]['decisions'] if x['en'].startswith('Structure'))
    if en not in t['en']:
        t['en'] += ' ' + en
        t['de'] += ' ' + de
for d, (en, de) in BODY.items():
    t = V[d]['decisions'][0]
    if en not in t['en']:
        t['en'] = t['en'].replace('grille plates, lug, stick fork', 'grille plates, stick fork') + ' ' + en
        t['de'] = t['de'].replace('Gitterbleche, Lasche, Stielgabel', 'Gitterbleche, Stielgabel') + ' ' + de
json.dump(doc, open(P, 'w'), ensure_ascii=False, indent=1)
print('[DEBUG] written')
