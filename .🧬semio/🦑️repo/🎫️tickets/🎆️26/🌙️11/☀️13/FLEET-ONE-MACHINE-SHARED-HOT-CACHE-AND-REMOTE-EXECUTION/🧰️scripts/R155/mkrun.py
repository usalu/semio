import os,re,shutil,sys
base=os.path.dirname(os.path.abspath(__file__))
real=os.path.join(base,'real','🔬️slice-lab'); run=os.path.join(base,'run','🔬️slice-lab')
live='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab'
shutil.rmtree(os.path.join(base,'run'),ignore_errors=True)
shutil.copytree(real,run)
pat=re.compile(r'''(from\s*|import\s*\(?\s*)(["'])(\.\.?/[^"']+)\2''')
for d,_,fs in os.walk(run):
    for f in fs:
        if not f.endswith(('.ts','.mjs')): continue
        p=os.path.join(d,f); t=open(p,encoding='utf8').read()
        rel=os.path.relpath(d,run); livedir=os.path.normpath(os.path.join(live,rel))
        def sub(m):
            spec=m.group(3); tgt=os.path.normpath(os.path.join(livedir,spec))
            if tgt.startswith(live+os.sep) and not spec.startswith('../../'.replace('../../','../') if False else '..'): return m.group(0)
            if tgt.startswith(live+os.sep):
                # inside lab: keep relative unless escapes run copy
                return m.group(0)
            return m.group(1)+m.group(2)+tgt+m.group(2)
        open(p,'w',encoding='utf8').write(pat.sub(sub,t))
