from pathlib import Path
import json
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any')
tests=root/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests'
(tests/'binary-cases.json').write_text(json.dumps({'canonical':['0','0F','17','23','31','00ABCDEF'],'invalid':['','4','1','2','3','18','24','32','0f','0G','"0F"']},indent=2)+'\n',encoding='utf-8')
(tests/'binary.feature').write_text('''Feature: Exact IFC binary scalar persistence
  Scenario Outline: Binary content survives the typed native and SQLite graph
    Given an IFC binary scalar with canonical STEP content <content>
    When the snapshot is exported to SQLite and independently queried
    Then its kind is binary and its literal content is <content>
    And reconstruction preserves binary scalars in headers, aggregates, typed values and complex members
    Examples:
      | content  |
      | 0        |
      | 0F       |
      | 17       |
      | 23       |
      | 31       |
      | 00ABCDEF |
  Scenario: Invalid unused-bit padding is refused
    Given binary content with a first hexadecimal digit that uses a left padding bit
    When the snapshot is projected or reconstructed
    Then the operation is rejected
''',encoding='utf-8')
p=tests/'🟦️.ts'
s=p.read_text(encoding='utf-8');s+='''

import binaryCases from "./binary-cases.json";
test("IFC binary scalars survive independently queried native SQL graphs",async()=>{
 const input=fixture(),values:IfcValue[]=binaryCases.canonical.map(value=>({kind:"binary",value}));input.header.fileName=values;input.entities[0]!.args=[{kind:"aggregate",value:values},{kind:"typedValue",value:{name:"IFCBINARY",items:values}}];input.entities[0]!.complex=[{name:"BINARY_EXTENSION",args:values}];
 const physical=Database.deserialize(await exportSqliteDatabase(await ifcSnapshotToSqliteDatabase(input)));
 try{expect(physical.query("SELECT kind,string_value FROM ifc_value WHERE kind='binary' ORDER BY id LIMIT ?").all(binaryCases.canonical.length)).toEqual(binaryCases.canonical.map(string_value=>({kind:"binary",string_value})));expect(await ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).toEqual(input);physical.exec("PRAGMA ignore_check_constraints=ON");physical.exec("UPDATE ifc_value SET string_value='18' WHERE kind='binary'");await expect(ifcSnapshotFromSqliteDatabase(await importSqliteDatabase(physical.serialize()))).rejects.toThrow();}finally{physical.close();}
 for(const value of binaryCases.invalid){const invalid=fixture();invalid.entities[0]!.args=[{kind:"binary",value}];await expect(ifcSnapshotToSqliteDatabase(invalid)).rejects.toThrow();}
});
''';p.write_text(s,encoding='utf-8')
p=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🧪️tests/🔬️unit/🦀️.rs');s=p.read_text(encoding='utf-8');s+='''

#[test]
fn binary_literals_preserve_padding_and_refuse_malformed_content() {
    for content in ["0", "0F", "17", "23", "31", "00ABCDEF"] {
        let source = format!("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCBLOB(\\\"{content}\\\");ENDSEC;END-ISO-10303-21;");
        let document = parse_part21(&source).unwrap();
        assert_eq!(document.instances[0].entities[0].1[0], Part21Value::Binary(content.into()));
        let printed = write_part21(&document);
        assert_eq!(parse_part21(&printed).unwrap(), document);
    }
    for content in ["", "4", "1", "18", "24", "32", "0G"] {
        let source = format!("ISO-10303-21;HEADER;ENDSEC;DATA;#1=IFCBLOB(\\\"{content}\\\");ENDSEC;END-ISO-10303-21;");
        assert!(parse_part21(&source).is_err(), "{content}");
    }
}
''';p.write_text(s,encoding='utf-8')
