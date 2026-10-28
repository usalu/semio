from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
def write(f,s):
 b=s.encode();old=f.stat().st_size
 with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
f=p/'🦀️.rs';s=f.read_text(encoding='utf8');a='    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=Self::catalog().entities.get(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}';assert a in s;s=s.replace(a,a+'''\n    /// 🔤️ Borrow each released enumeration member through the same immutable profile authority.
    fn enumeration_count(name:&str)->Option<usize>{let declaration=Self::catalog().types.get(name)?;(declaration.kind=="enum").then_some(declaration.members.len())}
    fn enumeration_member(name:&str,index:usize)->Option<&'static str>{let declaration=Self::catalog().types.get(name)?;if declaration.kind!="enum"{return None;}Some(declaration.members.get(index)?.as_str())}''');write(f,s)
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs');s=f.read_text(encoding='utf8');a='    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::entity(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}';assert a in s;s=s.replace(a,a+'''\n    fn enumeration_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::defined_type(name)?;(declaration.kind=="enum").then_some(declaration.members.len())}
    fn enumeration_member(name:&str,index:usize)->Option<&'static str>{let declaration=crate::validation::metadata::defined_type(name)?;if declaration.kind!="enum"{return None;}declaration.members.get(index).copied()}''');write(f,s)
f=p/'type_input_kind.rs';s=f.read_text(encoding='utf8').replace('    Integer,','    Integer,\n    Enum,').replace('            Self::Integer => "integer",','            Self::Integer => "integer",\n            Self::Enum => "enum",').replace('    depth: usize,','    depth: usize,\n    domain: Option<&\'static str>,')
s=s.replace('    pub(super) fn clear(&mut self) {','''    pub(super) fn domain(&self)->Option<&'static str>{self.domain}
    pub(super) fn member<C:IfcDetailsProfile>(domain:&str,index:usize)->Option<&'static str>{match domain{"BOOLEAN"=>["T","F"].get(index).copied(),"LOGICAL"=>["T","F","U"].get(index).copied(),_=>C::enumeration_member(domain,index)}}
    pub(super) fn count<C:IfcDetailsProfile>(domain:&str)->Option<usize>{match domain{"BOOLEAN"=>Some(2),"LOGICAL"=>Some(3),_=>C::enumeration_count(domain)}}
    pub(super) fn clear(&mut self) {''').replace('            "INTEGER" => Some(Kind::Integer),','            "INTEGER" => Some(Kind::Integer),\n            "BOOLEAN" | "LOGICAL" => Some(Kind::Enum),\n            _ if C::enumeration_count(name).is_some() => Some(Kind::Enum),').replace('        if let Some(kind) = kind {\n            return', '        if let Some(kind) = kind {\n            if kind==Kind::Enum {self.domain=Some(name);}\n            return');write(f,s)
f=p/'type_input.rs';s=f.read_text(encoding='utf8').replace('    previous: Option<usize>,','    previous: Option<usize>,\n    member: usize,\n    offset: usize,')
a='''                    self.raw = Some(std::mem::replace(payload_mut(event, self.state.next).ok_or_else(|| invalid("IFC type input donor missing"))?, DslValue::Null));
                    self.state.phase = 2;'''
b='''                    self.state.member=0;
                    self.state.offset=0;
                    self.state.phase=if kind==super::type_input_kind::Kind::Enum{9}else{10};'''
assert a in s;s=s.replace(a,b)
a='''            5 => {
                self.receiver ='''
b='''            9 => {
                let payload=rows(event).and_then(|rows|rows.get(self.state.next)).and_then(|row|row.get("value")).and_then(|value|value.get("value")).and_then(DslValue::as_str).ok_or_else(||invalid("IFC enumeration original text missing"))?;
                let domain=self.kind.domain().ok_or_else(||invalid("IFC enumeration declaration missing"))?;
                let member=super::type_input_kind::Cursor::member::<C>(domain,self.state.member).ok_or_else(||invalid("IFC required input is not an exact declared enumeration member"))?;
                if payload.len()!=member.len(){self.state.member+=1;self.state.offset=0;}
                else if self.state.offset==member.len(){self.state.phase=10;}
                else {
                    let end=member.len().min(self.state.offset+64);
                    if payload.as_bytes()[self.state.offset..end]==member.as_bytes()[self.state.offset..end]{self.state.offset=end;}else{self.state.member+=1;self.state.offset=0;}
                }
            }
            10 => {
                self.raw=Some(std::mem::replace(payload_mut(event,self.state.next).ok_or_else(||invalid("IFC type input donor missing"))?,DslValue::Null));
                self.state.phase=2;
            }
'''+a
assert a in s;s=s.replace(a,b);write(f,s)
