#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""W12+: rewrite one stdio artifact's `pub mod <name> { ... }` block in glue.rs to the new
standards/subsets tree, generalized from the hand-verified 💾️binary block. Must run AFTER
w12_migrate_stdio_artifact.py has physically moved that artifact's files. Idempotent: finds
the CURRENT block (old or new shape) by brace-matching and replaces it wholesale.

Usage: python3 w12_rewrite_glue.py <dir> [<dir> ...]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
STDIO_ART = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
GLUE = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs")
HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "w9_standards_table.json"), encoding="utf-8") as f:
    STANDARDS = json.load(f)["stdio"]
with open(os.path.join(HERE, "w9_owner_table_v2.json"), encoding="utf-8") as f:
    OWNER_V2 = json.load(f)
KIND_TO_DIR = {k: v["dir"] for k, v in OWNER_V2["stdio_roster"].items()}
DIR_TO_KIND = {v: k for k, v in KIND_TO_DIR.items()}


def artifact_name_from_root(dir_name):
    root_rs = os.path.join(STDIO_ART, dir_name, "🦀️component.rs")
    text = open(root_rs, encoding="utf-8").read()
    m = re.search(r"schema::snapshot::(\w+)Snapshot", text)
    if not m:
        raise SystemExit(f"could not find <Name>Snapshot pattern in {root_rs}")
    return m.group(1)


def leaf(art_rel_path, filename="🦀️component.rs"):
    return f"../../🗿️artifacts/{art_rel_path}/{filename}"


def has(rel_path):
    return os.path.exists(os.path.join(STDIO_ART, rel_path))


def scan_io_targets(kind, std_dir, direction, child):
    """Return sorted list of target stdio dirs this artifact's io/<direction>/<child>/artifacts/ has."""
    base = os.path.join(STDIO_ART, kind_dir(kind), "🏅️standards", std_dir, "🪆️subsets", "✳️any",
                         "🚪️io", direction, child, "🗿️artifacts")
    if not os.path.isdir(base):
        return []
    return sorted(d for d in os.listdir(base) if os.path.isdir(os.path.join(base, d)))


def kind_dir(kind):
    return KIND_TO_DIR[kind]


def build_schema_tree(art_rel, indent):
    """Emit the schema{snapshot,diff,mutations} nested block, mirroring taxonomy shape exactly."""
    I = "    " * indent
    out = []
    out.append(f'{I}#[path = "."]\n{I}pub mod schema {{\n')
    out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🧬️schema")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for child, subdirs in (("snapshot", ["text", "binary"]), ("diff", ["text", "binary"])):
        emoji_child = {"snapshot": "📸️snapshot", "diff": "🔺️diff"}[child]
        out.append(f'{I}    #[path = "."]\n{I}    pub mod {child} {{\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}")}"]\n{I}        mod component;\n{I}        pub use component::*;\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}/📝️text")}"]\n{I}        pub mod text;\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}/💾️binary")}"]\n{I}        pub mod binary;\n')
        out.append(f'{I}    }}\n')
    out.append(f'{I}    #[path = "."]\n{I}    pub mod mutations {{\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations")}"]\n{I}        mod component;\n{I}        pub use component::*;\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/📝️text")}"]\n{I}        pub mod text;\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/💾️binary")}"]\n{I}        pub mod binary;\n')
    out.append(f'{I}        #[path = "."]\n{I}        pub mod set_snapshot {{\n')
    out.append(f'{I}            #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation")}"]\n{I}            pub mod mutation;\n')
    out.append(f'{I}            #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff")}"]\n{I}            pub mod diff;\n')
    out.append(f'{I}            #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse")}"]\n{I}            pub mod inverse;\n')
    out.append(f'{I}        }}\n')
    out.append(f'{I}    }}\n')
    out.append(f'{I}}}\n')
    return "".join(out)


def build_io_tree(kind, art_rel, indent):
    I = "    " * indent
    out = []
    out.append(f'{I}#[path = "."]\n{I}pub mod io {{\n')
    out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🚪️io")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for direction, child, label in (("📥️import", "🧩️deserializers", "import"), ("📤️export", "🧵️serializers", "export")):
        mod_direction = {"📥️import": "import", "📤️export": "export"}[direction]
        mod_child = {"🧩️deserializers": "deserializers", "🧵️serializers": "serializers"}[child]
        targets = scan_io_targets(kind, STANDARDS[kind]["dir"], direction, child)
        out.append(f'{I}    #[path = "."]\n{I}    pub mod {mod_direction} {{\n')
        out.append(f'{I}        #[path = "."]\n{I}        pub mod {mod_child} {{\n')
        out.append(f'{I}            #[path = "."]\n{I}            pub mod artifacts {{\n')
        for target_dir in targets:
            target_kind = DIR_TO_KIND.get(target_dir)
            if target_kind is None:
                continue
            tstd_dir = STANDARDS[target_kind]["dir"]
            tmod = STANDARDS[target_kind]["rust_mod"]
            out.append(f'{I}                #[path = "."]\n{I}                pub mod {target_kind} {{\n')
            out.append(f'{I}                    #[path = "."]\n{I}                    pub mod {tmod} {{\n')
            out.append(f'{I}                        #[path = "."]\n{I}                        pub mod any {{\n')
            leaf_rel = f"{art_rel}/🚪️io/{direction}/{child}/🗿️artifacts/{target_dir}/{tstd_dir}/✳️any"
            out.append(f'{I}                            #[path = "{leaf(leaf_rel)}"]\n{I}                            mod component;\n{I}                            pub use component::*;\n')
            out.append(f'{I}                        }}\n{I}                    }}\n{I}                }}\n')
        out.append(f'{I}            }}\n{I}        }}\n{I}    }}\n')
    out.append(f'{I}}}\n')
    return "".join(out)


