from pathlib import Path
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
p=D/'type_input.rs'
s=p.read_text(encoding='utf-8')
a='("string"|"integer",DslValue::String(_))'
b='("string"|"integer"|"enum"|"binary",DslValue::String(_))'
assert s.count(a)==1
s=s.replace(a,b)
a='Kind::Number|super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array)||if kind=='
b='Kind::Number|super::type_input_kind::Kind::Enum|super::type_input_kind::Kind::Binary|super::type_input_kind::Kind::List|super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array)||if kind=='
assert s.count(a)==1
s=s.replace(a,b)
a='self.state.array=kind==super::type_input_kind::Kind::Array;self.state.phase=if matches!(kind,super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array){15}else{10};'
b='self.state.array=kind==super::type_input_kind::Kind::Array;self.state.integer=kind==super::type_input_kind::Kind::List;self.state.phase=if matches!(kind,super::type_input_kind::Kind::Set|super::type_input_kind::Kind::Array|super::type_input_kind::Kind::List){15}else if kind==super::type_input_kind::Kind::Binary{11}else{10};'
assert s.count(a)==1
s=s.replace(a,b)
a='if self.state.array{real_word(value)'
b='if self.state.integer{integer_word(value).ok_or_else(||invalid("IFC LIST member requires closed INTEGER"))?;}else if self.state.array{real_word(value)'
assert s.count(a)==1
s=s.replace(a,b)
a='let text = rows(event).and_then(|rows| rows.get(self.state.next)).and_then(|row| row.get("value")).and_then(|value| value.get("value")).and_then(DslValue::as_str).ok_or_else(|| invalid("IFC BINARY original text missing"))?;'
b='let payload=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).ok_or_else(||invalid("IFC BINARY original payload missing"))?;let text=if self.state.typed{typed_constructor(payload).and_then(|(_,items)|items.first()).and_then(|child|child.get("value")).and_then(DslValue::as_str)}else{payload.as_str()}.ok_or_else(||invalid("IFC BINARY original text missing"))?;'
assert s.count(a)==1
s=s.replace(a,b)
p.write_text(s,encoding='utf-8',newline='\n')
print('[DEBUG] native recursive receiver carrier patch complete')
