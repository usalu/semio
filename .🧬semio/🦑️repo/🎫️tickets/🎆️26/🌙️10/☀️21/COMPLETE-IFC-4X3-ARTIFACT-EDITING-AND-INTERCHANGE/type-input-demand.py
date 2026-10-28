exec((__import__('pathlib').Path(__file__).with_name('type-input-mount.py')).read_text(encoding='utf-8').split("update('🦀️.rs'")[0])
update('constituent_roster.rs',[
('pub(super) fn demands(&self, source:', 'pub(super) fn demands(&self, event:&editing::SnapshotEditEvent, source:'),
('''            22 => {
                return Err(invalid("IFC type input demand requires original event"));
            }''','''            22 => {
                let child=self.inputs.demands(event,body)?;
                demand.copy_bytes=demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(||invalid("IFC type input wrapper copy overflow"))?;
                demand.capacity_bytes=child.capacity_bytes;demand.release_bytes=child.release_bytes;demand.depth=demand.depth.max(child.depth);
            }'''),
('let demand = if self.state.phase==22 { self.input_demands(event,body)? } else { self.demands(Some(source), cursor, value_cursor, snapshot, body)? };','let demand=self.demands(event,Some(source),cursor,value_cursor,snapshot,body)?;'),
('''            let mut bytes=[0;256];let name=Self::type_name(event,address).and_then(|name|super::domain_cursor::canonical_name(name,&mut bytes)).ok_or_else(||invalid("IFC type input target changed"))?;
            if self.inputs.advance::<C>(event,name,self.state.target_arity,cx)?''','''            let mut bytes=[0;256];let name=Self::type_name(event,address).filter(|name|name.len()<=256&&name.is_ascii()).ok_or_else(||invalid("IFC type input target changed"))?;let length=name.len();for(index,byte)in name.bytes().enumerate(){bytes[index]=byte.to_ascii_uppercase();}let name=std::str::from_utf8(&bytes[..length]).map_err(|_|invalid("IFC type input target invalid"))?;
            if self.inputs.advance::<C>(event,name,self.state.target_arity,cx)?''')])
update('work.rs',[(
'self.constituent_roster.demands(self.source.as_ref(),&self.parts_cursor,&self.value_cursor,input.snapshot,body)',
'self.constituent_roster.demands(self.original_event.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"IFC original type event missing"))?,self.source.as_ref(),&self.parts_cursor,&self.value_cursor,input.snapshot,body)')])
