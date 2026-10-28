from pathlib import Path
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
p=home/'🦀️.rs';s=p.read_text(encoding='utf8');a='#[path="type_input.rs"]mod type_input;';assert s.count(a)==1;s=s.replace(a,a+'\n#[path="type_input_kind.rs"]mod type_input_kind;');a='    fn catalog() -> &\'static IfcCatalog;';assert s.count(a)==1;s=s.replace(a,a+'''\n    /// 🧬️ Borrow one defined alias expression from the same profile metadata authority.
    fn defined_type_expression(name:&str)->Option<&'static dyn IfcDetailsTypeMetadata>{Some(Self::catalog().types.get(name)?.type_expr.as_ref()? as &dyn IfcDetailsTypeMetadata)}''');p.open('r+b').write(s.encode())
p=home.parent/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs';s=p.read_text(encoding='utf8');a='    fn direct_attribute_count(name:&str)->Option<usize>{Some(crate::validation::metadata::entity(name)?.attributes.len())}';assert s.count(a)==1;s=s.replace(a,a+'''\n    fn defined_type_expression(name:&str)->Option<&'static dyn shared::IfcDetailsTypeMetadata>{Some(crate::validation::metadata::defined_type(name)?.type_expr? as &dyn shared::IfcDetailsTypeMetadata)}''');p.open('r+b').write(s.encode())
p=home/'type_input.rs';s=p.read_text(encoding='utf8');s=s.replace('    retirement: RetainedCloneClose,','    retirement: RetainedCloneClose,\n    kind: super::type_input_kind::Cursor,');s=s.replace('retirement: Default::default() }','retirement: Default::default(),kind:Default::default() }');s=s.replace(' + size_of::<IfcValueReceiver>() + 256',' + size_of::<IfcValueReceiver>() + size_of::<super::type_input_kind::Cursor>() + 256')
a=s.index('                let projection = attribute.type_expr.projection();');b=s.index('                self.state.phase = 2;',a)+len('                self.state.phase = 2;')
s=s[:a]+'''                if attribute.optional || attribute.derived {return Err(invalid("IFC type input requires an actual required target slot").into());}
                self.state.index=index;self.kind.clear();self.state.phase=8;
''' + s[b:]
a=s.index('            5 => {',s.index('pub(super) fn advance'))
s=s[:a]+'''            8=>{
                let attribute=C::normalization_attribute(name,self.state.index,true).map_err(|_|invalid("IFC type input declaration missing"))?.ok_or_else(||invalid("IFC type input attribute missing"))?;
                if let Some(kind)=self.kind.advance::<C>(attribute.type_expr)?{
                    let DslValue::Object(fields)=rows(event).and_then(|rows|rows.get(self.state.next)).ok_or_else(||invalid("IFC input row disappeared"))? else{return Err(invalid("IFC type input requires closed row").into())};
                    let DslValue::Object(value)=fields.iter().find(|(key,_)|key=="value").map(|(_,value)|value).ok_or_else(||invalid("IFC type input value missing"))? else{return Err(invalid("IFC type input requires tagged payload").into())};
                    if value.len()!=2||!value.iter().any(|(key,value)|key=="kind"&&value.as_str()==Some(kind.tag()))||!value.iter().any(|(key,value)|key=="value"&&matches!(value,DslValue::String(_))){return Err(invalid("IFC type input requires exact declared command payload").into());}
                    self.raw=Some(std::mem::replace(payload_mut(event,self.state.next).ok_or_else(||invalid("IFC type input donor missing"))?,DslValue::Null));self.state.phase=2;
                }
            }
''' + s[a:]
p.open('r+b').write(s.encode())
p=home/'type_input_tests.rs';s=p.read_text(encoding='utf8').rstrip()+'\n#[path = "type_scalar_input_tests.rs"]\nmod scalar;\n';p.open('r+b').write(s.encode())
