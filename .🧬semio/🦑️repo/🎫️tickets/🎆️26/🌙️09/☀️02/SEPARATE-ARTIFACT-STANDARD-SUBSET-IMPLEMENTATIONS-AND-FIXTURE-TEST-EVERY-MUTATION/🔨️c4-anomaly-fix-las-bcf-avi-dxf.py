#!/usr/bin/env python3
import os, shutil, re

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

def artifact_tests(art): return f"{STDIO}/{art}/🧪️tests"
def subset_dir(art, ver, sub): return f"{STDIO}/{art}/🏅️standards/🔖️{ver}/🪆️subsets/✳️{sub}"

def move_case(case_src, case_dst_parent):
    os.makedirs(case_dst_parent, exist_ok=True)
    dst = f"{case_dst_parent}/{os.path.basename(case_src)}"
    shutil.move(case_src, dst)
    return dst

def replace_in_file(path, old, new, required=True):
    if not os.path.exists(path):
        return 0
    text = open(path, encoding="utf8").read()
    n = text.count(old)
    if required and n == 0:
        raise AssertionError(f"'{old[:60]}...' not found in {path}")
    if n:
        text = text.replace(old, new)
        open(path, "w", encoding="utf8").write(text)
    return n

report = []

# ---------------- LAS cluster ----------------
LAS_SHARED = "🧪️pattern-sphere/🧊️.las"
las_src_fixture = f"{STDIO}/☁️las/🧫️fixtures/{LAS_SHARED}"
las_cases = {
    "mutate-las-1-0": "header",
    "mutate-las-1-0-points": "points",
    "mutate-las-1-0-vlr": "vlr",
}
for case, sub in las_cases.items():
    sub_dir = subset_dir("☁️las", "1.0", sub)
    dst_fixture_dir = f"{sub_dir}/🧫️fixtures/🧪️pattern-sphere"
    os.makedirs(dst_fixture_dir, exist_ok=True)
    shutil.copyfile(las_src_fixture, f"{dst_fixture_dir}/🧊️.las")
    case_src = f"{artifact_tests('☁️las')}/{case}"
    feature = f"{case_src}/🥒️.feature"
    n = replace_in_file(feature, "shared://🧪️pattern-sphere/🧊️.las", "shared://🧪️pattern-sphere/🧊️.las")  # scheme text unchanged
    dst = move_case(case_src, f"{sub_dir}/🧪️tests")
    report.append(("las", case, sub, dst))

# ---------------- BCF cluster ----------------
BCF_SHARED = "wellness-center-coordination-review.bcf"
bcf_src_fixture = f"{STDIO}/💬️bcf/🧫️fixtures/{BCF_SHARED}"
bcf_cases = {
    "mutate-bcf-2-1": "markup",
    "mutate-bcf-2-1-snapshot": "snapshot",
    "mutate-bcf-2-1-viewpoint": "viewpoint",
}
for case, sub in bcf_cases.items():
    sub_dir = subset_dir("💬️bcf", "2.1", sub)
    dst_fixture_dir = f"{sub_dir}/🧫️fixtures"
    os.makedirs(dst_fixture_dir, exist_ok=True)
    shutil.copyfile(bcf_src_fixture, f"{dst_fixture_dir}/{BCF_SHARED}")
    case_src = f"{artifact_tests('💬️bcf')}/{case}"
    dst = move_case(case_src, f"{sub_dir}/🧪️tests")
    report.append(("bcf", case, sub, dst))

# ---------------- AVI cluster ----------------
AVI_SHARED = "🎬️.avi"
avi_src_fixture = f"{STDIO}/📼️avi/🧫️fixtures/{AVI_SHARED}"
avi_cases = {
    "mutate-avi-1-0": "hdrl",
    "mutate-avi-1-0-idx1": "idx1",
    "mutate-avi-1-0-movi": "movi",
}
for case, sub in avi_cases.items():
    sub_dir = subset_dir("📼️avi", "1.0", sub)
    dst_fixture_dir = f"{sub_dir}/🧫️fixtures"
    os.makedirs(dst_fixture_dir, exist_ok=True)
    shutil.copyfile(avi_src_fixture, f"{dst_fixture_dir}/{AVI_SHARED}")
    case_src = f"{artifact_tests('📼️avi')}/{case}"
    dst = move_case(case_src, f"{sub_dir}/🧪️tests")
    report.append(("avi", case, sub, dst))

# ---------------- DXF cluster ----------------
DXF_OLD_URI = "asset://🏅️standards/🔖️r12/🪆️subsets/✳️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf"
dxf_src_asset = f"{STDIO}/🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️header/📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf"
dxf_cases = {
    "mutate-dxf-r12": "header",       # already at header, no copy needed
    "mutate-dxf-r12-blocks": "blocks",
    "mutate-dxf-r12-entities": "entities",
    "mutate-dxf-r12-tables": "tables",
}
for case, sub in dxf_cases.items():
    sub_dir = subset_dir("🖊️dxf", "r12", sub)
    if sub != "header":
        dst_asset_dir = f"{sub_dir}/📚️examples/🚏️bus-shelter/🖼️assets"
        os.makedirs(dst_asset_dir, exist_ok=True)
        shutil.copyfile(dxf_src_asset, f"{dst_asset_dir}/🖊️.dxf")
    new_uri = "asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf"
    case_src = f"{artifact_tests('🖊️dxf')}/{case}"
    feature = f"{case_src}/🥒️.feature"
    rustf = f"{case_src}/🦀️.rs"
    n1 = replace_in_file(feature, DXF_OLD_URI, new_uri)
    n2 = replace_in_file(rustf, DXF_OLD_URI, new_uri, required=False)
    dst = move_case(case_src, f"{sub_dir}/🧪️tests")
    report.append(("dxf", case, sub, dst, n1, n2))

for r in report:
    print(r)
