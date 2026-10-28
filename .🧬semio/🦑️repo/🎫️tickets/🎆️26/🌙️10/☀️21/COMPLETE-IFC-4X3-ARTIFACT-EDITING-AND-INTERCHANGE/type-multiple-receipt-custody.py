from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=p.read_text(encoding='utf8')
a=s.index('            28 if self.state.row_part < 7 => {',s.index('fn advance_inner'));b=s.index('            28 => {',a)
s=s[:a]+'''            28 if self.state.row_part < 7 => {
                let mut buffer=[0;20];let text=self.row_part_text(&mut buffer)?;let part=self.state.row_part;let reserve=text.len();let bytes=if part==6{0}else{text.len()};let mut literal=[0u8;20];if part!=6{literal[..bytes].copy_from_slice(text.as_bytes());}
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>()+20+2*bytes,capacity_bytes:reserve,release_bytes:0,depth:1};
                if construct_row_string(&mut self.row_parts.as_mut().unwrap().strings[part],reserve,std::str::from_utf8(&literal[..bytes]).expect("ASCII reference metadata"),physical,cx)?{self.state.row_part+=1;}
            }
''' + s[b:]
s=s.replace('demand.copy_bytes += size_of::<String>() + if self.state.row_part == 6 { 0 } else { text.len() };','demand.copy_bytes += size_of::<String>() + 20 + if self.state.row_part == 6 { 0 } else { 2*text.len() };')
a=s.index('impl<C: IfcDetailsProfile> IfcTypeInputSourceWork<C> {')
s=s[:a]+'''fn construct_row_string(output:&mut Option<String>,reserve:usize,text:&str,physical:OriginalConstructionDemand,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
    admit_original_construction(cx,output,physical,|_|{let mut value=String::new();value.try_reserve_exact(reserve).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type guide row text"))?;value.push_str(text);let capacity=value.capacity();Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})
}
''' + s[a:]
s=s.rstrip()+'\n#[cfg(test)]\n#[path = "type-guidance-tests.rs"]\nmod tests;\n';data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
