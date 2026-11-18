import re,sys
def frame(a,b):return "%d:%s%d:%s"%(len(a),a,len(b),b)
def fix(p,dry):
    s=open(p).read();n=0
    rowre=re.compile(r'\("([^"]+)\.composer\.([^"]+)", "composer", "([^"]+)", &\[\("dialect", "([^"]+)"\)\]')
    natives={}
    for m in rowre.finditer(s):
        o,_,_,d=m.groups()
        if d.split('@')[0]==o:natives[o]=d
    def r1(m):
        nonlocal n
        o,i,desc,d=m.groups()
        if o in natives and d.split('@')[0]!=o:
            n+=1;return '("%s.composer.%s", "composer", "%s", &[("composition-route", "%s")]'%(o,i,desc,frame(natives[o],d))
        return m.group(0)
    s=rowre.sub(r1,s)
    chain=re.compile(r'(ArtifactCapability::new\(ArtifactIdentity::parse\("([^"]+)\.composer\.([^"]+)"\)\?,\s*ArtifactCapabilityKind::composer\(\)\)(?:(?!ArtifactCapability::new).)*?)ArtifactIdentityNamespace::dialect\(\), "([^"]+)"\)',re.S)
    nat2={}
    for m in chain.finditer(s):
        o,d=m.group(2),m.group(4)
        if d.split('@')[0]==o:nat2[o]=d
    def r2(m):
        nonlocal n
        o,d=m.group(2),m.group(4)
        if o in nat2 and d.split('@')[0]!=o:
            n+=1;return m.group(1)+'ArtifactIdentityNamespace::composition_route(), "%s")'%frame(nat2[o],d)
        return m.group(0)
    s=chain.sub(r2,s)
    print(n,p[:90])
    if n and not dry:open(p,'w').write(s)
for p in sys.argv[2:]:fix(p,sys.argv[1]=='dry')
