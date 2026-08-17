#!/usr/bin/env python3
"""B1 (round 3): rewrite every path-bearing string that references a moving
crate/package dir, BEFORE the physical move happens. Same category battery as
round 2's rewriter, driven by translate-path-v3.py's longest-prefix crate map."""
import os
import re
import glob
import json
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("translate_path_v3", os.path.join(HERE, "translate-path-v3.py"))
tp = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tp)

SCOPE_ROOTS = ("🧰️framework/", "✏️s/", "🌎️hub/", "♻️mit-bestand/")
EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}

stats = {"files_changed": 0, "strings_changed": 0}


def resolve_old(base_dir, ref):
    if ref.startswith("/"):
        return None
    joined = os.path.normpath(os.path.join(base_dir, ref)).replace(os.sep, "/")
    if joined.startswith("../") or joined == "..":
        return None
    if not joined.startswith(SCOPE_ROOTS):
        return None
    return joined


def rewrite_ref(base_dir, ref, check_exists=True):
    if "${" in ref or "$(" in ref:
        return None
    query = ""
    core = ref
    if "?" in ref:
        core, query = ref.split("?", 1)
        query = "?" + query
    old_abs = resolve_old(base_dir, core)
    if old_abs is None:
        return None
    if check_exists and not os.path.exists(old_abs):
        return None
    new_abs = tp.translate(old_abs)
    if new_abs == old_abs:
        return None
    new_rel = os.path.relpath(new_abs, base_dir)
    if not new_rel.startswith("."):
        new_rel = "./" + new_rel
    return new_rel + query


