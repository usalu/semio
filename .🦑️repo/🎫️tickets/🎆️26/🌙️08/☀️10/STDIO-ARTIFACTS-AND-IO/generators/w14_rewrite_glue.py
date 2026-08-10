#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""W14: rewrite one domain artifact's `pub mod <kind> { ... }` block in its OWN plugin's glue.rs
to the new standards/subsets tree (uniform standard 🔖️1 / subset ✳️any). Generalizes
w12_rewrite_glue.py for domain plugins (each with its own glue.rs file).

Usage: python3 w14_rewrite_glue.py <plugin_dir> <artifact_dir> [<artifact_dir> ...]
"""
import json
import os
import re
import subprocess
import sys

REPO = "/Users/ueli/Documents/semio"
PLUGINS = os.path.join(REPO, "✏️s/🔌️plugins")
HERE = os.path.dirname(os.path.abspath(__file__))

with open(os.path.join(HERE, "w9_standards_table.json"), encoding="utf-8") as f:
    STANDARDS = json.load(f)["stdio"]
with open(os.path.join(HERE, "w9_owner_table_v2.json"), encoding="utf-8") as f:
    OWNER_V2 = json.load(f)
KIND_TO_DIR = {k: v["dir"] for k, v in OWNER_V2["stdio_roster"].items()}
DIR_TO_KIND = {v: k for k, v in KIND_TO_DIR.items()}

STD_DIR = "🔖️1"
STD_MOD = "v1"


def glue_path(plugin_dir):
    return os.path.join(PLUGINS, plugin_dir, "📦️packages/🦀️rust/📦️glue.rs")


def art_dir_of(plugin_dir, kind_mod):
    """Find the artifact's on-disk dir name from the glue's #[path] for its root component.rs."""
    text = open(glue_path(plugin_dir), encoding="utf-8").read()
    pat = re.compile(r'pub mod ' + re.escape(kind_mod) + r' \{\s*#\[path = "\.\./\.\./🗿️artifacts/([^"]+)/🦀️component\.rs"\]')
    m = pat.search(text)
    if not m:
        raise SystemExit(f"could not find artifact dir for {kind_mod} in {glue_path(plugin_dir)}")
    return m.group(1)


def artifact_name_from_root(plugin_dir, art_dir):
    root_rs = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir, "🦀️component.rs")
    text = open(root_rs, encoding="utf-8").read()
    m = re.search(r"schema::snapshot::(\w+)Snapshot", text)
    return m.group(1)


def leaf(art_rel_path, filename="🦀️component.rs"):
    return f"../../🗿️artifacts/{art_rel_path}/{filename}"


def has(art_root, rel_path):
    return os.path.exists(os.path.join(art_root, rel_path))


def scan_io_targets(art_root, direction, child):
    base = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🚪️io", direction, child, "🗿️artifacts")
    if not os.path.isdir(base):
        return []
    return sorted(d for d in os.listdir(base) if os.path.isdir(os.path.join(base, d)))


def rust_ident_from_slug_dir(dir_name):
    """'📄set-snapshot' / '✏️set-pencil-width' -> 'set_snapshot' / 'set_pencil_width'
    (strip leading emoji, VS16 optional -- these mutation dirs are inconsistently missing it)."""
    m = re.match(r"^\W*([a-z0-9][a-z0-9-]*)$", dir_name)
    if not m:
        # fallback: strip any non-ascii-alnum-hyphen prefix
        stripped = re.sub(r"^[^a-z0-9]+", "", dir_name)
        m = re.match(r"^([a-z0-9][a-z0-9-]*)$", stripped)
    if not m:
        raise SystemExit(f"could not derive rust ident from mutation dir {dir_name!r}")
    return m.group(1).replace("-", "_")


def list_mutation_dirs(abs_mutations_dir):
    if not os.path.isdir(abs_mutations_dir):
        return []
    skip = {"📝️text", "💾️binary"}
    out = []
    for d in sorted(os.listdir(abs_mutations_dir)):
        full = os.path.join(abs_mutations_dir, d)
        if not os.path.isdir(full) or d in skip:
            continue
        out.append(d)
    return out


