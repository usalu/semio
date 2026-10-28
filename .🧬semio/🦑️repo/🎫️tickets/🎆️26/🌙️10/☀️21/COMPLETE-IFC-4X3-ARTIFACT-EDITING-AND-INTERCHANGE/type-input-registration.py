from pathlib import Path
import json
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc')
path=root/'📦️packages/🟦️typescript/📜️script.ts'
data=path.read_text(encoding='utf-8');old='  "🔎️details/primary_remap.test.ts",';assert data.count(old)==1
data=data.replace(old,old+'\n  "🔎️details/type_input.test.ts",',1)
with path.open('r+b')as output:output.write(data.encode('utf-8'))
path=Path(__file__).with_name('details-contextual-tsconfig.json');data=json.loads(path.read_text(encoding='utf-8'));data['files'].append('../../../../../../../'+str(root/'🔎️details/type_input.test.ts').replace('\\','/'))
with path.open('r+b')as output:output.write((json.dumps(data,ensure_ascii=False,indent=2)+'\n').encode('utf-8'))
