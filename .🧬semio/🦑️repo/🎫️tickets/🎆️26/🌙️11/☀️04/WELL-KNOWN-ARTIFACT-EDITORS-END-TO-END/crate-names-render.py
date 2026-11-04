#!/usr/bin/env python3
# 📦️ Renders 📦️crate-names.md from the TSV index: owning cargo workspace (nearest [workspace] ancestor) and membership by TOML read only.
import fnmatch
import os
import sys
import tomllib

ROOT = "/home/deck/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END")
TSV = os.path.join(TICKET, "🗑️generated/crate-names-index.tsv")
OUT = os.path.join(TICKET, "📦️crate-names.md")
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

_ws_cache = {}


def ancestors_with_workspace(dir_rel):
    """Every ancestor directory (nearest first, relative to ROOT, the crate's own dir included) whose Cargo.toml declares [workspace]."""
    found = []
    current = dir_rel
    while True:
        if current not in _ws_cache:
            manifest = os.path.join(ROOT, current, "Cargo.toml")
            ws = None
            if os.path.isfile(manifest):
                with open(manifest, "rb") as handle:
                    data = tomllib.load(handle)
                ws = data.get("workspace")
            _ws_cache[current] = ws
        if _ws_cache[current] is not None:
            found.append((current, _ws_cache[current]))
        if current in ("", "."):
            return found
        current = os.path.dirname(current)


def owner_of(directory):
    """First ancestor workspace that lists the crate and does not exclude it (the one `cargo -p` resolves from)."""
    for ws_dir, ws in ancestors_with_workspace(directory):
        if is_member(ws_dir, ws, directory):
            return ws_dir or ".", "yes"
    return "(none)", "no"


def is_member(ws_dir, ws, crate_dir):
    rel = os.path.relpath(crate_dir, ws_dir) if ws_dir else crate_dir
    excluded = any(fnmatch.fnmatch(rel, pat.rstrip("/")) or rel == pat.rstrip("/") for pat in ws.get("exclude", []))
    members = [m.rstrip("/") for m in ws.get("members", [])]
    included = any(fnmatch.fnmatch(rel, m) or rel == m for m in members)
    return included and not excluded


