//! 🛫️ Controlled canonical physical Text emission.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::{RecordValue,RecordSpec,RecordLayout,FieldSpec,FieldValue,Shape,DslValue,Number,ExprValue,WireValue,WireNode,JoinMode,TextError};
use semio_framework_value::native_encoding::NativeEncodeControl;

/// 🖨️ Emits a declared physical record through caller-owned output admission.
pub fn print_controlled(value:&RecordValue,spec:&RecordSpec,mode:JoinMode,maximum_output_bytes:usize,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{
    control.scoped_stage(|control|->Result<_,ValueError>{
        control.checkpoint()?;let mut measure=Emitter::new(mode,None,maximum_output_bytes);measure.record(value,spec,control)?;measure.newline(control)?;
        control.charge(measure.bytes)?;let mut output=String::new();output.try_reserve_exact(measure.bytes).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"native Text output allocation failed"))?;
        let mut writer=Emitter::new(mode,Some(output),maximum_output_bytes);writer.record(value,spec,control)?;writer.newline(control)?;
        if writer.bytes!=measure.bytes{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native Text emission disagrees with measured output"))}Ok(writer.output.take().unwrap())
    })
}

/// 🧮️ Emits a literal expression under the same owned output and recursive work control.
pub fn print_expr_controlled(value:&ExprValue,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{control.scoped_stage(|control|->Result<_,ValueError>{control.checkpoint()?;let mut measure=Emitter::new(JoinMode::Inline,None,usize::MAX);measure.expression(value,0,control)?;control.charge(measure.bytes)?;let mut output=String::new();output.try_reserve_exact(measure.bytes).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"native expression output allocation failed"))?;let mut writer=Emitter::new(JoinMode::Inline,Some(output),usize::MAX);writer.expression(value,0,control)?;if writer.bytes!=measure.bytes{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native expression emission changed after admission"))}Ok(writer.output.unwrap())})}

#[derive(Clone,Copy)]
struct RecordTextStep {kind:u8,field:usize,state:u8,position:usize,quoted:bool}
semio_framework_value::artifact_retire_leaf!(RecordTextStep);

#[derive(Clone,Copy,Default)]
struct RecordKeyHeap {building:bool,build:usize,end:usize,limit:usize,root:usize,child:usize,candidate:usize,position:usize,phase:u8}
impl RecordKeyHeap {
    fn new(length:usize)->Self{Self{building:true,build:length/2,end:length,limit:length,..Default::default()}}
    fn step<T>(&mut self,items:&mut [(String,T)])->bool {
        match self.phase {
            0=>{if self.build>0{self.build-=1;self.root=self.build;self.phase=2;}else{self.building=false;self.phase=1;}},
            1=>{if self.end<=1{return true}self.end-=1;items.swap(0,self.end);self.root=0;self.limit=self.end;self.phase=2;},
            2=>{self.child=self.root*2+1;if self.child>=self.limit{self.phase=if self.building{0}else{1};}else{self.candidate=self.root;self.position=0;self.phase=3;}},
            3=>{let left=items[self.candidate].0.as_bytes();let right=items[self.child].0.as_bytes();let order=match(left.get(self.position),right.get(self.position)){(Some(left),Some(right))=>{let order=left.cmp(right);if order==std::cmp::Ordering::Equal{self.position+=1;return false}order},(None,None)=>std::cmp::Ordering::Equal,(None,_)=>std::cmp::Ordering::Less,(_,None)=>std::cmp::Ordering::Greater};if order==std::cmp::Ordering::Less{self.candidate=self.child;}if self.child==self.root*2+1&&self.child+1<self.limit{self.child+=1;self.position=0;}else{self.phase=4;}},
            4=>{if self.candidate==self.root{self.phase=if self.building{0}else{1};}else{items.swap(self.root,self.candidate);self.root=self.candidate;self.phase=2;}},
            _=>unreachable!(),
        }false
    }
}
#[derive(Clone,Copy,Default)]
struct RecordIntrinsicFrame {state:u8,index:usize,position:usize,quoted:bool}
semio_framework_value::artifact_retire_leaf!(RecordKeyHeap,RecordIntrinsicFrame);

#[derive(Clone,Copy,Default)]
struct RecordCompoundStep {field:usize,depth:usize,index:usize,variant:usize,position:usize,state:u8,kind:u8,heap:RecordKeyHeap}
semio_framework_value::artifact_retire_leaf!(RecordCompoundStep);

#[derive(Clone,Copy,Default)]
struct RecordWireStep {field:usize,state:u8,part:u8}
semio_framework_value::artifact_retire_leaf!(RecordWireStep);

/// 🧵️ Retains one projected record and its physical emitter across caller work grants.
pub struct RetainedRecordWriter {
    source:Option<RecordValue>,spec:Option<RecordSpec>,emitter:Emitter,mode:JoinMode,phase:u8,
    order:Vec<usize>,offsets:[usize;256],planning:u8,index:usize,rank:usize,positional:usize,last_present:Option<usize>,
    field_index:usize,field_state:u8,keyword_done:bool,text:Option<RecordTextStep>,
    intrinsic_field:Option<usize>,intrinsic_depth:usize,intrinsic_member:Option<(u8,usize,usize)>,intrinsic_frames:Vec<RecordIntrinsicFrame>,intrinsic_path:Vec<usize>,
    compound:Option<RecordCompoundStep>,wire:Option<RecordWireStep>,nested:Option<Box<RetainedRecordWriter>>,nested_mode:bool,table_row:bool,child_spec:Option<RecordSpec>,depth:usize,retiring:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}
impl RetainedRecordWriter {
    /// 🌱️ Takes the existing projected record and declared schema without cloning their source.
    pub fn new(source:RecordValue,spec:RecordSpec,mode:JoinMode,maximum_output_bytes:usize)->Self {
        Self{source:Some(source),spec:Some(spec),emitter:Emitter::new(mode,None,maximum_output_bytes),mode,phase:0,order:Vec::new(),offsets:[0;256],planning:0,index:0,rank:0,positional:0,last_present:None,field_index:0,field_state:0,keyword_done:false,text:None,intrinsic_field:None,intrinsic_depth:0,intrinsic_member:None,intrinsic_frames:Vec::new(),intrinsic_path:Vec::new(),compound:None,wire:None,nested:None,nested_mode:false,table_row:false,child_spec:None,depth:1,retiring:None}
    }
    /// 📍️ Reports admitted physical bytes and the writing phase.
    pub fn progress(&self)->(usize,bool){(self.emitter.bytes,self.phase==2)}
    /// ⏱️ Advances explicit schema-order or scalar-character transitions through the same emitter.
    pub fn step(&mut self,maximum_units:usize,control:&mut NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{
        for _ in 0..maximum_units {control.checkpoint()?;if self.phase==3{return Ok(None)}if self.advance_unit(control)?{return Ok(self.emitter.output.take())}control.step()?;}Ok(None)
    }
    fn advance_unit(&mut self,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if let Some(retirement)=self.retiring.as_mut(){if retirement.terminal_is_empty(){self.retiring.take();}else{retirement.close_step(1,1)?;}return Ok(false)}
        if self.nested.is_some(){let child=self.nested.as_mut().unwrap();child.advance_unit(control)?;if child.phase==3{self.finish_nested(control)?;}return Ok(false)}
        if self.phase==0{self.plan(control)?;return Ok(false)}
        if self.advance_record(control)?{
            if self.nested_mode{self.phase=3;return Ok(false)}
            if self.phase==1{control.charge(self.emitter.bytes)?;let mut output=String::new();output.try_reserve_exact(self.emitter.bytes).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"retained Text output allocation failed"))?;self.emitter=Emitter::new(self.mode,Some(output),self.emitter.maximum_output_bytes);self.field_index=0;self.field_state=0;self.keyword_done=false;self.phase=2;}else{self.phase=3;return Ok(true)}
        }Ok(false)
    }
    fn plan(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let spec=self.spec.as_ref().unwrap();match self.planning {
            0=>{self.order=control.allocate_vec(spec.fields.len())?;self.planning=if self.table_row{7}else{1};},
            1=>{if let Some(field)=spec.fields.get(self.index){if let Some(position)=field.position.filter(|_|!field.is_call_name){self.offsets[position as usize]+=1;self.positional+=1;}self.index+=1;}else{self.index=0;self.rank=0;self.planning=if self.positional==0{4}else{2};}},
            2=>{if self.index<256{let next=self.rank+self.offsets[self.index];self.offsets[self.index]=self.rank;self.rank=next;self.index+=1;}else{self.index=0;self.rank=0;self.planning=3;}},
            3=>{if self.order.len()<self.positional{self.order.push(0);}else{self.planning=4;}},
            4=>{if let Some(field)=spec.fields.get(self.index){if let Some(position)=field.position.filter(|_|!field.is_call_name){let slot=&mut self.offsets[position as usize];self.order[*slot]=self.index;*slot+=1;}self.index+=1;}else{self.index=0;self.planning=5;}},
            5=>{if self.index<self.positional{let field=&spec.fields[self.order[self.index]];if self.source.as_ref().unwrap().get(field.id).is_some_and(|value|!matches!(value,FieldValue::Absent)){self.last_present=Some(self.index);}self.index+=1;}else{self.index=0;self.rank=0;self.planning=6;}},
            6=>{if self.rank==5{self.phase=1;return Ok(())}if let Some(field)=spec.fields.get(self.index){if field.position.is_none()&&!field.key.is_empty()&&!field.is_call_name&&usize::from(super::keyed_field_rank(&field.shape))==self.rank{self.order.push(self.index);}self.index+=1;}else{self.index=0;self.rank+=1;}},
            7=>{if self.index<spec.fields.len(){self.order.push(self.index);self.index+=1;}else{self.positional=spec.fields.len();self.keyword_done=true;self.phase=1;}},
            _=>unreachable!(),
        }Ok(())
    }
    fn text_at<'a>(source:&'a RecordValue,spec:&'a RecordSpec,text:RecordTextStep,compound:Option<RecordCompoundStep>,wire:Option<RecordWireStep>)->Result<&'a str,ValueError>{
        if text.kind==0{return spec.keyword.as_deref().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"retained keyword is absent"))}
        if matches!(text.kind,1|4){return Ok(&spec.fields[text.field].key)}
        if matches!(text.kind,9|10){let step=wire.unwrap();let Some(FieldValue::Wire(wire))=source.get(spec.fields[step.field].id)else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained Wire source is absent"))};return Self::wire_text_at(wire,step.part)}
        if matches!(text.kind,5..=8){let step=compound.unwrap();let value=Self::compound_value(source,spec,step)?;return match(text.kind,value){(5,FieldValue::List(items))|(8,FieldValue::Tuple(items))=>if let FieldValue::Text(text)=&items[step.index]{Ok(text)}else{Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained list text is absent"))},(6,FieldValue::Map(items))=>Ok(&items[step.index].0),(7,FieldValue::Map(items))=>if let FieldValue::Text(text)=&items[step.index].1{Ok(text)}else{Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained map text is absent"))},_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained text container is absent"))}}
        match source.get(spec.fields[text.field].id){Some(FieldValue::Text(text))|Some(FieldValue::Value(DslValue::String(text)))=>Ok(text),_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained text source is absent"))}
    }
    fn advance_text(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let mut frame=self.text.unwrap();let text=Self::text_at(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),frame,self.compound,self.wire)?;
        match frame.state {
            0=>{if frame.kind<9{self.emitter.begin_atom(control)?;}frame.quoted=frame.kind==3||(matches!(frame.kind,2|5..=9)&&!bare_ident_prefix(text));frame.state=if matches!(frame.kind,2|5..=9)&&!frame.quoted{1}else{2};},
            1=>{if let Some(character)=text[frame.position..].chars().next(){let next=text[frame.position+character.len_utf8()..].chars().next();let valid=if frame.position==0{character.is_alphabetic()||character=='_'}else{(character.is_alphanumeric()||matches!(character,'_'|'-'|'.'|'/'))&&!(character=='-'&&next.is_some_and(|next|matches!(next,'>'|'-')))};if valid{frame.position+=character.len_utf8();}else{frame.quoted=true;frame.position=0;frame.state=2;}}else{frame.quoted=frame.position==0;frame.position=0;frame.state=2;}},
            2=>{if frame.quoted{self.emitter.raw("\"",control)?;}frame.state=3;},
            3=>{if let Some(character)=text[frame.position..].chars().next(){if frame.quoted{self.emitter.quoted_character(character,control)?;}else{let mut scalar=[0;4];self.emitter.raw(character.encode_utf8(&mut scalar),control)?;}frame.position+=character.len_utf8();}else{frame.state=4;}},
            4=>{if frame.quoted{self.emitter.raw("\"",control)?;}if matches!(frame.kind,1|6){self.emitter.raw("=",control)?;self.emitter.glued=true;}self.text=None;return Ok(());},
            _=>unreachable!(),
        }self.text=Some(frame);Ok(())
    }
    fn intrinsic_at<'a>(source:&'a RecordValue,spec:&RecordSpec,field:usize,member:Option<(u8,usize,usize)>,path:&[usize])->Result<&'a DslValue,ValueError>{
        let mut field_value=source.get(spec.fields[field].id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"retained intrinsic field is absent"))?;
        if let Some((kind,index,depth))=member.filter(|(kind,_,_)|*kind!=3){for _ in 0..depth{let FieldValue::Block(inner)=field_value else{unreachable!()};field_value=inner;}field_value=match(kind,field_value){(1,FieldValue::List(items))|(1,FieldValue::Tuple(items))=>&items[index],(2,FieldValue::Map(items))=>&items[index].1,_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained intrinsic member is absent"))};}
        let mut value=if member.is_some_and(|(kind,_,_)|kind==3){let FieldValue::Wire(wire)=field_value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained Wire intrinsic source is absent"))};&wire.properties}else{let FieldValue::Value(value)=field_value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained intrinsic source is absent"))};value};
        for &index in path {value=match value{DslValue::Array(items)=>&items[index],DslValue::Object(items)=>&items[index].1,_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained intrinsic path is invalid"))};}Ok(value)
    }
    fn start_intrinsic(&mut self,field:usize,member:Option<(u8,usize,usize)>,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.intrinsic_depth=self.depth+member.map_or(2,|(kind,_,depth)|if kind==3{2}else{3+depth});Self::check_depth(self.intrinsic_depth)?;if self.intrinsic_frames.capacity()==0{self.intrinsic_frames=control.allocate_vec(129)?;self.intrinsic_path=control.allocate_vec(128)?;}self.intrinsic_field=Some(field);self.intrinsic_member=member;self.intrinsic_frames.push(RecordIntrinsicFrame::default());Ok(())}
    fn advance_intrinsic(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let field=self.intrinsic_field.unwrap();let mut frame=*self.intrinsic_frames.last().unwrap();
        let value=Self::intrinsic_at(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),field,self.intrinsic_member,&self.intrinsic_path)?;let mut complete=false;let mut child=None;
        match value {
            value@(DslValue::Null|DslValue::Bool(_)|DslValue::Number(_))=>{self.emitter.intrinsic(value,control)?;complete=true;},
            DslValue::String(text)=>match frame.state {
                0=>{self.emitter.begin_atom(control)?;self.emitter.raw("\"",control)?;frame.state=1;},
                1=>{if let Some(character)=text[frame.position..].chars().next(){self.emitter.quoted_character(character,control)?;frame.position+=character.len_utf8();}else{frame.state=2;}},
                2=>{self.emitter.raw("\"",control)?;complete=true;},_=>unreachable!(),
            },
            DslValue::Bytes(bytes)=>match frame.state {
                0=>{self.emitter.begin_atom(control)?;self.emitter.raw("bytes64(\"",control)?;frame.state=1;},
                1=>{if frame.position<bytes.len(){const A:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";let part=&bytes[frame.position..(frame.position+3).min(bytes.len())];let a=part[0];let b=part.get(1).copied().unwrap_or(0);let c=part.get(2).copied().unwrap_or(0);let word=[A[(a>>2)as usize],A[((a&3)<<4|b>>4)as usize],if part.len()>1{A[((b&15)<<2|c>>6)as usize]}else{b'='},if part.len()>2{A[(c&63)as usize]}else{b'='}];self.emitter.raw(std::str::from_utf8(&word).unwrap(),control)?;frame.position+=part.len();}else{frame.state=2;}},
                2=>{self.emitter.raw("\")",control)?;complete=true;},_=>unreachable!(),
            },
            DslValue::Array(items)=>match frame.state {
                0=>{self.emitter.atom("[",control)?;frame.state=1;},
                1=>{if frame.index<items.len(){child=Some(frame.index);frame.index+=1;}else{self.emitter.atom("]",control)?;complete=true;}},_=>unreachable!(),
            },
            DslValue::Object(items)=>match frame.state {
                0=>{frame.state=11;},
                11=>{self.emitter.open(control)?;frame.state=12;},
                12=>{if frame.index==items.len(){self.emitter.close(control)?;complete=true;}else{frame.position=0;frame.quoted=false;frame.state=13;}},
                13=>{self.emitter.begin_atom(control)?;frame.quoted=!bare_ident_prefix(&items[frame.index].0);frame.state=if frame.quoted{15}else{14};},
                14=>{let text=&items[frame.index].0;if let Some(character)=text[frame.position..].chars().next(){let next=text[frame.position+character.len_utf8()..].chars().next();let valid=if frame.position==0{character.is_alphabetic()||character=='_'}else{(character.is_alphanumeric()||matches!(character,'_'|'-'|'.'|'/'))&&!(character=='-'&&next.is_some_and(|next|matches!(next,'>'|'-')))};if valid{frame.position+=character.len_utf8();}else{frame.quoted=true;frame.position=0;frame.state=15;}}else{frame.quoted=frame.position==0;frame.position=0;frame.state=15;}},
                15=>{if frame.quoted{self.emitter.raw("\"",control)?;}frame.state=16;},
                16=>{let text=&items[frame.index].0;if let Some(character)=text[frame.position..].chars().next(){if frame.quoted{self.emitter.quoted_character(character,control)?;}else{let mut scalar=[0;4];self.emitter.raw(character.encode_utf8(&mut scalar),control)?;}frame.position+=character.len_utf8();}else{frame.state=17;}},
                17=>{if frame.quoted{self.emitter.raw("\"",control)?;}self.emitter.raw("=",control)?;self.emitter.glued=true;child=Some(frame.index);frame.index+=1;frame.state=12;},_=>unreachable!(),
            },
        }
        if complete{self.intrinsic_frames.pop();if self.intrinsic_path.pop().is_none(){self.intrinsic_field=None;self.intrinsic_member=None;}return Ok(())}
        *self.intrinsic_frames.last_mut().unwrap()=frame;
        if let Some(index)=child {Self::check_depth(self.intrinsic_depth+self.intrinsic_path.len()+1)?;self.intrinsic_path.push(index);self.intrinsic_frames.push(RecordIntrinsicFrame::default());}Ok(())
    }
    fn compound_shape<'a>(spec:&'a RecordSpec,step:RecordCompoundStep)->Result<&'a Shape,ValueError>{let mut shape=&spec.fields[step.field].shape;for _ in 0..step.depth{let Shape::Block(inner)=shape else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained block schema path is absent"))};shape=inner;}Ok(shape)}
    fn compound_value<'a>(source:&'a RecordValue,spec:&RecordSpec,step:RecordCompoundStep)->Result<&'a FieldValue,ValueError>{let mut value=source.get(spec.fields[step.field].id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"retained compound source is absent"))?;for _ in 0..step.depth{let FieldValue::Block(inner)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained block source path is absent"))};value=inner;}Ok(value)}
    fn compound_value_mut<'a>(source:&'a mut RecordValue,spec:&RecordSpec,step:RecordCompoundStep)->Result<&'a mut FieldValue,ValueError>{let mut value=source.fields.get_mut(&spec.fields[step.field].id).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"retained compound source is absent"))?;for _ in 0..step.depth{let FieldValue::Block(inner)=value else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"retained block source path is absent"))};value=inner;}Ok(value)}
    fn start_nested(&mut self,producer:super::RecordSpecProducer,mut step:RecordCompoundStep,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let next_depth=self.depth+step.depth+match step.kind{3=>if step.depth==0&&self.spec.as_ref().unwrap().fields[step.field].position.is_none(){0}else{1},4|5=>3,_=>2};Self::check_depth(next_depth)?;
        let spec=if matches!(step.kind,3..=5){self.child_spec.take().unwrap()}else{producer.encode(control)?};control.charge(std::mem::size_of::<Self>())?;
        let value=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?;
        let record=match step.kind{1=>{let FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained nested record source is absent"))};std::mem::take(record)},2=>{let FieldValue::Statements(items)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained statement source is absent"))};std::mem::take(&mut items[step.index].1)},3|4=>{let FieldValue::List(items)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained list source is absent"))};let FieldValue::Record(record)=&mut items[step.index]else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained table row is not a record"))};std::mem::take(record)},5=>{let FieldValue::Map(items)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained map source is absent"))};let FieldValue::Record(record)=&mut items[step.index].1 else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained map member is not a record"))};std::mem::take(record)},_=>unreachable!()};
        let maximum=self.emitter.maximum_output_bytes;let mut child=Self::new(record,spec,self.mode,maximum);child.nested_mode=true;child.table_row=step.kind==3;child.depth=next_depth;child.intrinsic_frames=std::mem::take(&mut self.intrinsic_frames);child.intrinsic_path=std::mem::take(&mut self.intrinsic_path);child.emitter=std::mem::replace(&mut self.emitter,Emitter::new(self.mode,None,maximum));step.state=2;self.compound=Some(step);self.nested=Some(Box::new(child));Ok(())
    }
    fn finish_nested(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let mut child=self.nested.take().unwrap();let mut step=self.compound.unwrap();let record=child.source.take().unwrap();let value=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?;
        match step.kind{1=>{let FieldValue::Record(target)=value else{unreachable!()};*target=record;step.state=3;},2=>{let FieldValue::Statements(items)=value else{unreachable!()};items[step.index].1=record;step.index+=1;step.variant=0;step.position=0;step.state=1;},3|4=>{let FieldValue::List(items)=value else{unreachable!()};let FieldValue::Record(target)=&mut items[step.index]else{unreachable!()};*target=record;self.child_spec=child.spec.take();step.index+=1;step.state=if step.kind==3{14}else{10};},5=>{let FieldValue::Map(items)=value else{unreachable!()};let FieldValue::Record(target)=&mut items[step.index].1 else{unreachable!()};*target=record;self.child_spec=child.spec.take();step.index+=1;step.state=10;},_=>unreachable!()}
        self.intrinsic_frames=std::mem::take(&mut child.intrinsic_frames);self.intrinsic_path=std::mem::take(&mut child.intrinsic_path);
        self.emitter=std::mem::replace(&mut child.emitter,Emitter::new(self.mode,None,self.emitter.maximum_output_bytes));if matches!(step.kind,4|5){self.emitter.atom("}",control)?;}self.compound=Some(step);self.retiring=Some(semio_framework_value::retirement::owned_retirement(*child));Ok(())
    }
    fn check_depth(depth:usize)->Result<(),ValueError>{if depth>64{Err(ValueError::new(ValueRefusalKind::DepthLimit,"retained Text exceeds native encoding depth limit"))}else{Ok(())}}
    fn advance_compound(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let mut step=self.compound.unwrap();let shape=Self::compound_shape(self.spec.as_ref().unwrap(),step)?;
        if step.state!=3{Self::check_depth(self.depth+step.depth+1)?;}
        if step.state==0{if matches!(shape,Shape::Block(_)){self.emitter.open(control)?;step.depth+=1;}else{if matches!(shape,Shape::Map(_)){let FieldValue::Map(items)=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained Map is not map"))};step.heap=RecordKeyHeap::new(items.len());}step.state=1;}self.compound=Some(step);return Ok(())}
        if step.state==3{if step.depth>0{self.emitter.close(control)?;step.depth-=1;}else{self.compound=None;if let Some(spec)=self.child_spec.take(){self.retiring=Some(semio_framework_value::retirement::owned_retirement(spec));}return Ok(())}self.compound=Some(step);return Ok(())}
        match shape {
            Shape::Record(producer)=>{step.kind=1;self.start_nested(*producer,step,control)?;},
            Shape::List(inner)=>{
                let FieldValue::List(items)=Self::compound_value(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),step)?else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained List is not list"))};
                match step.state {
                    1=>{self.emitter.atom("[",control)?;step.state=10;self.compound=Some(step);},
                    10=>{if step.index==items.len(){self.emitter.atom("]",control)?;step.state=3;self.compound=Some(step);}else{Self::check_depth(self.depth+step.depth+2)?;match(&items[step.index],inner.as_ref()){
                        (FieldValue::Record(_),Shape::Record(producer))=>{if self.child_spec.is_none(){self.child_spec=Some(producer.encode(control)?);}self.emitter.atom("{",control)?;step.kind=4;self.start_nested(*producer,step,control)?;},
                        (FieldValue::Text(_),_)=>{self.text=Some(RecordTextStep{kind:5,field:step.field,state:0,position:0,quoted:false});step.state=11;self.compound=Some(step);},
                        (FieldValue::Value(_),Shape::Value)=>{step.state=11;self.compound=Some(step);self.start_intrinsic(step.field,Some((1,step.index,step.depth)),control)?;},
                        (value@(FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Enum(_)),_)=>{self.emitter.shape(value,inner,control)?;step.index+=1;self.compound=Some(step);},
                        _=>return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained nested list shape is absent")),
                    }}},
                    11=>{step.index+=1;step.state=10;self.compound=Some(step);},_=>unreachable!(),
                }
            },
            Shape::Map(inner)=>{
                match step.state {
                    1=>{let FieldValue::Map(items)=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?else{unreachable!()};if step.heap.step(items){self.emitter.open(control)?;step.state=10;}self.compound=Some(step);},
                    10=>{let FieldValue::Map(items)=Self::compound_value(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),step)?else{unreachable!()};if step.index==items.len(){self.emitter.close(control)?;step.state=3;}else{self.text=Some(RecordTextStep{kind:6,field:step.field,state:0,position:0,quoted:false});step.state=11;}self.compound=Some(step);},
                    11=>{let FieldValue::Map(items)=Self::compound_value(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),step)?else{unreachable!()};Self::check_depth(self.depth+step.depth+2)?;match(&items[step.index].1,inner.as_ref()){
                        (FieldValue::Record(_),Shape::Record(producer))=>{if self.child_spec.is_none(){self.child_spec=Some(producer.encode(control)?);}self.emitter.atom("{",control)?;step.kind=5;self.start_nested(*producer,step,control)?;},
                        (FieldValue::Value(_),Shape::Value)=>{step.state=12;self.compound=Some(step);self.start_intrinsic(step.field,Some((2,step.index,step.depth)),control)?;},
                        (FieldValue::Text(_),_)=>{self.text=Some(RecordTextStep{kind:7,field:step.field,state:0,position:0,quoted:false});step.state=12;self.compound=Some(step);},
                        (value@(FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Enum(_)),_)=>{self.emitter.shape(value,inner,control)?;step.index+=1;step.state=10;self.compound=Some(step);},
                        _=>return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained nested map shape is absent")),
                    }},
                    12=>{step.index+=1;step.state=10;self.compound=Some(step);},_=>unreachable!(),
                }
            },
            Shape::Tuple(_,_)|Shape::Coord(_)|Shape::Dir|Shape::Dim(_)|Shape::Range=>{
                let FieldValue::Tuple(items)=Self::compound_value(self.source.as_ref().unwrap(),self.spec.as_ref().unwrap(),step)?else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained Tuple is not tuple"))};
                match step.state {
                    1=>{self.emitter.begin_atom(control)?;match shape{Shape::Coord(_)=>self.emitter.raw("@",control)?,Shape::Dir=>self.emitter.raw("^",control)?,Shape::Range=>self.emitter.raw("(",control)?,_=>{}}step.state=10;self.compound=Some(step);},
                    10=>{if step.index==items.len(){if matches!(shape,Shape::Range){self.emitter.raw(")",control)?;}step.state=3;}else{Self::check_depth(self.depth+step.depth+if matches!(shape,Shape::Tuple(_,_)){2}else{1})?;if step.index>0{self.emitter.raw(match shape{Shape::Dim(_)=>"x",Shape::Range if step.index==1=>"..",_=>","},control)?;}match &items[step.index]{FieldValue::Float(value)=>self.emitter.float(*value,control)?,FieldValue::Int(value)=>self.emitter.number(value,control)?,FieldValue::UInt(value)=>self.emitter.number(value,control)?,FieldValue::Bool(value)=>self.emitter.raw(if *value{"true"}else{"false"},control)?,FieldValue::Text(_)=>{self.emitter.glued=true;self.text=Some(RecordTextStep{kind:8,field:step.field,state:0,position:0,quoted:false});step.state=11;self.compound=Some(step);return Ok(())},_=>return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained tuple item is absent"))}step.index+=1;}self.compound=Some(step);},
                    11=>{step.index+=1;step.state=10;self.compound=Some(step);},_=>unreachable!(),
                }
            },
            Shape::Table(producer)=>{
                match step.state {
                    1=>{self.child_spec=Some(producer.encode(control)?);self.emitter.atom("[",control)?;self.emitter.glued=true;step.variant=0;step.position=0;step.state=10;self.compound=Some(step);},
                    10=>{let spec=self.child_spec.as_ref().unwrap();if step.variant==spec.fields.len(){self.emitter.glued=true;self.emitter.atom("]",control)?;self.emitter.open(control)?;step.state=14;}else{self.emitter.begin_atom(control)?;step.position=0;step.state=11;}self.compound=Some(step);},
                    11=>{let key=&self.child_spec.as_ref().unwrap().fields[step.variant].key;if let Some(character)=key[step.position..].chars().next(){let mut scalar=[0;4];self.emitter.raw(character.encode_utf8(&mut scalar),control)?;step.position+=character.len_utf8();}else{self.emitter.raw(":",control)?;step.state=12;}self.compound=Some(step);},
                    12=>{self.emitter.raw(super::shape_type_name(&self.child_spec.as_ref().unwrap().fields[step.variant].shape),control)?;step.variant+=1;step.state=10;self.compound=Some(step);},
                    14=>{let FieldValue::List(items)=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained Table is not List"))};if step.index==items.len(){self.emitter.close(control)?;step.state=3;self.compound=Some(step);}else{self.emitter.newline(control)?;step.kind=3;self.start_nested(*producer,step,control)?;}},
                    _=>unreachable!(),
                }
            },
            Shape::Statements(variants)=>{
                let FieldValue::Statements(items)=Self::compound_value_mut(self.source.as_mut().unwrap(),self.spec.as_ref().unwrap(),step)?else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained Statements is not statements"))};
                if step.index==items.len(){step.state=3;self.compound=Some(step);return Ok(())}
                let (keyword,_)=variants.get(step.variant).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"retained statement keyword is undeclared"))?;let source=items[step.index].0.as_bytes();let expected=keyword.as_bytes();match(source.get(step.position),expected.get(step.position)){(Some(left),Some(right))if left==right=>{step.position+=1;self.compound=Some(step);},(None,None)=>{self.emitter.newline(control)?;step.kind=2;let producer=variants[step.variant].1;self.start_nested(producer,step,control)?;},_=>{step.variant+=1;step.position=0;self.compound=Some(step);}}
            },
            _=>return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained compound shape continuation is absent")),
        }Ok(())
    }
    fn wire_text_at(wire:&WireValue,part:u8)->Result<&str,ValueError>{let text=match part{0=>Some(wire.from.id.as_str()),1=>wire.from.kind.as_deref(),2=>wire.from.port.as_deref(),3=>wire.edge_label.id.as_deref(),4=>wire.edge_label.kind.as_deref(),5=>wire.edge.as_ref().map(|(_,to)|to.id.as_str()),6=>wire.edge.as_ref().and_then(|(_,to)|to.kind.as_deref()),7=>wire.edge.as_ref().and_then(|(_,to)|to.port.as_deref()),_=>None};text.ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"retained Wire text piece is absent"))}
    fn advance_wire(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let mut step=self.wire.unwrap();let Some(FieldValue::Wire(wire))=self.source.as_ref().unwrap().get(self.spec.as_ref().unwrap().fields[step.field].id)else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"retained Wire source is absent"))};let mut part=None;
        match step.state {
            0=>{self.emitter.begin_atom(control)?;part=Some(0);step.state=1;},
            1=>{if wire.from.kind.is_some(){self.emitter.raw(":",control)?;part=Some(1);}step.state=2;},
            2=>{if wire.from.port.is_some(){self.emitter.raw("@",control)?;part=Some(2);}step.state=3;},
            3=>{if let Some((directed,_))=wire.edge.as_ref(){if wire.edge_label.is_empty(){self.emitter.raw(if *directed{"->"}else{"--"},control)?;step.state=6;}else{self.emitter.raw(" -",control)?;step.state=4;}}else{step.state=9;}},
            4=>{if wire.edge_label.id.is_some(){part=Some(3);}step.state=5;},
            5=>{if wire.edge_label.kind.is_some(){self.emitter.raw(":",control)?;part=Some(4);}step.state=51;},
            51=>{self.emitter.raw(if wire.edge.as_ref().unwrap().0{">"}else{"-"},control)?;step.state=6;},
            6=>{part=Some(5);step.state=7;},
            7=>{if wire.edge.as_ref().unwrap().1.kind.is_some(){self.emitter.raw(":",control)?;part=Some(6);}step.state=8;},
            8=>{if wire.edge.as_ref().unwrap().1.port.is_some(){self.emitter.raw("@",control)?;part=Some(7);}step.state=9;},
            9=>{if !matches!(&wire.properties,DslValue::Object(items)if items.is_empty()){self.start_intrinsic(step.field,Some((3,0,0)),control)?;}step.state=10;},
            10=>{self.wire=None;return Ok(())},_=>unreachable!(),
        }
        if let Some(part)=part{step.part=part;self.text=Some(RecordTextStep{kind:if matches!(part,3|4){10}else{9},field:step.field,state:0,position:0,quoted:false});}self.wire=Some(step);Ok(())
    }
    fn advance_record(&mut self,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.text.is_some(){self.advance_text(control)?;return Ok(false)}
        if self.intrinsic_field.is_some(){self.advance_intrinsic(control)?;return Ok(false)}
        if self.compound.is_some(){self.advance_compound(control)?;return Ok(false)}
        if self.wire.is_some(){self.advance_wire(control)?;return Ok(false)}
        let spec=self.spec.as_ref().unwrap();let source=self.source.as_ref().unwrap();
        if spec.layout==RecordLayout::Call{return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained Call emission continuation is absent"))}
        if !self.keyword_done {self.keyword_done=true;if spec.keyword.is_some(){self.text=Some(RecordTextStep{kind:0,field:0,state:0,position:0,quoted:false});}return Ok(false)}
        if self.field_index==self.order.len(){if !self.nested_mode{self.emitter.newline(control)?;}return Ok(true)}
        let index=self.order[self.field_index];let field=&spec.fields[index];let value=source.get(field.id).filter(|value|!matches!(value,FieldValue::Absent));
        let Some(value)=value else{if self.table_row||(self.field_index<self.positional&&self.last_present.is_some_and(|last|self.field_index<last)){self.emitter.atom("_",control)?;}self.field_index+=1;self.field_state=0;return Ok(false)};
        Self::check_depth(self.depth+1)?;if matches!(value,FieldValue::Value(_)){Self::check_depth(self.depth+2)?;}
        if self.field_state==0 {self.field_state=1;if !self.table_row&&field.position.is_none()&&!matches!(field.shape,Shape::Statements(_)){let block=matches!(field.shape,Shape::Block(_)|Shape::Table(_));if block{self.emitter.newline(control)?;}self.text=Some(RecordTextStep{kind:if block{4}else{1},field:index,state:0,position:0,quoted:false});}return Ok(false)}
        if self.field_state==1 {
            self.field_state=2;match value {
                FieldValue::Text(_) if !matches!(field.shape,Shape::Embed(_)|Shape::EmbedFrom(_))=>self.text=Some(RecordTextStep{kind:2,field:index,state:0,position:0,quoted:false}),
                FieldValue::Value(DslValue::String(_))=>self.text=Some(RecordTextStep{kind:3,field:index,state:0,position:0,quoted:true}),
                FieldValue::Value(_)=>{self.start_intrinsic(index,None,control)?;},
                FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Enum(_)=>self.emitter.shape(value,&field.shape,control)?,
                FieldValue::Wire(_)=>{self.wire=Some(RecordWireStep{field:index,..Default::default()});},
                FieldValue::Record(_)|FieldValue::Block(_)|FieldValue::Statements(_)|FieldValue::List(_)|FieldValue::Map(_)|FieldValue::Tuple(_)=>{self.compound=Some(RecordCompoundStep{field:index,..Default::default()});},
                _=>return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"retained compound emission continuation is absent")),
            }return Ok(false)
        }
        self.field_index+=1;self.field_state=0;Ok(false)
    }
}
impl semio_framework_value::retirement::RetireOwned for RetainedRecordWriter {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::{RetireOwned,sequence,erased_cursor};let mut fields=vec![self.source.retirement(),self.spec.retirement(),self.order.retirement(),self.text.retirement(),self.intrinsic_frames.retirement(),self.intrinsic_path.retirement(),self.emitter.output.retirement(),self.compound.retirement(),self.wire.retirement(),self.nested.retirement(),self.child_spec.retirement()];if let Some(retiring)=self.retiring{fields.push(erased_cursor(retiring));}sequence(fields)}
}

