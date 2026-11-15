import re
import subprocess
from pathlib import Path
T = Path(__file__).resolve().parent
R = Path(subprocess.check_output(['git', '-C', str(T), 'rev-parse', '--show-toplevel'], text=True).strip())
A=str(R)+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
schema=open(A+"🧬️schema/📸️snapshot/🦀️.rs").read()
sq=open(A+"🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs").read()
structs={}
for m in re.finditer(r'pub struct (\w+) \{(.*?)\n\}',schema,re.S):
    structs[m.group(1)]=[(a.group(1),a.group(2)) for l in m.group(2).split('\n') for a in [re.match(r'\s*pub (\w+): (.+),$',l)] if a]
enums={m.group(1):re.findall(r'^\s{4}(\w+),',m.group(2),re.M) for m in re.finditer(r'pub enum (\w+) \{(.*?)\n\}',schema,re.S)}
texts={}
for m in re.finditer(r'fn \w+_text\(a:([\w:]+)\)->&.static str\{match a\{(.*?)\}\}',sq):
    texts[m.group(1).split('::')[-1]]={v.split('::')[-1]:t for v,t in re.findall(r'([\w:]+)=>"([^"]*)"',m.group(2))}
for m in re.finditer(r'impl (\w+) \{\s*pub fn as_str\(self\) -> &.static str \{\s*match self \{(.*?)\}\s*\}',schema,re.S):
    texts[m.group(1)]=dict(re.findall(r'Self::(\w+) => "([^"]*)"',m.group(2)))
def lit(ty):return '&['+','.join('"%s"'%texts[ty][v] for v in enums[ty])+']'
def col(struct,f):
    names=[n for n,_ in structs[struct]];i=names.index(f);t=dict(structs[struct])[f]
    if t=='f64':return f'Real({i})'
    if t=='bool':return f'Boolean({i})'
    if t=='String':return f'Text({i})'
    if t=='u8':return f'Unsigned({i},u8::MAX as u64)'
    if t in enums:return f'Enumeration({i},{lit(t)})'
    raise Exception((struct,f,t))
def split(body):
    parts=[];d=0;cur=''
    for ch in body:
        if ch in '([':d+=1
        if ch in ')]':d-=1
        if ch==',' and d==0:parts.append(cur);cur=''
        else:cur+=ch
    parts.append(cur);return [p.strip() for p in parts]
def cellcols(struct,body,skip):
    return [col(struct,re.search(r'\w+\.(\w+)',p).group(1)) for p in split(body)[skip:]]
def stmt(table):
    return re.search(r'"%s",(?:\w+,)?&\[(.*?)\],([A-Z_]+)\)'%table,sq,re.S)
def ent(struct,table,skip=2):
    m=stmt(table);return '['+','.join(cellcols(struct,m.group(1),skip))+']',m.group(2)
top=structs['En1998Snapshot'];tid={n:i for i,(n,_) in enumerate(top)}
tables=[('buildings','En1998Building','en1998_building'),('bridges','En1998Bridge','en1998_bridge'),('assessments','En1998Assessment','en1998_assessment'),('silos','En1998Silo','en1998_silo'),('tanks','En1998Tank','en1998_tank'),('foundations','En1998Foundation','en1998_foundation'),('retaining_walls','En1998RetainingWall','en1998_retaining_wall'),('towers','En1998Tower','en1998_tower'),('placements','En1998Placement','en1998_placement'),('tie_beams','En1998TieBeam','en1998_tie_beam'),('slopes','En1998Slope','en1998_slope'),('isolation_systems','En1998IsolationSystem','en1998_isolation_system'),('pier_bases','En1998PierBase','en1998_pier_base'),('critical_regions','En1998CriticalRegion','en1998_critical_region'),('soil_layers','En1998SoilLayer','en1998_soil_layer'),('diaphragms','En1998Diaphragm','en1998_diaphragm'),('accelerograms','En1998Accelerogram','en1998_accelerogram'),('piers','En1998Pier','en1998_pier'),('bearings','En1998Bearing','en1998_bearing'),('joints','En1998Joint','en1998_joint')]
assert {t[0] for t in tables}|{'annex','site'}==set(tid),set(tid)-{t[0] for t in tables}
nest={'buildings':[('systems','En1998System','en1998_system'),('storeys','En1998Storey','en1998_storey'),('members','En1998Member','en1998_member')],'bridges':[('variables','En1998VariableAction','en1998_bridge_variable')],'towers':[('variables','En1998VariableAction','en1998_tower_variable')]}
def varcols():
    return '[Text(0),Enumeration(1,%s),Real(2)]'%lit('ImposedCategory')
