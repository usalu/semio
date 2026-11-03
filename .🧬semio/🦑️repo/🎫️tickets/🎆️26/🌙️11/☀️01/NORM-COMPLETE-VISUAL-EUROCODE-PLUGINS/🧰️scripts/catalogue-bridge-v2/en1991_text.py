import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1991")
p = A.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = rd(p)
a = "        En1991Mutation::ChangeStructureKind("
i = s.index(a)
j = s.index("\n", i)
D = "semio_s_artifact_norm_en1991_mut_design_cases"
case = "semio_s_artifact_norm_en1991_model::En1991Snapshot::default().cases[0].clone()"
add = f'''
        En1991Mutation::InsertCase({D}::insert_case::InsertCase {{ index: 0, case: {case} }}),
        En1991Mutation::RemoveCase({D}::remove_case::RemoveCase {{ index: 0 }}),
        En1991Mutation::SetCaseNumber({D}::set_case_number::SetCaseNumber {{ case_id: "case-e11-04".into(), key: "area".into(), value: Some(60.0) }}),
        En1991Mutation::SetCaseNumber({D}::set_case_number::SetCaseNumber {{ case_id: "case-e11-04".into(), key: "area".into(), value: None }}),
        En1991Mutation::SetCaseText({D}::set_case_text::SetCaseText {{ case_id: "case-e11-04".into(), key: "category".into(), value: Some("B".into()) }}),
        En1991Mutation::ChangeCaseLabelEn({D}::change_case_label_en::ChangeCaseLabelEn {{ case_id: "case-e11-04".into(), new_value: "Reduction".into() }}),
        En1991Mutation::ChangeCaseLabelDe({D}::change_case_label_de::ChangeCaseLabelDe {{ case_id: "case-e11-04".into(), new_value: "Abminderung".into() }}),
        En1991Mutation::ChangeCaseEntryId({D}::change_case_entry_id::ChangeCaseEntryId {{ case_id: "case-e11-04".into(), new_value: "E11.03".into() }}),'''
wr(p, s[:j] + add + s[j:])
print("ok")
