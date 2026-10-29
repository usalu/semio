"""🔬 Diagnose marshal reference flags without replaying completed feature tests."""
import hashlib,inspect,json,marshal,types,struct
from pathlib import Path
ns=feature85;generated=ns['generated'];summary=json.loads((generated/'features-r85-summary.json').read_text());lookup={ns['canonical'](t):t for t in ns['selected']+ns['boundary']}
def canonical_code(code):
    def constant(value):
        if isinstance(value,types.CodeType):return canonical_code(value)
        if isinstance(value,bytes):return {'bytes':value.hex()}
        if isinstance(value,tuple):return {'tuple':[constant(x) for x in value]}
        if isinstance(value,frozenset):return {'frozenset':sorted([constant(x) for x in value],key=lambda x:json.dumps(x,sort_keys=True))}
        if value is Ellipsis:return {'ellipsis':True}
        if isinstance(value,float):return {'float_bits':struct.pack('>d',value).hex()}
        if isinstance(value,complex):return {'complex_bits':[struct.pack('>d',value.real).hex(),struct.pack('>d',value.imag).hex()]}
        return value
    return {'code':code.co_code.hex(),'constants':[constant(x) for x in code.co_consts],'names':list(code.co_names),'varnames':list(code.co_varnames),'freevars':list(code.co_freevars),'cellvars':list(code.co_cellvars),'argcount':code.co_argcount,'posonlyargcount':code.co_posonlyargcount,'kwonlyargcount':code.co_kwonlyargcount,'nlocals':code.co_nlocals,'stacksize':code.co_stacksize,'flags':code.co_flags,'firstlineno':code.co_firstlineno,'linetable':code.co_linetable.hex(),'exceptiontable':code.co_exceptiontable.hex(),'name':code.co_name,'qualname':code.co_qualname,'filename':code.co_filename}
def hash_value(value):return hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def walk_code(code):
    yield code
    for value in code.co_consts:
        if isinstance(value,types.CodeType):yield from walk_code(value)
def function_for(test,key):
    if key=='function':return test._testFunc
    if key=='bound_run':return next(v for v in test._testFunc.__defaults__ or () if inspect.ismodule(v)).run
    return getattr(test,key)
compiled={};changed=[];results=[];all_results=[];external=[]
for identity,before in summary['before']['selected_runtime'].items():
    after=summary['after']['selected_runtime'][identity]
    for key,record in before.items():
        if record is None:continue
        fn=inspect.unwrap(function_for(lookup[identity],key));fn=getattr(fn,'__func__',fn);code=fn.__code__;path=code.co_filename
        if not path.startswith('/repo/'):
            external.append({'id':identity,'part':key,'framework_file':path});continue
        if path not in compiled:compiled[path]=list(walk_code(compile(Path(path).read_text(),path,'exec',dont_inherit=True)))
        matches=[c for c in compiled[path] if c.co_qualname==code.co_qualname and c.co_firstlineno==code.co_firstlineno]
        assert len(matches)==1,(identity,key,path,code.co_qualname)
        live=canonical_code(code);fresh=canonical_code(matches[0]);item={'id':identity,'part':key,'source_file':path,'marshal_before':record['code'],'marshal_after':after[key]['code'],'canonical_live':hash_value(live),'canonical_fresh':hash_value(fresh),'same_canonical_code':live==fresh,'same_defaults_closure':all(record[field]==after[key][field] for field in ('defaults','keyword_defaults','closure'))};all_results.append(item)
        if record!=after[key]:results.append(item)

probe={};exec(compile('def probe():\n def child(): return 1\n return child()\n','<marshal-reference-probe>','exec'),probe);code=probe['probe'].__code__;canonical_before=hash_value(canonical_code(code));bytes_before=marshal.dumps(code);hash_before=hashlib.sha256(bytes_before).hexdigest();held=code.co_consts[1];bytes_held=marshal.dumps(code);hash_held=hashlib.sha256(bytes_held).hexdigest();canonical_after=hash_value(canonical_code(code));first_offset=next(i for i,(a,b) in enumerate(zip(bytes_before,bytes_held)) if a!=b);first_change={'offset':first_offset,'before_byte':bytes_before[first_offset],'after_byte':bytes_held[first_offset],'xor':bytes_before[first_offset]^bytes_held[first_offset]};reference_probe={'first_byte_difference':first_change,'before_hex':bytes_before.hex(),'extra_reference_hex':bytes_held.hex(),'marshal_before':hash_before,'marshal_with_extra_reference':hash_held,'marshal_changed':hash_before!=hash_held,'canonical_before':canonical_before,'canonical_after':canonical_after,'canonical_unchanged':canonical_before==canonical_after}
record={'changed_selected_records':len({r['id'] for r in results}),'changed_callable_records':len(results),'all_match_frozen_source_canonical_code':all(r['same_canonical_code'] for r in results),'all_defaults_closures_unchanged':all(r['same_defaults_closure'] for r in results),'details':results,'all_owned_selected_callable_records':len(all_results),'all_owned_selected_match_fresh_source':all(r['same_canonical_code'] for r in all_results),'all_owned_selected_defaults_closures_unchanged':all(r['same_defaults_closure'] for r in all_results),'all_owned_details':all_results,'framework_records_not_recompiled':external,'source_independent_reference_probe':reference_probe,'tests_replayed':0}
(generated/'features-r85-runtime-diagnosis.json').write_text(json.dumps(record,ensure_ascii=False));print({k:v for k,v in record.items() if k not in ('details','all_owned_details','framework_records_not_recompiled')})
