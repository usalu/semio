import sys,os,glob
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import widen
from engine import ident_set
from verify_installed import family_files,FAM,REPO
EXT={
 'sqlite':[REPO+'🌎️hub/📇️directory/🪶️sqlite/🧪️tests'],
 'postgres':[REPO+'🌎️hub/📇️directory/🐘️postgres/🧪️tests'],
 'neo4j':[REPO+'🌎️hub/📇️directory/🌐️neo4j/🧪️tests'],
 'model':[], 'tickets':[REPO+'🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧪️tests'],
 'gateway':[REPO+'🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests'],
}
def run(n):
    root,kids=family_files(n)
    files={f:open(f,encoding='utf-8').read() for f in [root]+kids}
    idents={f:ident_set(t) for f,t in files.items()}
    ext=set()
    for d in EXT[n]:
        for p in glob.glob(d+'/**/*.rs',recursive=True):ext|=ident_set(open(p,encoding='utf-8').read())
    total=0
    for f in kids:
        if n=='model' and os.path.basename(os.path.dirname(f)) in('🎨️ansi','🪪️entity'):continue
        others=set(ext)
        for g,s in idents.items():
            if g!=f:others|=s
        ed=widen.plan(files[f],others)
        total+=len(ed)
        for e in ed:print('  pending',os.path.relpath(f,os.path.dirname(root)),e[2])
    print(n,'pending widenings',total)
if __name__=='__main__':
    for n in sys.argv[1:] or FAM:run(n)
