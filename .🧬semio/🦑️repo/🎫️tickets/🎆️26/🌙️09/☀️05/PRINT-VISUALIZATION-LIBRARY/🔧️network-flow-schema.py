#!/usr/bin/env python3
"""🕸️ Writes the NETWORK-FLOW family option vocabularies into 🧬️schema/🔣️.json.

The catalogue generator (🔧️network-flow-catalog.py) rewrote the entries of sections 8, 9, 13, 57,
58, 63, 64, 66, 67 and the network layouts of 75/78 onto the families this namespace actually
registers with \\SemioVizFamily. This script replaces the placeholder families the catalogue
generator had invented for those sections with the real ones, keeping only the families that leaves
outside this namespace still reference.

Run:  python 🔧️network-flow-schema.py
"""

import io
import json
import os
import sys

PRINT = os.path.join(
    'C:\\', 'git', 'semio',
    '\U0001f9f0\ufe0fframework', '\U0001f6cd\ufe0fproducts', '\U0001f4d3\ufe0fprint')
SCHEMA = os.path.join(PRINT, '\U0001f9ec\ufe0fschema', '\U0001f523\ufe0f.json')

OWNER = 'NETWORK-FLOW'

# 🗑️ Placeholder families this namespace no longer registers; every leaf of theirs moved to a real one.
DROP = [
    'database', 'flow', 'formal-language', 'graph-community', 'graph-dense', 'graph-edge',
    'graph-layout', 'graph-special', 'lineage', 'security', 'transform-network',
]


def opt(t, en, de, **extra):
    d = {'type': t, 'description': {'en': en, 'de': de}}
    d.update(extra)
    return d


VARIANT = opt(
    'string',
    'Sub-kind the family renders; the catalogue slug of the chart kind. Carried for diagnostics and '
    'for the gallery caption; the geometry is decided by the options below, never by this key alone.',
    'Untertyp, den die Familie zeichnet; der Katalog-Slug der Diagrammart. Wird für Diagnose und '
    'Galerie-Beschriftung mitgeführt; die Geometrie bestimmen die folgenden Optionen, nie dieser '
    'Schlüssel allein.')

BIND = {
    'data': opt('string', 'Name of the graph or flow table to read.',
                'Name der zu lesenden Graph- oder Flusstabelle.'),
    'nodes': opt('string', 'Node table, when the graph table does not carry the `nodes` role.',
                 'Knotentabelle, wenn die Graphtabelle die Rolle `nodes` nicht trägt.'),
    'id': opt('string', 'Node id column of the node table.', 'Id-Spalte der Knotentabelle.'),
    'group': opt('string', 'Node group column; drives colour, partite columns and the bundling '
                           'hierarchy.',
                 'Gruppenspalte der Knoten; steuert Farbe, partite Spalten und die '
                 'Bündelungshierarchie.'),
    'source': opt('string', 'Edge source column.', 'Quellspalte der Kanten.'),
    'target': opt('string', 'Edge target column.', 'Zielspalte der Kanten.'),
    'weight': opt('string', 'Edge weight column.', 'Gewichtsspalte der Kanten.'),
    'label': opt('string', 'Edge label column.', 'Beschriftungsspalte der Kanten.'),
}

COMMON = {
    'labels': opt('boolean', 'Draw the node captions.', 'Knotenbeschriftungen zeichnen.'),
    'size': opt('number', 'Node radius in millimetres.', 'Knotenradius in Millimetern.'),
    'linkWidth': opt('number', 'Base link width in millimetres.',
                     'Grundlinienbreite der Kanten in Millimetern.'),
    'padding': opt('number', 'Inset of the drawing rectangle on all four sides, in millimetres.',
                   'Innenabstand der Zeichenfläche auf allen vier Seiten in Millimetern.'),
    'colorBy': opt('string', 'Node hue: none, group, component or degree.',
                   'Knotenfarbe: none, group, component oder degree.',
                   enum=['none', 'group', 'component', 'degree']),
    'order': opt('string', 'Node ordering used by the closed-form layouts.',
                 'Knotenreihenfolge der geschlossenen Layouts.',
                 enum=['index', 'name', 'degree', 'group', 'component', 'cluster']),
}

