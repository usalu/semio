"""🔍 OHR block printer: prints full non-stub diagnostic blocks of an `-it<N>-nonstub.txt` log whose head matches all given regexes (path tokens shortened). usage: blk.py <file> <regex>... [--n N]"""
import re, sys
a = [x for x in sys.argv[2:] if x != "--n"]
n = 6
if "--n" in sys.argv:
    n = int(sys.argv[sys.argv.index("--n") + 1]); a = [x for x in a if x != str(n)]
t = open(sys.argv[1], encoding="utf-8").read()
k = 0
for blk in re.split(r"(?m)^#\d+ ", t)[1:]:
    if all(re.search(r, blk) for r in a):
        s = re.sub(r"__m_semio_framework_os_renderer_wgpu_|🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/", "", blk)
        print(s[:1600]); print("-----")
        k += 1
        if k >= n:
            break
