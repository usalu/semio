import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1990")
p = A.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
s = rd(p)
mods = "{insert_case, remove_case, set_case_number, set_case_text, change_case_label_en, change_case_label_de, change_case_entry_id}"
a = "use semio_s_artifact_norm_en1990_mutations::En1990Mutation;\n"
assert s.count(a) == 1
s = s.replace(a, a + "use semio_s_artifact_norm_en1990_mut_design_cases::%s;\n" % mods, 1)
a = '        En1990Mutation::ChangeAccidentalAd(p) => format!("change-accidental-ad id={} new-ad={}", enc_json(&p.id), enc_json(&p.new_ad)),\n'
assert s.count(a) == 1
s = s.replace(a, a + '''        En1990Mutation::InsertCase(p) => format!("insert-case index={} case={}", enc_json(&p.index), enc_json(&p.case)),
        En1990Mutation::RemoveCase(p) => format!("remove-case index={}", enc_json(&p.index)),
        En1990Mutation::SetCaseNumber(p) => format!("set-case-number case-id={} key={} value={}", enc_json(&p.case_id), enc_json(&p.key), enc_json(&p.value)),
        En1990Mutation::SetCaseText(p) => format!("set-case-text case-id={} key={} value={}", enc_json(&p.case_id), enc_json(&p.key), enc_json(&p.value)),
        En1990Mutation::ChangeCaseLabelEn(p) => format!("change-case-label-en case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
        En1990Mutation::ChangeCaseLabelDe(p) => format!("change-case-label-de case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
        En1990Mutation::ChangeCaseEntryId(p) => format!("change-case-entry-id case-id={} new-value={}", enc_json(&p.case_id), enc_json(&p.new_value)),
''', 1)
a = '        other => Err(format!("unknown keyword {other}")),'
assert s.count(a) == 1
s = s.replace(a, '''        "insert-case" => Ok(En1990Mutation::InsertCase(insert_case::InsertCase { index: dec_json(arg("index")?)?, case: dec_json(arg("case")?)? })),
        "remove-case" => Ok(En1990Mutation::RemoveCase(remove_case::RemoveCase { index: dec_json(arg("index")?)? })),
        "set-case-number" => Ok(En1990Mutation::SetCaseNumber(set_case_number::SetCaseNumber { case_id: dec_json(arg("case-id")?)?, key: dec_json(arg("key")?)?, value: dec_json(arg("value")?)? })),
        "set-case-text" => Ok(En1990Mutation::SetCaseText(set_case_text::SetCaseText { case_id: dec_json(arg("case-id")?)?, key: dec_json(arg("key")?)?, value: dec_json(arg("value")?)? })),
        "change-case-label-en" => Ok(En1990Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-case-label-de" => Ok(En1990Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
        "change-case-entry-id" => Ok(En1990Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: dec_json(arg("case-id")?)?, new_value: dec_json(arg("new-value")?)? })),
''' + a, 1)
wr(p, s)
print("ok")