GRAPH = dict(BIND)
GRAPH.update(COMMON)
GRAPH.update({
    'variant': VARIANT,
    'layout': opt('string',
                  'Layout algorithm the kernel runs before the marks are placed.',
                  'Layout-Algorithmus, den der Kernel vor dem Setzen der Marken ausführt.',
                  enum=['force', 'circular', 'shell', 'grid', 'spectral', 'layered', 'radial',
                        'arc', 'random', 'partite']),
    'iterations': opt('number',
                      'Force ticks; the cost is O(iterations * n^2) so raise it deliberately.',
                      'Kraftschritte; die Kosten sind O(iterations * n^2), also bewusst erhöhen.'),
    'seed': opt('number', 'Seed of the deterministic generator (d3-force uses 1).',
                'Startwert des deterministischen Generators (d3-force nutzt 1).'),
    'charge': opt('number', 'Many-body strength; d3-force\'s default is -30.',
                  'Stärke der Vielteilchenkraft; d3-force nutzt -30.'),
    'linkDistance': opt('number', 'Rest length of a link; d3-force\'s default is 30.',
                        'Ruhelänge einer Kante; d3-force nutzt 30.'),
    'forces': opt('string',
                  'Comma-separated forces in application order: link, many-body, center, collide, '
                  'x, y, radial.',
                  'Kommagetrennte Kräfte in Anwendungsreihenfolge: link, many-body, center, '
                  'collide, x, y, radial.'),
    'columns': opt('number', 'Grid columns; 0 picks ceil(sqrt n).',
                   'Gitterspalten; 0 wählt ceil(sqrt n).'),
    'root': opt('number', 'Root node index of the radial and BFS layouts.',
                'Wurzelknotenindex der radialen und BFS-Layouts.'),
    'sweeps': opt('number', 'Barycenter ordering passes of the layered layout.',
                  'Baryzentrum-Sortierdurchläufe des geschichteten Layouts.'),
    'orientation': opt('string', 'Direction the layers or the axis run in.',
                       'Richtung der Schichten oder der Achse.',
                       enum=['horizontal', 'vertical']),
    'shape': opt('string', 'Node glyph.', 'Knotenform.',
                 enum=['circle', 'square', 'diamond', 'accept', 'box']),
    'sizeBy': opt('string', 'Node radius encoding.', 'Kodierung des Knotenradius.',
                  enum=['none', 'degree', 'weight']),
    'directed': opt('boolean', 'Draw arrow heads.', 'Pfeilspitzen zeichnen.'),
    'curvature': opt('number', 'Base link bend; parallel edges fan out from it.',
                     'Grundkrümmung der Kanten; parallele Kanten fächern davon auf.'),
    'weightWidth': opt('boolean', 'Encode |weight| as the link width.',
                       '|Gewicht| als Kantenbreite kodieren.'),
    'signed': opt('boolean', 'Colour negative weights with the danger hue.',
                  'Negative Gewichte im Gefahrton einfärben.'),
})

MATRIX = dict(BIND)
MATRIX.update(COMMON)
MATRIX.update({
    'variant': VARIANT,
    'symmetric': opt('boolean', 'Mirror every edge into the transposed cell.',
                     'Jede Kante in die transponierte Zelle spiegeln.'),
    'scaleBy': opt('string', 'Cell shade from |weight| or from presence alone.',
                   'Zellton aus |Gewicht| oder allein aus dem Vorhandensein.',
                   enum=['weight', 'binary']),
    'diagonal': opt('boolean', 'Draw the diagonal cells.', 'Diagonalzellen zeichnen.'),
    'blocks': opt('boolean', 'Hairlines between consecutive ordering groups.',
                  'Haarlinien zwischen aufeinanderfolgenden Ordnungsgruppen.'),
    'cellGap': opt('number', 'Gap between two cells in millimetres.',
                   'Abstand zwischen zwei Zellen in Millimetern.'),
    'labelWidth': opt('number', 'Room reserved for the row and column captions, in millimetres.',
                      'Für Zeilen- und Spaltenbeschriftungen reservierte Breite in Millimetern.'),
})

ARC = dict(BIND)
ARC.update(COMMON)
ARC.update({
    'variant': VARIANT,
    'mode': opt('string', 'Straight axis with bows, or a ring with chords.',
                'Gerade Achse mit Bögen oder Ring mit Sehnen.', enum=['linear', 'circular']),
    'side': opt('string', 'Which half the bows of a linear diagram occupy.',
                'Welche Hälfte die Bögen eines linearen Diagramms einnehmen.',
                enum=['above', 'below', 'both']),
    'bow': opt('number', 'Bow height as a fraction of the span.',
               'Bogenhöhe als Anteil der Spannweite.'),
    'weightWidth': opt('boolean', 'Encode |weight| as the link width.',
                       '|Gewicht| als Kantenbreite kodieren.'),
    'directed': opt('boolean', 'Draw arrow heads.', 'Pfeilspitzen zeichnen.'),
    'signed': opt('boolean', 'Colour negative weights with the danger hue.',
                  'Negative Gewichte im Gefahrton einfärben.'),
})

HIVE = dict(ARC)
HIVE.update({
    'innerRadius': opt('number', 'Radius the axes start at, as a fraction of the outer radius.',
                       'Radius, an dem die Achsen beginnen, als Anteil des Außenradius.'),
    'outerRadius': opt('number', 'Radius the axes end at.', 'Radius, an dem die Achsen enden.'),
})

