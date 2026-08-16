#!/usr/bin/env python3
# 🐍️ SCRATCH (ticket-local): W2 packet P3 (norm) — pass 3. Rewrites `📦️glue.rs`'s `🎛️Apps` region
# into independent `✏️Editor`/`👁️Viewer` regions (mirroring the cad pilot), and repoints the
# `📚️Examples` region's fifteen `app_<x>_demo_session` mounts at the new editor path. Every `#[path]`
# is derived programmatically from the same APPS table pass 1/2 used — never hand-typed.
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/📕️norm"
GLUE = PLUGIN / "📦️packages/🦀️rust/📦️glue.rs"

APPS = [
    "iso16757", "vdi3805", "din4108", "din16798",
    "en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999",
    "din18599",
]
DIRS = {
    "iso16757": "📓️iso16757", "vdi3805": "📔️vdi3805", "din4108": "📕️din4108", "din16798": "📗️din16798",
    "en1990": "📘️en1990", "en1991": "📘️en1991", "en1992": "📘️en1992", "en1993": "📘️en1993",
    "en1994": "📘️en1994", "en1995": "📘️en1995", "en1996": "📘️en1996", "en1997": "📘️en1997",
    "en1998": "📘️en1998", "en1999": "📘️en1999", "din18599": "📙️din18599",
}
STD = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def editor_block(v):
    d = DIRS[v]
    base = f"../../🗿️artifacts/{d}/{STD}/✏️editor"
    return f"""    #[path = "."]
    pub mod {v} {{
        #[path = "{base}/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {{
            #[path = "{base}/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "{base}/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
            #[path = "{base}/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
        }}

        #[path = "."]
        pub mod modes {{
            #[path = "."]
            pub mod edit {{
                #[path = "{base}/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {{
                    #[path = "{base}/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "{base}/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }}
            }}
        }}

        #[path = "."]
        pub mod panels {{
            #[path = "{base}/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "{base}/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "{base}/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }}
    }}
"""


def viewer_block(v):
    d = DIRS[v]
    base = f"../../🗿️artifacts/{d}/{STD}/👁️viewer"
    return f"""    #[path = "."]
    pub mod {v} {{
        #[path = "{base}/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {{
            #[path = "."]
            pub mod view {{
                #[path = "{base}/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {{
                    #[path = "{base}/🎭️modes/👁️view/🪟️windows/📊️report/🦀️component.rs"]
                    pub mod report;
                }}
            }}
        }}
    }}
"""


def build_apps_region():
    editor = "".join(editor_block(v) for v in APPS)
    viewer = "".join(viewer_block(v) for v in APPS)
    return (
        "//#region ✏️Editor\n"
        "#[path = \".\"]\n"
        "pub mod editor {\n"
        f"{editor}"
        "}\n"
        "//#endregion ✏️Editor\n\n"
        "//#region 👁️Viewer\n"
        "#[path = \".\"]\n"
        "pub mod viewer {\n"
        f"{viewer}"
        "}\n"
        "//#endregion 👁️Viewer\n"
    )


def main():
    text = GLUE.read_text(encoding="utf-8")

    start = text.index("//#region 🎛️Apps")
    end = text.index("//#endregion 🎛️Apps") + len("//#endregion 🎛️Apps") + 1
    old_region = text[start:end]
    assert "🎛️apps/📕️din4108" in old_region
    text = text[:start] + build_apps_region() + text[end:]

    # repoint the fifteen app_<x>_demo_session mounts at the new editor path — the ORIGINAL block is
    # alphabetical by variant (din16798, din18599, din4108, en1990..en1999, iso16757, vdi3805), a
    # different order than the APPS table above; matched here verbatim so the old-block assertion
    # actually anchors on the real file content instead of guessing.
    EXAMPLES_ORDER = ["din16798", "din18599", "din4108", "en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999", "iso16757", "vdi3805"]
    old_lines = []
    for v in EXAMPLES_ORDER:
        d = DIRS[v]
        old_lines.append(f'    #[path = "../../🎛️apps/{d}/📚️examples/🎬️demo-session/🦀️component.rs"]\n    pub mod app_{v}_demo_session;\n')
    old_block = "".join(old_lines)
    assert old_block in text, "old examples block not found verbatim"
    new_lines = []
    for v in EXAMPLES_ORDER:
        d = DIRS[v]
        new_lines.append(f'    #[path = "../../🗿️artifacts/{d}/{STD}/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]\n    pub mod app_{v}_demo_session;\n')
    text = text.replace(old_block, "".join(new_lines), 1)

    GLUE.write_text(text, encoding="utf-8")
    print("glue.rs rewritten")


if __name__ == "__main__":
    main()
