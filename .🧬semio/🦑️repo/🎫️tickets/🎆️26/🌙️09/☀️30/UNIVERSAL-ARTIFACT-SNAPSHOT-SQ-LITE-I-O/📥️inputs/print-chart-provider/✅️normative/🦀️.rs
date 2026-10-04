fn normative(domain:Domain,fields:&[(String,DslValue)])->Result<()>{
 if domain==Domain::Guide{
  if let Some(value)=member(fields,"ticks"){let value=numeric(value)?.as_f64();if value<1.0||value.fract()!=0.0{return Err(invalid("guide ticks must be an integer of at least one"))}}
  if let Some(value)=member(fields,"orient"){if !["top","right","bottom","left"].contains(&text(value)?){return Err(invalid("unknown guide orientation"))}}
 }
 if domain==Domain::Scale{if let Some(value)=member(fields,"options"){
  let options=object(value)?;
  for name in["nice","constant","ticks"]{if let Some(value)=member(options,name){if !(name=="nice"&&matches!(value,DslValue::Bool(_)))&&numeric(value)?.as_f64()<=0.0{return Err(invalid("positive scale option required"))}}}
  for name in["padding","paddingOuter"]{if let Some(value)=member(options,name){if numeric(value)?.as_f64()<0.0{return Err(invalid("nonnegative scale padding required"))}}}
  for name in["paddingInner","align"]{if let Some(value)=member(options,name){if !(0.0..=1.0).contains(&numeric(value)?.as_f64()){return Err(invalid("scale fraction must be between zero and one"))}}}
  if let Some(value)=member(options,"base"){let value=numeric(value)?.as_f64();if value<=0.0||value==1.0{return Err(invalid("scale base must be positive and differ from one"))}}
  if let Some(value)=member(options,"interpolator"){if !["number","round","rgb","lab","hcl","oklab"].contains(&text(value)?){return Err(invalid("unknown scale interpolator"))}}
  if let Some(value)=member(options,"interval"){if !["auto","day","week","month","year"].contains(&text(value)?){return Err(invalid("unknown scale interval"))}}
  if let Some(value)=member(options,"scheme"){if text(value)?.is_empty(){return Err(invalid("scale scheme must be nonempty"))}}
  if ["band","point"].contains(&text(member(fields,"kind").ok_or_else(||invalid("missing scale kind"))?)?)&&member(options,"unknown").is_some(){return Err(invalid("band and point scales do not admit unknown"))}
 }}Ok(())
}
