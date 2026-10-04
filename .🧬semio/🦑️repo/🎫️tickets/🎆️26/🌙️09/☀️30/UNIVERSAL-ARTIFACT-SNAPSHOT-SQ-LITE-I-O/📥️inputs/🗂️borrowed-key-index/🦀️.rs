/// 🗂️ Validates full borrowed object keys in admitted indexed slots.
pub fn object_controlled<'v>(&'v self,control:&mut NativeDecodeControl<'_>)->Result<&'v[(String,DslValue)],ValueError>{
 let Self::Object(entries)=self else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue,"expected object"))};
 control.scoped_stage(|control|{
  control.begin_stage(entries.len())?;
  let count=if entries.is_empty(){0}else{entries.len().checked_mul(2).and_then(usize::checked_next_power_of_two).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index size overflow"))?};
  let mut slots=control.allocate_vec::<Option<(u64,&str)>>(count)?;slots.resize(count,None);
  for(key,_)in entries{
   let hash=key_hash(key,control)?;let mask=count-1;let mut slot=usize::try_from(hash&u64::try_from(mask).map_err(|_|ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index mask overflow"))?).map_err(|_|ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index offset overflow"))?;
   control.scoped_stage(|control|{
    control.begin_stage(0)?;
    for _ in 0..count{
     control.step()?;
     match slots[slot]{
      None=>{slots[slot]=Some((hash,key));return Ok(());},
      Some((previous_hash,previous))=>{if hash==previous_hash&&key_equal(previous,key,control)?{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue,"duplicate object key"));}}
     }
     slot=(slot+1)&mask;
    }
    Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated,"object key index has no vacant slot"))
   })?;
   control.step()?;
  }
  Ok(entries.as_slice())
 })
}
