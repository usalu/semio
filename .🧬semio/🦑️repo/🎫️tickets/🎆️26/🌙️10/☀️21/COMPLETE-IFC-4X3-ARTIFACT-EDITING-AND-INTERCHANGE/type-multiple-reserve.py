from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=p.read_text(encoding='utf8')
s=s.replace('23 => demand.capacity_bytes=self.state.target*(size_of::<super::type_input::Input>()+size_of::<Field>()),','23 => demand.capacity_bytes=self.state.target*size_of::<super::type_input::Input>(),\n            25 => demand.capacity_bytes=self.state.target*size_of::<Field>(),')
a=s.index('            23 => {',s.index('fn advance_inner'));b=s.index('            3 => {',a)
s=s[:a]+'''            23 => {
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:self.state.target*size_of::<super::type_input::Input>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.placeholders,physical,|_|{let mut values=Vec::new();values.try_reserve_exact(self.state.target).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type required roster"))?;let capacity=values.capacity()*size_of::<super::type_input::Input>();Ok((values,RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=25;}
            }
            25 => {
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:self.state.target*size_of::<Field>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.fields,physical,|_|{let mut values=Vec::new();values.try_reserve_exact(self.state.target).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type required fields"))?;let capacity=values.capacity()*size_of::<Field>();Ok((values,RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=3;}
            }
''' + s[b:]
data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
