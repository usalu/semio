import os, shutil, re
A = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff"
S = A + "/🏅️standards/🔖️6.0/🪆️subsets"
D = S + "/🧾️document"; B = S + "/🧱️baseline"
ORIG = os.path.dirname(os.path.abspath(__file__)) + "/orig"
def snap(path, tag):
    dst = f"{ORIG}/tiff-{tag}"
    if not os.path.exists(dst): shutil.copy(path, dst)
def rw(path, fn, tag):
    snap(path, tag)
    t = open(path, encoding="utf-8").read()
    open(path, "w", encoding="utf-8").write(fn(t))
# 1 NativeFieldType trait -> snapshot component
state = {}
def cut_trait(t):
    s = t.index("pub trait NativeFieldType {")
    e = t.index("fn native_value_count(")
    state["block"] = t[s:e]
    return t[:s] + t[e:]
rw(D + "/🚪️io/🦀️.rs", cut_trait, "io-root-t2.rs")
rw(D + "/🧬️schema/📸️snapshot/🦀️.rs", lambda t: t.rstrip("\n") + "\n\n" + state["block"].rstrip("\n") + "\n", "snapshot.rs")
def imp(t):
    a = "use crate::schema::snapshot::{\n    TiffBinary32,"
    assert t.count(a) == 1
    return t.replace(a, "use crate::schema::snapshot::{\n    NativeFieldType, TiffBinary32,")
rw(D + "/🚪️io/🦀️.rs", imp, "io-root-t2b.rs")
rw(D + "/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs", lambda t: t.replace("crate::standards::v6_0::subsets::document::io::NativeFieldType", "crate::standards::v6_0::subsets::document::schema::snapshot::NativeFieldType").replace("crate::standards::v6_0::subsets::baseline::io::check_tiff_baseline_conformance_controlled", "crate::standards::v6_0::subsets::baseline::schema::conformance::check_tiff_baseline_conformance_controlled"), "sqlite-snapshot.rs")
# 2 controlled conformance wrapper -> conformance
def cut_ctl(t):
    m = re.search(r"    pub fn check_tiff_baseline_conformance_controlled\(.*\n", t)
    assert m and t.count("pub fn check_tiff_baseline_conformance_controlled") == 1
    line = m.group(0).strip()
    state["ctl"] = line.replace("check_tiff_baseline_conformance_with(", "check_tiff_baseline_conformance_with(").replace("Vec<Diagnostic>", "Vec<Diagnostic>")
    return t.replace(m.group(0), "")
rw(B + "/🚪️io/🦀️.rs", cut_ctl, "baseline-io-t2.rs")
rw(B + "/🧬️schema/🏅️conformance/🦀️.rs", lambda t: t.rstrip("\n") + "\n" + state["ctl"] + "\n", "conformance.rs")
print("ok")
