#!/usr/bin/env python3
"""undeclared-g independent oracle of the hybrid cover: evaluates the RENDERED Seatbelt profile text the way the kernel does (rules in order, the LAST matching rule decides, the profile starts by denying the root) and prints the readable workspace files.
usage: evaluate.py <probe.json>  (probe: {root, files: [...], profiles: {id: profileText}}) -> {id: [readable relative files]}"""
import json, re, sys

probe = json.load(open(sys.argv[1], encoding="utf-8"))
root = probe["root"]


def unquote(text):
    return text.replace('\\"', '"').replace("\\\\", "\\")


def readable(profile, files):
    rules = []
    for line in profile.splitlines():
        match = re.fullmatch(r'\((allow|deny) file-read-data \((subpath|literal) "(.*)"\)\)', line)
        if match:
            rules.append((match.group(1), match.group(2), unquote(match.group(3))))
    out = []
    for rel in files:
        path = root + "/" + rel
        verdict = False
        for action, kind, base in rules:
            hit = path == base or (kind == "subpath" and path.startswith(base + "/"))
            if hit:
                verdict = action == "allow"
        out.append((rel, verdict))
    return sorted(rel for rel, ok in out if ok)


print(json.dumps({key: readable(text, probe["files"]) for key, text in probe["profiles"].items()}))
