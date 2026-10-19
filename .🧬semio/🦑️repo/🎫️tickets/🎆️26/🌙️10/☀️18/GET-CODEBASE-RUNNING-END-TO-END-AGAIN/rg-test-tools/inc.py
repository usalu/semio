import re,os,sys
f=sys.argv[1]
d=os.path.dirname(f)
s=open(f,encoding='utf8').read()
for m in re.finditer(r'include_(?:str|bytes)!\("([^"]+)"\)',s):
    t=os.path.normpath(os.path.join(d,m.group(1)))
    print(os.path.exists(t), m.group(1)[-60:])
