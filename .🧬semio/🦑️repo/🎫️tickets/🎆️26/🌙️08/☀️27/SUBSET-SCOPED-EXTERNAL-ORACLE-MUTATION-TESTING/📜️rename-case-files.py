#!/usr/bin/env python3
"""📓 Rename test-case files inside **/🧪️tests/<case>/ to emoji-only canonical
names, then rewrite stale references repo-wide. Scoped to avoid production
🦀️component.rs siblings and the coordinator-owned test-platform module.
"""
import os
import re
import sys
import json

REPO_ROOT = "/Users/ueli/Documents/semio"

PRUNE_DIR_NAMES = {"node_modules", ".git", "target", "dist", "build"}

# Directories that are meta/ticket-management data or generated build cache —
# never live source, docs, or feature files that discoverTestCases reads.
# Rewriting references inside them would either revise other sessions'
# historical audit trail (.🧬semio ticket folders) or hand-edit a generated
# cache that nx regenerates on its own (.nx). Only applies to the reference
# rewrite pass — the rename pass never touches test-case files there either
# since none of the found case dirs live under these roots except the current
# ticket's own scratch files, which this also correctly leaves alone.
REFS_EXCLUDED_ROOTS = {".🧬semio", ".nx"}

# Absolute path prefix that is off-limits (coordinator-owned test platform module).
FORBIDDEN_PREFIX = os.path.join(
    REPO_ROOT,
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test",
)

TESTS_DIRNAME = "🧪️tests"

RENAME_MAP = {
    "component.feature": "🥒️.feature",
    "🦀️component.rs": "🦀️.rs",
    "🟦️component.ts": "🟦️.ts",
    "🐍️component.py": "🐍️.py",
    "🐹️component.go": "🐹️.go",
    "🔷️component.cs": "🔷️.cs",
}

FRAGMENT_MAP = {
    "/component.feature": "/🥒️.feature",
    "/🦀️component.rs": "/🦀️.rs",
    "/🟦️component.ts": "/🟦️.ts",
    "/🐍️component.py": "/🐍️.py",
    "/🐹️component.go": "/🐹️.go",
    "/🔷️component.cs": "/🔷️.cs",
}

TEXT_EXT_SKIP = {".rlib", ".png", ".jpg", ".jpeg", ".gif", ".webp", ".ico",
                 ".pdf", ".zip", ".gz", ".tar", ".woff", ".woff2", ".ttf",
                 ".eot", ".so", ".dylib", ".dll", ".exe", ".rmeta", ".class",
                 ".jar", ".wasm"}


def is_forbidden(path):
    return path == FORBIDDEN_PREFIX or path.startswith(FORBIDDEN_PREFIX + os.sep)


FEATURE_NAMES = {"component.feature", "🥒️.feature"}
MUTATIONS_SEGMENT = "🧬️mutations"


def find_case_dirs():
    """Yield case directories: dirpath is a direct child of a 🧪️tests
    directory AND directly contains a feature file AND no path segment is
    🧬️mutations (mutation-vector scenario bundles are a different contract
    and must never be touched by this script)."""
    case_dirs = []
    for dirpath, dirnames, filenames in os.walk(REPO_ROOT):
        dirnames[:] = [d for d in dirnames if d not in PRUNE_DIR_NAMES]
        if is_forbidden(dirpath):
            dirnames[:] = []
            continue
        grandparent_dir = os.path.dirname(dirpath)
        grandparent_name = os.path.basename(grandparent_dir)
        if grandparent_name != TESTS_DIRNAME:
            continue
        if MUTATIONS_SEGMENT in dirpath.split(os.sep):
            continue
        if not any(fname in FEATURE_NAMES for fname in filenames):
            continue
        case_dirs.append(dirpath)
    return case_dirs


def find_rename_targets():
    """Yield (dirpath, old_name, new_name) for files directly inside a
    qualifying case directory (see find_case_dirs)."""
    targets = []
    for dirpath in find_case_dirs():
        filenames = os.listdir(dirpath)
        for fname in filenames:
            if fname in RENAME_MAP:
                new_name = RENAME_MAP[fname]
                if fname == new_name:
                    continue
                targets.append((dirpath, fname, new_name))
    return targets


def do_renames(targets, dry_run=False):
    per_kind = {}
    renamed = []
    skipped_forbidden = []
    for dirpath, old_name, new_name in targets:
        if is_forbidden(dirpath):
            skipped_forbidden.append(os.path.join(dirpath, old_name))
            continue
        old_path = os.path.join(dirpath, old_name)
        new_path = os.path.join(dirpath, new_name)
        if not os.path.exists(old_path):
            continue
        if os.path.exists(new_path):
            print(f"[WARN] target already exists, skipping: {new_path}", file=sys.stderr)
            continue
        per_kind[old_name] = per_kind.get(old_name, 0) + 1
        if not dry_run:
            os.rename(old_path, new_path)
        renamed.append((old_path, new_path))
    return renamed, per_kind, skipped_forbidden


