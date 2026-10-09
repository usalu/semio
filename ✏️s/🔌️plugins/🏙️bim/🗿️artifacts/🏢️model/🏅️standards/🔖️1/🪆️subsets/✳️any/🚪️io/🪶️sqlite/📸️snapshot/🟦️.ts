/** 🏢️ Authored BIM parameters projected into individually named semantic entities. */
import type * as domain from "../../../🧬️schema/🟦️.ts";
import type {ModelSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow,SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {BIM_MODEL_SQLITE_SCHEMA} from "./🗄️schema/🟦️.ts";
type Cells=readonly SqliteValue[];
type ObjectValue=Record<string,unknown>;
interface Codec<T=unknown>{readonly width:number;to(value:T):Cells;from(cursor:Cursor):T}
const fail=(message:string):never=>{throw Error("BIM SQLite "+message)};
const object=(value:unknown):ObjectValue=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as ObjectValue:fail("requires object");
const sequence=<T>(value:readonly T[]|undefined):readonly T[]=>value===undefined?[]:Array.isArray(value)?value:fail("requires ordered collection");
type SemanticPhase="projectSnapshot"|"reconstructSnapshot";
type Work={units:number};
const mapObject=<T>(value:Record<string,T>|undefined):Record<string,T>=>value===undefined?{}:object(value)as Record<string,T>;
async function advance(work:Work,options:ArtifactSqliteOptions,phase:SemanticPhase):Promise<void>{if(!Number.isSafeInteger(++work.units))fail("map work overflow");if(work.units%256===0||options.signal?.aborted)await artifactSqliteCheckpoint(options,phase,work.units,0)}
async function compareKeys(a:string,b:string,options:ArtifactSqliteOptions,phase:SemanticPhase,work:Work):Promise<number>{let i=0,j=0;while(i<a.length&&j<b.length){await advance(work,options,phase);const x=a.codePointAt(i)!,y=b.codePointAt(j)!;if(x!==y)return x<y?-1:1;i+=x>65535?2:1;j+=y>65535?2:1}return i<a.length?1:j<b.length?-1:0}
async function entries<T>(value:Record<string,T>|undefined,options:ArtifactSqliteOptions):Promise<[string,T][]>{
 const source=mapObject(value),work:Work={units:0};let count=0;
 for(const key in source)if(Object.hasOwn(source,key)){if(++count>(options.maxRows??1_000_000))fail("map row limit");await advance(work,options,"projectSnapshot")}
 await artifactSqliteCheckpoint(options,"projectSnapshot",work.units,0);
 const result:[string,T][]=new Array(count);let at=0;for(const key in source)if(Object.hasOwn(source,key)){await advance(work,options,"projectSnapshot");result[at++]=[literalText(key),source[key]!]}if(at!==count)fail("map changed during admission");
 const sift=async(root:number,end:number)=>{while(root<Math.floor(end/2)){await advance(work,options,"projectSnapshot");let child=root*2+1;if(child+1<end&&await compareKeys(result[child]![0],result[child+1]![0],options,"projectSnapshot",work)<0)child++;if(await compareKeys(result[root]![0],result[child]![0],options,"projectSnapshot",work)>=0)break;const row=result[root]!;result[root]=result[child]!;result[child]=row;root=child}};
 for(let root=Math.floor(count/2)-1;root>=0;root--)await sift(root,count);for(let end=count-1;end>0;end--){const row=result[0]!;result[0]=result[end]!;result[end]=row;await sift(0,end)}
 await artifactSqliteCheckpoint(options,"projectSnapshot",work.units,work.units);return result;
}
const literalText=(value:unknown):string=>typeof value==="string"&&value.isWellFormed()?value:fail("requires Unicode TEXT");
const nulls=(width:number):Cells=>Array.from({length:width},()=>null);
const text:Codec<string>={width:1,to:v=>[literalText(v)],from:c=>c.text()};
const integer=(min:number,max:number):Codec<number>=>({width:1,to:v=>typeof v==="number"&&Number.isInteger(v)&&v>=min&&v<=max?[BigInt(v)]:fail("integer width"),from:c=>{const v=c.integer();return v>=BigInt(min)&&v<=BigInt(max)?Number(v):fail("integer width")}});
const uint=integer(0,4294967295),int=integer(-2147483648,2147483647);
const boolean:Codec<boolean>={width:1,to:v=>typeof v==="boolean"?[v?1n:0n]:fail("requires Boolean"),from:c=>{const v=c.integer();return v===0n?false:v===1n?true:fail("Boolean width")}};
const symbol=<T extends string>(members:readonly T[]):Codec<T>=>({width:1,to:v=>members.includes(v)?[v]:fail("undeclared symbol"),from:c=>{const v=c.text();return members.includes(v as T)?v as T:fail("undeclared symbol")}});
const classification=(v:number)=>Number.isNaN(v)?"nan":v===Infinity?"positiveInfinity":v===-Infinity?"negativeInfinity":"finite";
const number:Codec<number>={width:3,to:v=>{if(typeof v!=="number")return fail("requires binary64");const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,v,true);return[Number.isFinite(v)?v:null,bytes.getBigInt64(0,true),classification(v)]},from:c=>{const query=c.next(),word=c.integer(),kind=c.text(),bytes=new DataView(new ArrayBuffer(8));bytes.setBigInt64(0,word,true);const v=bytes.getFloat64(0,true);if(kind!==classification(v))return fail("binary64 class disagrees");if(!Number.isFinite(v)){if(query!==null)return fail("nonfinite query must be NULL")}else if(typeof query==="bigint"){if(!Number.isInteger(v)||BigInt(v)!==query)return fail("binary64 integer query disagrees")}else if(typeof query!=="number"||query!==v)return fail("binary64 REAL query disagrees");return v}};
const optional=<T>(codec:Codec<T>):Codec<T|undefined>=>({width:codec.width,to:v=>v===undefined?nulls(codec.width):codec.to(v),from:c=>{if(c.absent(codec.width))return undefined;return codec.from(c)}});
const optionalNumber=optional(number);
type Field<T> = readonly [keyof T,Codec<any>];
function entity<T extends object>(fields:readonly Field<T>[],nested:readonly string[]=[]):Codec<T>{return{width:fields.reduce((n,[,c])=>n+c.width,0),to:value=>{const v=object(value),allowed=[...fields.map(([key])=>String(key)),...nested];for(const key of Object.keys(v))if(!allowed.includes(key))fail("undeclared entity field "+key);return fields.flatMap(([key,c])=>c.to(v[String(key)]))},from:cursor=>{const result:ObjectValue={};for(const[key,c]of fields){const v=c.from(cursor);if(v!==undefined)Object.defineProperty(result,key,{value:v,enumerable:true,writable:true,configurable:true})}return result as T}}}
const point=entity<domain.Point2>([["x",number],["y",number]]);
const color=entity<domain.Rgb>([["r",number],["g",number],["b",number]]);
const vertex=entity<domain.Vertex>([["point",point],["bulge",number]]);
const slope=entity<domain.Slope>([["direction",number],["angle",number]]);
const stringer=entity<domain.StairStringer>([["kind",symbol(["None","Closed","Open","Mono"])],["width",number],["depth",number]]);
const layer=entity<domain.Layer>([["material",text],["thickness",number],["function",symbol(["Structure","Substrate","Insulation","Finish","Membrane","Core"])]]);
type Rule=readonly string[];
function variant<T>(rules:Readonly<Record<string,Rule>>,fields:readonly(readonly[string,Codec<any>])[],units:readonly string[]=[],children:Readonly<Record<string,readonly string[]>>={}):Codec<T>{return{width:1+fields.reduce((n,[,c])=>n+c.width,0),to:value=>{const kind=typeof value==="string"?value:Object.keys(object(value))[0];if(kind===undefined||!Object.hasOwn(rules,kind))return fail("undeclared parameter variant");let payload:ObjectValue={};if(units.includes(kind)){if(value!==kind)return fail("unit variant requires literal symbol")}else{const wrapper=object(value);if(Object.keys(wrapper).length!==1)return fail("variant has multiple cases");payload=object(wrapper[kind]);for(const key of Object.keys(payload))if(!rules[kind]!.includes(key)&&!children[kind]?.includes(key))fail("undeclared variant parameter "+key)}return[kind,...fields.flatMap(([key,c])=>rules[kind]!.includes(key)?c.to(payload[key]):nulls(c.width))]},from:cursor=>{const kind=cursor.text();if(!Object.hasOwn(rules,kind))return fail("undeclared parameter variant");const payload:ObjectValue={};for(const[key,c]of fields)if(rules[kind]!.includes(key))payload[key]=c.from(cursor);else cursor.empty(c.width);return(units.includes(kind)?kind:{[kind]:payload})as T}}}
const axis=variant<domain.Axis>({Line:["start","end"],Arc:["start","end","bulge"]},[["start",point],["end",point],["bulge",number]]);
const top=variant<domain.TopConstraint>({Unconnected:["height"],StoreyTop:["offset"],Storey:["offset","storey"],Roof:["offset","roof"],Slab:["offset","slab"],Ceiling:["offset","ceiling"]},[["height",number],["offset",number],["storey",text],["roof",text],["slab",text],["ceiling",text]]);
const profile=variant<domain.Profile>({Rectangle:["width","depth"],Circle:["diameter"],IShape:["width","depth","web","flange"],Custom:[],Family:["family"]},[["width",number],["depth",number],["diameter",number],["web",number],["flange",number],["family",text]],[],{Custom:["outline"]});
const roofShape=variant<domain.RoofShape>({Flat:[],Shed:["pitch","direction"],Gable:["pitch","ridge_direction"],Hip:["pitch"],Mansard:["lower_pitch","upper_pitch","break_height"]},[["pitch",number],["direction",number],["ridge_direction",number],["lower_pitch",number],["upper_pitch",number],["break_height",number]],["Flat"]);
const openingKind=variant<domain.OpeningKind>({Window:["window_type"],Door:["door_type"],Void:["width","height"]},[["window_type",text],["door_type",text],["width",number],["height",number]]);
const flight=variant<domain.StairFlight>({Straight:[],LTurn:["split","turn"],UTurn:["gap"],Spiral:["radius","sweep"]},[["split",number],["turn",symbol(["Left","Right"])],["gap",number],["radius",number],["sweep",number]],["Straight"]);
const infill=variant<domain.Infill>({None:[],Glass:["thickness"],Panel:["thickness"]},[["thickness",number]],["None"]);
const boundary=variant<domain.SpaceBoundary>({Bounded:["seed"],Explicit:[]},[["seed",point]],[],{Explicit:["outline"]});
const material=entity<domain.Material>([["name",text],["category",symbol(["Concrete","Masonry","Wood","Metal","Glass","Insulation","Finish","Membrane","Other"])],["color",color],["density",number],["conductivity",number],["specific_heat",number]]);
const layeredType=entity<domain.WallType>([["name",text]],["layers"]);
const memberType=entity<domain.ColumnType>([["name",text],["material",text]],["profile"]);
const windowType=entity<domain.WindowType>([["name",text],["width",number],["height",number],["sill",number],["frame_width",number],["frame_depth",number],["panes",uint],["material",text]]);
const doorType=entity<domain.DoorType>([["name",text],["width",number],["height",number],["frame_width",number],["frame_depth",number],["leaves",symbol(["Single","Double"])],["swing",symbol(["Left","Right"])],["material",text]]);
const site=entity<domain.Site>([["name",text],["latitude",number],["longitude",number],["elevation",number],["true_north",number]],["boundary"]);
const building=entity<domain.Building>([["site",text],["name",text],["origin",point],["rotation",number],["elevation",number]]);
const storey=entity<domain.Storey>([["building",text],["name",text],["level",int],["height",number],["cut_height",optionalNumber]]);
const grid=entity<domain.GridLine>([["building",text],["label",text],["start",point],["end",point]]);
const phase=symbol<domain.Phase>(["Existing","New","Demolished","Temporary"]);
const side=symbol<domain.WallSide>(["Left","Right"]);
const endJoin=symbol<domain.EndJoin>(["Miter","Butt","None"]);
const curtainGrid=variant<domain.CurtainGrid>({Spacing:["spacing"],Lines:[]},[["spacing",number]],[],{Lines:["positions"]});
const curtainPanel=variant<domain.CurtainPanel>({Glass:[],Empty:[],Solid:["material"],Door:["door_type"],Window:["window_type"]},[["material",text],["door_type",text],["window_type",text]],["Glass","Empty"]);
const curtainType=entity<domain.CurtainWallType>([["name",text],["panel",curtainPanel],["panel_material",text],["mullion_material",text]],["u_grid","v_grid","interior_mullion","border_mullion"]);
const wall=entity<domain.Wall>([["storey",text],["wall_type",text],["location",symbol(["Center","Interior","Exterior","CoreCenter"])],["base_offset",number],["phase",phase],["start_join",optional(endJoin)],["end_join",optional(endJoin)],["name",text],["base_slab",optional(text)]],["axis","top"]);
const curtainWall=entity<domain.CurtainWall>([["storey",text],["curtain_wall_type",text],["base_offset",number],["phase",phase],["name",text]],["axis","top","u_grid","v_grid"]);
const curtainOverride=entity<domain.CurtainPanelOverride>([["curtain",text],["u",uint],["v",uint],["panel",curtainPanel]]);
const column=entity<domain.Column>([["storey",text],["column_type",text],["position",point],["rotation",number],["tilt",optional(slope)],["base_offset",number],["phase",phase],["name",text]],["top"]);
const beam=entity<domain.Beam>([["storey",text],["beam_type",text],["top_offset",number],["end_top_offset",optionalNumber],["phase",phase],["name",text]],["axis"]);
const slab=entity<domain.Slab>([["storey",text],["slab_type",text],["offset",number],["slope",optional(slope)],["phase",phase],["name",text]],["boundary","holes"]);
const roof=entity<domain.Roof>([["storey",text],["roof_type",text],["shape",roofShape],["overhang",number],["base_offset",number],["phase",phase],["name",text]],["footprint"]);
const opening=entity<domain.Opening>([["host",text],["kind",openingKind],["offset",number],["sill_override",optionalNumber],["width",optionalNumber],["height",optionalNumber],["flip_hand",boolean],["flip_facing",boolean],["name",text],["reveal_depth",optionalNumber],["reveal_material",optional(text)]]);
const stair=entity<domain.Stair>([["storey",text],["start",point],["direction",number],["width",number],["flight",flight],["max_riser",number],["min_tread",number],["stringer",stringer],["nosing",number],["tread_thickness",number],["riser",symbol(["Open","Closed"])],["landing_depth",number],["phase",phase],["name",text]],["top"]);
const railingHost=entity<domain.RailingHost>([["element",text],["side",side],["edge",uint],["inset",number]]);
const railing=entity<domain.Railing>([["storey",text],["height",number],["post_spacing",number],["infill",infill],["material",text],["base_offset",number],["host",optional(railingHost)],["phase",phase],["name",text]],["path","profile","post_profile","baluster"]);
const ramp=entity<domain.Ramp>([["storey",text],["width",number],["landing_start",number],["landing_end",number],["landing_turn",number],["max_slope",number],["thickness",number],["material",text],["base_offset",number],["railing_left",boolean],["railing_right",boolean],["name",text]],["path","top"]);
const ceiling=entity<domain.Ceiling>([["storey",text],["ceiling_type",text],["offset",number],["slope",optional(slope)],["name",text]],["boundary","holes"]);
const space=entity<domain.Space>([["storey",text],["number",text],["name",text],["boundary",boundary],["usage",text],["phase",phase],["zone",optional(text)],["floor_finish",optional(text)],["wall_finish",optional(text)],["ceiling_finish",optional(text)]]);
const zone=entity<domain.Zone>([["name",text],["category",text],["occupancy_density",number]]);
const areaScheme=entity<domain.AreaScheme>([["name",text],["measure",symbol(["Gross","Net"])]],["usages","zones"]);
const viewPlane=entity<domain.ViewPlane>([["start",point],["end",point]]);
const viewCrop=entity<domain.ViewCrop>([["min",point],["max",point]]);
const viewCamera=entity<domain.ViewCamera>([["target",point],["target_height",number],["azimuth",number],["pitch",number],["distance",number]]);
const viewCategory=symbol<domain.ViewCategory>(["Walls","CurtainWalls","Columns","Beams","Slabs","Roofs","Openings","Stairs","Railings","Spaces","Grids"]);
const view=entity<domain.View>([["building",text],["name",text],["kind",symbol(["Plan","CeilingPlan","Section","Elevation","Orthographic","Perspective"])],["storey",optional(text)],["plane",optional(viewPlane)],["camera",optional(viewCamera)],["cut_height",optionalNumber],["depth",number],["crop",optional(viewCrop)],["phase",optional(phase)],["scale",uint],["detail",symbol(["Coarse","Medium","Fine"])]],["hidden"]);
const paper=variant<domain.Paper>({Iso:["size"],Custom:["width","height"]},[["size",symbol(["A0","A1","A2","A3","A4"])],["width",number],["height",number]]);
const sheet=entity<domain.Sheet>([["number",text],["name",text],["paper",paper],["orientation",symbol(["Landscape","Portrait"])],["project",text],["drawn_by",text],["checked_by",text],["date",text],["revision",text],["scale_label",text]]);
const viewport=entity<domain.Viewport>([["sheet",text],["view",text],["position",point],["scale",uint],["crop",optional(viewCrop)],["label",optional(text)]]);
const sheetRevision=entity<domain.SheetRevision>([["sheet",text],["number",text],["date",text],["description",text],["author",text]]);
const anchor=variant<domain.AnnotationAnchor>({Point:["point"],WallFace:["wall","side"],WallAxis:["wall"],WallEnd:["wall","end"],OpeningCentre:["opening"],Grid:["grid"],ColumnCentre:["column"]},[["point",point],["wall",text],["side",side],["end",symbol(["Start","End"])],["opening",text],["grid",text],["column",text]]);
const dimension=entity<domain.Dimension>([["storey",text],["angle",number],["offset",number],["style",text],["lock",optionalNumber],["name",text]],["anchors"]);
const tag=entity<domain.Tag>([["storey",text],["element",text],["category",symbol(["Name","Type","Number","Size"])],["offset",point],["style",text]]);
const textNote=entity<domain.TextNote>([["storey",text],["position",point],["text",text],["rotation",number],["style",text]]);
const leader=entity<domain.Leader>([["storey",text],["offset",point],["text",text],["style",text]],["anchor"]);
const annotationStyle=entity<domain.AnnotationStyle>([["name",text],["text_height",number],["terminator",symbol(["Tick","Arrow","Dot"])],["unit",symbol(["Metre","Centimetre","Millimetre"])],["precision",uint],["mark_size",number],["gap",number],["overshoot",number]]);
const wallSweep=entity<domain.WallSweep>([["host",text],["side",side],["height",number],["inset",number],["material",text],["name",text]],["profile"]);
const scheduleField=symbol<domain.ScheduleField>(["Id","Name","Kind","Storey","Level","Type","Phase","Material","Host","Number","Usage","Surface","Swing","Leaves","Panes","Count","Length","Width","Height","Perimeter","GrossSideArea","OpeningArea","NetSideArea","GrossArea","NetArea","SurfaceArea","GrossVolume","NetVolume","Mass","Risers","Thickness","LayerArea","LayerVolume","LayerMass","FinishArea"]);
const scheduleKey=variant<domain.ScheduleKey>({Field:["field"],Property:["set","name"]},[["field",scheduleField],["set",text],["name",text]]);
const scheduleColumn=entity<domain.ScheduleColumn>([["key",scheduleKey],["heading",optional(text)],["total",boolean]]);
const scheduleSort=entity<domain.ScheduleSort>([["key",scheduleKey],["descending",boolean]]);
const scheduleFilter=entity<domain.ScheduleFilter>([["key",scheduleKey],["op",symbol(["Equals","NotEquals","Contains","Greater","GreaterOrEqual","Less","LessOrEqual","Empty","NotEmpty"])],["value",text]]);
const scheduleGroup=entity<domain.ScheduleGroup>([["key",scheduleKey]]);
const schedule=entity<domain.Schedule>([["name",text],["category",symbol(["Wall","CurtainWall","Slab","Roof","Column","Beam","Window","Door","Void","Stair","Railing","Space","Finish","Material"])],["itemize",boolean]],["columns","sort","filter","group","storeys","phases"]);
const family=entity<domain.Family>([["name",text],["category",symbol(["Furniture","Equipment","Casework","Plumbing","Lighting","Mechanical","Electrical","Generic","Profile"])]]);
const familyParameter=entity<domain.FamilyParameter>([["family",text],["name",text],["kind",symbol(["Length","Angle","Real","Integer","Boolean","Text","Material"])],["value",text]]);
const exprPoint=entity<domain.ExprPoint>([["x",text],["y",text]]);
const exprPoint3=entity<domain.ExprPoint3>([["x",text],["y",text],["z",text]]);
const parametricProfile=variant<domain.ParametricProfile>({Rectangle:["width","depth"],Circle:["diameter"],IShape:["width","depth","web","flange"],Polygon:[]},[["width",text],["depth",text],["diameter",text],["web",text],["flange",text]],[],{Polygon:["points"]});
const solidShape=variant<domain.SolidShape>({Extrusion:["base","height"],Revolution:["axis","angle"],Sweep:[],Cuboid:["x","y","z","width","depth","height"]},[["base",text],["height",text],["axis",symbol(["X","Y","Z"])],["angle",text],["x",text],["y",text],["z",text],["width",text],["depth",text]],[],{Extrusion:["profile"],Revolution:["profile"],Sweep:["profile","path"]});
const familySolid=entity<domain.FamilySolid>([["family",text],["name",text],["shape",solidShape],["material",text],["visible",text],["offset",exprPoint3]]);
const propertyKind=symbol<domain.PropertyKind>(["Text","Real","Integer","Boolean","Length","Area","Volume","Angle"]);
const propertyDef=entity<domain.PropertyDef>([["name",text],["kind",propertyKind],["unit",optional(text)],["description",optional(text)],["required",boolean],["minimum",optionalNumber],["maximum",optionalNumber]],["default_value","allowed"]);
const templateTarget=symbol<domain.TemplateTarget>(["Site","Building","Storey","Wall","CurtainWall","Column","Beam","Slab","Ceiling","Roof","Window","Door","Void","Stair","Ramp","Railing","Space","Zone","WallType","SlabType","CeilingType","RoofType","ColumnType","BeamType","WindowType","DoorType"]);
const propertyTemplate=entity<domain.PropertyTemplate>([["name",text]],["applies_to","properties"]);
const classificationSystem=entity<domain.ClassificationSystem>([["name",text],["edition",text],["source",optional(text)]],["entries"]);
const classificationItem=entity<domain.ClassificationItem>([["code",text],["title",text],["parent",optional(text)]]);
const project=entity<domain.Project>([["name",text],["description",text],["author",text],["organization",text]],["phase_names"]);
const collections=[
 ["materials","bim_material",material],["wall_types","bim_wall_type",layeredType],["slab_types","bim_slab_type",layeredType],["roof_types","bim_roof_type",layeredType],["column_types","bim_column_type",memberType],["beam_types","bim_beam_type",memberType],["window_types","bim_window_type",windowType],["door_types","bim_door_type",doorType],
 ["curtain_wall_types","bim_curtain_wall_type",curtainType],["sites","bim_site",site],["buildings","bim_building",building],["storeys","bim_storey",storey],["grids","bim_grid",grid],["walls","bim_wall",wall],["curtain_walls","bim_curtain_wall",curtainWall],["curtain_panel_overrides","bim_curtain_panel_override",curtainOverride],["columns","bim_column",column],["beams","bim_beam",beam],["slabs","bim_slab",slab],["roofs","bim_roof",roof],["openings","bim_opening",opening],["stairs","bim_stair",stair],["railings","bim_railing",railing],["ramps","bim_ramp",ramp],
 ["ceiling_types","bim_ceiling_type",layeredType],["ceilings","bim_ceiling",ceiling],["spaces","bim_space",space],["zones","bim_zone",zone],["area_schemes","bim_area_scheme",areaScheme],["views","bim_view",view],["sheets","bim_sheet",sheet],["viewports","bim_viewport",viewport],["sheet_revisions","bim_sheet_revision",sheetRevision],["dimensions","bim_dimension",dimension],["tags","bim_tag",tag],["text_notes","bim_text_note",textNote],["leaders","bim_leader",leader],["annotation_styles","bim_annotation_style",annotationStyle],["wall_sweeps","bim_wall_sweep",wallSweep],["schedules","bim_schedule",schedule],["families","bim_family",family],["family_parameters","bim_family_parameter",familyParameter],["family_solids","bim_family_solid",familySolid],["property_templates","bim_property_template",propertyTemplate],["classification_systems","bim_classification_system",classificationSystem],["properties","bim_property_element",null],["classifications","bim_classification_element",null]
]as const;
type CollectionName=typeof collections[number][0];
type OrderedRole=readonly[property:string,table:string,codec:Codec<any>];
const orderedRoles:Partial<Record<CollectionName,readonly OrderedRole[]>>={
 wall_types:[["layers","bim_wall_layer",layer]],slab_types:[["layers","bim_slab_layer",layer]],roof_types:[["layers","bim_roof_layer",layer]],ceiling_types:[["layers","bim_ceiling_layer",layer]],
 sites:[["boundary","bim_site_boundary",point]],slabs:[["boundary","bim_slab_boundary",vertex]],ceilings:[["boundary","bim_ceiling_boundary",vertex]],roofs:[["footprint","bim_roof_footprint",vertex]],railings:[["path","bim_railing_path",point]],ramps:[["path","bim_ramp_path",vertex]],
 area_schemes:[["usages","bim_area_usage",text],["zones","bim_area_zone",text]],views:[["hidden","bim_view_hidden",viewCategory]],
 schedules:[["columns","bim_schedule_column",scheduleColumn],["sort","bim_schedule_sort",scheduleSort],["filter","bim_schedule_filter",scheduleFilter],["group","bim_schedule_group",scheduleGroup],["storeys","bim_schedule_storey",text],["phases","bim_schedule_phase",phase]],
 property_templates:[["applies_to","bim_template_target",templateTarget]],classification_systems:[["entries","bim_classification_item",classificationItem]]
};
const profileRows=(value:domain.Profile):number=>1+("Custom"in value?sequence(value.Custom.outline).length:0);
const gridRows=(value:domain.CurtainGrid|undefined):number=>value===undefined?0:1+("Lines"in value?sequence(value.Lines.positions).length:0);
async function forecast(snapshot:ModelSnapshot,options:ArtifactSqliteOptions):Promise<number>{
 let rows=2,work=0;const add=async(n:number)=>{rows+=n;if(!Number.isSafeInteger(rows)||rows>(options.maxRows??1_000_000))fail("row limit");if(++work%256===0||options.signal?.aborted)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0)};
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);await add(sequence(snapshot.project.phase_names).length);
 for(const[key]of collections){const source=mapObject(snapshot[key]as Record<string,any>|undefined);for(const name in source)if(Object.hasOwn(source,name)){await add(1);const v=source[name];switch(key){
 case"column_types":case"beam_types":case"wall_sweeps":await add(profileRows(v.profile));break;
 case"curtain_wall_types":await add(profileRows(v.interior_mullion)+profileRows(v.border_mullion)+gridRows(v.u_grid)+gridRows(v.v_grid));break;
 case"walls":await add(2);break;case"curtain_walls":await add(2+gridRows(v.u_grid)+gridRows(v.v_grid));break;
 case"columns":case"stairs":case"ramps":case"beams":await add(1);break;
 case"slabs":case"ceilings":await add(sequence(v.holes).length);for(const h of sequence<domain.Vertex[]>(v.holes))await add(sequence(h).length);break;
 case"railings":await add(profileRows(v.profile)+profileRows(v.post_profile)+(v.baluster===undefined?0:1+profileRows(v.baluster.profile)));break;
 case"spaces":if("Explicit"in v.boundary)await add(sequence(v.boundary.Explicit.outline).length);break;
 case"dimensions":await add(sequence(v.anchors).length);break;case"leaders":await add(1);break;
 case"family_solids":if(!("Cuboid"in v.shape)){const body=Object.values(v.shape)[0]as any;await add(1+("Polygon"in body.profile?sequence(body.profile.Polygon.points).length:0));if("Sweep"in v.shape)await add(sequence(v.shape.Sweep.path).length)}break;
 case"property_templates":for(const def of sequence<domain.PropertyDef>(v.properties))await add(1+(def.default_value===undefined?0:1)+sequence(def.allowed).length);break;
 case"properties":{const sets=mapObject(v as domain.PropertySet);for(const setName in sets)if(Object.hasOwn(sets,setName)){await add(1);const properties=mapObject(sets[setName]);for(const propertyName in properties)if(Object.hasOwn(properties,propertyName))await add(1)}break}
 case"classifications":{const values=mapObject(v as domain.ClassificationSet);for(const system in values)if(Object.hasOwn(values,system))await add(1);break}
 }for(const[property]of orderedRoles[key]??[])await add(sequence(v[property]).length)}}
 await artifactSqliteCheckpoint(options,"projectSnapshot",work,work);return rows;
}
const owners=(width:number,column:number,id:bigint):Cells=>Array.from({length:width},(_,i)=>i===column?id:null);
async function ordered<T>(p:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly T[]|undefined,codec:Codec<T>):Promise<void>{const list=sequence(values);for(let i=0;i<list.length;i++){await p.checkpoint();await p.insert(table,[parent,BigInt(i),...codec.to(list[i]!)])}}
async function writeProfile(p:ArtifactSqliteProjection,parent:bigint,slot:number,value:domain.Profile):Promise<void>{const id=await p.insert("bim_profile",[...owners(8,slot,parent),...profile.to(value)]);if("Custom"in value)await ordered(p,"bim_profile_outline",id,value.Custom.outline,vertex)}
async function writeGrid(p:ArtifactSqliteProjection,parent:bigint,slot:number,value:domain.CurtainGrid|undefined):Promise<void>{if(value===undefined)return;const id=await p.insert("bim_curtain_grid",[...owners(4,slot,parent),...curtainGrid.to(value)]);if("Lines"in value)await ordered(p,"bim_curtain_grid_position",id,value.Lines.positions,number)}
async function writeAnchor(p:ArtifactSqliteProjection,parent:bigint,slot:number,index:number,value:domain.AnnotationAnchor):Promise<void>{await p.insert("bim_annotation_anchor",[...owners(2,slot,parent),BigInt(index),...anchor.to(value)])}
async function writeSolid(p:ArtifactSqliteProjection,parent:bigint,value:domain.SolidShape):Promise<void>{if("Cuboid"in value)return;const body=Object.values(value)[0]as any,profileValue=body.profile as domain.ParametricProfile,id=await p.insert("bim_parametric_profile",[parent,...parametricProfile.to(profileValue)]);if("Polygon"in profileValue)await ordered(p,"bim_parametric_polygon_point",id,profileValue.Polygon.points,exprPoint);if("Sweep"in value)await ordered(p,"bim_solid_sweep_point",parent,value.Sweep.path,exprPoint)}
function propertyCells(value:domain.PropertyValue):Cells{const objectValue=object(value),keys=Object.keys(objectValue);if(keys.length!==1)return fail("property variant count");const kind=keys[0]!,body=object(objectValue[kind]);if(Object.keys(body).length!==1||!Object.hasOwn(body,"value"))return fail("property value field");const v=body.value;if(kind==="Text")return[kind,...text.to(v as string),null,null,null,null,null];if(kind==="Integer")return[kind,null,...int.to(v as number),null,null,null,null];if(kind==="Boolean")return[kind,null,null,...boolean.to(v as boolean),null,null,null];if(!["Real","Length","Area","Volume","Angle"].includes(kind))return fail("property kind");return[kind,null,null,null,...number.to(v as number)]}

