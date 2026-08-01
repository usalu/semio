from pathlib import Path
from typing import Optional
import re

CFG_NO = "_cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>"

METHODS = [
    "handle_action",
    "handle_command",
    "handle_typed_command",
    "copy_fragment",
    "cut_operations",
    "pending_effects",
    "render",
    "window_engagements",
    "window_measures",
    "tool_measures",
    "context_menu",
]


def find_param_list(text: str, start_paren: int) -> int:
    depth = 0
    j = start_paren
    while j < len(text):
        c = text[j]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return j + 1
        j += 1
    raise ValueError("unclosed")


def insert_cfg_after_document_view(params: str, cfg_param: str) -> Optional[str]:
    if "ConfigView" in params or "DocumentView" not in params:
        return None
    m = re.search(r"DocumentView\s*<", params)
    if not m:
        return None
    angle_start = m.end() - 1
    depth = 0
    k = angle_start
    while k < len(params):
        if params[k] == "<":
            depth += 1
        elif params[k] == ">":
            depth -= 1
            if depth == 0:
                k += 1
                break
        k += 1
    comma_idx = params.find(",", k)
    if comma_idx < 0:
        return params[:k].rstrip() + ", " + cfg_param + params[k:]
    return params[: comma_idx + 1] + " " + cfg_param + "," + params[comma_idx + 1 :]


def fix_self_mut(params: str) -> str:
    return re.sub(r"^(\s*)&mut\s+self", r"\1&self", params)


def resolve_cfg_param(text: str) -> str:
    m = re.search(r"type\s+Config\s*=\s*([^;]+);", text)
    if not m:
        return CFG_NO
    cfg = m.group(1).strip()
    if cfg in ("semio_framework_plugin::NoConfig", "NoConfig"):
        return CFG_NO
    return f"_cfg: &semio_framework_plugin::ConfigView<'_, {cfg}>"


def process_file(path: Path) -> int:
    text = path.read_text()
    if "DocumentView" not in text or "DocumentApp" not in text:
        return 0
    cfg_param = resolve_cfg_param(text)
    changes = 0
    for method in METHODS:
        while True:
            did = False
            for m in re.finditer(rf"fn {method}\s*\(", text):
                open_paren = m.end() - 1
                try:
                    close = find_param_list(text, open_paren)
                except ValueError:
                    continue
                params = text[open_paren + 1 : close - 1]
                if "DocumentView" not in params:
                    continue
                new_params = params
                if "ConfigView" not in params:
                    inserted = insert_cfg_after_document_view(params, cfg_param)
                    if inserted is None:
                        continue
                    new_params = inserted
                fixed = fix_self_mut(new_params)
                if fixed == params:
                    continue
                text = text[: open_paren + 1] + fixed + text[close - 1 :]
                changes += 1
                did = True
                break
            if not did:
                break
    if changes:
        path.write_text(text)
    return changes


changed = []
roots = [Path("/Users/ueli/Documents/semio/✏️s")]
for p in Path("/Users/ueli/Documents/semio").iterdir():
    if "framework" in p.name:
        roots.append(p)

total = 0
for root in roots:
    for path in root.rglob("*.rs"):
        if "target" in path.parts:
            continue
        n = process_file(path)
        if n:
            changed.append((n, str(path)))
            total += n

print(f"total={total} files={len(changed)}")
for n, p in changed:
    print(f"  {n} {p}")
