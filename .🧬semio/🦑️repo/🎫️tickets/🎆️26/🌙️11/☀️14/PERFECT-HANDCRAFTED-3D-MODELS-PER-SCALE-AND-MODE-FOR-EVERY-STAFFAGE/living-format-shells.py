"""🍃 Keep author-owned indexed rows legible while limiting catalog text and allocation size."""
import json
import re
import sys
from pathlib import Path

root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
ticket=Path(__file__).parent
decoder=json.JSONDecoder()
for slug in sys.argv[1:]:
    for path in (next(root.glob(f'*/{slug}/*.json')),ticket/'volumes/vegetation'/f'{slug}.json'):
        raw=path.read_text()
        offset=0
        pieces=[]
        for match in re.finditer(r'"(?:vertices|triangles)":\s*\[',raw):
            start=match.end()-1
            value,end=decoder.raw_decode(raw,start)
            pieces.extend((raw[offset:start],json.dumps(value,separators=(',',':'))))
            offset=end
        pieces.append(raw[offset:])
        updated=''.join(pieces)
        if updated==raw:continue
        if path.read_text()!=raw:raise RuntimeError(path)
        path.write_text(updated)
        print('[DEBUG]',slug,len(raw),len(updated))

