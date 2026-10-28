"""📚 Pin primary machine-schema/test inventory separately from released IFC view authority."""
from pathlib import Path
import urllib.request,json,hashlib,xml.etree.ElementTree as ET
class PrimaryResources:
    """🔗 Return primary URL, byte hash and parsed inventory without runtime dependencies."""
    def read(self,url):
        with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'semio-read-only-audit'}),timeout=30) as r:return r.geturl(),r.read()
o=PrimaryResources();out={};api='https://api.github.com/repos/buildingSMART/mvdXML';url,raw=o.read(api+'/commits/master');pin=json.loads(raw)['sha'];out['commit']=pin;url,raw=o.read(api+'/git/trees/'+pin+'?recursive=1');tree=json.loads(raw);items=[r for r in tree['tree'] if r['type']=='blob' and (r['path'].endswith('.xsd') or r['path'].endswith('.xml') or r['path']=='README.md')];out['treeTruncated']=tree.get('truncated');out['assets']=items;out['observations']=[]
for path in ['README.md','mvdXML1.2/xsd/mvdXML_V1.2_draft8.xsd','mvdXML1.1/xml/Example-CV104.xml']:
    url,raw=o.read('https://raw.githubusercontent.com/buildingSMART/mvdXML/'+pin+'/'+path);row={'path':path,'url':url,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
    if path.endswith('.xsd'):
        root=ET.fromstring(raw);row['targetNamespace']=root.attrib.get('targetNamespace');row['imports']=[r.attrib for r in root.findall('{http://www.w3.org/2001/XMLSchema}import')]
    if path.endswith('.xml'):
        root=ET.fromstring(raw);row['views']=[r.attrib for r in root.iter() if r.tag.endswith('ModelView')]
    if path=='README.md':row['text']=raw.decode()
    out['observations'].append(row)
for url in ['https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/content/introduction.htm','https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex/annex-a.html']:
    try:
        final,raw=o.read(url);out.setdefault('canonical',[]).append({'url':url,'finalUrl':final,'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()})
    except Exception as e:out.setdefault('canonical',[]).append({'url':url,'error':str(e)})
(Path(__file__).parent/'🗑️generated/mvd-primary-revisit.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG]',pin,len(items))
