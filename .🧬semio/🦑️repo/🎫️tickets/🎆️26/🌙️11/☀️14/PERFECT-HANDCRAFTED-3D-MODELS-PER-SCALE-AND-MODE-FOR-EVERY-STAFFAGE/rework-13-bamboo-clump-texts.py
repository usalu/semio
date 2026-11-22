"""📝 Rewrites the sentences of the volume decisions of vegetation/bamboo-clump that the stems of pass 1 made untrue (source file), and holds the same for the drawing decisions (imported by `rework-13-bamboo-clump-stems2d.py`).

Usage: python3 rework-13-bamboo-clump-texts.py
"""
import json

SOURCE = 'volumes/vegetation/bamboo-clump.json'
VOLUME = {
    '100': [
        ('branchlets are below the scale and omitted.', 'a branchlet alone is below the scale, so each spray carries its own as a 10 mm stem that leaves the branch at about 55 degrees: every spray grows from its branch and none hangs free.'),
        ('Zweiglein liegen unter dem Maßstab und entfallen.', 'ein Zweiglein allein liegt unter dem Maßstab, daher trägt jedes Büschel seines als 10-mm-Stiel, der den Ast unter etwa 55 Grad verlässt: Jedes Büschel wächst aus seinem Ast, keines hängt frei.'),
    ],
    '200': [
        ('is one drooping tuft hanging from the culm along the branch (100 tufts)', 'is one drooping tuft that starts at the node as a 30 mm branch and swells into the leaves along the branch (100 tufts, each grown onto its culm)'),
        ('ist ein hängender Schopf, der entlang des Asts vom Halm hängt (100 Schöpfe)', 'ist ein hängender Schopf, der am Knoten als 30-mm-Ast beginnt und entlang des Asts in das Laub übergeht (100 Schöpfe, jeder am Halm angewachsen)'),
    ],
    '500': [
        ('on each culm three foliage masses (lower, middle, top) that follow the branch tiers and lean with the culm', 'on each culm seven to ten foliage masses in three zones (lower, middle, top) and a top plume; each mass gathers two or three branch tiers, grows out of the culm on a 75 mm stem and leans with the culm'),
        ('barrier, bed and branches are below the scale.', 'barrier, bed and single branches are below the scale.'),
        ('an jedem Halm drei Laubmassen (unten, Mitte, oben), die den Astetagen folgen und sich mit dem Halm neigen', 'an jedem Halm sieben bis zehn Laubmassen in drei Zonen (unten, Mitte, oben) und ein Spitzenschopf; jede Masse fasst zwei oder drei Astetagen, wächst an einem 75-mm-Stiel aus dem Halm und neigt sich mit ihm'),
        ('Sperre, Beet und Äste liegen unter dem Maßstab.', 'Sperre, Beet und einzelne Äste liegen unter dem Maßstab.'),
    ],
}
DRAWING = {
    '100': [
        ('Omitted: branchlets and single leaves (below 15 mm).', 'Omitted: single leaves (below 15 mm); the branchlet of every spray is one stroke of the detail pen.'),
        ('Weggelassen: Zweiglein und einzelne Blätter (unter 15 mm).', 'Weggelassen: einzelne Blätter (unter 15 mm); das Zweiglein jedes Büschels ist ein Strich im Detailstift.'),
    ],
    '200': [
        ('Omitted: nodes and single branches (below 30 mm).', 'Omitted: nodes (below 30 mm); the branch of every tuft is one stroke of the detail pen from its node into the tuft.'),
        ('Weggelassen: Knoten und einzelne Äste (unter 30 mm).', 'Weggelassen: Knoten (unter 30 mm); der Ast jedes Schopfs ist ein Strich im Detailstift vom Knoten in den Schopf.'),
    ],
    '500': [
        ('six culms and three foliage masses on each, sunlit tops and shaded lower masses.', 'six culms with seven to ten foliage masses each in three zones, every mass on its stem from the culm, sunlit tops and shaded lower masses.'),
        ('sechs Halme mit je drei Laubmassen, besonnte Spitzen und beschattete untere Massen.', 'sechs Halme mit je sieben bis zehn Laubmassen in drei Zonen, jede Masse an ihrem Stiel aus dem Halm, besonnte Spitzen und beschattete untere Massen.'),
        ('Retained: every culm and its crown as three masses.', 'Retained: every culm, its crown as masses in three zones and the stem of every mass.'),
        ('Beibehalten: jeder Halm und seine Krone als drei Massen.', 'Beibehalten: jeder Halm, seine Krone als Massen in drei Zonen und der Stiel jeder Masse.'),
        ('Merged: the branch tiers of a culm into three masses.', 'Merged: two or three branch tiers of a culm into one mass.'),
        ('Zusammengefasst: die Astetagen eines Halms zu drei Massen.', 'Zusammengefasst: zwei oder drei Astetagen eines Halms zu einer Masse.'),
        ('Omitted: barrier, bed, nodes and branches (below 75 mm).', 'Omitted: barrier, bed, nodes and single branches (below 75 mm).'),
        ('Weggelassen: Sperre, Beet, Knoten und Äste (unter 75 mm).', 'Weggelassen: Sperre, Beet, Knoten und einzelne Äste (unter 75 mm).'),
    ],
}


def rewrite(value, pairs, seen):
    if isinstance(value, str):
        for old, new in pairs:
            if old in value:
                seen.add(old)
                value = value.replace(old, new)
        return value
    if isinstance(value, list):
        return [rewrite(item, pairs, seen) for item in value]
    if isinstance(value, dict):
        return {key: rewrite(item, pairs, seen) for key, item in value.items()}
    return value


def apply(decisions_of, table):
    for scale, pairs in table.items():
        seen = set()
        decisions_of[scale]['decisions'] = rewrite(decisions_of[scale]['decisions'], pairs, seen)
        missing = [old for old, new in pairs if old not in seen]
        if missing:
            raise SystemExit('not found at 1:%s: %s' % (scale, missing))


if __name__ == '__main__':
    volumes = json.load(open(SOURCE))['volumes']
    apply(volumes, VOLUME)
    open(SOURCE, 'w').write('{\n  "volumes": ' + json.dumps(volumes, ensure_ascii=False, separators=(',', ':')) + '\n}\n')
    print('volume decisions rewritten')
