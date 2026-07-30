#!/usr/bin/env python3
"""B1: rewrite every path-bearing string in the repo (Cargo.toml, package.json,
.gitignore, go.work/go.mod, nx.json, vitest.config.ts, .dependency-cruiser.cjs,
launch.json, root script.ts, all *.rs include!/path! literals, all *.ts/*.tsx
relative imports and vite/vitest resolve.alias targets) from old (pre-rename)
segment names to new (emoji+name) segment names, BEFORE the physical directory
rename happens. Uses translate-path-v2.py's prefix-walk translator so dual-status
basenames (e.g. "projekt") resolve correctly per their actual tree location.

Strategy per category: extract candidate path-like strings with a regex, strip
leading "./" and any leading "../" run (remembering the count), resolve the
REMAINDER as a repo-root-relative OR (for relative refs) file-relative old path,
translate it, and reassemble -- verified by checking the OLD path resolves to
a real pre-rename file/dir before rewriting (skip if it doesn't, to avoid
touching unrelated strings that happen to look path-shaped)."""
import os
import re
import glob
import json
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("translate_path_v2", os.path.join(HERE, "translate-path-v2.py"))
tp = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tp)

SCOPE_ROOTS = ("🧰/", "✏️/", "🌎/", "♻️/")
EXCLUDE_DIRS = {"node_modules", "target", ".git", ".repo", ".nx", "pkg"}

stats = {"files_changed": 0, "strings_changed": 0, "skipped_no_resolve": []}


def resolve_old(base_dir, ref):
    """Resolve a possibly-relative reference against base_dir (both repo-root-relative,
    POSIX, using the file's CURRENT pre-rename location). Returns repo-root-relative
    old path or None if it doesn't stay under a scope root / doesn't exist."""
    if ref.startswith("/"):
        return None
    joined = os.path.normpath(os.path.join(base_dir, ref)).replace(os.sep, "/")
    if joined.startswith("../") or joined == "..":
        return None
    if not joined.startswith(SCOPE_ROOTS):
        return None
    return joined


def rewrite_ref(base_dir, ref, check_exists=True):
    """Try to translate a single path-like reference string. Returns new ref or None
    if unchanged/not applicable."""
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
    if check_exists and not (os.path.exists(old_abs) or os.path.isdir(old_abs)):
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
    """For files where the matched string is REPO-ROOT-relative (not relative to the
    file itself), e.g. Cargo.toml members, package.json workspaces, .gitignore."""
    return process_generic(fpath, pattern, group=group, base_dir_override="")


# #region 🦀Rust
RS_INCLUDE_RE = re.compile(r'(include_(?:str|bytes)!\(\s*")([^"]+)(")')
RS_PATH_ATTR_RE = re.compile(r'(#\[path\s*=\s*")([^"]+)(")')


def rust_files():
    for root in ("🧰", "✏️", "🌎", "♻️"):
        yield from glob.glob(f"{root}/**/*.rs", recursive=True)


def do_rust():
    for f in rust_files():
        if any(f"/{d}/" in f for d in EXCLUDE_DIRS):
            continue
        process_generic(f, RS_INCLUDE_RE, group=2)
        process_generic(f, RS_PATH_ATTR_RE, group=2)


# #endregion 🦀Rust

# #region 🦀CargoToml
CARGO_PATH_DEP_RE = re.compile(r'path\s*=\s*"([^"]+)"')
CARGO_LIB_PATH_RE = re.compile(r'^\s*path\s*=\s*"([^"]+)"', re.M)


def cargo_files():
    for root in ("🧰", "✏️", "🌎", "♻️"):
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
        if m and m.group(2).startswith(("🧰/", "✏️/", "🌎/", "♻️/")):
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


# #endregion 🦀CargoToml

# #region 🟦PackageJson
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
        if w.startswith(("🧰/", "✏️/", "🌎/", "♻️/")):
            # workspaces may end in /* glob -- translate the concrete prefix only
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


# #endregion 🟦PackageJson

