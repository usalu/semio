"""🛰️ Splits the Dock unit test: the laws that only need the dock crate move in-crate (they read private dock items), the shell-bound laws stay in the bin as a sibling file.
usage: split_dock.py plan|apply"""
import re, sys
from lib import *
APPLY = "apply" in sys.argv
U = EL + "/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs"
SB_DIR = EL + "/🛰️Dock/🧪️tests/🔬️wgpu-shell-bound"
SB = SB_DIR + "/🦀️.rs"
text = read(U)
clean = blank(text)
lines, cl = text.split("\n"), clean.split("\n")
items, depth, i, n = [], 0, 0, len(cl)
while i < n:
    l = cl[i]
    if depth == 0 and l and not l[0].isspace() and l[0] not in "}#/":
        s = i
        while s > 0 and lines[s - 1].lstrip().startswith(("#[", "///", "//")):
            s -= 1
        d, seen, j = 0, False, i
        while j < n:
            d += cl[j].count("{") - cl[j].count("}")
            seen = seen or "{" in cl[j]
            if (seen and d <= 0) or (not seen and cl[j].rstrip().endswith(";")):
                break
            j += 1
        m = re.match(r"(?:pub )?(?:fn|struct|enum|const|static|type) (\w+)", l.strip())
        items.append(dict(s=s, e=j, head=l.strip(), name=m.group(1) if m else None, test="#[test]" in "\n".join(lines[s:i + 1]), use=l.startswith("use ")))
        i = j + 1
        continue
    depth += l.count("{") - l.count("}")
    i += 1
UPPER = set("ShellState SessionCtx tiled_map SpacePanelState SpawnedAppEntry take_spawned_entry key_event_matches_chord resolve_window_utilities resolved_execute_args".split()) | {"semio_framework_os_renderer_wgpu_shell_dock", "semio_framework_os_renderer_wgpu_shell_docs", "semio_framework_os_renderer_wgpu_shell_session", "semio_framework_os_renderer_wgpu_shell_state", "semio_framework_os_renderer_wgpu_shell_commands", "semio_framework_os_renderer_wgpu_shell_prefs", "semio_framework_os_renderer_wgpu_engine_board"}
for it in items:
    it["tok"] = set(re.findall(r"[A-Za-z_]\w*", "\n".join(cl[it["s"]:it["e"] + 1])))
sb = {id(it) for it in items if it["tok"] & UPPER and not it["use"]}
byname = {it["name"]: it for it in items if it["name"]}
def calls(it, name):
    return re.search(r"(?<![\w.])" + name + r"\s*(?:::<[^()]*>)?\(", "\n".join(cl[it["s"]:it["e"] + 1])) is not None
tainted = True
while tainted:
    tainted = False
    for h in [x for x in items if id(x) in sb and x["name"] and not x["test"]]:
        for it in items:
            if id(it) not in sb and not it["use"] and calls(it, h["name"]) and h["name"] in {"shell", "actions_utilities_app"} | {y["name"] for y in items if id(y) in sb and y["name"] and not y["test"] and (y["tok"] & UPPER)}:
                sb.add(id(it))
                tainted = True
changed = True
while changed:
    changed = False
    for it in items:
        if id(it) in sb:
            for tk in it["tok"]:
                h = byname.get(tk)
                if h and not h["test"] and id(h) not in sb:
                    sb.add(id(h))
                    changed = True
others = [it for it in items if id(it) not in sb and not it["use"]]
others_text = "\n".join("\n".join(cl[it["s"]:it["e"] + 1]) for it in others)
shared = {it["name"] for it in items if id(it) in sb and it["name"] and not it["test"] and re.search(r"(?<![\w.])" + it["name"] + r"\s*(?:::<[^()]*>)?\(", others_text)}
sb_tests = [it["name"] for it in items if id(it) in sb and it["test"]]
print("shell-bound tests", sb_tests)
print("moved helpers", [it["name"] for it in items if id(it) in sb and not it["test"] and it["name"] not in shared and it["name"]])
print("shared helpers (kept in both)", sorted(shared))
uses = [it for it in items if it["use"]]
def chunk(it):
    return "\n".join(lines[it["s"]:it["e"] + 1])
sb_uses = [chunk(it) for it in uses]
u_uses = [chunk(it) for it in uses if not (it["tok"] & UPPER)]
sb_body = [chunk(it) for it in items if id(it) in sb and not it["use"]]
u_body = [chunk(it) for it in items if (id(it) not in sb or it["name"] in shared) and not it["use"]]
header = "\n".join(lines[:items[0]["s"]]).rstrip()
sb_text = "//! 🛰️ Dock laws that read shell state: the unit laws of `🔬️wgpu-unit` that need the shell crates, mounted by the shell test bin.\n\nuse super::*;\n" + "\n".join(sb_uses[0:0]) + "\n".join(u for u in sb_uses if "super::*" not in u and u.strip() != "use super::*;") + "\n\n" + "\n\n".join(sb_body) + "\n"
u_text = (header + "\n" if header and "super::*" not in header else "") 
u_text = "\n".join(lines[:items[0]["s"]]).rstrip() + ("\n" if items[0]["s"] else "")
u_text += "\n".join(u_uses) + "\n\n" + "\n\n".join(u_body) + "\n"
print("unit file:", len(u_text.split("\n")), "lines; shell-bound file:", len(sb_text.split("\n")), "lines; original", len(lines))
if APPLY:
    edit(U, u_text)
    write(SB, sb_text)
