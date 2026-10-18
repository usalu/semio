#!/usr/bin/env python3
"""🔒 plugfix lock edit: inserts one dependency edge into a Cargo.lock package entry, keeping cargo's sorted order. usage: lockdep.py <Cargo.lock> <package> <dependency>..."""
import re, sys


def add(lock, package, dependency):
    m = re.search(r'name = "%s"\nversion = "[^"]*"\n(?:source = "[^"]*"\n)?dependencies = \[\n(.*?)\n\]' % re.escape(package), lock, re.S)
    if not m:
        raise SystemExit(f"package {package} has no dependencies array in the lock")
    rows = m.group(1).split("\n")
    entry = ' "%s",' % dependency
    if entry in rows:
        return lock
    rows.append(entry)
    rows.sort(key=lambda row: row.strip().strip(",").strip('"'))
    return lock[: m.start(1)] + "\n".join(rows) + lock[m.end(1) :]


def main():
    path, package, *deps = sys.argv[1:]
    lock = open(path, encoding="utf-8").read()
    for dep in deps:
        if 'name = "%s"' % dep not in lock:
            raise SystemExit(f"dependency {dep} is not a package of the lock")
        lock = add(lock, package, dep)
    open(path, "w", encoding="utf-8").write(lock)


if __name__ == "__main__":
    main()
