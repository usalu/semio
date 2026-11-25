"""🏭 Compiles every Print document (templates, course assignments in both languages, fixtures, the
whole viz gallery) with the ticket build on parallel workers and records errors, overfull boxes and
pages per document in <scratch>/build-all.txt.

usage: python build-all.py [workers] [comma-separated name filters]
"""
import io, os, re, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

here = os.path.dirname(os.path.abspath(__file__))
scratch = os.environ["SEMIO_PRINT_SCRATCH"]
workers = int(sys.argv[1]) if len(sys.argv) > 1 else 4
only = sys.argv[2] if len(sys.argv) > 2 else ""
template = "🧰️framework/🛍️products/📓️print/🧾️template/"
course = "🎓️teaching/🏛️architecture/"
sources = os.path.join(scratch, "sources")
jobs = [
    (here + "/junctions", "junctions.tex", "junctions", 3),
    (here + "/junctions", "chapter.tex", "chapter", 3),
    (template + "📋️report", "📋️report.tex", "report", 3),
    (sources + "/report-dark", "📋️report.tex", "report-dark", 3),
    (template + "🎓️thesis", "🎓️thesis.tex", "thesis", 3),
    (template + "📃️paper", "🎓️paper.tex", "paper", 3),
    (template + "🎓️phd", "🎓️phd.tex", "phd", 3),
    (template + "📰️flyer", "📰️flyer.tex", "flyer", 3),
    (template + "🏗️zukunftbau", "🚧️zwischenbericht.tex", "zwischen", 3),
    (template + "🏗️zukunftbau", "📑️forschungsbericht.tex", "forschung", 3),
    (template + "🏗️zukunftbau", "📝️kompaktbericht.tex", "kompakt", 3),
    (template + "📊️viz-api", "🔓️viz-api.tex", "vizapi", 3),
    (course + "⚡️energy/📝️assignment", "en.tex", "energy", 2),
    (course + "⚡️energy/📝️assignment", "de.tex", "energy-de", 2),
    (sources + "/param", "en.tex", "param-en", 2),
    (sources + "/param", "de.tex", "param-de", 2),
    (sources + "/typo", "en.tex", "typo-en", 2),
    (sources + "/typo", "de.tex", "typo-de", 2),
]
gallery = "C:/semio/" + template + "📊️viz-gallery"
jobs += [(gallery, name, "gallery-" + re.search(r"viz-\d+", name).group(0), 1) for name in sorted(os.listdir(gallery)) if name.endswith(".tex")]
jobs = [job for job in jobs if any(part in job[2] for part in only.split(","))]


def build(job):
    source, main, name, passes = job
    output = subprocess.run(["bun", os.path.join(here, "build.ts"), source, main, name, "40"], capture_output=True, text=True, encoding="utf-8", cwd="C:/semio",
                            env={**os.environ, "SEMIO_PRINT_PASSES": str(passes), "SEMIO_PRINT_TIMEOUT": "1500000"}).stdout
    folder = os.path.join(scratch, "out", name)
    logs = [file for file in os.listdir(folder) if file.endswith(".log")] if os.path.isdir(folder) else []
    text = io.open(os.path.join(folder, logs[0]), encoding="utf-8", errors="replace").read() if logs else ""
    overfull = len(set(re.findall(r"Overfull \\hbox \([0-9.]+pt too wide\)[^\n]*", text)))
    pdfs = [file for file in os.listdir(folder) if file.endswith(".pdf")] if os.path.isdir(folder) else []
    errors = [line for line in output.split("\n") if line.startswith(("error", "!"))]
    return "%s pdf=%s overfull=%d errors=%s" % (name, pdfs[0] if pdfs else "-", overfull, errors[:1])


with io.open(os.path.join(scratch, "build-all.txt"), "w", encoding="utf-8") as report, ThreadPoolExecutor(workers) as pool:
    for line in pool.map(build, jobs):
        report.write(line + "\n")
        report.flush()
    report.write("ALLDONE\n")
