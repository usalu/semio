#!/usr/bin/env python3
"""🔒️ Reject unaudited local references in release HTML and JavaScript."""
import re
import sys
from pathlib import Path

LOCAL = re.compile(r"file://|localhost|127(?:\.\d{1,3}){3}|(?<![0-9a-f:])::1(?![0-9a-f:])", re.I)
HOST = (
    'hub:"http://127.0.0.1:8787",dataDir:".🧬semio/🔗space/s-dev",userDataDir:".🧬semio/🔗space/s-user{N}",localOnly:!0',
    't.startsWith("file://")?"file":t.startsWith("folder://")?"folder":t.startsWith("remote://")?"remote":"unknown"',
    'typeof globalThis.location>"u"?"http://127.0.0.1:7777":globalThis.location.origin',
    'e.hostname==="127.0.0.1"||e.hostname==="localhost"||e.hostname==="[::1]"||e.hostname==="::1"',
    'e==="remote"?"127.0.0.1:8787/studio-1/demo":e==="folder"?"/absolute/project/folder":"/absolute/document.json"',
)
PDF = ('"PDFNodeStream only supports file:// URLs."',)

def scan(root):
    failures = []
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.suffix.lower() not in {".html", ".js", ".mjs"}:
            continue
        try:
            content = path.read_bytes().decode("utf-8")
        except (OSError, UnicodeError) as error:
            failures.append(f"{path}: cannot scan: {error}")
            continue
        allow = HOST if path.parent.name == "assets" and path.name.startswith("🌐️-") and path.suffix == ".js" else PDF if path.parent.name == "assets" and path.name.startswith("pdf-") and path.suffix == ".js" else ()
        spans = [m.span() for fragment in allow for m in re.finditer(re.escape(fragment), content)]
        for match in LOCAL.finditer(content):
            if not any(start <= match.start() and match.end() <= end for start, end in spans):
                line = content.count("\n", 0, match.start()) + 1
                failures.append(f"{path}:{line}: unaudited local literal {match.group()!r}")
    return failures

def main():
    if len(sys.argv) != 2 or not Path(sys.argv[1]).is_dir():
        print("usage: validate-local-references.py SITE", file=sys.stderr)
        return 2
    failures = scan(Path(sys.argv[1]))
    for failure in failures:
        print(failure, file=sys.stderr)
    return 1 if failures else 0

if __name__ == "__main__":
    raise SystemExit(main())