struct Emitter { prospective:bool, output:Option<String>, bytes:usize, maximum_output_bytes:usize, mode:JoinMode, indent:usize, line_open:bool, glued:bool }
impl Emitter {
    fn new(mode:JoinMode,output:Option<String>,maximum_output_bytes:usize)->Self{Self{prospective:false,output,bytes:0,maximum_output_bytes,mode,indent:0,line_open:false,glued:false}}
    fn raw(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(text.len()).filter(|length|*length<=if self.prospective{self.maximum_output_bytes}else{control.maximum_bytes().min(self.maximum_output_bytes)}).ok_or_else(||ValueError::new(if self.prospective{ValueRefusalKind::WorkLimit}else{ValueRefusalKind::OwnershipLimit},"native Text output exceeds caller limit"))?;if let Some(output)=self.output.as_ref(){if self.bytes>output.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"native Text output exceeded measured allocation"))}}if text.len()<=65536{if let Some(output)=self.output.as_mut(){output.push_str(text);}return Ok(())}control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(text.len())?;let mut start=0;while start<text.len(){let mut end=start.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}if let Some(output)=self.output.as_mut(){output.push_str(&text[start..end]);}control.advance(end-start)?;start=end;}Ok(())})}
    fn begin_atom(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if !self.line_open{if self.mode==JoinMode::Document{for _ in 0..self.indent{self.raw("  ",control)?;}}self.line_open=true;}else if !self.glued{self.raw(" ",control)?;}self.glued=false;Ok(())}
    fn atom(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.begin_atom(control)?;self.raw(text,control)}
    fn newline(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if self.mode==JoinMode::Document&&self.line_open{self.raw("\n",control)?;self.line_open=false;}Ok(())}
    fn open(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if self.mode==JoinMode::Inline{self.atom("{",control)}else{self.raw(if self.glued{"{"}else{" {"},control)?;self.raw("\n",control)?;self.line_open=false;self.glued=false;self.indent=self.indent.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native Text indentation overflow"))?;Ok(())}}
    fn close(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if self.mode==JoinMode::Inline{self.atom("}",control)}else{self.newline(control)?;self.indent=self.indent.saturating_sub(1);for _ in 0..self.indent{self.raw("  ",control)?;}self.raw("}\n",control)}}
    fn inline<T>(&mut self,control:&mut NativeEncodeControl<'_>,operation:impl FnOnce(&mut Self,&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let previous=(self.mode,self.indent,self.line_open,self.glued);self.mode=JoinMode::Inline;self.indent=0;self.line_open=false;self.glued=false;let result=operation(self,control);(self.mode,self.indent,self.line_open,self.glued)=previous;result}
    fn number(&mut self,value:impl std::fmt::Display,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{let mut buffer=StackText{bytes:[0;1088],length:0};std::fmt::write(&mut buffer,format_args!("{value}")).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"native numeric format exceeded bounded stack buffer"))?;self.raw(buffer.text(),control)}
    fn float(&mut self,value:f64,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if value.is_nan(){let mut buffer=StackText{bytes:[0;1088],length:0};std::fmt::write(&mut buffer,format_args!("nan64_{:016x}",value.to_bits())).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"native IEEE formatting failed"))?;self.raw(buffer.text(),control)}else{self.number(value,control)}}
    fn quoted_character(&mut self,character:char,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{match character{'\\'=>self.raw("\\\\",control)?,'\"'=>self.raw("\\\"",control)?,'\n'=>self.raw("\\n",control)?,'\r'=>self.raw("\\r",control)?,'\t'=>self.raw("\\t",control)?,value if (value as u32)<32=>{self.raw("\\u{",control)?;let mut buffer=StackText{bytes:[0;1088],length:0};std::fmt::write(&mut buffer,format_args!("{:x}",value as u32)).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"native escape formatting failed"))?;self.raw(buffer.text(),control)?;self.raw("}",control)?;},value=>{let mut bytes=[0;4];self.raw(value.encode_utf8(&mut bytes),control)?;}}Ok(())}
    fn quoted(&mut self,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.raw("\"",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(text.len())?;for character in text.chars(){self.quoted_character(character,control)?;control.advance(character.len_utf8())?;}Ok(())})?;self.raw("\"",control)}
    fn text(&mut self,text:&str,force_quote:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if !force_quote&&bare_ident(text,control)?{control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(text.len())?;for character in text.chars(){let mut bytes=[0;4];self.raw(character.encode_utf8(&mut bytes),control)?;control.advance(character.len_utf8())?;}Ok(())})}else{self.quoted(text,control)}}
    fn octets(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";self.raw("\"",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(bytes.len())?;for part in bytes.chunks(3){let a=part[0];let b=part.get(1).copied().unwrap_or(0);let c=part.get(2).copied().unwrap_or(0);let word=[ALPHABET[(a>>2)as usize],ALPHABET[((a&3)<<4|b>>4)as usize],if part.len()>1{ALPHABET[((b&15)<<2|c>>6)as usize]}else{b'='},if part.len()>2{ALPHABET[(c&63)as usize]}else{b'='}];self.raw(std::str::from_utf8(&word).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"invalid native base64 alphabet"))?,control)?;control.advance(part.len())?;}Ok(())})?;self.raw("\"",control)}
    fn key(&mut self,key:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.begin_atom(control)?;self.text(key,false,control)?;self.raw("=",control)?;self.glued=true;Ok(())}
    fn field_key(&mut self,key:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.begin_atom(control)?;self.raw(key,control)?;self.raw("=",control)?;self.glued=true;Ok(())}
    fn verbatim(&mut self,language:&str,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.begin_atom(control)?;if self.mode==JoinMode::Inline{return self.quoted(text,control)}self.raw("```",control)?;self.raw(language,control)?;self.raw("\n",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(text.len())?;for character in text.chars(){let mut bytes=[0;4];self.raw(character.encode_utf8(&mut bytes),control)?;control.advance(character.len_utf8())?;}Ok(())})?;if !text.is_empty(){self.raw("\n",control)?;}self.raw("```",control)}
    fn record(&mut self,value:&RecordValue,spec:&RecordSpec,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_depth(64,|control|->Result<_,ValueError>{control.checkpoint()?;if spec.layout==RecordLayout::Call{let name=spec.fields.iter().find(|field|field.is_call_name).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"native Call has no name field"))?;let Some(FieldValue::Text(text))=value.get(name.id)else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native Call name is not Text"))};self.begin_atom(control)?;self.text(text,false,control)?;self.atom("=",control)?;if let Some(keyword)=spec.keyword.as_deref(){self.atom(keyword,control)?;}self.raw("(",control)?;self.inline(control,|writer,control|writer.fields(value,spec,control))?;self.raw(")",control)}else{if let Some(keyword)=spec.keyword.as_deref(){self.atom(keyword,control)?;}self.fields(value,spec,control)}})}
    fn fields(&mut self,value:&RecordValue,spec:&RecordSpec,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
        let mut fields=control.allocate_vec::<&FieldSpec>(spec.fields.len())?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(spec.fields.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native positional order workload overflow"))?)?;let mut offsets=[0usize;256];let mut count=0usize;for field in &spec.fields{if let Some(position)=field.position.filter(|_|!field.is_call_name){offsets[position as usize]+=1;count+=1;}control.step()?;}let mut offset=0;for slot in &mut offsets{let next=offset+*slot;*slot=offset;offset=next;}if count>0{fields.resize(count,&spec.fields[0]);}for field in &spec.fields{if let Some(position)=field.position.filter(|_|!field.is_call_name){let slot=&mut offsets[position as usize];fields[*slot]=field;*slot+=1;}control.step()?;}Ok(())})?;
        for(index,field)in fields.iter().enumerate(){match value.get(field.id){Some(value)if !matches!(value,FieldValue::Absent)=>self.shape(value,&field.shape,control)?,_=>{if fields[index+1..].iter().any(|field|matches!(value.get(field.id),Some(value)if !matches!(value,FieldValue::Absent))){self.atom("_",control)?;}}}}
        fields.clear();control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(spec.fields.len().checked_mul(5).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native field order workload overflow"))?)?;for rank in 0..5{for field in &spec.fields{if field.position.is_none()&&!field.key.is_empty()&&!field.is_call_name&&super::keyed_field_rank(&field.shape)==rank{fields.push(field);}control.step()?;}}Ok(())})?;
        for field in fields{let Some(item)=value.get(field.id).filter(|value|!matches!(value,FieldValue::Absent))else{continue};match &field.shape{
            Shape::EmbedFrom(key)=>{let language=spec.fields.iter().find(|field|field.key==*key).and_then(|field|value.get(field.id)).and_then(|value|if let FieldValue::Text(text)=value{Some(text.as_str())}else{None}).unwrap_or("plaintext");self.newline(control)?;self.field_key(&field.key,control)?;if let FieldValue::Text(text)=item{self.verbatim(language,text,control)?;}else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native EmbedFrom is not Text"))}},
            Shape::Statements(_)=>self.shape(item,&field.shape,control)?,
            Shape::Block(_)=>{self.newline(control)?;self.atom(&field.key,control)?;self.shape(item,&field.shape,control)?;},
            Shape::Table(make)=>{self.newline(control)?;self.atom(&field.key,control)?;let FieldValue::List(items)=item else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native Table is not List"))};self.table(items,&make.encode(control)?,true,control)?;},
            _=>{self.field_key(&field.key,control)?;self.shape(item,&field.shape,control)?;}
        }}Ok(())
    }
    fn table(&mut self,items:&[FieldValue],spec:&RecordSpec,columnar:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.atom("[",control)?;if columnar{self.glued=true;for field in &spec.fields{self.begin_atom(control)?;self.raw(&field.key,control)?;self.raw(":",control)?;self.raw(super::shape_type_name(&field.shape),control)?;}self.glued=true;self.atom("]",control)?;self.open(control)?;}control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for item in items{let FieldValue::Record(record)=item else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native table item is not Record"))};if columnar{self.newline(control)?;for field in &spec.fields{match record.get(field.id){Some(value)if !matches!(value,FieldValue::Absent)=>{if matches!(field.shape,Shape::Record(_)){self.atom("{",control)?;self.glued=true;self.shape(value,&field.shape,control)?;self.glued=true;self.atom("}",control)?;}else{self.shape(value,&field.shape,control)?;}},_=>self.atom("_",control)?}}}else{self.atom("{",control)?;self.glued=true;self.record(record,spec,control)?;self.glued=true;self.atom("}",control)?;}control.step()?;}Ok(())})?;if columnar{self.close(control)}else{self.atom("]",control)}}
    fn shape(&mut self,value:&FieldValue,shape:&Shape,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_depth(64,|control|->Result<_,ValueError>{control.checkpoint()?;match(value,shape){
        (FieldValue::Float(value),Shape::Quantity(unit)|Shape::Angle(unit))=>{self.begin_atom(control)?;self.float(*value,control)?;self.raw(unit.symbol,control)},
        (FieldValue::UInt(value),Shape::Count)=>{self.begin_atom(control)?;self.raw("x",control)?;self.number(value,control)},
        (FieldValue::Tuple(items),Shape::Coord(_)|Shape::Dir|Shape::Dim(_)|Shape::Range)=>{self.begin_atom(control)?;match shape{Shape::Coord(_)=>self.raw("@",control)?,Shape::Dir=>self.raw("^",control)?,Shape::Range=>self.raw("(",control)?,_=>{}}for(index,item)in items.iter().enumerate(){if index>0{self.raw(match shape{Shape::Dim(_)=>"x",Shape::Range if index==1=>"..",_=>","},control)?;}let FieldValue::Float(value)=item else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native numeric tuple element is not Float"))};self.float(*value,control)?;}if matches!(shape,Shape::Range){self.raw(")",control)?;}Ok(())},
        (FieldValue::Expr(value),Shape::Expr)=>{self.begin_atom(control)?;self.raw("(",control)?;self.expression(value,0,control)?;self.raw(")",control)},
        (FieldValue::Text(value),Shape::Embed(language))=>self.verbatim(language,value,control),
        (FieldValue::Text(value),Shape::EmbedFrom(_))=>self.verbatim("plaintext",value,control),
        (FieldValue::Bool(value),_)=>self.atom(if *value{"true"}else{"false"},control),
        (FieldValue::Int(value),_)=>{self.begin_atom(control)?;self.number(value,control)},
        (FieldValue::UInt(value),_)=>{self.begin_atom(control)?;self.number(value,control)},
        (FieldValue::Float(value),_)=>{self.begin_atom(control)?;self.float(*value,control)},
        (FieldValue::Text(value),_)=>{self.begin_atom(control)?;self.text(value,false,control)},
        (FieldValue::Bytes64(value),_)=>{self.begin_atom(control)?;self.octets(value,control)},
        (FieldValue::Enum(ordinal),Shape::Enum(variants))=>{let(keyword,_)=variants.iter().find(|(_,value)|value==ordinal).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"native enum ordinal is undeclared"))?;self.atom(keyword,control)},
        (FieldValue::Tuple(items),Shape::Tuple(element,_))=>{self.begin_atom(control)?;for(index,item)in items.iter().enumerate(){if index>0{self.raw(",",control)?;}self.inline(control,|writer,control|writer.shape(item,element,control))?;}Ok(())},
        (FieldValue::List(items),Shape::Table(make))=>self.table(items,&make.encode(control)?,false,control),
        (FieldValue::List(items),Shape::List(element))=>{self.atom("[",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for item in items{control.scoped_stage(|control|->Result<_,ValueError>{if matches!(element.as_ref(),Shape::Record(_)){self.atom("{",control)?;self.shape(item,element,control)?;self.atom("}",control)}else{self.shape(item,element,control)}})?;control.step()?;}Ok(())})?;self.atom("]",control)},
        (FieldValue::Record(value),Shape::Record(make))=>self.record(value,&make.encode(control)?,control),
        (FieldValue::Block(value),Shape::Block(shape))=>{self.open(control)?;self.shape(value,shape,control)?;self.close(control)},
        (FieldValue::Statements(items),Shape::Statements(variants))=>control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for(keyword,record)in items{let(_,make)=variants.iter().find(|(name,_)|name==keyword).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"native statement keyword is undeclared"))?;self.newline(control)?;control.scoped_stage(|control|self.record(record,&make.encode(control)?,control))?;control.step()?;}Ok(())}),
        (FieldValue::Map(items),Shape::Map(shape))=>{let mut sorted=control.allocate_vec::<&(String,FieldValue)>(items.len())?;sorted.extend(items.iter());sort_keys(&mut sorted,|item|item.0.as_str(),control)?;self.open(control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(sorted.len())?;for(key,value)in sorted{control.scoped_stage(|control|->Result<_,ValueError>{self.key(key,control)?;if matches!(shape.as_ref(),Shape::Record(_)){self.atom("{",control)?;self.shape(value,shape,control)?;self.atom("}",control)}else{self.shape(value,shape,control)}})?;control.step()?;}Ok(())})?;self.close(control)},
        (FieldValue::Value(value),Shape::Value)=>self.intrinsic(value,control),
        (FieldValue::Wire(value),Shape::Wire)=>self.wire(value,control),
        _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"native field value disagrees with declared shape"))
    }})}
    fn intrinsic(&mut self,value:&DslValue,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_depth(64,|control|->Result<_,ValueError>{control.checkpoint()?;match value{
        DslValue::Null=>self.atom("null",control),DslValue::Bool(value)=>self.atom(if *value{"true"}else{"false"},control),
        DslValue::Number(Number::Int(value)) if *value>=0=>{self.begin_atom(control)?;self.raw("int64(",control)?;self.number(value,control)?;self.raw(")",control)},
        DslValue::Number(Number::Int(value))=>{self.begin_atom(control)?;self.number(value,control)},DslValue::Number(Number::UInt(value))=>{self.begin_atom(control)?;self.number(value,control)},
        DslValue::Number(Number::Float(value))=>{self.begin_atom(control)?;let start=self.bytes;self.float(*value,control)?;if value.is_finite(){let mut buffer=StackText{bytes:[0;1088],length:0};std::fmt::write(&mut buffer,format_args!("{value}")).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"native float formatting failed"))?;if !buffer.text().contains(['.','e','E'])&&self.bytes>start{self.raw(".0",control)?;}}Ok(())},
        DslValue::String(value)=>{self.begin_atom(control)?;self.quoted(value,control)},
        DslValue::Bytes(value)=>{self.begin_atom(control)?;self.raw("bytes64(",control)?;self.octets(value,control)?;self.raw(")",control)},
        DslValue::Array(items)=>{self.atom("[",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for value in items{control.scoped_stage(|control|self.intrinsic(value,control))?;control.step()?;}Ok(())})?;self.atom("]",control)},
        DslValue::Object(items)=>{self.open(control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for(key,value)in items{control.scoped_stage(|control|->Result<_,ValueError>{self.key(key,control)?;self.intrinsic(value,control)})?;control.step()?;}Ok(())})?;self.close(control)}
    }})}
    fn expression(&mut self,value:&ExprValue,minimum:u8,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_depth(64,|control|->Result<_,ValueError>{control.checkpoint()?;let precedence=match value{ExprValue::Neg(_)=>3,ExprValue::Binary(operator,_,_)=>operator.precedence(),_=>255};let bracket=precedence<minimum;if bracket{self.raw("(",control)?;}match value{ExprValue::Num(value)=>self.float(*value,control)?,ExprValue::Var(name)=>{self.text(name,false,control)?;},ExprValue::Neg(value)=>{self.raw("-",control)?;self.expression(value,4,control)?;},ExprValue::Binary(operator,left,right)=>{self.expression(left,precedence,control)?;self.raw(" ",control)?;self.raw(operator.symbol(),control)?;self.raw(" ",control)?;self.expression(right,precedence+1,control)?;},ExprValue::Call(name,items)=>{self.text(name,false,control)?;self.raw("(",control)?;control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(items.len())?;for(index,item)in items.iter().enumerate(){if index>0{self.raw(", ",control)?;}control.scoped_stage(|control|self.expression(item,0,control))?;control.step()?;}Ok(())})?;self.raw(")",control)?;}}if bracket{self.raw(")",control)?;}Ok(())})}
    fn wire_text(&mut self,value:&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if bare_ident(value,control)?{self.raw(value,control)}else{self.quoted(value,control)}}
    fn wire_node(&mut self,value:&WireNode,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.wire_text(&value.id,control)?;if let Some(kind)=value.kind.as_deref(){self.raw(":",control)?;self.wire_text(kind,control)?;}if let Some(port)=value.port.as_deref(){self.raw("@",control)?;self.wire_text(port,control)?;}Ok(())}
    fn wire(&mut self,value:&WireValue,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{self.begin_atom(control)?;self.wire_node(&value.from,control)?;if let Some((directed,to))=value.edge.as_ref(){if value.edge_label.is_empty(){self.raw(if *directed{"->"}else{"--"},control)?;}else{self.raw(" -",control)?;if let Some(id)=value.edge_label.id.as_deref(){self.raw(id,control)?;}if let Some(kind)=value.edge_label.kind.as_deref(){self.raw(":",control)?;self.raw(kind,control)?;}self.raw(if *directed{">"}else{"-"},control)?;}self.wire_node(to,control)?;}if !matches!(&value.properties,DslValue::Object(items)if items.is_empty()){self.intrinsic(&value.properties,control)?;}Ok(())}
}

