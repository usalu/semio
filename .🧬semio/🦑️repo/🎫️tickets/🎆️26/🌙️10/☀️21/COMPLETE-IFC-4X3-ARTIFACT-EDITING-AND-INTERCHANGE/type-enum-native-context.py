from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=f.read_text(encoding='utf8');s=s.replace('    previous: bool,\n}', '    previous: bool,\n    field:usize,\n}',1)
s=s.replace('                    description: None,','                    description: Some(&page.fields[navigation.field].attribute),',1)
a='''                            previous: self.state.navigation[0] == self.nav_spec(),''';b=a+'''\n                            field:self.fields.as_ref().unwrap().iter().position(|field|Some(field.index)==self.nav_spec().map(|value|value.0)).expect("source-owned page target"),''';assert a in s;s=s.replace(a,b)
b=s.encode();old=f.stat().st_size
with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))