# Per fragment, a regex capturing (a) the "🧪️tests/" anchor, (b) the case-dir
# name as a single path segment with no embedded "/" (so it can only match a
# case dir that is a DIRECT child of 🧪️tests — the same rule the renamer used),
# immediately followed by the literal fragment. Whether to actually replace is
# decided by checking the captured case name against CASE_DIR_NAMES — the exact
# whitelist of directories this run renamed — so unrelated directories that
# merely resemble a case dir (a Rust-only fixture dir with no 🥒️.feature, a
# "🧬️mutation-regressions" regression dir, a "🧬️mutations" vector bundle) are
# never touched, even though they also sit directly under some 🧪️tests/.
FRAGMENT_PATTERNS = {
    old: (re.compile(r'🧪️tests/([^\s"\'`)/]+)' + re.escape(old)), new)
    for old, new in FRAGMENT_MAP.items()
}


def rewrite_line(line, case_dir_names):
    changed = False

    def make_repl(new_suffix):
        def repl(m):
            nonlocal changed
            case_name = m.group(1)
            if case_name not in case_dir_names:
                return m.group(0)
            changed = True
            return "🧪️tests/" + case_name + new_suffix
        return repl

    new_line = line
    for old, (pattern, new_suffix) in FRAGMENT_PATTERNS.items():
        new_line = pattern.sub(make_repl(new_suffix), new_line)
    return new_line, changed


def rewrite_references(dry_run=False):
    case_dir_names = {os.path.basename(d) for d in find_case_dirs()}
    rewritten_files = []
    total_replacements_lines = 0
    for dirpath, dirnames, filenames in os.walk(REPO_ROOT):
        if dirpath == REPO_ROOT:
            dirnames[:] = [d for d in dirnames if d not in REFS_EXCLUDED_ROOTS]
        dirnames[:] = [d for d in dirnames if d not in PRUNE_DIR_NAMES]
        if is_forbidden(dirpath):
            dirnames[:] = []
            continue
        for fname in filenames:
            ext = os.path.splitext(fname)[1].lower()
            if ext in TEXT_EXT_SKIP:
                continue
            fpath = os.path.join(dirpath, fname)
            if is_forbidden(fpath):
                continue
            if os.path.basename(dirpath) == "🧪️oracle" and fname == "🔣️.json":
                # Owned by another agent — must not be touched under any circumstance.
                continue
            # quick pre-filter: only open files that might contain a fragment.
            try:
                with open(fpath, "rb") as fh:
                    raw = fh.read()
            except (IsADirectoryError, PermissionError, OSError):
                continue
            if b"\x00" in raw:
                continue  # binary
            if not any(frag.encode("utf-8") in raw for frag in FRAGMENT_MAP):
                continue
            try:
                text = raw.decode("utf-8")
            except UnicodeDecodeError:
                continue
            lines = text.split("\n")
            any_line_changed = False
            new_lines = []
            for line in lines:
                if "🧪️tests/" in line and any(f in line for f in FRAGMENT_MAP):
                    new_line, changed = rewrite_line(line, case_dir_names)
                    if changed:
                        any_line_changed = True
                        total_replacements_lines += 1
                    new_lines.append(new_line)
                else:
                    new_lines.append(line)
            if any_line_changed:
                rewritten_files.append(fpath)
                if not dry_run:
                    with open(fpath, "w", encoding="utf-8") as fh:
                        fh.write("\n".join(new_lines))
    return rewritten_files, total_replacements_lines


def main():
    dry_run = "--dry-run" in sys.argv
    do_refs_only = "--refs-only" in sys.argv
    do_rename_only = "--rename-only" in sys.argv
    list_cases = "--list-cases" in sys.argv

    if list_cases:
        case_dirs = find_case_dirs()
        bad = [d for d in case_dirs if MUTATIONS_SEGMENT in d.split(os.sep)]
        print(json.dumps({
            "case_dir_count": len(case_dirs),
            "contains_mutations_segment": bad,
            "case_dirs": sorted(case_dirs),
        }, indent=2, ensure_ascii=False))
        return

    result = {}

    if not do_refs_only:
        targets = find_rename_targets()
        renamed, per_kind, skipped_forbidden = do_renames(targets, dry_run=dry_run)
        result["renamed_count"] = len(renamed)
        result["renamed_per_kind"] = per_kind
        result["skipped_forbidden"] = skipped_forbidden
        result["renamed_sample"] = renamed[:10]

    if not do_rename_only:
        rewritten_files, total_lines = rewrite_references(dry_run=dry_run)
        result["rewritten_files_count"] = len(rewritten_files)
        result["rewritten_lines_count"] = total_lines
        result["rewritten_files"] = rewritten_files

    print(json.dumps(result, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
