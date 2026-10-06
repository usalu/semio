import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
if(process.argv[2]==="mount"){
 const pairs=JSON.parse(readFileSync(join(import.meta.dir,"held-pairs.json"),"utf8"));
 for(const pair of pairs)assert.equal(existsSync(pair.path)?readFileSync(pair.path,"utf8"):null,pair.before,"Concurrent Generation demand change: "+pair.path);
 for(const pair of pairs){mkdirSync(join(pair.path,".."),{recursive:true});writeFileSync(pair.path,pair.after)}
 console.log("[DEBUG] Generation neutral complete semantic demand mounted paths="+pairs.length+" rows=122 tables=36 cases=10");
 process.exit(0);
}
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural";
const fixture={schemaVersion:1,rows:122,tables:["generation_document","generation_host","generation_widget","generation_neuron_widget","generation_widget_port","generation_slider_widget","generation_note_widget","generation_image_widget","generation_variable_widget","generation_preview_widget","generation_widget_expanded","generation_action_widget","generation_export_widget","generation_cluster_widget","generation_host_synapse","generation_host_layout","generation_neural_dictionary","generation_neural_entry","generation_neural_value","generation_tree","generation_tree_neuron","generation_tree_synapse","generation_gui","generation_gui_node","generation_gui_plain","generation_gui_slider","generation_gui_note","generation_gui_image","generation_gui_variable","generation_gui_preview","generation_gui_expanded","generation_preset","generation_answer","generation_value","generation_array_element","generation_object_member"],cases:[{name:"complete-owner",word:null,bytes:4136},{name:"camera-positive-zero",word:"0000000000000000",bytes:4136},{name:"camera-negative-zero",word:"8000000000000000",bytes:4136},{name:"camera-minimum-subnormal",word:"0000000000000001",bytes:4136},{name:"camera-minimum-normal",word:"0010000000000000",bytes:4136},{name:"camera-maximum-finite",word:"7fefffffffffffff",bytes:4136},{name:"camera-positive-infinity",word:"7ff0000000000000",bytes:4146},{name:"camera-negative-infinity",word:"fff0000000000000",bytes:4146},{name:"camera-quiet-nan",word:"7ff8000000000001",bytes:4125},{name:"camera-signaling-nan",word:"7ff0000000000001",bytes:4125}]};
for(const file of ["generation-independent-semantic-cell-extents.json","generation2d-independent-semantic-cell-extents.json"]){const observed=JSON.parse(readFileSync(join(import.meta.dir,"../../🗑️generated",file),"utf8"));assert.equal(observed.length,fixture.cases.length);for(let i=0;i<observed.length;i++){assert.deepEqual({name:observed[i].name,word:observed[i].word,bytes:observed[i].bytes},fixture.cases[i]);assert.equal(observed[i].rows,fixture.rows);assert.deepEqual(observed[i].tables.map((t:{name:string})=>t.name),fixture.tables)}}
const pairs:{path:string,before:string|null,after:string}[]=[];
const fresh=(path:string,after:string)=>{assert(!existsSync(path),path);pairs.push({path,before:null,after})};
fresh(join(root,"🫀️core/🧬️generation/🪶️sqlite/🧫️fixtures/📏️semantic-cells.json"),JSON.stringify(fixture,null,2)+"\n");
fresh(join(root,"🫀️core/🧬️generation/🪶️sqlite/🧫️fixtures/📏️semantic-cells/🧬️schema/🔣️.json"),JSON.stringify({$schema:"https://json-schema.org/draft/2020-12/schema",const:fixture},null,2)+"\n");
for(const dimension of ["2d","3d"]){
 const base=join(root,dimension==="2d"?"🗿️artifacts/🌀️generation2d":"🗿️artifacts/🧊️generation3d","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
 const sourcePath=join(base,"🧪️tests/🪶️sqlite/🟦️.ts"),sourceBefore=readFileSync(sourcePath,"utf8");assert(!sourceBefore.includes("complete SQL semantic cells match both independently owned native families"));
 const sourceDemand=String.raw`

import semanticCells from "../../../../../../../../../../🫀️core/🧬️generation/🪶️sqlite/🧫️fixtures/📏️semantic-cells.json";
test("complete SQL semantic cells match both independently owned native families",async()=>{
 for(const c of semanticCells.cases){
  const s=proceduralSnapshotFixture(corpus);if(c.word!==null)s.hostSnapshot.camera.x={bits:BigInt("0x"+c.word)};
  const d=await generationDIMENSIONSnapshotToSqliteDatabase(s),oracle=Database.deserialize(await exportSqliteDatabase(d));
  try{expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(d.tables.map(t=>t.name)).toEqual(semanticCells.tables);let bytes=0,rows=0;for(const t of d.tables){const actual=oracle.query<Record<string,null|string|number|bigint|Uint8Array>,[]>('SELECT * FROM "'+t.name+'"').all();rows+=actual.length;for(const row of actual)for(const value of Object.values(row))bytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):value instanceof Uint8Array?value.length:8;}expect(rows).toBe(semanticCells.rows);expect(bytes).toBe(c.bytes);
   expect(await generationDIMENSIONSnapshotToSqliteDatabase(s,{maxValueBytes:c.bytes})).toEqual(d);
   expect(await generationDIMENSIONSnapshotFromSqliteDatabase(d,{maxValueBytes:c.bytes})).toEqual(s);
   await expect(generationDIMENSIONSnapshotToSqliteDatabase(s,{maxValueBytes:c.bytes-1})).rejects.toThrow();
   await expect(generationDIMENSIONSnapshotFromSqliteDatabase(d,{maxValueBytes:c.bytes-1})).rejects.toThrow();
  }finally{oracle.close()}
 }
});
`.replaceAll("DIMENSION",dimension);
 pairs.push({path:sourcePath,before:sourceBefore,after:sourceBefore+sourceDemand});
 const nativePath=join(base,"🧪️tests/🪶️sqlite/🦀️.rs"),nativeBefore=readFileSync(nativePath,"utf8");assert(!nativeBefore.includes("complete_semantic_cell_extent_matches_independent_sqlite"));
 const nativeDemand=String.raw`

#[test]
fn sqlite_snapshot_generation_complete_semantic_cell_extent_matches_independent_sqlite() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../\u{1fac0}\u{fe0f}core/\u{1f9ec}\u{fe0f}generation/\u{1fab6}\u{fe0f}sqlite/\u{1f9eb}\u{fe0f}fixtures/\u{1f4cf}\u{fe0f}semantic-cells.json")).unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let mut source = fixture();
        if let Some(word) = case["word"].as_str() { source.host_snapshot.camera.x = f64::from_bits(u64::from_str_radix(word,16).unwrap()); }
        let expected = project(&source);
        let maximum = case["bytes"].as_u64().unwrap() as usize;
        assert_eq!(expected.tables.len(), cases["tables"].as_array().unwrap().len());
        assert_eq!(expected.tables.iter().map(|table| table.rows.len()).sum::<usize>(), cases["rows"].as_u64().unwrap() as usize);
        let file = export_sqlite_database(&expected, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let oracle = r#"import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));const names=JSON.parse(process.env.SEMIO_GENERATION_EXPECTED_TABLES);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const actual=d.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(r=>r.name);if(JSON.stringify(actual)!==JSON.stringify([...names].sort()))throw Error('complete table names');let bytes=0,rows=0;for(const name of names){const values=d.query('SELECT * FROM "'+name+'"').all();rows+=values.length;for(const row of values)for(const value of Object.values(row))bytes+=value===null?0:typeof value==='string'?Buffer.byteLength(value,'utf8'):value instanceof Uint8Array?value.length:8;}if(bytes!==Number(process.env.SEMIO_GENERATION_EXPECTED_BYTES)||rows!==Number(process.env.SEMIO_GENERATION_EXPECTED_ROWS))throw Error('complete independent SQL cell extent '+bytes+'/'+rows);d.close();"#;
        let mut child=Command::new("bun").args(["-e",oracle]).env("SEMIO_GENERATION_EXPECTED_TABLES",cases["tables"].to_string()).env("SEMIO_GENERATION_EXPECTED_BYTES",maximum.to_string()).env("SEMIO_GENERATION_EXPECTED_ROWS",cases["rows"].to_string()).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&file).unwrap();let observed=child.wait_with_output().unwrap();assert!(observed.status.success(),"{}",String::from_utf8_lossy(&observed.stderr));
        let limits=SqliteDatabaseLimits{max_value_bytes:maximum,..SqliteDatabaseLimits::default()};
        assert_eq!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),expected);
        let restored=GenerationDIMENSIONSnapshotRead::new(GenerationDIMENSIONSnapshot::from_sqlite_database(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap());assert_eq!(project(&restored),expected);
        let short=SqliteDatabaseLimits{max_value_bytes:maximum-1,..SqliteDatabaseLimits::default()};
        assert!(source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());
        match GenerationDIMENSIONSnapshot::from_sqlite_database(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,short)){Err(_)=>{},Ok(owner)=>{let _held=GenerationDIMENSIONSnapshotRead::new(owner);panic!("short semantic reconstruction accepted");}}
        for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text] {
            let payload=match encoding{SnapshotEncoding::Binary=>store::os_io::IoPayload::Binary(<GenerationDIMENSIONSnapshot as store::ArtifactPack>::encode_pack(&source)),SnapshotEncoding::Text=>store::os_io::IoPayload::Text(<GenerationDIMENSIONSnapshot as store::ArtifactDsl>::print_dsl(&source))};
            assert_eq!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),payload);
            let actual=GenerationDIMENSIONSnapshotRead::new(GenerationDIMENSIONSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap());assert_eq!(project(&actual),expected);
            assert!(source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"{} short semantic native encode",case["name"]);
            match GenerationDIMENSIONSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,short)){Err(_)=>{},Ok(owner)=>{let _held=GenerationDIMENSIONSnapshotRead::new(owner);panic!("short semantic native decode accepted");}}
        }
        eprintln!("[DEBUG] GenerationDIMENSION complete independent SQL cell semantic boundary case={} bytes={} rows=122 tables=36 both_native=true",case["name"],maximum);
    }
}
`.replaceAll("DIMENSION",dimension);
 pairs.push({path:nativePath,before:nativeBefore,after:nativeBefore+nativeDemand});
}
writeFileSync(join(import.meta.dir,"held-pairs.json"),JSON.stringify(pairs,null,2)+"\n");
console.log("[DEBUG] Generation neutral complete semantic demand held paths="+pairs.length+" production_mutations=0");