def build_schema_tree(art_root, art_rel, indent):
    I = "    " * indent
    out = []
    out.append(f'{I}#[path = "."]\n{I}pub mod schema {{\n')
    out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🧬️schema")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for child, emoji_child in (("snapshot", "📸️snapshot"), ("diff", "🔺️diff")):
        out.append(f'{I}    #[path = "."]\n{I}    pub mod {child} {{\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}")}"]\n{I}        mod component;\n{I}        pub use component::*;\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}/📝️text")}"]\n{I}        pub mod text;\n')
        if child == "diff":
            # Some plugins' diff-function callers reference `schema::diff::diff_*` directly
            # (pre-migration convenience re-export) rather than `schema::diff::text::diff_*`.
            out.append(f'{I}        pub use text::*;\n')
        out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/{emoji_child}/💾️binary")}"]\n{I}        pub mod binary;\n')
        out.append(f'{I}    }}\n')
    out.append(f'{I}    #[path = "."]\n{I}    pub mod mutations {{\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations")}"]\n{I}        mod component;\n{I}        pub use component::*;\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/📝️text")}"]\n{I}        pub mod text;\n')
    out.append(f'{I}        #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/💾️binary")}"]\n{I}        pub mod binary;\n')
    mutations_abs = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🧬️schema", "🧬️mutations")
    for mdir in list_mutation_dirs(mutations_abs):
        ident = rust_ident_from_slug_dir(mdir)
        mdir_abs = os.path.join(mutations_abs, mdir)
        out.append(f'{I}        #[path = "."]\n{I}        pub mod {ident} {{\n')
        for leaf_name, emoji_leaf in (("mutation", "🦠️mutation"), ("diff", "🔺️diff"), ("inverse", "↩️inverse")):
            if not os.path.isdir(os.path.join(mdir_abs, emoji_leaf)):
                continue  # not every mutation has a complete mutation/diff/inverse triad
            out.append(f'{I}            #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/{mdir}/{emoji_leaf}")}"]\n{I}            pub mod {leaf_name};\n')
        out.append(f'{I}        }}\n')
    out.append(f'{I}    }}\n{I}}}\n')
    return "".join(out)


def build_io_tree(art_root, art_rel, indent):
    I = "    " * indent
    out = []
    out.append(f'{I}#[path = "."]\n{I}pub mod io {{\n')
    out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🚪️io")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for direction, child in (("📥️import", "🧩️deserializers"), ("📤️export", "🧵️serializers")):
        mod_direction = {"📥️import": "import", "📤️export": "export"}[direction]
        mod_child = {"🧩️deserializers": "deserializers", "🧵️serializers": "serializers"}[child]
        targets = scan_io_targets(art_root, direction, child)
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


