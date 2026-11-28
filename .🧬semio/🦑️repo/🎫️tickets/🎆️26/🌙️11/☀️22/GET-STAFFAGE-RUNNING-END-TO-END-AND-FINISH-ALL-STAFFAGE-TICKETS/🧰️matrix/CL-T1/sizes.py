"""⚖️ Where the bytes of the GLB of the default pose go, per tree of lane CL-T1, as Markdown: read from `GEN/light/<id>-glb.log` (`glb.ts`): the file, its JSON chunk, and the accessors owned by the body scenes (authored solids), by the thermal scenes (air flow tubes, puffs, arrows, outlets) and by the structure scenes (members, spheres, force and moment glyphs), in MiB. sizes.py"""
import glob, os, re

GEN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🗑️generated", "matrix", "CL-T1")
print("| Asset | GLB | JSON chunk | Body scenes | Thermal scenes, own | Structure scenes, own | Without the body scenes |")
print("| --- | --- | --- | --- | --- | --- | --- |")
for path in sorted(glob.glob(os.path.join(GEN, "light", "*-glb.log"))):
    text = open(path, encoding="utf-8").read()
    head = re.search(r"glb (\d+) bytes.*json (\d+), bin (\d+)", text)
    if head is None:
        continue
    own = {"body": 0, "thermal": 0, "structure": 0}
    for mode, n in re.findall(r"s\d+-(body|thermal|structure): \d+ meshes, \d+ triangles, (\d+) bytes", text):
        own[mode] += int(n)
    mib = lambda n: f"{int(n) / 1048576:.2f}"
    total = int(head.group(1))
    print(f"| `{os.path.basename(path)[:-8]}` | {mib(total)} | {mib(head.group(2))} | {mib(own['body'])} | {mib(own['thermal'])} | {mib(own['structure'])} | {mib(total - own['body'])} |")