struct StackText { bytes:[u8;1088], length:usize }
impl StackText {fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).unwrap()}}
impl std::fmt::Write for StackText{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}

fn sort_keys<T>(items:&mut Vec<&T>,key:impl Fn(&T)->&str,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if items.len()<2{return Ok(())}let mut scratch=control.allocate_vec::<&T>(items.len())?;scratch.extend(items.iter().copied());control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(0)?;let mut width=1usize;while width<items.len(){for start in (0..items.len()).step_by(width.saturating_mul(2)){let middle=start.saturating_add(width).min(items.len());let end=middle.saturating_add(width).min(items.len());let(mut left,mut right)=(start,middle);for output in start..end{let from_left=if left==middle{false}else if right==end{true}else{compare_keys(key(items[left]),key(items[right]),control)?!=std::cmp::Ordering::Greater};scratch[output]=if from_left{let value=items[left];left+=1;value}else{let value=items[right];right+=1;value};control.step()?;}}std::mem::swap(items,&mut scratch);width=width.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"native key sort width overflow"))?;}Ok(())})}
fn compare_keys(left:&str,right:&str,control:&mut NativeEncodeControl<'_>)->Result<std::cmp::Ordering,ValueError>{control.scoped_stage(|control|->Result<_,ValueError>{let count=left.len().min(right.len());control.begin_stage(count)?;let mut start=0;while start<count{let end=start.saturating_add(65536).min(count);let order=left.as_bytes()[start..end].cmp(&right.as_bytes()[start..end]);control.advance(end-start)?;if order!=std::cmp::Ordering::Equal{return Ok(order)}start=end;}Ok(left.len().cmp(&right.len()))})}

fn bare_ident_prefix(text:&str)->bool{if text.len()>semio_framework_diagnostic::Limits::default().max_bytes||matches!(text,"_"|"true"|"false"|"null"|"nan"|"inf"){return false}for(prefix,width)in[("nan64_",16),("nan32_",8)]{if let Some(rest)=text.strip_prefix(prefix){if rest.len()>=width&&rest.as_bytes()[..width].iter().all(u8::is_ascii_hexdigit){return false}}}for prefix in ["inf","nan"]{if let Some(suffix)=text.strip_prefix(prefix){if semio_framework_dsl::unit_by_symbol(suffix).is_some(){return false}}}true}

fn bare_ident(text:&str,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{control.scoped_stage(|control|->Result<_,ValueError>{control.begin_stage(text.len())?;if !bare_ident_prefix(text){return Ok(false)}let mut characters=text.chars().peekable();let Some(first)=characters.next()else{return Ok(false)};if !(first.is_alphabetic()||first=='_'){return Ok(false)}control.advance(first.len_utf8())?;while let Some(value)=characters.next(){if !(value.is_alphanumeric()||matches!(value,'_'|'-'|'.'|'/'))||(value=='-'&&characters.peek().is_some_and(|next|matches!(*next,'>'|'-'))){return Ok(false)}control.advance(value.len_utf8())?;}Ok(true)})}

#[path = "🫳️borrowed/🦀️.rs"]
mod borrowed;
pub(super) use borrowed::measure;
