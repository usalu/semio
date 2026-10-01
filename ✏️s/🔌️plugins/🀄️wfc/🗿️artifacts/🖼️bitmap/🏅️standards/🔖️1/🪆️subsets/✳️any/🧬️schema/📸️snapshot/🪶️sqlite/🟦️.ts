/** 🖼️ Hand-authored palette, problem parameters and pinned pixel relationships. */
import type {BitmapSnapshot,BitmapColor,BitmapPinnedPixel} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteBoolean,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const WFC_BITMAP_SQLITE_SCHEMA=`CREATE TABLE wfc_bitmap_document (id INTEGER PRIMARY KEY, schema_id TEXT NOT NULL, seed TEXT NOT NULL);
CREATE TABLE wfc_bitmap_input (id INTEGER PRIMARY KEY REFERENCES wfc_bitmap_document(id), width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295), palette_indices_base64 TEXT NOT NULL);
CREATE TABLE wfc_bitmap_palette (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_bitmap_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), alpha INTEGER NOT NULL CHECK(alpha BETWEEN 0 AND 4294967295));
CREATE TABLE wfc_bitmap_output (id INTEGER PRIMARY KEY REFERENCES wfc_bitmap_document(id), width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295), periodic INTEGER NOT NULL CHECK(periodic IN(0,1)));
CREATE TABLE wfc_bitmap_model (id INTEGER PRIMARY KEY REFERENCES wfc_bitmap_document(id), pattern_size INTEGER NOT NULL CHECK(pattern_size BETWEEN 0 AND 4294967295), symmetry INTEGER NOT NULL CHECK(symmetry BETWEEN 0 AND 4294967295), periodic_input INTEGER NOT NULL CHECK(periodic_input IN(0,1)), ground_palette_index INTEGER CHECK(ground_palette_index BETWEEN 0 AND 4294967295));
CREATE TABLE wfc_bitmap_pin (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES wfc_bitmap_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), x INTEGER NOT NULL CHECK(x BETWEEN 0 AND 4294967295), y INTEGER NOT NULL CHECK(y BETWEEN 0 AND 4294967295), palette_index INTEGER NOT NULL CHECK(palette_index BETWEEN 0 AND 4294967295));
`;
function identity(row:SqliteRow,count:number):void{if(row.values.length!==count||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid)throw Error("bitmap entity identity or field count differs");}
function unsigned(value:number):bigint{if(!Number.isInteger(value)||value<0||value>4294967295)throw Error("bitmap scalar exceeds unsigned32");return BigInt(value);}
function word(row:SqliteRow,index:number):number{const n=artifactSqliteInteger(row,index);if(n<0n||n>4294967295n)throw Error("bitmap scalar exceeds unsigned32");return Number(n);}
function flag(value:boolean):bigint{if(typeof value!=="boolean")throw Error("bitmap boolean differs");return value?1n:0n;}
/** 📤️ Keeps the authored scalar pixel string and all ordered problem entities. */
export async function bitmapSnapshotToSqliteDatabase(s:BitmapSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(WFC_BITMAP_SQLITE_SCHEMA,options);p.checkRowsAdditional(4+s.input.palette.length+s.pinned.length);
 if(typeof s.seed!=="bigint"||s.seed<0n||s.seed>18446744073709551615n)throw Error("bitmap seed exceeds unsigned64");
 await p.insert("wfc_bitmap_document",[s.schema,s.seed.toString()],1n);
 await p.insert("wfc_bitmap_input",[unsigned(s.input.width),unsigned(s.input.height),s.input.pixels],1n);
 await p.insert("wfc_bitmap_output",[unsigned(s.output.width),unsigned(s.output.height),flag(s.output.periodic)],1n);
 await p.insert("wfc_bitmap_model",[unsigned(s.model.patternSize),unsigned(s.model.symmetry),flag(s.model.periodicInput),s.model.ground==null?null:unsigned(s.model.ground)],1n);
 for(let i=0;i<s.input.palette.length;i++){const c=s.input.palette[i]!;await p.insert("wfc_bitmap_palette",[1n,BigInt(i),unsigned(c.r),unsigned(c.g),unsigned(c.b),unsigned(c.a)]);}
 for(let i=0;i<s.pinned.length;i++){const c=s.pinned[i]!;await p.insert("wfc_bitmap_pin",[1n,BigInt(i),unsigned(c.x),unsigned(c.y),unsigned(c.color)]);}
 return p.finish();
}
/** 📥️ Checks ownership, scalar widths and contiguous source order before reconstructing. */
export async function bitmapSnapshotFromSqliteDatabase(d:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<BitmapSnapshot>{
 const [documents,inputs,palettes,outputs,models,pins]=await artifactSqliteTables(d,WFC_BITMAP_SQLITE_SCHEMA,options);
 const document=artifactSqliteDocument(documents!),input=artifactSqliteDocument(inputs!),output=artifactSqliteDocument(outputs!),model=artifactSqliteDocument(models!);identity(document,3);identity(input,4);identity(output,4);identity(model,5);
 const seedText=artifactSqliteText(document,2);if(!/^(0|[1-9][0-9]*)$/.test(seedText)||seedText.length>20)throw Error("bitmap seed requires unsigned64 decimal");const seed=BigInt(seedText);if(seed>18446744073709551615n)throw Error("bitmap seed exceeds unsigned64");
 const palette:BitmapColor[]=[],pinned:BitmapPinnedPixel[]=[];let count=0;const ids=new Set<bigint>();
 for(const row of await artifactSqliteOrderedRowsControlled(palettes!,2,options)){identity(row,7);artifactSqliteDocumentReference(row,1);if(ids.has(row.rowid))throw Error("duplicate bitmap palette identity");ids.add(row.rowid);palette.push({r:word(row,3),g:word(row,4),b:word(row,5),a:word(row,6)});if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);}
 ids.clear();for(const row of await artifactSqliteOrderedRowsControlled(pins!,2,options)){identity(row,6);artifactSqliteDocumentReference(row,1);if(ids.has(row.rowid))throw Error("duplicate bitmap pin identity");ids.add(row.rowid);pinned.push({x:word(row,3),y:word(row,4),color:word(row,5)});if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);}
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,count);
 return{schema:artifactSqliteText(document,1),seed,input:{width:word(input,1),height:word(input,2),pixels:artifactSqliteText(input,3),palette},output:{width:word(output,1),height:word(output,2),periodic:artifactSqliteBoolean(output,3)},model:{patternSize:word(model,1),symmetry:word(model,2),periodicInput:artifactSqliteBoolean(model,3),ground:model.values[4]===null?null:word(model,4)},pinned};
}
