/** 🏔️ Three authored entities preserve parameters and independent mesh identity. */
import type {GisTerrainArtifact} from "../../🟦️.ts";
import type {ArtifactDialect} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteTextBytes,artifactSqliteValueBudget,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {parseBinary64,encodeIeee754Cells,readBinary64,ieee754CellByteLength} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export const GIS_TERRAIN_SQLITE_SCHEMA=String.raw`CREATE TABLE gis_terrain_document (id INTEGER PRIMARY KEY);
CREATE TABLE gis_terrain_parameters (id INTEGER PRIMARY KEY REFERENCES gis_terrain_document(id), exaggeration REAL, imported_features_json TEXT NOT NULL, exaggeration_ieee754_bits INTEGER NOT NULL, exaggeration_numeric_class TEXT NOT NULL CHECK(exaggeration_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE gis_terrain_mesh_child (id INTEGER PRIMARY KEY REFERENCES gis_terrain_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
`;
const SCALAR=[{index:1,width:64}]as const;
/** 📤️ Admit exact field bytes before creating copied cells. */
export async function gisTerrainSnapshotToSqliteDatabase(snapshot:GisTerrainArtifact,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(GIS_TERRAIN_SQLITE_SCHEMA,options);parseBinary64(snapshot.exaggeration);let bytes=16+artifactSqliteTextBytes(snapshot.importedFeaturesJson)+ieee754CellByteLength(snapshot.exaggeration,64);
 if(snapshot.mesh){bytes+=8;for(const value of[snapshot.mesh.childId,snapshot.mesh.target.artifactId,snapshot.mesh.target.dialect.artifactKind,snapshot.mesh.target.dialect.standard,snapshot.mesh.target.dialect.subset])bytes+=artifactSqliteTextBytes(value);}
 p.checkRowsAdditional(snapshot.mesh?3:2);p.checkValueBytesAdditional(bytes);await p.insert("gis_terrain_document",[],1n);await p.insert("gis_terrain_parameters",encodeIeee754Cells([1n,snapshot.exaggeration,snapshot.importedFeaturesJson],SCALAR,options.maxColumns).slice(1),1n);
 if(snapshot.mesh){const c=snapshot.mesh;await p.insert("gis_terrain_mesh_child",[c.childId,c.target.artifactId,c.target.dialect.artifactKind,c.target.dialect.standard,c.target.dialect.subset],1n);}return p.finish();
}
/** 📥️ Validate each one-to-one parent before restoring fields under the same admission. */
export async function gisTerrainSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<GisTerrainArtifact>{
 const tables=await artifactSqliteTables(database,GIS_TERRAIN_SQLITE_SCHEMA,options);if(tables[0]!.length!==1||tables[1]!.length!==1||tables[2]!.length>1)throw Error("GIS terrain entity ownership differs");const document=tables[0]![0]!,parameters=tables[1]![0]!;
 const identity=(row:SqliteRow,width:number)=>{if(row.values.length!==width||integer(row,0)!==row.rowid)throw Error("GIS terrain row identity differs");};identity(document,1);identity(parameters,5);if(parameters.rowid!==document.rowid)throw Error("GIS terrain parameter owner differs");let bytes=0;
 const ownText=(row:SqliteRow,index:number)=>{const value=text(row,index);bytes+=artifactSqliteTextBytes(value);artifactSqliteValueBudget(bytes,options);return value;};
 const exaggeration=readBinary64(parameters,1,SCALAR);bytes+=8;artifactSqliteValueBudget(bytes,options);const value:GisTerrainArtifact={exaggeration,importedFeaturesJson:ownText(parameters,2)};
 if(tables[2]!.length){const row=tables[2]![0]!;identity(row,6);if(row.rowid!==document.rowid)throw Error("GIS terrain mesh owner differs");value.mesh={childId:ownText(row,1),target:{artifactId:ownText(row,2),dialect:{artifactKind:ownText(row,3),standard:ownText(row,4),subset:ownText(row,5)}}};}await artifactSqliteCheckpoint(options,"reconstructSnapshot",tables[2]!.length+2,tables[2]!.length+2);return value;
}
/** 🧭️ Exact declared coordinates admit a complete matching typed state, independent of surrogate IDs. */
export async function validateGisTerrainSnapshotSqliteDialect(snapshot:GisTerrainArtifact,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.gis.gisterrain"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("GIS terrain does not own this semantic subset");const restored=await gisTerrainSnapshotFromSqliteDatabase(database,options);
 if(snapshot.exaggeration.bits!==restored.exaggeration.bits||snapshot.importedFeaturesJson!==restored.importedFeaturesJson||!!snapshot.mesh!==!!restored.mesh)throw Error("GIS terrain owned identity differs");if(snapshot.mesh&&restored.mesh){const a=snapshot.mesh,b=restored.mesh;if(a.childId!==b.childId||a.target.artifactId!==b.target.artifactId||a.target.dialect.artifactKind!==b.target.dialect.artifactKind||a.target.dialect.standard!==b.target.dialect.standard||a.target.dialect.subset!==b.target.dialect.subset)throw Error("GIS terrain child identity differs");}return[];
}
