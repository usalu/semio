import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1997")
p = A.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = rd(p)
s = s.replace("use semio_s_artifact_norm_en1997_model::{Placement, AnchorTest,", "use semio_s_artifact_norm_en1997_model::{CaseRow, Placement, AnchorTest,", 1)
mods = "{insert_case, remove_case, set_case_number, set_case_text, change_case_label_en, change_case_label_de, change_case_entry_id}"
anchor = "use semio_s_artifact_norm_en1997_model::{CaseRow,"
assert anchor in s
s = s.replace(anchor, "use semio_s_artifact_norm_en1997_mut_design_cases::%s;\n" % mods + anchor, 1)
variants = """    InsertCase {
        index: usize,
        #[dsl(block)]
        case: CaseRow,
    },
    RemoveCase { index: usize },
    SetCaseNumber { case_id: String, key: String, value: Option<f64> },
    SetCaseText { case_id: String, key: String, value: Option<String> },
    ChangeCaseLabelEn { case_id: String, new_value: String },
    ChangeCaseLabelDe { case_id: String, new_value: String },
    ChangeCaseEntryId { case_id: String, new_value: String },
"""
a = "    RemovePlacement { index: usize },\n}"
assert s.count(a) == 1
s = s.replace(a, "    RemovePlacement { index: usize },\n" + variants + "}", 1)
a = "        En1997Mutation::RemovePlacement(payload) => En1997MutationDsl::RemovePlacement { index: payload.index },\n"
assert s.count(a) == 1
s = s.replace(a, a + """        En1997Mutation::InsertCase(payload) => En1997MutationDsl::InsertCase { index: payload.index, case: payload.case.clone() },
        En1997Mutation::RemoveCase(payload) => En1997MutationDsl::RemoveCase { index: payload.index },
        En1997Mutation::SetCaseNumber(payload) => En1997MutationDsl::SetCaseNumber { case_id: payload.case_id.clone(), key: payload.key.clone(), value: payload.value },
        En1997Mutation::SetCaseText(payload) => En1997MutationDsl::SetCaseText { case_id: payload.case_id.clone(), key: payload.key.clone(), value: payload.value.clone() },
        En1997Mutation::ChangeCaseLabelEn(payload) => En1997MutationDsl::ChangeCaseLabelEn { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
        En1997Mutation::ChangeCaseLabelDe(payload) => En1997MutationDsl::ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
        En1997Mutation::ChangeCaseEntryId(payload) => En1997MutationDsl::ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
""", 1)
a = "        En1997MutationDsl::RemovePlacement { index } => En1997Mutation::RemovePlacement(remove_placement::RemovePlacement { index }),\n"
assert s.count(a) == 1
s = s.replace(a, a + """        En1997MutationDsl::InsertCase { index, case } => En1997Mutation::InsertCase(insert_case::InsertCase { index, case }),
        En1997MutationDsl::RemoveCase { index } => En1997Mutation::RemoveCase(remove_case::RemoveCase { index }),
        En1997MutationDsl::SetCaseNumber { case_id, key, value } => En1997Mutation::SetCaseNumber(set_case_number::SetCaseNumber { case_id, key, value }),
        En1997MutationDsl::SetCaseText { case_id, key, value } => En1997Mutation::SetCaseText(set_case_text::SetCaseText { case_id, key, value }),
        En1997MutationDsl::ChangeCaseLabelEn { case_id, new_value } => En1997Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id, new_value }),
        En1997MutationDsl::ChangeCaseLabelDe { case_id, new_value } => En1997Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id, new_value }),
        En1997MutationDsl::ChangeCaseEntryId { case_id, new_value } => En1997Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id, new_value }),
""", 1)
wr(p, s)
print("ok")