def build_block(dir_name):
    kind = DIR_TO_KIND[dir_name]
    Name = artifact_name_from_root(dir_name)
    std = STANDARDS[kind]
    std_dir = std["dir"]
    mod = std["rust_mod"]
    art_rel = dir_name

    lines = []
    lines.append(f'    pub mod {kind} {{\n')
    lines.append(f'        #[path = "{leaf(art_rel)}"]\n        mod component;\n        pub use component::*;\n\n')

    lines.append(f'        #[path = "."]\n        pub mod standards {{\n')
    lines.append(f'            #[path = "."]\n            pub mod {mod} {{\n')
    lines.append(f'                #[path = "{leaf(f"{art_rel}/🏅️standards/{std_dir}/⚙️engine")}"]\n                pub mod engine;\n')
    lines.append(f'                #[path = "{leaf(f"{art_rel}/🏅️standards/{std_dir}/🏗️builder")}"]\n                pub mod builder;\n')
    lines.append(f'                #[path = "{leaf(f"{art_rel}/🏅️standards/{std_dir}/🧐️analyzer")}"]\n                pub mod analyzer;\n')
    lines.append(f'                #[path = "{leaf(f"{art_rel}/🏅️standards/{std_dir}/🎹️composer")}"]\n                pub mod composer;\n')
    lines.append(f'                #[path = "."]\n                pub mod subsets {{\n')
    lines.append(f'                    #[path = "."]\n                    pub mod any {{\n')
    subset_art_rel = f"{art_rel}/🏅️standards/{std_dir}/🪆️subsets/✳️any"
    lines.append(build_schema_tree(subset_art_rel, 6))
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🏗️builder")}"]\n                        pub mod builder;\n')
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🧐️analyzer")}"]\n                        pub mod analyzer;\n')
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🎹️composer")}"]\n                        pub mod composer;\n')
    if has(f"{subset_art_rel}/🚪️io"):
        lines.append(build_io_tree(kind, subset_art_rel, 6))
    lines.append(f'                    }}\n                }}\n')
    lines.append(f'            }}\n        }}\n\n')

    lines.append('        // ---- Shims: keep pre-migration module paths resolving for external callers ----\n')
    lines.append(f'        pub mod schema {{\n            pub use super::standards::{mod}::subsets::any::schema::*;\n        }}\n')
    lines.append(f'        pub mod engine {{\n            pub use super::standards::{mod}::engine::*;\n        }}\n')
    if has(f"{subset_art_rel}/🚪️io"):
        lines.append(f'        pub mod io {{\n            pub use super::standards::{mod}::subsets::any::io::*;\n        }}\n')
    lines.append('\n')

    lines.append(f'        #[path = "{leaf(f"{art_rel}/🏗️builder")}"]\n        pub mod builder;\n')
    lines.append(f'        #[path = "{leaf(f"{art_rel}/🧐️analyzer")}"]\n        pub mod analyzer;\n')
    lines.append(f'        #[path = "{leaf(f"{art_rel}/🎹️composer")}"]\n        pub mod composer;\n\n')

    if has(f"{art_rel}/📚️examples/🎬️demo"):
        lines.append(f'        #[path = "."]\n        pub mod examples {{\n            #[path = "."]\n            pub mod demo {{\n')
        lines.append(f'                #[path = "{leaf(f"{art_rel}/📚️examples/🎬️demo")}"]\n                mod component;\n                pub use component::*;\n')
        lines.append(f'            }}\n        }}\n')

    lines.append('    }\n')
    return "".join(lines)


def replace_block(text, kind):
    marker = f"    pub mod {kind} {{\n"
    if marker not in text:
        raise SystemExit(f"marker not found for {kind}: {marker!r}")
    start = text.index(marker)
    brace_start = text.index("{", start)
    depth = 0
    end = None
    for idx in range(brace_start, len(text)):
        c = text[idx]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                end = idx + 1
                break
    if end is None:
        raise SystemExit(f"no matching brace for {kind}")
    new_block = build_block(KIND_TO_DIR[kind]).rstrip("\n")
    return text[:start] + new_block + text[end:]


if __name__ == "__main__":
    text = open(GLUE, encoding="utf-8").read()
    for d in sys.argv[1:]:
        kind = DIR_TO_KIND[d]
        text = replace_block(text, kind)
        print("rewrote glue for", d)
    open(GLUE, "w", encoding="utf-8").write(text)
