"""📐 Pin released alignment child concepts and exact usage source hashes."""
import json,hashlib,urllib.request,re
from pathlib import Path
root=Path(__file__).resolve().parent;out=root/'🗑️generated'/'alignment-concepts';out.mkdir(exist_ok=True)
base='https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/'
paths=['concepts/Object_Composition/Nesting/Alignment_Layouts/Alignment_Layout_-_Horizontal,_Vertical_and_Cant/content.html','concepts/Object_Composition/Nesting/Alignment_Layouts/Alignment_Layout_-_Reusing_Horizontal_Layout/content.html','lexical/IfcAlignment.htm']
rows=[]
for i,path in enumerate(paths):
 raw=urllib.request.urlopen(base+path).read();(out/(str(i)+'.html')).write_bytes(raw);text=raw.decode('utf8');main=text.split('id="main-content"',1)[-1];before=main.split('<figure>',1)[0];plain=re.sub('<[^>]+>',' ',before);plain=re.sub(r'\s+',' ',plain).strip();rows.append({'url':base+path,'sha256':hashlib.sha256(raw).hexdigest(),'intro':plain,'xmlLinks':re.findall(r'href="([^"]*(?:mvdxml|\.xml)[^"]*)"',text)})
(out/'sources.json').write_text(json.dumps(rows,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(rows))
