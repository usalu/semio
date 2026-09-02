#!/usr/bin/env python3
import os, shutil

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

def subset_dir(art, ver, sub): return f"{STDIO}/{art}/🏅️standards/🔖️{ver}/🪆️subsets/✳️{sub}"
def artifact_tests(art): return f"{STDIO}/{art}/🧪️tests"

def replace_in_files(paths, old, new, required_in_at_least_one=True):
    total = 0
    for path in paths:
        if not os.path.exists(path):
            continue
        text = open(path, encoding="utf8").read()
        n = text.count(old)
        if n:
            text = text.replace(old, new)
            open(path, "w", encoding="utf8").write(text)
            total += n
    if required_in_at_least_one and total == 0:
        raise AssertionError(f"'{old[:50]}' not found in any of {paths}")
    return total

def move_case(art, case, sub_dir):
    src = f"{artifact_tests(art)}/{case}"
    dst_parent = f"{sub_dir}/🧪️tests"
    os.makedirs(dst_parent, exist_ok=True)
    dst = f"{dst_parent}/{case}"
    shutil.move(src, dst)
    return dst

def copy_flat_fixture(art_fixture_src, sub_dir, rel_name):
    dst = f"{sub_dir}/🧫️fixtures/{rel_name}"
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(art_fixture_src, dst)
    return dst

report = []

# ================= ZIP =================
zip_src = f"{STDIO}/🎒️zip/🧫️fixtures/🗜️.zip"
zip_cases = {"mutate-zip-2-0": "any", "mutate-zip-2-0-iso21320": "iso21320"}
for case, sub in zip_cases.items():
    sd = subset_dir("🎒️zip", "2.0", sub)
    copy_flat_fixture(zip_src, sd, "🗜️.zip")
    dst = move_case("🎒️zip", case, sd)
    report.append(("zip", case, sub, dst))

# ================= PPTX =================
pptx_src = f"{STDIO}/🎞️pptx/🧫️fixtures/📽️.pptx"
pptx_cases = {"mutate-pptx-ecma-376": "any", "mutate-pptx-ecma-376-strict": "strict", "mutate-pptx-ecma-376-transitional": "transitional"}
for case, sub in pptx_cases.items():
    sd = subset_dir("🎞️pptx", "ecma-376", sub)
    copy_flat_fixture(pptx_src, sd, "📽️.pptx")
    dst = move_case("🎞️pptx", case, sd)
    report.append(("pptx", case, sub, dst))

# ================= SVG =================
svg_src = f"{STDIO}/🎨️svg/🧫️fixtures/qr-code.svg"
svg_cases = {"mutate-svg-1-1": "base", "mutate-svg-1-1-tiny": "tiny"}
for case, sub in svg_cases.items():
    sd = subset_dir("🎨️svg", "1.1", sub)
    copy_flat_fixture(svg_src, sd, "qr-code.svg")
    dst = move_case("🎨️svg", case, sd)
    report.append(("svg", case, sub, dst))

# ================= XLSX =================
xlsx_src = f"{STDIO}/📕️xlsx/🧫️fixtures/📕️reuse-marketplaces.xlsx"
xlsx_cases = {"mutate-xlsx-ecma-376": "any", "mutate-xlsx-ecma-376-strict": "strict", "mutate-xlsx-ecma-376-transitional": "transitional"}
for case, sub in xlsx_cases.items():
    sd = subset_dir("📕️xlsx", "ecma-376", sub)
    copy_flat_fixture(xlsx_src, sd, "📕️reuse-marketplaces.xlsx")
    dst = move_case("📕️xlsx", case, sd)
    report.append(("xlsx", case, sub, dst))

# ================= DOCX =================
docx_src = f"{STDIO}/📜️docx/🧫️fixtures/📜️example-readme.docx"
docx_cases = {"mutate-docx-ecma-376": "base", "mutate-docx-ecma-376-strict": "strict", "mutate-docx-ecma-376-transitional": "transitional"}
for case, sub in docx_cases.items():
    sd = subset_dir("📜️docx", "ecma-376", sub)
    copy_flat_fixture(docx_src, sd, "📜️example-readme.docx")
    dst = move_case("📜️docx", case, sd)
    report.append(("docx", case, sub, dst))

# ================= JSON =================
json_src = f"{STDIO}/🔣️json/🧫️fixtures/🔣️.json"
json_cases = {"mutate-json-rfc8259": "base", "mutate-json-rfc8259-i-json": "i-json"}
for case, sub in json_cases.items():
    sd = subset_dir("🔣️json", "rfc8259", sub)
    copy_flat_fixture(json_src, sd, "🔣️.json")
    dst = move_case("🔣️json", case, sd)
    report.append(("json", case, sub, dst))

