#[cfg(test)]
fn with_neutral_scene_encoding<T>(body:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,NormalPortError>)->Result<T,NormalPortError>{
 let mut callbacks=0_usize;
 let mut progress=|_|{callbacks+=1;callbacks<=16_777_216};
 body(&mut semio_framework_value::NativeEncodeControl::new(256*1024*1024,&mut progress))
}
#[cfg(test)]
fn encode_neutral_pick_targets(host:&BoardHost,sx:f64,sy:f64)->String{
 with_neutral_scene_encoding(|control|host.pick_targets_at_screen_json(sx,sy,control)).expect("test-owned pick encoding")
}
#[cfg(test)]
fn encode_neutral_highlights(host:&BoardHost)->Result<String,NormalPortError>{
 with_neutral_scene_encoding(|control|host.highlighted_ids_json(control))
}