def fidof(struct,f):return [n for n,_ in structs[struct]].index(f)
siteS=stmt('en1998_site')
site_cols=','.join(cellcols('En1998Site',siteS.group(1),1))
sc=len(split(siteS.group(1)))-1
L=[]
L.append(' out.insert_key_float("en1998_document",1,&n::cells(root,[Enumeration(0,&["En","De"])])?,&[])?;')
L.append(' {let c=n::cells(n::record(n::field(root,1)?)?,[%s])?;out.insert_key_float("en1998_site",1,&[Cell::Integer(1),%s],SITE)?;}'%(site_cols,','.join(f'c[{i}]' for i in range(sc))))
for lst,S,tb in tables:
    cs,const=ent(S,tb)
    nn=nest.get(lst,[])
    if not nn:
        L.append(f' for(index,v)in n::list(n::field(root,{tid[lst]})?)?.iter().enumerate(){{n::entity(out,"{tb}",1,index,&n::cells(n::record(v)?,{cs})?,{const})?;}}')
        continue
    inner=''
    for f,S2,tb2 in nn:
        if S2=='En1998VariableAction':
            c2,k2=varcols(),'VARIABLE'
            inner+=f'for(index,a)in n::list(n::field(v,{fidof(S,f)})?)?.iter().enumerate(){{n::entity(out,"{tb2}",parent,index,&n::cells(n::record(a)?,{c2})?,{k2})?;}}'
        else:
            c2,k2=ent(S2,tb2)
            if S2=='En1998Storey':
                sv=f'for(index,w)in n::list(n::field(a,{fidof("En1998Storey","variables")})?)?.iter().enumerate(){{n::entity(out,"en1998_storey_variable",storey,index,&n::cells(n::record(w)?,{varcols()})?,VARIABLE)?;}}'
                inner+=f'for(index,a)in n::list(n::field(v,{fidof(S,f)})?)?.iter().enumerate(){{let a=n::record(a)?;let storey=n::entity(out,"{tb2}",parent,index,&n::cells(a,{c2})?,{k2})?;{sv}}}'
            else:
                inner+=f'for(index,a)in n::list(n::field(v,{fidof(S,f)})?)?.iter().enumerate(){{n::entity(out,"{tb2}",parent,index,&n::cells(n::record(a)?,{c2})?,{k2})?;}}'
    L.append(f' for(index,v)in n::list(n::field(root,{tid[lst]})?)?.iter().enumerate(){{let v=n::record(v)?;let parent=n::entity(out,"{tb}",1,index,&n::cells(v,{cs})?,{const})?;{inner}}}')
# count
cnt='let mut rows=2;for id in %d..=%d{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}'%(min(tid[t[0]] for t in tables),max(tid[t[0]] for t in tables))
assert sorted(tid[t[0]] for t in tables)==list(range(2,22))
cnt+='for(id,nested)in[%s]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}'%','.join(f'({tid[l]},{fidof(S,f)})' for l,S,_ in tables for f,S2,_ in nest.get(l,[]))
nested_storey=f'for owner in n::list(n::field(root,{tid["buildings"]})?)?{{for storey in n::list(n::field(n::record(owner)?,{fidof("En1998Building","storeys")})?)?{{rows=n::add(rows,n::list(n::field(n::record(storey)?,{fidof("En1998Storey","variables")})?)?.len())?}}}}'
out='''//! 🫨️ Borrowed seismic site, building and parts-two-through-six entity cells before typed ownership.
use crate::sqlite::snapshot::*;
use store::sqlite_snapshot::artifact::RowWriter;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native as n;
use semio_s_artifact_norm_contract::sqlite_native::Column::*;
fn count(root:&R)->Result<usize,ValueError>{%s%s Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
%s
 Ok(())
}
/// 🛂️ Complete authored seismic tables and copied semantic controls.
pub fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1998Snapshot::SQLITE_SCHEMA,28,95,count(root)?,write)}
'''%(cnt,nested_storey,'\n'.join(L))
import os
d=A+"🚪️io/🪶️sqlite/📸️snapshot/🛂️admission"
os.makedirs(d,exist_ok=True)
open(d+"/🦀️.rs","w").write(out)
print(len(out))
