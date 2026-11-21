"""🧷 Prepares and verifies the reviewed PDF progress release against exact source digests."""

import argparse
import hashlib
import json
from pathlib import Path
import re

TICKET = Path(__file__).resolve().parent
PATCH = TICKET / "📜️registered-png-progress.patch"
RECEIPT = TICKET / "🗑️generated" / "image-progress-release" / "release.json"
CONTROLLER = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs")
PREIMAGE = "b5ec68ca591c9f17645612b70b2007c991e6d82d8e94e8ed6f782d52d0b002df"
POSTIMAGE = "05dd506f3c6a28a6838f53cfac25fdae997fca91eaad46fb705b5e9e3409a5a9"


def digest(source):
    return hashlib.sha256(source).hexdigest()


def candidate(root):
    target = root / CONTROLLER
    if len(str(target.resolve()).encode("utf-16-le")) // 2 > 256:
        raise ValueError(f"Controller path exceeds the workspace limit: {target}")
    before = target.read_bytes()
    if digest(before) != PREIMAGE:
        raise ValueError(f"Controller preimage changed: {target}")
    patch = PATCH.read_bytes()
    path = CONTROLLER.as_posix()
    headers, *lines = patch.decode("utf-8").splitlines(keepends=True)
    if headers != f"--- a/{path}\n" or not lines or lines.pop(0) != f"+++ b/{path}\n":
        raise ValueError("The reviewed patch does not address the PDF controller")
    source = before.decode("utf-8").splitlines(keepends=True)
    output, cursor, captures = [], 0, []
    while lines:
        hunk = re.fullmatch(r"@@ -(\d+),(\d+) \+(\d+),(\d+) @@\n", lines.pop(0))
        if not hunk:
            raise ValueError("The reviewed patch has an invalid hunk header")
        old_line, old_count, new_line, new_count = map(int, hunk.groups())
        start = old_line - 1
        output.extend(source[cursor:start])
        if not (start >= cursor and len(output) == new_line - 1):
            raise ValueError("The reviewed patch has overlapping hunks or changed positions")
        old, new, added = [], [], []
        while lines and not lines[0].startswith("@@"):
            sign, *rest = lines.pop(0)
            value = "".join(rest)
            if sign not in " -+":
                raise ValueError("The reviewed patch has an invalid hunk line")
            if sign != "+":
                old.append(value)
            if sign != "-":
                new.append(value)
            if sign == "+":
                added.append(value)
        cursor = start + old_count
        if len(old) != old_count or len(new) != new_count or source[start:cursor] != old:
            raise ValueError("The reviewed patch hunk does not match the controller")
        output.extend(new)
        captures.append("".join(added))
    after = "".join(output + source[cursor:]).encode("utf-8")
    if digest(after) != POSTIMAGE:
        raise ValueError("The staged controller postimage changed")
    return target, after, {"path": path, "preimage": PREIMAGE, "postimage": POSTIMAGE, "patch": digest(patch)}, captures


def release(root, mode):
    target, after, observed, _ = candidate(root)
    if mode == "--prepare":
        RECEIPT.parent.mkdir(parents=True, exist_ok=True)
        RECEIPT.write_text(json.dumps(observed, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        print("[DEBUG] Prepared the PDF PNG progress preimage and staged postimage")
        return
    if json.loads(RECEIPT.read_text(encoding="utf-8")) != observed:
        raise ValueError("The reviewed release receipt changed")
    if mode == "--apply-reviewed":
        if digest(target.read_bytes()) != PREIMAGE:
            raise ValueError("Controller changed after release verification")
        target.write_bytes(after)
        if digest(target.read_bytes()) != POSTIMAGE:
            raise ValueError("Written controller postimage differs from the reviewed receipt")
    print("[DEBUG] " + ("Released" if mode == "--apply-reviewed" else "Verified") + " the PDF PNG progress controller")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path)
    action = parser.add_mutually_exclusive_group(required=True)
    for mode in ("--prepare", "--verify", "--apply-reviewed"):
        action.add_argument(mode, action="store_const", dest="mode", const=mode)
    args = parser.parse_args()
    release(args.root, args.mode)
