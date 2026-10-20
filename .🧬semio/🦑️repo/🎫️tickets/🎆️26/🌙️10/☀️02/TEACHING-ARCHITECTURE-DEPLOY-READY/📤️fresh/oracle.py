import base64,hashlib,json,sqlite3,sys
files={'proctor/.env':b'PROCTOR_TAG=neutral\n','proctor/Caddyfile':b'neutral','proctor/README.txt':b'load and up --wait','proctor/compose.yaml':b'neutral','proctor/proctor-image.tar':b'archive','site/index.html':'<html>Grüße</html>'.encode(),'site/Grüße.html':b'unicode path'}
rows=[{'path':p,'kind':'directory','mode':448} for p in ['proctor','proctor/certificates','site']]
rows += [{'path':p,'kind':'file','mode':420,'size':len(b),'sha256':hashlib.sha256(b).hexdigest()} for p,b in files.items()]
rows.sort(key=lambda r:r['path']); manifest={'version':1,'rows':rows,'totalBytes':sum(map(len,files.values())),'fileCount':len(files)}
raw=json.dumps(manifest,ensure_ascii=False,separators=(',',':')).encode(); digest=hashlib.sha256(raw).hexdigest(); payload=b''.join(files[r['path']] for r in rows if r['kind']=='file'); wire=raw+b'\n'+payload+b'\nSEMIO-EXPORT-DONE '+digest.encode()+b'\n'
db=sqlite3.connect(':memory:');db.execute('create table files(path text primary key,size integer,sha text)');db.executemany('insert into files values(?,?,?)',[(p,len(b),hashlib.sha256(b).hexdigest()) for p,b in files.items()]);sql=db.execute('select count(*),sum(size) from files').fetchone();assert tuple(sql)==(manifest['fileCount'],manifest['totalBytes'])
if len(sys.argv)>1:
 results=json.loads(sys.argv[1]);assert all(r['actual']==r['expected'] for r in results)
print(json.dumps({'manifest':manifest,'manifestSha256':digest,'frame':base64.b64encode(wire).decode(),'oracle':'Python hashlib + SQLite','sql':sql},separators=(',',':')))
