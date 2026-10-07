#!/usr/bin/env python3
"""🧭️ Qualifies the bare `io::<representation>::<role>::<item>` references a stdio crate root (its `pilot_languages`, its
inference leaf) kept after the codecs moved into `🚪️io`: the root has no `io` in scope, its `schema` re-export names the primary subset,
and that subset's `io` module is the one the references mean (they read `schema::<role>::<representation>::…` before).

Usage, from the repository root: `python3 <this> [--apply] [--only <crate>,<crate>]`. Without `--apply` it prints the plan
and exits 1 on any problem. A root is rewritten only when every reference resolves to a constant that exists on disk;
anything else is reported and left alone. Nothing but `🗿️artifacts/<crate>/🦀️.rs` is ever read for writing."""
import os
import re
import sys

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
SHIM = re.compile(r"pub mod schema \{\s*pub use super::(standards::\w+::subsets::\w+)::schema::\*;\s*\}")
BARE = re.compile(r"(?<![\w:])io::(text|binary|sqlite)::(snapshot|mutations|diff|inferences)::(\w+)\b")
ANY_BARE = re.compile(r"(?<![\w:])io::")
USE = re.compile(r"^(?:pub(?:\([a-z]+\))? )?use ([^;]+);", re.M)
MOUNT = re.compile(r"^(?:pub(?:\([a-z]+\))? )?(?:mod|extern crate) io\b", re.M)
OPEN = re.compile(r"(?:pub(?:\([a-z]+\))? )?mod (\w+) \{")
LEAF = re.compile(r"(?:pub(?:\([a-z]+\))? )?mod (\w+);")
PATH = re.compile(r'#\[path = "([^"]+)"\]')
DIRS = {"text": "📝️text", "binary": "💾️binary", "sqlite": "🪶️sqlite", "snapshot": "📸️snapshot", "mutations": "🧬️mutations", "diff": "🔺️diff", "inferences": "💡️inferences"}


def code(text):
    """✂️ The root without its comment lines (prose there names `io::register()` and is no reference)."""
    return "\n".join(line for line in text.split("\n") if not line.strip().startswith("//"))


def in_scope(text):
    """🔎️ Whether the crate root already names an `io` (a mount, or a `use` whose imported names include it)."""
    if MOUNT.search(text):
        return True
    return any(re.search(r"(?:^|[\s:{,])io\s*(?:,|\}|$)|\bas io\b|\bio::\{[^}]*\bself\b", body.strip()) for body in USE.findall(text))


def mounts(text):
    """🗺️ Module path → mounted source file of the root's inline `#[path]` tree."""
    stack, depth, pending, found = [], 0, None, {}
    for line in text.splitlines():
        stripped = line.strip()
        attribute = PATH.fullmatch(stripped)
        if attribute:
            pending = attribute.group(1)
            continue
        opened, leaf = OPEN.fullmatch(stripped), LEAF.fullmatch(stripped)
        if opened:
            stack.append((opened.group(1), depth))
            pending = None
        elif leaf:
            if pending and pending != ".":
                names = [name for name, _ in stack]
                found["::".join(names if leaf.group(1) == "component" else names + [leaf.group(1)])] = pending
            pending = None
        depth += stripped.count("{") - stripped.count("}")
        while stack and depth <= stack[-1][1]:
            stack.pop()
    return found


def plan(crate):
    """📋️ `(state, detail, text, rewritten)` of one crate root."""
    path = os.path.join(ROOT, crate, "🦀️.rs")
    text = open(path, encoding="utf-8").read()
    references = BARE.findall(code(text))
    bare = len(ANY_BARE.findall(code(text)))
    if not bare:
        return ("clean", "no bare io reference", text, text)
    if in_scope(text):
        return ("skip", "the root names an `io` already", text, text)
    if bare != len(references):
        return ("problem", f"{bare - len(references)} bare io reference(s) of an unknown form", text, text)
    shims = SHIM.findall(text)
    if len(shims) != 1:
        return ("problem", f"{len(shims)} schema re-exports where one names the primary subset", text, text)
    module = f"{shims[0]}::io"
    source = mounts(text).get(module)
    if source is None:
        return ("problem", f"`{module}` is not mounted by the root", text, text)
    folder = os.path.join(ROOT, crate, os.path.dirname(source))
    missing = []
    for representation, role, constant in sorted(set(references)):
        chain = [(os.path.join(ROOT, crate, source), representation), (os.path.join(folder, DIRS[representation], "🦀️.rs"), role)]
        for file, name in chain:
            if not os.path.isfile(file) or not re.search(rf"^\s*pub mod {name}\b", open(file, encoding="utf-8").read(), re.M):
                missing.append(f"{os.path.relpath(file, os.path.join(ROOT, crate))} mounts no `{name}`")
        leaf = os.path.join(folder, DIRS[representation], DIRS[role], "🦀️.rs")
        if not os.path.isfile(leaf) or not re.search(rf"^\s*pub (?:const|static|fn|struct|enum|type) {constant}\b", open(leaf, encoding="utf-8").read(), re.M):
            missing.append(f"{os.path.relpath(leaf, os.path.join(ROOT, crate))} has no `{constant}`")
    if missing:
        return ("problem", "; ".join(sorted(set(missing))[:3]), text, text)
    rewritten = "\n".join(line if line.strip().startswith("//") else BARE.sub(lambda found: f"{module}::{found.group(1)}::{found.group(2)}::{found.group(3)}", line) for line in text.split("\n"))
    count = len(references)
    assert not ANY_BARE.search(code(rewritten)) and len(rewritten) - len(text) == count * (len(module) - 2)
    return ("apply", f"{count} reference(s) → {module}", text, rewritten)


def main(arguments):
    apply = "--apply" in arguments
    only = set(arguments[arguments.index("--only") + 1].split(",")) if "--only" in arguments else None
    if not os.path.isdir(ROOT):
        sys.exit(f"{ROOT} is no directory: run from the repository root")
    crates = sorted(entry for entry in os.listdir(ROOT) if os.path.isfile(os.path.join(ROOT, entry, "🦀️.rs")))
    if not crates:
        sys.exit("no crate root found")
    problems = written = 0
    for crate in crates:
        if only is not None and crate not in only:
            continue
        state, detail, text, rewritten = plan(crate)
        if state == "clean":
            continue
        print(f"{state:8} {crate}: {detail}")
        problems += state == "problem"
        if apply and state == "apply":
            open(os.path.join(ROOT, crate, "🦀️.rs"), "w", encoding="utf-8").write(rewritten)
            written += 1
    print(f"{'written' if apply else 'planned'}: {written if apply else sum(1 for crate in crates if (only is None or crate in only) and plan(crate)[0] == 'apply')} root(s), problems: {problems}")
    sys.exit(1 if problems and not apply else 0)


if __name__ == "__main__":
    main(sys.argv[1:])
