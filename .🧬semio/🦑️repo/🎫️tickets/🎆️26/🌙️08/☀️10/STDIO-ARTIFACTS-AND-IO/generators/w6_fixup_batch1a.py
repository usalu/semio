#!/usr/bin/env python3
"""Post-migrate fixes: schema includes, glue, io imports."""
from __future__ import annotations

import importlib.util
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
W6 = TICKET / "generators/w6_migrate_batch1a.py"
spec = importlib.util.spec_from_file_location("w6", W6)
w6 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(w6)

PLUGINS = [
    ("✒️writer", "✒️writer"),
    ("➗️mathematical", "➗️mathematical"),
    ("🌊️flow", "🌊️flow"),
    ("🌿️vcs", "🌿️vcs"),
    ("🕸️dag", "🕸️dag"),
]


def fix_schema_includes(art: Path) -> bool:
    schema_rs = art / "🧬️schema" / "🦀️component.rs"
    if not schema_rs.exists():
        return False
    t = schema_rs.read_text(encoding="utf-8")
    t2 = t.replace("../📸️snapshot/🧬️schema/", "📸️snapshot/")
    t2 = t2.replace("../🔺️diff/🧬️schema/", "🔺️diff/")
    if t2 != t:
        schema_rs.write_text(t2, encoding="utf-8")
        return True
    return False


def fix_io_imports(art: Path, mod: str) -> None:
    pdf_imp = """use semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc;
use semio_s_plugin_stdio::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};"""
    docx_imp = """use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxEntry;
use semio_s_plugin_stdio::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};"""
    for slug, fix in (("pdf", pdf_imp), ("docx", docx_imp)):
        p = art / "🚪️io/📥️import" / w6.DESER / "🗿️artifacts" / w6.STDIO_DIRS[slug] / w6.RS_LEAF
        if p.exists():
            t = p.read_text(encoding="utf-8")
            t = re.sub(
                r"use semio_s_plugin_stdio::artifacts::pdf::\{PageDoc, PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA\};",
                pdf_imp,
                t,
            )
            t = re.sub(
                r"use semio_s_plugin_stdio::artifacts::docx::\{DocxEntry, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA\};",
                docx_imp,
                t,
            )
            p.write_text(t, encoding="utf-8")
    exp_pdf = art / "🚪️io/📤️export" / w6.SER / "🗿️artifacts" / w6.STDIO_DIRS["pdf"] / w6.RS_LEAF
    if exp_pdf.exists():
        t = exp_pdf.read_text(encoding="utf-8")
        t = t.replace(
            "use semio_s_plugin_stdio::artifacts::pdf::{PageDoc, PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};",
            pdf_imp,
        )
        exp_pdf.write_text(t, encoding="utf-8")
    exp_docx = art / "🚪️io/📤️export" / w6.SER / "🗿️artifacts" / w6.STDIO_DIRS["docx"] / w6.RS_LEAF
    if exp_docx.exists():
        t = exp_docx.read_text(encoding="utf-8")
        t = t.replace(
            "use semio_s_plugin_stdio::artifacts::docx::{DocxEntry, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};",
            docx_imp,
        )
        exp_docx.write_text(t, encoding="utf-8")
    svg = art / "🚪️io/📥️import" / w6.DESER / "🗿️artifacts" / w6.STDIO_DIRS["svg"] / w6.RS_LEAF
    if svg.exists():
        t = svg.read_text(encoding="utf-8")
        if "from.body" in t:
            t = t.replace(
                "<{snap} as store::DocumentDsl>::parse_dsl(&from.body)".format(snap=f"{w6.BATCH[4]['prefix']}Snapshot" if mod == 'dag' else 'DagSnapshot'),
                "{\n    let text = semio_s_plugin_stdio::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);\n    <DagSnapshot as store::DocumentDsl>::parse_dsl(&text)\n}".replace("DagSnapshot", f"crate::artifacts::{mod}::DagSnapshot" if mod == 'dag' else 'X'),
            )
        # simpler fix for dag only
        if mod == "dag":
            svg.write_text(
                f"""//! Deserialize dag via stdio.svg.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::svg::{{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA}};
use semio_s_plugin_stdio::artifacts::xml::schema::snapshot::xml_document_to_text;

pub fn register() {{}}

pub fn deserialize(from: &SvgSnapshot) -> Result<DagSnapshot, store::TextError> {{
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let text = xml_document_to_text(&from.doc);
    <DagSnapshot as store::DocumentDsl>::parse_dsl(&text)
}}
""",
                encoding="utf-8",
            )


