"""📝️ Writes RESULT-RENDERER-TEST-BINS.md from the live census and the bin roots. usage: result_gen.py [verdict-file]"""
import subprocess, sys
from lib import *

def run(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, cwd=HERE).stdout.strip()

census = run(["python3", "census_table.py"])
bins = run(["python3", "bins_table.py"])
extra = open(sys.argv[1], encoding="utf8").read() if len(sys.argv) > 1 else ""
baseline = open(SPLIT + "/RESULT-RENDERER-TEST-BINS.md", encoding="utf8").read()
base_section = baseline[baseline.index("## 1. Baseline"):baseline.index("(The remaining sections")] if "(The remaining sections" in baseline else baseline[baseline.index("## 1. Baseline"):]
doc = open(HERE + "/result_template.md", encoding="utf8").read()
doc = doc.replace("{{VERDICT}}", open(HERE + "/result_verdict.md", encoding="utf8").read().strip()).replace("{{BASELINE}}", base_section.strip()).replace("{{CENSUS}}", census).replace("{{BINS}}", bins).replace("{{EXTRA}}", extra)
write(SPLIT + "/RESULT-RENDERER-TEST-BINS.md", doc)
print(len(doc.split("\n")), "lines")
