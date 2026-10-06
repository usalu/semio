#[cfg(test)]
fn publish_neutral_scene(host:&mut BoardHost,json:&str)->Result<(),super::BoardSceneError>{
 use semio_framework_value::{ErasedSnapshotRetirement,NativeDecodeControl,SnapshotRetirementStep};
 let mut recipient=semio_framework_value::native_decoding::NativeDecodeRetirementRecipient::new();
 let mut callbacks=0_usize;
 let mut progress=|_|{callbacks+=1;callbacks<=16_777_216};
 let result={
  let mut control=NativeDecodeControl::new(256*1024*1024,&mut progress);
  control.install_retirement_recipient(&mut recipient).expect("one explicit test retirement recipient");
  super::scene::from_json(json,&mut control).and_then(|scene|scene.admit_publication(&mut control)).and_then(|scene|host.synchronize_scene(scene))
 };
 if let Err(mut error)=result{
  if let Some(mut owner)=error.take_retirement(){drain_neutral_scene_retirement(&mut owner);}
  drain_neutral_decode_retirement(&mut recipient);
  return Err(error);
 }
 if let Some(mut owner)=host.take_scene_retirement(){drain_neutral_scene_retirement(&mut owner);}
 drain_neutral_decode_retirement(&mut recipient);
 Ok(())
}

#[cfg(test)]
fn drain_neutral_scene_retirement(owner:&mut super::BoardSceneRetirement){
 for _ in 0..16_777_216{
  let bytes=owner.next_close_byte_demand().max(65_536);
  if matches!(owner.close_step(64,bytes).expect("owned scene retirement"),semio_framework_value::SnapshotRetirementStep::Complete){
   assert!(owner.terminal_is_empty());return;
  }
 }
 panic!("actual scene owner must reach its terminal retirement");
}

#[cfg(test)]
fn drain_neutral_decode_retirement(owner:&mut impl semio_framework_value::ErasedSnapshotRetirement){
 for _ in 0..16_777_216{
  let bytes=owner.next_close_byte_demand().max(65_536);
  if matches!(owner.close_step(64,bytes).expect("owned decoder retirement"),semio_framework_value::SnapshotRetirementStep::Complete){
   assert!(owner.terminal_is_empty());return;
  }
 }
 panic!("actual decoder owner must reach its terminal retirement");
}

