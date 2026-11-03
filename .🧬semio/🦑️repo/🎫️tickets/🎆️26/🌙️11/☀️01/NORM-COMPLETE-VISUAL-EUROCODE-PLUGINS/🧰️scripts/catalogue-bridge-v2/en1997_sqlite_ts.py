import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1997")
p = A.any + "/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"
sub(p, "LayerWater,Placement}from", "LayerWater,Placement,CaseRow,CaseNumber,CaseText}from")
sub(p, "export type En1997SqliteSnapshot={", 'export type En1997SqliteCaseNumber=Omit<CaseNumber,"value">&{value:Binary64};\nexport type En1997SqliteCase=Omit<CaseRow,"numbers">&{numbers:En1997SqliteCaseNumber[]};\nexport type En1997SqliteSnapshot={')
sub(p, "placements:En1997SqlitePlacement[]};", "placements:En1997SqlitePlacement[];cases:En1997SqliteCase[]};")
sub(p, '"layerWaters","placements"]);for(const v of[s.layers,', '"layerWaters","placements","cases"]);for(const v of[s.layers,')
sub(p, "s.layerWaters,s.placements])collection(v);", "s.layerWaters,s.placements,s.cases])collection(v);")
w = ' for(let n=0;n<s.placements.length;n++){const v=s.placements[n]!;fields(v,["id","x","y","z","yaw"]);await write(out,"en1997_placement",[1n,BigInt(n),v.id,v.x,v.y,v.z,v.yaw],PLACEMENT,o)}\n'
sub(p, w, w + ' for(let n=0;n<s.cases.length;n++){const v=s.cases[n]!;fields(v,["id","labelEn","labelDe","entryId","numbers","texts"]);collection(v.numbers);collection(v.texts);const id=await write(out,"en1997_case",[1n,BigInt(n),v.id,v.labelEn,v.labelDe,v.entryId],[],o);for(let k=0;k<v.numbers.length;k++){const a=v.numbers[k]!;fields(a,["id","value"]);await write(out,"en1997_case_number",[id,BigInt(k),a.id,a.value],CASENUMBER,o)}for(let k=0;k<v.texts.length;k++){const a=v.texts[k]!;fields(a,["id","value"]);await write(out,"en1997_case_text",[id,BigInt(k),a.id,a.value],[],o)}}\n')
sub(p, "const DOCUMENT:readonly Ieee754Column[]", "const CASENUMBER:readonly Ieee754Column[]=[4].map(index=>({index,width:64}));\nconst DOCUMENT:readonly Ieee754Column[]")
sub(p, "placements:En1997SqlitePlacement[]=[];", "placements:En1997SqlitePlacement[]=[],cases:En1997SqliteCase[]=[];")
r = " await artifactSqliteCheckpoint(o,\"reconstructSnapshot\",1,1);return{"
rd_ = ' const caseRows=await entities(t[17]!,7,[],o),caseNumbers=await groups(await entities(t[18]!,5,CASENUMBER,o),caseRows,o),caseTexts=await groups(await entities(t[19]!,5,[],o),caseRows,o);\n for(const v of await ordered(caseRows.values(),1n,o)){await checkpoint(o,cases.length);const numbers:En1997SqliteCaseNumber[]=[];for(const a of await ordered(caseNumbers.get(v.rowid)??[],v.rowid,o))numbers.push({id:artifactSqliteText(a,3),value:readBinary64(a,4,CASENUMBER)});const texts:CaseText[]=[];for(const a of await ordered(caseTexts.get(v.rowid)??[],v.rowid,o))texts.push({id:artifactSqliteText(a,3),value:artifactSqliteText(a,4)});cases.push({id:artifactSqliteText(v,3),labelEn:artifactSqliteText(v,4),labelDe:artifactSqliteText(v,5),entryId:artifactSqliteText(v,6),numbers,texts})}\n'
sub(p, r, rd_ + r)
sub(p, "layerWaters,placements};", "layerWaters,placements,cases};")
print(len(LOG))