def build_engine_mount(art_root, art_dir, indent):
    """⚙️engine is usually a single file, but some domain artifacts' engines are a directory with
    their own sub-facets (e.g. lowpoly's 🎨️paint/, 🧵️media/) -- mount those too when present."""
    I = "    " * indent
    engine_abs = os.path.join(art_root, "🏅️standards", STD_DIR, "⚙️engine")
    subdirs = sorted(
        d for d in os.listdir(engine_abs)
        if os.path.isdir(os.path.join(engine_abs, d))
    ) if os.path.isdir(engine_abs) else []
    if not subdirs:
        return f'{I}#[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine")}"]\n{I}pub mod engine;\n'
    out = [f'{I}#[path = "."]\n{I}pub mod engine {{\n']
    out.append(f'{I}    #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for sd in subdirs:
        ident = rust_ident_from_slug_dir(sd)
        out.append(f'{I}    #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine/{sd}")}"]\n{I}    pub mod {ident};\n')
    out.append(f'{I}}}\n')
    return "".join(out)


def build_block(plugin_dir, kind_mod, art_dir):
    art_root = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir)
    subset_art_rel = f"{art_dir}/🏅️standards/{STD_DIR}/🪆️subsets/✳️any"

    lines = []
    lines.append(f'    pub mod {kind_mod} {{\n')
    lines.append(f'        #[path = "{leaf(art_dir)}"]\n        mod component;\n        pub use component::*;\n\n')

    lines.append(f'        #[path = "."]\n        pub mod standards {{\n')
    lines.append(f'            #[path = "."]\n            pub mod {STD_MOD} {{\n')
    lines.append(build_engine_mount(art_root, art_dir, 4))
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🏗️builder")}"]\n                pub mod builder;\n')
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🧐️analyzer")}"]\n                pub mod analyzer;\n')
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🎹️composer")}"]\n                pub mod composer;\n')
    lines.append(f'                #[path = "."]\n                pub mod subsets {{\n')
    lines.append(f'                    #[path = "."]\n                    pub mod any {{\n')
    lines.append(build_schema_tree(art_root, subset_art_rel, 6))
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🏗️builder")}"]\n                        pub mod builder;\n')
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🧐️analyzer")}"]\n                        pub mod analyzer;\n')
    lines.append(f'                        #[path = "{leaf(f"{subset_art_rel}/🎹️composer")}"]\n                        pub mod composer;\n')
    if has(art_root, f"🏅️standards/{STD_DIR}/🪆️subsets/✳️any/🚪️io"):
        lines.append(build_io_tree(art_root, subset_art_rel, 6))
    lines.append(f'                    }}\n                }}\n')
    lines.append(f'            }}\n        }}\n\n')

    lines.append('        // ---- Shims: keep pre-migration module paths resolving for external callers ----\n')
    lines.append(f'        pub mod schema {{\n            pub use super::standards::{STD_MOD}::subsets::any::schema::*;\n        }}\n')
    lines.append(f'        pub mod engine {{\n            pub use super::standards::{STD_MOD}::engine::*;\n        }}\n')
    if has(art_root, f"🏅️standards/{STD_DIR}/🪆️subsets/✳️any/🚪️io"):
        lines.append(f'        pub mod io {{\n            pub use super::standards::{STD_MOD}::subsets::any::io::*;\n        }}\n')
    for shim_line in extract_legacy_shims(plugin_dir, kind_mod, art_dir, kind_mod):
        lines.append(shim_line + "\n")
    lines.append('\n')

    lines.append(f'        #[path = "{leaf(f"{art_dir}/🏗️builder")}"]\n        pub mod builder;\n')
    lines.append(f'        #[path = "{leaf(f"{art_dir}/🧐️analyzer")}"]\n        pub mod analyzer;\n')
    lines.append(f'        #[path = "{leaf(f"{art_dir}/🎹️composer")}"]\n        pub mod composer;\n\n')

    if has(art_root, "📚️examples/🎬️demo"):
        lines.append(f'        #[path = "."]\n        pub mod examples {{\n            #[path = "."]\n            pub mod demo {{\n')
        lines.append(f'                #[path = "{leaf(f"{art_dir}/📚️examples/🎬️demo")}"]\n                mod component;\n                pub use component::*;\n')
        lines.append(f'            }}\n        }}\n')

    lines.append('    }\n')
    return "".join(lines)


def replace_block(text, kind_mod, block):
    marker = f"    pub mod {kind_mod} {{\n"
    if marker not in text:
        raise SystemExit(f"marker not found for {kind_mod}: {marker!r}")
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
        raise SystemExit(f"no matching brace for {kind_mod}")
    return text[:start] + block.rstrip("\n") + text[end:]


# Known structural facets that are NOT legacy pass-through shims -- never touch these even if a
# future artifact happens to declare one (defensive allow-list). Everything else that has no
# #[path] attribute anywhere in its body (i.e. carries no real file, purely re-exports) is treated
# as legacy shim residue and preserved with its `schema::` targets redirected.
LEGACY_SHIM_EXCLUDE = {"builder", "analyzer", "decomposer", "composer", "engine", "schema", "io", "examples", "standards"}


def find_matching_brace(text, brace_start):
    depth = 0
    for idx in range(brace_start, len(text)):
        c = text[idx]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return idx + 1
    return None


