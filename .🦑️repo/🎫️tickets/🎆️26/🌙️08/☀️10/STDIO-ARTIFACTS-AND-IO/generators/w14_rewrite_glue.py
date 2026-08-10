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
# 🚨️ This repo runs a background auto-commit process on the shared working tree, so `HEAD` is a
# MOVING target that can already include this session's own (in-progress, sometimes still-buggy)
# edits -- reading "HEAD" for "the pre-migration original" is unsafe once auto-commit has fired.
# Pinned instead to 678a50d6c5, the last commit shown in this session's initial `gitStatus`
# snapshot (confirmed via `git log` to be the exact pre-session boundary commit).
PRE_SESSION_COMMIT = "678a50d6c5"

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


CORE_TRIAD_FACETS = {"snapshot", "diff", "mutations"}


def list_extra_schema_facets(schema_abs):
    """Some domain artifacts' schema has sibling facets beyond the snapshot/diff/mutations triad
    (e.g. program/architect's 🧱️kernel + 🗄️registers: shared entity/enum types the triad's own
    files import). Discover any such top-level schema subdirectory that carries its own
    🦀️component.rs so build_schema_tree can mount it structurally instead of silently dropping it
    (a dropped facet breaks every leaf file across the plugin that references it -- see cad/program
    incident in this ticket's STATUS.md)."""
    if not os.path.isdir(schema_abs):
        return []
    skip = {"📸️snapshot", "🔺️diff", "🧬️mutations"}
    out = []
    for d in sorted(os.listdir(schema_abs)):
        if d in skip:
            continue
        full = os.path.join(schema_abs, d)
        if os.path.isdir(full) and os.path.isfile(os.path.join(full, "🦀️component.rs")):
            out.append(d)
    return out


def build_schema_tree(art_root, art_rel, indent):
    I = "    " * indent
    out = []
    out.append(f'{I}#[path = "."]\n{I}pub mod schema {{\n')
    out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🧬️schema")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    schema_abs = os.path.join(art_root, "🏅️standards", STD_DIR, "🪆️subsets", "✳️any", "🧬️schema")
    extra_facets = list_extra_schema_facets(schema_abs)
    extra_idents = []
    for extra_dir in extra_facets:
        ident = rust_ident_from_slug_dir(extra_dir)
        extra_idents.append(ident)
        out.append(f'{I}    #[path = "{leaf(f"{art_rel}/🧬️schema/{extra_dir}")}"]\n{I}    pub mod {ident};\n')
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
            # Check the FILE, not just the directory -- some triad dirs exist but are empty.
            if not os.path.isfile(os.path.join(mdir_abs, emoji_leaf, "🦀️component.rs")):
                continue  # not every mutation has a complete mutation/diff/inverse triad
            out.append(f'{I}            #[path = "{leaf(f"{art_rel}/🧬️schema/🧬️mutations/{mdir}/{emoji_leaf}")}"]\n{I}            pub mod {leaf_name};\n')
        out.append(f'{I}        }}\n')
    out.append(f'{I}    }}\n{I}}}\n')
    return "".join(out), extra_idents


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