# ================= JPG (excluding artifact-wide create-and-read-jpeg) =================
jpg_src = f"{STDIO}/📷️jpg/🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg"
jpg_cases = {"mutate-jpg-jfif-1-01": "document", "mutate-jpg-jfif-1-01-baseline": "baseline"}
for case, sub in jpg_cases.items():
    sd = subset_dir("📷️jpg", "jfif-1.01", sub)
    copy_flat_fixture(jpg_src, sd, "🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg")
    dst = move_case("📷️jpg", case, sd)
    report.append(("jpg", case, sub, dst))

# ================= STEP =================
step_src = f"{STDIO}/📐️step/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp"
step_cases = {
    "mutate-step-ap214": "any", "mutate-step-ap214-cc1": "cc1", "mutate-step-ap214-cc2": "cc2",
    "mutate-step-ap214-cc3": "cc3", "mutate-step-ap214-cc4": "cc4", "mutate-step-ap214-cc5": "cc5",
    "mutate-step-ap214-cc6": "cc6",
}
for case, sub in step_cases.items():
    sd = subset_dir("📐️step", "ap214", sub)
    copy_flat_fixture(step_src, sd, "🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp")
    dst = move_case("📐️step", case, sd)
    report.append(("step", case, sub, dst))

# ================= IFC =================
ifc_2x3_src = f"{STDIO}/🏗️ifc/🧫️fixtures/🧪️wellness-center-sama-street-level/🏗️.ifc"
ifc_2x3_cases = {
    "differential-ifc-2x3": "any", "mutate-ifc-2x3": "any",
    "mutate-ifc-2x3-cobie": "cobie", "mutate-ifc-2x3-cv20": "cv20",
}
for case, sub in ifc_2x3_cases.items():
    sd = subset_dir("🏗️ifc", "2x3", sub)
    copy_flat_fixture(ifc_2x3_src, sd, "🧪️wellness-center-sama-street-level/🏗️.ifc")
    dst = move_case("🏗️ifc", case, sd)
    report.append(("ifc", case, "2x3/" + sub, dst))

ifc_4_src = f"{STDIO}/🏗️ifc/🧫️fixtures/🧪️nakagin-capsule-tower/🏗️.ifc"
ifc_4_cases = {"differential-ifc-4": "any", "mutate-ifc-4": "any"}
for case, sub in ifc_4_cases.items():
    sd = subset_dir("🏗️ifc", "4", sub)
    copy_flat_fixture(ifc_4_src, sd, "🧪️nakagin-capsule-tower/🏗️.ifc")
    dst = move_case("🏗️ifc", case, sd)
    report.append(("ifc", case, "4/" + sub, dst))

# ================= DWG (cross-version asset://) =================
DWG_OLD_URI = "asset://🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg"
dwg_src = f"{STDIO}/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg"
sd = subset_dir("🖊️dwg", "ac1024", "any")
dst_dir = f"{sd}/📚️examples/🏛️architectural/🖼️assets"
os.makedirs(dst_dir, exist_ok=True)
shutil.copyfile(dwg_src, f"{dst_dir}/📄️architectural.dwg")
new_uri = "asset://📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg"
case_src = f"{artifact_tests('🖊️dwg')}/mutate-dwg-ac1024"
n = replace_in_files([f"{case_src}/🥒️.feature", f"{case_src}/🦀️.rs"], DWG_OLD_URI, new_uri)
dst = move_case("🖊️dwg", "mutate-dwg-ac1024", sd)
report.append(("dwg", "mutate-dwg-ac1024", "ac1024/any", dst, n))

# ================= TIFF =================
tiff_src = f"{STDIO}/🖼️tiff/🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff"
sd_doc = subset_dir("🖼️tiff", "6.0", "document")
sd_base = subset_dir("🖼️tiff", "6.0", "baseline")
# baseline needs its own copy; document is the case's true fixture-tree ancestor already reachable
# once its case relocates into ✳️document (asset already inside the SAME fixture dir it was in) --
# but the fixture currently lives at ARTIFACT level, so document ALSO needs a copy (its case is
# moving out of the artifact tree).
copy_flat_fixture(tiff_src, sd_doc, "🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff")
copy_flat_fixture(tiff_src, sd_base, "🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff")
dst1 = move_case("🖼️tiff", "mutate-tiff-6-0", sd_doc)
dst2 = move_case("🖼️tiff", "mutate-tiff-6-0-baseline", sd_base)
report.append(("tiff", "mutate-tiff-6-0", "document", dst1))
report.append(("tiff", "mutate-tiff-6-0-baseline", "baseline", dst2))

for r in report:
    print(r)
