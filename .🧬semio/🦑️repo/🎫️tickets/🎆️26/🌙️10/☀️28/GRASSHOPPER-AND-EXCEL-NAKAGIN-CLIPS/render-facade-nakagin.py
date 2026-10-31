from pathlib import Path
from urllib.request import urlopen
import io,json
import pypdfium2 as pdfium
from pypdf import PdfReader,PdfWriter
ROOT=Path(__file__).resolve().parent;OUT=ROOT/'🗑️generated'
raw=urlopen('https://bpb-us-e1.wpmucdn.com/blogs.uoregon.edu/dist/1/16902/files/2019/02/Facade-Construction-Manual-2h2g0dz.pdf',timeout=90).read()
reader=PdfReader(io.BytesIO(raw));writer=PdfWriter();writer.add_page(reader.pages[133]);buf=io.BytesIO();writer.write(buf);subset=buf.getvalue();(OUT/'facade-nakagin-pages.pdf').write_bytes(subset)
doc=pdfium.PdfDocument(subset);im=doc[0].render(scale=4.0).to_pil();im.save(OUT/'facade-nakagin-page134.png');im.crop((im.width//2,0,im.width,im.height)).save(OUT/'facade-nakagin-detail.png')
(OUT/'facade-nakagin-index.json').write_text(json.dumps({'url':'https://bpb-us-e1.wpmucdn.com/blogs.uoregon.edu/dist/1/16902/files/2019/02/Facade-Construction-Manual-2h2g0dz.pdf','bytes':len(raw),'pages':155,'nakagin_one_based':134,'pixel_size':im.size,'method':'image-only PDF, identified by low-resolution page contact sheets, relevant page rendered at4x'},indent=2));print('[DEBUG] Nakagin page134 rendered',im.size,flush=True)
