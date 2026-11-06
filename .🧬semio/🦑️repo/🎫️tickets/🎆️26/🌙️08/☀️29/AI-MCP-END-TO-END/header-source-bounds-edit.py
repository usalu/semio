from pathlib import Path
import os
p=Path('🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🦀️.rs');s=p.read_text(encoding='utf8');old=s
a=' pub fn owns_header_pointer(&self,pointer:usize)->bool'
b=' pub fn preborn_header_address(&self)->usize{self.header.as_ptr() as usize}\n pub fn owns_header_pointer(&self,pointer:usize)->bool'
assert s.count(a)==1;s=s.replace(a,b)
a=' pub fn handback_demand()->OriginalEscrowPublicationDemand'
b=' pub fn core_wire_header_slot_extent()->Result<usize,ValueError>{if std::mem::size_of::<Option<OriginalEscrowHeaderLoan>>()>MAX_METADATA||std::mem::size_of::<Option<OriginalEscrowHeaderReturned>>() > MAX_METADATA{return Err(invalid())}Ok(MAX_METADATA)}\n pub fn handback_demand()->OriginalEscrowPublicationDemand'
assert s.count(a)==1;s=s.replace(a,b)
assert p.read_text(encoding='utf8')==old;t=p.with_name(p.name+'.terra.tmp');t.write_text(s,encoding='utf8',newline='');assert p.read_text(encoding='utf8')==old;os.replace(t,p)
