from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=p.read_text(encoding='utf8')
a=s.index('fn row_capacity(');b=s.index('fn decimal(',a)
s=s[:a]+'''#[derive(semio_framework_value::RetireOwned)]
struct RowParts { strings: [Option<String>;7], outer: Option<Vec<(String,DslValue)>>, inner: Option<Vec<(String,DslValue)>> }
''' + s[b:]
s=s.replace('    offset: usize,\n}', '    offset: usize,\n    row_part: usize,\n}')
s=s.replace('    copied_row: ManuallyDrop<Option<DslValue>>,','    copied_row: ManuallyDrop<Option<DslValue>>,\n    row_parts: ManuallyDrop<Option<RowParts>>,')
s=s.replace('            copied_row: ManuallyDrop::new(None),','            copied_row: ManuallyDrop::new(None),\n            row_parts: ManuallyDrop::new(None),')
s=s.replace('''            14 => { let (index,text)=self.copying_row()?;let mut buffer=[0;20];demand.capacity_bytes=row_capacity(decimal(index,&mut buffer),text);demand.copy_bytes+=demand.capacity_bytes-text.len()+size_of::<DslValue>(); }''','''            14 => demand.copy_bytes+=size_of::<RowParts>(),
            28 if self.state.row_part<7=>{let mut buffer=[0;20];let text=self.row_part_text(&mut buffer)?;demand.capacity_bytes=text.len();demand.copy_bytes+=size_of::<String>()+if self.state.row_part==6{0}else{text.len()};},
            28=>{demand.capacity_bytes=2*size_of::<(String,DslValue)>();demand.copy_bytes+=size_of::<Vec<(String,DslValue)>>();},
            29=>{demand.copy_bytes+=2*size_of::<RowParts>()+4*size_of::<(String,DslValue)>();},''')
s=s.replace('''    pub fn original_request_received(&self) -> bool {''','''    fn row_part_text<'a>(&'a self,buffer:&'a mut [u8;20])->Result<&'a str,ValueError>{
        let (index,text)=self.copying_row()?;Ok(match self.state.row_part{0=>"index",1|4=>"value",2=>"kind",3=>"reference",5=>decimal(index,buffer),6=>text,_=>return Err(invalid())})
    }
    pub fn original_request_received(&self) -> bool {''')
a=s.index('            14 => {',s.index('fn advance_inner'));b=s.index('            20 => {',a)
s=s[:a]+'''            14 => {
                metadata(cx,base+size_of::<RowParts>(),||{*self.row_parts=Some(RowParts{strings:Default::default(),outer:None,inner:None});self.state.row_part=0;self.state.phase=28;})?;
            }
            28 if self.state.row_part<7 => {
                let mut buffer=[0;20];let text=self.row_part_text(&mut buffer)?;let part=self.state.row_part;let bytes=if part==6{0}else{text.len()};let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>()+bytes,capacity_bytes:text.len(),release_bytes:0,depth:1};let mut output=None;
                if admit_original_construction(cx,&mut output,physical,|_|{let mut value=String::new();value.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type guide row text"))?;if part!=6{value.push_str(text);}let capacity=value.capacity();Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.row_parts.as_mut().unwrap().strings[part]=output;self.state.row_part+=1;}
            }
            28 => {
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Vec<(String,DslValue)>>(),capacity_bytes:2*size_of::<(String,DslValue)>(),release_bytes:0,depth:1};let parts=self.row_parts.as_mut().ok_or_else(invalid)?;let target=if self.state.row_part==7{&mut parts.outer}else{&mut parts.inner};
                if admit_original_construction(cx,target,physical,|_|{let mut values=Vec::new();values.try_reserve_exact(2).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type guide row fields"))?;let capacity=values.capacity()*size_of::<(String,DslValue)>();Ok((values,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.row_part+=1;if self.state.row_part==9{self.state.phase=29;}}
            }
            29 => {
                metadata(cx,base+2*size_of::<RowParts>()+4*size_of::<(String,DslValue)>(),||{
                    let mut parts=self.row_parts.take().unwrap();let mut inner=parts.inner.take().unwrap();let mut outer=parts.outer.take().unwrap();
                    inner.push((parts.strings[2].take().unwrap(),DslValue::String(parts.strings[3].take().unwrap())));inner.push((parts.strings[4].take().unwrap(),DslValue::String(parts.strings[6].take().unwrap())));
                    outer.push((parts.strings[0].take().unwrap(),DslValue::String(parts.strings[5].take().unwrap())));outer.push((parts.strings[1].take().unwrap(),DslValue::Object(inner)));*self.copied_row=Some(DslValue::Object(outer));self.state.offset=0;self.state.phase=20;
                })?;
            }
''' + s[b:]
s=s.replace('self.copied_row.is_none() && self.plan.map','self.copied_row.is_none() && self.row_parts.is_none() && self.plan.map')
s=s.replace('        owner!(&self.copied_row, DslValue);','        owner!(&self.row_parts, RowParts);\n        owner!(&self.copied_row, DslValue);')
s=s.replace('        owner!(&mut self.copied_row);','        owner!(&mut self.row_parts);\n        owner!(&mut self.copied_row);')
data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
