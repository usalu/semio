"""🏃️ Runs one mutation-case Python adapter through the repository's own Python host with a plan expanded from the case's own feature.

usage: pyrun.py <case dir> <adapter 🐍️.py> <results.jsonl> [local-source dir]
The plan mirrors the coordinator: every Scenario Outline row expands to `<outline base id>-<row id>` with the row values
substituted into step text, doc strings and data tables; plain scenarios keep their `@id-` tag. Every `shared://` and `asset://`
URI named by a step is a declared input. Only the oracle role (the Python adapter) is executed.
"""
import json, os, re, shutil, subprocess, sys, tempfile

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
HOST = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
PYTHON = ROOT + "/.venv/bin/python3"
URI = re.compile(r"\b(shared|local|asset|schema)://([^\s\"'`,;)\]]+)")
STEP = re.compile(r"^(Given|When|Then|And|But|\*)\s+(.*)$")


def cells(line):
    parts = re.split(r"(?<!\\)\|", line.strip())[1:-1]
    return [part.strip().replace("\\|", "|") for part in parts]


def parse_feature(text):
    lines = text.split("\n")
    scenarios, tags, i = [], [], 0
    current = None
    while i < len(lines):
        raw = lines[i]
        line = raw.strip()
        if line.startswith("@"):
            tags.extend(line.split())
            i += 1
            continue
        m = re.match(r"^(Scenario Outline|Scenario|Background):\s*(.*)$", line)
        if m:
            current = {"kind": m.group(1), "title": m.group(2), "tags": tags, "steps": [], "examples": None}
            tags = []
            if m.group(1) != "Background":
                scenarios.append(current)
            i += 1
            continue
        if line.startswith("Examples:"):
            header, rows = None, []
            i += 1
            while i < len(lines) and (lines[i].strip().startswith("|") or not lines[i].strip()):
                if lines[i].strip().startswith("|"):
                    row = cells(lines[i])
                    if header is None:
                        header = row
                    else:
                        rows.append(dict(zip(header, row)))
                i += 1
            current["examples"] = (current["examples"] or []) + rows
            continue
        sm = STEP.match(line) if current is not None else None
        if sm:
            step = {"text": sm.group(2)}
            current["steps"].append(step)
            i += 1
            continue
        if line == '"""' and current is not None and current["steps"]:
            indent = len(raw) - len(raw.lstrip())
            body = []
            i += 1
            while lines[i].strip() != '"""':
                body.append(lines[i][indent:] if lines[i].startswith(" " * indent) else lines[i].lstrip())
                i += 1
            current["steps"][-1]["docString"] = "\n".join(body)
            i += 1
            continue
        if line.startswith("|") and current is not None and current["steps"] and current["examples"] is None:
            table = current["steps"][-1].setdefault("dataTable", [])
            table.append(cells(line))
            i += 1
            continue
        i += 1
    return scenarios


def sub(value, row):
    if isinstance(value, str):
        return re.sub(r"<([^<>]+)>", lambda m: row.get(m.group(1), m.group(0)), value)
    if isinstance(value, list):
        return [sub(v, row) for v in value]
    if isinstance(value, dict):
        return {k: sub(v, row) for k, v in value.items()}
    return value


def expand(scenarios):
    out = []
    for sc in scenarios:
        base = next(t[len("@id-"):] for t in sc["tags"] if t.startswith("@id-"))
        level = next((t[len("@level-"):] for t in sc["tags"] if t.startswith("@level-")), "long")
        if sc["kind"] == "Scenario Outline":
            for row in sc["examples"]:
                first = list(row.values())[0]
                out.append({"id": base + "-" + first, "outlineOf": base, "level": level, "steps": sub(sc["steps"], row)})
        else:
            out.append({"id": base, "level": level, "steps": sc["steps"]})
    return out


def main():
    case_dir, adapter, out = sys.argv[1], sys.argv[2], sys.argv[3]
    local = sys.argv[4:]
    case = os.path.basename(case_dir.rstrip("/"))
    subset = os.path.dirname(os.path.dirname(case_dir.rstrip("/")))
    scenarios = expand(parse_feature(open(os.path.join(case_dir, "🥒️.feature"), encoding="utf-8").read()))
    uris = {}
    for sc in scenarios:
        blob = json.dumps(sc["steps"], ensure_ascii=False)
        for m in URI.finditer(blob):
            scheme, name = m.group(1), m.group(2).rstrip("\\")
            if scheme == "shared":
                uris[m.group(0).rstrip("\\")] = os.path.join(subset, "🧪️tests", "🧫️fixtures", name.rstrip("\\"))
            elif scheme == "asset":
                uris[m.group(0).rstrip("\\")] = os.path.join(subset, "🖼️assets", name.rstrip("\\"))
    inputs = []
    for uri, path in sorted(uris.items()):
        if not os.path.exists(path):
            print("MISSING input", uri, path)
            continue
        inputs.append({"uri": uri, "path": os.path.relpath(path, ROOT)})
    scratch = F + "/🗑️generated"
    work = tempfile.mkdtemp(prefix="c4p51-py-", dir=scratch)
    plan = {"owner": "c4p51", "case": case, "implementation": "python", "role": "oracle", "workDir": os.path.join(work, "work"), "outputDir": os.path.join(work, "out"), "inputs": inputs, "scenarios": scenarios}
    plan_path = os.path.join(work, "plan.json")
    json.dump(plan, open(plan_path, "w", encoding="utf-8"), ensure_ascii=False)
    result = subprocess.run([PYTHON, "-I", "-B", HOST] + [x for d in local for x in ("--local-source", d)] + ["--plan", plan_path, "--out", out, "--adapter", adapter], capture_output=True, text=True, cwd=work, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"})
    print("exit", result.returncode, "scenarios", len(scenarios), "inputs", len(inputs))
    print(result.stdout[-300:], result.stderr[-1200:])
    rows = [json.loads(line) for line in open(out, encoding="utf-8") if line.strip()]
    status = {}
    for row in rows:
        status[row["status"]] = status.get(row["status"], 0) + 1
    print("results", len(rows), status)
    assert os.path.realpath(work).startswith(os.path.realpath(scratch)), work
    shutil.rmtree(work)


main()
