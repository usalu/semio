//! 📏️ Energy native syntax admits each authored relational cell before either typed owner.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R,DslField};
use semio_framework_value::{DslValue as D,FromValue,NativeDecodeControl as N,ValueRefusalKind};
use store::sqlite_snapshot::SqliteDatabaseLimits;
type Result<T>=std::result::Result<T,ValueError>;
type Roles<'a>=&'a[(&'a str,Role)];
#[derive(Clone,Copy)]enum Role{U8,U16,U32,Bool,Text,Float,OptionalId,OptionalFloat,Enum(&'static[&'static str])}
use Role::*;
const WIDTHS:&[(&str,usize)]=&[
("energy_document",27),
("energy_structure_child",6),
("energy_zones_child",6),
("energy_referenced_model_link",12),
("energy_weather_link",12),
("energy_ground_month",9),
("energy_zone",11),
("energy_space",9),
("energy_surface",13),
("energy_surface_vertex",12),
("energy_fenestration",43),
("energy_fenestration_vertex",12),
("energy_material",27),
("energy_glazing_material",38),
("energy_gas_material",9),
("energy_construction",5),
("energy_construction_layer",4),
("energy_people",19),
("energy_lighting",18),
("energy_equipment",15),
("energy_thermostat",13),
("energy_humidistat",13),
("energy_setpoint_manager",7),
("energy_setpoint_outdoor_reset",13),
("energy_ideal_loads",23),
("energy_zone_equipment",13),
("energy_air_loop",10),
("energy_air_loop_terminal",4),
("energy_plant_loop",15),
("energy_plant_loop_equipment",4),
("energy_outdoor_air_system",9),
("energy_infiltration",34),
("energy_mechanical_ventilation",15),
("energy_shading_surface",6),
("energy_shading_vertex",12),
("energy_space_list",5),
("energy_space_list_member",4),
("energy_thermal_enclosure",5),
("energy_thermal_enclosure_zone",4),
("energy_adjacency_pair",5),
("energy_airflow_network",2),
("energy_airflow_zone_node",5),
("energy_airflow_link",4),
("energy_electrical_load_center",5),
("energy_electrical_generator",4),
("energy_electrical_pv",4),
("energy_electrical_battery",4),
("energy_pv_system",22),
("energy_battery",16),
("energy_shw_system",14),
("energy_solar_thermal",19),
("energy_refrigeration",9),
("energy_water_system",9),
("energy_fault",10),
("energy_output_variable",6),
("energy_sizing_object",7),
("energy_daylight_zone",14),
("energy_room_air_model",5),
("energy_constant_schedule",7),
("energy_daily_schedule",5),
("energy_daily_hour",6),
("energy_daily_limits",7),
("energy_weekly_schedule",4),
("energy_weekly_day",4),
("energy_annual_schedule",6),
("energy_annual_rule",8),
("energy_annual_holiday",6),
("energy_time_series_schedule",5),
("energy_time_series_value",6),
];
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Energy semantic extent overflow"))}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{
  n.step()?;let width=WIDTHS.iter().find(|entry|entry.0==table).ok_or_else(||invalid("Energy census table is not authored"))?.1;
  let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;
  if width>self.limits.max_columns||rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Energy semantic row or column limit exceeded"));}
  if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Energy semantic value limit exceeded"));}
  self.rows=rows;self.bytes=bytes;Ok(())
 }
}
fn scalar<T:FromValue>(value:&D,n:&mut N<'_>)->Result<T>{n.scoped_stage(|n|{n.begin_stage(0)?;T::from_value_controlled(value,n)})}
fn member<'a>(value:&'a D,key:&str,n:&mut N<'_>)->Result<&'a D>{let D::Object(fields)=value else{return Err(invalid("Energy native role requires literal object"))};let mut found=None;for(name,value)in fields{n.step()?;if name==key{if found.is_some(){return Err(invalid("Energy native role is duplicated"));}found=Some(value);}}found.ok_or_else(||invalid("Energy required native role is absent"))}
fn items(value:&D)->Result<&[D]>{match value{D::Array(values)=>Ok(values),_=>Err(invalid("Energy native role requires array"))}}
fn fixed(value:&D,length:usize)->Result<&[D]>{let values=items(value)?;if values.len()!=length{return Err(invalid("Energy fixed native array length differs"));}Ok(values)}
fn text(value:&D)->Result<&str>{match value{D::String(value)=>Ok(value),_=>Err(invalid("Energy native role requires text"))}}
fn cost(value:&D,role:Role,n:&mut N<'_>)->Result<usize>{match role{
 U8=>{let _=scalar::<u8>(value,n)?;Ok(8)},U16=>{let _=scalar::<u16>(value,n)?;Ok(8)},U32=>{let _=scalar::<u32>(value,n)?;Ok(8)},Bool=>{let _=scalar::<bool>(value,n)?;Ok(8)},
 Text=>{n.step()?;Ok(text(value)?.len())},Float=>{let number=scalar::<f64>(value,n)?;Ok(if number.is_nan(){11}else if number.is_infinite(){32}else{22})},
 OptionalId=>if matches!(value,D::Null){n.step()?;Ok(0)}else{cost(value,U32,n)},OptionalFloat=>if matches!(value,D::Null){n.step()?;Ok(0)}else{cost(value,Float,n)},
 Enum(values)=>{let value=text(value)?;n.step()?;if !values.contains(&value){return Err(invalid("Energy native enum scalar differs"));}Ok(value.len())}
}}
fn fields(value:&D,roles:Roles<'_>,n:&mut N<'_>)->Result<usize>{let mut bytes=0;for(name,role)in roles{bytes=add(bytes,cost(member(value,name,n)?,*role,n)?)?;}Ok(bytes)}
fn entity(table:&str,value:&D,roles:Roles<'_>,c:&mut Census,n:&mut N<'_>)->Result<()>{c.row(table,add(24,fields(value,roles,n)?)?,n)}
fn collection(model:&D,key:&str,table:&str,roles:Roles<'_>,c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(member(model,key,n)?)?{entity(table,value,roles,c,n)?;}Ok(())}
fn ids(value:&D,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{c.row(table,add(24,cost(value,U32,n)?)?,n)?;}Ok(())}
fn vertices(value:&D,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{let mut bytes=24;for value in fixed(value,3)?{bytes=add(bytes,cost(value,Float,n)?)?;}c.row(table,bytes,n)?;}Ok(())}
fn required(value:Option<&F>)->Result<&F>{value.ok_or_else(||invalid("Energy required record role is absent"))}
fn record(value:Option<&F>)->Result<&R>{match required(value)?{F::Record(value)=>Ok(value),_=>Err(invalid("Energy native role requires record"))}}
fn record_text(value:Option<&F>)->Result<&str>{match required(value)?{F::Text(value)=>Ok(value),_=>Err(invalid("Energy native record role requires text"))}}
fn shape(value:&R,count:u16)->Result<()>{if value.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("Energy native record field map differs"))}}
fn reference(value:&R)->Result<usize>{shape(value,4)?;let mut bytes=0;for id in 0..4{bytes=add(bytes,record_text(value.get(id))?.len())?;}Ok(bytes)}
fn child(value:Option<&F>,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{let value=record(value)?;shape(value,2)?;c.row(table,add(8,add(record_text(value.get(0))?.len(),reference(record(value.get(1))?)?)?)?,n)}
fn link(value:Option<&F>,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{
 if matches!(required(value)?,F::Absent){return Ok(())}let value=record(value)?;shape(value,3)?;let pin=record(value.get(1))?;let F::Enum(kind)=required(pin.get(0))? else{return Err(invalid("Energy link pin requires enum"))};
 let(tag,active)=match kind{0=>("head",&[0u16][..]),1=>("checkpoint",&[0u16,1][..]),2=>("snapshot",&[0u16,2,3,4][..]),_=>return Err(invalid("Energy link pin kind differs"))};
 if pin.fields.keys().any(|id|*id>4)||active.iter().any(|id|matches!(pin.get(*id),None|Some(F::Absent)))||pin.fields.iter().any(|(id,value)|!matches!(value,F::Absent)&&!active.contains(id)){return Err(invalid("Energy link pin payload differs"));}
 let mut bytes=add(8,add(reference(record(value.get(0))?)?,add(record_text(value.get(2))?.len(),tag.len())?)?)?;
 match kind{0=>{},1=>bytes=add(bytes,record_text(pin.get(1))?.len())?,2=>{let F::UInt(_)=required(pin.get(3))? else{return Err(invalid("Energy link blob size requires unsigned64"))};bytes=add(bytes,add(16,add(record_text(pin.get(2))?.len(),record_text(pin.get(4))?.len())?)?)?;},_=>unreachable!()}
 c.row(table,bytes,n)
}
fn envelope(m:&D,c:&mut Census,n:&mut N<'_>)->Result<()>{
 collection(m,"zones","energy_zone",&[("id",U32),("name",Text),("volume_m3",Float),("multiplier",U32),("conditioned",Bool),("part_of_total_floor_area",Bool)],c,n)?;
 collection(m,"spaces","energy_space",&[("id",U32),("name",Text),("zone_id",U32),("floor_area_m2",Float)],c,n)?;
 for v in items(member(m,"surfaces",n)?)?{
  let boundary=member(v,"outside_boundary_condition",n)?;let extra=match boundary{D::String(value)=>{if !["OutdoorAir","Ground","OtherSideTemperature","Adiabatic"].contains(&value.as_str()){return Err(invalid("Energy outside boundary differs"));}value.len()},D::Object(values)if values.len()==1&&values[0].0=="Interzone"=>add(9,cost(&values[0].1,U32,n)?)?,_=>return Err(invalid("Energy outside boundary payload differs"))};
  let bytes=fields(v,&[("id",U32),("name",Text),("zone_id",U32),("class",Enum(&["ExteriorWall","InteriorWall","Roof","Ceiling","Floor","Interzone","Adiabatic","Ground"])),("construction_id",U32),("sun_exposed",Bool),("wind_exposed",Bool),("multiplier",U32)],n)?;
  c.row("energy_surface",add(24,add(bytes,extra)?)?,n)?;vertices(member(v,"vertices_m",n)?,"energy_surface_vertex",c,n)?;
 }
 for v in items(member(m,"fenestrations",n)?)?{entity("energy_fenestration",v,&[("id",U32),("name",Text),("surface_id",U32),("u_value_w_m2k",Float),("shgc",Float),("vlt",Float),("area_m2",Float),("height_m",Float),("sill_height_m",Float),("frame_conductance_w_k",Float),("divider_conductance_w_k",Float),("overhang_depth_m",Float),("overhang_offset_m",Float),("fin_depth_m",Float),("fin_offset_m",Float),("glazing_construction_id",OptionalId)],c,n)?;vertices(member(v,"vertices_m",n)?,"energy_fenestration_vertex",c,n)?;}
 collection(m,"materials","energy_material",&[("id",U32),("name",Text),("roughness",Enum(&["VeryRough","Rough","MediumRough","MediumSmooth","Smooth","VerySmooth"])),("thickness_m",Float),("conductivity_w_m_k",Float),("density_kg_m3",Float),("specific_heat_j_kg_k",Float),("thermal_absorptance",Float),("solar_absorptance",Float),("visible_absorptance",Float)],c,n)?;
 collection(m,"glazing_materials","energy_glazing_material",&[("id",U32),("name",Text),("thickness_m",Float),("conductivity_w_m_k",Float),("solar_transmittance",Float),("solar_reflectance_front",Float),("solar_reflectance_back",Float),("visible_transmittance",Float),("visible_reflectance_front",Float),("visible_reflectance_back",Float),("infrared_transmittance",Float),("infrared_emissivity_front",Float),("infrared_emissivity_back",Float)],c,n)?;
 collection(m,"gas_materials","energy_gas_material",&[("id",U32),("name",Text),("thickness_m",Float),("gas",Enum(&["Air","Argon","Krypton","Xenon"]))],c,n)?;
 for(key,table,relation,items_key)in[("constructions","energy_construction","energy_construction_layer","layer_material_ids"),("space_lists","energy_space_list","energy_space_list_member","space_ids"),("thermal_enclosures","energy_thermal_enclosure","energy_thermal_enclosure_zone","zone_ids")]{for v in items(member(m,key,n)?)?{entity(table,v,&[("id",U32),("name",Text)],c,n)?;ids(member(v,items_key,n)?,relation,c,n)?;}}
 for v in items(member(m,"shading_surfaces",n)?)?{entity("energy_shading_surface",v,&[("id",U32),("name",Text),("transmittance_schedule_id",OptionalId)],c,n)?;vertices(member(v,"vertices_m",n)?,"energy_shading_vertex",c,n)?;}
 collection(m,"adjacency_pairs","energy_adjacency_pair",&[("surface_a_id",U32),("surface_b_id",U32)],c,n)
}
fn systems(m:&D,c:&mut Census,n:&mut N<'_>)->Result<()>{
 collection(m,"people","energy_people",&[("id",U32),("zone_id",U32),("schedule_id",U32),("activity_schedule_id",U32),("people_per_area",Float),("sensible_fraction",Float),("latent_fraction",Float),("radiant_fraction",Float)],c,n)?;
 collection(m,"lighting","energy_lighting",&[("id",U32),("zone_id",U32),("schedule_id",U32),("watts_per_area",Float),("radiant_fraction",Float),("visible_fraction",Float),("return_air_fraction",Float)],c,n)?;
 collection(m,"equipment","energy_equipment",&[("id",U32),("zone_id",U32),("schedule_id",U32),("watts_per_area",Float),("radiant_fraction",Float),("latent_fraction",Float)],c,n)?;
 collection(m,"thermostats","energy_thermostat",&[("id",U32),("zone_id",U32),("heating_setpoint_schedule_id",U32),("cooling_setpoint_schedule_id",U32),("heating_throttle_range_k",Float),("cooling_throttle_range_k",Float)],c,n)?;
 collection(m,"humidistats","energy_humidistat",&[("id",U32),("zone_id",U32),("humidifying_setpoint_schedule_id",U32),("dehumidifying_setpoint_schedule_id",U32),("humidifying_throttle_range",Float),("dehumidifying_throttle_range",Float)],c,n)?;
 for v in items(member(m,"setpoint_managers",n)?)?{
  let kind=member(v,"kind",n)?;let tag=match kind{D::String(value)=>{if !["Scheduled","WarmestZone","ColdestZone"].contains(&value.as_str()){return Err(invalid("Energy setpoint scalar differs"));}value.as_str()},D::Object(values)if values.len()==1&&values[0].0=="OutdoorAirReset"=>{c.row("energy_setpoint_outdoor_reset",add(8,fields(&values[0].1,&[("low_outdoor_c",Float),("high_outdoor_c",Float),("low_setpoint_c",Float),("high_setpoint_c",Float)],n)?)?,n)?;"OutdoorAirReset"},_=>return Err(invalid("Energy setpoint payload differs"))};
  c.row("energy_setpoint_manager",add(24,add(fields(v,&[("id",U32),("name",Text),("schedule_id",OptionalId)],n)?,tag.len())?)?,n)?;
 }
 collection(m,"ideal_loads","energy_ideal_loads",&[("id",U32),("zone_id",U32),("max_heating_supply_air_temp_c",Float),("min_cooling_supply_air_temp_c",Float),("max_heating_capacity_w",OptionalFloat),("max_cooling_capacity_w",OptionalFloat),("outdoor_air_per_person_m3_s",Float),("outdoor_air_per_area_m3_s_m2",Float)],c,n)?;
 collection(m,"zone_equipment","energy_zone_equipment",&[("id",U32),("zone_id",U32),("equipment_type",Enum(&["Baseboard","Radiant","FanCoil","Ptac","VrfTerminal","Erv","UnitHeater","WaterToAirHp"])),("priority",U8),("heating_capacity_w",Float),("cooling_capacity_w",Float)],c,n)?;
 for v in items(member(m,"air_loops",n)?)?{entity("energy_air_loop",v,&[("id",U32),("name",Text),("supply_node_id",U32),("return_node_id",U32),("design_supply_air_flow_m3_s",Float)],c,n)?;ids(member(v,"terminal_zone_ids",n)?,"energy_air_loop_terminal",c,n)?;}
 for v in items(member(m,"plant_loops",n)?)?{entity("energy_plant_loop",v,&[("id",U32),("name",Text),("loop_type",Enum(&["Heating","Cooling","Condenser"])),("supply_temperature_c",Float),("return_temperature_c",Float),("design_flow_kg_s",Float)],c,n)?;ids(member(v,"equipment_ids",n)?,"energy_plant_loop_equipment",c,n)?;}
 collection(m,"outdoor_air_systems","energy_outdoor_air_system",&[("id",U32),("air_loop_id",U32),("min_oa_flow_m3_s",Float),("economizer_enabled",Bool)],c,n)?;
 collection(m,"infiltrations","energy_infiltration",&[("id",U32),("zone_id",U32),("schedule_id",U32),("method",Enum(&["ScheduledAch","PerExteriorArea","EffectiveLeakageArea","WindAndStack"])),("design_flow_ach",Float),("flow_per_exterior_area_m3_s_m2",Float),("effective_leakage_area_m2",Float),("discharge_coefficient",Float),("stack_height_m",Float),("constant_term_coefficient",Float),("temperature_term_coefficient",Float),("velocity_term_coefficient",Float),("velocity_squared_term_coefficient",Float)],c,n)?;
 collection(m,"mechanical_ventilations","energy_mechanical_ventilation",&[("id",U32),("zone_id",U32),("schedule_id",U32),("design_flow_m3_s",Float),("fan_total_efficiency",Float),("fan_delta_pressure_pa",Float)],c,n)?;
 let network=member(m,"airflow_network",n)?;if !matches!(network,D::Null){c.row("energy_airflow_network",add(8,fields(network,&[("outdoor_node_id",U32)],n)?)?,n)?;for v in items(member(network,"zone_node_ids",n)?)?{let v=fixed(v,2)?;c.row("energy_airflow_zone_node",add(24,add(cost(&v[0],U32,n)?,cost(&v[1],U32,n)?)?)?,n)?;}ids(member(network,"link_ids",n)?,"energy_airflow_link",c,n)?;}
 for v in items(member(m,"electrical_load_centers",n)?)?{entity("energy_electrical_load_center",v,&[("id",U32),("name",Text)],c,n)?;for(key,table)in[("generator_ids","energy_electrical_generator"),("pv_ids","energy_electrical_pv"),("battery_ids","energy_electrical_battery")]{ids(member(v,key,n)?,table,c,n)?;}}
 collection(m,"pv_systems","energy_pv_system",&[("id",U32),("dc_capacity_w",Float),("area_m2",Float),("tilt_deg",Float),("azimuth_deg",Float),("module_efficiency",Float),("inverter_efficiency",Float)],c,n)?;
 collection(m,"battery_storage","energy_battery",&[("id",U32),("capacity_kwh",Float),("max_charge_w",Float),("max_discharge_w",Float),("round_trip_efficiency",Float)],c,n)?;
 collection(m,"shw_systems","energy_shw_system",&[("id",U32),("heater_capacity_w",Float),("storage_volume_m3",Float),("setpoint_c",Float),("schedule_id",U32)],c,n)?;
 collection(m,"solar_thermal_systems","energy_solar_thermal",&[("id",U32),("collector_area_m2",Float),("efficiency",Float),("storage_volume_m3",Float),("tilt_deg",Float),("azimuth_deg",Float)],c,n)?;
 collection(m,"refrigeration_systems","energy_refrigeration",&[("id",U32),("case_count",U32),("design_load_w",Float),("defrost_schedule_id",U32)],c,n)?;
 collection(m,"water_systems","energy_water_system",&[("id",U32),("fixture_count",U32),("peak_flow_l_s",Float),("schedule_id",U32)],c,n)?;
 collection(m,"faults","energy_fault",&[("id",U32),("target_equipment_id",U32),("fault_type",Enum(&["SensorBias","CoilFouling","DamperStuck","ChillerFouling","BoilerEfficiencyDegradation"])),("severity",Float),("start_schedule_id",U32)],c,n)?;
 collection(m,"output_variables","energy_output_variable",&[("name",Text),("key",Text),("reporting_frequency",Enum(&["Timestep","Hourly","Daily","Monthly","RunPeriod"]))],c,n)?;
 collection(m,"sizing_objects","energy_sizing_object",&[("id",U32),("zone_id",U32),("sizing_type",Enum(&["Heating","Cooling","OutdoorAir"])),("design_day_type",Enum(&["Heating","Cooling"]))],c,n)?;
 collection(m,"daylight_zones","energy_daylight_zone",&[("id",U32),("zone_id",U32),("illuminance_target_lux",Float),("glare_limit",Float),("window_transmittance",Float)],c,n)?;
 collection(m,"room_air_models","energy_room_air_model",&[("zone_id",U32),("model",Enum(&["WellMixed","OneNodeDisplacement","TwoNodeBuoyancy","UnderFloorAirDistribution"]))],c,n)
}
fn schedules(s:&D,c:&mut Census,n:&mut N<'_>)->Result<()>{
 collection(s,"constants","energy_constant_schedule",&[("id",U32),("value",Float)],c,n)?;
 for v in items(member(s,"daily",n)?)?{entity("energy_daily_schedule",v,&[("id",U32),("interpolation",Enum(&["Continuous","Discrete"]))],c,n)?;for value in fixed(member(v,"hourly_values",n)?,24)?{c.row("energy_daily_hour",add(24,cost(value,Float,n)?)?,n)?;}let limits=member(v,"limits",n)?;if !matches!(limits,D::Null){c.row("energy_daily_limits",add(8,fields(limits,&[("min",Float),("max",Float)],n)?)?,n)?;}}
 for v in items(member(s,"weekly",n)?)?{entity("energy_weekly_schedule",v,&[("id",U32)],c,n)?;for value in fixed(member(v,"daily_schedule_ids",n)?,7)?{c.row("energy_weekly_day",add(24,cost(value,U32,n)?)?,n)?;}}
 for v in items(member(s,"annual",n)?)?{entity("energy_annual_schedule",v,&[("id",U32),("default_daily_schedule_id",U32),("holiday_daily_schedule_id",OptionalId)],c,n)?;for rule in items(member(v,"rules",n)?)?{entity("energy_annual_rule",rule,&[("start_month",U8),("start_day",U8),("end_month",U8),("end_day",U8),("daily_schedule_id",U32)],c,n)?;}for date in items(member(v,"holiday_dates",n)?)?{let date=fixed(date,3)?;c.row("energy_annual_holiday",add(24,add(cost(&date[0],U16,n)?,add(cost(&date[1],U8,n)?,cost(&date[2],U8,n)?)?)?)?,n)?;}}
 for v in items(member(s,"time_series",n)?)?{entity("energy_time_series_schedule",v,&[("id",U32),("timestep_seconds",U32)],c,n)?;for value in items(member(v,"values",n)?)?{c.row("energy_time_series_value",add(24,cost(value,Float,n)?)?,n)?;}}Ok(())
}
/// 🏛️ Admits the actual complete handwritten Energy schema extent before native work.
pub(super)fn admit_extent(limits:SqliteDatabaseLimits)->Result<()>{if WIDTHS.len()>limits.max_tables||WIDTHS.iter().any(|(_,width)|*width>limits.max_columns)||SCHEMA.len()>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Energy authored schema exceeds caller limits"));}Ok(())}
/// 🔋️ Completes every SQL semantic role on borrowed native syntax without building any domain shadow.
pub(super)fn admit_record(source:&R,n:&mut N<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 n.scoped_stage(|n|{n.begin_stage(0)?;admit_extent(limits)?;let mut c=Census{limits,rows:0,bytes:0};let F::Value(model)=required(source.get(1))? else{return Err(invalid("Energy native model field differs"))};
 let site=member(model,"site",n)?;let ground=member(model,"ground_temperature",n)?;let period=member(model,"run_period",n)?;
 let bytes=add(8,add(record_text(source.get(0))?.len(),add(fields(model,&[("name",Text),("version",Text)],n)?,add(fields(site,&[("latitude_deg",Float),("longitude_deg",Float),("elevation_m",Float),("time_zone_hours",Float),("north_axis_deg",Float)],n)?,add(fields(ground,&[("deep_c",Float)],n)?,fields(period,&[("start_month",U8),("start_day",U8),("end_month",U8),("end_day",U8),("year",U16)],n)?)?)?)?)?)?;
 c.row("energy_document",bytes,n)?;child(source.get(2),"energy_structure_child",&mut c,n)?;child(source.get(3),"energy_zones_child",&mut c,n)?;link(source.get(4),"energy_referenced_model_link",&mut c,n)?;link(source.get(5),"energy_weather_link",&mut c,n)?;
 let surface=fixed(member(ground,"building_surface_c",n)?,12)?;let shallow=fixed(member(ground,"shallow_c",n)?,12)?;for (left,right)in surface.iter().zip(shallow){c.row("energy_ground_month",add(24,add(cost(left,Float,n)?,cost(right,Float,n)?)?)?,n)?;}
 envelope(model,&mut c,n)?;systems(model,&mut c,n)?;schedules(member(model,"schedules",n)?,&mut c,n)?;n.checkpoint()
 })
}
