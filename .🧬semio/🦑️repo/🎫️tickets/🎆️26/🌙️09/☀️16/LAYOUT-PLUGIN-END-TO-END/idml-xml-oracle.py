from pathlib import Path
import json
import xml.etree.ElementTree as ET
base=Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml")
texts=set()
def walk(value):
 if isinstance(value,str):texts.add(value)
 elif isinstance(value,list):
  if all(isinstance(n,int) and 0<=n<=255 for n in value):
   try:
    text=bytes(value).decode("utf-8")
    texts.add(text)
   except (ValueError,UnicodeError):pass
  else:
   for item in value:walk(item)
 elif isinstance(value,dict):
  for item in value.values():walk(item)
for source in base.rglob("*.json"):
 try:walk(json.loads(source.read_text()))
 except (ValueError,UnicodeError):pass
records={}
for text in texts:
 try:
  root=ET.fromstring(text)
  records[text]={"valid":True,"attributes":root.attrib,"version":root.attrib.get("DOMVersion") if root.tag=="Document" else None}
 except ET.ParseError:records[text]={"valid":False,"version":None}
output=Path(__file__).parent/"🗑️generated/idml-xml-oracle.json"
output.parent.mkdir(exist_ok=True)
output.write_text(json.dumps(records,ensure_ascii=False))
print(json.dumps({"independentXmlInputs":len(records)}))
