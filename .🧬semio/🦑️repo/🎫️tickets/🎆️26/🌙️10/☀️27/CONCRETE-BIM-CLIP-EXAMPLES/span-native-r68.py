"""🧵 Verify corrected native empty-text spans without changing any STEP line byte."""
import hashlib,importlib,json
from pathlib import Path
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
api=importlib.import_module(prefix+'🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
native=importlib.import_module(prefix+'🧱️model.🔌️exchange.🐍️');importlib.reload(native)
source=api.load_model();new_str,new_value=native._str,native._value
def original_str(value,role=None):
    return ["'",(native._escape(value),role),"'"]
def original_value(entry):
    value=entry['value']
    if isinstance(value,bool):return ['IFCBOOLEAN(',('.T.' if value else '.F.','value'),')']
    if isinstance(value,str):return [('IFCTEXT' if entry.get('ifc_type')=='IfcText' else 'IFCLABEL')+"('",(native._escape(value),'value'),"')"]
    return ['IFCREAL(',(native._real(value),'value'),')']
samples=[]
for language in ('en','de'):
    try:
        native._str,native._value=original_str,original_value
        previous=native.step_excerpt('SL-1',source,language=language)
    finally:native._str,native._value=new_str,new_value
    current=native.step_excerpt('SL-1',source,language=language)
    assert previous['lines']==current['lines'],'Serialized IFC bytes changed'
    assert all(0<=span['start']<span['end']<=len(current['lines'][span['line']]) for span in current['spans'])
    key=current['ids']['ownership_SharedOwnerTags'];index=next(i for i,line in enumerate(current['lines']) if line.startswith(key+'='))
    quoted=[current['lines'][index][span['start']:span['end']] for span in current['spans'] if span['line']==index and span['role']=='value']
    assert quoted==["''"],quoted
    text='\n'.join(current['lines'])
    samples.append({'language':language,'text':text,'property':key,'text_sha256':hashlib.sha256(text.encode()).hexdigest(),'line':current['lines'][index],'quoted':quoted,'old_zero_spans':sum(s['end']==s['start'] for s in previous['spans'])})
result={'source_hash':hashlib.sha256(Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes()).hexdigest(),'samples':samples}
(Path(__file__).parent/'🗑️generated/span-native-r68.json').write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print({'source':result['source_hash'],'samples':[{k:s[k] for k in ('language','text_sha256','quoted','old_zero_spans')} for s in samples]})
