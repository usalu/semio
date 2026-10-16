"""✍️ Re-points references to the moved names in the given Rust files: brace `use` lists, single `use` paths and qualified paths.

Usage: rewrite_callers.py [--toolrun-crate] [--roots semio_framework,semio_framework_plugin] FILE...
`--toolrun-crate` sends the tool-run vocabulary to `semio_framework_tool_run` (the owner crate) instead of
`semio_framework::tool_run`.
"""
import re, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import moved_names as m

args = sys.argv[1:]
toolrun_crate = "--toolrun-crate" in args
args = [a for a in args if a != "--toolrun-crate"]
roots = ["semio_framework", "semio_framework_plugin"]
if args and args[0] == "--roots":
    roots = args[1].split(",")
    args = args[2:]
FACADE_FNS = {"app_declares_tool_run", "tool_run_action_definitions"}

def target(name):
    module = m.MODULE[name]
    if module == "tool_run" and toolrun_crate and name not in FACADE_FNS:
        return "semio_framework_tool_run"
    return f"semio_framework::{module}"

def split_top(body):
    items, depth, cur = [], 0, ""
    for ch in body:
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            items.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        items.append(cur)
    return items

for path in args:
    p = pathlib.Path(path)
    s = p.read_text()
    original = s
    root_alt = "|".join(re.escape(r) for r in roots)
    pattern = re.compile(r"(?m)^([ \t]*)((?:pub(?:\([^)]*\))? )?use )(?:::)?(" + root_alt + r")::\{")
    pos = 0
    while True:
        match = pattern.search(s, pos)
        if not match:
            break
        start = match.end()
        depth, i = 1, start
        while depth:
            if s[i] == "{":
                depth += 1
            elif s[i] == "}":
                depth -= 1
            i += 1
        end_brace = i - 1
        assert s[i] == ";", (path, s[match.start():i + 5])
        body = s[start:end_brace]
        items = split_top(body)
        moved, kept = [], []
        for item in items:
            name = item.strip()
            bare = re.sub(r"\s+as\s+\w+$", "", name)
            if bare in m.MODULE:
                moved.append(name)
            elif bare in m.REMOVED:
                raise SystemExit(f"{path}: removed name {bare} imported")
            else:
                kept.append(item)
        if not moved:
            pos = i
            continue
        line_start = s.rfind("\n", 0, match.start()) + 1
        attrs_start = line_start
        while True:
            prev_end = attrs_start - 1
            prev_start = s.rfind("\n", 0, prev_end) + 1
            prev = s[prev_start:prev_end]
            if prev_end > 0 and prev.strip().startswith("#[cfg"):
                attrs_start = prev_start
            else:
                break
        attrs = s[attrs_start:line_start]
        indent, use_kw, root = match.group(1), match.group(2), match.group(3)
        groups = {}
        for name in moved:
            bare = re.sub(r"\s+as\s+\w+$", "", name)
            groups.setdefault(target(bare), []).append(name)
        kept_body = ",".join(kept).strip().rstrip(",")
        if kept_body:
            new_stmt = f"{indent}{use_kw}{root}::{{{kept_body}}};"
            if not kept_body.count(",") and "{" not in kept_body and "::" not in kept_body.strip() and "\n" not in kept_body:
                new_stmt = f"{indent}{use_kw}{root}::{kept_body.strip()};"
        else:
            new_stmt = None
        extra = []
        for tgt, names in groups.items():
            listed = ", ".join(n.strip() for n in names)
            extra.append(f"{attrs}{indent}{use_kw}{tgt}::{{{listed}}};" if len(names) > 1 else f"{attrs}{indent}{use_kw}{tgt}::{listed};")
        if new_stmt is None:
            replacement = "\n".join(extra)
            s = s[:attrs_start] + replacement + s[i + 1:]
            pos = attrs_start + len(replacement)
        else:
            replacement = new_stmt + "\n" + "\n".join(extra)
            s = s[:match.start()] + replacement + s[i + 1:]
            pos = match.start() + len(replacement)
    def qualified(match_obj):
        prefix, name = match_obj.group(1), match_obj.group(2)
        if name in m.REMOVED:
            raise SystemExit(f"{path}: removed name {name} used")
        return f"{target(name)}::{name}"
    names_alt = "|".join(sorted(m.ALL, key=len, reverse=True))
    s = re.sub(r"(?<![\w:])((?:::)?(?:" + root_alt + r")(?:::manifest|::ui)?)::(" + names_alt + r")\b", qualified, s)
    if s != original:
        p.write_text(s)
        print("rewrote", path)
    else:
        print("unchanged", path)
