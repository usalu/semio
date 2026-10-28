"""usage: stage-copy.py <src> <dst> — copies a test file with relative imports and import.meta.dir/url made absolute so it runs outside its module (freeze-safe verification)."""
import os,re,sys
src,dst=os.path.abspath(sys.argv[1]),sys.argv[2]
d=os.path.dirname(src)
s=open(src).read()
def fix(m):
    q,spec=m.group(1),m.group(2)
    return q+os.path.normpath(os.path.join(d,spec))+q
s=re.sub(r'''(?<=from)\s*(["'])(\.{1,2}/[^"']*)\1''',lambda m:' '+fix(m),s)
s=re.sub(r'''(?<=import)\s*(["'])(\.{1,2}/[^"']*)\1''',lambda m:' '+fix(m),s)
s=re.sub(r'''import\(\s*(["'])(\.{1,2}/[^"']*)\1\s*\)''',lambda m:'import('+fix(m)+')',s)
s=s.replace('import.meta.dir','"'+d+'"').replace('import.meta.path','"'+src+'"').replace('import.meta.url','"file://'+src+'"')
open(dst,'w').write(s)
