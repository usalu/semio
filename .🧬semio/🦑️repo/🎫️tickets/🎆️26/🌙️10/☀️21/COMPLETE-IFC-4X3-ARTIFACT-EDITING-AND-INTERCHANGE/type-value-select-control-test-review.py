from pathlib import Path
import json
T=Path(__file__).parent
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type_value_select_input.test.ts')
s=p.read_text(encoding='utf-8').replace('for(const prefix of [1,Math.floor(total/2),total])','for(let prefix=1;prefix<=total;prefix++)').replace('}},60000);','}},120000);')
p.write_text(s,encoding='utf-8',newline='\n')
config=json.loads((T/'type-measure-select-tsconfig.json').read_text(encoding='utf-8'))
config['files']=[config['files'][0].replace('type_measure_select_input','type_value_select_input')]
(T/'type-value-select-tsconfig.json').write_text(json.dumps(config)+'\n',encoding='utf-8')
