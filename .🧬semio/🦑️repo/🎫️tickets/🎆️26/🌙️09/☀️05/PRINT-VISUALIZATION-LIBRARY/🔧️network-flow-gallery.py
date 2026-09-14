#!/usr/bin/env python3
"""🖼️ Builds a gallery document of every NETWORK-FLOW catalogue kind and checks it renders distinctly.

The generated catalogue gallery (`🧾️template/📊️viz-gallery`) loads `semio-viz.sty`, which pulls in
every namespace of the library, so it cannot be built while another namespace has a broken package.
This builds the same thing for the sections this agent owns: it reads the kinds out of
`🖼️assets/🔣️viz-catalog.json`, renders each one through `\\SemioVizRunFamily` with the options the
catalogue records, and turns the probe stream into one geometry hash per kind, so "renders" and
"renders differently from its neighbours" are both measured rather than eyeballed.

Run:  python 🔧️network-flow-gallery.py [out-dir]
"""

import hashlib
import io
import json
import os
import subprocess
import sys

TICKET = os.path.dirname(os.path.abspath(__file__))
PRINT = os.path.join(
    'C:\\', 'git', 'semio',
    '\U0001f9f0\ufe0fframework', '\U0001f6cd\ufe0fproducts', '\U0001f4d3\ufe0fprint')
LATEX = os.path.join(PRINT, '\U0001f58b\ufe0flatex')
CATALOG = os.path.join(PRINT, '\U0001f5bc\ufe0fassets', '\U0001f523\ufe0fviz-catalog.json')

FAMILIES = {
    'graph': 'semio-viz-network-graph',
    'state-machine': 'semio-viz-network-graph',
    'neural-network': 'semio-viz-network-graph',
    'commit-graph': 'semio-viz-network-graph',
    'schema-graph': 'semio-viz-network-graph',
    'data-structure': 'semio-viz-network-graph',
    'adjacency-matrix': 'semio-viz-network-matrix',
    'node-link-matrix': 'semio-viz-network-matrix',
    'biofabric': 'semio-viz-network-matrix',
    'arc-diagram': 'semio-viz-network-arc',
    'hive-plot': 'semio-viz-network-arc',
    'chord': 'semio-viz-network-chord',
    'dependency-wheel': 'semio-viz-network-chord',
    'edge-bundled': 'semio-viz-network-bundling',
    'sankey': 'semio-viz-flow-sankey',
    'alluvial': 'semio-viz-flow-alluvial',
    'parallel-sets': 'semio-viz-flow-parallelsets',
}

PACKAGES = [
    'semio-viz-network-matrix', 'semio-viz-network-arc', 'semio-viz-network-chord',
    'semio-viz-network-bundling', 'semio-viz-flow-sankey', 'semio-viz-flow-alluvial',
    'semio-viz-flow-parallelsets',
]


def option_string(entry):
    parts = ['data=' + entry['data']]
    for key, value in entry['options'].items():
        if isinstance(value, bool):
            parts.append('%s=%s' % (key, 'true' if value else 'false'))
        else:
            parts.append('%s={%s}' % (key, value))
    return ','.join(parts)


def kinds():
    doc = json.load(io.open(CATALOG, encoding='utf-8'))
    return [k for k in doc['kinds'] if k['family'] in FAMILIES]


def document(entries):
    lines = [
        r'\documentclass[a4paper]{article}',
        r'\usepackage[margin=8mm]{geometry}',
        r'\usepackage{tikz}',
    ]
    for name in PACKAGES:
        lines.append(r'\usepackage{%s}' % name)
    lines += [
        r'\begin{document}',
        r'\ExplSyntaxOn',
        r'\SemioVizProbeOn',
        r'\bool_set_true:N \l_semio_viz_in_figure_bool',
        r'\fp_new:N \l_semio_viz_width_fp \fp_set:Nn \l_semio_viz_width_fp { 84 }',
        r'\fp_new:N \l_semio_viz_height_fp \fp_set:Nn \l_semio_viz_height_fp { 50 }',
        r'\ExplSyntaxOff',
        r'\pagestyle{empty}\footnotesize',
    ]
    for index, entry in enumerate(entries):
        lines.append(r'\ExplSyntaxOn\SemioVizProbeBegin{network-flow-gallery}{%s}\ExplSyntaxOff'
                     % entry['slug'])
        lines.append(r'\noindent\texttt{%s}\par\nobreak' % entry['slug'].replace('-', '-'))
        lines.append(r'\begin{tikzpicture}[x=1mm,y=1mm]')
        lines.append(r'\SemioVizRunFamily{%s}[%s]' % (entry['family'], option_string(entry)))
        lines.append(r'\end{tikzpicture}\par\medskip')
        if index % 2 == 1:
            lines.append(r'\par')
    lines.append(r'\end{document}')
    return '\n'.join(lines) + '\n'


def main():
    out_dir = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        TICKET, '\U0001f5d1\ufe0fgenerated', 'NETWORK-FLOW', 'gallery')
    os.makedirs(out_dir, exist_ok=True)
    entries = kinds()
    tex = os.path.join(out_dir, 'gallery.tex')
    with open(tex, 'wb') as fh:
        fh.write(document(entries).encode('utf-8'))
    stale = os.path.join(out_dir, 'gallery.probe.jsonl')
    if os.path.exists(stale):
        os.remove(stale)
    env = dict(os.environ)
    env['TEXINPUTS'] = LATEX + ';'
    proc = subprocess.run(
        ['xelatex', '-interaction=nonstopmode', '-halt-on-error', 'gallery.tex'],
        cwd=out_dir, env=env, capture_output=True)
    out = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    text = proc.stdout.decode('utf-8', 'replace')
    if proc.returncode != 0:
        for line in text.splitlines():
            if line.startswith('!') or line.startswith('l.'):
                print(line, file=out)
        print('kinds: %d — COMPILE FAILED' % len(entries), file=out)
        return 1
    stream = os.path.join(out_dir, 'gallery.probe.jsonl')
    seen = {}
    empty = []
    collisions = []
    records = {}
    for line in io.open(stream, encoding='utf-8'):
        record = json.loads(line)
        records.setdefault(record['scenario'], []).append((record['key'], record['values']))
    for entry in entries:
        payload = records.get(entry['slug'])
        if not payload:
            empty.append(entry['slug'])
            continue
        digest = hashlib.sha1(json.dumps(payload).encode()).hexdigest()[:12]
        if digest in seen:
            collisions.append('%s == %s' % (entry['slug'], seen[digest]))
        seen[digest] = entry['slug']
    print('kinds: %d, rendered: %d, distinct projections: %d'
          % (len(entries), len(entries) - len(empty), len(seen)), file=out)
    if empty:
        print('drew nothing: %s' % ', '.join(empty), file=out)
    if collisions:
        print('identical projections: %s' % '; '.join(collisions), file=out)
    return 1 if empty or collisions else 0


if __name__ == '__main__':
    sys.exit(main())
