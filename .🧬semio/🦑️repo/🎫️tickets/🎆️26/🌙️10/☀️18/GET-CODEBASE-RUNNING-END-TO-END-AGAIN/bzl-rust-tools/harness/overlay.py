import glob,json,os,re,shutil,sys
sys.path.insert(0,os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import patches
P={'C':glob.glob('.🧬semio/🦑*/🎫*/🎆*/🌙*10/☀*18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN')[0],'B':glob.glob('🧰*/🛍*/🦑*/🔨*/📚*/🏗*/🔧*/🏗*')[0],'TS':'\U0001F7E6\uFE0F','JS':'\U0001F523\uFE0F','SCH':'\U0001F9EC\uFE0F','TST':'\U0001F9EA\uFE0F','ACT':'\U0001F3C3\uFE0F'}
P['S']=glob.glob(os.path.join(P['C'],'bzl-rust-tools','staged','🧰*','**','🦀️cargo'),recursive=True)[0]
H=os.path.join(P['C'],'bzl-rust-tools','harness','overlay')
if os.path.exists(H): shutil.rmtree(H)
os.makedirs(os.path.join(H,P['SCH']+'schema'));os.makedirs(os.path.join(H,P['TST']+'tests'))
root=os.getcwd()
validator=os.path.join(root,glob.glob('🧰*/🔨*/🧬*/✅*/📦*/🟦*/🟦*.ts')[0])
mod=re.sub(r'from "(\.\./)+[^"]*validator[^"]*"','from "%s"'%validator,open(os.path.join(P['S'],P['TS']+'.ts'),encoding='utf8').read())
module=os.path.join(H,P['TS']+'.ts');open(module,'w',encoding='utf8').write(mod)
shutil.copy(os.path.join(P['S'],P['SCH']+'schema',P['JS']+'.json'),os.path.join(H,P['SCH']+'schema',P['JS']+'.json'))
B=os.path.join(root,P['B']);script=os.path.join(B,'📜️script.ts')
text=patches.apply(open(script,encoding='utf8').read())
out=os.path.join(H,'adapter.ts')
def point(m):
    head,quote,path=m.group(1),m.group(2),m.group(3)
    target=os.path.relpath(os.path.normpath(os.path.join(B,path)),H)
    if not target.startswith('.'): target='./'+target
    return head+quote+target+quote
text=re.sub(r'(from\s*|import\()(["\'])(\.{1,2}/[^"\']+)\2',point,text)
text=text.replace('from "./%scargo/%s.ts"'%(patches.CRAB,patches.TS),'from "./%s.ts"'%patches.TS) if False else text
text=re.sub(r'from "[^"]*%scargo/%s\.ts"'%(patches.CRAB,patches.TS),'from "./%s.ts"'%patches.TS,text)
open(out,'w',encoding='utf8').write(text)
shutil.copytree(os.path.join(P['S'],P['ACT']+'action'),os.path.join(H,P['ACT']+'action'))
td=os.path.join(P['S'],P['TST']+'tests')
for n in os.listdir(td):
    t=open(os.path.join(td,n),encoding='utf8').read()
    t=t.replace('import("../../%sscript.ts")'%'\U0001F4DC️','import("../adapter.ts")')
    open(os.path.join(H,P['TST']+'tests',n),'w',encoding='utf8').write(t)
print(H)
