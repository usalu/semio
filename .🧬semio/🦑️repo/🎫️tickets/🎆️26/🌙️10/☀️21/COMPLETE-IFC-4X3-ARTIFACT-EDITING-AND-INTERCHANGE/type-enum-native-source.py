from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs')
s=f.read_text(encoding='utf8')
s=s.replace('use super::*;', 'use super::*;\n#[path="type_input_choices.rs"]mod choice_rows;\nuse choice_rows::Choices;',1)
s=s.replace('struct Page {','''struct ChoiceParts {strings:[Option<String>;5],payload:Option<Vec<(String,DslValue)>>}
#[derive(semio_framework_value::RetireOwned)]
struct Navigation {binding:StagedFormPageBinding,previous:bool}
#[derive(semio_framework_value::RetireOwned)]
struct Page {''',1).replace('    fields: Vec<Field>,','    fields: Vec<Field>,\n    navigation:Vec<Navigation>,',1).replace('    kind: InputKind,\n}', '    kind: InputKind,\n    choices:Option<Choices>,\n}',1)
s=s.replace('Some("integer") => InputKind::Integer,', 'Some("integer") => InputKind::Integer,\n        Some("enum") => InputKind::Enum,')
s=s.replace('"Supply required type reference"','"Supply required type values"').replace('"Supply required type reference"','"Supply required type values"').replace('"Supply required type reference"','"Supply required type values"').replace('"Supply required type reference"','"Supply required type values"')
s=s.replace('Locale::En => "Supply required type reference",','Locale::En => "Supply required type values",').replace('Locale::En => "Supply required type reference",','Locale::En => "Supply required type values",').replace('Locale::En => "Supply required type reference",','Locale::En => "Supply required type values",')
s=s.replace('Locale::En => "Supply required type reference",','Locale::En => "Supply required type values",').replace('Locale::De => "Erforderliche Typreferenz angeben",','Locale::De => "Erforderliche Typwerte angeben",')
s=s.replace('self.page.as_ref().map_or(0, |page| page.fields.len())','self.page.as_ref().map_or(0, |page| page.fields.len()+page.navigation.len())')
a='''        let field = page.fields.get(index).ok_or_else(invalid)?;
        let request = page.request.as_ref().ok_or_else(stale)?;'''
b='''        let request = page.request.as_ref().ok_or_else(stale)?;
        if index>=page.fields.len(){
            let navigation=page.navigation.get(index-page.fields.len()).ok_or_else(invalid)?;
            let label=match(navigation.previous,request.locale){(true,Locale::En)=>"Previous choices",(true,Locale::De)=>"Vorherige Auswahl",(false,Locale::En)=>"Next choices",(false,Locale::De)=>"Weitere Auswahl"};
            return Ok(StagedBodyControl{scalar:StagedScalarControl{owner:self.identity(),address:u64::MAX-2+(index-page.fields.len())as u64,controller:"s.stdio.ifc@4x3/*#editor",label,description:None,disabled:false,kind:StagedScalarKind::Button{page:true}},required:false,error:None,binding:Some(StagedBodyBinding::Page(&navigation.binding))});
        }
        let field = page.fields.get(index).ok_or_else(invalid)?;'''
assert a in s;s=s.replace(a,b)
s=s.replace('            (InputKind::Integer, _, Locale::De) => "Erforderliche Ganzzahl",','            (InputKind::Integer, _, Locale::De) => "Erforderliche Ganzzahl",\n            (InputKind::Enum, _, Locale::En) => "Required declared choice",\n            (InputKind::Enum, _, Locale::De) => "Erforderliche deklarierte Auswahl",')
s=s.replace('kind: StagedScalarKind::Text { value } },','kind: if field.kind==InputKind::Enum{StagedScalarKind::Choice{value,choices:field.choices.as_ref().ok_or_else(invalid)?.rows()}}else{StagedScalarKind::Text { value }} },')
s=s.replace('    kinds: [InputKind; 16],','''    kinds: [InputKind; 16],
    domains:[Option<&'static str>;16],
    selected:[Option<usize>;16],
    member:usize,
    active_target:Option<usize>,
    active_found:bool,
    page_offset:usize,
    navigation:[Option<(usize,usize)>;2],
    nav:usize,
    option:usize,
    choice_part:usize,
    scan:usize,''')
