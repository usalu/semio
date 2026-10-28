from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs')
s=p.read_text(encoding='utf-8')
s=s.replace('layout:if empty||set{StagedFormLayout::Replace}', 'layout:if empty||set||leaf==InputKind::Number{StagedFormLayout::Replace}')
s=s.replace('let source=if field.present&&row_data(request,field.row)?.1==InputKind::Select{Some(constructor(request,field.row)?)}else{None};let kind=if kind==InputKind::Number', 'let source=if field.present&&row_data(request,field.row)?.1==InputKind::Select&&(!matches!(kind,InputKind::Set|InputKind::Array)||state.leaves[field.ordinal]==kind){Some(constructor(request,field.row)?)}else{None};let kind=if kind==InputKind::Number')
p.write_text(s,encoding='utf-8',newline='\n')
