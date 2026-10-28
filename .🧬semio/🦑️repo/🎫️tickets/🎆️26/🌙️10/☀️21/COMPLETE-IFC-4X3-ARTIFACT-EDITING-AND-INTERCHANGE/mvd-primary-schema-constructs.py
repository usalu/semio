"""🧾 Observe exact final-named and draft schema constructs without treating XSD as view authority."""
from pathlib import Path
import json,urllib.request,hashlib,xml.etree.ElementTree as ET
p=Path(__file__).parent; prior=json.loads((p/'🗑️generated/mvd-primary-revisit.json').read_text(encoding='utf8')); pin=prior['commit'];rows=[]
for path in ['mvdXML1.2/xsd/mvdXML_V1.2.xsd','mvdXML1.2/xsd/mvdXML_V1-2_Draft9.xsd']:
    url='https://raw.githubusercontent.com/buildingSMART/mvdXML/'+pin+'/'+path
    raw=urllib.request.urlopen(url,timeout=30).read();root=ET.fromstring(raw);ns={'x':'http://www.w3.org/2001/XMLSchema'}
    rows.append({'url':url,'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'namespace':root.get('targetNamespace'),'imports':[r.attrib for r in root.findall('x:import',ns)],'namedTypes':[r.get('name') for r in root.findall('x:complexType',ns)],'namedElements':[r.get('name') for r in root.findall('x:element',ns)]})
(p/'🗑️generated/mvd-primary-schema-constructs.json').write_text(json.dumps(rows,indent=2),encoding='utf8');print('[DEBUG]',json.dumps(rows))
