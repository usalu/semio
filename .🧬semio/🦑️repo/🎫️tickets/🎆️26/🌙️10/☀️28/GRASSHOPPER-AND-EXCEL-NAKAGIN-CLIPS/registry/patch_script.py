import os, re
p = os.path.abspath("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts")
d = open(p, "rb").read().decode("utf-8")
anchor = d.index('.📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️"') if '.📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️"' in d else -1
print("anchor", anchor)
if anchor >= 0 and "excel.🎬️clip.🧪️tests.📇️registry" not in d:
    end = d.index('"', anchor) + 1
    d = d[:end] + ', "🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.📇️registry.🐍️"' + d[end:]
    with open(p, "r+b") as f:
        f.seek(0); f.write(d.encode("utf-8")); f.truncate()
