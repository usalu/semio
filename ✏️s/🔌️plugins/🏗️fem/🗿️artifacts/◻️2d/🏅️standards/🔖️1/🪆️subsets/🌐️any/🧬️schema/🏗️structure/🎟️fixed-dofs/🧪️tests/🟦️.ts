import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {parseFemFixedDofs as planar} from "../../../📸️snapshot/🟦️.ts";
import {parseFemFixedDofs as volumetric} from "../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🟦️.ts";
import fixture from "../../../../🧪️tests/🧫️fixtures/🎟️fixed-dofs/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** 🔒️ Every canonical restraint subset agrees across actual dimensions and independent SQLite. */
test("FEM fixed restraints admit all64 canonical sets and refuse malformed boundaries",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);expect(valid({...fixture,extra:true})).toBe(false);
 const db=new Database(":memory:");
 try{
  const oracle=db.query<{canonical:number},{$input:string}>(`WITH members AS (
   SELECT key AS ordinal,CASE value WHEN 'Tx' THEN 1 WHEN 'Ty' THEN 2 WHEN 'Tz' THEN 3
    WHEN 'Rx' THEN 4 WHEN 'Ry' THEN 5 WHEN 'Rz' THEN 6 ELSE 0 END AS rank
   FROM json_each($input)
  ),ordered AS (SELECT rank,lag(rank,1,0) OVER(ORDER BY ordinal) AS previous FROM members)
  SELECT json_type($input)='array' AND json_array_length($input)<=6 AND
   NOT EXISTS(SELECT 1 FROM ordered WHERE rank<=previous OR rank=0) AS canonical`);
  for(const row of fixture.cases){
   expect(Boolean(oracle.get({$input:JSON.stringify(row.input)})?.canonical)).toBe(row.canonical);
   for(const decode of [planar,volumetric]){
    if(row.canonical)expect<unknown>(decode(row.input)).toEqual(row.input);
    else expect(()=>decode(row.input)).toThrow();
   }
  }
 }finally{db.close();}
 console.log(`[DEBUG] Actual FEM2D/FEM3D restraint parsers agree with SQLite for ${fixture.cases.length} shared boundaries including all64canonical subsets`);
});

/** 🧭️ Guards helpers at actual defining children and rejects dissolved crate-root paths. */
test("FEM structure fields resolve their real shared finite canonical decoder",()=>{
 const planar=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),volumetric=readFileSync(new URL("../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🏗️structure/🦀️.rs",import.meta.url),"utf8");
 for(const name of ["decode_fixed_dofs","decode_fixed_dofs_controlled","retire_fixed_dofs"])expect(planar).toContain(`pub fn ${name}(`);
 expect(planar).toContain("validate_fixed_dofs(value,Some(&mut *control))?;");
 expect(planar.indexOf("validate_fixed_dofs(value,Some(&mut *control))?;")).toBeLessThan(planar.indexOf("Vec::<FemDof>::from_value_controlled(value,control)"));
 expect(planar).not.toContain("retire_decoded(fixed);Err");
 for(const source of [planar,volumetric]){
  expect(source).toContain('deserialize_with="self::decode_fixed_dofs"');
  expect(source).toContain('deserialize_controlled_with="self::decode_fixed_dofs_controlled"');
  expect(source).toContain('retire_with="self::retire_fixed_dofs"');
  expect(source).not.toContain('crate::decode_fixed_dofs');
 }
 expect(volumetric).toContain("{decode_fixed_dofs,decode_fixed_dofs_controlled,retire_fixed_dofs}");
});
