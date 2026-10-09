import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {Database} from "bun:sqlite";
const root=process.argv[2]!,ticket=process.argv[3]!;
type F=readonly[name:string,type:string|readonly string[],optional?:boolean];
const old=readFileSync(root+"/🗄️.sql","utf8"),statements=new Map<string,string>();
for(const line of old.trim().split("\n")){const match=/^CREATE TABLE (\w+)/.exec(line);if(match)statements.set(match[1]!,line)}
const id="id INTEGER PRIMARY KEY",ordinal="ordinal INTEGER NOT NULL CHECK(ordinal>=0)";
const enumValues=(v:readonly string[])=>v.map(x=>"'"+x+"'").join(",");
function columns(fields:readonly F[]):string[]{
 const result:string[]=[],checks:string[]=[];
 for(const[name,type,optional=false]of fields){
  if(type==="f64"){
   result.push(name+" REAL",name+"_bits INTEGER"+(optional?"":" NOT NULL"),name+"_class TEXT"+(optional?"":" NOT NULL"));
   const absent="("+name+" IS NULL AND "+name+"_bits IS NULL AND "+name+"_class IS NULL)";
   const present="("+name+"_bits IS NOT NULL AND "+name+"_class IS NOT NULL AND "+name+"_class IN('finite','nan','positiveInfinity','negativeInfinity') AND (("+name+"_class='finite' AND "+name+" IS NOT NULL) OR ("+name+"_class IN('nan','positiveInfinity','negativeInfinity') AND "+name+" IS NULL)))";
   checks.push("CHECK("+(optional?absent+" OR ":"")+present+")");
  }else{
   const sql=Array.isArray(type)?"TEXT":type==="u32"||type==="i32"||type==="bool"?"INTEGER":"TEXT";
   const predicate=Array.isArray(type)?name+" IN("+enumValues(type)+")":type==="u32"?name+">=0 AND "+name+"<=4294967295":type==="i32"?name+">=-2147483648 AND "+name+"<=2147483647":type==="bool"?name+" IN(0,1)":null;
   result.push(name+" "+sql+(optional?"":" NOT NULL")+(predicate?" CHECK("+predicate+")":""));
  }
 }
 return [...result,...checks];
}
const text=(name:string,optional=false):F=>[name,"text",optional],float=(name:string,optional=false):F=>[name,"f64",optional];
const point=(prefix:string,optional=false):F[]=>[float(prefix+"x",optional),float(prefix+"y",optional)];
const vertex:F[]=[...point(""),float("bulge")];
const PHASE=["Existing","New","Demolished","Temporary"],SIDE=["Left","Right"],KIND=["Text","Real","Integer","Boolean","Length","Area","Volume","Angle"];
const PROPERTY_KIND=KIND;
const TEMPLATE_TARGET=["Site","Building","Storey","Wall","CurtainWall","Column","Beam","Slab","Ceiling","Roof","Window","Door","Void","Stair","Ramp","Railing","Space","Zone","WallType","SlabType","CeilingType","RoofType","ColumnType","BeamType","WindowType","DoorType"];
const VIEW_CATEGORY=["Walls","CurtainWalls","Columns","Beams","Slabs","Roofs","Openings","Stairs","Railings","Spaces","Grids"];
const SCHEDULE_FIELD=["Id","Name","Kind","Storey","Level","Type","Phase","Material","Host","Number","Usage","Surface","Swing","Leaves","Panes","Count","Length","Width","Height","Perimeter","GrossSideArea","OpeningArea","NetSideArea","GrossArea","NetArea","SurfaceArea","GrossVolume","NetVolume","Mass","Risers","Thickness","LayerArea","LayerVolume","LayerMass","FinishArea"];
function table(name:string,header:string[],fields:readonly F[]=[],checks:string[]=[]):void{statements.set(name,"CREATE TABLE "+name+" ("+[id,...header,...columns(fields),...checks].join(", ")+");")}
const owner=(name:string,target:string,optional=false)=>name+" INTEGER"+(optional?"":" NOT NULL")+" REFERENCES "+target+"(id)";
function map(name:string,fields:readonly F[]=[],checks:string[]=[]):void{table(name,[owner("document_id","bim_document"),ordinal,"map_key TEXT NOT NULL"],fields,checks)}
function child(name:string,parent:string,target:string,fields:readonly F[]=[],checks:string[]=[]):void{table(name,[owner(parent,target),ordinal],fields,checks)}
const flatten=(fields:readonly F[]):string[]=>fields.flatMap(([name,type])=>type==="f64"?[name,name+"_bits",name+"_class"]:[name]);
const absent=(fields:readonly F[])=>flatten(fields).map(n=>n+" IS NULL").join(" AND ");
const present=(fields:readonly F[])=>fields.flatMap(([n,t])=>t==="f64"?[n+"_bits IS NOT NULL",n+"_class IS NOT NULL"]:[n+" IS NOT NULL"]).join(" AND ");
function compound(fields:readonly F[]):string{return"CHECK(("+absent(fields)+") OR ("+present(fields)+"))"}
function variant(prefix:string,rules:Record<string,readonly string[]>,fields:readonly F[]):{fields:F[],checks:string[]}{
 const branches=Object.entries(rules).map(([kind,active])=>"("+prefix+"kind='"+kind+"'"+(fields.length?" AND ":"")+fields.map(f=>active.includes(f[0])?present([f]):absent([f])).join(" AND ")+")");
 return{fields:[[prefix+"kind",Object.keys(rules)],...fields.map(([n,t])=>[n,t,true]as F)],checks:["CHECK("+branches.join(" OR ")+")"]};
}
const PANEL=variant("panel_",{Glass:[],Empty:[],Solid:["panel_material_key"],Door:["panel_door_type_key"],Window:["panel_window_type_key"]},[text("panel_material_key"),text("panel_door_type_key"),text("panel_window_type_key")]);
map("bim_curtain_wall_type",[text("name"),...PANEL.fields,text("panel_material_key_default"),text("mullion_material_key")],PANEL.checks);
map("bim_wall",[text("storey_key"),text("wall_type_key"),["location",["Center","Interior","Exterior","CoreCenter"]],float("base_offset"),["phase",PHASE],["start_join",["Miter","Butt","None"],true],["end_join",["Miter","Butt","None"],true],text("name"),text("base_slab_key",true)]);
map("bim_curtain_wall",[text("storey_key"),text("curtain_wall_type_key"),float("base_offset"),["phase",PHASE],text("name")]);
map("bim_curtain_panel_override",[text("curtain_key"),["u","u32"],["v","u32"],...PANEL.fields],PANEL.checks);
const tilt=[float("tilt_direction",true),float("tilt_angle",true)];
map("bim_column",[text("storey_key"),text("column_type_key"),...point("position_"),float("rotation"),...tilt,float("base_offset"),["phase",PHASE],text("name")],[compound(tilt)]);
map("bim_beam",[text("storey_key"),text("beam_type_key"),float("top_offset"),float("end_top_offset",true),["phase",PHASE],text("name")]);
const slope=[float("slope_direction",true),float("slope_angle",true)];
map("bim_slab",[text("storey_key"),text("slab_type_key"),float("offset"),...slope,["phase",PHASE],text("name")],[compound(slope)]);
const ROOF=variant("shape_",{Flat:[],Shed:["pitch","direction"],Gable:["pitch","ridge_direction"],Hip:["pitch"],Mansard:["lower_pitch","upper_pitch","break_height"]},["pitch","direction","ridge_direction","lower_pitch","upper_pitch","break_height"].map(n=>float(n)));
map("bim_roof",[text("storey_key"),text("roof_type_key"),...ROOF.fields,float("overhang"),float("base_offset"),["phase",PHASE],text("name")],ROOF.checks);
const OPENING=variant("opening_",{Window:["window_type_key"],Door:["door_type_key"],Void:["void_width","void_height"]},[text("window_type_key"),text("door_type_key"),float("void_width"),float("void_height")]);
map("bim_opening",[text("host_key"),...OPENING.fields,float("offset"),float("sill_override",true),float("width",true),float("height",true),["flip_hand","bool"],["flip_facing","bool"],text("name"),float("reveal_depth",true),text("reveal_material_key",true)],OPENING.checks);
const FLIGHT=variant("flight_",{Straight:[],LTurn:["split","turn"],UTurn:["gap"],Spiral:["radius","sweep"]},[float("split"),["turn",SIDE],float("gap"),float("radius"),float("sweep")]);
map("bim_stair",[text("storey_key"),...point("start_"),float("direction"),float("width"),...FLIGHT.fields,float("max_riser"),float("min_tread"),["stringer_kind",["None","Closed","Open","Mono"]],float("stringer_width"),float("stringer_depth"),float("nosing"),float("tread_thickness"),["riser",["Open","Closed"]],float("landing_depth"),["phase",PHASE],text("name")],FLIGHT.checks);
const INFILL=variant("infill_",{None:[],Glass:["infill_thickness"],Panel:["infill_thickness"]},[float("infill_thickness")]);
const host=[text("host_element_key",true),["host_side",SIDE,true]as F,["host_edge","u32",true]as F,float("host_inset",true)];
map("bim_railing",[text("storey_key"),float("height"),float("post_spacing"),...INFILL.fields,text("material_key"),float("base_offset"),...host,["phase",PHASE],text("name")],[...INFILL.checks,compound(host)]);
map("bim_ramp",[text("storey_key"),float("width"),float("landing_start"),float("landing_end"),float("landing_turn"),float("max_slope"),float("thickness"),text("material_key"),float("base_offset"),["railing_left","bool"],["railing_right","bool"],text("name")]);
map("bim_ceiling_type",[text("name")]);
map("bim_ceiling",[text("storey_key"),text("ceiling_type_key"),float("offset"),...slope,text("name")],[compound(slope)]);
const BOUNDARY=variant("boundary_",{Bounded:["seed_x","seed_y"],Explicit:[]},point("seed_"));
map("bim_space",[text("storey_key"),text("number"),text("name"),...BOUNDARY.fields,text("usage"),["phase",PHASE],text("zone_key",true),text("floor_finish_key",true),text("wall_finish_key",true),text("ceiling_finish_key",true)],BOUNDARY.checks);
map("bim_zone",[text("name"),text("category"),float("occupancy_density")]);
map("bim_area_scheme",[text("name"),["measure",["Gross","Net"]]]);
const plane=[...point("plane_start_",true),...point("plane_end_",true)];
const crop=[...point("crop_min_",true),...point("crop_max_",true)];
const camera=[...point("camera_target_",true),...["target_height","azimuth","pitch","distance"].map(n=>float("camera_"+n,true))];
map("bim_view",[text("building_key"),text("name"),["kind",["Plan","CeilingPlan","Section","Elevation","Orthographic","Perspective"]],text("storey_key",true),...plane,...camera,float("cut_height",true),float("depth"),...crop,["phase",PHASE,true],["scale","u32"],["detail",["Coarse","Medium","Fine"]]],[compound(plane),compound(camera),compound(crop)]);
const PAPER=variant("paper_",{Iso:["paper_size"],Custom:["paper_width","paper_height"]},[["paper_size",["A0","A1","A2","A3","A4"]],float("paper_width"),float("paper_height")]);
map("bim_sheet",[text("number"),text("name"),...PAPER.fields,["orientation",["Landscape","Portrait"]],text("project"),text("drawn_by"),text("checked_by"),text("date"),text("revision"),text("scale_label")],PAPER.checks);
map("bim_viewport",[text("sheet_key"),text("view_key"),...point("position_"),["scale","u32"],...crop,text("label",true)],[compound(crop)]);
map("bim_sheet_revision",[text("sheet_key"),text("number"),text("date"),text("description"),text("author")]);
map("bim_dimension",[text("storey_key"),float("angle"),float("offset"),text("style_key"),float("lock",true),text("name")]);
map("bim_tag",[text("storey_key"),text("element_key"),["category",["Name","Type","Number","Size"]],...point("offset_"),text("style_key")]);
map("bim_text_note",[text("storey_key"),...point("position_"),text("text"),float("rotation"),text("style_key")]);
map("bim_leader",[text("storey_key"),...point("offset_"),text("text"),text("style_key")]);
map("bim_annotation_style",[text("name"),float("text_height"),["terminator",["Tick","Arrow","Dot"]],["unit",["Metre","Centimetre","Millimetre"]],["precision","u32"],float("mark_size"),float("gap"),float("overshoot")]);
map("bim_wall_sweep",[text("host_key"),["side",SIDE],float("height"),float("inset"),text("material_key"),text("name")]);
map("bim_schedule",[text("name"),["category",["Wall","CurtainWall","Slab","Roof","Column","Beam","Window","Door","Void","Stair","Railing","Space","Finish","Material"]],["itemize","bool"]]);
map("bim_family",[text("name"),["category",["Furniture","Equipment","Casework","Plumbing","Lighting","Mechanical","Electrical","Generic","Profile"]]]);
map("bim_family_parameter",[text("family_key"),text("name"),["kind",["Length","Angle","Real","Integer","Boolean","Text","Material"]],text("value_expression")]);
const SOLID=variant("shape_",{Extrusion:["base_expression","height_expression"],Revolution:["axis","angle_expression"],Sweep:[],Cuboid:["x_expression","y_expression","z_expression","width_expression","depth_expression","height_expression"]},[text("base_expression"),text("height_expression"),["axis",["X","Y","Z"]],text("angle_expression"),text("x_expression"),text("y_expression"),text("z_expression"),text("width_expression"),text("depth_expression")]);
map("bim_family_solid",[text("family_key"),text("name"),...SOLID.fields,text("material_key"),text("visible_expression"),text("offset_x_expression"),text("offset_y_expression"),text("offset_z_expression")],SOLID.checks);
map("bim_property_template",[text("name")]);
map("bim_classification_system",[text("name"),text("edition"),text("source",true)]);
map("bim_classification_element");
statements.delete("bim_classification");
const refs=(pairs:readonly(readonly[string,string])[])=>pairs.map(([n,t])=>owner(n,t,true));
const single=(pairs:readonly(readonly[string,string])[])=>"CHECK("+pairs.map(([n])=>"("+n+" IS NOT NULL)").join("+")+"=1)";
const AXIS=variant("",{Line:["start_x","start_y","end_x","end_y"],Arc:["start_x","start_y","end_x","end_y","bulge"]},[...point("start_"),...point("end_"),float("bulge")]);
const axes=[["wall_id","bim_wall"],["curtain_wall_id","bim_curtain_wall"],["beam_id","bim_beam"]]as const;
table("bim_axis",refs(axes),AXIS.fields,[single(axes),...AXIS.checks]);
const TOP=variant("",{Unconnected:["height"],StoreyTop:["offset"],Storey:["offset","storey_key"],Roof:["offset","roof_key"],Slab:["offset","slab_key"],Ceiling:["offset","ceiling_key"]},[float("height"),float("offset"),text("storey_key"),text("roof_key"),text("slab_key"),text("ceiling_key")]);
const tops=[["wall_id","bim_wall"],["curtain_wall_id","bim_curtain_wall"],["column_id","bim_column"],["stair_id","bim_stair"],["ramp_id","bim_ramp"]]as const;
table("bim_top",refs(tops),TOP.fields,[single(tops),...TOP.checks]);
const PROFILE=variant("",{Rectangle:["width","depth"],Circle:["diameter"],IShape:["width","depth","web","flange"],Custom:[],Family:["family_key"]},[float("width"),float("depth"),float("diameter"),float("web"),float("flange"),text("family_key")]);
const profiles=[["column_type_id","bim_column_type"],["beam_type_id","bim_beam_type"],["curtain_type_interior_id","bim_curtain_wall_type"],["curtain_type_border_id","bim_curtain_wall_type"],["railing_id","bim_railing"],["railing_post_id","bim_railing"],["baluster_id","bim_baluster"],["wall_sweep_id","bim_wall_sweep"]]as const;
table("bim_profile",refs(profiles),PROFILE.fields,[single(profiles),...PROFILE.checks]);
const CURTAIN_GRID=variant("",{Spacing:["spacing"],Lines:[]},[float("spacing")]);
const grids=[["curtain_type_u_id","bim_curtain_wall_type"],["curtain_type_v_id","bim_curtain_wall_type"],["curtain_wall_u_id","bim_curtain_wall"],["curtain_wall_v_id","bim_curtain_wall"]]as const;
table("bim_curtain_grid",refs(grids),CURTAIN_GRID.fields,[single(grids),...CURTAIN_GRID.checks]);
child("bim_curtain_grid_position","grid_id","bim_curtain_grid",[float("position")]);
child("bim_ceiling_layer","ceiling_type_id","bim_ceiling_type",[text("material_key"),float("thickness"),["function",["Structure","Substrate","Insulation","Finish","Membrane","Core"]]]);
child("bim_ceiling_boundary","ceiling_id","bim_ceiling",vertex);
child("bim_ceiling_hole","ceiling_id","bim_ceiling");
child("bim_ceiling_hole_vertex","hole_id","bim_ceiling_hole",vertex);
child("bim_ramp_path","ramp_id","bim_ramp",vertex);
child("bim_area_usage","scheme_id","bim_area_scheme",[text("usage")]);
child("bim_area_zone","scheme_id","bim_area_scheme",[text("zone_key")]);
child("bim_view_hidden","view_id","bim_view",[["category",VIEW_CATEGORY]]);
const ANCHOR=variant("",{Point:["point_x","point_y"],WallFace:["wall_key","side"],WallAxis:["wall_key"],WallEnd:["wall_key","end"],OpeningCentre:["opening_key"],Grid:["grid_key"],ColumnCentre:["column_key"]},[...point("point_"),text("wall_key"),["side",SIDE],["end",["Start","End"]],text("opening_key"),text("grid_key"),text("column_key")]);
const anchors=[["dimension_id","bim_dimension"],["leader_id","bim_leader"]]as const;
table("bim_annotation_anchor",[...refs(anchors),ordinal],ANCHOR.fields,[single(anchors),...ANCHOR.checks]);
const PARAMETRIC=variant("",{Rectangle:["width_expression","depth_expression"],Circle:["diameter_expression"],IShape:["width_expression","depth_expression","web_expression","flange_expression"],Polygon:[]},["width","depth","diameter","web","flange"].map(n=>text(n+"_expression")));
table("bim_parametric_profile",[owner("solid_id","bim_family_solid")],PARAMETRIC.fields,PARAMETRIC.checks);
child("bim_parametric_polygon_point","profile_id","bim_parametric_profile",[text("x_expression"),text("y_expression")]);
child("bim_solid_sweep_point","solid_id","bim_family_solid",[text("x_expression"),text("y_expression")]);
const KEY=variant("key_",{Field:["field"],Property:["set_name","property_name"]},[["field",SCHEDULE_FIELD],text("set_name"),text("property_name")]);
child("bim_schedule_column","schedule_id","bim_schedule",[...KEY.fields,text("heading",true),["total","bool"]],KEY.checks);
child("bim_schedule_sort","schedule_id","bim_schedule",[...KEY.fields,["descending","bool"]],KEY.checks);
child("bim_schedule_filter","schedule_id","bim_schedule",[...KEY.fields,["op",["Equals","NotEquals","Contains","Greater","GreaterOrEqual","Less","LessOrEqual","Empty","NotEmpty"]],text("value")],KEY.checks);
child("bim_schedule_group","schedule_id","bim_schedule",KEY.fields,KEY.checks);
child("bim_schedule_storey","schedule_id","bim_schedule",[text("storey_key")]);
child("bim_schedule_phase","schedule_id","bim_schedule",[["phase",PHASE]]);
child("bim_template_target","template_id","bim_property_template",[["target",TEMPLATE_TARGET]]);
child("bim_property_def","template_id","bim_property_template",[text("name"),["kind",PROPERTY_KIND],text("unit",true),text("description",true),["required","bool"],float("minimum",true),float("maximum",true)]);
const VALUE=variant("",{Text:["text_value"],Integer:["integer_value"],Boolean:["boolean_value"],Real:["value"],Length:["value"],Area:["value"],Volume:["value"],Angle:["value"]},[text("text_value"),["integer_value","i32"],["boolean_value","bool"],float("value")]);
table("bim_property_default",[owner("def_id","bim_property_def")],VALUE.fields,VALUE.checks);
child("bim_property_allowed","def_id","bim_property_def",VALUE.fields,VALUE.checks);
child("bim_classification_item","system_id","bim_classification_system",[text("code"),text("title"),text("parent_code",true)]);
child("bim_classification_assignment","element_id","bim_classification_element",[text("system_key"),text("code")]);
const sql=[...statements.values()].join("\n")+"\n";
const db=new Database(":memory:");db.run(sql);
const tables=db.query<{name:string},[]>("SELECT name FROM sqlite_schema WHERE type='table'").all().map(({name})=>({name,columns:db.query("PRAGMA table_info("+name+")").all(),foreignKeys:db.query("PRAGMA foreign_key_list("+name+")").all()}));
const maxColumns=Math.max(...tables.map(t=>t.columns.length));
if(tables.some(t=>t.columns.some((c:any)=>c.type==="BLOB")))throw Error("No opaque BLOB columns admitted");
writeFileSync(root+"/🗄️.sql",sql);
writeFileSync(root+"/🗄️schema/🟦️.ts","/** 🏢️ Individually authored BIM entities, parameter variants and owned ordered relations. */\nexport const BIM_MODEL_SQLITE_SCHEMA = "+JSON.stringify(sql)+";\n");
mkdirSync(ticket+"/🗑️generated",{recursive:true});
writeFileSync(ticket+"/🗑️generated/bim-ddl.json",JSON.stringify({tables:tables.length,maxColumns,owners:tables},null,2)+"\n");
writeFileSync(ticket+"/📓️bim-ddl.md","# Current BIM Relational Schema\n\nThe literal schema owns all current persisted BIM entity families. Each parameter branch has named columns with closed presence constraints. Owned ordered children and empty collection parents have explicit relational owners. References to authored domain keys remain literal TEXT, including unresolved external references. Floating values have query REAL, signed binary64 word and classification columns. No BLOB or hidden JSON content is present.\n\nThe independent Bun SQLite engine created "+tables.length+" tables; the largest table has "+maxColumns+" columns. This schema admission is not a provider round-trip qualification. Source and Native implementations must still prove the complete current witness over original public IO.\n");
console.log("[DEBUG] current handcrafted BIM SQLite DDL independent admission",JSON.stringify({tables:tables.length,maxColumns}));db.close();