CHORD = dict(BIND)
CHORD.update(COMMON)
CHORD.update({
    'variant': VARIANT,
    'padAngle': opt('number', 'Gap between two group arcs, in radians (d3-chord padAngle).',
                    'Lücke zwischen zwei Gruppenbögen im Bogenmaß (d3-chord padAngle).'),
    'sortGroups': opt('string', 'Order of the group arcs by their total.',
                      'Reihenfolge der Gruppenbögen nach ihrer Summe.',
                      enum=['none', 'ascending', 'descending']),
    'sortSubgroups': opt('string', 'Order of the subgroup arcs inside a group.',
                         'Reihenfolge der Untergruppenbögen innerhalb einer Gruppe.',
                         enum=['none', 'ascending', 'descending']),
    'symmetric': opt('boolean', 'Mirror every edge into the transposed cell.',
                     'Jede Kante in die transponierte Zelle spiegeln.'),
    'innerRadius': opt('number', 'Inner edge of the group band, as a fraction of the radius.',
                       'Innenkante des Gruppenbands als Anteil des Radius.'),
    'ribbonInset': opt('number', 'Radius the ribbons start from.',
                       'Radius, an dem die Bänder beginnen.'),
    'tension': opt('number', 'How strongly a ribbon is pulled towards the centre.',
                   'Wie stark ein Band zur Mitte gezogen wird.'),
})

BUNDLE = dict(BIND)
BUNDLE.update(COMMON)
BUNDLE.update({
    'variant': VARIANT,
    'beta': opt('number',
                'Bundle tension: 1 follows the hierarchy, 0 is the straight chord (d3 curveBundle).',
                'Bündelspannung: 1 folgt der Hierarchie, 0 ist die gerade Sehne (d3 curveBundle).'),
    'hubRadius': opt('number', 'Radius of the group hubs, as a fraction of the leaf radius.',
                     'Radius der Gruppenknoten als Anteil des Blattradius.'),
    'samples': opt('number', 'Points sampled on the B-spline through the control polygon.',
                   'Abtastpunkte auf dem B-Spline durch das Kontrollpolygon.'),
    'weightWidth': opt('boolean', 'Encode |weight| as the link width.',
                       '|Gewicht| als Kantenbreite kodieren.'),
})

FLOW = dict(BIND)
FLOW.update(COMMON)
FLOW.update({
    'variant': VARIANT,
    'nodeWidth': opt('number', 'Width of a node rectangle in millimetres (d3-sankey nodeWidth).',
                     'Breite eines Knotenrechtecks in Millimetern (d3-sankey nodeWidth).'),
    'nodePadding': opt('number',
                       'Vertical gap between the nodes of one column (d3-sankey nodePadding).',
                       'Vertikale Lücke zwischen den Knoten einer Spalte (d3-sankey nodePadding).'),
    'nodeAlign': opt('string', 'Column assignment rule, exactly d3-sankey\'s four alignments.',
                     'Regel der Spaltenzuordnung, genau die vier Ausrichtungen von d3-sankey.',
                     enum=['left', 'right', 'center', 'justify']),
    'nodeSort': opt('string', 'auto keeps d3\'s "sort each column by y0"; none keeps input order.',
                    'auto behält d3s Sortierung jeder Spalte nach y0; none behält die Eingabefolge.',
                    enum=['auto', 'none']),
    'linkSort': opt('string', 'auto keeps d3\'s link ordering; none keeps input order.',
                    'auto behält d3s Kantenreihenfolge; none behält die Eingabefolge.',
                    enum=['auto', 'none']),
    'iterations': opt('number', 'Relaxation passes (d3-sankey iterations).',
                      'Relaxationsdurchläufe (d3-sankey iterations).'),
    'curvature': opt('number', 'Horizontal control-point fraction of a link ribbon.',
                     'Horizontaler Kontrollpunktanteil eines Kantenbands.'),
    'labelWidth': opt('number', 'Room reserved left and right for the captions, in millimetres.',
                      'Links und rechts für Beschriftungen reservierte Breite in Millimetern.'),
    'colorBy': opt('string', 'Ribbon hue: from the source node, the target node, the group or none.',
                   'Bandfarbe: vom Quellknoten, vom Zielknoten, von der Gruppe oder keine.',
                   enum=['source', 'target', 'group', 'none']),
})

STATE = dict(GRAPH)
STATE.update({
    'mode': opt('string', 'Which transition-system reading the glyphs and labels follow.',
                'Welcher Lesart eines Übergangssystems Formen und Beschriftungen folgen.',
                enum=['automaton', 'markov', 'petri', 'factor', 'causal', 'argument', 'attack']),
    'role': opt('string', 'Node column holding start, accept or normal.',
                'Knotenspalte mit start, accept oder normal.'),
    'edgeLabels': opt('boolean', 'Draw the transition captions.',
                      'Übergangsbeschriftungen zeichnen.'),
})