s=s.replace('    scalar: ManuallyDrop<Option<String>>,','    scalar: ManuallyDrop<Option<String>>,\n    choice_parts:ManuallyDrop<Option<ChoiceParts>>,\n    navigation:ManuallyDrop<Option<Vec<Navigation>>>,\n    nav_item:ManuallyDrop<Option<Navigation>>,')
s=s.replace('            scalar: ManuallyDrop::new(None),','            scalar: ManuallyDrop::new(None),\n            choice_parts:ManuallyDrop::new(None),\n            navigation:ManuallyDrop::new(None),\n            nav_item:ManuallyDrop::new(None),')
s=s.replace('|| request.page.is_some() {','|| request.page.as_ref().is_some_and(|page|!page.admitted()||page.field_offset!="0"||page.diagnostic_offset!="0") {')
a='''                if entity.complex.is_empty() || count == 0 || count == entity.args.len() || entity.args.is_empty() || C::direct_attribute_count(&entity.name) != Some(entity.args.len()) {'''
b='''                let repair=name.eq_ignore_ascii_case(&entity.name)&&count==entity.args.len();
                if count == 0 || entity.args.is_empty() || C::direct_attribute_count(&entity.name) != Some(entity.args.len()) || if repair{entity.complex.is_empty()&&C::rootless_attribute_count(name)!=Some(count)}else{entity.complex.is_empty()||count==entity.args.len()} {'''
assert a in s;s=s.replace(a,b)
s=s.replace('self.state.kinds[index_in_roster] = kind;','self.state.kinds[index_in_roster] = kind;\n                            self.state.domains[index_in_roster]=cursor.domain();')
s=s.replace('metadata(cx, base, || self.state.phase = 22)?;','metadata(cx, base, || {self.state.scan=0;self.state.phase=61;})?;',1)
s=s.replace('InputKind::String => None,','InputKind::String | InputKind::Enum => None,').replace('let valid = if kind == InputKind::String { present }','let valid = if matches!(kind,InputKind::String|InputKind::Enum) { present }')
s=s.replace('layout: if present { StagedFormLayout::Preserve } else { StagedFormLayout::Replace }','layout: if present && kind!=InputKind::Enum { StagedFormLayout::Preserve } else { StagedFormLayout::Replace }')
s=s.replace('                        kind,\n                    });','                        kind,\n                        choices:None,\n                    });',1)
s=s.replace('''                    if kind == InputKind::String && present {
                        self.state.phase = 30;
                    }''','''                    if kind == InputKind::String && present {self.state.phase=30;}
                    else if kind==InputKind::Enum && present{self.state.member=0;self.state.offset=0;self.state.phase=60;}''')
s=s.replace('Some(IfcValue::String(self.scalar.take().unwrap()))','Some(if self.state.kinds[self.state.part-1]==InputKind::Enum{IfcValue::Enum(self.scalar.take().unwrap())}else{IfcValue::String(self.scalar.take().unwrap())})')
s=s.replace('                        fields: self.fields.take().unwrap(),','                        fields: self.fields.take().unwrap(),\n                        navigation:self.navigation.take().unwrap(),')
s=s.replace('''            15 => {
                let mut buffer = [0; 20];
                let row = decimal(self.current_field()?.row, &mut buffer);''','''            15 if self.current_field()?.kind==InputKind::Enum => {metadata(cx,base,||{self.state.option=0;self.state.choice_part=0;self.state.phase=33;})?;}
            15 => {
                let mut buffer = [0; 20];
                let row = decimal(self.current_field()?.row, &mut buffer);''')
s=s.replace('            && self.scalar.is_none()','            && self.scalar.is_none()\n            && self.choice_parts.is_none()\n            && self.navigation.is_none()\n            && self.nav_item.is_none()')
s=s.replace('        owner!(&self.scalar, String);','        owner!(&self.scalar, String);\n        owner!(&self.choice_parts, ChoiceParts);\n        owner!(&self.navigation, Vec<Navigation>);\n        owner!(&self.nav_item, Navigation);')
s=s.replace('        owner!(&mut self.scalar);','        owner!(&mut self.scalar);\n        owner!(&mut self.choice_parts);\n        owner!(&mut self.navigation);\n        owner!(&mut self.nav_item);')
b=s.encode();old=f.stat().st_size
with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
