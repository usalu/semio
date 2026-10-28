"""📋 Inventory exact original certification headers without changing source bytes."""
import hashlib,json,re
from pathlib import Path
root=Path(__file__).resolve().parent
inputs=json.loads((root/'🗑️generated/clipping-official-input-observations.json').read_text(encoding='utf-8-sig'))
rows=[]
for item in inputs:
    raw=Path(item['local']).read_bytes();text=raw.decode('utf-8-sig');headers={key:re.search(r'\b'+key+r'\s*\(.*?\);',text,re.S).group(0) for key in ['FILE_SCHEMA','FILE_NAME','FILE_DESCRIPTION']}
    rows.append(dict(path=item['path'],commit=item['commit'],url=item['url'],sha256=hashlib.sha256(raw).hexdigest(),bytes=len(raw),headers=headers,mvd=re.findall(r'ViewDefinition\s*\[[^\]]*\]',headers['FILE_DESCRIPTION'],re.I)))
(root/'🗑️generated/certification-independent-headers.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(files=len(rows),bytes=sum(r['bytes']for r in rows),descriptions=sorted(set(r['headers']['FILE_DESCRIPTION']for r in rows)))))

