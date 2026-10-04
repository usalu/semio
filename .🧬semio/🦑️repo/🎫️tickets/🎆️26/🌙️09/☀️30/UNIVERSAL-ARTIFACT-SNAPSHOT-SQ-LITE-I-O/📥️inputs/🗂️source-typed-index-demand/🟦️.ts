import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {ArtifactSqliteProjection,artifactSqliteTables} from "../../🧩️artifact/🟦️.ts";
import {sqliteOperation} from "../../🟦️.ts";
const quote=(name:string)=>'"'+name.replaceAll('"','""')+'"';
const sql=fixture.tableNames.map(name=>`CREATE TABLE ${quote(name)} (id INTEGER PRIMARY KEY,payload TEXT)`).join(';');
test('closed checked index collision contract has independent Map and SQLite answers',()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:0})).toBe(false);
 const index=new Map(fixture.tableNames.map((name,index)=>[name,index]));for(let i=0;i<fixture.lookupNames.length;i++)expect(index.get(fixture.lookupNames[i]!)??null).toBe(fixture.lookupIndices[i]);
 const hash=(name:string)=>{let value=fixture.hash.offset;for(const code of name){const n=code.charCodeAt(0);value=Math.imul(value^(n>=65&&n<=90?n+32:n),fixture.hash.prime)>>>0;}return value&fixture.hash.mask;};for(const name of fixture.tableNames)expect(hash(name)).toBe(fixture.hash.collisionBucket);
 expect(new Uint32Array(fixture.capacity).byteLength).toBe(fixture.tableSlotBytes);
 const db=new Database(':memory:');try{db.exec(sql);for(let i=0;i<fixture.tableNames.length;i++)db.query(`INSERT INTO ${quote(fixture.tableNames[i]!)} VALUES (1,?)`).run(String(i));for(let i=0;i<fixture.tableNames.length;i++)expect(db.query(`SELECT payload FROM ${quote(fixture.tableNames[i]!)} WHERE id=1`).get()).toEqual({payload:String(i)});}finally{db.close();}
 for(const group of fixture.groups){const actual=fixture.rows.map((row,index)=>({row,index})).filter(v=>v.row.parent===group.parent).sort((a,b)=>a.row.ordinal-b.row.ordinal).map(v=>v.index);expect(actual).toEqual(group.positions);}
});
for(const direction of ['projection','reconstruction'] as const)test('actual supplied checked table slots exact short cumulative '+direction,async()=>{
 const plain=await ArtifactSqliteProjection.create(sql);for(let i=0;i<fixture.tableNames.length;i++)await plain.insert(fixture.tableNames[i]!,[String(i)]);const expected=await plain.finish();
 const run=async(operation:ReturnType<typeof sqliteOperation>)=>{if(direction==='reconstruction')return artifactSqliteTables({...expected,tables:[...expected.tables].reverse()},sql,operation);const projection=await ArtifactSqliteProjection.create(sql,operation);for(let i=0;i<fixture.tableNames.length;i++)await projection.insert(fixture.tableNames[i]!,[String(i)]);return projection.finish();};
 const probe=sqliteOperation();const result=await run(probe);expect(probe.ownedBytes).toBeGreaterThanOrEqual(fixture.tableSlotBytes);expect(result).toEqual(direction==='projection'?expected:expected.tables.map(table=>table.rows));
 const exact=sqliteOperation({maxAllocationBytes:probe.ownedBytes});expect(await run(exact)).toEqual(result);expect(exact.remainingBytes()).toBe(0);await expect(run(exact)).rejects.toMatchObject({kind:'ownershipLimit'});
 await expect(run(sqliteOperation({maxAllocationBytes:probe.ownedBytes-1}))).rejects.toMatchObject({kind:'ownershipLimit'});
 const twice=sqliteOperation({maxAllocationBytes:probe.ownedBytes*2});expect(await run(twice)).toEqual(result);expect(await run(twice)).toEqual(result);expect(twice.remainingBytes()).toBe(0);await expect(run(twice)).rejects.toMatchObject({kind:'ownershipLimit'});
});
