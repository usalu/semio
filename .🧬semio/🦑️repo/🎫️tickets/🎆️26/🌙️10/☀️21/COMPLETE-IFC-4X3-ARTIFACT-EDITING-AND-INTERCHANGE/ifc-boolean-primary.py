import pathlib,urllib.request,hashlib,json
p=pathlib.Path(__file__).resolve().parent;rows=[]
class PrimaryReader:
 def bytes(self,url):
  with urllib.request.urlopen(url,timeout=30)as response:return response.read()
reader=PrimaryReader()
for name in ['IfcBooleanResult','IfcBooleanClippingResult','IfcHalfSpaceSolid']:
 url='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/'+name+'.htm';body=reader.bytes(url);rows.append({'name':name,'url':url,'sha256':hashlib.sha256(body).hexdigest(),'bytes':len(body)})
(p/'🗑️generated/ifc-boolean-primary.json').write_text(json.dumps(rows,indent=2),encoding='utf8');print('[DEBUG] primary3 hashes retained')
