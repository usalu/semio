"""🧪️ Inserts `store::os_store::test_support::mutation_report_json` (wp-t12/drafts/mutation-report.rs) into the kernel's
store test-support module, and the committed-vector law module (wp-t12/drafts/law-vector.rs) into the stdio test-oracle
crate's `law` module as `law::vector`."""
root = "/Users/ueli/Documents/semio/"
store = root + "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
text = open(store, encoding="utf-8").read()
anchor = "pub mod test_support {\n    use super::*;\n"
assert text.count(anchor) == 1
helper = open(root + ".tmp-ticket/wp-t12/drafts/mutation-report.rs", encoding="utf-8").read()
if "pub fn mutation_report_json<" not in text:
    open(store, "w", encoding="utf-8").write(text.replace(anchor, anchor + "\n" + helper))
law_dir = root + "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/"
import os
os.makedirs(law_dir + "🧬️vector", exist_ok=True)
open(law_dir + "🧬️vector/🦀️.rs", "w", encoding="utf-8").write(open(root + ".tmp-ticket/wp-t12/drafts/law-vector.rs", encoding="utf-8").read())
law = open(law_dir + "🦀️.rs", encoding="utf-8").read()
mount = '\n//#region 🔖️Vector\n#[path = "🧬️vector/🦀️.rs"]\npub mod vector;\n//#endregion 🔖️Vector\n'
tests_anchor = "//#region 🧪️Tests\n"
assert law.count(tests_anchor) == 1
if "pub mod vector;" not in law:
    open(law_dir + "🦀️.rs", "w", encoding="utf-8").write(law.replace(tests_anchor, mount.lstrip("\n") + "\n" + tests_anchor))
os.makedirs(law_dir + "🧬️vector/🧪️tests/🔬️unit", exist_ok=True)
open(law_dir + "🧬️vector/🧪️tests/🔬️unit/🦀️.rs", "w", encoding="utf-8").write(open(root + ".tmp-ticket/wp-t12/drafts/law-vector-tests.rs", encoding="utf-8").read())
print("ok")
