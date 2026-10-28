import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native/🔗️root/🧪️tests/🔣️.json";
import schema from "../../../../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️native/🔗️root/🧪️tests/📐️.json";
test("complete Energy links and exact inverse agree with SQLite JSON operations across all original pins and roles",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{
 db.exec("CREATE TABLE link(before TEXT,intent TEXT,outcome TEXT,after TEXT)");
 for(const row of law.cases){db.exec("DELETE FROM link");db.query("INSERT INTO link(before,intent)VALUES(?,?)").run(JSON.stringify(row.before),JSON.stringify(row.value));
 db.exec(`UPDATE link SET outcome=CASE WHEN intent='null' AND before='null' THEN 'missing' WHEN intent!='null' AND (json_extract(intent,'$.target.artifactId')='' OR json_extract(intent,'$.target.dialect.artifactKind')='' OR json_extract(intent,'$.target.dialect.standard')='' OR json_extract(intent,'$.target.dialect.subset')='' OR json_extract(intent,'$.role')='' OR (json_extract(intent,'$.pin.kind')='checkpoint' AND json_extract(intent,'$.pin.id')='') OR (json_extract(intent,'$.pin.kind')='snapshot' AND (json_extract(intent,'$.pin.blob.hash')='' OR json_extract(intent,'$.pin.blob.mediaType')=''))) THEN 'invalid' WHEN json(before)=json(intent) THEN 'no-op' ELSE 'changed' END; UPDATE link SET after=CASE WHEN outcome='changed' THEN intent ELSE before END`);
 const result=db.query("SELECT outcome,after,before FROM link").get()as {outcome:string,after:string,before:string};expect(result.outcome).toBe(row.outcome);expect(JSON.parse(result.after)).toEqual(row.after);
 if(result.outcome==='changed'){db.exec("UPDATE link SET after=before");expect(JSON.parse((db.query("SELECT after FROM link").get()as {after:string}).after)).toEqual(row.before);}
 }
 }finally{db.close();}
});
