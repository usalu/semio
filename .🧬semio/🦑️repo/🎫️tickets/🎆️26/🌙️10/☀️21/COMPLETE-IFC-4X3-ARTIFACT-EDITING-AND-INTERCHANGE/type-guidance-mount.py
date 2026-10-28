from pathlib import Path
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
def update(path,changes):
    data=path.read_text(encoding='utf-8')
    for old,new in changes:
        assert data.count(old)==1,(path.name,old[:80],data.count(old))
        data=data.replace(old,new,1)
    encoded=data.encode('utf-8');oldsize=path.stat().st_size
    with path.open('r+b')as output:
        output.write(encoded)
        if len(encoded)<oldsize:output.write(b' '*(oldsize-len(encoded)))
update(home/'🦀️.rs',[('#[path="type_input.rs"]mod type_input;','#[path="type_input.rs"]mod type_input;\n#[path="type-guidance.rs"]mod type_guidance;\npub use type_guidance::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};')])
editor=home.parent/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs'
update(editor,[('pub use shared::IfcDetailsRenderWork;','pub use shared::IfcDetailsRenderWork;\npub use shared::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};')])
path=home/'type-guidance.rs'
update(path,[
('demand.capacity_bytes=if present{1}else{2}*size_of::<StagedFormBindingEdit>();','let missing=super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?.1.is_none();demand.capacity_bytes=if present{1}else if missing{3}else{2}*size_of::<StagedFormBindingEdit>();'),
('let count=if present{1}else{2};','let missing=super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?.1.is_none();let count=if present{1}else if missing{3}else{2};'),
('self.state.phase=if present{15}else{13};','self.state.phase=if present{15}else if missing{20}else{13};'),
('''            14|16=>demand.copy_bytes+=size_of::<StagedFormBindingEdit>(),''','''            14|16|21=>demand.copy_bytes+=size_of::<StagedFormBindingEdit>(),
            20=>{demand.capacity_bytes=2*size_of::<String>()+15;demand.copy_bytes=demand.capacity_bytes+size_of::<StagedFormBindingEdit>()+size_of::<State>();},'''),
('''            18=>return Ok(true),_=>return Err(invalid().into()),''','''            20=>{let capacity=2*size_of::<String>()+15;let physical=OriginalConstructionDemand{items:1,copy_bytes:capacity+size_of::<StagedFormBindingEdit>()+size_of::<State>(),capacity_bytes:capacity,release_bytes:0,depth:1};if admit_original_construction(cx,&mut self.edit,physical,|_|Ok((StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Set{path:vec!["value".into(),"$ifcInputs".into()],value:DslValue::Array(Vec::new())}},RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})))?{self.state.phase=21;}},
            21=>{metadata(cx,base+size_of::<StagedFormBindingEdit>(),||{self.page.as_mut().unwrap().binding.edits.push(self.edit.take().unwrap());self.state.phase=13;})?;},
            18=>return Ok(true),_=>return Err(invalid().into()),''')])
data=path.read_text(encoding='utf-8')
old='copy_bytes:capacity+size_of::<StagedFormBindingEdit>(),'
assert data.count(old)==2
update(path,[(old,data.count(old)*'')]) if False else None
data=data.replace(old,'copy_bytes:capacity+size_of::<StagedFormBindingEdit>()+size_of::<State>(),')
with path.open('r+b')as output:output.write(data.encode('utf-8'))
