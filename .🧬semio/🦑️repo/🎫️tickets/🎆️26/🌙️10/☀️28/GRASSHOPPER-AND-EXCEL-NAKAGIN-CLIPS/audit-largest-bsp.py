"""⏱️ Export the actual largest cutaway component and bound its kernel work."""
from pathlib import Path
import sys,importlib,json,hashlib,time
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️');K=V._fragments;T=Path(__file__).parent;G=T/'🗑️generated'
paths=[Path(V.__file__),Path(V._model.__file__),Path(V._model.__file__).with_name('🔣️.json'),Path(K.__file__)]
hashes=lambda:{p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
result={'source_start':hashes(),'rendered':False};began=time.monotonic();graph_deadline=began+120;selected=[];towards_selected=None
original_order=K.ordered_fragments;original_section=V._section_order
class Stop(Exception):pass
def intercept(faces,towards,progress=None,cancelled=None):
    global selected,towards_selected
    if len(faces)==2443:
        selected=list(faces);towards_selected=towards;raise Stop()
    return faces
K.ordered_fragments=intercept;V._restore_order=lambda items,recipes:[]
def graph_progress(row):
    if row['completed']==0 or row['completed']==row['total']:print('[DEBUG] graph '+json.dumps(row),flush=True)
def section(items,view,*args,**kwargs):
    original_section(items,view,progress=graph_progress,cancelled=lambda:time.monotonic()>graph_deadline)
    raise RuntimeError('Expected2443face component not found')
V._section_order=section
try:V.NakaginDrawing().capsule_axon(V._model.capsule_types()[0],cutaway=True,bolts=True)
except Stop:pass
result['graph_elapsed']=time.monotonic()-began
if len(selected)!=2443:raise RuntimeError('No actual2443Face component')
export={'towards':list(towards_selected),'faces':[{'source':f.source,'polygon':f.polygon,'vertices':f.vertices,'normal':f.normal} for f in selected]}
data=json.dumps(export,separators=(',',':')).encode();(G/'largest-component-faces.json').write_bytes(data);result['component_sha256']=hashlib.sha256(data).hexdigest();result['input_faces']=len(selected)
counts={'classifications':0,'true_splits':0,'epsilon_vertex_scans':0,'input_vertex_classifications':0,'maximum_classified_vertices':0,'last_partition_node':0,'maximum_pending_nodes':0,'prepared_completed':0}
original_split=K._split;original_epsilon=K._epsilon
class Deadline(Exception):pass
def counted_split(face,*args):
    counts['classifications']+=1;counts['input_vertex_classifications']+=len(face.vertices);counts['maximum_classified_vertices']=max(counts['maximum_classified_vertices'],len(face.vertices));row=original_split(face,*args)
    if row[0]==2:counts['true_splits']+=1
    return row
def counted_epsilon(points):
    def tracked():
        for point in points:
            counts['epsilon_vertex_scans']+=1;yield point
    return original_epsilon(tracked())
K._split=counted_split;K._epsilon=counted_epsilon;start=time.monotonic();deadline=start+30
journal=G/'largest-bsp-progress.jsonl'
def progress(row):
    if row['phase']=='prepare':counts['prepared_completed']=row['completed']
    if row['phase']=='partition':counts['last_partition_node']=row['completed'];counts['maximum_pending_nodes']=max(counts['maximum_pending_nodes'],row['total']-row['completed'])
    if row['completed']==1 or row['completed']==row['total'] or row['completed']%128==0:
        record={'elapsed':time.monotonic()-start,**row,'counts':dict(counts)}
        with journal.open('a',encoding='utf8') as stream:stream.write(json.dumps(record)+'\n')
        print('[DEBUG] kernel '+json.dumps(record),flush=True)
try:
    pieces=original_order(tuple(selected),towards_selected,progress=progress,cancelled=lambda:time.monotonic()>deadline)
    result['outcome']='completed';result['output_pieces']=len(pieces)
except K.FragmentCancelled:result['outcome']='cancelled-at30s'
result['kernel_elapsed']=time.monotonic()-start;result['counts']=counts;result['source_end']=hashes();result['unchanged']=result['source_start']==result['source_end'];(G/'largest-bsp-benchmark.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] result '+json.dumps(result),flush=True)
