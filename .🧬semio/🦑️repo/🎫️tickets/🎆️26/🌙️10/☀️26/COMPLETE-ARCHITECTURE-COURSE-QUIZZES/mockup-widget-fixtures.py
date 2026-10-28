from pathlib import Path
from copy import deepcopy
import json
root=Path('🧰️framework/🛍️products/📚️course')
path=root/'🧫️fixtures/🖥️mockups/🔣️.json'
vectors=json.loads(path.read_text(encoding='utf-8'))
text=lambda value: {'en': value,'de':value}
for widget in [{'kind':'parameter'},{'kind':'component'},{'kind':'slider','min':-2,'max':8,'value':3},{'kind':'panel','lines':[text('12'),text('24')]},{'kind':'plugin','label':text('Solver')}]:
    mockup=deepcopy(vectors[1]['mockup'])
    mockup['nodes'][0]['widget']=widget
    mockup['target']={'kind':'node','node':'source','index':0}
    vectors.append({'id':'widget-'+widget['kind'],'mockup':mockup,'valid':True,'schemaValid':True})
for name, widget, schema_valid in [('slider-range',{'kind':'slider','min':2,'max':2,'value':2}, True),('slider-value',{'kind':'slider','min':0,'max':1,'value':2}, True),('panel-language',{'kind':'panel','lines':[{'en':'12'}]},False),('widget-kind',{'kind':'browser'},False)]:
    mockup=deepcopy(vectors[1]['mockup'])
    mockup['nodes'][0]['widget']=widget
    vectors.append({'id':name,'mockup':mockup,'valid':False,'schemaValid':schema_valid})
path.write_text(json.dumps(vectors,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
path=root/'🧬️schema/🟦️.ts'
source=path.read_text(encoding='utf-8')
source='\n'.join(line.replace('; readonly mockup?: Mockup','') if line.startswith(('export type Dimension =','export type CourseTaskView =')) else line for line in source.splitlines())+'\n'
path.write_text(source,encoding='utf-8')
