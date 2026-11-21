"""🌱 Replace only the individual author-selected volume solids in a shared catalog."""
import json
from pathlib import Path

def edit(path, author):
    raw=path.read_text()
    decoder=json.JSONDecoder()
    start=raw.index('{',raw.index('"volumes":'))
    pos=start; parts=[];cursor=0
    for scale in ('50','100','200','500'):
        pos=raw.index('{',raw.index('"'+scale+'"',pos))
        for mode in ('body','thermal','structure'):
            pos=raw.index('[',raw.index('"'+mode+'"',pos))+1
            while True:
                while raw[pos] in ' \t\n\r,':pos+=1
                if raw[pos]==']':pos+=1;break
                solid,end=decoder.raw_decode(raw,pos)
                if author(scale, mode, solid):
                    parts.extend((raw[cursor:pos],json.dumps(solid,ensure_ascii=False,separators=(',',':'))))
                    cursor=end
                pos=end
    parts.append(raw[cursor:]);text=''.join(parts)
    if text!=raw:
        if path.read_text()!=raw:raise RuntimeError(path)
        path.write_text(text)