def emit_engine_subtree(abs_dir, leaf_rel, indent):
    """Mount one ⚙️engine subdir, recursing into ITS OWN subdirectories that carry a
    🦀️component.rs (e.g. puzzle's engine/geometry/flatten/ -- a facet nested two levels deep, not
    just the one level lowpoly's paint/media already covered). A leaf-flat directory (no further
    nested facets) still gets the simple single #[path] form; only directories that actually need
    a body get the brace-nested `mod component; pub use component::*;` + child mounts form."""
    I = "    " * indent
    children = sorted(
        d for d in os.listdir(abs_dir)
        if os.path.isdir(os.path.join(abs_dir, d)) and os.path.isfile(os.path.join(abs_dir, d, "🦀️component.rs"))
    )
    if not children:
        return None  # caller emits the simple flat form
    out = [f'{I}#[path = "."]\n{I}pub mod {{ident}} {{\n']
    out.append(f'{I}    #[path = "{leaf(leaf_rel)}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for c in children:
        c_ident = rust_ident_from_slug_dir(c)
        c_abs = os.path.join(abs_dir, c)
        c_leaf_rel = f"{leaf_rel}/{c}"
        nested = emit_engine_subtree(c_abs, c_leaf_rel, indent + 1)
        if nested:
            out.append(nested.replace("{ident}", c_ident, 1))
        else:
            out.append(f'{I}    #[path = "{leaf(c_leaf_rel)}"]\n{I}    pub mod {c_ident};\n')
    out.append(f'{I}}}\n')
    return "".join(out)


def build_engine_mount(art_root, art_dir, indent, plugin_dir, kind_mod):
    """⚙️engine is usually a single file, but some domain artifacts' engines are a directory with
    their own sub-facets (e.g. lowpoly's 🎨️paint/, 🧵️media/), occasionally nested two levels deep
    (puzzle's engine/geometry/flatten/) -- mount those too when present."""
    I = "    " * indent
    engine_abs = os.path.join(art_root, "🏅️standards", STD_DIR, "⚙️engine")
    subdirs = sorted(
        d for d in os.listdir(engine_abs)
        # Skip genuinely empty subdirs (no component.rs anywhere in the subtree) -- confirmed one
        # such stray empty dir (process3d's engine/catalogs/) that isn't even in the pre-session
        # commit's tracked tree (git doesn't track empty dirs), so it was always dead cruft, not
        # something the move step should have populated. Blindly mounting it as `pub mod X;`
        # produces a "couldn't read <path>: No such file" hard error.
        if os.path.isdir(os.path.join(engine_abs, d)) and any(
            fn == "🦀️component.rs" for _r, _dirs, files in os.walk(os.path.join(engine_abs, d)) for fn in files
        )
    ) if os.path.isdir(engine_abs) else []
    engine_idents = {rust_ident_from_slug_dir(sd) for sd in subdirs}
    engine_shims = extract_engine_legacy_shims(plugin_dir, kind_mod, engine_idents)
    if not subdirs and not engine_shims:
        return f'{I}#[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine")}"]\n{I}pub mod engine;\n'
    out = [f'{I}#[path = "."]\n{I}pub mod engine {{\n']
    out.append(f'{I}    #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine")}"]\n{I}    mod component;\n{I}    pub use component::*;\n')
    for sd in subdirs:
        ident = rust_ident_from_slug_dir(sd)
        sd_abs = os.path.join(engine_abs, sd)
        sd_leaf_rel = f"{art_dir}/🏅️standards/{STD_DIR}/⚙️engine/{sd}"
        nested = emit_engine_subtree(sd_abs, sd_leaf_rel, indent + 1)
        if nested:
            out.append(nested.replace("{ident}", ident, 1))
        else:
            out.append(f'{I}    #[path = "{leaf(sd_leaf_rel)}"]\n{I}    pub mod {ident};\n')
    for shim_text in engine_shims:
        out.append(f'{I}    {shim_text}\n')
    out.append(f'{I}}}\n')
    return "".join(out)


def extract_engine_legacy_shims(plugin_dir, kind_mod, known_engine_idents):
    """Some artifacts' pre-migration ⚙️engine block carries an extra pure re-export namespace
    alongside the flat per-file mounts -- e.g. animate/present's `pub mod animate { pub mod
    sobject { pub use super::super::scene::sobject::*; } ... }`, a convenience alias tree used
    pervasively by the engine's own code (`engine::animate::sobject::Sobject`). It has NO
    `#[path]` anywhere (pure `super::`-relative re-exports of sibling flat mounts, verified) so it
    needs no path redirection at all -- the flat sibling names it references are unchanged, just
    moved one level deeper as a whole subtree. Preserved verbatim; dropping it silently breaks
    every reference across the plugin (see animate/present incident, this ticket's STATUS.md)."""
    gp = glue_path(plugin_dir)
    try:
        head_text = subprocess.run(
            ["git", "show", f"{PRE_SESSION_COMMIT}:{os.path.relpath(gp, REPO)}"],
            cwd=REPO, capture_output=True, text=True, check=True,
        ).stdout
    except subprocess.CalledProcessError:
        return []
    art_marker = f"    pub mod {kind_mod} {{\n"
    if art_marker not in head_text:
        return []
    art_start = head_text.index(art_marker)
    art_end = find_matching_brace(head_text, head_text.index("{", art_start))
    if art_end is None:
        return []
    art_block = head_text[art_start:art_end]
    engine_marker = re.search(r"pub mod engine\s*\{", art_block)
    if not engine_marker:
        return []
    engine_start = engine_marker.start()
    engine_end = find_matching_brace(art_block, art_block.index("{", engine_start))
    if engine_end is None:
        return []
    engine_block = art_block[engine_start:engine_end]
    shims = []
    for name, child_text in top_level_children(engine_block):
        if name in known_engine_idents or name == "component":
            continue
        if "#[path" in child_text:
            # A #[path]-backed mount under a CUSTOM name that doesn't match our directory-derived
            # ident (e.g. animate/present's 🎥️video mounted as `pub mod animate_video;` instead of
            # the naive `video`) -- same physical file our scan already mounts under its derived
            # name, just needs an additional alias, not a second #[path] (would duplicate-mount).
            path_m = re.search(r'#\[path = "[^"]*/([^/"]+)/🦀️component\.rs"\]', child_text)
            if path_m:
                mapped_ident = rust_ident_from_slug_dir(path_m.group(1))
                if mapped_ident in known_engine_idents:
                    shims.append(f'pub use {mapped_ident} as {name};')
            continue
        shims.append(child_text)
    # Bare `pub mod X;` (semicolon-terminated, no brace body -- single-file mounts) preceded by
    # their own #[path], under a CUSTOM name that doesn't match our directory-derived ident --
    # top_level_children's brace-seeking regex never sees these at all (see animate/present's
    # `pub mod animate_video;` mounting 🎥️video/component.rs, while our scan derives `video` from
    # the same directory). Same fix as the braced case: alias, not a second #[path] mount.
    for m in re.finditer(r'#\[path = "([^"]*/([^/"]+)/🦀️component\.rs)"\]\s*pub mod (\w+);', engine_block):
        dir_name, name = m.group(2), m.group(3)
        if name in known_engine_idents or name == "component":
            continue
        mapped_ident = rust_ident_from_slug_dir(dir_name)
        if mapped_ident in known_engine_idents and mapped_ident != name:
            shims.append(f'pub use {mapped_ident} as {name};')
    return shims


def build_block(plugin_dir, kind_mod, art_dir):
    art_root = os.path.join(PLUGINS, plugin_dir, "🗿️artifacts", art_dir)
    subset_art_rel = f"{art_dir}/🏅️standards/{STD_DIR}/🪆️subsets/✳️any"

    lines = []
    lines.append(f'    pub mod {kind_mod} {{\n')
    lines.append(f'        #[path = "{leaf(art_dir)}"]\n        mod component;\n        pub use component::*;\n\n')

    lines.append(f'        #[path = "."]\n        pub mod standards {{\n')
    lines.append(f'            #[path = "."]\n            pub mod {STD_MOD} {{\n')
    lines.append(build_engine_mount(art_root, art_dir, 4, plugin_dir, kind_mod))
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🏗️builder")}"]\n                pub mod builder;\n')
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🧐️analyzer")}"]\n                pub mod analyzer;\n')
    lines.append(f'                #[path = "{leaf(f"{art_dir}/🏅️standards/{STD_DIR}/🎹️composer")}"]\n                pub mod composer;\n')
    lines.append(f'                #[path = "."]\n                pub mod subsets {{\n')
    lines.append(f'                    #[path = "."]\n                    pub mod any {{\n')
    schema_tree_text, extra_schema_facets = build_schema_tree(art_root, subset_art_rel, 6)
    lines.append(schema_tree_text)
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
    structural_facet_names = CORE_TRIAD_FACETS | set(extra_schema_facets)
    for shim_line in extract_legacy_shims(plugin_dir, kind_mod, art_dir, kind_mod, structural_facet_names):
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


