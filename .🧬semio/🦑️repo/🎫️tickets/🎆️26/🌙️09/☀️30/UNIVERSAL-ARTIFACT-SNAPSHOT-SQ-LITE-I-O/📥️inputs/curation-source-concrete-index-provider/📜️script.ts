import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const path="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
const before=readFileSync(path,"utf8");
const replacement=String.raw`
export async function curationSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<CurationSqliteSnapshot>{
 options=sqliteOperation(options);
 const tables=await artifactSqliteTables(database,sql,options),total=tables.reduce((n,rows)=>n+rows.length,0),names=["curation_document","curation_catalog","curation_stock_extra","curation_typology_segment","curation_geometry","curation_box","curation_frame","curation_slab","curation_mesh","curation_glb","curation_mesh_position","curation_mesh_normal","curation_mesh_index","curation_curated"],columns=[1,7,7,4,3,11,14,11,2,6,6,6,4,5];
 let completed=0;
 for(let at=0;at<tables.length;at++)for(const row of tables[at]!){alias(row,columns[at]!);if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total)}
 const index=await ArtifactSqliteRowIndex.create(database.tables,options),document=artifactSqliteDocument(tables[0]!);
 if(tables[1]!.length!==1)throw Error("Curation requires exactly one mandatory Kit catalog");
 const catalog=tables[1]![0]!;if(artifactSqliteInteger(catalog,1)!==1n)throw Error("Curation catalog has wrong document owner");
 const child=parseCurationArtifact({catalog:{childId:artifactSqliteText(catalog,2),target:{artifactId:artifactSqliteText(catalog,3),dialect:{artifactKind:artifactSqliteText(catalog,4),standard:artifactSqliteText(catalog,5),subset:artifactSqliteText(catalog,6)}}},stockExtra:[],curated:[]}).catalog;
 await index.take(document);await index.take(catalog);
 const one=async(table:number,parent:bigint):Promise<SqliteRow>=>{const rows=await index.grouped(names[table]!,parent,null);if(rows.length!==1)throw Error("Curation recipe requires one complete matching entity");const row=rows[0]!;await index.take(row);return row};
 const stockExtra:CurationSqliteSnapshot["stockExtra"]=[],curated:CurationSnapshot["curated"]=[];
 for(const row of await index.grouped(names[2]!,1n)){
  await index.take(row);const recipe=await one(4,row.rowid),kind=artifactSqliteText(recipe,2);let geometry:CurationSqliteGeometry;
  switch(kind){
   case"box":{const r=await one(5,recipe.rowid);geometry={kind,width:readBinary64(r,2,BOX),height:readBinary64(r,3,BOX),depth:readBinary64(r,4,BOX)};break}
   case"frame":{const r=await one(6,recipe.rowid);geometry={kind,width:readBinary64(r,2,FRAME),height:readBinary64(r,3,FRAME),depth:readBinary64(r,4,FRAME),profile:readBinary64(r,5,FRAME)};break}
   case"slab":{const r=await one(7,recipe.rowid);geometry={kind,width:readBinary64(r,2,BOX),depth:readBinary64(r,3,BOX),thickness:readBinary64(r,4,BOX)};break}
   case"glb":{const r=await one(9,recipe.rowid);geometry={kind,url:artifactSqliteText(r,2),extent:readBinary64(r,3,GLB)};break}
   case"mesh":{
    const r=await one(8,recipe.rowid),positions:Binary32[]=[],normals:Binary32[]=[],indices:number[]=[];
    for(const row of await index.grouped(names[10]!,r.rowid)){positions.push(readBinary32(row,3,MESH));await index.take(row)}
    for(const row of await index.grouped(names[11]!,r.rowid)){normals.push(readBinary32(row,3,MESH));await index.take(row)}
    for(const row of await index.grouped(names[12]!,r.rowid)){indices.push(readUint(row,3));await index.take(row)}
    geometry={kind,positions,normals,indices};break
   }
   default:throw Error("Curation recipe kind is undeclared");
  }
  const typologyPath:string[]=[];for(const entry of await index.grouped(names[3]!,row.rowid)){typologyPath.push(artifactSqliteText(entry,3));await index.take(entry)}
  stockExtra.push({id:artifactSqliteText(row,3),name:artifactSqliteText(row,4),moduleId:artifactSqliteText(row,5),typologyPath,availability:readUint(row,6),geometry});
 }
 for(const row of await index.grouped(names[13]!,1n)){curated.push({objectId:artifactSqliteText(row,3),count:readUint(row,4)});await index.take(row)}
 await index.finish();return{catalog:child,stockExtra,curated};
}
`;
const start=before.indexOf("export async function curationSnapshotFromSqliteDatabase(");assert(start>0);
let after=before.slice(0,start)+replacement.trimStart();
after=after.replace("ArtifactSqliteProjection,artifactSqliteTables","ArtifactSqliteProjection,ArtifactSqliteRowIndex,artifactSqliteTables").replace(",artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions",",type ArtifactSqliteOptions");
assert(!after.includes("new Map<"));assert(!after.includes("artifactSqliteOrderedRowsControlled"));
writeFileSync(join(import.meta.dir,"held-provider-pairs.json"),JSON.stringify([{path,before,after}],null,2)+"\n");
console.log("[DEBUG] Curation complete Source byte-backed identity and parent/ordinal index staged production_mutations=0");
