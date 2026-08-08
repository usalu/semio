
from pathlib import Path

p = next(Path("/Users/ueli/Documents/semio").glob("**/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs"))
text = p.read_text()
orig = text

text = text.replace(
    "store::test_support::assert_op_text_binary_equivalence",
    "store::os_store::test_support::assert_op_text_binary_equivalence",
)

old1 = 'assert!(error.contains("must not emit operations"), "unexpected error: {error}");'
new1 = 'assert!(error.message.contains("must not emit operations"), "unexpected error: {}", error.message);'
old2 = 'assert!(error.contains("typed command channel"), "unexpected error: {error}");'
new2 = 'assert!(error.message.contains("typed command channel"), "unexpected error: {}", error.message);'
text = text.replace(old1, new1)
text = text.replace(old2, new2)

text = text.replace(
    'assert_eq!(all_tree.sections[0].label.as_deref(), Some("Actions"));',
    'assert_eq!(all_tree.sections[0].label.as_ref().map(|l| l.as_str()), Some("Actions"));',
)
text = text.replace(
    'assert_eq!(all_tree.sections[1].label.as_deref(), Some("Commands"));',
    'assert_eq!(all_tree.sections[1].label.as_ref().map(|l| l.as_str()), Some("Commands"));',
)

text = text.replace("fn command_id(&self, command: &TestCommand) -> &str {", "fn command_id(command: &TestCommand) -> &'static str {", 1)
text = text.replace("fn clipboard_media_type(&self) -> Option<MediaType> {", "fn clipboard_media_type() -> Option<MediaType> {", 1)
text = text.replace("fn cut_operations(&self, doc:", "fn cut_operations(doc:", 1)
text = text.replace("fn paste_operations(&self, _doc:", "fn paste_operations(_doc:", 1)
text = text.replace("if !self.clipboard_accepts()", "if !Self::clipboard_accepts()", 1)

# Fault import for testkit_tests
marker = "use protocol::{Mutation, MutationDiff};\n            use serde::{Deserialize, Serialize};"
if marker in text and "use semio_framework::Fault;" not in text[text.find("mod testkit_tests"):text.find("struct DummySnapshot")]:
    text = text.replace(marker, marker + "\n            use semio_framework::Fault;", 1)

anchor = "use store::EngineHandles;\n        use crate::{selection_count_phrase"
if anchor in text:
    text = text.replace(anchor, "use store::EngineHandles;\n        use semio_framework::Fault;\n        use crate::{selection_count_phrase", 1)

p.write_text(text)
print("changed", text != orig)
for s in ["fn cut_operations(&self", "fn paste_operations(&self", "fn clipboard_media_type(&self)", "fn command_id(&self, command: &TestCommand)", "self.clipboard_accepts()", "error.contains("]:
    print(repr(s), s in text)
