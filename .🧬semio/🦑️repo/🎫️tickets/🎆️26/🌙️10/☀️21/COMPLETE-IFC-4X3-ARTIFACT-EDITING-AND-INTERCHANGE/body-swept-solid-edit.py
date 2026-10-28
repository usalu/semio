"""🌊️ Extract one shared body concept executor and mount the released SweptSolid definition."""
from pathlib import Path

concepts = Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts")
tess = concepts / "🔺️tessellation"
body = concepts / "🧱️body"
swept = concepts / "🌊️swept-solid"
body.mkdir(exist_ok=True)
source = (tess / "🟦️.ts").read_text(encoding="utf-8")

def replace(before, after):
    """🧷️ Require the exact retained body implementation at each extraction point."""
    global source
    assert source.count(before) == 1, before[:160]
    source = source.replace(before, after)

replace("/** 🔺️ Execute only the released Body Tessellation predicates under explicit caller applicability. */", "/** 🧱️ Share controlled selected body representation clauses without inferring view applicability. */")
replace("import definition from './definition.json';\n", "")
source = source.replace("IfcBodyTessellationRequest", "IfcBodyConceptRequest").replace("IfcBodyTessellationReport", "IfcBodyConceptReport")
replace("import type {IfcBodyConceptRequest,IfcBodyConceptReport,", "import type {IfcBodyConceptDefinition,IfcBodyConceptRequest,IfcBodyConceptReport,")
replace("export type * from './types.ts';\n", "")
start = source.index("export const ifcBodyTessellationDefinition=")
end = source.index("\n", start)
source = source[:start]+source[end+1:]
replace("function diagnostic(code:IfcConceptDiagnosticCode,path:readonly string[]=[],pointer?:string):IfcConceptDiagnostic{const [en,de]=labels[code];", "function diagnostic(code:IfcConceptDiagnosticCode,path:readonly string[]=[],pointer?:string,itemLabel?:readonly [string,string]):IfcConceptDiagnostic{const [en,de]=code==='mvd.items.type'&&itemLabel?itemLabel:labels[code];")
replace("function report(targets:", "function report(definition:IfcBodyConceptDefinition,targets:")
replace("concept:ifcBodyTessellationDefinition.id,source:ifcBodyTessellationDefinition.source", "concept:definition.id,source:definition.source")
replace("function* evaluate(input:IfcBodyConceptRequest,index:ReadonlyMap<bigint,Indexed|null>)", "function* evaluate(definition:IfcBodyConceptDefinition,input:IfcBodyConceptRequest,index:ReadonlyMap<bigint,Indexed|null>,itemLabel:readonly [string,string])")
replace("for(const predicate of ifcBodyTessellationDefinition.predicates)", "for(const predicate of definition.predicates)")
replace("const accepted=yield* membership(resolved.entity,predicate.value);", """let matched=false,excluded=false,unresolved=false;
    for(const name of typeof predicate.value==='string'?[predicate.value]:predicate.value){const value=yield* membership(resolved.entity,name);matched ||= value===true;unresolved ||= value===undefined;}
    for(const name of predicate.exclude??[]){const value=yield* membership(resolved.entity,name);excluded ||= value===true;unresolved ||= value===undefined;}
    const accepted=unresolved?undefined:matched&&!excluded;""")
