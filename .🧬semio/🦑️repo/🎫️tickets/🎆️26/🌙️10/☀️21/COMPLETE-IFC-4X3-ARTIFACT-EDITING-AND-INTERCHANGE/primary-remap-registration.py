import json
from pathlib import Path
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc')
def write(path,text):
    encoded=text.encode('utf-8')
    with path.open('r+b') as output:
        output.write(encoded)
        if len(encoded)<path.stat().st_size:
            output.write(b' '*(path.stat().st_size-len(encoded)))
path=root/'📦️packages/🟦️typescript/📜️script.ts'
text=path.read_text(encoding='utf-8')
old='  "🔎️details/constituent_remap.test.ts",'
assert text.count(old)==1
write(path,text.replace(old,old+'\n  "🔎️details/primary_remap.test.ts",',1))
path=Path('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/details-contextual-tsconfig.json')
config=json.loads(path.read_text(encoding='utf-8'))
config['files'].append('../../../../../../../'+str(root/'🔎️details/primary_remap.test.ts').replace('\\','/'))
write(path,json.dumps(config,ensure_ascii=False,indent=2)+'\n')
for name,key in [('equal-arity-type.json','primary-different-arity-frontier'),('primary-equal-arity-type.json','different-arity-needs-defaults'),('primary-optional-shape.json','nonzero-remapping-frontier'),('constituent-remap.json','primary-remap-frontier')]:
    path=root/'🔎️details/🧪️tests'/name
    text=path.read_text(encoding='utf-8')
    lines=text.splitlines(keepends=True)
    index=next(index for index,line in enumerate(lines) if '"id":"'+key+'"' in line)
    assert lines[index].count('"id"')==1
    del lines[index]
    if lines[index].strip().startswith(']}'):
        lines[index-1]=lines[index-1].rstrip().removesuffix(',')+'\n'
    write(path,''.join(lines))
