import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1990")
B = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
p = B + "/🟦️.ts"
sub(p, "En1990Placement as Placement,", "En1990Placement as Placement,CaseRow,CaseNumber,CaseText,")
sub(p, "/** ⚖️ The complete persisted EN1990 state", 'export type En1990SqliteCaseNumber=Omit<CaseNumber,"value">&{value:Binary64};\nexport type En1990SqliteCase=Omit<CaseRow,"numbers">&{numbers:En1990SqliteCaseNumber[]};\n/** ⚖️ The complete persisted EN1990 state')
sub(p, '|"placements">&{altitudeM', '|"placements"|"cases">&{altitudeM')
sub(p, "placements:En1990SqlitePlacement[]};", "placements:En1990SqlitePlacement[];cases:En1990SqliteCase[]};")
sub(p, "const PLACEMENT_FLOATS:", "const CASE_NUMBER_FLOATS:readonly Ieee754Column[]=[{index:4,width:64}];\nconst PLACEMENT_FLOATS:")
sub(p, " return out.finish();\n}", ''' let numberId=1n,textId=1n;
 for(let position=0;position<snapshot.cases.length;position++){const k=snapshot.cases[position]!;const id=BigInt(position+1);await write(out,"en1990_case",id,[1n,BigInt(position),k.id,k.labelEn,k.labelDe,k.entryId],[],options);
  for(let index=0;index<k.numbers.length;index++){const n=k.numbers[index]!;await write(out,"en1990_case_number",numberId++,[id,BigInt(index),n.id,n.value],CASE_NUMBER_FLOATS,options)}
  for(let index=0;index<k.texts.length;index++){const t=k.texts[index]!;await write(out,"en1990_case_text",textId++,[id,BigInt(index),t.id,t.value],[],options)}}
 return out.finish();
}''')
sub(p, "const placementRows=await entities(tables[12]!,8,PLACEMENT_FLOATS,options);", "const placementRows=await entities(tables[12]!,8,PLACEMENT_FLOATS,options);const caseRows=await entities(tables[13]!,7,[],options);const caseNumberRows=await entities(tables[14]!,7,CASE_NUMBER_FLOATS,options);const caseTextRows=await entities(tables[15]!,5,[],options);")
sub(p, " const annex=artifactSqliteText(document,1);", ''' const caseNumbers=new Map<bigint,SqliteRow[]>(),caseTexts=new Map<bigint,SqliteRow[]>();
 for(const row of caseNumberRows.values()){const parent=artifactSqliteInteger(row,1);if(!caseRows.has(parent))throw Error("EN1990 case number has an unknown case");caseNumbers.set(parent,[...(caseNumbers.get(parent)??[]),row])}
 for(const row of caseTextRows.values()){const parent=artifactSqliteInteger(row,1);if(!caseRows.has(parent))throw Error("EN1990 case text has an unknown case");caseTexts.set(parent,[...(caseTexts.get(parent)??[]),row])}
 const cases:En1990SqliteCase[]=[];for(const row of await ordered([...caseRows.values()],options)){await checkpoint(options,cases.length,0);const numbers:En1990SqliteCaseNumber[]=[];for(const n of await artifactSqliteOrderedRowsControlled(caseNumbers.get(row.rowid)??[],2,options))numbers.push({id:artifactSqliteText(n,3),value:readBinary64(n,4,CASE_NUMBER_FLOATS)});const texts:CaseText[]=[];for(const t of await artifactSqliteOrderedRowsControlled(caseTexts.get(row.rowid)??[],2,options))texts.push({id:artifactSqliteText(t,3),value:artifactSqliteText(t,4)});cases.push({id:artifactSqliteText(row,3),labelEn:artifactSqliteText(row,4),labelDe:artifactSqliteText(row,5),entryId:artifactSqliteText(row,6),numbers,texts})}
 const annex=artifactSqliteText(document,1);''')
sub(p, "testSeries,placements};\n await artifactSqliteCheckpoint(options,\"reconstructSnapshot\",1,1)", "testSeries,placements,cases};\n await artifactSqliteCheckpoint(options,\"reconstructSnapshot\",1,1)")
t = B + "/🧪️tests/🟦️.ts"
s = rd(t)
s = s.replace("maxTables:13,maxColumns:57", "maxTables:16,maxColumns:76").replace("{...limits,maxTables:11},{...limits,maxColumns:56}", "{...limits,maxTables:14},{...limits,maxColumns:75}")
wr(t, s)
print(len(LOG))
