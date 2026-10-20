import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📚️part4/🦀️.rs"
phspec.build(F, "check_dictionary_typing", [
 ("check_dictionary_subjects_and_relationships", "Checks the dictionary structure, the subject id uniqueness and the relationships of the dictionary.", 311, 368, {"drop_outs": ["subject_ids"]}),
 ("check_dictionary_properties", "Checks the properties of the dictionary.", 471, 471),
 ("check_dictionary_values_and_catalogue_tokens", "Checks the dictionary values and collects the tokens the catalogue uses.", 603, 679, {"drop_ins": ["subject_ids"]}),
 ("check_dictionary_references", "Checks the references of the dictionary against the subject ids.", 716, 716),
], types={"dict": "&semio_s_artifact_norm_iso16757_model::part_4::Dictionary", "subject_ids": "HashSet<String>"}, keep=[310], out=sys.argv[1])