def process_generic(fpath, pattern, group=1, base_dir_override=None):
    try:
        content = open(fpath, encoding="utf-8").read()
    except Exception:
        return False
    base_dir = base_dir_override if base_dir_override is not None else os.path.dirname(fpath)
    changed = {"v": False}

    def repl(m):
        ref = m.group(group)
        new_ref = rewrite_ref(base_dir, ref)
        if new_ref is None:
            return m.group(0)
        changed["v"] = True
        stats["strings_changed"] += 1
        start, end = m.span(group)
        return m.group(0)[: start - m.start()] + new_ref + m.group(0)[end - m.start() :]

    new_content = pattern.sub(repl, content)
    if changed["v"]:
        open(fpath, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1
    return changed["v"]


def process_repo_root_relative(fpath, pattern, group=1):
    return process_generic(fpath, pattern, group=group, base_dir_override="")


# #region 🦀️Rust
RS_INCLUDE_RE = re.compile(r'((?:include_(?:str|bytes)|include)!\(\s*")([^"]+)(")')
RS_PATH_ATTR_RE = re.compile(r'(#\[path\s*=\s*")([^"]+)(")')


def rust_files():
    for root in ("🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"):
        yield from glob.glob(f"{root}/**/*.rs", recursive=True)


def do_rust():
    for f in rust_files():
        if any(f"/{d}/" in f for d in EXCLUDE_DIRS):
            continue
        process_generic(f, RS_INCLUDE_RE, group=2)
        process_generic(f, RS_PATH_ATTR_RE, group=2)


def do_build_rs_joins():
    """.join("...") chains in build.rs — same pattern used for icon/asset lookups."""
    pattern = re.compile(r'\.join\(\s*"((?:\.\./)*[^"]+)"\s*\)')
    for f in rust_files():
        if os.path.basename(f) != "build.rs":
            continue
        if any(f"/{d}/" in f for d in EXCLUDE_DIRS):
            continue
        process_generic(f, pattern, group=1)


# #endregion 🦀️Rust

# #region 🦀️CargoToml
CARGO_PATH_DEP_RE = re.compile(r'path\s*=\s*"([^"]+)"')


def cargo_files():
    for root in ("🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"):
        yield from glob.glob(f"{root}/**/Cargo.toml", recursive=True)
    yield "Cargo.toml"


def do_cargo():
    for f in cargo_files():
        if not os.path.exists(f):
            continue
        process_generic(f, CARGO_PATH_DEP_RE, group=1)


def do_root_cargo_members():
    f = "Cargo.toml"
    content = open(f, encoding="utf-8").read()
    lines = content.split("\n")
    out_lines = []
    changed = False
    for line in lines:
        m = re.match(r'^(\s*")([^"]+)(",?)\s*$', line)
        if m and m.group(2).startswith(SCOPE_ROOTS):
            old = m.group(2)
            new = tp.translate(old)
            if new != old:
                changed = True
                stats["strings_changed"] += 1
                line = f'{m.group(1)}{new}{m.group(3)}'
        out_lines.append(line)
    if changed:
        open(f, "w", encoding="utf-8").write("\n".join(out_lines))
        stats["files_changed"] += 1


def do_cargo_metadata_string_fields():
    """[[package.metadata.semio.*]] fields holding path strings under non-`path=`
    keys (engines, roots, root, placeholder) -- same class as round 2's fix."""
    pattern = re.compile(r'"((?:🧰️framework|✏️s|🌎️hub|♻️mit-bestand)/[^"]+)"')
    for f in cargo_files():
        if not os.path.exists(f):
            continue
        process_repo_root_relative(f, pattern, group=1)


# #endregion 🦀️CargoToml

# #region 🟦️PackageJson
def do_package_json():
    for f in glob.glob("**/package.json", recursive=True):
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        try:
            raw = open(f, encoding="utf-8").read()
            d = json.loads(raw)
        except Exception:
            continue
        base_dir = os.path.dirname(f)
        changed = False
        if "exports" in d and isinstance(d["exports"], dict):
            for k, v in list(d["exports"].items()):
                if isinstance(v, str):
                    newv = rewrite_ref(base_dir, v)
                    if newv:
                        d["exports"][k] = newv
                        changed = True
        if changed:
            open(f, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
            stats["files_changed"] += 1
            stats["strings_changed"] += 1


def do_root_package_json_workspaces():
    f = "package.json"
    d = json.loads(open(f, encoding="utf-8").read())
    changed = False
    new_ws = []
    for w in d.get("workspaces", []):
        if w.startswith(SCOPE_ROOTS):
            suffix = ""
            core = w
            if core.endswith("/*"):
                core = core[:-2]
                suffix = "/*"
            new_core = tp.translate(core)
            new_w = new_core + suffix
            if new_w != w:
                changed = True
                stats["strings_changed"] += 1
            new_ws.append(new_w)
        else:
            new_ws.append(w)
    if changed:
        d["workspaces"] = new_ws
        open(f, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
        stats["files_changed"] += 1


# #endregion 🟦️PackageJson

# #region 🟦️TypeScript
TS_IMPORT_RE = re.compile(r'((?:from\s+|import\()\s*")(\.\.?/[^"]+)(")')
RESOLVE_ABS_RE = re.compile(r'"((?:🧰️framework|✏️s|🌎️hub|♻️mit-bestand)/[^"]+)"')
DOTSLASH_ABS_RE = re.compile(r'"(\./(?:🧰️framework|✏️s|🌎️hub|♻️mit-bestand)/[^"]+)"')


def ts_files():
    for root in ("🧰️framework", "✏️s", "🌎️hub", "♻️mit-bestand"):
        yield from glob.glob(f"{root}/**/*.ts", recursive=True) + glob.glob(f"{root}/**/*.tsx", recursive=True)


def do_ts_imports():
    for f in ts_files():
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        process_generic(f, TS_IMPORT_RE, group=2)


def do_ts_absolute_refs():
    for f in ts_files():
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        process_repo_root_relative(f, RESOLVE_ABS_RE, group=1)
        # "./<root>/..." hybrid form (redundant "./" prefix on a repo-root path)
        _fix_dotslash(f)
    for f in ("script.ts", "vitest.config.ts"):
        if os.path.exists(f):
            process_repo_root_relative(f, RESOLVE_ABS_RE, group=1)
            _fix_dotslash(f)


def _fix_dotslash(fpath):
    try:
        content = open(fpath, encoding="utf-8").read()
    except Exception:
        return

    def repl(m):
        ref = m.group(1)
        old_core = ref[2:]
        new_core = tp.translate(old_core)
        if new_core == old_core:
            return m.group(0)
        stats["strings_changed"] += 1
        return f'"./{new_core}"'

    new_content = DOTSLASH_ABS_RE.sub(repl, content)
    if new_content != content:
        open(fpath, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


# #endregion 🟦️TypeScript

# #region 🔧️Misc configs
def do_gitignore():
    f = ".gitignore"
    lines = open(f, encoding="utf-8").read().split("\n")
    changed = False
    out = []
    for line in lines:
        stripped = line.lstrip("!/")
        prefix = line[: len(line) - len(stripped)]
        core = stripped.rstrip("/")
        trailing_slash = "/" if stripped.endswith("/") else ""
        if core.startswith(SCOPE_ROOTS):
            new_core = tp.translate(core)
            if new_core != core:
                changed = True
                stats["strings_changed"] += 1
            out.append(prefix + new_core + trailing_slash)
        else:
            out.append(line)
    if changed:
        open(f, "w", encoding="utf-8").write("\n".join(out))
        stats["files_changed"] += 1


def do_dependency_cruiser():
    f = ".dependency-cruiser.cjs"
    if not os.path.exists(f):
        return
    content = open(f, encoding="utf-8").read()

    def repl(m):
        ref = m.group(1)
        if not ref.startswith(SCOPE_ROOTS):
            return m.group(0)
        core = ref.rstrip("/")
        new_ref = tp.translate(core) + ("/" if ref.endswith("/") else "")
        if new_ref == ref:
            return m.group(0)
        stats["strings_changed"] += 1
        return f'"{new_ref}"'

    new_content = re.sub(r'"((?:🧰️framework|✏️s|🌎️hub|♻️mit-bestand)/[^"]*)"', repl, content)
    if new_content != content:
        open(f, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


def do_launch_json():
    for f in (".vscode/launch.json", ".claude/launch.json"):
        if os.path.exists(f):
            process_repo_root_relative(f, RESOLVE_ABS_RE, group=1)


# #endregion 🔧️Misc configs


def main():
    do_root_cargo_members()
    do_cargo()
    do_cargo_metadata_string_fields()
    do_root_package_json_workspaces()
    do_package_json()
    do_ts_imports()
    do_ts_absolute_refs()
    do_rust()
    do_build_rs_joins()
    do_gitignore()
    do_dependency_cruiser()
    do_launch_json()
    print(json.dumps(stats, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
