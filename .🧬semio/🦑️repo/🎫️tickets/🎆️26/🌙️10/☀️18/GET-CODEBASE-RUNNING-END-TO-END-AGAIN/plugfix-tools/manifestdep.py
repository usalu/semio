#!/usr/bin/env python3
"""📦 plugfix manifest edit: appends one dependency line to a section of a Cargo manifest unless the key is already declared there. usage: manifestdep.py <Cargo.toml> <dependencies|dev-dependencies> '<key = value>'"""
import re, sys


def main():
    path, section, line = sys.argv[1:]
    text = open(path, encoding="utf-8").read()
    key = line.split("=")[0].strip()
    head = "[%s]\n" % section
    start = text.index(head) + len(head)
    match = re.search(r"^\[", text[start:], re.M)
    end = start + match.start() if match else len(text)
    body = text[start:end]
    if re.search(r"^%s\s*[=.]" % re.escape(key), body, re.M):
        return
    body = body.rstrip("\n") + "\n" + line + "\n" + ("\n" if match else "")
    open(path, "w", encoding="utf-8").write(text[:start] + body + text[end:])


if __name__ == "__main__":
    main()
