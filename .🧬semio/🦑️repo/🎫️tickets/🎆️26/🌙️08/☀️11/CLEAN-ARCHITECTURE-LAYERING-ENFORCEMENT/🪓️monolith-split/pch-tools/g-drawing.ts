// 🖊️ Drawing oracle: the DXF reader support has exactly one owner, the drawing oracle crate; the dxf artifact oracle (already its dependent) imports it instead of path-mounting it.
import {DRY, log, put, read, replaceOnce} from "./dd.ts";
const D = "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing";
const support = `${D}/🧰️support/🦀️.rs`;
const text = read(support);
if (text.split("pub(super) fn ").length !== 4) throw new Error("expected three pub(super) functions");
put(support, text.replaceAll("pub(super) fn ", "pub fn "));
replaceOnce(`${D}/🦀️.rs`, '#[path = "🧰️support/🦀️.rs"]\nmod reference_support;', '#[path = "🧰️support/🦀️.rs"]\npub mod reference_support;');
replaceOnce("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🔮️oracles/🦀️.rs", '#[path = "../../../../../../../🔮️oracles/🖊️drawing/🧰️support/🦀️.rs"]\nmod reference_support;', "use semio_s_plugin_stdio_drawing_test_oracle::reference_support;");
log(DRY ? "dry run" : "written");