replace("diagnostic(accepted===undefined?'mvd.items.unknown':'mvd.items.type',itemPath,pointer)", "diagnostic(accepted===undefined?'mvd.items.unknown':'mvd.items.type',itemPath,pointer,itemLabel)")
replace("export async function validateIfcBodyTessellation(source:IfcSnapshot,request:IfcBodyConceptRequest,control:IfcIoControl={}):Promise<IfcBodyConceptReport>", "export async function validateIfcBodyConcept(source:IfcSnapshot,request:IfcBodyConceptRequest,control:IfcIoControl,definition:IfcBodyConceptDefinition,itemLabel:readonly [string,string]):Promise<IfcBodyConceptReport>")
replace("return report(await run(unavailable(input)", "return report(definition,await run(unavailable(input)")
replace("run(evaluate(input,index),control", "run(evaluate(definition,input,index,itemLabel),control")
replace("return report(result.rows,result.status);", "return report(definition,result.rows,result.status);")
source += """
/** 🧊 Freeze repository-authored static definitions before exposing their source and family policy. */
export function freezeIfcBodyDefinition(input:IfcBodyConceptDefinition):IfcBodyConceptDefinition{
 return Object.freeze({...input,source:Object.freeze({...input.source}),predicates:Object.freeze(input.predicates.map(row=>Object.freeze({...row,...(typeof row.value==='string'?{}:{value:Object.freeze(row.value)}),...(row.exclude?{exclude:Object.freeze(row.exclude)}:{})})))});
}
"""
(body / "🟦️.ts").write_text(source, encoding="utf-8")
types = (tess / "types.ts").read_text(encoding="utf-8").replace("IfcBodyTessellationRequest", "IfcBodyConceptRequest").replace("IfcBodyTessellationReport", "IfcBodyConceptReport")
types += """
/** 📜 Repository-authored released body clauses retain their exact inclusion and exclusion families. */
export interface IfcBodyConceptDefinition {readonly id:string;readonly source:IfcConceptSource;readonly predicates:readonly Readonly<{attribute:string;kind:'equals-string'|'every-subtype'|'every-subtype-except';value:string|readonly string[];exclude?:readonly string[]}>[]}
"""
(body / "types.ts").write_text(types, encoding="utf-8")
(tess / "types.ts").write_text("""/** 🔺️ Named Body Tessellation input and output share the selected body contract. */
import type {IfcBodyConceptRequest,IfcBodyConceptReport} from '../🧱️body/types.ts';
export type {IfcConceptStatus,IfcConceptSource,IfcConceptTarget,IfcConceptTargetReport,IfcConceptDiagnostic,IfcConceptDiagnosticCode} from '../🧱️body/types.ts';
export type IfcBodyTessellationRequest=IfcBodyConceptRequest;
export type IfcBodyTessellationReport=IfcBodyConceptReport;
""", encoding="utf-8")
for folder, name, item_label in [(tess,"Tessellation",["A representation item is not a tessellated face set.","Ein Repräsentationselement ist kein tessellierter Flächensatz."]),(swept,"SweptSolid",["A representation item is not a basic extruded or revolved area solid.","Ein Repräsentationselement ist kein einfacher extrudierter oder gedrehter Flächenkörper."])]:
    if name == "SweptSolid":
        (folder / "types.ts").write_text("""/** 🌊️ Explicit Body SweptSolid reports retain separate schema and view boundaries. */
import type {IfcBodyConceptRequest,IfcBodyConceptReport} from '../🧱️body/types.ts';
export type {IfcConceptTarget,IfcConceptTargetReport,IfcConceptDiagnostic,IfcConceptDiagnosticCode} from '../🧱️body/types.ts';
export type IfcBodySweptSolidRequest=IfcBodyConceptRequest;
export type IfcBodySweptSolidReport=IfcBodyConceptReport;
""", encoding="utf-8")
    label_text = __import__("json").dumps(item_label, ensure_ascii=False)
    (folder / "🟦️.ts").write_text(f"""/** 🧭️ Execute the released selected Body {name} clauses under explicit caller applicability. */
import definition from './definition.json';
import {{freezeIfcBodyDefinition,validateIfcBodyConcept}} from '../🧱️body/🟦️.ts';
import type {{IfcBodyConceptDefinition}} from '../🧱️body/types.ts';
import type {{IfcBody{name}Request,IfcBody{name}Report}} from './types.ts';
import type {{IfcSnapshot}} from '../../../📸️snapshot/🟦️.ts';
import type {{IfcIoControl}} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBody{name}Definition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
/** 🧾 Report only the selected released representation clauses against privately captured source. */
export async function validateIfcBody{name}(source:IfcSnapshot,request:IfcBody{name}Request,control:IfcIoControl={{}}):Promise<IfcBody{name}Report>{{return validateIfcBodyConcept(source,request,control,ifcBody{name}Definition,{label_text});}}
""", encoding="utf-8")
print("[DEBUG] Shared body executor and both named concept entries authored")