def extract_legacy_shims(plugin_dir, kind_mod, art_dir, kind, structural_facet_names=frozenset()):
    """Read the artifact's block as last committed (git HEAD) and preserve any legacy pass-through
    shim modules -- single-line (`pub mod op { pub use ...; }`) or multi-line/nested (`pub mod
    snapshot { pub mod schema {...} pub mod pack {...} }`) -- residue from an earlier
    dsl/pack/op/spr/diff/mutations -> schema/{snapshot,diff,mutations} migration that some
    plugins' app-layer code still references directly. A child counts as a shim if its own body
    has no `#[path` anywhere (i.e. it carries no real file, purely re-exports/nests other shims).
    Their `schema::` targets get redirected to the new subset-relative location; harmless no-op
    if none exist.

    `structural_facet_names` (typically {"snapshot","diff","mutations"} plus any extra schema
    siblings like program's kernel/registers) are names build_schema_tree ALWAYS mounts as real
    #[path] leaves under schema::. For plugins whose PRE-migration shape had these same names as
    the PRIMARY content (i.e. schema lived directly at crate::artifacts::<kind>::{mutations,diff,
    snapshot,...} before the standards/subsets wrapper existed -- cad's original shape), a naive
    #[path]-preserving re-emit here would mount the identical physical file a SECOND time
    (build_schema_tree already mounts it structurally), producing two non-unified module instances
    of the same type and E0119/E0592/E0308 conflicting-impl errors (see cad incident, this
    ticket's STATUS.md). Any name in this set is therefore ALWAYS forced to a pure `pub use`
    alias of the structural mount, never a physical #[path] remount, regardless of what shape the
    original glue had."""
    gp = glue_path(plugin_dir)
    try:
        head_text = subprocess.run(
            ["git", "show", f"{PRE_SESSION_COMMIT}:{os.path.relpath(gp, REPO)}"],
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
    art_marker = f"🗿️artifacts/{art_dir}/"
    art_marker_new = f"🗿️artifacts/{art_dir}/🏅️standards/{STD_DIR}/🪆️subsets/✳️any/"
    for name, child_text in top_level_children(old_block):
        if name in LEGACY_SHIM_EXCLUDE:
            continue
        if name in structural_facet_names:
            # Always a pure re-export alias of the structural mount -- never a physical remount.
            target = f"crate::artifacts::{kind}::standards::v1::subsets::any::schema::{name}"
            if name in CORE_TRIAD_FACETS:
                # snapshot/diff/mutations pre-migration access patterns vary by plugin -- some
                # call it flat (`snapshot::XSnapshot`), some nested under a `schema`/`pack`/`text`
                # submodule (`snapshot::schema::XSnapshot`, `snapshot::pack::PROTO`, note/draw's
                # own established `diff::text::diff_*` convention). Emit a superset covering every
                # observed shape at once (harmless if a given plugin only ever uses one of them) --
                # a flat-only glob broke space/home's `snapshot::schema::`/`snapshot::pack::`
                # call sites (see this ticket's STATUS.md).
                shims.append(
                    f'        pub mod {name} {{ pub use {target}::*; '
                    f'pub mod schema {{ pub use {target}::*; }} '
                    f'pub mod text {{ pub use {target}::text::*; }} '
                    f'pub mod pack {{ pub use {target}::binary::*; }} '
                    f'pub mod binary {{ pub use {target}::binary::*; }} }}'
                )
            else:
                # Extra schema-sibling facets (e.g. program's kernel/registers) are simple
                # single-file mounts with no text/binary substructure -- flat alias only.
                shims.append(f'        pub mod {name} {{ pub use {target}::*; }}')
            continue
        if "#[path" in child_text:
            # A #[path]-backed top-level alias to a facet-internal leaf (e.g. cad's
            # `pub mod op { #[path = "…/🧬️schema/🧬️mutations/📝️text/component.rs"] … }`) --
            # NOT a pure pub-use shim, but still legacy residue naming an old facet path
            # directly. Redirect the path string into the new subset location, same as any
            # other moved leaf. Original glue always has a `#[path = "."]` attribute
            # immediately BEFORE such a block wrapper (cumulative #[path] base reset) --
            # top_level_children's regex starts matching at `pub mod`, so that preceding
            # attribute line is never part of the captured span; restore it here.
            shims.append('        #[path = "."]\n        ' + child_text.replace(art_marker, art_marker_new))
            continue
        shims.append("        " + child_text.replace(old_prefix, new_prefix))
    # Bare top-level re-exports (no `pub mod` wrapper at all) directly in the glue file, e.g.
    # `pub use crate::artifacts::flow::schema::diff::FlowDiff;` sitting right after `mod
    # component; pub use component::*;` -- some plugins' root component.rs never re-exports
    # Diff/Mutation itself, relying entirely on the glue doing it. Only scan the header region
    # before the first `pub mod` to avoid double-capturing lines already inside a shim block above.
    # Search AFTER old_block's own opening brace -- old_block starts with the artifact's own
    # "pub mod <kind_mod> {" wrapper line, which the naive search-from-0 would match first.
    search_from = old_block.index("{") + 1
    header_end_match = re.search(r"pub mod \w+\s*\{", old_block[search_from:])
    if header_end_match:
        header_end_match_start = search_from + header_end_match.start()
    else:
        header_end_match_start = None
    header_region = old_block[:header_end_match_start] if header_end_match_start is not None else old_block
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
