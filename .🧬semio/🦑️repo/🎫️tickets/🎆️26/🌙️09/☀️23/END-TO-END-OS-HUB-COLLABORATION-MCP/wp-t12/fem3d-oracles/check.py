#!/usr/bin/env python3
"""🧪️ Validates patch.py without touching the tree: copies the five fem3d case adapters into a scratch root, applies the
patch there, then (1) runs every row of every Scenario Outline each committed feature expands through the patched
reference's registered oracle handler, with the real host `Context` and the committed fixtures, and (2) replays every
committed `(before, mutation, after, outcome)` vector of each subset through the patched `apply_mutation`: `applied`
must land on the committed after-model, `rejected` must raise, `no-op` must return the before-model.
Usage: check.py <scratch-dir>"""
import importlib.util
import json
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
scratch = Path(sys.argv[1])
SUBSETS = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
CASES = ["🏋️load/🧪️tests/🏋️mutate-fem3d-1-load", "🧱️material/🧪️tests/🧱️mutate-fem3d-1-material", "🛡️boundary/🧪️tests/🛡️mutate-fem3d-1-boundary", "📈️analysis/🧪️tests/📈️mutate-fem3d-1-analysis", "🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh"]
for case in CASES + ["🌐️any/🧪️tests/🕸️mutate-fem3d-1-any-mesh"]:
    target = scratch / SUBSETS / case
    target.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(REPO / SUBSETS / case / "🐍️.py", target / "🐍️.py")
    shutil.copyfile(REPO / SUBSETS / case / "🥒️.feature", target / "🥒️.feature")
result = subprocess.run([sys.executable, str(Path(__file__).with_name("patch.py")), "--write", "--root", str(scratch)], capture_output=True, text=True)
print(result.stdout.strip(), result.stderr.strip())
if "problems=0" not in result.stdout:
    sys.exit(1)
spec = importlib.util.spec_from_file_location("semio_repo_test", REPO / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)


def outlines(feature_text):
    """🥒️ `(base id, steps, rows)` for every Scenario Outline of one feature; a step is `(text, doc string or None)`."""
    lines = feature_text.split("\n")
    found = []
    for index, line in enumerate(lines):
        tag = line.strip()
        if not tag.startswith("@id-"):
            continue
        base = tag[len("@id-"):]
        steps, table, at = [], [], index + 1
        while not lines[at].strip().startswith("Examples:"):
            text = lines[at].strip()
            if text.split(" ", 1)[0] in ("Given", "And", "When", "Then", "But"):
                steps.append([text, None])
            elif text == '"""':
                end = at + 1
                while lines[end].strip() != '"""':
                    end += 1
                steps[-1][1] = "\n".join(entry.strip() for entry in lines[at + 1:end])
                at = end
            at += 1
        at += 1
        header = [cell.strip() for cell in lines[at].strip().strip("|").split("|")]
        at += 1
        while at < len(lines) and lines[at].strip().startswith("|"):
            table.append(dict(zip(header, [cell.strip() for cell in lines[at].strip().strip("|").split("|")])))
            at += 1
        found.append((base, steps, table))
    return found


def expand(text, row):
    """🪆️ Substitutes one Examples row into a step text or doc string."""
    for key, value in row.items():
        text = text.replace("<%s>" % key, value)
    return text


total = failed = 0
corpus = {"applied": [0, 0], "rejected": [0, 0], "no-op": [0, 0]}
for number, case in enumerate(CASES + ["🌐️any/🧪️tests/🕸️mutate-fem3d-1-any-mesh"]):
    owner = REPO / SUBSETS / case.split("/🧪️tests/")[0]
    module_spec = importlib.util.spec_from_file_location("case_%d" % number, scratch / SUBSETS / case / "🐍️.py")
    module = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(module)
    adapter = module.adapter()
    for base, steps, rows in outlines((scratch / SUBSETS / case / "🥒️.feature").read_text(encoding="utf-8")):
        for row in rows:
            expanded = [{"text": expand(text, row), **({"docString": expand(doc, row)} if doc is not None else {})} for text, doc in steps]
            uris = [token for step in expanded for token in step["text"].split() if token.startswith("shared://")]
            fixtures = [{"uri": uri, "path": str((owner / "🧫️fixtures" / uri[len("shared://"):]).relative_to(REPO))} for uri in uris]
            scenario = {"id": "%s-%s" % (base, row["id"]), "outlineOf": base, "steps": expanded}
            context = host.Context({"workDir": str(scratch / "work"), "fixtures": fixtures, "case": case}, scenario, "oracle", str(REPO))
            handler = adapter.handler(scenario, "oracle")
            total += 1
            if handler is None:
                failed += 1
                print("UNREGISTERED", case.split("/")[-1], scenario["id"])
                continue
            try:
                handler(context)
            except Exception as error:
                failed += 1
                print("FAIL", case.split("/")[-1], scenario["id"], str(error)[:220])
    for vector in sorted((owner / "🧫️fixtures" / "🧬️mutations").glob("*/*")):
        read = lambda relative: json.loads((vector / relative / "🔣️.json").read_text(encoding="utf-8"))
        status = read("🎯️outcome")["status"]
        before = module.document_of(read("📸️snapshot/⬅️before"))
        after = module.document_of(read("📸️snapshot/➡️after"))
        try:
            produced, raised = module.apply_mutation(before, read("🦠️mutation")), None
        except AssertionError as error:
            produced, raised = None, str(error)
        agrees = {"applied": raised is None and produced == after, "rejected": raised is not None, "no-op": raised is None and produced == before}[status]
        corpus[status][0 if agrees else 1] += 1
        if not agrees:
            print("VECTOR", status, vector.relative_to(REPO / SUBSETS), (raised or "applied, and moved the model")[:200])
print(f"rows={total} failed={failed} corpus=" + " ".join("%s %d/%d" % (status, good, good + bad) for status, (good, bad) in corpus.items()))