/** 📤️ Export every persisted BIM field through its authored relational owner. */
export async function bimModelSnapshotToSqliteDatabase(snapshot:ModelSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const rows=await forecast(snapshot,options),root=object(snapshot);for(const key of Object.keys(root))if(key!=="schema"&&key!=="project"&&!collections.some(([name])=>name===key))fail("undeclared snapshot field "+key);
 const p=await ArtifactSqliteProjection.create(BIM_MODEL_SQLITE_SCHEMA,options);p.checkRowsAdditional(rows);const doc=await p.insert("bim_document",text.to(snapshot.schema)),projectId=await p.insert("bim_project",[doc,...project.to(snapshot.project)]);await ordered(p,"bim_phase_name",projectId,snapshot.project.phase_names,text);
 for(const[key,table,codec]of collections){let ordinal=0;for(const[name,v]of await entries(snapshot[key]as Record<string,any>|undefined,options)){await p.checkpoint();const id=await p.insert(table,[doc,BigInt(ordinal++),literalText(name),...(codec===null?[]:codec.to(v))]);switch(key){
 case"column_types":case"beam_types":await writeProfile(p,id,key==="column_types"?0:1,v.profile);break;
 case"curtain_wall_types":await writeProfile(p,id,2,v.interior_mullion);await writeProfile(p,id,3,v.border_mullion);await writeGrid(p,id,0,v.u_grid);await writeGrid(p,id,1,v.v_grid);break;
 case"walls":case"curtain_walls":await p.insert("bim_axis",[...owners(3,key==="walls"?0:1,id),...axis.to(v.axis)]);await p.insert("bim_top",[...owners(5,key==="walls"?0:1,id),...top.to(v.top)]);if(key==="curtain_walls"){await writeGrid(p,id,2,v.u_grid);await writeGrid(p,id,3,v.v_grid)}break;
 case"beams":await p.insert("bim_axis",[...owners(3,2,id),...axis.to(v.axis)]);break;
 case"columns":case"stairs":case"ramps":await p.insert("bim_top",[...owners(5,key==="columns"?2:key==="stairs"?3:4,id),...top.to(v.top)]);break;
 case"slabs":case"ceilings":for(let i=0;i<sequence(v.holes).length;i++){const hole=await p.insert(key==="slabs"?"bim_slab_hole":"bim_ceiling_hole",[id,BigInt(i)]);await ordered(p,key==="slabs"?"bim_slab_hole_vertex":"bim_ceiling_hole_vertex",hole,v.holes[i],vertex)}break;
 case"railings":await writeProfile(p,id,4,v.profile);await writeProfile(p,id,5,v.post_profile);if(v.baluster!==undefined){const b=object(v.baluster);for(const k of Object.keys(b))if(k!=="profile"&&k!=="spacing")fail("undeclared baluster field");const baluster=await p.insert("bim_baluster",[id,...number.to(v.baluster.spacing)]);await writeProfile(p,baluster,6,v.baluster.profile)}break;
 case"spaces":if("Explicit"in v.boundary)await ordered(p,"bim_space_outline",id,v.boundary.Explicit.outline,vertex);break;
 case"wall_sweeps":await writeProfile(p,id,7,v.profile);break;
 case"dimensions":{const list=sequence<domain.AnnotationAnchor>(v.anchors);for(let i=0;i<list.length;i++)await writeAnchor(p,id,0,i,list[i]!);break}
 case"leaders":await writeAnchor(p,id,1,0,v.anchor);break;
 case"family_solids":await writeSolid(p,id,v.shape);break;
 case"property_templates":{const defs=sequence<domain.PropertyDef>(v.properties);for(let i=0;i<defs.length;i++){const def=defs[i]!,owner=await p.insert("bim_property_def",[id,BigInt(i),...propertyDef.to(def)]);if(def.default_value!==undefined)await p.insert("bim_property_default",[owner,...propertyCells(def.default_value)]);for(let j=0;j<sequence(def.allowed).length;j++)await p.insert("bim_property_allowed",[owner,BigInt(j),...propertyCells(def.allowed![j]!)]);}break}
 case"properties":{let j=0;for(const[setName,set]of await entries(v as domain.PropertySet,options)){const setId=await p.insert("bim_property_set",[id,BigInt(j++),literalText(setName)]);let k=0;for(const[propertyName,value]of await entries(set,options))await p.insert("bim_property_value",[setId,BigInt(k++),literalText(propertyName),...propertyCells(value)])}break}
 case"classifications":{let j=0;for(const[system,code]of await entries(v as domain.ClassificationSet,options))await p.insert("bim_classification_assignment",[id,BigInt(j++),literalText(system),...text.to(code)]);break}
 }for(const[property,child,childCodec]of orderedRoles[key]??[])await ordered(p,child,id,v[property],childCodec)}}
 return p.finish();
}
class Cursor{
 constructor(readonly row:SqliteRow,private index=1){}
 next():SqliteValue{if(this.index>=this.row.values.length)fail("absent scalar");return this.row.values[this.index++]!}
 text():string{return literalText(this.next())}
 integer():bigint{const v=this.next();return typeof v==="bigint"?v:fail("requires INTEGER")}
 empty(width:number):void{for(let i=0;i<width;i++)if(this.next()!==null)fail("unused parameter field")}
 absent(width:number):boolean{if(this.row.values.slice(this.index,this.index+width).every(v=>v===null)){this.empty(width);return true}return false}
 done():void{if(this.index!==this.row.values.length)fail("extra entity scalar")}
}
class Reader{
 readonly used=new Set<SqliteRow>();
 readonly indexes=new Map<string,Map<bigint,SqliteRow[]>>();
 private constructor(readonly tables:Map<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl,readonly options:ArtifactSqliteOptions){}
 static async create(database:SqliteDatabase,options:ArtifactSqliteOptions):Promise<Reader>{await artifactSqliteTables(database,BIM_MODEL_SQLITE_SCHEMA,options);const control=new NativeDecodeControl(0,p=>{options.onProgress?.({phase:"reconstructSnapshot",completed:p.completed,total:p.total});return!options.signal?.aborted},options.signal);await control.beginStage(database.tables.reduce((n,t)=>n+t.rows.length,0));return new Reader(new Map(database.tables.map(v=>[v.name,v.rows])),control,options)}
 all(name:string):readonly SqliteRow[]{return this.tables.get(name)??fail("missing table "+name)}
 async take(row:SqliteRow,start=1):Promise<Cursor>{if(this.used.has(row))fail("entity has multiple owners");await this.control.step();this.used.add(row);return new Cursor(row,start)}
 async group(name:string,parent:bigint,column=1,ordinal:number|null=2):Promise<SqliteRow[]>{const key=name+":"+column;let index=this.indexes.get(key);if(index===undefined){index=new Map();for(const row of this.all(name)){await this.control.checkpoint();const p=row.values[column];if(p!==null){if(typeof p!=="bigint")return fail("invalid owner key");const list=index.get(p);if(list)list.push(row);else index.set(p,[row])}}this.indexes.set(key,index)}const rows=index.get(parent)??[];return ordinal===null?rows:artifactSqliteOrderedRowsControlled(rows,ordinal,this.options)}
 async one(name:string,parent:bigint,column=1,required=true):Promise<SqliteRow|null>{const rows=await this.group(name,parent,column,null);if(rows.length>1||required&&rows.length!==1)fail("singleton cardinality "+name);return rows[0]??null}
 async finish():Promise<void>{for(const rows of this.tables.values())for(const row of rows){if(!this.used.has(row))fail("unowned entity");await this.control.checkpoint()}}
}
async function readOrdered<T>(r:Reader,table:string,parent:bigint,codec:Codec<T>):Promise<T[]>{const result:T[]=[];for(const row of await r.group(table,parent)){const c=await r.take(row,3);result.push(codec.from(c));c.done()}return result}
async function readOwned<T>(r:Reader,table:string,parent:bigint,slot:number,ownersCount:number,codec:Codec<T>):Promise<T>{const row=(await r.one(table,parent,slot+1))!,c=await r.take(row);for(let i=0;i<ownersCount;i++)if(c.next()!==(i===slot?parent:null))fail("multiple parameter owners");const v=codec.from(c);c.done();return v}
async function readProfile(r:Reader,parent:bigint,slot:number):Promise<domain.Profile>{const row=(await r.one("bim_profile",parent,slot+1))!,c=await r.take(row);for(let i=0;i<8;i++)if(c.next()!==(i===slot?parent:null))fail("profile owner differs");const v=profile.from(c);c.done();const outline=await readOrdered(r,"bim_profile_outline",row.rowid,vertex);if("Custom"in v)v.Custom.outline=outline;else if(outline.length)return fail("outline on noncustom profile");return v}
async function readGrid(r:Reader,parent:bigint,slot:number,required:boolean):Promise<domain.CurtainGrid|undefined>{const row=await r.one("bim_curtain_grid",parent,slot+1,required);if(row===null)return undefined;const c=await r.take(row);for(let i=0;i<4;i++)if(c.next()!==(i===slot?parent:null))fail("curtain grid owner differs");const value=curtainGrid.from(c);c.done();const positions=await readOrdered(r,"bim_curtain_grid_position",row.rowid,number);if("Lines"in value)value.Lines.positions=positions;else if(positions.length)fail("positions on spacing grid");return value}
async function readAnchors(r:Reader,parent:bigint,slot:number):Promise<domain.AnnotationAnchor[]>{const result:domain.AnnotationAnchor[]=[];for(const row of await r.group("bim_annotation_anchor",parent,slot+1,3)){const c=await r.take(row);for(let i=0;i<2;i++)if(c.next()!==(i===slot?parent:null))fail("annotation owner differs");c.integer();result.push(anchor.from(c));c.done()}return result}
async function readSolid(r:Reader,parent:bigint,value:domain.SolidShape):Promise<void>{const row=await r.one("bim_parametric_profile",parent,1,!("Cuboid"in value)),path=await readOrdered(r,"bim_solid_sweep_point",parent,exprPoint);if("Cuboid"in value){if(row!==null||path.length)fail("children on cuboid");return}const c=await r.take(row!,2),profileValue=parametricProfile.from(c);c.done();const points=await readOrdered(r,"bim_parametric_polygon_point",row!.rowid,exprPoint);if("Polygon"in profileValue)profileValue.Polygon.points=points;else if(points.length)fail("points on nonpolygon profile");const body=Object.values(value)[0]as any;body.profile=profileValue;if("Sweep"in value)value.Sweep.path=path;else if(path.length)fail("path on nonsweep solid")}
function propertyFrom(c:Cursor):domain.PropertyValue{const kind=c.text();if(kind==="Text"){const value=c.text();c.empty(5);return{Text:{value}}}if(kind==="Integer"){c.empty(1);const value=int.from(c);c.empty(4);return{Integer:{value}}}if(kind==="Boolean"){c.empty(2);const value=boolean.from(c);c.empty(3);return{Boolean:{value}}}if(!["Real","Length","Area","Volume","Angle"].includes(kind))return fail("property kind");c.empty(3);return{[kind]:{value:number.from(c)}}as domain.PropertyValue}
const assign=(target:ObjectValue,key:string,value:unknown):void=>{Object.defineProperty(target,key,{value,enumerable:true,writable:true,configurable:true})};
async function readNamed(r:Reader,table:string,parent:bigint):Promise<readonly(readonly[string,SqliteRow])[]>{const rows=await r.group(table,parent),result:[string,SqliteRow][]=[];let previous:string|undefined;const work:Work={units:0};for(const row of rows){await r.control.checkpoint();const key=literalText(row.values[3]);if(previous!==undefined&&await compareKeys(previous,key,r.options,"reconstructSnapshot",work)>=0)fail("map keys are not unique and ordered");result.push([key,row]);previous=key}return result}

