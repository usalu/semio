from pathlib import Path
import json

TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
glue_path = Path("✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs")
glue = glue_path.read_text(encoding="utf-8")
md_dir = ROSTER["md"]["dir"]
def_dir = ROSTER["deflate"]["dir"]
zip_dir = ROSTER["zip"]["dir"]
txt_dir = ROSTER["txt"]["dir"]
bin_dir = ROSTER["binary"]["dir"]
DESER = TOKENS["deserializers"]
SER = TOKENS["serializers"]

marker = '    #[path = "."]\n    pub mod md {'
ms = glue.find(marker)
assert ms >= 0
insert_at = glue.rfind("\n}", 0, glue.find("//#endregion Artifacts"))
md_close = glue.rfind("\n    }", 0, insert_at)
md_block = glue[ms:md_close + 6]

def adapt(block, from_mid, to_mid, from_dir, to_dir):
    b = block.replace(from_dir, to_dir)
    b = b.replace("pub mod " + from_mid, "pub mod " + to_mid)
    return b

deflate_block = adapt(md_block, "md", "deflate", md_dir, def_dir)
deflate_block = deflate_block.replace(txt_dir, bin_dir)
deflate_block = deflate_block.replace("pub mod txt", "pub mod binary")

zip_block = adapt(md_block, "md", "zip", md_dir, zip_dir)
zip_block = zip_block.replace(txt_dir, bin_dir)
zip_block = zip_block.replace("pub mod txt", "pub mod binary")

def inject_deflate_peer(block, direction, folder, zip_dir, bin_dir, def_dir):
    needle = (
        '                        pub mod binary {\n'
        + '                            #[path = "../../🗿️artifacts/' + zip_dir + '/🚪️io/' + direction + '/' + folder + '/🗿️artifacts/' + bin_dir + '/🦀️component.rs"]'
    )
    idx = block.find(needle)
    if idx < 0:
        raise SystemExit("missing needle for " + direction)
    sub = block[idx:]
    end_rel = sub.find("\n                        }")
    end = idx + end_rel + len("\n                        }")
    extra = (
        "\n                        pub mod deflate {\n"
        + '                            #[path = "../../🗿️artifacts/' + zip_dir + '/🚪️io/' + direction + '/' + folder + '/🗿️artifacts/' + def_dir + '/🦀️component.rs"]\n'
        + "                            mod component;\n"
        + "                            pub use component::*;\n"
        + "                        }"
    )
    return block[:end] + extra + block[end:]

zip_block = inject_deflate_peer(zip_block, "📥️import", DESER, zip_dir, bin_dir, def_dir)
zip_block = inject_deflate_peer(zip_block, "📤️export", SER, zip_dir, bin_dir, def_dir)

if "pub mod deflate" not in glue:
    at = md_close + len("\n    }")
    new_glue = glue[:at] + "\n\n" + deflate_block + "\n\n" + zip_block + glue[at:]
    glue_path.write_text(new_glue, encoding="utf-8")
    print("glued", new_glue.count(chr(10))+1)
else:
    print("glue already has deflate")

idx_path = Path("✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/📦️index.ts")
idx = idx_path.read_text(encoding="utf-8")
for mid, d in [("deflate", def_dir), ("zip", zip_dir)]:
    line = 'export * as ' + mid + ' from "../../🗿️artifacts/' + d + '/🟦️component.ts";'
    if line not in idx:
        if not idx.endswith("\n"):
            idx += "\n"
        idx += line + "\n"
idx_path.write_text(idx, encoding="utf-8")
print(idx_path.read_text())

plug_path = Path("✏️s/🔌️plugins/🗄️stdio/🔌️plugin/�idx_path.read_text())

plug_path = Path("✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️component.rs")
plug = plug_path.read_text(encoding="utf-8")
for mid in ["deflate", "zip"]:
    line = "    crate::artifacts::" + mid + "::engine::register();"
    if line not in plug:
        plug = plug.replace('    Plugin::builder("stdio")', line + "\n    Plugin::builder(\"stdio\")')
plug_path.write_text(plug, encoding="utf-8")
print(plug_path.read_text())
