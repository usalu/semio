"""🧩 Extract bounded primary rule structures as metadata, never source-file copies."""
from pathlib import Path
import urllib.request,xml.etree.ElementTree as ET,json,hashlib
pin='bd158e10901a2a49c067e3f57449431cab7130f0';base='https://raw.githubusercontent.com/buildingSMART/mvdXML/'+pin+'/'
def get(path):return urllib.request.urlopen(base+path,timeout=30).read()
def project(node):return {'tag':node.tag.split('}')[-1],'attributes':node.attrib,'children':[project(c) for c in node if not c.tag.endswith('annotation')]}
x=get('mvdXML1.2/xsd/mvdXML_V1.2.xsd');root=ET.fromstring(x);types={r.get('name'):project(r) for r in root if r.tag.endswith('complexType') and r.get('name') in ['AttributeRule','EntityRule','TemplateRule','TemplateRules','ConceptTemplate','Concept','ModelView','ConceptRoot','Requirements','GenericReference']};examples=[]
for name in ['Example-CV100.xml','Example-CV104.xml','Example-CV106.xml']:
    raw=get('mvdXML1.1/xml/'+name);r=ET.fromstring(raw);examples.append({'path':name,'sha256':hashlib.sha256(raw).hexdigest(),'templates':[{'uuid':n.get('uuid'),'name':n.get('name'),'entity':n.get('applicableEntity'),'schema':n.get('applicableSchema'),'rules':[project(c) for c in n if c.tag.endswith('Rules')]} for n in r.iter() if n.tag.endswith('ConceptTemplate')],'ruleParameters':[n.attrib for n in r.iter() if n.tag.endswith('TemplateRule')]})
(Path(__file__).parent/'🗑️generated/mvd-rule-contract-primary.json').write_text(json.dumps({'types':types,'examples':examples},indent=2),encoding='utf8');print('[DEBUG]',[(e['path'],len(e['templates']),e['ruleParameters']) for e in examples])
