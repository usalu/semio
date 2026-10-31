from pathlib import Path
from urllib.request import urlopen
import io,json
import pypdfium2 as pdfium
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parent;OUT=ROOT/'🗑️generated'
raw=urlopen('https://bpb-us-e1.wpmucdn.com/blogs.uoregon.edu/dist/1/16902/files/2019/02/Facade-Construction-Manual-2h2g0dz.pdf',timeout=90).read();doc=pdfium.PdfDocument(raw)
print('[DEBUG] image-only pages',len(doc),flush=True)
for start in range(0,len(doc),25):
 contact=Image.new('RGB',(1200,1700),'#dddddd');draw=ImageDraw.Draw(contact)
 for index in range(start,min(start+25,len(doc))):
  page=doc[index];scale=min(230/page.get_width(),310/page.get_height());im=page.render(scale=scale).to_pil().convert('RGB');x=((index-start)%5)*240;y=((index-start)//5)*340;contact.paste(im,(x+(240-im.width)//2,y+22));draw.text((x+5,y+4),f'PDF page{index+1}',fill='black');page.close()
 contact.save(OUT/f'facade-contacts-{start//25+1}.png');print('[DEBUG] contact',start//25+1,flush=True)
