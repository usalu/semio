import sys,re,os
M=sys.argv[1]
def expand_methods(a,b,c,d):
    return ('pub fn %s(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}\n'
            '    pub fn %s(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}\n'
            '    pub fn %s(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}\n'
            '    pub fn %s(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}')%(a,b,c,d)
def expand_demand(owner):
    return ('match axis {Puzzle2dCloseAxis::Copy=>%s.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>%s.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>%s.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>%s.next_close_depth_demand()}')%((owner,)*4)
n=0
for root,_,fs in os.walk(M):
    for f in fs:
        if f!='🦀️.rs': continue
        p=os.path.join(root,f); t=open(p,encoding='utf8').read(); o=t
        if 'native_preparation_child/' in p: continue
        t=re.sub(r'close_demand_methods!\(([a-z_]+),([a-z_]+),([a-z_]+),([a-z_]+)\);',lambda m:expand_methods(*m.groups()),t)
        t=re.sub(r'close_child_demand!\(axis,([A-Za-z_.]+)\)',lambda m:expand_demand(m.group(1)),t)
        t=t.replace('{Puzzle2dCloseAxis,close_child_demand,close_demand_methods}','Puzzle2dCloseAxis')
        if t!=o: open(p,'w',encoding='utf8').write(t); n+=1
print('sites',n)
c=M+'/🎮️prepare/🧰️child/🦀️.rs'; t=open(c,encoding='utf8').read()
t=re.sub(r'macro_rules! close_child_demand \{.*?\n\}\npub\(crate\) use close_child_demand;\n\n','',t,flags=re.S)
t=re.sub(r'macro_rules! close_demand_methods \{.*?\n\}\npub\(crate\) use close_demand_methods;\n\n','',t,flags=re.S)
t=re.sub(r'close_child_demand!\(axis,owner\)',expand_demand('owner'),t)
t=re.sub(r'close_demand_methods!\(([a-z_]+),([a-z_]+),([a-z_]+),([a-z_]+)\);',lambda m:expand_methods(*m.groups()),t)
t=t.replace('pub(crate)','pub')
# move cursor impls out
m=re.search(r'macro_rules! borrowed_preparation \{.*?\n\}\n\n',t,flags=re.S); assert m
macro_text=m.group(0)
t=t.replace(macro_text,'')
for line in ('borrowed_preparation!(crate::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::Puzzle2dConnectHandlesPreparationCursor);\n','borrowed_preparation!(crate::standards::v1::subsets::any::schema::preparation::delete_node_preparation::Puzzle2dDeleteNodePreparationCursor);\n'):
    assert line in t; t=t.replace(line,'')
m2=re.search(r'\nimpl Puzzle2dNativePreparation for crate::standards::v1::subsets::any::schema::preparation::connect_handles_preparation::edge::Puzzle2dConnectEdgeCursor \{.*?\n\}\n',t,flags=re.S); assert m2
edge_impl=m2.group(0); t=t.replace(edge_impl,'')
open(c,'w',encoding='utf8').write(t.rstrip('\n')+'\n')
open(os.path.dirname(c)+'/.moved-macro','w').write(macro_text+'\n=====\n'+edge_impl)
print('child ok')
