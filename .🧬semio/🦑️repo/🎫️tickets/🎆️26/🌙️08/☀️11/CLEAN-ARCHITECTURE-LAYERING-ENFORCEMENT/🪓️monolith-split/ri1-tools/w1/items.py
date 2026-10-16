#!/usr/bin/env python3
# 🧱️ Column-0 item table of a Rust text: name, kind, visibility, cfg attribute lines, span (uses the blanking lexer + brace depth).
import re
from rs import blank

HEAD = re.compile(r"^(pub(?:\([^)]*\))?\s+)?((?:(?:async|const|unsafe|extern\s+\"[^\"]*\")\s+)*)(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?")


def table(text):
    clean = blank(text)
    L = clean.split("\n")
    R = text.split("\n")
    depth = 0
    starts = []
    for l in L:
        starts.append(depth)
        depth += l.count("{") - l.count("}")
    out = []
    n = len(L)
    i = 0
    while i < n:
        l = L[i]
        m = HEAD.match(l) if starts[i] == 0 and l and not l[0].isspace() and l[0] not in "}#" else None
        if m:
            d, seen, j = 0, False, i
            while j < n:
                d += L[j].count("{") - L[j].count("}")
                seen = seen or "{" in L[j]
                if (seen and d <= 0) or (not seen and L[j].rstrip().endswith(";")):
                    break
                j += 1
            s = i
            attrs = []
            while s > 0 and R[s - 1].lstrip().startswith(("///", "#[", "//!", "#![")):
                s -= 1
            for k in range(s, i):
                if R[k].lstrip().startswith("#["):
                    attrs.append(R[k].strip())
            kind = m.group(3)
            name = (m.group(4) or "").strip()
            if kind == "thread_local!":
                for sm in re.finditer(r"static\s+([A-Za-z_][A-Za-z0-9_]*)", "\n".join(L[i:j + 1])):
                    out.append(dict(name=sm.group(1), kind="static", vis="", attrs=attrs, start=s + 1, end=j + 1, head=i + 1, thread_local=True))
            elif kind != "use":
                out.append(dict(name=re.split(r"[<(\s:]", name)[0] if kind != "impl" else l.strip()[:60], kind=kind, vis=(m.group(1) or "").strip(), attrs=attrs, start=s + 1, end=j + 1, head=i + 1))
            i = j + 1
        else:
            i += 1
    return out


def cfg_of(attrs):
    cf = [a for a in attrs if a.startswith("#[cfg(")]
    return cf
