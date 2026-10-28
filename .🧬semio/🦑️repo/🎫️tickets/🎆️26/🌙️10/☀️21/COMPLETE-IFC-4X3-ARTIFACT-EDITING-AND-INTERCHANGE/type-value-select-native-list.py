from pathlib import Path
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
p=D/'type_input_list_copy.rs';s=p.read_text(encoding='utf-8')
s=s.replace('struct State{phase:', '#[derive(Clone,Copy,PartialEq,Eq)]\npub(super) enum Mode{Real,Integer}\nstruct State{phase:')
s=s.replace('#[derive(Clone,Copy,Default)]\n#[derive(Clone,Copy,PartialEq,Eq)]', '#[derive(Clone,Copy,PartialEq,Eq)]')
s=s.replace('struct State{phase:', '#[derive(Clone,Copy,Default)]\nstruct State{phase:')
s=s.replace('Option<(usize,usize,usize)>','Option<(usize,usize,usize,Mode)>')
a=s.index('pub(super) fn word(');b=s.index('impl ListCopy{',a)
s=s[:a]+'''pub(super) fn word(source:&[DslValue],index:usize,mode:Mode)->Result<&str,ValueError>{let Some(value)=source.get(index)else{return Ok("")};match mode{Mode::Real=>super::super::type_input::real_word(value),Mode::Integer=>super::super::type_input::integer_word(value)}.ok_or_else(invalid)}
fn literal(mode:Mode,index:usize)->Result<&'static str,ValueError>{["kind",if mode==Mode::Real{"real"}else{"integer"},"value","bits"].get(index).copied().ok_or_else(invalid)}
'''+s[b:]
s=s.replace('source:&[DslValue],target:usize)', 'source:&[DslValue],target:usize,mode:Mode)')
s=s.replace('source:&[DslValue],target:usize,cx:', 'source:&[DslValue],target:usize,mode:Mode,cx:')
s=s.replace('self.valid(source,target)?','self.valid(source,target,mode)?')
s=s.replace('self.demands(source,target)?','self.demands(source,target,mode)?')
s=s.replace('(source.as_ptr()as usize,source.len(),target)', '(source.as_ptr()as usize,source.len(),target,mode)')
s=s.replace('3=>demand.capacity_bytes=size_of::<(String,DslValue)>()','3 if mode==Mode::Real=>demand.capacity_bytes=size_of::<(String,DslValue)>()')
s=s.replace('word(source,self.state.member)?','word(source,self.state.member,mode)?')
s=s.replace('["kind","real","value","bits"][self.state.string]','literal(mode,self.state.string)?')
s=s.replace('2|3=>{let count=', '3 if mode==Mode::Integer=>self.state.phase=4,\n   2|3=>{let count=')
s=s.replace('4 if self.state.string<5=>{let raw=', '4 if self.state.string==3&&mode==Mode::Integer=>self.state.string=4,\n   4 if self.state.string<5=>{let raw=')
a=s.index('   6=>{');b=s.index('   7=>{',a)
s=s[:a]+'''   6=>{let parts=self.parts.as_mut().ok_or_else(invalid)?;if parts.member.is_none()||parts.root.is_none()||[0,1,2,4].iter().any(|index|parts.strings[*index].is_none())||mode==Mode::Real&&(parts.bits.is_none()||parts.strings[3].is_none()){return Err(invalid().into());}let text=DslValue::String(parts.strings[4].take().unwrap());let value=if mode==Mode::Real{let mut bits=parts.bits.take().unwrap();bits.push((parts.strings[3].take().unwrap(),text));DslValue::Object(bits)}else{text};let mut member=parts.member.take().unwrap();member.push((parts.strings[0].take().unwrap(),DslValue::String(parts.strings[1].take().unwrap())));member.push((parts.strings[2].take().unwrap(),value));parts.root.as_mut().unwrap().push(DslValue::Object(member));self.state.member+=1;self.state.string=0;self.state.phase=2;},
'''+s[b:]
p.write_text(s,encoding='utf-8',newline='\n')
p=D/'type_input_list_copy_tests.rs';s=p.read_text(encoding='utf-8')
s=s.replace('.demands(&source,3)', '.demands(&source,3,Mode::Real)').replace('.advance(&source,3,cx)', '.advance(&source,3,Mode::Real,cx)').replace('.advance(actual,target,cx)', '.advance(actual,target,Mode::Real,cx)')
s=s.replace('super::tests::type_guidance_request_tests::retire_owner(&mut caller,&mut output);', '''let mut retirement=RetainedCloneClose::default();for turn in 0..100000{if output.is_none()&&retirement.is_empty(){break;}let demand=retirement.next_owner_demands::<DslValue>(output.is_some(),&None,4096).unwrap();caller.turn(OriginalWorkFixture::stock(),|cx|cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,demand,|grant|if retirement.is_empty(){retirement.begin_granted(&mut output,grant)?.ok_or_else(invalid)}else{retirement.step_granted(grant)})?;Ok(())})).unwrap();assert!(turn<99999);}assert!(output.is_none()&&retirement.is_empty());''')
p.write_text(s,encoding='utf-8',newline='\n')
p=D/'type_input_typed_copy.rs';s=p.read_text(encoding='utf-8')
s=s.replace('use super::list_copy::ListCopy;', 'use super::list_copy::{ListCopy,Mode};')
s=s.replace('InputKind::String|InputKind::Integer', 'InputKind::String|InputKind::Integer|InputKind::Enum|InputKind::Binary')
s=s.replace('InputKind::Set|InputKind::Array', 'InputKind::Set|InputKind::Array|InputKind::List')
s=s.replace('self.kind==InputKind::Array{6}', 'matches!(self.kind,InputKind::Array|InputKind::List){6}')
s=s.replace('InputKind::Array=>12', 'InputKind::Array|InputKind::List=>12')
s=s.replace('let array=input.kind==InputKind::Array;', 'let array=matches!(input.kind,InputKind::Array|InputKind::List);')
s=s.replace('self.array_copy.demands(values,values.len())', 'self.array_copy.demands(values,values.len(),if input.kind==InputKind::List{Mode::Integer}else{Mode::Real})')
s=s.replace('copy.advance(values,values.len(),cx)', 'copy.advance(values,values.len(),if input.kind==InputKind::List{Mode::Integer}else{Mode::Real},cx)')
p.write_text(s,encoding='utf-8',newline='\n')
p=D/'type-guidance.rs';s=p.read_text(encoding='utf-8')
s=s.replace('self.list_copy.demands(source,target)', 'self.list_copy.demands(source,target,list_copy::Mode::Real)').replace('copy.advance(source,target,cx)', 'copy.advance(source,target,list_copy::Mode::Real,cx)')
p.write_text(s,encoding='utf-8',newline='\n')
print('[DEBUG] explicit original-bound INTEGER/REAL collection modes written')
