import json
def pr(items):
    out=[]
    for x in items or []:
        if isinstance(x,str):out.append(x)
        elif isinstance(x,dict):
            if 'env' in x:
                if x['env'].startswith('SEMIO_SOURCE_CLOSURE_'):out.append({'t':x['env'][21:]})
            elif 'input' in x:out.append({'input':x['input'],'projects':x.get('projects')})
    return out
o=open('model.jsonl','w')
for line in open('rust-nodes.jsonl'):
    d=json.loads(line)
    o.write(json.dumps({'name':d['name'],'root':d['root'],'groups':{g:pr(v) for g,v in d['groups'].items()},'targets':{t:pr(x['inputs']) for t,x in d['targets'].items() if t in('build','check','test','lint')},'edges':d['edges']},ensure_ascii=False)+'\n')
