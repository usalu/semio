//! 🔣️ Literal Program JSON scalar fields preserve their complete owned words.
use semio_framework_value::DslValue;
use semio_framework_value::Number;
fn field<'a>(value:&'a mut DslValue,key:&str)->Result<&'a mut DslValue,String>{match value{semio_framework_value::DslValue::Object(entries)=>entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||format!("Program JSON field {key} is absent")),_=>Err("Program JSON row requires an object".into())}}
fn float(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode {
  let number=match value {
   semio_framework_value::DslValue::Number(value)=>{let number=value.as_f64();if !number.is_finite(){return Err("Program JSON numeric transport requires a finite double".into())}number},
   semio_framework_value::DslValue::Object(entries)=>{if entries.len()!=1||entries[0].0!="bits"{return Err("Program JSON binary64 object differs".into())}let text=entries[0].1.as_str().ok_or("Program JSON binary64 word requires text")?;if text.len()!=16||!text.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b)){return Err("Program JSON binary64 word requires sixteen lowercase hexadecimal digits".into())}f64::from_bits(u64::from_str_radix(text,16).map_err(|e|e.to_string())?)},
   _=>return Err("Program JSON binary64 scalar differs".into())
  };*value=semio_framework_value::DslValue::float(number);
 }else{let number=value.as_f64().ok_or("Program JSON native binary64 is absent")?;*value=semio_framework_value::DslValue::object([("bits".into(),semio_framework_value::DslValue::String(format!("{:016x}",number.to_bits())))]);}
 Ok(())
}
fn member<'a>(value:&'a mut DslValue,key:&str)->Result<Option<&'a mut DslValue>,String>{match value{semio_framework_value::DslValue::Object(entries)=>Ok(entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value)),_=>Err("Program JSON row requires an object".into())}}
fn optional_field(value:&mut DslValue,key:&str,convert:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{if let Some(value)=member(value,key)?{if !matches!(value,DslValue::Null){convert(value,decode)?;}}Ok(())}
fn optional_float_field(value:&mut DslValue,key:&str,decode:bool)->Result<(),String>{optional_field(value,key,float,decode)}
fn optional_float(value:&mut DslValue,decode:bool)->Result<(),String>{if matches!(value,DslValue::Null){Ok(())}else{float(value,decode)}}
fn unsigned(value:&mut DslValue,decode:bool)->Result<(),String>{
 if decode {
  let word=match value{
   semio_framework_value::DslValue::String(text)=>{if text.is_empty()||text.len()>20||!text.bytes().all(|b|b.is_ascii_digit())||(text.len()>1&&text.starts_with('0')){return Err("Program JSON unsigned64 requires canonical decimal text".into())}text.parse::<u64>().map_err(|e|e.to_string())?},
   semio_framework_value::DslValue::Number(Number::UInt(word)) if *word<=9007199254740991=>*word,
   semio_framework_value::DslValue::Number(Number::Int(word)) if *word>=0&&*word<=9007199254740991=>*word as u64,
   semio_framework_value::DslValue::Number(Number::Float(word)) if word.is_finite()&&*word>=0.0&&*word<=9007199254740991.0&&word.fract()==0.0=>*word as u64,
   _=>return Err("Program JSON numeric unsigned64 requires a safe unsigned integer".into())
  };*value=semio_framework_value::DslValue::uint(word);
 }else{let word=value.as_u64().ok_or("Program JSON native unsigned64 is absent")?;*value=semio_framework_value::DslValue::String(word.to_string());}
 Ok(())
}
fn quantity(value:&mut DslValue,decode:bool)->Result<(),String>{
 if !matches!(value,DslValue::Object(_)){return Err("Program JSON quantity requires an object".into())}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="min"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="max"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="target"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="current"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="forecast"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="peak"){optional_float(value,decode)?;}}
 if let semio_framework_value::DslValue::Object(entries)=value{if let Some((_,value))=entries.iter_mut().find(|(name,_)|name=="average"){optional_float(value,decode)?;}}
 Ok(())
}
fn rows(value:&mut DslValue,convert:fn(&mut DslValue,bool)->Result<(),String>,decode:bool)->Result<(),String>{match value{semio_framework_value::DslValue::Array(values)=>{for value in values{convert(value,decode)?}Ok(())},_=>Err("Program JSON register requires an array".into())}}
fn _accessibility_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"clearWidthM",decode)?;
 optional_float_field(value,"clearHeightM",decode)?;
 optional_float_field(value,"turningCircleM",decode)?;
 optional_float_field(value,"rampSlope",decode)?;
 Ok(())
}
fn _adjacency(value:&mut DslValue,decode:bool)->Result<(),String>{
 float(field(value,"weight")?,decode)?;
 optional_float_field(value,"distanceMaxM",decode)?;
 optional_float_field(value,"distanceMinM",decode)?;
 Ok(())
}
fn _benchmark_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 float(field(value,"value")?,decode)?;
 Ok(())
}
fn _change_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costImpact",decode)?;
 Ok(())
}
fn _conflict(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costImpact",decode)?;
 Ok(())
}
fn _cost_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"amount",decode)?;
 optional_float_field(value,"unitCost",decode)?;
 optional_float_field(value,"contingencyPercent",decode)?;
 optional_float_field(value,"escalationRate",decode)?;
 Ok(())
}
fn _decision(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costImpact",decode)?;
 Ok(())
}
fn _environmental_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"targetValue",decode)?;
 optional_float_field(value,"minValue",decode)?;
 optional_float_field(value,"maxValue",decode)?;
 Ok(())
}
fn _equipment(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"weightKg",decode)?;
 optional_float_field(value,"powerKw",decode)?;
 optional_float_field(value,"noiseLevelDb",decode)?;
 optional_float_field(value,"replacementCost",decode)?;
 optional_field(value,"quantity",quantity,decode)?;
 Ok(())
}
fn _flexibility_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costOfChange",decode)?;
 Ok(())
}
fn _flow_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"peakRate",decode)?;
 optional_float_field(value,"clearWidthM",decode)?;
 optional_float_field(value,"clearHeightM",decode)?;
 optional_field(value,"volume",quantity,decode)?;
 Ok(())
}
fn _growth_plan(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"growthRate",decode)?;
 optional_float_field(value,"budgetEnvelope",decode)?;
 optional_field(value,"headcountGrowth",quantity,decode)?;
 optional_field(value,"areaGrowth",quantity,decode)?;
 Ok(())
}
fn _infrastructure_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"peakDemand",decode)?;
 optional_float_field(value,"diversityFactor",decode)?;
 optional_float_field(value,"lifecycleCost",decode)?;
 optional_field(value,"capacity",quantity,decode)?;
 Ok(())
}
fn _operational_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"uptimeTarget",decode)?;
 optional_field(value,"staffing",quantity,decode)?;
 Ok(())
}
fn _option_evaluation(value:&mut DslValue,decode:bool)->Result<(),String>{
 rows(field(value,"scores")?,float,decode)?;
 optional_float_field(value,"weightedScore",decode)?;
 optional_float_field(value,"costEstimate",decode)?;
 Ok(())
}
fn _performance_criterion(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"target",decode)?;
 optional_float_field(value,"minimum",decode)?;
 optional_float_field(value,"maximum",decode)?;
 optional_float_field(value,"baseline",decode)?;
 optional_float_field(value,"weight",decode)?;
 optional_float_field(value,"penaltyThreshold",decode)?;
 optional_float_field(value,"incentiveThreshold",decode)?;
 Ok(())
}
fn _priority_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"weight",decode)?;
 optional_float_field(value,"score",decode)?;
 Ok(())
}
fn _quantity_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"tolerancePercent",decode)?;
 optional_float_field(value,"peakFactor",decode)?;
 optional_float_field(value,"growthFactor",decode)?;
 optional_float_field(value,"unitCost",decode)?;
 optional_field(value,"quantity",quantity,decode)?;
 Ok(())
}
fn _relationship(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"strength",decode)?;
 optional_float_field(value,"distanceConstraintM",decode)?;
 Ok(())
}
fn _requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costEstimate",decode)?;
 Ok(())
}
fn _resource(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"costPerUnit",decode)?;
 optional_float_field(value,"sharingRatio",decode)?;
 optional_field(value,"quantity",quantity,decode)?;
 Ok(())
}
fn _risk(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"riskScore",decode)?;
 Ok(())
}
fn _scenario(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"probability",decode)?;
 optional_float_field(value,"costDelta",decode)?;
 optional_float_field(value,"areaDelta",decode)?;
 optional_float_field(value,"headcountDelta",decode)?;
 Ok(())
}
fn _site_context(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"latitude",decode)?;
 optional_float_field(value,"longitude",decode)?;
 optional_float_field(value,"elevationM",decode)?;
 optional_float_field(value,"maxHeightM",decode)?;
 optional_float_field(value,"maxCoverage",decode)?;
 Ok(())
}
fn _status_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"progressPercent",decode)?;
 Ok(())
}
fn _storage_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"volumeM3",decode)?;
 optional_float_field(value,"weightKg",decode)?;
 optional_float_field(value,"growthAllowance",decode)?;
 optional_field(value,"quantity",quantity,decode)?;
 Ok(())
}
fn _survey(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"responseRate",decode)?;
 Ok(())
}
fn _sustainability_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"baseline",decode)?;
 optional_float_field(value,"targetValue",decode)?;
 optional_float_field(value,"embodiedCarbon",decode)?;
 optional_float_field(value,"operationalCarbon",decode)?;
 Ok(())
}
fn _wayfinding_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"maximumSignageDistanceM",decode)?;
 Ok(())
}
fn _workshop(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_float_field(value,"budget",decode)?;
 Ok(())
}
fn _activity(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"participants",quantity,decode)?;
 Ok(())
}
fn _function(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"staffing",quantity,decode)?;
 Ok(())
}
fn _program_element(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"area",quantity,decode)?;
 optional_field(value,"volume",quantity,decode)?;
 optional_field(value,"height",quantity,decode)?;
 optional_field(value,"occupancy",quantity,decode)?;
 Ok(())
}
fn _organizational_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"headcount",quantity,decode)?;
 Ok(())
}
fn _service_requirement(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"capacity",quantity,decode)?;
 optional_field(value,"staffing",quantity,decode)?;
 Ok(())
}
fn _analysis_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 optional_field(value,"durationMs",unsigned,decode)?;
 Ok(())
}
fn _search_filter(value:&mut DslValue,decode:bool)->Result<(),String>{
 unsigned(field(value,"useCount")?,decode)?;
 Ok(())
}
fn _template_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 unsigned(field(value,"usageCount")?,decode)?;
 Ok(())
}
fn _knowledge_record(value:&mut DslValue,decode:bool)->Result<(),String>{
 unsigned(field(value,"usageCount")?,decode)?;
 Ok(())
}
pub(crate) fn convert(mut value:DslValue,decode:bool)->Result<DslValue,String>{
 rows(field(&mut value,"accessibility")?,_accessibility_requirement,decode)?;
 rows(field(&mut value,"adjacencies")?,_adjacency,decode)?;
 rows(field(&mut value,"benchmarksPayload")?,_benchmark_record,decode)?;
 rows(field(&mut value,"changes")?,_change_record,decode)?;
 rows(field(&mut value,"conflicts")?,_conflict,decode)?;
 rows(field(&mut value,"costs")?,_cost_requirement,decode)?;
 rows(field(&mut value,"decisions")?,_decision,decode)?;
 rows(field(&mut value,"environmental")?,_environmental_requirement,decode)?;
 rows(field(&mut value,"equipment")?,_equipment,decode)?;
 rows(field(&mut value,"flexibility")?,_flexibility_requirement,decode)?;
 rows(field(&mut value,"flows")?,_flow_requirement,decode)?;
 rows(field(&mut value,"growth")?,_growth_plan,decode)?;
 rows(field(&mut value,"infrastructure")?,_infrastructure_requirement,decode)?;
 rows(field(&mut value,"operations")?,_operational_requirement,decode)?;
 rows(field(&mut value,"options")?,_option_evaluation,decode)?;
 rows(field(&mut value,"performance")?,_performance_criterion,decode)?;
 rows(field(&mut value,"priorities")?,_priority_record,decode)?;
 rows(field(&mut value,"quantities")?,_quantity_requirement,decode)?;
 rows(field(&mut value,"relationships")?,_relationship,decode)?;
 rows(field(&mut value,"requirements")?,_requirement,decode)?;
 rows(field(&mut value,"resources")?,_resource,decode)?;
 rows(field(&mut value,"risks")?,_risk,decode)?;
 rows(field(&mut value,"scenarios")?,_scenario,decode)?;
 rows(field(&mut value,"siteContext")?,_site_context,decode)?;
 rows(field(&mut value,"statusRecords")?,_status_record,decode)?;
 rows(field(&mut value,"storage")?,_storage_requirement,decode)?;
 rows(field(&mut value,"surveys")?,_survey,decode)?;
 rows(field(&mut value,"sustainability")?,_sustainability_requirement,decode)?;
 rows(field(&mut value,"wayfinding")?,_wayfinding_requirement,decode)?;
 rows(field(&mut value,"workshops")?,_workshop,decode)?;
 rows(field(&mut value,"activities")?,_activity,decode)?;
 rows(field(&mut value,"functions")?,_function,decode)?;
 rows(field(&mut value,"elements")?,_program_element,decode)?;
 rows(field(&mut value,"organizational")?,_organizational_requirement,decode)?;
 rows(field(&mut value,"services")?,_service_requirement,decode)?;
 rows(field(&mut value,"analyses")?,_analysis_record,decode)?;
 rows(field(&mut value,"searchFilters")?,_search_filter,decode)?;
 rows(field(&mut value,"templates")?,_template_record,decode)?;
 rows(field(&mut value,"knowledgePayload")?,_knowledge_record,decode)?;
 Ok(value)
}

