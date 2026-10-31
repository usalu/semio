from pathlib import Path
from urllib.request import urlopen
import io,json,logging
from pypdf import PdfReader,PdfWriter
import pypdfium2 as pdfium
logging.getLogger('pypdf').setLevel(logging.ERROR)
ROOT=Path(__file__).resolve().parent;OUT=ROOT/'🗑️generated'
url='https://bpb-us-e1.wpmucdn.com/blogs.uoregon.edu/dist/1/16902/files/2019/02/Facade-Construction-Manual-2h2g0dz.pdf'
print('[DEBUG] Fetching technical PDF in memory',flush=True)
raw=urlopen(url,timeout=90).read();reader=PdfReader(io.BytesIO(raw));print('[DEBUG] bytes',len(raw),'pages',len(reader.pages),flush=True)
found=[]
for i,page in enumerate(reader.pages):
 text=page.extract_text()or''
 if any(word in ''.join(text.lower().split()) for word in ('nakagin','kurokawa','capsuletower')):
  found.append(i);print('[DEBUG] MATCH',i+1,text[:15000],flush=True)
 if i%50==0:print('[DEBUG] scanned',i,'chars',len(text),'sample',text[:180],flush=True)
print('[DEBUG] matching pages',found,flush=True)
selected=sorted({j for i in found for j in range(max(0,i-1),min(len(reader.pages),i+2))})
writer=PdfWriter()
for i in selected:writer.add_page(reader.pages[i])
buffer=io.BytesIO();writer.write(buffer);subset=buffer.getvalue();(OUT/'facade-nakagin-pages.pdf').write_bytes(subset)
doc=pdfium.PdfDocument(subset)
for number,i in enumerate(selected):
 bitmap=doc[number].render(scale=3.2);bitmap.to_pil().save(OUT/f'facade-nakagin-page-{i+1}.png')
(OUT/'facade-nakagin-index.json').write_text(json.dumps({'url':url,'bytes':len(raw),'pages':len(reader.pages),'matches_one_based':[i+1 for i in found],'rendered_one_based':[i+1 for i in selected],'metadata':{str(k):str(v)for k,v in(reader.metadata or{}).items()}},indent=2))
print('[DEBUG] selected original pages rendered',selected,flush=True)
