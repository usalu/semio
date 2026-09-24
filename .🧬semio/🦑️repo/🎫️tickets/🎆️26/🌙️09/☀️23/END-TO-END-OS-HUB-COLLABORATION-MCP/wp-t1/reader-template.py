"""🔁️ Rewrites a committed serde_json carrier READER onto json-rust, keeping its crate, noun and probe ids."""
import re, sys
path = sys.argv[1]
source = open(path, encoding="utf-8").read()
crate = re.search(r"use (\w+)::project;", source).group(1)
noun = re.search(r"projects a committed (.+?) JSON carrier", source).group(1)
probe = re.search(r'report\("([\w-]+)-project"', source).group(1)
template = open("/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📖️reader/🦀️.rs", encoding="utf-8").read()
out = template.replace("equation_json", crate).replace("a committed equation JSON carrier", f"a committed {noun} JSON carrier").replace("equation-json", probe)
open(path, "w", encoding="utf-8").write(out)
print(crate, noun, probe)
