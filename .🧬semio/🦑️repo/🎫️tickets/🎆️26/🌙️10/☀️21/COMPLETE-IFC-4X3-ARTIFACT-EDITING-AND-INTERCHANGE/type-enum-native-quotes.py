from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=f.read_text(encoding='utf8')
s=s.replace('fn copy_text(target: &mut String, text: &str, cx: &mut StepContext<\'_>) -> Result<bool, OriginalTurnFailure> {','''fn copy_text(target:&mut String,text:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{copy_text_extra(target,text,0,cx)}
fn copy_text_extra(target: &mut String, text: &str, extra:usize, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {''')
s=s.replace('copy_bytes: text.len() + size_of::<String>() + size_of::<State>(), capacity_bytes: text.len()', 'copy_bytes: text.len() + size_of::<String>() + size_of::<State>() + extra, capacity_bytes: text.len()',1)
a='''    fn copying_row(&self) -> Result<(usize, InputKind, &str), ValueError> {'''
s=s.replace(a,'''    fn choice_shape(&self)->Result<(usize,usize,Option<usize>),ValueError>{
        let field=self.current_field()?;let domain=self.state.domains[self.state.field].ok_or_else(invalid)?;
        let count=InputKindCursor::count::<C>(domain).ok_or_else(invalid)?;
        let start=if self.state.active_target==Some(field.index){self.state.page_offset}else{0};
        let length=if count>32{(count-start).min(31)}else{count};
        let pin=self.state.selected[self.state.field].filter(|index|*index<start||*index>=start+length);
        Ok((start,length,pin))
    }
    fn choice_word(&self)->Result<&'static str,ValueError>{let(start,length,pin)=self.choice_shape()?;let index=if self.state.option<length{start+self.state.option}else{pin.ok_or_else(invalid)?};InputKindCursor::member::<C>(self.state.domains[self.state.field].ok_or_else(invalid)?,index).ok_or_else(invalid)}
    fn choice_text(&self)->Result<&'static str,ValueError>{Ok(match self.state.choice_part{0|4=>self.choice_word()?,1=>"kind",2=>"enum",3=>"value",_=>return Err(invalid())})}
    fn choice_label(word:&'static str,locale:Locale)->&'static str{match(word,locale){("T",Locale::En)=>"True",("T",Locale::De)=>"Wahr",("F",Locale::En)=>"False",("F",Locale::De)=>"Falsch",("U",Locale::En)=>"Unknown",("U",Locale::De)=>"Unbekannt",_=>word}}
    fn nav_spec(&self)->Option<(usize,usize)>{self.state.navigation.iter().flatten().copied().nth(self.state.nav)}
    fn page_path_text<'a>(&self,part:usize,target:usize,buffer:&'a mut[u8;20])->Result<&'a str,ValueError>{Ok(match part{0=>"entities",1=>decimal(self.state.position,buffer),2=>"args",3=>decimal(target,buffer),_=>return Err(invalid())})}
'''+a)
a='''            4 => demand.copy_bytes += size_of::<Field>() + size_of::<super::type_input::Input>(),'''
s=s.replace(a,a+'''
            33=>demand.copy_bytes+=size_of::<StagedFormBindingEdit>()+size_of::<ChoiceParts>(),
            34=>{demand.copy_bytes+=size_of::<Vec<String>>();demand.capacity_bytes=4*size_of::<String>();}
            35=>{let(_,length,pin)=self.choice_shape()?;demand.capacity_bytes=(length+usize::from(pin.is_some()))*size_of::<StagedFormBindingOption>();}
            36=>{let(_,length,pin)=self.choice_shape()?;demand.copy_bytes+=size_of::<Choices>();demand.capacity_bytes=(length+usize::from(pin.is_some()))*size_of::<StagedScalarChoice>();}
            37=>{let StagedFormBindingEdit::Choice{path,..}=self.edit.as_ref().ok_or_else(invalid)? else{return Err(invalid())};if path.len()==self.state.choice_part{demand.copy_bytes+=size_of::<String>();}else{let mut buffer=[0;20];let word=match self.state.choice_part{0=>"value",1=>"$ifcInputs",2=>decimal(self.current_field()?.row,&mut buffer),3=>"value",_=>return Err(invalid())};let mut demand=text(word);demand.copy_bytes+=40;return Ok(demand);}}
            38 if self.state.choice_part<5=>{let word=self.choice_text()?;demand.capacity_bytes=word.len();demand.copy_bytes+=size_of::<String>()+word.len();}
            39=>{demand.capacity_bytes=2*size_of::<(String,DslValue)>();demand.copy_bytes+=size_of::<Vec<(String,DslValue)>>();}
            40=>demand.copy_bytes+=4*size_of::<ChoiceParts>()+2*size_of::<StagedFormBindingOption>()+size_of::<StagedScalarChoice>(),
            62=>{demand.capacity_bytes=self.state.navigation.iter().flatten().count()*size_of::<Navigation>();}
            63|70=>demand.copy_bytes+=2*size_of::<Navigation>(),
            64=>return Ok(text(token_part(&self.request.as_ref().ok_or_else(stale)?.token,self.state.part))),
            65|66=>return Ok(text("0")),
            67=>{let mut buffer=[0;20];let mut demand=text(decimal(self.nav_spec().ok_or_else(invalid)?.1,&mut buffer));demand.copy_bytes+=40;return Ok(demand);}
            68=>{demand.capacity_bytes=4*size_of::<String>();}
            69=>{let path=&self.nav_item.as_ref().ok_or_else(invalid)?.binding.page.choice.as_ref().ok_or_else(invalid)?.path;if path.len()==self.state.part{demand.copy_bytes+=size_of::<String>();}else{let mut buffer=[0;20];let mut demand=text(self.page_path_text(self.state.part,self.nav_spec().ok_or_else(invalid)?.0,&mut buffer)?);demand.copy_bytes+=40;return Ok(demand);}}
''')
s=s.replace('''            15 => {
                let mut buffer = [0; 20];
                demand.capacity_bytes''','''            15 if self.current_field()?.kind==InputKind::Enum=>{},
            15 => {
                let mut buffer = [0; 20];
                demand.capacity_bytes''',1)
b=s.encode();old=f.stat().st_size
with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
