"""📊️ Compiles every viz gallery document with the ticket build and records errors and overfull boxes per document.

usage: python build-gallery.py <shard index> <shard count>
"""
import io, os, re, subprocess, sys

here = os.path.dirname(os.path.abspath(__file__))
gallery = "C:/semio/🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery"
shard, count = int(sys.argv[1]), int(sys.argv[2])
scratch = os.environ["SEMIO_PRINT_SCRATCH"]
documents = sorted(name for name in os.listdir(gallery) if name.endswith(".tex"))
report = io.open(os.path.join(scratch, "gallery-%d.txt" % shard), "w", encoding="utf-8")
for index, document in enumerate(documents):
    if index % count != shard:
        continue
    name = "gallery-" + re.search(r"viz-\d+", document).group(0)
    output = subprocess.run(["bun", os.path.join(here, "build.ts"), gallery, document, name, "30"], capture_output=True, text=True, encoding="utf-8", env={**os.environ, "SEMIO_PRINT_PASSES": "1"}).stdout
    log = os.path.join(scratch, "out", name, re.search(r"viz-\d+", document).group(0) + ".log")
    overfull = len(set(re.findall(r"Overfull \\hbox \([0-9.]+pt too wide\)[^\n]*", io.open(log, encoding="utf-8", errors="replace").read()))) if os.path.exists(log) else -1
    pages = len([page for page in os.listdir(os.path.join(scratch, "out", name)) if page.endswith(".png")])
    errors = [line for line in output.split("\n") if line.startswith("error")]
    report.write("%s pages=%d overfull=%d errors=%s\n" % (name, pages, overfull, errors[:1]))
    report.flush()
report.write("SHARDDONE\n")
report.close()
