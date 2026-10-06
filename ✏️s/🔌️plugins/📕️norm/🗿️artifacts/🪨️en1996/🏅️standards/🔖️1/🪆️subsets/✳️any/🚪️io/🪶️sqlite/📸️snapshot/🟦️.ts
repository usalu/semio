/// <reference path="./🗄️.d.ts" />
import sql from"./🗄️.sql"with{type:"text"};
import type{En1996Snapshot,MasonryWall,WallOpening,WallLoadCase,ConcentratedLoad}from"../../../🧬️schema/📸️snapshot/🟦️.ts";
import{ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteBoolean,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,type Ieee754Column,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

const ANNEX=["En","De"]as const;
const MASONRY_CLASS=["Class1","Class2","Class3","Class4","Class5"]as const;
const DESIGN_SITUATION=["Persistent","Transient","Accidental","Seismic"]as const;
const WALL_TYPE=["LoadBearing","Shear","NonLoadBearing"]as const;
const UNIT_GROUP=["Group1","Group2","Group3","Group4"]as const;
const UNIT_MATERIAL=["Clay","CalciumSilicate","Aerated","Concrete"]as const;
const MORTAR_TYPE=["GeneralPurpose","ThinLayer","Lightweight"]as const;
const MORTAR_CLASS=["M1","M2_5","M5","M10","M15","M20"]as const;
const EXPOSURE=["Mx1","Mx2","Mx3","Mx4","Mx5"]as const;
export type En1996SqliteOpening=Omit<WallOpening,"widthM"|"heightM"|"sillHeightM">&{widthM:Binary64;heightM:Binary64;sillHeightM:Binary64};
export type En1996SqliteConcentratedLoad=Omit<ConcentratedLoad,"forceN"|"bearingAreaM2"|"bearingLengthM">&{forceN:Binary64;bearingAreaM2:Binary64;bearingLengthM:Binary64};
export type En1996SqliteLoadCase=Omit<WallLoadCase,"gKSlabN"|"qKImposedPa"|"tributaryAreaM2"|"slabSpanM"|"qKSnowPa"|"qPWindPa"|"cPe"|"hKEarthN"|"concentrated">&{gKSlabN:Binary64;qKImposedPa:Binary64;tributaryAreaM2:Binary64;slabSpanM:Binary64;qKSnowPa:Binary64;qPWindPa:Binary64;cPe:Binary64;hKEarthN:Binary64;concentrated:En1996SqliteConcentratedLoad[]};
export type En1996SqliteWall=Omit<MasonryWall,"wallType"|"unitGroup"|"unitMaterial"|"mortarType"|"mortarClass"|"exposure"|"thicknessM"|"heightM"|"lengthM"|"slabBearingDepthM"|"eccentricityTopM"|"eccentricityBottomM"|"fBPa"|"unitLengthM"|"unitWidthM"|"unitHeightM"|"mortarStrengthPa"|"bedJointThicknessM"|"asVerticalM2"|"asHorizontalM2"|"fYdPa"|"mu"|"densityKgM3"|"phiInfinity"|"openings"|"loadCases">&{
 wallType:typeof WALL_TYPE[number];unitGroup:typeof UNIT_GROUP[number];unitMaterial:typeof UNIT_MATERIAL[number];mortarType:typeof MORTAR_TYPE[number];mortarClass:typeof MORTAR_CLASS[number];exposure:typeof EXPOSURE[number];
 thicknessM:Binary64;heightM:Binary64;lengthM:Binary64;slabBearingDepthM:Binary64;eccentricityTopM:Binary64;eccentricityBottomM:Binary64;fBPa:Binary64;unitLengthM:Binary64;unitWidthM:Binary64;unitHeightM:Binary64;mortarStrengthPa:Binary64;bedJointThicknessM:Binary64;asVerticalM2:Binary64;asHorizontalM2:Binary64;fYdPa:Binary64;mu:Binary64;densityKgM3:Binary64;phiInfinity:Binary64;openings:En1996SqliteOpening[];loadCases:En1996SqliteLoadCase[];
};
/** 🪨️ Complete persisted masonry snapshot with exact words in every numeric domain. */
export type En1996SqliteSnapshot=Omit<En1996Snapshot,"annex"|"masonryClass"|"designSituation"|"walls">&{annex:typeof ANNEX[number];masonryClass:typeof MASONRY_CLASS[number];designSituation:typeof DESIGN_SITUATION[number];walls:En1996SqliteWall[]};
export const EN1996_SQLITE_SCHEMA:string=sql;
const WALL_FLOATS:readonly Ieee754Column[]=[7,8,9,11,12,13,16,17,18,19,22,23,25,26,27,30,31,32].map(index=>({index,width:64}));
const OPENING_FLOATS:readonly Ieee754Column[]=[4,5,6].map(index=>({index,width:64}));
const LOAD_CASE_FLOATS:readonly Ieee754Column[]=[6,7,8,9,10,11,12,13].map(index=>({index,width:64}));
const CONCENTRATED_FLOATS:readonly Ieee754Column[]=[4,5,6].map(index=>({index,width:64}));
type Entities=Map<bigint,SqliteRow>;
function choice<T extends string>(value:string,choices:readonly T[]):T{if(!choices.includes(value as T))throw Error("EN1996 unsupported owned choice");return value as T}
function unsigned(value:number,max:number):bigint{if(!Number.isInteger(value)||value<0||value>max)throw Error("EN1996 field exceeds owned unsigned width");return BigInt(value)}
function readUnsigned(row:SqliteRow,column:number,max:bigint):number{const value=artifactSqliteInteger(row,column);if(value<0n||value>max)throw Error("EN1996 field exceeds owned unsigned width");return Number(value)}
function flag(value:boolean):bigint{if(typeof value!=="boolean")throw Error("EN1996 boolean is not owned");return value?1n:0n}
async function checkpoint(options:ArtifactSqliteOptions,position:number,total:number):Promise<void>{if(position%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",position,total,position>0)}
async function entities(rows:readonly SqliteRow[],columns:number,floats:readonly Ieee754Column[],options:ArtifactSqliteOptions):Promise<Entities>{const result:Entities=new Map();for(let position=0;position<rows.length;position++){await checkpoint(options,position,rows.length);const row=rows[position]!;if(row.values.length!==columns+floats.length*2||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||result.has(row.rowid))throw Error("EN1996 requires exact fields and unique positive aliased identities");for(const column of floats)readBinary64(row,column.index,floats);result.set(row.rowid,row)}return result}
async function ordered(rows:Iterable<SqliteRow>,parent:bigint,options:ArtifactSqliteOptions):Promise<SqliteRow[]>{const result:SqliteRow[]=[];for(const row of rows){await checkpoint(options,result.length,0);if(artifactSqliteInteger(row,1)!==parent)throw Error("EN1996 entity has a foreign parent");result.push(row)}return artifactSqliteOrderedRowsControlled(result,2,options)}
async function groups(rows:Entities,parents:Entities,options:ArtifactSqliteOptions):Promise<Map<bigint,SqliteRow[]>>{const result=new Map<bigint,SqliteRow[]>();let position=0;for(const row of rows.values()){await checkpoint(options,position++,rows.size);const parent=artifactSqliteInteger(row,1);if(!parents.has(parent))throw Error("EN1996 child has an unknown owning entity");const children=result.get(parent)??[];children.push(row);result.set(parent,children)}return result}

async function write(out:ArtifactSqliteProjection,table:string,cells:readonly Ieee754Cell[],floats:readonly Ieee754Column[],options:ArtifactSqliteOptions):Promise<bigint>{return out.insert(table,encodeIeee754Cells([0n,...cells],floats,options.maxColumns).slice(1))}

/** 📤️ Projects the complete handwritten masonry domain into separately queryable entities. */
export async function en1996SnapshotToSqliteDatabase(snapshot:En1996SqliteSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);
 await out.insert("en1996_document",[choice(snapshot.annex,ANNEX),choice(snapshot.masonryClass,MASONRY_CLASS),choice(snapshot.designSituation,DESIGN_SITUATION),unsigned(snapshot.storeys,4294967295)],1n);
 for(let position=0;position<snapshot.walls.length;position++){
  const w=snapshot.walls[position]!,id=await write(out,"en1996_wall",[1n,BigInt(position),w.id,w.labelEn,w.labelDe,choice(w.wallType,WALL_TYPE),w.thicknessM,w.heightM,w.lengthM,unsigned(w.supportSides,255),w.slabBearingDepthM,w.eccentricityTopM,w.eccentricityBottomM,choice(w.unitGroup,UNIT_GROUP),choice(w.unitMaterial,UNIT_MATERIAL),w.fBPa,w.unitLengthM,w.unitWidthM,w.unitHeightM,choice(w.mortarType,MORTAR_TYPE),choice(w.mortarClass,MORTAR_CLASS),w.mortarStrengthPa,w.bedJointThicknessM,flag(w.reinforced),w.asVerticalM2,w.asHorizontalM2,w.fYdPa,unsigned(w.fireReiMin,4294967295),choice(w.exposure,EXPOSURE),w.mu,w.densityKgM3,w.phiInfinity,flag(w.isBasement)],WALL_FLOATS,options);
  for(let position=0;position<w.openings.length;position++){const o=w.openings[position]!;await write(out,"en1996_opening",[id,BigInt(position),o.id,o.widthM,o.heightM,o.sillHeightM],OPENING_FLOATS,options)}
  for(let position=0;position<w.loadCases.length;position++){
   const c=w.loadCases[position]!,caseId=await write(out,"en1996_load_case",[id,BigInt(position),c.id,c.designSituation,c.imposedCategory,c.gKSlabN,c.qKImposedPa,c.tributaryAreaM2,c.slabSpanM,c.qKSnowPa,c.qPWindPa,c.cPe,c.hKEarthN],LOAD_CASE_FLOATS,options);
   for(let position=0;position<c.concentrated.length;position++){const l=c.concentrated[position]!;await write(out,"en1996_concentrated_load",[caseId,BigInt(position),l.id,l.forceN,l.bearingAreaM2,l.bearingLengthM],CONCENTRATED_FLOATS,options)}
  }
 }
 return out.finish();
}

/** 📥️ Reconstructs all masonry fields after exact choice, width, word, ordinal and parent validation. */
export async function en1996SnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<En1996SqliteSnapshot>{
 const tables=await artifactSqliteTables(database,sql,options),documents=await entities(tables[0]!,5,[],options);if(documents.size!==1||!documents.has(1n))throw Error("EN1996 requires exactly document identity one");const d=documents.get(1n)!;
 const wallRows=await entities(tables[1]!,34,WALL_FLOATS,options),caseRows=await entities(tables[3]!,14,LOAD_CASE_FLOATS,options),openings=await groups(await entities(tables[2]!,7,OPENING_FLOATS,options),wallRows,options),loads=await groups(await entities(tables[4]!,7,CONCENTRATED_FLOATS,options),caseRows,options),cases=await groups(caseRows,wallRows,options);
 const walls:En1996SqliteWall[]=[];
 for(const r of await ordered(wallRows.values(),1n,options)){
  await checkpoint(options,walls.length,0);const wallOpenings:En1996SqliteOpening[]=[];for(const o of await ordered(openings.get(r.rowid)??[],r.rowid,options)){await checkpoint(options,wallOpenings.length,0);wallOpenings.push({id:artifactSqliteText(o,3),widthM:readBinary64(o,4,OPENING_FLOATS),heightM:readBinary64(o,5,OPENING_FLOATS),sillHeightM:readBinary64(o,6,OPENING_FLOATS)})}
  const loadCases:En1996SqliteLoadCase[]=[];for(const c of await ordered(cases.get(r.rowid)??[],r.rowid,options)){
   await checkpoint(options,loadCases.length,0);const concentrated:En1996SqliteConcentratedLoad[]=[];for(const l of await ordered(loads.get(c.rowid)??[],c.rowid,options)){await checkpoint(options,concentrated.length,0);concentrated.push({id:artifactSqliteText(l,3),forceN:readBinary64(l,4,CONCENTRATED_FLOATS),bearingAreaM2:readBinary64(l,5,CONCENTRATED_FLOATS),bearingLengthM:readBinary64(l,6,CONCENTRATED_FLOATS)})}
   loadCases.push({id:artifactSqliteText(c,3),designSituation:artifactSqliteText(c,4),imposedCategory:artifactSqliteText(c,5),gKSlabN:readBinary64(c,6,LOAD_CASE_FLOATS),qKImposedPa:readBinary64(c,7,LOAD_CASE_FLOATS),tributaryAreaM2:readBinary64(c,8,LOAD_CASE_FLOATS),slabSpanM:readBinary64(c,9,LOAD_CASE_FLOATS),qKSnowPa:readBinary64(c,10,LOAD_CASE_FLOATS),qPWindPa:readBinary64(c,11,LOAD_CASE_FLOATS),cPe:readBinary64(c,12,LOAD_CASE_FLOATS),hKEarthN:readBinary64(c,13,LOAD_CASE_FLOATS),concentrated});
  }
  walls.push({id:artifactSqliteText(r,3),labelEn:artifactSqliteText(r,4),labelDe:artifactSqliteText(r,5),wallType:choice(artifactSqliteText(r,6),WALL_TYPE),thicknessM:readBinary64(r,7,WALL_FLOATS),heightM:readBinary64(r,8,WALL_FLOATS),lengthM:readBinary64(r,9,WALL_FLOATS),supportSides:readUnsigned(r,10,255n),openings:wallOpenings,slabBearingDepthM:readBinary64(r,11,WALL_FLOATS),eccentricityTopM:readBinary64(r,12,WALL_FLOATS),eccentricityBottomM:readBinary64(r,13,WALL_FLOATS),unitGroup:choice(artifactSqliteText(r,14),UNIT_GROUP),unitMaterial:choice(artifactSqliteText(r,15),UNIT_MATERIAL),fBPa:readBinary64(r,16,WALL_FLOATS),unitLengthM:readBinary64(r,17,WALL_FLOATS),unitWidthM:readBinary64(r,18,WALL_FLOATS),unitHeightM:readBinary64(r,19,WALL_FLOATS),mortarType:choice(artifactSqliteText(r,20),MORTAR_TYPE),mortarClass:choice(artifactSqliteText(r,21),MORTAR_CLASS),mortarStrengthPa:readBinary64(r,22,WALL_FLOATS),bedJointThicknessM:readBinary64(r,23,WALL_FLOATS),reinforced:artifactSqliteBoolean(r,24),asVerticalM2:readBinary64(r,25,WALL_FLOATS),asHorizontalM2:readBinary64(r,26,WALL_FLOATS),fYdPa:readBinary64(r,27,WALL_FLOATS),fireReiMin:readUnsigned(r,28,4294967295n),exposure:choice(artifactSqliteText(r,29),EXPOSURE),mu:readBinary64(r,30,WALL_FLOATS),densityKgM3:readBinary64(r,31,WALL_FLOATS),phiInfinity:readBinary64(r,32,WALL_FLOATS),isBasement:artifactSqliteBoolean(r,33),loadCases});
 }
 const result:En1996SqliteSnapshot={annex:choice(artifactSqliteText(d,1),ANNEX),masonryClass:choice(artifactSqliteText(d,2),MASONRY_CLASS),designSituation:choice(artifactSqliteText(d,3),DESIGN_SITUATION),storeys:readUnsigned(d,4,4294967295n),walls};await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,1);return result;
}
