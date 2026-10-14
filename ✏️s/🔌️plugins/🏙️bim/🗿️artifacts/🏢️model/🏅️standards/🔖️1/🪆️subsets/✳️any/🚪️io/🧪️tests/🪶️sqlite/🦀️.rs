//! 🏢️ Original BIM public IO owns every authored SQLite role.
#[path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs"]
mod original_owners;

#[test]
fn sqlite_snapshot_bim_original_public_sql_edits_are_authoritative(){
 let provider=crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let original=fixture();let input=payload(&original,encoding);let database=original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let edited=oracle(&bytes,"edit",None);assert!(edited.starts_with(b"SQLite format 3\0"));let database=import_sqlite_database(&edited,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let output=original_owners::with_encoding(|native_owner|(provider.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT, &mut Some(database),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).expect("actual public import reads independently edited SQL").value;let mut expected=original;expected.project.name="SQLite edited BIM".into();*expected.properties.get_mut("external/element").unwrap().get_mut("set/λ").unwrap().get_mut("Length").unwrap()=crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Length{value:9.75};assert_eq!(decode(&output),expected);println!("[DEBUG] BIM original public {encoding:?} physical Bun SQLite project and typed Length edits are authoritative");}
}
#[test]
fn sqlite_snapshot_bim_original_public_limits_and_cancellation(){
 let provider=crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");let snapshot=fixture();let schema_bytes=include_str!("../../🪶️sqlite/📸️snapshot/🗄️.sql").len();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&snapshot,encoding);for(limits,expected)in[
 (SqliteDatabaseLimits{max_rows:3972,max_tables:93,max_columns:59,..SqliteDatabaseLimits::default()},true),
 (SqliteDatabaseLimits{max_rows:3971,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_tables:92,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_columns:58,..SqliteDatabaseLimits::default()},false),
 (SqliteDatabaseLimits{max_schema_bytes:schema_bytes-1,..SqliteDatabaseLimits::default()},false)]{let result=original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner));assert_eq!(result.is_ok(),expected,"public {encoding:?} {limits:?}");}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.completed>=256{reached=true;false}else{true}};assert!(original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default()), native_owner)).is_err(),"original public export cancels");assert!(reached);
 let database=original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;for limits in[SqliteDatabaseLimits{max_rows:3971,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_tables:92,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_columns:58,..SqliteDatabaseLimits::default()},SqliteDatabaseLimits{max_schema_bytes:schema_bytes-1,..SqliteDatabaseLimits::default()}]{assert!(original_owners::with_encoding(|native_owner|(provider.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT, &mut Some(database.clone()),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits), native_owner)).is_err(),"original public import refuses one-short");}
 let mut reached=false;let mut callback=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{if event.completed>=256{reached=true;false}else{true}};assert!(original_owners::with_encoding(|native_owner|(provider.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT, &mut Some(database),encoding,&mut SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default()), native_owner)).is_err(),"original public import cancels");assert!(reached);println!("[DEBUG] BIM original public {encoding:?} exact3972/93/59 and independent one-short structural/schema limits plus export/import cancellation");}
}
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, mutations::ModelMutation, BIM_MODEL_DOCUMENT_SCHEMA, BIM_MODEL_DIALECT};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits,SqliteSnapshotControl,export_sqlite_database,import_sqlite_database};
use store::io_schema::IoPayload;
fn fixture()->ModelSnapshot{let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();semio_framework_pack_json::from_json_str(&corpus["cases"][0]["snapshot"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("original all-role schema fixture binds")}
fn payload(snapshot:&ModelSnapshot,encoding:SnapshotEncoding)->IoPayload{match encoding{SnapshotEncoding::Binary=>IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(snapshot)),SnapshotEncoding::Text=>IoPayload::Text(pack::snapshot::ArtifactDsl::print_dsl(snapshot))}}
fn decode(payload:&IoPayload)->ModelSnapshot{match payload{IoPayload::Binary(bytes)=><ModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).unwrap(),IoPayload::Text(text)=><ModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).unwrap()}}
fn oracle(bytes:&[u8],mode:&str,word:Option<&str>)->Vec<u8>{use std::{io::Write,process::{Command,Stdio}};let mut command=Command::new("bun");command.arg(concat!(env!("CARGO_MANIFEST_DIR"),"/📜️script.ts")).arg("sqlite-oracle").arg(mode);if let Some(word)=word{command.arg(word);}let mut child=command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("registered original Bun SQLite oracle");{let mut input=child.stdin.take().unwrap();input.write_all(bytes).expect("original physical SQLite bytes reach independent oracle");}let result=child.wait_with_output().expect("independent physical SQLite oracle finishes");assert!(result.status.success(),"Bun SQLite oracle: {}",String::from_utf8_lossy(&result.stderr));result.stdout}
const BIT_COLUMNS:&[(&str,&[usize])]=&[
 ("bim_material",&[7,10,13,16,19,22]),
 ("bim_window_type",&[6,9,12,15,18]),
 ("bim_door_type",&[6,9,12,15]),
 ("bim_site",&[6,9,12,15]),
 ("bim_building",&[7,10,13,16]),
 ("bim_storey",&[8,11]),
 ("bim_grid",&[7,10,13,16]),
 ("bim_wall",&[8]),
 ("bim_curtain_wall",&[7]),
 ("bim_column",&[7,10,13,16,19,22]),
 ("bim_beam",&[7,10]),
 ("bim_slab",&[7,10,13]),
 ("bim_roof",&[8,11,14,17,20,23,26,29]),
 ("bim_opening",&[9,12,15,18,21,24,30]),
 ("bim_stair",&[6,9,12,15,19,23,26,29,32,35,39,42,45,48,52]),
 ("bim_railing",&[6,9,13,17,23]),
 ("bim_space",&[9,12]),
 ("bim_wall_layer",&[5]),
 ("bim_slab_layer",&[5]),
 ("bim_roof_layer",&[5]),
 ("bim_site_boundary",&[4,7]),
 ("bim_axis",&[6,9,12,15,18]),
 ("bim_top",&[8,11]),
 ("bim_baluster",&[3]),
 ("bim_profile",&[11,14,17,20,23]),
 ("bim_profile_outline",&[4,7,10]),
 ("bim_slab_boundary",&[4,7,10]),
 ("bim_slab_hole_vertex",&[4,7,10]),
 ("bim_roof_footprint",&[4,7,10]),
 ("bim_railing_path",&[4,7]),
 ("bim_space_outline",&[4,7,10]),
 ("bim_property_value",&[9]),
 ("bim_ramp",&[6,9,12,15,18,21,25]),
 ("bim_ceiling",&[7,10,13]),
 ("bim_zone",&[7]),
 ("bim_view",&[9,12,15,18,21,24,27,30,33,36,39,42,45,48,51,54]),
 ("bim_sheet",&[9,12]),
 ("bim_viewport",&[7,10,14,17,20,23]),
 ("bim_dimension",&[6,9,13]),
 ("bim_tag",&[8,11]),
 ("bim_text_note",&[6,9,13]),
 ("bim_leader",&[6,9]),
 ("bim_annotation_style",&[6,12,15,18]),
 ("bim_wall_sweep",&[7,10]),
 ("bim_curtain_grid",&[7]),
 ("bim_curtain_grid_position",&[4]),
 ("bim_ceiling_layer",&[5]),
 ("bim_ceiling_boundary",&[4,7,10]),
 ("bim_ceiling_hole_vertex",&[4,7,10]),
 ("bim_ramp_path",&[4,7,10]),
 ("bim_annotation_anchor",&[6,9]),
 ("bim_property_def",&[9,12]),
 ("bim_property_default",&[7]),
 ("bim_property_allowed",&[8]),
];
fn word_cells(database:&store::sqlite_snapshot::SqliteDatabase,bits:u64)->usize{assert_eq!(BIT_COLUMNS.iter().map(|(_,columns)|columns.len()).sum::<usize>(),184);let mut checked=0;for(table,columns)in BIT_COLUMNS{let table=database.table(table).expect("original named binary64 table");for index in *columns{let mut populated=0;for row in &table.rows{match &row.values[*index]{store::sqlite_snapshot::SqliteValue::Null=>{},store::sqlite_snapshot::SqliteValue::Integer(word)=>{assert_eq!(*word as u64,bits,"{} wordcolumn{} row{}",table.name,index,row.rowid);checked+=1;populated+=1;},_=>panic!("binary64 word is not exact integer")}}assert!(populated>0,"{}.wordcolumn{} has no independent witness",table.name,index);}}assert!(checked>0);checked}
#[test]
fn sqlite_snapshot_bim_original_public_io_owns_current_model(){
 let bare=store::bare_artifact_codec::<ModelSnapshot,ModelMutation>(BIM_MODEL_DOCUMENT_SCHEMA);
 assert!(bare.snapshot_sqlite.is_some(),"original BIM bare factory has no authored SQLite capability");
 let declaration=crate::standards::v1::subsets::any::io::io();
 let provider=declaration.native.codec.snapshot_sqlite.expect("original BIM public IO has no authored SQLite capability");
 assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<ModelSnapshot>()));
 let counts:serde_json::Value=serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/📏️counts/🔣️.json")).unwrap();
 let expected=fixture();
 let roles=expected.__dsl_to_record();assert_eq!(roles.fields.len(),49,"original current model fields");
 for id in 2..49{let Some(semio_framework_dsl_record::FieldValue::Map(values))=roles.get(id)else{panic!("original collection role {id} is absent")};assert!(!values.is_empty(),"current collection role {id} has no independent witness");}
 let table_count=counts["tableCount"].as_u64().unwrap()as usize;let row_count=counts["totalRows"].as_u64().unwrap()as usize;
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let input=payload(&expected,encoding);let mut progress=0usize;
  let database=original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut |_|{progress+=1;true},SqliteDatabaseLimits::default()), native_owner)).expect("original native public payload projects").value;
  assert_eq!(database.tables.len(),table_count);assert_eq!(database.tables.iter().map(|table|table.rows.len()).sum::<usize>(),row_count);
  for(name,count)in counts["tables"].as_object().unwrap(){assert_eq!(database.table(name).unwrap().rows.len(),count.as_u64().unwrap()as usize,"{name}");}
  assert!(progress>0,"original public route emits progress");
  let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert!(bytes.starts_with(b"SQLite format 3\0"));
  let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
  let output=original_owners::with_encoding(|native_owner|(provider.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT, &mut Some(database),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).expect("original public SQLite restores").value;
  assert_eq!(decode(&output),expected);
  println!("[DEBUG] BIM original public IO encoding={encoding:?} fields=49 collections=47 tables={table_count} rows={row_count} bytes={} progress={progress}",bytes.len());
 }
}
fn point(value:&mut crate::standards::v1::subsets::any::schema::snapshot::Point2,word:f64){value.x=word;value.y=word;}
fn vertices(values:&mut[crate::standards::v1::subsets::any::schema::snapshot::Vertex],word:f64){for value in values{point(&mut value.point,word);value.bulge=word;}}
fn profile(value:&mut crate::standards::v1::subsets::any::schema::snapshot::Profile,word:f64){match value{crate::standards::v1::subsets::any::schema::snapshot::Profile::Rectangle{width,depth}=>{*width=word;*depth=word},crate::standards::v1::subsets::any::schema::snapshot::Profile::Circle{diameter}=>*diameter=word,crate::standards::v1::subsets::any::schema::snapshot::Profile::IShape{width,depth,web,flange}=>{*width=word;*depth=word;*web=word;*flange=word},crate::standards::v1::subsets::any::schema::snapshot::Profile::Custom{outline}=>vertices(outline,word),crate::standards::v1::subsets::any::schema::snapshot::Profile::Family{..}=>{}}}
fn axis(value:&mut crate::standards::v1::subsets::any::schema::snapshot::Axis,word:f64){match value{crate::standards::v1::subsets::any::schema::snapshot::Axis::Line{start,end}=>{point(start,word);point(end,word)},crate::standards::v1::subsets::any::schema::snapshot::Axis::Arc{start,end,bulge}=>{point(start,word);point(end,word);*bulge=word}}}
fn top(value:&mut crate::standards::v1::subsets::any::schema::snapshot::TopConstraint,word:f64){match value{crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::Unconnected{height}=>*height=word,crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::StoreyTop{offset}|crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::Storey{offset,..}|crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::Roof{offset,..}|crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::Slab{offset,..}|crate::standards::v1::subsets::any::schema::snapshot::TopConstraint::Ceiling{offset,..}=>*offset=word}}
fn grid(value:&mut crate::standards::v1::subsets::any::schema::snapshot::CurtainGrid,word:f64){match value{crate::standards::v1::subsets::any::schema::snapshot::CurtainGrid::Spacing{spacing}=>*spacing=word,crate::standards::v1::subsets::any::schema::snapshot::CurtainGrid::Lines{positions}=>positions.fill(word)}}
fn anchor(value:&mut crate::standards::v1::subsets::any::schema::snapshot::AnnotationAnchor,word:f64){if let crate::standards::v1::subsets::any::schema::snapshot::AnnotationAnchor::Point{point:value}=value{point(value,word)}}
fn crop(value:&mut crate::standards::v1::subsets::any::schema::snapshot::ViewCrop,word:f64){point(&mut value.min,word);point(&mut value.max,word)}
fn property_word(value:&mut crate::standards::v1::subsets::any::schema::snapshot::PropertyValue,word:f64){match value{crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Real{value}|crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Length{value}|crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Area{value}|crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Volume{value}|crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Angle{value}=>*value=word,crate::standards::v1::subsets::any::schema::snapshot::PropertyValue::Integer{value}=>*value=i32::MAX,_=>{}}}
fn every_word(s:&mut ModelSnapshot,word:f64){
 for v in s.materials.values_mut(){v.color=crate::standards::v1::subsets::any::schema::snapshot::Rgb{r:word,g:word,b:word};v.density=word;v.conductivity=word;v.specific_heat=word;}
 for v in s.wall_types.values_mut(){for layer in &mut v.layers{layer.thickness=word}}for v in s.slab_types.values_mut(){for layer in &mut v.layers{layer.thickness=word}}for v in s.roof_types.values_mut(){for layer in &mut v.layers{layer.thickness=word}}
 for v in s.column_types.values_mut(){profile(&mut v.profile,word)}for v in s.beam_types.values_mut(){profile(&mut v.profile,word)}
 for v in s.window_types.values_mut(){v.width=word;v.height=word;v.sill=word;v.frame_width=word;v.frame_depth=word;v.panes=u32::MAX;}
 for v in s.door_types.values_mut(){v.width=word;v.height=word;v.frame_width=word;v.frame_depth=word;}
 for v in s.sites.values_mut(){v.latitude=word;v.longitude=word;v.elevation=word;v.true_north=word;for p in &mut v.boundary{point(p,word)}}
 for v in s.buildings.values_mut(){point(&mut v.origin,word);v.rotation=word;v.elevation=word;}
 for v in s.storeys.values_mut(){v.level=i32::MIN;v.height=word;v.cut_height=v.cut_height.map(|_|word);}
 for v in s.grids.values_mut(){point(&mut v.start,word);point(&mut v.end,word)}
 for v in s.walls.values_mut(){axis(&mut v.axis,word);v.base_offset=word;top(&mut v.top,word)}
 for v in s.curtain_walls.values_mut(){axis(&mut v.axis,word);v.base_offset=word;top(&mut v.top,word);if let Some(v)=&mut v.u_grid{grid(v,word)}if let Some(v)=&mut v.v_grid{grid(v,word)}}
 for v in s.curtain_wall_types.values_mut(){grid(&mut v.u_grid,word);grid(&mut v.v_grid,word);profile(&mut v.interior_mullion,word);profile(&mut v.border_mullion,word)}
 for v in s.columns.values_mut(){point(&mut v.position,word);v.rotation=word;v.base_offset=word;top(&mut v.top,word);if let Some(v)=&mut v.tilt{v.direction=word;v.angle=word;}}
 for v in s.beams.values_mut(){axis(&mut v.axis,word);v.top_offset=word;v.end_top_offset=v.end_top_offset.map(|_|word);}
 for v in s.slabs.values_mut(){vertices(&mut v.boundary,word);for hole in &mut v.holes{vertices(hole,word)}v.offset=word;if let Some(slope)=&mut v.slope{slope.direction=word;slope.angle=word;}}
 for v in s.roofs.values_mut(){vertices(&mut v.footprint,word);match &mut v.shape{crate::standards::v1::subsets::any::schema::snapshot::RoofShape::Flat=>{},crate::standards::v1::subsets::any::schema::snapshot::RoofShape::Shed{pitch,direction}=>{*pitch=word;*direction=word},crate::standards::v1::subsets::any::schema::snapshot::RoofShape::Gable{pitch,ridge_direction}=>{*pitch=word;*ridge_direction=word},crate::standards::v1::subsets::any::schema::snapshot::RoofShape::Hip{pitch}=>*pitch=word,crate::standards::v1::subsets::any::schema::snapshot::RoofShape::Mansard{lower_pitch,upper_pitch,break_height}=>{*lower_pitch=word;*upper_pitch=word;*break_height=word}}v.overhang=word;v.base_offset=word;}
 for v in s.openings.values_mut(){if let crate::standards::v1::subsets::any::schema::snapshot::OpeningKind::Void{width,height}=&mut v.kind{*width=word;*height=word}v.offset=word;v.sill_override=v.sill_override.map(|_|word);v.width=v.width.map(|_|word);v.height=v.height.map(|_|word);v.reveal_depth=v.reveal_depth.map(|_|word);}
 for v in s.stairs.values_mut(){point(&mut v.start,word);v.direction=word;v.width=word;match &mut v.flight{crate::standards::v1::subsets::any::schema::snapshot::StairFlight::Straight=>{},crate::standards::v1::subsets::any::schema::snapshot::StairFlight::LTurn{split,..}=>*split=word,crate::standards::v1::subsets::any::schema::snapshot::StairFlight::UTurn{gap}=>*gap=word,crate::standards::v1::subsets::any::schema::snapshot::StairFlight::Spiral{radius,sweep}=>{*radius=word;*sweep=word}}top(&mut v.top,word);v.max_riser=word;v.min_tread=word;v.stringer.width=word;v.stringer.depth=word;v.nosing=word;v.tread_thickness=word;v.landing_depth=word;}
 for v in s.railings.values_mut(){for p in &mut v.path{point(p,word)}v.height=word;v.post_spacing=word;profile(&mut v.profile,word);profile(&mut v.post_profile,word);if let Some(b)=&mut v.baluster{b.spacing=word;profile(&mut b.profile,word)}match &mut v.infill{crate::standards::v1::subsets::any::schema::snapshot::Infill::None=>{},crate::standards::v1::subsets::any::schema::snapshot::Infill::Glass{thickness}|crate::standards::v1::subsets::any::schema::snapshot::Infill::Panel{thickness}=>*thickness=word}v.base_offset=word;if let Some(host)=&mut v.host{host.inset=word;}}
 for v in s.ramps.values_mut(){vertices(&mut v.path,word);v.width=word;v.landing_start=word;v.landing_end=word;v.landing_turn=word;v.max_slope=word;v.thickness=word;v.base_offset=word;top(&mut v.top,word)}
 for v in s.ceiling_types.values_mut(){for layer in &mut v.layers{layer.thickness=word}}for v in s.ceilings.values_mut(){vertices(&mut v.boundary,word);for hole in &mut v.holes{vertices(hole,word)}v.offset=word;if let Some(slope)=&mut v.slope{slope.direction=word;slope.angle=word;}}
 for v in s.spaces.values_mut(){match &mut v.boundary{crate::standards::v1::subsets::any::schema::snapshot::SpaceBoundary::Bounded{seed}=>point(seed,word),crate::standards::v1::subsets::any::schema::snapshot::SpaceBoundary::Explicit{outline}=>vertices(outline,word)}}
 for v in s.zones.values_mut(){v.occupancy_density=word;}
 for v in s.views.values_mut(){if let Some(v)=&mut v.plane{point(&mut v.start,word);point(&mut v.end,word)}if let Some(v)=&mut v.camera{point(&mut v.target,word);v.target_height=word;v.azimuth=word;v.pitch=word;v.distance=word;}v.cut_height=v.cut_height.map(|_|word);v.depth=word;if let Some(v)=&mut v.crop{crop(v,word)}}
 for v in s.sheets.values_mut(){if let crate::standards::v1::subsets::any::schema::snapshot::Paper::Custom{width,height}=&mut v.paper{*width=word;*height=word}}
 for v in s.viewports.values_mut(){point(&mut v.position,word);if let Some(v)=&mut v.crop{crop(v,word)}}
 for v in s.dimensions.values_mut(){for v in &mut v.anchors{anchor(v,word)}v.angle=word;v.offset=word;v.lock=v.lock.map(|_|word);}
 for v in s.tags.values_mut(){point(&mut v.offset,word)}for v in s.text_notes.values_mut(){point(&mut v.position,word);v.rotation=word;}
 for v in s.leaders.values_mut(){anchor(&mut v.anchor,word);point(&mut v.offset,word)}
 for v in s.annotation_styles.values_mut(){v.text_height=word;v.mark_size=word;v.gap=word;v.overshoot=word;}
 for v in s.wall_sweeps.values_mut(){profile(&mut v.profile,word);v.height=word;v.inset=word;}
 for v in s.property_templates.values_mut(){for v in &mut v.properties{v.minimum=v.minimum.map(|_|word);v.maximum=v.maximum.map(|_|word);if let Some(v)=&mut v.default_value{property_word(v,word)}for v in &mut v.allowed{property_word(v,word)}}}
 for sets in s.properties.values_mut(){for values in sets.values_mut(){for v in values.values_mut(){property_word(v,word)}}}
}
#[test]
fn sqlite_snapshot_bim_exact_words_use_original_public_io(){
 let provider=crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.expect("original BIM public capability");
 let laws:serde_json::Value=serde_json::from_str(include_str!("../../🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎛️control/🔣️.json")).unwrap();let words=laws["binary64Words"].as_array().expect("neutral ordered binary64 corpus");assert_eq!(words.len(),11);
 for word in words{let word=word.as_str().expect("neutral word is hexadecimal text");assert_eq!(word.len(),16);let bits=u64::from_str_radix(word,16).expect("neutral binary64 word");
  let mut expected=fixture();every_word(&mut expected,f64::from_bits(bits));let original=pack::snapshot::ArtifactPack::encode_pack(&expected);
  for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=payload(&expected,encoding);let database=original_owners::with_decoding(|native_owner|(provider.export)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT,&input,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;
   let populated=word_cells(&database,bits);let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let receipt:serde_json::Value=serde_json::from_slice(&oracle(&bytes,"words",Some(word))).expect("independent word query receipt");assert_eq!(receipt["schema"],"bim.sqlite.oracle/v1");assert_eq!(receipt["mode"],"words");assert_eq!(receipt["word"],word);assert_eq!(receipt["tableCount"],93);assert_eq!(receipt["totalRows"],3972);assert_eq!(receipt["populatedWords"],populated as u64);assert_eq!(receipt["checked"],true);assert_eq!(receipt["declaredRoles"],184);assert_eq!(receipt["populatedRoles"],184);let database=import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();assert_eq!(word_cells(&database,bits),populated);
   let output=original_owners::with_encoding(|native_owner|(provider.import)(BIM_MODEL_DOCUMENT_SCHEMA,&BIM_MODEL_DIALECT, &mut Some(database),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;
   assert_eq!(pack::snapshot::ArtifactPack::encode_pack(&decode(&output)),original,"{bits:016x} {encoding:?}");
  }
 }
 println!("[DEBUG] BIM original public IO exact words=11 encodings=2 all authored float roles and integer extrema");
}
