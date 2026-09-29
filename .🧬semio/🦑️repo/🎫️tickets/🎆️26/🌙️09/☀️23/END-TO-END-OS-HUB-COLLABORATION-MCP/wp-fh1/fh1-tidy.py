"""🧹️ FH1 tidy for owner files in the overlay: `|x| app_fault("…")` whose closure ignores `x` → `|_| …`; a use list the
codemods opened as `{app_fault, ⏎` is written `{⏎    app_fault, …`. usage: python3 fh1-tidy.py <path-substring…>"""
import re, subprocess, sys
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
files = subprocess.run(["git", "--git-dir=/Users/ueli/Documents/semio/.git", f"--work-tree={O}", "grep", "-l", "--untracked", "-e", "app_fault", "--", "*.rs"], cwd=O, capture_output=True, text=True).stdout.split("\n")
for path in [f for f in files if f and any(s in f for s in sys.argv[1:])]:
    text = open(O + path).read()
    new = re.sub(r"\|(\w+)\|(\s*(?:[A-Za-z_]\w*::)*app_fault\(\"[^\"]+\"\))(?!\s*\.with_parameter)", r"|_|\2", text)
    new = re.sub(r"\{app_fault, \n(\s+)", r"{\n\1app_fault, ", new)
    if new != text:
        open(O + path, "w").write(new)
        print("tidied", path.split("🔌️plugins/")[-1])
