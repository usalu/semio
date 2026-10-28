/** 🔬 Independently tokenize the complete native template literal and compare canonical data. */
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import assert from "node:assert/strict";
const base=resolve(import.meta.dir,"../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates");
function literal(source:string):any{
 const tokens=source.match(/"(?:\\.|[^"\\])*"|[A-Za-z_][A-Za-z_0-9]*|::|[^\s]/g)!;let at=0;
 const take=()=>tokens[at++]!,expect=(value:string)=>assert.equal(take(),value);
 function value():any{
  const token=take();if(token==="&")return value();
  if(token==="["){const rows=[];while(tokens[at]!=="]"){rows.push(value());if(tokens[at]===",")take();else break;}expect("]");return rows;}
  if(token==="Some"){expect("(");const result=value();expect(")");return result;}if(token==="None")return null;
  if(token.startsWith('"'))return JSON.parse(token);
  if(tokens[at]==="::"){take();return take();}
  expect("{");const result:any={$type:token};while(tokens[at]!=="}"){const name=take();expect(":");result[name]=value();if(tokens[at]===",")take();else break;}expect("}");return result;
 }
 const result=value();assert.equal(at,tokens.length);return result;
}
const canonical=JSON.parse(await readFile(resolve(base,"🔣️.json"),"utf8")),rust=await readFile(resolve(base,"data.rs"),"utf8");
const marker="= IfcTemplateCatalog",start=rust.indexOf(marker);assert.ok(start>=0);
const actual=literal(rust.slice(start+2).trim().replace(/;$/,""));
const kinds:Record<string,string>={P_SINGLEVALUE:"SingleValue",P_REFERENCEVALUE:"ReferenceValue",P_ENUMERATEDVALUE:"EnumeratedValue",P_LISTVALUE:"ListValue",P_BOUNDEDVALUE:"BoundedValue",P_TABLEVALUE:"TableValue",Q_WEIGHT:"Weight",Q_LENGTH:"Length",Q_AREA:"Area",Q_VOLUME:"Volume",Q_COUNT:"Count",Q_TIME:"Time"};
const expected={$type:"IfcTemplateCatalog",schema:canonical.schema,source:{$type:"IfcTemplateSource",...canonical.source},templates:canonical.templates.map((template:any)=>({$type:"IfcTemplate",name:template.name,definition:template.definition,kind:template.kind,applicable:template.applicable,classes:template.classes,source_file:template.sourceFile,source_sha256:template.sourceSha256,entries:template.entries.map((entry:any)=>({$type:"IfcTemplateEntry",name:entry.name,definition:entry.definition,kind:kinds[entry.kind],data_type:entry.dataType,enumeration:entry.enumeration?{$type:"IfcTemplateEnumeration",name:entry.enumeration.name,literals:entry.enumeration.literals}:null,secondary_type:entry.secondaryType??null,expression:entry.expression??null}))}))};
assert.deepEqual(actual,expected);
console.log(JSON.stringify({nativeTemplatesExact:actual.templates.length,nativeEntriesExact:actual.templates.reduce((count:number,template:any)=>count+template.entries.length,0)}));
