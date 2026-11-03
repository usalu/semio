"""Task 1 layer E: examples, reads/outline lists."""
from lib import *
import glob
for p in glob.glob(SUB + "/📚️examples/*/🦀️.rs"):
    def go(t):
        if "k_fi_declared" not in t:
            return t
        t = sub(t, r"reliability_class: (\d),", lambda m: "reliability_class: ReliabilityClass::RC%s," % m.group(1)) if False else re.sub(r"reliability_class: (\d),", r"reliability_class: ReliabilityClass::RC\1,", t)
        t = rep(t, "        k_fi_declared: 1.0,\n", "")
        t = rep(t, "use semio_s_artifact_norm_en1990_model::{", "use semio_s_artifact_norm_en1990_model::{ReliabilityClass, ", count=1)
        return t
    edit(p, go)
for p in (INF + "/🦀️.rs", INF + "/🧾outline/🦀️.rs"):
    edit(p, lambda t: t.replace('                "kFiDeclared",\n', "").replace('    "kFiDeclared",\n', ""))
