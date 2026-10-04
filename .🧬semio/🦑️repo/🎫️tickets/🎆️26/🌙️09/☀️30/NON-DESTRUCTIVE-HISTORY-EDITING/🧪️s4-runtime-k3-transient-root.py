#!/usr/bin/env python3
"""🫧️ K3 (audit-s3-tools K3, design §21.5): reports the top-level items of a hand-written whole-root transient module and, with
--apply, replaces its mutation/codec/owner boilerplate by `semio_framework_plugin::transient_root!` (+ `window_transient_owners!`).

Usage: s4-runtime-k3-transient-root.py <file> [--apply] [--owners NAME=EXPR,...] [--no-window]
Only items whose header names the state's mutation, the state's codec/diff impls, or the owner/registration helpers are removed;
everything else (the state, its retirement, domain helpers) stays byte for byte.
"""
import re
import sys


def items(text):
    lines = text.split("\n")
    out, start, depth, pending = [], 0, 0, None
    index = 0
    while index < len(lines):
        line = lines[index]
        if pending is None:
            if line.strip() == "" or line.startswith(" ") or line.startswith("\t") or line.startswith("}") or line.startswith("//#region") or line.startswith("//#endregion"):
                out.append(("blank" if line.strip() == "" else "loose", index, index))
                index += 1
                continue
            pending = index
            depth = 0
        depth += line.count("{") - line.count("}")
        stripped = line.strip()
        is_attr = stripped.startswith("#[") or stripped.startswith("///") or stripped.startswith("//!") or stripped.startswith("//")
        if depth <= 0 and not is_attr and (stripped.endswith("}") or stripped.endswith(";") or stripped.endswith("})")):
            out.append(("item", pending, index))
            pending = None
        index += 1
    if pending is not None:
        out.append(("item", pending, len(lines) - 1))
    return lines, out


def header(lines, start, end):
    for line in lines[start:end + 1]:
        s = line.strip()
        if not (s.startswith("#[") or s.startswith("//")):
            return s
    return ""


def main():
    path = sys.argv[1]
    apply = "--apply" in sys.argv
    text = open(path, encoding="utf-8").read()
    state = re.search(r"enum (\w+TransientMutation|\w+Mutation)\s*\{\s*Snapshot \{ transient: (\w+) \}", text)
    if not state:
        print("no whole-root mutation in", path)
        return 1
    mutation, root = state.group(1), state.group(2)
    lines, parsed = items(text)
    remove_patterns = [
        rf"enum {mutation}\b",
        rf"impl .*for {mutation}\b",
        rf"impl .*MutationDiff<{root}> for {root}\b",
        rf"impl store::ArtifactDsl for {root}\b",
        rf"impl store::ArtifactPack for {root}\b",
        r"fn preflight\(",
        r"fn transfer\(",
        r"macro_rules! \w*transient_owner",
        r"^\w*transient_owner!\(",
        r"pub(\(crate\))? use \w*transient_owner;",
        r"pub fn register\(",
        r"fn from_owner<",
        r"pub fn from_snapshot\(",
        r"pub fn current(<T>)?\(",
        r"pub fn addressed\(",
        r"pub fn owner_bundle\(",
        r"pub struct \w+TransientOwner;",
        r"impl (semio_framework_plugin::)?WindowTransientOwner for \w+",
    ] + [extra for arg in sys.argv if arg.startswith("--remove=") for extra in arg[len("--remove="):].split(";;")]
    kept, removed = [], []
    for kind, start, end in parsed:
        h = header(lines, start, end) if kind == "item" else ""
        if kind == "item" and any(re.search(p, h) for p in remove_patterns):
            removed.append(h)
        else:
            kept.extend(lines[start:end + 1])
    print("mutation", mutation, "root", root)
    for h in removed:
        print("  remove:", h[:140])
    if not apply:
        return 0
    def grab(pattern, default=None):
        m = re.search(pattern, text, re.S)
        return m.group(1) if m else default
    owner = grab(r'owner: "([^"]+)"')
    kind_name = grab(r'semantic_kind: "([^"]+)"')
    display = grab(r'display_name: "([^"]+)"')
    schema = grab(r'payload_schema: "([^"]+)"')
    extension = grab(r'const EXTENSION: &\'static str = "([^"]+)"')
    envelope = grab(r'fn envelope_id\(\) -> &\'static str \{\s*"([^"]+)"')
    owners = []
    for arg in sys.argv:
        if arg.startswith("--owners="):
            owners = [pair.split("=", 1) for pair in arg[len("--owners="):].split(",") if pair]
    if not owners and "--no-window" not in sys.argv:
        owners = re.findall(r'^\w*transient_owner!\((\w+), ([^)]+)\);', text, re.M)
        owners += re.findall(r'impl (?:semio_framework_plugin::)?WindowTransientOwner for (\w+) \{\s*const WINDOW_KIND_ID: &\'static str = ([^;]+);', text)
    body = "\n".join(kept).rstrip("\n")
    while "\n\n\n" in body:
        body = body.replace("\n\n\n", "\n\n")
    invocation = f"""
semio_framework_plugin::transient_root! {{
    state: {root},
    mutation: {mutation},
    owner: "{owner}",
    kind: "{kind_name}",
    display_name: "{display}",
    payload_schema: "{schema}",
    envelope: "{envelope}",
    extension: "{extension}",
}}
"""
    if owners:
        listed = ",\n".join(f"        {name} => {expr}" for name, expr in owners)
        invocation += f"""
semio_framework_plugin::window_transient_owners! {{
    state: {root},
    mutation: {mutation},
    windows: {{
{listed},
    }},
}}
"""
    tests = re.search(r"\n(#\[cfg\(test\)\]\n#\[path = [^\n]+\]\nmod tests;)\s*$", body)
    if tests:
        body = body[:tests.start()] + "\n" + invocation + "\n" + tests.group(1)
    else:
        body = body + "\n" + invocation
    while "\n\n\n" in body:
        body = body.replace("\n\n\n", "\n\n")
    open(path, "w", encoding="utf-8").write(body.rstrip("\n") + "\n")
    print("applied", path, "owners", owners, "missing" if None in (owner, kind_name, display, schema, extension, envelope) else "complete")
    return 0


if __name__ == "__main__":
    sys.exit(main())
