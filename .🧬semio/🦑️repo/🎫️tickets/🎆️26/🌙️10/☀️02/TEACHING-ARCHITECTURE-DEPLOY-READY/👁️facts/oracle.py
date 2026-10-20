import base64,hashlib,json,re,sqlite3,sys
reference='ghcr.io/usalu/architecture-course-proctor:neutral'
a='sha256:'+'a'*64;b='sha256:'+'b'*64
line='[deploy] '+reference+' '+a+' linux/amd64 64 bytes\n'
variants={'base':'Grüße\n\n'+line,'repeat':line+line,'different':line+line.replace(a,b),'missing':'no build identity\n'}
output={}
for name,text in variants.items():
 body=text.encode();ids=[]
 for row in text.splitlines():
  if '[deploy] '+reference+' ' in row:
   match=re.search(r' (sha256:[a-f0-9]{64}) linux/amd64 [0-9]+ bytes\s*$',row)
   if match:ids.append(match.group(1))
 db=sqlite3.connect(':memory:');db.execute('create table identities(id text)');db.executemany('insert into identities values(?)',[(v,) for v in ids]);n=db.execute('select count(distinct id) from identities').fetchone()[0]
 output[name]={'body':base64.b64encode(body).decode(),'bytes':len(body),'sha256':hashlib.sha256(body).hexdigest(),'lines':body.count(b'\n'),'ids':ids,'uniqueIds':n,'accepted':bool(ids) and n==1}
if len(sys.argv)>1:assert all(r['actual']==r['expected'] for r in json.loads(sys.argv[1]))
print(json.dumps({'variants':output,'oracle':'Python hashlib + SQLite distinct image identity'},separators=(',',':')))
