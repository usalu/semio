"""👁️ Retain official ADD2 view scope as owned declaration data, without asserting conformance."""
import hashlib
import json
import pathlib
import urllib.request
from html.parser import HTMLParser

ROOT = pathlib.Path(__file__).resolve().parents[7]
TARGET = ROOT / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd'
COMMIT = '610ee3f95db9d0380b1eb8de4dc6e17833531518'
BASE = f'https://raw.githubusercontent.com/buildingSMART/Standards-builds/{COMMIT}/IFC/RELEASE/IFC4_3/HTML/'


class ScopeParser(HTMLParser):
    """🧾 Extract direct wrapper links and their own scope badges, excluding nested child rows."""
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack = []
        self.wrappers = []
        self.rows = []
        self.capture = None

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag in ('meta', 'link', 'img', 'br', 'hr', 'input', 'source', 'wbr'):
            return
        self.stack.append(tag)
        if tag == 'div' and attrs.get('class') == 'wrapper':
            self.wrappers.append({'depth': len(self.stack), 'href': None, 'name': '', 'badges': []})
        if not self.wrappers:
            return
        row = self.wrappers[-1]
        if tag == 'a' and row['href'] is None:
            row['href'] = attrs.get('href')
            self.capture = (len(self.stack), 'name', row)
        if tag == 'abbr':
            badge = {'label': '', 'title': attrs.get('title', '')}
            row['badges'].append(badge)
            self.capture = (len(self.stack), 'label', badge)

    def handle_endtag(self, tag):
        if not self.stack or tag not in self.stack:
            return
        depth = len(self.stack) - self.stack[::-1].index(tag)
        if self.capture and self.capture[0] >= depth:
            self.capture = None
        if self.wrappers and self.wrappers[-1]['depth'] >= depth:
            self.rows.append(self.wrappers.pop())
        self.stack = self.stack[:depth - 1]

    def handle_data(self, value):
        if self.capture:
            _, key, item = self.capture
            item[key] += value


def read_scope(path):
    payload = urllib.request.urlopen(BASE + path).read()
    parser = ScopeParser()
    parser.feed(payload.decode('utf-8'))
    return parser.rows, {'url': BASE + path, 'sha256': hashlib.sha256(payload).hexdigest()}


def normalized(row, entity=False):
    views = {}
    universal = False
    for badge in row['badges']:
        label = badge['label'].strip()
        if label.startswith('product'):
            universal = True
        view = 'ReferenceView' if label.startswith('RV') else 'Alignment-basedReferenceView' if label.startswith('AbRV') else None
        if view:
            title = badge['title'].removeprefix('Included in ')
            views[view] = [item.strip() for item in title.split(',')] if entity else []
    name = ' '.join(row['name'].split())
    return {'id': name.upper() if entity else row['href'].removesuffix('/content.html').removesuffix('content.html').rstrip('/'), 'name': name, 'href': row['href'], 'universal': universal, 'views': views}


def main():
    entities, entity_source = read_scope('annex-b1.html')
    concepts, concept_source = read_scope('concepts/content.html')
    output = {'schema': 'IFC4X3_ADD2', 'sourceCommit': COMMIT, 'sources': [entity_source, concept_source], 'conformanceEvaluated': False,
              'views': [
                  {'id': 'ReferenceView', 'label': {'en': 'Reference View', 'de': 'Referenzansicht'}},
                  {'id': 'Alignment-basedReferenceView', 'label': {'en': 'Alignment Based View', 'de': 'Trassierungsbasierte Ansicht'}}],
              'entities': [normalized(row, True) for row in entities if row['href'] and row['href'].startswith('lexical/Ifc')],
              'concepts': [normalized(row) for row in concepts if row['href'] and row['href'].endswith('content.html')]}
    catalog = json.loads((TARGET.parent / '📚️catalog/🔣️.json').read_text(encoding='utf-8'))
    assert {row['id'] for row in output['entities']} == set(catalog['entities'])
    assert len({row['id'] for row in output['concepts']}) == len(output['concepts'])
    TARGET.mkdir(parents=True, exist_ok=True)
    (TARGET / '🔣️.json').write_text(json.dumps(output, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    quote = lambda value: json.dumps(value, ensure_ascii=False)
    optional = lambda row, view: 'Some(&[' + ', '.join(map(quote, row['views'][view])) + '])' if view in row['views'] else 'None'
    native = ['/// 🗂️ Immutable scope rows retained from the official ADD2 listings.']
    for name in ['entities', 'concepts']:
        native.append(f'const {name.upper()}: &[IfcMvdEntry] = &[')
        for row in sorted(output[name], key=lambda row: row['id']):
            native.append('    IfcMvdEntry { ' + ', '.join([
                'id: ' + quote(row['id']), 'name: ' + quote(row['name']), 'href: ' + quote(row['href']),
                'universal: ' + str(row['universal']).lower(), 'reference: ' + optional(row, 'ReferenceView'), 'alignment: ' + optional(row, 'Alignment-basedReferenceView')]) + ' },')
        native.append('];')
    native.append('const SOURCES: &[IfcMvdSource] = &[')
    for row in output['sources']:
        native.append('    IfcMvdSource { url: ' + quote(row['url']) + ', sha256: ' + quote(row['sha256']) + ' },')
    native.append('];')
    native.append('const CATALOG: IfcMvdCatalog = IfcMvdCatalog { schema: "IFC4X3_ADD2", source_commit: ' + quote(COMMIT) + ', sources: SOURCES, conformance_evaluated: false, entities: ENTITIES, concepts: CONCEPTS };')
    (TARGET / 'data.rs').write_text('\n'.join(native) + '\n', encoding='utf-8')
    print(json.dumps({'entities': len(output['entities']), 'concepts': len(output['concepts']), 'sources': output['sources']}))


if __name__ == '__main__':
    main()
