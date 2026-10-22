import sys
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers"
def sub(path,old,new,count=1):
    s=open(path,encoding='utf8').read()
    if s.count(old)!=count: print("SKIP",path[-40:],s.count(old)); return
    open(path,'w',encoding='utf8').write(s.replace(old,new)); print("ok",path[-40:])
sub(D+"/🧬️schema/🔣️.json",'      "maxItems": 1024\n','      "maxItems": 2048\n')
sub(D+"/📦️snapshot/🧬️schema/🔣️.json",'      "maxItems": 1024,\n','      "maxItems": 2048,\n')
f=D+"/🧪️tests/🧫️fixtures/🔣️.json"
sub(f,'"maximumFiles": 1024','"maximumFiles": 2048')
sub(f,'"files": 1024,','"files": 2048,')
sub(f,'"files": 1025,','"files": 2049,')
