from pathlib import Path
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
p=D/'type-guidance.rs';s=p.read_text(encoding='utf-8')
def replace(a,b,n=1):
 global s
 assert s.count(a)==n,(a,s.count(a))
 s=s.replace(a,b)
replace('InputKind::List|InputKind::Set|InputKind::Array|InputKind::List','InputKind::List|InputKind::Set|InputKind::Array',2)
replace('if empty||set||leaf==InputKind::Number','if empty||set||matches!(leaf,InputKind::Number|InputKind::Enum)')
replace('if set&&valid{self.state.phase=96;}else if kind==InputKind::List&&present', 'if set&&valid{self.state.phase=96;}else if declared==InputKind::Select&&present&&valid&&self.state.leaves[self.state.part-1]==InputKind::Enum{self.state.member=0;self.state.offset=0;self.state.phase=103;}else if declared==InputKind::Select&&present&&valid&&self.state.leaves[self.state.part-1]==InputKind::Binary{self.state.offset=0;self.state.phase=104;}else if kind==InputKind::List&&present')
replace('            60 => {','            60|103 => {')
replace('let domain = self.domains[self.state.part - 1].ok_or_else(invalid)?;', 'let domain = if self.state.phase==103{self.leaf_domains[self.state.part-1]}else{self.domains[self.state.part-1]}.ok_or_else(invalid)?;')
replace('next.phase = 30;', 'next.phase = if self.state.phase==103{4}else{30};')
replace('            81=>{','            81|104=>{')
replace('if end==bytes.len(){next.phase=30;}', 'if end==bytes.len(){next.phase=if self.state.phase==104{4}else{30};}')
replace('let label = if let Some(remove)=field.operation{let array=', 'let label = if let Some(remove)=field.operation{let array=')
a='match(remove,array,empty,request.locale){'
b='if field.kind==InputKind::List{match(remove,empty,request.locale){(true,_,Locale::En)=>"Remove integer word",(true,_,Locale::De)=>"INTEGER-Wort entfernen",(false,true,Locale::En)=>"Create LIST aggregate",(false,true,Locale::De)=>"LIST-Aggregat erstellen",(false,false,Locale::En)=>"Add integer word",(false,false,Locale::De)=>"INTEGER-Wort hinzufügen"}}else{match(remove,array,empty,request.locale){'
replace(a,b)
replace('(false,false,false,Locale::De)=>"Referenz hinzufügen"}}else{match', '(false,false,false,Locale::De)=>"Referenz hinzufügen"}}}else{match')
p.write_text(s,encoding='utf-8',newline='\n')
print('[DEBUG] selected enum and binary drafts reuse progressive original child admission')
