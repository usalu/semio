"""🗂️ Pin primary machine-readable candidates without assigning release authority."""
import urllib.request,json,hashlib,xml.etree.ElementTree as E
from pathlib import Path
root=Path(__file__).parent;out=root/'🗑️generated'/'mvd-machine';out.mkdir(exist_ok=True)
def get(url):return urllib.request.urlopen(url,timeout=60).read()
def api(url):return json.loads(get(url))
repo='buildingSMART/IFC-translations';sha='d7465622831898747817ed4767332f432fa55be9';url=f'https://raw.githubusercontent.com/{repo}/{sha}/IFC4.3.mvdxml';data=get(url);(out/'candidate.mvdxml').write_bytes(data);xml=E.fromstring(data)
local=lambda e:e.tag.split('}')[-1]
views=[{'tag':local(e),'attributes':e.attrib}for e in xml.iter()if local(e)=='ModelView'];templates=[e for e in xml.iter()if local(e)=='ConceptTemplate'];rules={name:sum(local(e)==name for e in xml.iter())for name in ['AttributeRule','EntityRule','TemplateRule','TemplateRules','ConceptRoot','Concept']}
release='610ee3f95db9d0380b1eb8de4dc6e17833531518';listing=api(f'https://api.github.com/repos/buildingSMART/Standards-builds/contents/IFC/RELEASE/IFC4_3?ref={release}')
result={'candidate':{'repository':repo,'commit':sha,'url':url,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'root':xml.attrib,'views':views,'templates':len(templates),'rules':rules},'pinnedReleaseRootListing':[{'name':e['name'],'type':e['type']}for e in listing]};(out/'manifest.json').write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps(result))


selected=[e for e in templates if 'Alignment' in e.get('name','')]
(out/'alignment-templates.json').write_text(json.dumps([{'attributes':e.attrib,'rules':[{'tag':local(n),'attributes':n.attrib} for n in e.iter() if local(n) in ['AttributeRule','EntityRule','TemplateRule']]}for e in selected],indent=2),encoding='utf-8')

