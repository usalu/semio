from pathlib import Path
import json,hashlib
root=Path(__file__).resolve().parents[7]
ticket=Path(__file__).resolve().parent
base=root/"🧰️framework/🔨️modules/🧵️job/📬️outcome/📮️return"
for filename,owners_name,reservation_name in [("🦀️.rs","ReturnOwners","OriginalJobPageReservation"),("📦️bytes/🦀️.rs","BytesOwners","OriginalJobBytesReservation")]:
 p=base/filename;s=p.read_text()
 if filename=="🦀️.rs":
  s=s.replace("use semio_framework_job_payload::RetainedJobPageTransfer;","use semio_framework_job_payload::RetainedJobPageTransfer;\nuse semio_framework_job_context::OriginalCallerReturnPlace;")
 old="binding:Option<OriginalSourceBinding>}"
 assert old in s;s=s.replace(old,"binding:Option<OriginalSourceBinding>,credit:OriginalCallerReturnPlace}",1)
 old="binding:None})";assert old in s;s=s.replace(old,"binding:None,credit:OriginalCallerReturnPlace::empty()})",1)
 old="""   checked.owners().binding=Some(OriginalSourceBinding::borrow(cx));slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=CLAIMING;
   *output=Some("""+reservation_name+"""{registry:self,index,generation,active:true,ready:false});
   registry_control(cx,false)?;return Ok(false)"""
 new="""   let admission=checked.owners().credit.admit(cx);
   if checked.owners().credit.is_admitted(){checked.owners().binding=Some(OriginalSourceBinding::borrow(cx));slot.identity.store(identity,Ordering::Relaxed);slot.generation.store(generation,Ordering::Relaxed);checked.restore=CLAIMING;*output=Some("""+reservation_name+"""{registry:self,index,generation,active:true,ready:false});}
   admission?;return Ok(false)"""
 assert old in s;s=s.replace(old,new)
 needle="   if let Some(source)=owners.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?;}}"
 assert needle in s
 s=s.replace(needle,needle+"""
   if owners.credit.is_admitted(){let before=cx.retained_progress();owners.credit.settle_return(cx)?;let after=cx.retained_progress();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}})}
""",1)
 if filename=="🦀️.rs":
  needle="checked.owners().returned=Some(page);checked.restore=RETURNED;"
  new='checked.owners().returned=Some(page);checked.owners().credit.record_return().expect("original page retains its one-shot held source return");checked.restore=RETURNED;'
 else:
  needle="checked.owners().bytes=Some(bytes);checked.restore=RETURNED;"
  new='checked.owners().bytes=Some(bytes);checked.owners().credit.record_return().expect("original bytes retain their one-shot held source return");checked.restore=RETURNED;'
 assert needle in s;s=s.replace(needle,new,1)
 needle='assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.restore=RETURNED;'
 new='assert_eq!(slot.generation.load(Ordering::Relaxed),self.generation);checked.owners().credit.record_return().expect("cancelled original reservation retains its held source return");checked.restore=RETURNED;'
 assert needle in s;s=s.replace(needle,new,1);p.write_text(s)
print("applied genuine held returns")
