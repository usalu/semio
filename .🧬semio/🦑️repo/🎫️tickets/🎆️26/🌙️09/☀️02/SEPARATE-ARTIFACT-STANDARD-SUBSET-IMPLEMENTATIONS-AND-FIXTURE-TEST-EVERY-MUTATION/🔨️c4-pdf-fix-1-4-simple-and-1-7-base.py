#!/usr/bin/env python3
import os, shutil, hashlib

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)

PDF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf"
TESTS = f"{PDF}/🧪️tests"

def std(v): return f"{PDF}/🏅️standards/🔖️{v}"
def subset_dir(v, s): return f"{std(v)}/🪆️subsets/✳️{s}"

OLD_URI = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf"
NEW_URI_SEED = "asset://📚️examples/🧬️conformance-seed/🖼️assets/📄️conformance-seed.pdf"
NEW_URI_THESIS = "asset://📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf"

def sha256(path):
    return hashlib.sha256(open(path,"rb").read()).hexdigest()

# ---------- 1.4/a and 1.4/x: trivial catalogs, generator seed ----------
SIMPLE_14 = {
    "a": ("mutate-pdf-1-4-a", "set-page-text"),
    "x": ("mutate-pdf-1-4-x", "set-page-size"),
}

OLD_FEATURE_PARA_14 = "  Every scenario copies the committed 65-page thesis before changing it."
NEW_FEATURE_PARA_14 = ("  Every scenario copies this subset's own committed lopdf-generated seed document (the same\n"
                        "  one this catalog's oracle already registers as third-party-generated evidence for its\n"
                        "  per-mutation fixture pairs, in this subset's own 🧪️oracle/🔣️.json, built by this subset's\n"
                        "  own 🏭️generator) before changing it.")

OLD_RS_COMMENT_14 = "//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory\n//! first; the committed asset is never written to."
NEW_RS_COMMENT_14 = "//! Every scenario copies this subset's own committed lopdf-generated seed document (built by\n//! this subset's own 🏭️generator) into the case work directory first; the committed asset is\n//! never written to."

results = []
for subset, (case, kind) in SIMPLE_14.items():
    sub = subset_dir("1.4", subset)
    seed_src = f"{sub}/🧫️fixtures/{kind}/base.pdf"
    seed_dst_dir = f"{sub}/📚️examples/🧬️conformance-seed/🖼️assets"
    seed_dst = f"{seed_dst_dir}/📄️conformance-seed.pdf"
    os.makedirs(seed_dst_dir, exist_ok=True)
    shutil.copyfile(seed_src, seed_dst)

    case_src = f"{TESTS}/{case}"
    feature = f"{case_src}/🥒️.feature"
    rustf = f"{case_src}/🦀️.rs"

    ftext = open(feature, encoding="utf8").read()
    assert OLD_URI in ftext
    n_uri = ftext.count(OLD_URI)
    ftext = ftext.replace(OLD_URI, NEW_URI_SEED)
    assert OLD_FEATURE_PARA_14 in ftext, case
    ftext = ftext.replace(OLD_FEATURE_PARA_14, NEW_FEATURE_PARA_14)
    open(feature, "w", encoding="utf8").write(ftext)

    rtext = open(rustf, encoding="utf8").read()
    assert OLD_URI in rtext
    n_uri_rs = rtext.count(OLD_URI)
    rtext = rtext.replace(OLD_URI, NEW_URI_SEED)
    assert OLD_RS_COMMENT_14 in rtext, case
    rtext = rtext.replace(OLD_RS_COMMENT_14, NEW_RS_COMMENT_14)
    rtext = rtext.replace('Some("bachelor-thesis.pdf")', 'Some("conformance-seed.pdf")')
    open(rustf, "w", encoding="utf8").write(rtext)

    case_dst = f"{sub}/🧪️tests/{case}"
    os.makedirs(f"{sub}/🧪️tests", exist_ok=True)
    shutil.move(case_src, case_dst)
    results.append((case, "1.4", subset, seed_src, sha256(seed_dst), n_uri, n_uri_rs))

# ---------- 1.7/base: duplicate the real bachelor-thesis, URI-only rewrite ----------
thesis_src = f"{std('1.4')}/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf"
base17 = subset_dir("1.7", "base")
thesis_dst_dir = f"{base17}/📚️examples/🎓️bachelor-thesis/🖼️assets"
thesis_dst = f"{thesis_dst_dir}/📄️bachelor-thesis.pdf"
os.makedirs(thesis_dst_dir, exist_ok=True)
shutil.copyfile(thesis_src, thesis_dst)

case = "mutate-pdf-1-7"
case_src = f"{TESTS}/{case}"
feature = f"{case_src}/🥒️.feature"
rustf = f"{case_src}/🦀️.rs"

ftext = open(feature, encoding="utf8").read()
n_uri = ftext.count(OLD_URI)
ftext = ftext.replace(OLD_URI, NEW_URI_THESIS)
open(feature, "w", encoding="utf8").write(ftext)

rtext = open(rustf, encoding="utf8").read()
n_uri_rs = rtext.count(OLD_URI)
rtext = rtext.replace(OLD_URI, NEW_URI_THESIS)
open(rustf, "w", encoding="utf8").write(rtext)

case_dst = f"{base17}/🧪️tests/{case}"
os.makedirs(f"{base17}/🧪️tests", exist_ok=True)
shutil.move(case_src, case_dst)
results.append((case, "1.7", "base", thesis_src, sha256(thesis_dst), n_uri, n_uri_rs))

print("DONE:")
for r in results:
    print(r)
