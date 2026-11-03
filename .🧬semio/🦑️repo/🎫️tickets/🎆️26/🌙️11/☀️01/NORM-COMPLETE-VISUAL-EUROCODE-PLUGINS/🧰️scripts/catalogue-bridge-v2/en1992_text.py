import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1992")
p = A.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = rd(p)
a = "use crate::En1992Mutation;\n"
assert s.count(a) == 1
s = s.replace(a, a + "use semio_s_artifact_norm_en1992_mut_design_cases::{insert_case, remove_case, change_case_number, change_case_text, change_case_label_en, change_case_label_de, change_case_entry_id};\n", 1)
a = "fn dec_json<T: semio_framework_value::FromValue>"
s = s.replace(a, '''fn enc_token<T: semio_framework_value::ToValue>(v: &T) -> String {
    enc_json(v).replace(' ', "\\\\u0020")
}
''' + a, 1)
a = '        En1992Mutation::RemovePlacement(p) => format!("remove-placement index={}", p.index),\n'
assert s.count(a) == 1
s = s.replace(a, a + '''        En1992Mutation::InsertCase(p) => format!("insert-case index={} case={}", p.index, enc_token(&p.case)),
        En1992Mutation::RemoveCase(p) => format!("remove-case index={}", p.index),
        En1992Mutation::ChangeCaseNumber(p) => format!("change-case-number case-id={} key={} value={}", enc_token(&p.case_id), enc_token(&p.key), enc_token(&p.value)),
        En1992Mutation::ChangeCaseText(p) => format!("change-case-text case-id={} key={} value={}", enc_token(&p.case_id), enc_token(&p.key), enc_token(&p.value)),
        En1992Mutation::ChangeCaseLabelEn(p) => format!("change-case-label-en case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
        En1992Mutation::ChangeCaseLabelDe(p) => format!("change-case-label-de case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
        En1992Mutation::ChangeCaseEntryId(p) => format!("change-case-entry-id case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
''', 1)
a = '        other => Err(format!("unknown mutation kind {other}")),'
assert s.count(a) == 1
s = s.replace(a, '''        "insert-case" => Ok(En1992Mutation::InsertCase(insert_case::InsertCase { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, case: dec_json(&get("case")?)? })),
        "remove-case" => Ok(En1992Mutation::RemoveCase(remove_case::RemoveCase { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-case-number" => Ok(En1992Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: dec_json(&get("case-id")?)?, key: dec_json(&get("key")?)?, value: dec_json(&get("value")?)? })),
        "change-case-text" => Ok(En1992Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: dec_json(&get("case-id")?)?, key: dec_json(&get("key")?)?, value: dec_json(&get("value")?)? })),
        "change-case-label-en" => Ok(En1992Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
        "change-case-label-de" => Ok(En1992Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
        "change-case-entry-id" => Ok(En1992Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
''' + a, 1)
wr(p, s)
print("ok")
