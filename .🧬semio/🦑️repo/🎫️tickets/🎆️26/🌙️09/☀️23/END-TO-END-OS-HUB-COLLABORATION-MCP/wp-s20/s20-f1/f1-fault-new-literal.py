"""🔒️ S20 F1 (faults overlay): `Fault::new(origin, FaultCode, message)` takes a `FaultCode` — every
`Fault::new(<origin>, "<literal>", …)` (a `&'static str` that used to convert) becomes
`Fault::new(<origin>, FaultCode::new("<literal>"), …)`, `FaultCode` named the way the file names `Fault` (the call's own
path, the file's `use …::{…, Fault, …}` list it joins, or a `use <path>::Fault;`). Every Rust source, tests included.
Idempotent. Usage: python3 f1-fault-new-literal.py <root> [--dry-run]"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
DRY = "--dry-run" in sys.argv
SKIP = {"target", "node_modules", ".git", ".🧬semio", "🗑️generated"}
CALL = re.compile(r'(?P<path>\b(?:[A-Za-z_]\w*::)*)Fault::new\((?P<origin>\s*[^,()"]+(?:\([^()]*\))?\s*),(?P<space>\s*)"(?P<code>[^"\\]+)"')
LIST_USE = re.compile(r"use ((?:::)?[A-Za-z_]\w*(?:::[A-Za-z_]\w*)*)::\{([^}]*)\}\s*;")
SINGLE_USE = re.compile(r"use ((?:::)?[A-Za-z_]\w*(?:::[A-Za-z_]\w*)*)::Fault\s*;")
NAMED = lambda name, names: re.search(rf"(^|[\s,]){name}([\s,]|$)", names)


def sources():
    stack = [ROOT]
    while stack:
        directory = stack.pop()
        for entry in directory.iterdir():
            if entry.is_dir():
                if entry.name not in SKIP and not entry.name.startswith("target-"):
                    stack.append(entry)
            elif entry.suffix == ".rs":
                yield entry


def rewrite(text: str) -> tuple[str, int]:
    matches = list(CALL.finditer(text))
    if not matches:
        return text, 0
    bare = any(match.group("path") == "" for match in matches)
    prefix = ""
    add_to_list = None
    if bare and not re.search(r"\bFaultCode\b", text.replace("FaultCode::new(\"", "")):
        lists = [match for match in LIST_USE.finditer(text) if NAMED("Fault", match.group(2)) and not match.group(2).strip().startswith("self")]
        single = SINGLE_USE.search(text)
        if lists:
            add_to_list = lists[0]
        elif single:
            prefix = f"{single.group(1)}::"
        else:
            prefix = "crate::"
    out = CALL.sub(lambda match: f'{match.group("path")}Fault::new({match.group("origin")},{match.group("space")}{match.group("path") or prefix}FaultCode::new("{match.group("code")}")', text)
    if add_to_list is not None:
        names = add_to_list.group(2)
        start = out.find(add_to_list.group(0))
        body = start + add_to_list.group(0).index("{") + 1
        out = out[:body] + "FaultCode, " + out[body:]
    return out, len(matches)


def main() -> None:
    total = files = 0
    for path in sources():
        text = path.read_text(errors="strict")
        new, count = rewrite(text)
        if count == 0:
            continue
        total += count
        files += 1
        print(f"{count:3d} {path.relative_to(ROOT)}")
        if not DRY:
            path.write_text(new)
    print(f"{'dry-run' if DRY else 'applied'}: {total} literal Fault::new codes in {files} files")


if __name__ == "__main__":
    main()