# #region 🟦TypeScript
TS_IMPORT_RE = re.compile(r'((?:from\s+|import\()\s*")(\.\.?/[^"]+)(")')


def ts_files():
    for root in ("🧰", "✏️", "🌎", "♻️"):
        yield from glob.glob(f"{root}/**/*.ts", recursive=True) + glob.glob(f"{root}/**/*.tsx", recursive=True)


def do_ts_imports():
    for f in ts_files():
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        process_generic(f, TS_IMPORT_RE, group=2)


# resolve(X, "🧰/...") or path.resolve(X, "✏️/...") style ABSOLUTE-from-repo-root
# string args used in vite/vitest configs and root script.ts / dev script.ts.
RESOLVE_ABS_RE = re.compile(r'"((?:🧰|✏️|🌎|♻️)/[^"]+)"')


def do_ts_absolute_refs():
    for f in ts_files():
        if any(d in f for d in EXCLUDE_DIRS) or "compose/" in f:
            continue
        process_repo_root_relative(f, RESOLVE_ABS_RE, group=1)
    do_file_repo_root_relative("script.ts")
    do_file_repo_root_relative("vitest.config.ts")


def do_file_repo_root_relative(f):
    if not os.path.exists(f):
        return
    process_repo_root_relative(f, RESOLVE_ABS_RE, group=1)


# #endregion 🟦TypeScript

# #region 🔧Misc configs
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
        if core.startswith(("🧰/", "✏️/", "🌎/", "♻️/")):
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


def do_go_work():
    f = "go.work"
    content = open(f, encoding="utf-8").read()

    def repl(m):
        ref = m.group(1)
        core = ref[2:] if ref.startswith("./") else ref
        if not core.startswith(("🧰/", "✏️/", "🌎/", "♻️/")):
            return m.group(0)
        new_core = tp.translate(core)
        if new_core == core:
            return m.group(0)
        stats["strings_changed"] += 1
        return "./" + new_core

    new_content = re.sub(r"(\./(?:🧰|✏️|🌎|♻️)/\S+)", repl, content)
    if new_content != content:
        open(f, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


def do_go_mod_replace():
    for f in glob.glob("**/go.mod", recursive=True):
        if any(d in f for d in EXCLUDE_DIRS):
            continue
        process_generic(f, re.compile(r"replace [^\s]+ => (\S+)"), group=1)


def do_nx_json():
    f = "nx.json"
    process_repo_root_relative(f, re.compile(r'"(\./(?:🧰|✏️|🌎|♻️)/[^"]+)"'), group=1)


def do_dependency_cruiser():
    f = ".dependency-cruiser.cjs"
    content = open(f, encoding="utf-8").read()

    def repl(m):
        ref = m.group(1)
        if not ref.startswith(("🧰/", "✏️/", "🌎/", "♻️/")):
            return m.group(0)
        new_ref = tp.translate(ref.rstrip("/")) + ("/" if ref.endswith("/") else "")
        if new_ref == ref:
            return m.group(0)
        stats["strings_changed"] += 1
        return f'"{new_ref}"'

    new_content = re.sub(r'"((?:🧰|✏️|🌎|♻️)/[^"]*)"', repl, content)
    if new_content != content:
        open(f, "w", encoding="utf-8").write(new_content)
        stats["files_changed"] += 1


def do_launch_json():
    f = ".vscode/launch.json"
    if not os.path.exists(f):
        return
    process_repo_root_relative(f, re.compile(r'"((?:🧰|✏️|🌎|♻️)/[^"]+)"'), group=1)


# #endregion 🔧Misc configs


def main():
    do_root_cargo_members()
    do_cargo()
    do_root_package_json_workspaces()
    do_package_json()
    do_ts_imports()
    do_ts_absolute_refs()
    do_rust()
    do_gitignore()
    do_go_work()
    do_go_mod_replace()
    do_nx_json()
    do_dependency_cruiser()
    do_launch_json()
    print(json.dumps(stats, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