def patch_glue_fixed(cfg: dict) -> None:
    art_e, mod_, plug = cfg["artifact"], cfg["mod"], w6.plugin_path(cfg)
    art = w6.art_path(cfg)
    glue = plug / "📦️packages/🦀️rust/📦️glue.rs"
    muts = art / "🧬️schema/🧬️mutations"
    mut_dirs = [c.name for c in sorted(muts.iterdir()) if c.is_dir() and c.name not in (w6.TEXT, w6.BINARY)] if muts.exists() else []
    ap = f"../../🗿️artifacts/{art_e}"
    pfx, snap, mutn, diffn = cfg["prefix"], f"{cfg['prefix']}Snapshot", cfg["mutation"], cfg["diff"]
    parts = ["//#region 🗿️Artifacts", '#[path = "."]', "pub mod artifacts {", '    #[path = "."]', f"    pub mod {mod_} {{", f'        #[path = "{ap}/🦀️component.rs"]', "        mod component;", "        pub use component::*;", f"        pub use crate::artifacts::{mod_}::schema::snapshot::{snap};", f"        pub use crate::artifacts::{mod_}::schema::mutations::{mutn};", f"        pub use crate::artifacts::{mod_}::schema::diff::{diffn};", ""]
    parts += ['        #[path = "."]', "        pub mod schema {", f'            #[path = "{ap}/🧬️schema/🦀️component.rs"]', "            mod component;", "            pub use component::*;", '            #[path = "."]', "            pub mod snapshot {", f'                #[path = "{ap}/🧬️schema/📸️snapshot/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/📸️snapshot/{w6.TEXT}/🦀️component.rs"]', "                pub mod text;", f'                #[path = "{ap}/🧬️schema/📸️snapshot/{w6.BINARY}/🦀️component.rs"]', "                pub mod binary;", "            }", '            #[path = "."]', "            pub mod diff {", f'                #[path = "{ap}/🧬️schema/🔺️diff/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/🔺️diff/{w6.TEXT}/🦀️component.rs"]', "                pub mod text;", "                pub use text::*;", f'                #[path = "{ap}/🧬️schema/🔺️diff/{w6.BINARY}/🦀️component.rs"]', "                pub mod binary;", "            }", '            #[path = "."]', "            pub mod mutations {", f'                #[path = "{ap}/🧬️schema/🧬️mutations/🦀️component.rs"]', "                mod component;", "                pub use component::*;", f'                #[path = "{ap}/🧬️schema/🧬️mutations/{w6.TEXT}/🦀️component.rs"]', "                pub mod text;", f'                #[path = "{ap}/🧬️schema/🧬️mutations/{w6.BINARY}/🦀️component.rs"]', "                pub mod binary;"]
    for d in mut_dirs:
        mm = w6.mut_mod(d)
        base = f"{ap}/🧬️schema/🧬️mutations/{d}"
        parts += ['                #[path = "."]', f"                pub mod {mm} {{", f'                    #[path = "{base}/🦠️mutation/🦀️component.rs"]', "                    pub mod mutation;"]
        if (art / "🧬️schema/🧬️mutations" / d / "🔺️diff" / "🦀️component.rs").exists():
            parts += [f'                    #[path = "{base}/🔺️diff/🦀️component.rs"]', "                    pub mod diff;"]
        if (art / "🧬️schema/🧬️mutations" / d / "↩️inverse" / "🦀️component.rs").exists():
            parts += [f'                    #[path = "{base}/↩️inverse/🦀️component.rs"]', "                    pub mod inverse;"]
        parts += ["                }"]
    parts += ["            }", "        }", ""]
    parts += [
        f'        pub mod op {{ pub use crate::artifacts::{mod_}::schema::mutations::text::*; pub use crate::artifacts::{mod_}::schema::mutations::{mutn}; }}',
        f'        pub mod dsl {{ pub use crate::artifacts::{mod_}::schema::snapshot::text::*; }}',
        f'        pub mod spr {{ pub use crate::artifacts::{mod_}::schema::mutations::binary::*; }}',
        f'        pub mod pack {{ pub use crate::artifacts::{mod_}::schema::snapshot::binary::*; }}',
        f'        pub mod diff {{ pub use crate::artifacts::{mod_}::schema::diff::*; pub mod schema {{ pub use crate::artifacts::{mod_}::schema::diff::*; }} pub mod text {{ pub use crate::artifacts::{mod_}::schema::diff::text::*; }} }}',
        f'        pub mod mutations {{ pub use crate::artifacts::{mod_}::schema::mutations::*; }}',
        '        #[path = "."]',
        "        pub mod snapshot {",
        f'            pub mod schema {{ pub use crate::artifacts::{mod_}::schema::snapshot::*; }}',
        f'            pub mod pack {{ pub use crate::artifacts::{mod_}::schema::snapshot::binary::*; }}',
        "        }",
        f'        #[path = "{ap}/{w6.BUILDER}/🦀️component.rs"]',
        "        pub mod builder;",
        f'        #[path = "{ap}/{w6.DECOMPOSER}/🦀️component.rs"]',
        "        pub mod decomposer;",
        '        #[path = "."]',
        "        pub mod io {",
        f'            #[path = "{ap}/🚪️io/🦀️component.rs"]',
        "            mod component;",
        "            pub use component::*;",
        '            #[path = "."]',
        "            pub mod import {",
        '                #[path = "."]',
        f"                pub mod deserializers {{",
        '                    #[path = "."]',
        "                    pub mod artifacts {",
    ]
    for slug in cfg["stdio"]:
        dname = w6.STDIO_DIRS[slug]
        parts += ['                        #[path = "."]', f"                        pub mod {slug} {{", f'                            #[path = "{ap}/🚪️io/📥️import/{w6.DESER}/🗿️artifacts/{dname}/🦀️component.rs"]', "                            mod component;", "                            pub use component::*;", "                        }"]
    parts += ["                    }", "                }", "            }", '            #[path = "."]', "            pub mod export {", '                #[path = "."]', f"                pub mod serializers {{", '                    #[path = "."]', "                    pub mod artifacts {"]
    for slug in cfg["stdio"]:
        dname = w6.STDIO_DIRS[slug]
        parts += ['                        #[path = "."]', f"                        pub mod {slug} {{", f'                            #[path = "{ap}/🚪️io/📤️export/{w6.SER}/🗿️artifacts/{dname}/🦀️component.rs"]', "                            mod component;", "                            pub use component::*;", "                        }"]
    parts += ["                    }", "                }", "            }"]
    for slug in cfg["stdio"]:
        parts += ['            #[path = "."]', f"            pub mod {slug} {{", '                #[path = "."]', "                pub mod export {", f"                    pub use crate::artifacts::{mod_}::io::export::serializers::artifacts::{slug}::*;", "                }", '                #[path = "."]', "                pub mod import {", f"                    pub use crate::artifacts::{mod_}::io::import::deserializers::artifacts::{slug}::*;", "                }", "            }"]
    parts += ["        }", f'        #[path = "{ap}/⚙️engine/🦀️component.rs"]', "        pub mod engine;", "    }", "}", ""]
    new_region = "\n".join(parts) + "\n"
    text = glue.read_text(encoding="utf-8")
    start = text.find("//#region 🗿️Artifacts")
    end = text.find("//#endregion 🗿️Artifacts")
    glue.write_text(text[:start] + new_region + text[end:], encoding="utf-8")


def main() -> None:
    cfg_by_mod = {c["mod"]: c for c in w6.BATCH}
    mod_by_plug = {
        "✒️writer": "writer",
        "➗️mathematical": "mathematical",
        "🌊️flow": "flow",
        "🌿️vcs": "vcs",
        "🕸️dag": "dag",
    }
    for plug, art_name in PLUGINS:
        art = ROOT / "✏️s/🔌️plugins" / plug / "🗿️artifacts" / art_name
        mod = mod_by_plug[plug]
        fix_schema_includes(art)
        fix_io_imports(art, mod)
        patch_glue_fixed(cfg_by_mod[mod])
        print("fixed", plug)


if __name__ == "__main__":
    main()