ABBREVIATIONS = [
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/", "stdio/"),
    ("✏️s/🔌️plugins/🗄️stdio/", "stdio-root/"),
    ("🌎️hub/🧩️compositions/🗄️stdio/", "hub/"),
    ("✏️s/🔌️plugins/🔋️energy/", "energy/"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/", "os/"),
    ("📦️packages/🦀️rust", "rust"),
]


def short(path):
    if path in ("", "."):
        return "."
    for full, alias in ABBREVIATIONS:
        if path.startswith(full):
            return alias + path[len(full):]
    return path


def role_label(role, dir_rel):
    if role == "other" and dir_rel.endswith("📦️packages/🦀️rust") and dir_rel.count("/") <= 6:
        return "root"
    return role


EXTRA = [
    ("os-tests", "test", "semio-framework-plugin-tests-07", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍇️tests-07/📦️packages/🦀️rust"),
    ("wgpu-shell", "test", "semio-framework-os-renderer-wgpu-shell-tests-docs-2", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🥪️shell-tests-docs-2/📦️packages/🦀️rust"),
]

CORRECTIONS = [
    "`semio-s-artifact-stdio-zip-editor` EXISTS (`stdio/🎒️zip/✏️editor/📦️packages/🦀️rust`). It is a member of the zip artifact's own workspace (`stdio/🎒️zip/Cargo.toml`), which the root workspace excludes. Run it from that workspace, not from the repo root: `cd <stdio/🎒️zip> && cargo test -p semio-s-artifact-stdio-zip-editor`. The earlier `did not match any packages` error came from the root-level selection. [TOML and workspace read]",
    "There is no `semio-s-artifact-stdio-obj-editor` package. The obj root is `semio-s-artifact-stdio-obj` (`stdio/🗽️obj/📦️packages/🦀️rust`), owned by the obj artifact workspace (`stdio/🗽️obj/Cargo.toml`), which the root workspace excludes. The obj subset `✏️editor` directory has no Cargo.toml, so no editor package exists for obj. The earlier `e-obj` run (`cargo test -p semio-s-artifact-stdio-obj editor::` from the repo root) failed for the same workspace reason: the name exists, but the root selection does not see it. Re-queue from the obj artifact workspace.",
    "`natural-file-lifecycle` fixture: JSON at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json`. The Rust test is `🔌️plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs`, mounted by the companion crate `semio-framework-plugin-tests-07` via `#[path]`. The OS plugin tests crates listed under `os-plugin-tests` are other suites.",
    "wgpu Shell relay test `🔀️wgpu-document-relay/🦀️.rs` is mounted by the companion crate `semio-framework-os-renderer-wgpu-shell-tests-docs-2` (`🥪️shell-tests-docs-2`, `#[path]` in its lib `🥪️shell-tests-docs-2/🦀️.rs`). The Shell directory itself has no Cargo.toml.",
    "Energy model editor: `semio-s-artifact-energy-model-editor` (`energy/🗿️artifacts/🔋️model/.../✏️editor/📦️packages/🦀️rust`), owned by the energy model artifact workspace.",
    "OS io: `semio-framework-os-io` (`os/🚪️io/📦️packages/🦀️rust`), owned by the repo-root workspace. OS semio: `semio-framework-os-semio` (`os/🧬️semio/📦️packages/🦀️rust`), owned by the repo-root workspace.",
]


def main():
    rows = []
    with open(TSV, encoding="utf-8") as tsv:
        next(tsv)
        for line in tsv:
            group, role, name, directory, workspace = line.rstrip("\n").split("\t")
            if name == "(workspace-only)":
                continue
            if group == "energy" and not directory.startswith("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model"):
                continue
            owner, member = owner_of(directory)
            rows.append((group, role_label(role, directory), name, short(directory), short(owner), member))
    for group, role, name, directory in EXTRA:
        owner, member = owner_of(directory)
        rows.append((group, role, name, short(directory), short(owner), member))
    order = {"root": 0, "editor": 1, "io": 2, "snapshot": 3, "mutations": 4, "schema": 5, "oracle": 6, "test": 7, "other": 8}
    lines = []
    lines.append("# 📦️ Cargo Package Names: Well-Known Stdio Artifacts, Hub, OS and Energy")
    lines.append("")
    lines.append("Generated from `Cargo.toml` `[package] name` only (TOML read; no cargo run). Index: `🗑️generated/crate-names-index.tsv`; renderer: `crate-names-render.py` (both in this ticket). Owner is the nearest ancestor `Cargo.toml` with `[workspace]` that lists the crate (ancestors are checked nearest first, so the repo-root workspace can own OS crates). `cargo -p <name>` only resolves from that owner directory (or with `--manifest-path`); the root workspace excludes artifact-level workspaces such as `🗿️artifacts/🎒️zip` and `🗿️artifacts/🗽️obj`, so the root-level command fails for those crates.")
    lines.append("")
    lines.append("Path abbreviations: `stdio/` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, `stdio-root/` = `✏️s/🔌️plugins/🗄️stdio/`, `hub/` = `🌎️hub/🧩️compositions/🗄️stdio/`, `energy/` = `✏️s/🔌️plugins/🔋️energy/`, `os/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`, `rust` = `📦️packages/🦀️rust`. The energy section lists only the model artifact (`energy/🗿️artifacts/🔋️model`); other energy plugin crates are omitted.")
    lines.append("")
    lines.append("Member = `yes` when the owning workspace lists the crate (and does not exclude it). `no` means `cargo -p` from that owner will not find it; an owner-less crate runs only with `--manifest-path`.")
    lines.append("")
    lines.append("## Corrections to the fleet names (verified by TOML and workspace read)")
    lines.append("")
    for item in CORRECTIONS:
        lines.append(f"- {item}")
    lines.append("")
    lines.append("## Summary by artifact (stdio)")
    lines.append("")
    lines.append("Columns: root crate(s), editor crate(s), io / snapshot / mutations crate(s), test crate(s). `-` means none in the index.")
    lines.append("")
    lines.append("| Artifact | Root | Editor | IO / snapshot / mutations | Tests |")
    lines.append("|---|---|---|---|---|")
    stdio_groups = sorted({r[0] for r in rows if r[3].startswith("stdio/") or r[0] in ()} | {r[0] for r in rows if r[3].startswith("stdio/")})
    for group in stdio_groups:
        items = [r for r in rows if r[0] == group]
        def names(roles):
            picked = sorted({r[2] for r in items if r[1] in roles})
            return ", ".join(f"`{n}`" for n in picked) if picked else "-"
        lines.append(f"| {group} | {names(['root'])} | {names(['editor'])} | {names(['io','snapshot','mutations'])} | {names(['test'])} |")
    lines.append("")
    groups = {}
    for row in rows:
        groups.setdefault(row[0], []).append(row)
    sections = sorted(groups.keys(), key=lambda g: (g.startswith("🗿") or g.startswith("🗽") or False, g))
    for group in sections:
        items = sorted(groups[group], key=lambda r: (order.get(r[1], 9), r[2]))
        title = group
        lines.append(f"## {title}")
        lines.append("")
        lines.append("| Role | Package | Directory (relative to repo root) | Owning workspace | Member |")
        lines.append("|---|---|---|---|---|")
        for r in items:
            lines.append(f"| {r[1]} | `{r[2]}` | `{r[3]}` | `{r[4]}` | {r[5]} |")
        lines.append("")
    with open(OUT, "w", encoding="utf-8") as out:
        out.write("\n".join(lines) + "\n")
    print(len(rows), "rows", file=sys.stderr)


if __name__ == "__main__":
    main()