def top_level_children(block_text):
    """Yield (name, child_text) for each `pub mod <name> { ... }` directly inside block_text's
    outermost braces (one level of nesting only). Must skip past each matched span before
    continuing the scan, or regex finditer will also "match" nested mods (e.g. an inner `pack`
    living inside an outer `snapshot { ... }`) as if they were separate top-level siblings."""
    brace_start = block_text.index("{")
    body_start = brace_start + 1
    body_end = find_matching_brace(block_text, brace_start) - 1
    body = block_text[body_start:body_end]
    pattern = re.compile(r"pub mod (\w+)\s*\{")
    pos = 0
    while True:
        m = pattern.search(body, pos)
        if not m:
            break
        cb = body.index("{", m.start())
        ce = find_matching_brace(body, cb)
        if ce is None:
            pos = m.end()
            continue
        pos = ce  # resume scanning AFTER this whole block, not inside it
        yield m.group(1), body[m.start():ce]


def extract_legacy_shims(plugin_dir, kind_mod, art_dir, kind):
    """Read the artifact's block as last committed (git HEAD) and preserve any legacy pass-through
    shim modules -- single-line (`pub mod op { pub use ...; }`) or multi-line/nested (`pub mod
    snapshot { pub mod schema {...} pub mod pack {...} }`) -- residue from an earlier
    dsl/pack/op/spr/diff/mutations -> schema/{snapshot,diff,mutations} migration that some
    plugins' app-layer code still references directly. A child counts as a shim if its own body
    has no `#[path` anywhere (i.e. it carries no real file, purely re-exports/nests other shims).
    Their `schema::` targets get redirected to the new subset-relative location; harmless no-op
    if none exist."""
    gp = glue_path(plugin_dir)
    try:
        head_text = subprocess.run(
            ["git", "show", f"HEAD:{os.path.relpath(gp, REPO)}"],
            cwd=REPO, capture_output=True, text=True, check=True,
        ).stdout
    except subprocess.CalledProcessError:
        return []
    marker = f"    pub mod {kind_mod} {{\n"
    if marker not in head_text:
        return []
    start = head_text.index(marker)
    end = find_matching_brace(head_text, head_text.index("{", start))
    if end is None:
        return []
    old_block = head_text[start:end]
    old_prefix = f"crate::artifacts::{kind}::schema::"
    new_prefix = f"crate::artifacts::{kind}::standards::v1::subsets::any::schema::"
    shims = []
    for name, child_text in top_level_children(old_block):
        if name in LEGACY_SHIM_EXCLUDE or "#[path" in child_text:
            continue
        shims.append("        " + child_text.replace(old_prefix, new_prefix))
    # Bare top-level re-exports (no `pub mod` wrapper at all) directly in the glue file, e.g.
    # `pub use crate::artifacts::flow::schema::diff::FlowDiff;` sitting right after `mod
    # component; pub use component::*;` -- some plugins' root component.rs never re-exports
    # Diff/Mutation itself, relying entirely on the glue doing it. Only scan the header region
    # before the first `pub mod` to avoid double-capturing lines already inside a shim block above.
    header_end_match = re.search(r"pub mod \w+\s*\{", old_block)
    header_region = old_block[:header_end_match.start()] if header_end_match else old_block
    for m in re.finditer(r"pub use " + re.escape(old_prefix) + r"[\w:]*;", header_region):
        shims.append("        " + m.group(0).replace(old_prefix, new_prefix))
    return shims


if __name__ == "__main__":
    plugin_dir = sys.argv[1]
    gp = glue_path(plugin_dir)
    text = open(gp, encoding="utf-8").read()
    for kind_mod in sys.argv[2:]:
        art_dir = art_dir_of(plugin_dir, kind_mod)
        block = build_block(plugin_dir, kind_mod, art_dir)
        text = replace_block(text, kind_mod, block)
        print("rewrote glue for", plugin_dir, kind_mod)
    open(gp, "w", encoding="utf-8").write(text)
