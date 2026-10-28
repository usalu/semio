#!/usr/bin/env python3
"""shadow-file.py <original test file> <edited copy> <out dir>: writes a runnable shadow of an edited TEST file OUTSIDE the tree: relative specifiers become absolute against the ORIGINAL directory, import.meta.dir/dirname/url keep the original value, bare imports resolve through a node_modules symlink of the original's nearest one. Prints the shadow path."""
import os,re,sys,json
orig,edited,out=[os.path.abspath(p) for p in sys.argv[1:4]]
base=os.path.dirname(orig);text=open(edited,encoding='utf8').read()
def absolute(spec): return os.path.normpath(os.path.join(base,spec))
text=re.sub(r'''(\bfrom\s*|\bimport\s*\(\s*|\bimport\s*|\brequire\s*\(\s*)(["'])(\.{1,2}/[^"']*|\.)\2''',lambda m:m.group(1)+m.group(2)+absolute(m.group(3))+m.group(2),text)
if not os.environ.get('KEEP_META'): text=text.replace('import.meta.dirname',json.dumps(base)).replace('import.meta.dir',json.dumps(base)).replace('import.meta.url',json.dumps('file://'+orig))
os.makedirs(out,exist_ok=True)
d=base
while d!='/':
    nm=os.path.join(d,'node_modules')
    if os.path.isdir(nm):
        link=os.path.join(out,'node_modules')
        if os.path.islink(link): os.remove(link)
        if not os.path.exists(link): os.symlink(nm,link)
        break
    d=os.path.dirname(d)
path=os.path.join(out,'shadow-'+str(abs(hash(orig))%10**6)+'.ts');open(path,'w',encoding='utf8').write(text);print(path)
