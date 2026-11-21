"""📦 Publish reviewed authoring fields after checking their candidate against the current source."""
import json, sys
from pathlib import Path
from runpy import run_path
T=Path(__file__).resolve().parent
def fields(original,value,keys,wrapped):
    helpers=run_path(str(T/'r5-range-gaps-lib.py'))
    begin=helpers['key_value'](original,'asset',0,len(original))[0] if wrapped else original.index('{')
    pos=begin+1;decoder=json.JSONDecoder();spans=[]
    while True:
        while original[pos].isspace() or original[pos]==',':pos+=1
        if original[pos]=='}':break
        key,stop=decoder.raw_decode(original,pos);lo=original.index(':',stop)+1
        while original[lo].isspace():lo+=1
        _,hi=decoder.raw_decode(original,lo)
        if key in keys:spans.append((lo,hi,key))
        pos=hi
    assert set(keys)=={q[2] for q in spans}
    result=original
    for lo,hi,key in reversed(spans):
        line=original[original.rfind('\n',0,lo)+1:lo]
        result=result[:lo]+helpers['render'](value[key],len(line)-len(line.lstrip()),2)+result[hi:]
    return result
def publish(author,cat,candidate):
    source=T/'volumes'/cat.parent.parent.name.split('️')[-1]/(cat.parent.name+'.json')
    if not source.exists():source=next((T/'volumes').glob('*/'+cat.parent.name+'.json'))
    before=cat.read_text();sb=source.read_text()
    doc=json.loads(before);src=json.loads(sb);base=doc['asset'];tested=json.loads(candidate.read_text())['asset']
    assert src['volumes']==base['volumes'],'source has concurrent modifications'
    old=dict(base);old={key:json.dumps(value) for key,value in old.items()}
    author(base);keys=[k for k,v in base.items()if json.dumps(v)!=old[k]]
    assert all(base[k]==tested[k] for k in keys),'candidate and current authoring differ'
    if not keys:return
    a=fields(before,base,keys,True);b=fields(sb,base,['volumes'],False)
    assert json.loads(a)==doc and json.loads(b)['volumes']==base['volumes']
    assert before==cat.read_text() and sb==source.read_text(),'source changed during formatting'
    cat.write_text(a);source.write_text(b)
    print(cat.parent.name,'installed',', '.join(keys))
if __name__=='__main__':
    m=run_path(str(T/sys.argv[1]));publish(m['author'],m['CAT'],T/'🗑️generated'/sys.argv[2])