/** 📥️ Restore only complete, uniquely owned BIM parameters from their semantic rows. */
export async function bimModelSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<ModelSnapshot>{
 const r=await Reader.create(database,options),documents=r.all("bim_document");if(documents.length!==1)fail("one document required");const doc=documents[0]!,dc=await r.take(doc),schema=dc.text();dc.done();const pr=(await r.one("bim_project",doc.rowid))!,pc=await r.take(pr,2),projectValue=project.from(pc);pc.done();projectValue.phase_names=await readOrdered(r,"bim_phase_name",pr.rowid,text);const snapshot:ModelSnapshot={schema,project:projectValue};
 for(const[key,table,codec]of collections){const map:ObjectValue={};for(const[name,row]of await readNamed(r,table,doc.rowid)){const c=await r.take(row,4),value: any=codec===null?{}:codec.from(c);c.done();switch(key){
 case"column_types":case"beam_types":value.profile=await readProfile(r,row.rowid,key==="column_types"?0:1);break;
 case"curtain_wall_types":value.interior_mullion=await readProfile(r,row.rowid,2);value.border_mullion=await readProfile(r,row.rowid,3);value.u_grid=await readGrid(r,row.rowid,0,true);value.v_grid=await readGrid(r,row.rowid,1,true);break;
 case"walls":case"curtain_walls":value.axis=await readOwned(r,"bim_axis",row.rowid,key==="walls"?0:1,3,axis);value.top=await readOwned(r,"bim_top",row.rowid,key==="walls"?0:1,5,top);if(key==="curtain_walls"){const u=await readGrid(r,row.rowid,2,false),v=await readGrid(r,row.rowid,3,false);if(u!==undefined)value.u_grid=u;if(v!==undefined)value.v_grid=v}break;
 case"beams":value.axis=await readOwned(r,"bim_axis",row.rowid,2,3,axis);break;
 case"columns":case"stairs":case"ramps":value.top=await readOwned(r,"bim_top",row.rowid,key==="columns"?2:key==="stairs"?3:4,5,top);break;
 case"slabs":case"ceilings":value.holes=[];for(const h of await r.group(key==="slabs"?"bim_slab_hole":"bim_ceiling_hole",row.rowid)){const hc=await r.take(h,3);hc.done();value.holes.push(await readOrdered(r,key==="slabs"?"bim_slab_hole_vertex":"bim_ceiling_hole_vertex",h.rowid,vertex))}break;
 case"railings":value.profile=await readProfile(r,row.rowid,4);value.post_profile=await readProfile(r,row.rowid,5);{const b=await r.one("bim_baluster",row.rowid,1,false);if(b){const bc=await r.take(b,2),spacing=number.from(bc);bc.done();value.baluster={spacing,profile:await readProfile(r,b.rowid,6)}}}break;
 case"spaces":{const outline=await readOrdered(r,"bim_space_outline",row.rowid,vertex);if("Explicit"in value.boundary)value.boundary.Explicit.outline=outline;else if(outline.length)fail("outline on bounded space");break}
 case"wall_sweeps":value.profile=await readProfile(r,row.rowid,7);break;
 case"dimensions":value.anchors=await readAnchors(r,row.rowid,0);break;
 case"leaders":{const anchors=await readAnchors(r,row.rowid,1);if(anchors.length!==1)fail("leader anchor cardinality");value.anchor=anchors[0];break}
 case"family_solids":await readSolid(r,row.rowid,value.shape);break;
 case"property_templates":value.properties=[];for(const defRow of await r.group("bim_property_def",row.rowid)){const c=await r.take(defRow,3),def=propertyDef.from(c);c.done();const defaultRow=await r.one("bim_property_default",defRow.rowid,1,false);if(defaultRow){const c=await r.take(defaultRow,2);def.default_value=propertyFrom(c);c.done()}def.allowed=[];for(const allowedRow of await r.group("bim_property_allowed",defRow.rowid)){const c=await r.take(allowedRow,3);def.allowed.push(propertyFrom(c));c.done()}value.properties.push(def)}break;
 case"properties":for(const[setName,setRow]of await readNamed(r,"bim_property_set",row.rowid)){const sc=await r.take(setRow,4);sc.done();const set:ObjectValue={};for(const[propertyName,propertyRow]of await readNamed(r,"bim_property_value",setRow.rowid)){const vc=await r.take(propertyRow,4);assign(set,propertyName,propertyFrom(vc));vc.done()}assign(value,setName,set)}break;
 case"classifications":for(const[system,assignment]of await readNamed(r,"bim_classification_assignment",row.rowid)){const c=await r.take(assignment,4);assign(value,system,c.text());c.done()}break;
 }for(const[property,child,childCodec]of orderedRoles[key]??[])value[property]=await readOrdered(r,child,row.rowid,childCodec);assign(map,name,value)}assign(snapshot as unknown as ObjectValue,key,map)}
 await r.finish();return snapshot;
}
