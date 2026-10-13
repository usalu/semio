"""🧭️ Maps external `semio_s_artifact_stdio_semio::…` paths of the monolith onto the crate that owns the item after the cut."""
import re
from semio_split import NAMES
from ownership import rust_name
from use_rewrite import rewrite_uses, _mask

AGG = "semio_s_artifact_stdio_semio"


def target(seg):
    """🎯️ The rewritten path for a monolith path `seg` (list) headed by the aggregator crate, or None."""
    if seg[:1] != [AGG]:
        return None
    s = seg[1:]
    if s[:1] == ["base_foundation"]:
        return [rust_name("base")] + s[1:]
    if s[:3] == ["standards", "v1", "subsets"] and len(s) > 3:
        sub = s[3]
        if sub == "base":
            return [AGG] + s[4:]
        if sub in NAMES:
            if s[4:6] in (["io", "import"], ["io", "export"]):
                return [rust_name(sub + "-" + s[5])] + s[6:]
            return [rust_name(sub)] + s[4:]
    if s[:1] == ["apply_diff"]:
        return ["semio_framework_os_kernel", "apply_diff"] + s[1:]
    return None


LIN = re.compile(r"(?<![\w$])semio_s_artifact_stdio_semio::(base_foundation|standards::v1::subsets::(\w+)(?:::io::(import|export)\b)?|apply_diff\b)")


def rewrite(src):
    """🔁️ Returns (src, changed) with `use` trees and linear paths mapped."""
    s1, n = rewrite_uses(src, target)
    masked = _mask(s1)
    out, last, c = [], 0, 0
    for m in LIN.finditer(masked):
        if m.group(1) == "base_foundation":
            rep = rust_name("base")
        elif m.group(1).startswith("apply_diff"):
            rep = "semio_framework_os_kernel::apply_diff"
        elif m.group(2) == "base":
            rep = AGG
        elif m.group(2) in NAMES:
            rep = rust_name(m.group(2) + ("-" + m.group(3) if m.group(3) else ""))
        else:
            continue
        out.append(s1[last:m.start()])
        out.append(rep)
        last = m.end()
        c += 1
    out.append(s1[last:])
    s2 = "".join(out)
    s2 = re.sub(r"\b%s::::" % AGG, AGG + "::", s2)
    return s2, (n + c) > 0