NEURAL = dict(COMMON)
NEURAL.update({
    'variant': VARIANT,
    'data': BIND['data'],
    'label': BIND['label'],
    'mode': opt('string', 'Architecture the layer table is read as.',
                'Architektur, als die die Schichttabelle gelesen wird.',
                enum=['feedforward', 'cnn', 'rnn', 'lstm', 'transformer', 'encoder-decoder',
                      'residual', 'gan', 'autoencoder', 'computational-graph', 'tensor',
                      'pipeline', 'training-loop', 'diffusion']),
    'render': opt('string', 'Unit circles per layer, or one labelled block per layer.',
                  'Einheitskreise je Schicht oder ein beschrifteter Block je Schicht.',
                  enum=['units', 'blocks']),
    'connections': opt('string', 'Pattern of the links between two consecutive layers.',
                       'Muster der Verbindungen zwischen zwei aufeinanderfolgenden Schichten.',
                       enum=['full', 'adjacent', 'residual', 'none']),
    'maxUnits': opt('number', 'Units drawn before a layer is elided.',
                    'Einheiten, die gezeichnet werden, bevor eine Schicht gekürzt wird.'),
    'unitSize': opt('number', 'Unit radius in millimetres.', 'Einheitsradius in Millimetern.'),
    'layerColumn': opt('string', 'Layer index column.', 'Spalte des Schichtindex.'),
    'unitsColumn': opt('string', 'Unit count column.', 'Spalte der Einheitenzahl.'),
    'kindColumn': opt('string', 'Layer kind column; drives the hue.',
                      'Spalte der Schichtart; steuert den Farbton.'),
})

COMMIT = dict(GRAPH)
COMMIT.update({
    'laneGap': opt('number', 'Distance between two lanes in millimetres.',
                   'Abstand zwischen zwei Spuren in Millimetern.'),
})

SCHEMAG = dict(GRAPH)
SCHEMAG.update({
    'mode': opt('string', 'Which database reading the layout and glyphs follow.',
                'Welcher Datenbank-Lesart Layout und Formen folgen.',
                enum=['er', 'schema', 'table-relationship', 'star', 'snowflake', 'query-plan',
                      'join']),
})

STRUCT = dict(COMMON)
STRUCT.update({
    'variant': VARIANT,
    'data': BIND['data'],
    'label': BIND['label'],
    'mode': opt('string', 'Container or tree the hierarchy rows are drawn as.',
                'Container oder Baum, als den die Hierarchiezeilen gezeichnet werden.',
                enum=['tree', 'bst', 'heap', 'trie', 'b-tree', 'red-black', 'suffix-tree',
                      'parse-tree', 'hash', 'linked-list', 'memory', 'stack', 'queue', 'trace']),
    'idColumn': opt('string', 'Node id column.', 'Spalte der Knoten-Id.'),
    'parentColumn': opt('string', 'Parent id column.', 'Spalte der Eltern-Id.'),
    'valueColumn': opt('string', 'Value column; drives the shade of the cell modes.',
                       'Wertspalte; steuert den Ton der Zellenmodi.'),
})

FAMILIES = {
    'graph': GRAPH,
    'state-machine': STATE,
    'neural-network': NEURAL,
    'commit-graph': COMMIT,
    'schema-graph': SCHEMAG,
    'data-structure': STRUCT,
    'adjacency-matrix': MATRIX,
    'node-link-matrix': MATRIX,
    'biofabric': MATRIX,
    'arc-diagram': ARC,
    'hive-plot': HIVE,
    'chord': CHORD,
    'dependency-wheel': CHORD,
    'edge-bundled': BUNDLE,
    'sankey': FLOW,
    'alluvial': FLOW,
    'parallel-sets': FLOW,
}


def main():
    doc = json.load(io.open(SCHEMA, encoding='utf-8'))
    opts = doc['x-semio-family-options']
    for name in DROP:
        if name in opts and opts[name].get('owner') == OWNER:
            del opts[name]
    for name, options in FAMILIES.items():
        opts[name] = {'owner': OWNER, 'options': options}
    doc['x-semio-family-options'] = dict(sorted(opts.items()))
    payload = (json.dumps(doc, ensure_ascii=False, indent=2) + '\n').encode('utf-8')
    tmp = SCHEMA + '.tmp'
    with open(tmp, 'wb') as fh:
        fh.write(payload)
    os.replace(tmp, SCHEMA)
    out = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    print('families declared: %d, placeholders dropped: %d' % (len(FAMILIES), len(DROP)), file=out)


if __name__ == '__main__':
    main()
