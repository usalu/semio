import sys,os,shutil
ROOT="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
F=L+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R149/backup-catalog"
pairs=[
('observedFiles=new Set(),dependencies=new Set(),','observedFiles=new Set(),catalogProducers=new Set(),dependencies=new Set(),'),
('addFile(contribution.projectRoot+"/"+PROJECT_BASENAME);if(contribution.project!==json.name)dependencies.add(contribution.project);}','addFile(contribution.projectRoot+"/"+PROJECT_BASENAME);if(contribution.project!==json.name)catalogProducers.add(contribution.project);}'),
(' for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);\n',' for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);\n for(const producer of catalogProducers)dependencies.add(producer);\n'),
]
mode=sys.argv[1]
path=os.path.join(sys.argv[2] if len(sys.argv)>2 else ROOT,F)
s=open(path,encoding="utf8").read()
if mode=="apply":
    os.makedirs(SC,exist_ok=True);open(os.path.join(SC,"owner-discovery.mjs"),"w",encoding="utf8").write(s)
    for a,b in pairs:
        assert s.count(a)==1,("anchor",a[:60],s.count(a));s=s.replace(a,b)
else:
    for a,b in pairs:
        assert s.count(b)==1,("anchor",b[:60],s.count(b));s=s.replace(b,a)
open(path,"w",encoding="utf8").write(s)
print(mode,"ok")
