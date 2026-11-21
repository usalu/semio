"""🧾 Exercises the live PDF semantic cursors against mixed-resource file objects."""

import os
from pathlib import Path
import runpy
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
driver = runpy.run_path(str(ticket / "🎞️document-tests.py"))
base = Path(sys.argv[1]).resolve()
patch = driver["patch_harness"]
stage = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else None

def mixed_patch(scratch, main, root):
    patch(scratch, main, root)
    if stage:
        pointer = scratch / "document_lift.rs"
        source = pointer.read_text()
        active = root / "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs"
        assert str(active) in source
        pointer.write_text(source.replace(str(active), str(stage / "document.rs")))
        path = scratch / "images.rs"
        source = path.read_text()
        active = root / "🔨️modules/🖼️images/🎞️controlled/🦀️.rs"
        assert str(active) in source
        path.write_text(source.replace(str(active), str(stage / "images.rs")))
    shim = scratch / "document.rs"
    source = shim.read_text().replace('["complete", "cancel", "missing-name", "mask", "short", "alias", "codec"]','["complete"]')
    source = source.replace("Ok(Event::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,",
                            "Ok(Event::NeedsAllocation { bytes }) => { assert!(bytes <= if reserved { 1 << 20 } else { 32_768 - 128 - cursor.next_inline_body_bytes() }); grant.allocation_bytes = bytes; },")
    insert = '                    for ((page, role), name)'
    before = source.index(insert)
    source = source[:before] + r'''
                    assert_eq!(output.fonts.len(), 1);
                    assert_eq!(output.fonts[0].id, "F1");
                    for (index, role) in output.admitted_stream_roles[3..].iter().enumerate() {
                        use crate::content_operators::PdfTextString;
                        let PdfStreamRoleValue::Operators { content } = &role.value else { unreachable!(); };
                        assert!(content.iter().any(|op| matches!(op, PdfOp::SetFont { name, size } if name == "F1" && *size == 12.)));
                        assert!(content.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if *text == format!("Mixed page {}", index + 1))));
                        assert_eq!(role.dependencies.len(), 4);
                        let dependency = &role.dependencies[1];
                        let owner = [2, 6, 10, 2][index];
                        let keys: Vec<_> = dependency.path.iter().map(|PdfGraphPath::Entry { key }| key.as_str()).collect();
                        assert_eq!(dependency.owner, ObjRef { num: owner, gen: 0 });
                        assert_eq!(keys, if index == 2 { vec!["Font"] } else { vec!["Resources", "Font"] });
                        assert_eq!(role.dependencies[2].owner, ObjRef { num: [4, 7, 12, 14][index], gen: 0 });
                        assert_eq!(role.dependencies[3].owner, ObjRef { num: 15, gen: 0 });
                    }
''' + source[before:]
    shim.write_text(source)

runner = ticket / "🎞️document-tests.py"
source = runner.read_text().replace('str(OUTPUT / "🎞️controlled-images.pdf")', 'str(OUTPUT / "mixed.pdf")')
source = source.replace('def exercise(base):', 'patch_harness = mixed_patch\n\ndef exercise(base):')
os.environ["PDF_CONTROLLED_LIVE"] = "1"
namespace = {"__file__": str(runner), "mixed_patch": mixed_patch}
exec(compile(source, str(runner), "exec"), namespace)
namespace["exercise"](base)
