from pathlib import Path
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
I=A/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any'
B=A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base'
T=A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base'
for root,enum in [(I,'IfcValue'),(T,'StepValue'),(B,'Part21Value')]:
 p=root/'🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs';s=p.read_text(encoding='utf-8');needle='cells[0]=Cell::Text(match value';assert needle in s;s=s.replace(needle,f'if let {enum}::Binary(content)=value{{if !semio_s_artifact_stdio_contract::part21::is_part21_binary(content){{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid canonical STEP binary content"));}}}}'+needle);p.write_text(s,encoding='utf-8',newline='\n')
for root,tag,prefix in [(B,'{kind:"binary",value:content}','ifc2x3'),(T,'{binary:content}','step')]:
 S=root/'🚪️io/🪶️sqlite/📸️snapshot'
 body='''import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";
'''
 if root==B:
  body+='''import {ifc2x3SnapshotToSqliteDatabase as project,ifc2x3SnapshotFromSqliteDatabase as restore} from "../🟦️.ts";
import type {Ifc2x3Snapshot,Part21Value} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
function fixture(values:Part21Value[]):Ifc2x3Snapshot{return{schema:"stdio.ifc2x3",document:{header:{fileDescription:[],fileName:[],fileSchema:[]},instances:[{id:1n,entities:[{typeName:"BINARY_EXTENSION",arguments:[{kind:"list",values},{kind:"typed",typeName:"BINARY",values}]}]}]}};}
'''
 else:
  body+='''import {stepSnapshotToSqliteDatabase as project,stepSnapshotFromSqliteDatabase as restore} from "../🟦️.ts";
import type {StepSnapshot,StepValue} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
function fixture(values:StepValue[]):StepSnapshot{return{schema:"stdio.step",header:{fileDescription:{description:[""],implementationLevel:"2;1"},fileName:{name:"",timeStamp:"",author:[""],organization:[""],preprocessorVersion:"",originatingSystem:"",authorization:""},fileSchema:{schemas:["AUTOMOTIVE_DESIGN"]}},entities:[{id:1n,name:"BINARY_EXTENSION",args:values,complex:[]}]};}
'''
 body+='''test("FAMILY shared Part21 binary values remain exact in independent SQLite",async()=>{
 const contents=["0","0F","17","23","31","00ABCDEF"],input=fixture(contents.map(content=>VALUE));const db=Database.deserialize(await exportSqliteDatabase(await project(input)));
 try{expect(db.query("SELECT string_value FROM FAMILY_value WHERE kind='binary' ORDER BY id LIMIT 6").all()).toEqual(contents.map(string_value=>({string_value})));expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(input);db.exec("UPDATE FAMILY_value SET string_value='18' WHERE kind='binary'");await expect(restore(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}
 for(const content of["","4","1","18","24","32","0f","0G"]){await expect(project(fixture([VALUE]))).rejects.toThrow();}
});
'''.replace('FAMILY',prefix).replace('VALUE',tag)
 (S/'🧪️tests/binary.test.ts').write_text(body,encoding='utf-8',newline='\n')
