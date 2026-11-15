"""🐍️ Independent oracle of the Nx failure summary, written without sharing a line with the implementation (standard library only): reads the footer of a transcript (the lists after `Failed tasks:` and `Tasks not run because their dependencies failed`), collects for every failed task the lines under its `> nx run` header and the lines that carry its `task: ` prefix, takes as reason the first usable line that matches the reason pattern (else the last usable line), cut to the declared length, and decides the verdict with the refusal class table. usage: 🐍️.py <refusals.json> <tail schema.json> <vectors.json>"""
import json
import re
import sys


def main():
    table = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]["Table"]["const"]
    policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Summary"]["const"]
    cases = json.load(open(sys.argv[3], encoding="utf-8"))["cases"]
    pattern = re.compile(policy["reasonPattern"])

    def items(lines, heading):
        start = next((i for i, line in enumerate(lines) if re.match(heading, line)), None)
        if start is None:
            return []
        out = []
        for line in lines[start + 1:]:
            if line.startswith("- "):
                out.append(line[2:].strip())
            elif line.strip() == "" and not out:
                continue
            else:
                break
        return out

    def verdict(text):
        for row in table["classes"]:
            if any(m["text"] and m["text"] in text for m in row["messages"]):
                return "no" if row["disposition"] == "fatal" else ("implementation" if row["disposition"] == "reexec" else "inputs")
        return "no"

    result = {}
    for case in cases:
        lines = [line.rstrip("\r") for line in case["tail"].split("\n")]
        failed = items(lines, r"^Failed tasks:\s*$")
        if not failed:
            result[case["id"]] = None
            continue
        sections = {name: [] for name in failed}
        current = None
        for line in lines:
            header = re.match(r"^> nx run (\S+)", line)
            if header:
                current = header.group(1)
                continue
            if line.startswith(" NX "):
                current = None
                continue
            prefixed = re.match(r"^(\S+?): (.*)$", line)
            if prefixed and prefixed.group(1) in sections:
                sections[prefixed.group(1)].append(prefixed.group(2))
            elif current in sections:
                sections[current].append(line)
        rows = []
        for name in failed:
            usable = [l for l in sections[name] if l.strip() and not l.startswith("[script]") and not re.match(r"^\s*\d+ \|", l) and not re.match(r"^\s*\^", l)]
            reason = next((l for l in usable if pattern.search(l)), usable[-1] if usable else "")
            rows.append({"task": name, "reason": reason.strip()[: policy["maximumReasonCharacters"]], "rerun": verdict("\n".join(sections[name]))})
        result[case["id"]] = {"failed": rows, "notRun": items(lines, r"^Tasks not run because their dependencies failed")}
    print(json.dumps(result, ensure_ascii=False))


main()
