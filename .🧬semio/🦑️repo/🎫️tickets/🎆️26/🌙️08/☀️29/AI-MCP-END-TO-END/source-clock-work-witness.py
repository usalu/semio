"""Compare the pinned StepContext source with a zero/single-work clock fast path."""
import json
import os
from pathlib import Path
import re
import shlex
import sqlite3
from statistics import median
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
build = ticket / "🗑️generated/native-mcp/debug/build"
job = Path("🧰️framework/🔨️modules/🧵️job").resolve()
context = Path("⏱️context/🦀️.rs")
crate = "semio_framework_job_context"
channel = re.search(r'^channel\s*=\s*"([^"]+)"', Path("rust-toolchain.toml").read_text(), re.M)[1]
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", channel, "rustc", "--edition=2021", "-Copt-level=3",
         "-Cdebug-assertions=on", "-Coverflow-checks=on", "-Cdebuginfo=0", "-Zthreads=1"]
baseline = "let mut clock=self.clock.get();clock.note_work(units);self.clock.set(clock);"
candidate = "if units > 1 {\n            self.clock.get_mut().note_work(units);\n        }"
rows = {}
for fingerprint in build.glob("*/*/fingerprint/lib-*.json"):
    unit = fingerprint.parent.parent
    name = fingerprint.stem.removeprefix("lib-")
    for extension in ("rlib", "dylib", "so", "dll"):
        artifact = unit / "out" / f"lib{name}-{unit.name}.{extension}"
        if artifact.is_file() and fingerprint.with_suffix("").is_file():
            rows[fingerprint.with_suffix("").read_text().strip()] = (name, artifact, json.loads(fingerprint.read_text()))
            break


def direct(row):
    return {name: rows[key.to_bytes(8, "little").hex()] for _, name, _, key in row[2]["deps"] if name != "build_script_build"}


def paths(row, closure):
    artifact = row[1]
    if artifact not in closure:
        closure.add(artifact)
        for dependency in direct(row).values():
            paths(dependency, closure)


for row in sorted(rows.values(), key=lambda row: row[1].stat().st_mtime, reverse=True):
    if row[0] == crate and json.loads(row[2]["features"]) == []:
        try:
            closure = set()
            paths(row, closure)
            break
        except KeyError:
            continue
else:
    raise SystemExit("Cached native StepContext dependency closure is missing")

search = [part for path in sorted({artifact.parent for artifact in closure}) for part in ("-L", f"dependency={path}")]


def extern(name, artifact):
    files = [artifact.with_suffix(".rmeta"), artifact]
    return [part for file in files if file.is_file() for part in ("--extern", f"{name}={file}")]


externs = [part for name, row in direct(row).items() for part in extern(name, row[1])]
references = row[1].with_name(row[1].name.removeprefix("lib")).with_suffix(".d").read_text().splitlines()[0].split(": ", 1)[1]
sources = {Path(reference).resolve() for reference in shlex.split(references)}
code = (job / context).read_text()
assert code.count(baseline) + code.count(candidate) == 1
fixture = json.loads((ticket / "source-clock-work.json").read_text())
iterations, allowance, charges = (fixture[key] for key in ("iterations", "allowance", "charges"))
with sqlite3.connect(":memory:") as db:
    db.execute("CREATE TABLE work(charge)")
    db.executemany("INSERT INTO work VALUES (?)", [(charge,) for charge in charges])
    independent = db.execute("SELECT ?+sum(charge),?-?-sum(charge) FROM work", (iterations, allowance, iterations)).fetchone()
assert list(independent) == fixture["workReceipt"]
expected = [str(value) for value in [*independent, *fixture["clockReceipt"]]]
timings = {"baseline": [], "candidate": []}
with tempfile.TemporaryDirectory(prefix="ck-", dir=ticket / "🗑️generated") as directory:
    temp = Path(directory)
    for variant, old, new in (("baseline", candidate, baseline), ("candidate", baseline, candidate)):
        source = temp / variant[0]
        for path in sources:
            copy = source / path.relative_to(job)
            assert len(str(copy)) <= 256, copy
            copy.parent.mkdir(parents=True, exist_ok=True)
            if copy != source / context:
                os.link(path, copy)
        (source / context).write_text(code.replace(old, new))
        library = source / f"lib{crate}.rlib"
        subprocess.run([*rustc, "--crate-name", crate, "--crate-type=rlib", *search, *externs, str(source / context), "-o", str(library)], check=True)
        executable = temp / (variant + (".exe" if os.name == "nt" else ""))
        subprocess.run([*rustc, *search, *extern(crate, library), str(ticket / "source-clock-work-witness.rs"), "-o", str(executable)], check=True)

    for i in range(9):
        for variant in timings if i % 2 == 0 else reversed(timings):
            executable = temp / (variant + (".exe" if os.name == "nt" else ""))
            args = [str(value) for value in [iterations, allowance, *charges]]
            elapsed, *observed = subprocess.check_output([str(executable), *args], text=True).split()
            assert observed == expected
            timings[variant].append(int(elapsed))
    print(f"[DEBUG] Receipt and clock proof: {' '.join(expected)}")
for variant, values in timings.items():
    print(f"[DEBUG] {variant} median {median(values) / 1e6:.3f} ms, samples {[round(value / 1e6, 3) for value in values]}")
