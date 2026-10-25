import sys
p=sys.argv[1];t=open(p).read()
t=t.replace("PATH=re.compile(r'#\\[path","PATH=re.compile(r'(?m)^#\\[path")
t=t.replace("    d=os.path.dirname(entry)\n","    t=re.sub(r'(?m)^\\s*//.*$','',t)\n    d=os.path.dirname(entry)\n",1)
open(p,'w').write(t)
