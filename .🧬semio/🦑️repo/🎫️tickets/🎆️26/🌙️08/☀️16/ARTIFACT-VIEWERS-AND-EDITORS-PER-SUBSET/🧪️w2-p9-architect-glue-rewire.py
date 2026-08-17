import re

glue_path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"
text = open(glue_path, encoding="utf-8").read()

start_marker = "//#region 🎛️Apps"
end_marker = "//#endregion 🎛️Apps"
start = text.index(start_marker)
end = text.index(end_marker) + len(end_marker)
old_block = text[start:end]

# Extract the inner "apps { architect { ... } }" body (between the outer pub mod apps { ... } braces)
inner_start = old_block.index("pub mod architect {")
inner_end = old_block.rindex("}\n}\n//#endregion")
inner_body = old_block[inner_start:inner_end]  # "pub mod architect { ... }\n"

old_prefix = "../../🎛️apps/🏛️architect/"
editor_prefix = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"

editor_inner = inner_body.replace(old_prefix, editor_prefix)

editor_region = f"""//#region ✏️Editor
#[path = "."]
pub mod editor {{
    #[path = "."]
    {editor_inner}
}}
//#endregion ✏️Editor
"""

viewer_prefix = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/"

viewer_region = f"""//#region 👁️Viewer
#[path = "."]
pub mod viewer {{
    #[path = "."]
    pub mod architect {{
        #[path = "{viewer_prefix}🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {{
            #[path = "."]
            pub mod view {{
                #[path = "{viewer_prefix}🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {{
                    #[path = "{viewer_prefix}🎭️modes/👁️view/🪟️windows/📋️register/🦀️component.rs"]
                    pub mod register;
                }}
            }}
        }}
    }}
}}
//#endregion 👁️Viewer
"""

new_block = editor_region + "\n" + viewer_region
new_text = text[:start] + new_block + text[end:]
open(glue_path, "w", encoding="utf-8").write(new_text)
print("done")
print("---- editor_region preview ----")
print(editor_region[:800])
