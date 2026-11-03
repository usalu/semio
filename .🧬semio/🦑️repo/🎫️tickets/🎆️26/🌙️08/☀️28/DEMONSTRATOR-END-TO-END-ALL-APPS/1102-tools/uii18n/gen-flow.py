import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mkpatch import patch
root = "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
out = str(pathlib.Path(__file__).parent / "patches")
patch(root + "🗣️terminology/🦀️.rs", [(
 '        generation_needed: native_en "Add a generation to edit input values.",',
 '        form_no_inputs: native_en "No input widgets to generate from.", native_de "Keine Eingabeelemente zum Generieren vorhanden.", reuse_en "No input widgets to generate from.", reuse_de "Keine Eingabeelemente zum Generieren vorhanden.";\n        form_no_image: native_en "(no image)", native_de "(kein Bild)", reuse_en "(no image)", reuse_de "(kein Bild)";\n        generation_needed: native_en "Add a generation to edit input values.",')],
 out + "/flow-form-labels-1.patch")
patch(root + "🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs", [
 ('fn question_field(question: &PlaybookBlock, values: &PlaybookValues, patch_action: &str, generation_id: &str) ->',
  'fn question_field(question: &PlaybookBlock, values: &PlaybookValues, patch_action: &str, generation_id: &str, labels: &crate::editor::flow::terminology::FlowPlayLabels) ->'),
 ('question.src.clone().unwrap_or_else(|| "(no image)".into())', 'question.src.clone().unwrap_or_else(|| labels.form_no_image.as_str().into())'),
 ('question_field(question, &active.values, "updateGenerationValues", &active.id)?', 'question_field(question, &active.values, "updateGenerationValues", &active.id, labels)?'),
 ('ui_label("No input widgets to generate from.")?', 'ui_label(labels.form_no_inputs.as_str())?')],
 out + "/flow-form-labels-2.patch")
