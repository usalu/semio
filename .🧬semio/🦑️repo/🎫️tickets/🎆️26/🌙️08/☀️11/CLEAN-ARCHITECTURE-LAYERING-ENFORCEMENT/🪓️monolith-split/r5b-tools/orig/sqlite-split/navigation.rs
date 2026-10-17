//! 🎯️ Destinations, action chains and recursive bookmark containment.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_7::subsets::base::io::sqlite::snapshot::*;

pub(super) fn write_destination(out: &mut Projection<'_, '_>, destination: &PdfDestination) -> Result<i64,ValueError> {
    let (kind,page,name,fit) = match destination { PdfDestination::Page { page,fit } => ("page",C::Integer(i64::from(*page)),C::Null,Some(fit)),PdfDestination::RemotePage { page,fit } => ("remotePage",C::Integer(i64::from(*page)),C::Null,Some(fit)),PdfDestination::Named { name } => ("named",C::Null,C::Text(name),None) };
    let mut fields = [C::Null; 8];
    if let Some(fit) = fit { fields[0] = C::Text(match fit { PdfDestinationFit::Xyz { left,top,zoom } => { fields[1] = real_cell(*left);fields[2] = real_cell(*top);fields[3] = real_cell(*zoom);"xyz" },PdfDestinationFit::Fit => "fit",PdfDestinationFit::FitHorizontal { top } => { fields[2] = real_cell(*top);"fitHorizontal" },PdfDestinationFit::FitVertical { left } => { fields[1] = real_cell(*left);"fitVertical" },PdfDestinationFit::FitRectangle { rect } => { for(index,value)in rect.iter().enumerate(){fields[4+index]=C::Real(*value);} "fitRectangle" },PdfDestinationFit::FitBoundingBox => "fitBoundingBox",PdfDestinationFit::FitBoundingBoxHorizontal { top } => { fields[2]=real_cell(*top);"fitBoundingBoxHorizontal" },PdfDestinationFit::FitBoundingBoxVertical { left } => { fields[1]=real_cell(*left);"fitBoundingBoxVertical" } }); }
    let mut cells = vec![C::Text(kind),page,name];cells.extend(fields);out.insert("pdf_destination",&cells)
}
pub(super) fn read_destination(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfDestination,ValueError> {
    let row = reader.take("pdf_destination",key,12)?;
    if row.text(1)? == "named" { null_except(row,2..12,&[3])?;return Ok(PdfDestination::Named { name:reader.text(row,3)? }); }
    null_except(row,3..4,&[])?;
    let (fit,present): (PdfDestinationFit,&[usize]) = match row.text(4)? { "xyz" => (PdfDestinationFit::Xyz { left:optional_real(row,5)?,top:optional_real(row,6)?,zoom:optional_real(row,7)? },&[5,6,7]),"fit" => (PdfDestinationFit::Fit,&[]),"fitHorizontal" => (PdfDestinationFit::FitHorizontal { top:optional_real(row,6)? },&[6]),"fitVertical" => (PdfDestinationFit::FitVertical { left:optional_real(row,5)? },&[5]),"fitRectangle" => (PdfDestinationFit::FitRectangle { rect:[row.real(8)?,row.real(9)?,row.real(10)?,row.real(11)?] },&[8,9,10,11]),"fitBoundingBox" => (PdfDestinationFit::FitBoundingBox,&[]),"fitBoundingBoxHorizontal" => (PdfDestinationFit::FitBoundingBoxHorizontal { top:optional_real(row,6)? },&[6]),"fitBoundingBoxVertical" => (PdfDestinationFit::FitBoundingBoxVertical { left:optional_real(row,5)? },&[5]),_ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF destination fit kind")) };
    null_except(row,5..12,present)?;
    Ok(match row.text(1)? { "page" => PdfDestination::Page { page:integer(row,2)?,fit },"remotePage" => PdfDestination::RemotePage { page:integer(row,2)?,fit },_ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF destination kind")) })
}
pub(super) fn write_file_spec(out: &mut Projection<'_, '_>, file: &PdfFileSpecification) -> Result<i64,ValueError> { match file { PdfFileSpecification::Path { path } => out.insert("pdf_file_specification",&[C::Text("path"),C::Text(path)]),PdfFileSpecification::Embedded { file } => out.insert("pdf_file_specification",&[C::Text("embedded"),C::Text(file)]) } }
pub(super) fn read_file_spec(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfFileSpecification,ValueError> { let row=reader.take("pdf_file_specification",key,3)?;Ok(match row.text(1)? { "path" => PdfFileSpecification::Path { path:reader.text(row,2)? },"embedded" => PdfFileSpecification::Embedded { file:reader.text(row,2)? },_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF file specification kind")) }) }

fn write_action_shallow(out: &mut Projection<'_, '_>, action: &PdfAction) -> Result<i64,ValueError> {
    let mut fields=[C::Null;21];
    let kind=match &action.kind {
        PdfActionKind::GoTo { destination } => {fields[0]=C::Integer(write_destination(out,destination)?);"goTo"},
        PdfActionKind::GoToRemote { file,destination,new_window } => {fields[0]=C::Integer(write_destination(out,destination)?);fields[1]=C::Integer(write_file_spec(out,file)?);fields[2]=boolean_cell(*new_window);"goToRemote"},
        PdfActionKind::GoToEmbedded { destination,new_window } => {fields[0]=C::Integer(write_destination(out,destination)?);fields[2]=boolean_cell(*new_window);"goToEmbedded"},
        PdfActionKind::Launch { file,new_window } => {fields[1]=C::Integer(write_file_spec(out,file)?);fields[2]=boolean_cell(*new_window);"launch"},
        PdfActionKind::Thread { file,thread } => {if let Some(file)=file{fields[1]=C::Integer(write_file_spec(out,file)?);}fields[3]=C::Integer(i64::from(*thread));"thread"},
        PdfActionKind::Uri { uri,is_map } => {fields[4]=C::Text(uri);fields[5]=C::Integer(i64::from(*is_map));"uri"},
        PdfActionKind::Sound { sound,volume,synchronous,repeat,mix } => {fields[6]=C::Text(sound);fields[7]=real_cell(*volume);fields[8]=C::Integer(i64::from(*synchronous));fields[9]=C::Integer(i64::from(*repeat));fields[10]=C::Integer(i64::from(*mix));"sound"},
        PdfActionKind::Movie { annotation,operation } => {fields[11]=text_cell(annotation);fields[12]=text_cell(operation);"movie"},
        PdfActionKind::Hide { hide,.. } => {fields[13]=C::Integer(i64::from(*hide));"hide"}, PdfActionKind::Named { name } => {fields[14]=C::Text(name);"named"},
        PdfActionKind::SubmitForm { url,flags,.. } => {fields[15]=C::Text(url);fields[16]=C::Integer(i64::from(*flags));"submitForm"},PdfActionKind::ResetForm { flags,.. } => {fields[16]=C::Integer(i64::from(*flags));"resetForm"},
        PdfActionKind::ImportData { file } => {fields[1]=C::Integer(write_file_spec(out,file)?);"importData"},PdfActionKind::JavaScript { script } => {fields[17]=C::Text(script);"javaScript"},
        PdfActionKind::SetOptionalContentState { states,preserve_radio_buttons } => {fields[18]=C::Integer(cos::write_dictionary(out,states)?);fields[19]=C::Integer(i64::from(*preserve_radio_buttons));"setOptionalContentState"},
        PdfActionKind::Rendition { entries } => {fields[18]=C::Integer(cos::write_dictionary(out,entries)?);"rendition"},PdfActionKind::Transition { entries } => {fields[18]=C::Integer(cos::write_dictionary(out,entries)?);"transition"},PdfActionKind::GoTo3dView { entries } => {fields[18]=C::Integer(cos::write_dictionary(out,entries)?);"goTo3dView"},PdfActionKind::Unknown { subtype,entries } => {fields[18]=C::Integer(cos::write_dictionary(out,entries)?);fields[20]=C::Text(subtype);"unknown"},
    };
    let mut cells=vec![C::Text(kind)];cells.extend(fields);let key=out.insert("pdf_action",&cells)?;
    if let PdfActionKind::Hide { annotations:names,.. }|PdfActionKind::SubmitForm { fields:names,.. }|PdfActionKind::ResetForm { fields:names,.. }=&action.kind{for(ordinal,name)in names.iter().enumerate(){out.insert("pdf_action_name",&[C::Integer(key),C::Integer(ordinal as i64),C::Text(name)])?;}}
    Ok(key)
}
fn action_names(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<Vec<String>,ValueError>{let mut names=Vec::new();for row in reader.children("pdf_action_name",1,2,key)?{let row=reader.take("pdf_action_name",row.rowid,4)?;names.push(reader.text(row,3)?);}Ok(names)}
fn read_action_shallow(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfAction,ValueError>{
    let row=reader.take("pdf_action",key,23)?;
    let(kind,present):(PdfActionKind,&[usize])=match row.text(1)? {
        "goTo"=>(PdfActionKind::GoTo{destination:read_destination(reader,row.integer(2)?)?},&[2]),
        "goToRemote"=>(PdfActionKind::GoToRemote{file:read_file_spec(reader,row.integer(3)?)?,destination:read_destination(reader,row.integer(2)?)?,new_window:optional_boolean(row,4)?},&[2,3,4]),
        "goToEmbedded"=>(PdfActionKind::GoToEmbedded{destination:read_destination(reader,row.integer(2)?)?,new_window:optional_boolean(row,4)?},&[2,4]),
        "launch"=>(PdfActionKind::Launch{file:read_file_spec(reader,row.integer(3)?)?,new_window:optional_boolean(row,4)?},&[3,4]),
        "thread"=>(PdfActionKind::Thread{file:optional_integer(row,3)?.map(|value|read_file_spec(reader,value)).transpose()?,thread:integer(row,5)?},&[3,5]),
        "uri"=>(PdfActionKind::Uri{uri:reader.text(row,6)?,is_map:boolean(row,7)?},&[6,7]),
        "sound"=>(PdfActionKind::Sound{sound:reader.text(row,8)?,volume:optional_real(row,9)?,synchronous:boolean(row,10)?,repeat:boolean(row,11)?,mix:boolean(row,12)?},&[8,9,10,11,12]),
        "movie"=>(PdfActionKind::Movie{annotation:reader.optional_text(row,13)?,operation:reader.optional_text(row,14)?},&[13,14]),
        "hide"=>(PdfActionKind::Hide{annotations:action_names(reader,key)?,hide:boolean(row,15)?},&[15]),"named"=>(PdfActionKind::Named{name:reader.text(row,16)?},&[16]),
        "submitForm"=>(PdfActionKind::SubmitForm{url:reader.text(row,17)?,fields:action_names(reader,key)?,flags:integer(row,18)?},&[17,18]),"resetForm"=>(PdfActionKind::ResetForm{fields:action_names(reader,key)?,flags:integer(row,18)?},&[18]),
        "importData"=>(PdfActionKind::ImportData{file:read_file_spec(reader,row.integer(3)?)?},&[3]),"javaScript"=>(PdfActionKind::JavaScript{script:reader.text(row,19)?},&[19]),
        "setOptionalContentState"=>(PdfActionKind::SetOptionalContentState{states:cos::read_dictionary(reader,row.integer(20)?)?,preserve_radio_buttons:boolean(row,21)?},&[20,21]),
        "rendition"=>(PdfActionKind::Rendition{entries:cos::read_dictionary(reader,row.integer(20)?)?},&[20]),"transition"=>(PdfActionKind::Transition{entries:cos::read_dictionary(reader,row.integer(20)?)?},&[20]),"goTo3dView"=>(PdfActionKind::GoTo3dView{entries:cos::read_dictionary(reader,row.integer(20)?)?},&[20]),"unknown"=>(PdfActionKind::Unknown{subtype:reader.text(row,22)?,entries:cos::read_dictionary(reader,row.integer(20)?)?},&[20,22]),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF action kind")),
    };null_except(row,2..23,present)?;
    Ok(PdfAction{kind,next:Vec::new()})
}

fn write_outline_shallow(out:&mut Projection<'_,'_>,outline:&PdfOutlineItem)->Result<i64,ValueError>{
    let destination=outline.destination.as_ref().map(|value|write_destination(out,value)).transpose()?;let action=outline.action.as_ref().map(|value|write_action(out,value)).transpose()?;let extra=cos::write_dictionary(out,&outline.extra)?;
    let mut cells=vec![C::Text(&outline.title),destination.map_or(C::Null,C::Integer),action.map_or(C::Null,C::Integer)];cells.extend(array_cells(&outline.color));cells.extend([C::Integer(i64::from(outline.italic)),C::Integer(i64::from(outline.bold)),C::Integer(i64::from(outline.open)),C::Integer(extra)]);let key=out.insert("pdf_outline",&cells)?;
    Ok(key)
}
fn read_outline_shallow(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfOutlineItem,ValueError>{
    let row=reader.take("pdf_outline",key,11)?;let children=Vec::new();
    Ok(PdfOutlineItem{title:reader.text(row,1)?,destination:optional_integer(row,2)?.map(|value|read_destination(reader,value)).transpose()?,action:optional_integer(row,3)?.map(|value|read_action(reader,value)).transpose()?,color:optional_array(row,4)?,italic:boolean(row,7)?,bold:boolean(row,8)?,open:boolean(row,9)?,children,extra:cos::read_dictionary(reader,row.integer(10)?)?})
}
pub(super) fn write_named_destination(out:&mut Projection<'_,'_>,destination:&PdfNamedDestination)->Result<i64,ValueError>{let target=write_destination(out,&destination.destination)?;out.insert("pdf_named_destination",&[C::Text(&destination.name),C::Integer(target)])}
pub(super) fn read_named_destination(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfNamedDestination,ValueError>{let row=reader.take("pdf_named_destination",key,3)?;Ok(PdfNamedDestination{name:reader.text(row,1)?,destination:read_destination(reader,row.integer(2)?)?})}
pub(super) fn write_label(out:&mut Projection<'_,'_>,label:&PdfPageLabelRange)->Result<i64,ValueError>{let style=label.style.map(|value|match value{PdfPageLabelStyle::Decimal=>"decimal",PdfPageLabelStyle::RomanUpper=>"romanUpper",PdfPageLabelStyle::RomanLower=>"romanLower",PdfPageLabelStyle::LettersUpper=>"lettersUpper",PdfPageLabelStyle::LettersLower=>"lettersLower"});out.insert("pdf_page_label",&[C::Integer(i64::from(label.start_index)),style.map_or(C::Null,C::Text),text_cell(&label.prefix),C::Integer(i64::from(label.start))])}
pub(super) fn read_label(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfPageLabelRange,ValueError>{let row=reader.take("pdf_page_label",key,5)?;let style=row.optional_text(2)?.map(|value|match value{"decimal"=>Ok(PdfPageLabelStyle::Decimal),"romanUpper"=>Ok(PdfPageLabelStyle::RomanUpper),"romanLower"=>Ok(PdfPageLabelStyle::RomanLower),"lettersUpper"=>Ok(PdfPageLabelStyle::LettersUpper),"lettersLower"=>Ok(PdfPageLabelStyle::LettersLower),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF page label style"))}).transpose()?;Ok(PdfPageLabelRange{start_index:integer(row,1)?,style,prefix:reader.optional_text(row,3)?,start:integer(row,4)?})}
pub(super) fn write_open_action(out:&mut Projection<'_,'_>,action:&PdfOpenAction)->Result<i64,ValueError>{let(kind,destination,action)=match action{PdfOpenAction::Destination{destination}=>("destination",C::Integer(write_destination(out,destination)?),C::Null),PdfOpenAction::Action{action}=>("action",C::Null,C::Integer(write_action(out,action)?))};out.insert("pdf_open_action",&[C::Text(kind),destination,action])}
pub(super) fn read_open_action(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfOpenAction,ValueError>{let row=reader.take("pdf_open_action",key,4)?;Ok(match row.text(1)?{"destination"=>{null_except(row,2..4,&[2])?;PdfOpenAction::Destination{destination:read_destination(reader,row.integer(2)?)?}},"action"=>{null_except(row,2..4,&[3])?;PdfOpenAction::Action{action:read_action(reader,row.integer(3)?)?}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF open action kind"))})}

pub(super) fn write_action(out:&mut Projection<'_,'_>,action:&PdfAction)->Result<i64,ValueError>{
    let key=write_action_shallow(out,action)?;let mut pending=vec![(key,action.next.iter().enumerate())];
    while let Some((parent,children))=pending.last_mut(){if let Some((ordinal,child))=children.next(){let parent=*parent;let key=write_action_shallow(out,child)?;out.insert("pdf_action_next",&[C::Integer(parent),C::Integer(ordinal as i64),C::Integer(key)])?;pending.push((key,child.next.iter().enumerate()));}else{pending.pop();}}Ok(key)
}
pub(super) fn read_action(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfAction,ValueError>{
    let action=read_action_shallow(reader,key)?;let children=reader.children("pdf_action_next",1,2,key)?.into_iter();let mut pending=vec![(action,children)];
    loop{let child=pending.last_mut().and_then(|(_,children)|children.next());if let Some(child)=child{let child=reader.take("pdf_action_next",child.rowid,4)?;let key=child.integer(3)?;let action=read_action_shallow(reader,key)?;let children=reader.children("pdf_action_next",1,2,key)?.into_iter();pending.push((action,children));}else{let(action,_)=pending.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"PDF action stack is empty"))?;if let Some((parent,_))=pending.last_mut(){parent.next.push(action);}else{return Ok(action);}}}
}
pub(super) fn write_outline(out:&mut Projection<'_,'_>,outline:&PdfOutlineItem)->Result<i64,ValueError>{
    let key=write_outline_shallow(out,outline)?;let mut pending=vec![(key,outline.children.iter().enumerate())];
    while let Some((parent,children))=pending.last_mut(){if let Some((ordinal,child))=children.next(){let parent=*parent;let key=write_outline_shallow(out,child)?;out.insert("pdf_outline_child",&[C::Integer(parent),C::Integer(ordinal as i64),C::Integer(key)])?;pending.push((key,child.children.iter().enumerate()));}else{pending.pop();}}Ok(key)
}
pub(super) fn read_outline(reader:&mut Reader<'_,'_,'_>,key:i64)->Result<PdfOutlineItem,ValueError>{
    let outline=read_outline_shallow(reader,key)?;let children=reader.children("pdf_outline_child",1,2,key)?.into_iter();let mut pending=vec![(outline,children)];
    loop{let child=pending.last_mut().and_then(|(_,children)|children.next());if let Some(child)=child{let child=reader.take("pdf_outline_child",child.rowid,4)?;let key=child.integer(3)?;let outline=read_outline_shallow(reader,key)?;let children=reader.children("pdf_outline_child",1,2,key)?.into_iter();pending.push((outline,children));}else{let(outline,_)=pending.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"PDF outline stack is empty"))?;if let Some((parent,_))=pending.last_mut(){parent.children.push(outline);}else{return Ok(outline);}}}
}
