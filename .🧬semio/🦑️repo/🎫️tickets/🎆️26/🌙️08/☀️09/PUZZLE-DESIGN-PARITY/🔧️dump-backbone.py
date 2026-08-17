from pathlib import Path
import re

f = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs")
text = f.read_text()
for key in ["pub struct MemoryBackbone", "impl MemoryBackbone"]:
    j = text.find(key)
    print("\n====", key, "at", j)
    print(text[j : j + 3500] if j >= 0 else "MISSING")

# find fn pair near MemoryBackbone
for m in re.finditer(r"fn pair\(", text):
    snippet = text[max(0, m.start() - 200) : m.start() + 1500]
    if "MemoryBackbone" in snippet:
        print("\n==== pair near MemoryBackbone ====")
        print(snippet)
        break

p = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in p.iterdir() if x.name.endswith("puzzle"))
apps = next(x for x in puzzle.iterdir() if "apps" in x.name)
threed = next(x for x in apps.iterdir() if "3d" in x.name)
comp = next(x for x in threed.iterdir() if "component" in x.name)
text = comp.read_text()
idx = text.find("two_instances_converge_disjoint_object_edits_via_backbone")
print("\n==== 3d test ====")
print(text[idx - 50 : idx + 1000])
