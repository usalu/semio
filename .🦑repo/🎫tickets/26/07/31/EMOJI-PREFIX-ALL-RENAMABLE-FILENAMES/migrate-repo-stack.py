#!/usr/bin/env python3
"""Rename .repo → .🦑repo and expand bare emoji folders; path-prefix text rewrites."""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

SKIP = frozenset({"node_modules", "target", ".git", ".nx", ".repo-cache", "dist", "pkg"})

# Inside .repo only — deepest segments first
REPO_DIR_RENAMES = [
    (".repo/⚡/🔀", ".repo/⚡/🔀diff"),
    (".repo/⚡/🤖", ".repo/⚡/🤖generated"),
    (".repo/⚡", ".repo/⚡cache"),
    (".repo/prompts", ".repo/💬prompts"),
    (".repo/tmp", ".repo/🧹tmp"),
    (".repo/🎫", ".repo/🎫tickets"),
    (".repo/🎯", ".repo/🎯goals"),
    (".repo/🛂", ".repo/🛂manifest"),
    (".repo/📊", ".repo/📊metrics"),
    (".repo/💬", ".repo/💬chat"),
    (".repo/💡", ".repo/💡ideas"),
    (".repo/🧑‍💻", ".repo/🧑‍💻devs"),
    (".repo/✍️", ".repo/✍️notes"),
]

ROOT_RENAME = (".repo", ".🦑repo")

REPO_FILE_RENAMES = [
    ("config.toml", "📋config.toml"),
    ("bullets.txt", "📝bullets.txt"),
    ("compose-micro-commit-bun", "🐹compose-micro-commit-bun"),
]

# Longest-prefix path string replacements (after physical renames)
PATH_REPLACEMENTS = [
    (".repo/⚡cache/🔀diff/", ".🦑repo/⚡cache/🔀diff/"),
    (".repo/⚡cache/🤖generated/", ".🦑repo/⚡cache/🤖generated/"),
    (".repo/⚡cache/", ".🦑repo/⚡cache/"),
    (".repo/🎫tickets/", ".🦑repo/🎫tickets/"),
    (".repo/🎯goals/", ".🦑repo/🎯goals/"),
    (".repo/🛂manifest/", ".🦑repo/🛂manifest/"),
    (".repo/📊metrics/", ".🦑repo/📊metrics/"),
    (".repo/💬chat/", ".🦑repo/💬chat/"),
    (".repo/💬prompts/", ".🦑repo/💬prompts/"),
    (".repo/💡ideas/", ".🦑repo/💡ideas/"),
    (".repo/🧑‍💻devs/", ".🦑repo/🧑‍💻devs/"),
    (".repo/✍️notes/", ".🦑repo/✍️notes/"),
    (".repo/🧹tmp/", ".🦑repo/🧹tmp/"),
    (".repo/cache/", ".🦑repo/⚡cache/"),  # legacy refs
    (".repo/coverage/", ".🦑repo/📊metrics/coverage/"),  # if exists
    (".repo/🎫/", ".🦑repo/🎫tickets/"),
    (".repo/🛂", ".🦑repo/🛂manifest"),
    (".repo/", ".🦑repo/"),
    ("/.repo/", "/.🦑repo/"),
]


def apply_dir_renames():
    for old, new in REPO_DIR_RENAMES:
        if os.path.isdir(old) and not os.path.exists(new):
            os.rename(old, new)
            print("dir", old, "->", new)
        elif os.path.isdir(old) and os.path.exists(new):
            print("skip dir collision", old, new)


def apply_root_rename():
    if os.path.isdir(".repo") and not os.path.exists(".🦑repo"):
        os.rename(".repo", ".🦑repo")
        print("root .repo -> .🦑repo")


def apply_file_renames_in_repo():
    root = ".🦑repo"
    if not os.path.isdir(root):
        root = ".repo"
    for dirpath, _, files in os.walk(root):
        for old, new in REPO_FILE_RENAMES:
            if old in files:
                op = os.path.join(dirpath, old)
                np = os.path.join(dirpath, new)
                if not os.path.exists(np):
                    os.rename(op, np)


def rewrite_paths():
    changed = 0
    for dirpath, dirs, files in os.walk(".", topdown=True):
        if any(p in SKIP for p in dirpath.split(os.sep)):
            dirs[:] = []
            continue
        dirs[:] = [d for d in dirs if d not in SKIP]
        for f in files:
            if f == "AGENTS.md":
                continue
            path = os.path.join(dirpath, f).replace(os.sep, "/").lstrip("./")
            try:
                raw = open(path, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError):
                continue
            new = raw
            for a, b in PATH_REPLACEMENTS:
                new = new.replace(a, b)
            if new != raw:
                open(path, "w", encoding="utf-8").write(new)
                changed += 1
    print(f"path rewrites in {changed} files")


def main():
    apply_dir_renames()
    apply_file_renames_in_repo()
    apply_root_rename()
    rewrite_paths()


if __name__ == "__main__":
    main()
