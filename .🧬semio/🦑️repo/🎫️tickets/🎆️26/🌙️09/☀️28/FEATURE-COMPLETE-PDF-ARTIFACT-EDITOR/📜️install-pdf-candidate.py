"""🩹 Check and install explicitly selected PDF source patches on the shared replica without Git mutation."""
import argparse
import hashlib
import os
from pathlib import Path
import re
import stat
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument("--install", action="store_true")
parser.add_argument("patch", type=Path, nargs="+")
options = parser.parse_args()
workspace = Path.cwd().resolve()
generated = (Path(__file__).resolve().parent / "🗑️generated").resolve()
header = re.compile(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@.*\n")
planned = {}
preimages = {}


def product_path(name):
    """📍 Resolves one authorized product source path under the shared workspace."""
    target = (workspace / name).resolve()
    relative = target.relative_to(workspace)
    assert relative.parts[0] in ("🧰️framework", "✏️s", "🌎️hub", "🦑️repo"), relative
    assert "AGENTS.md" not in relative.parts and len(str(target).encode("utf-16-le")) // 2 <= 256, target
    return target


def read(target):
    """📖 Captures the exact current source bytes, including its newline style."""
    return target.read_bytes() if target.is_file() else None


def stage(artifact):
    """🧩 Applies only exact-context hunks to the authorized candidate in memory."""
    path = artifact.resolve()
    assert path.is_relative_to(generated), path
    lines = path.read_bytes().decode("utf8").splitlines(keepends=True)
    i = 0
    while i < len(lines):
        if lines[i].startswith("diff --git "):
            i += 1
        while i < len(lines) and lines[i].startswith(("index ", "new file mode ")):
            i += 1
        assert i + 2 < len(lines), (path, i)
        begin = lines[i].rstrip("\n").split("\t", 1)[0]
        end = lines[i + 1].rstrip("\n").split("\t", 1)[0]
        assert begin.startswith(("--- a/", "--- /dev/null")) and end.startswith("+++ b/"), (path, i)
        target = product_path(end[6:])
        if target not in preimages:
            preimages[target] = read(target)
        source = planned.get(target, preimages[target])
        assert (source is None) == (begin == "--- /dev/null"), target
        if source is not None:
            assert product_path(begin[6:]) == target, (begin, end)
        original = [] if source is None else source.decode("utf8").splitlines(keepends=True)
        candidate = []
        cursor = 0
        i += 2
        while i < len(lines) and (hunk := header.fullmatch(lines[i])):
            before, old_count, _, new_count = map(int, (hunk[1], hunk[2] or "1", hunk[3], hunk[4] or "1"))
            start = before - 1 if old_count else before
            old = []
            new = []
            i += 1
            while len(old) < old_count or len(new) < new_count:
                marker, line = lines[i][0], lines[i][1:]
                assert marker in " +-", (target, i)
                if marker in " -":
                    old.append(line)
                if marker in " +":
                    new.append(line)
                i += 1
            assert len(old) == old_count and len(new) == new_count, (target, i)
            if start < cursor or original[start:start + old_count] != old:
                assert old, (target, before)
                matches = [index for index in range(cursor, len(original) - old_count + 1)
                           if original[index] == old[0] and original[index:index + old_count] == old]
                assert len(matches) == 1, (target, before, matches)
                start = matches[0]
            assert cursor <= start <= len(original), (target, before)
            candidate.extend(original[cursor:start])
            candidate.extend(new)
            cursor = start + old_count
        candidate.extend(original[cursor:])
        updated = "".join(candidate).encode("utf8")
        assert updated and updated != source, target
        planned[target] = updated


for patch in options.patch:
    stage(patch)
assert all(read(path) == original for path, original in preimages.items()), "Replica moved during preflight"
if not options.install:
    print(f"[DEBUG] Verified {len(planned)} exact product preimages")
else:
    with tempfile.TemporaryDirectory(prefix="pdf-write-", dir=generated) as stage_directory:
        staged = Path(stage_directory)
        for index, (target, content) in enumerate(planned.items()):
            original = preimages[target]
            temporary = staged / f"{index}.part"
            temporary.write_bytes(content)
            assert read(target) == original, ("Replica moved before source write", target)
            if original is None:
                target.parent.mkdir(parents=True, exist_ok=True)
                os.link(temporary, target)
                temporary.unlink()
            else:
                temporary.chmod(stat.S_IMODE(target.stat().st_mode))
                assert read(target) == original, ("Replica moved immediately before source write", target)
                temporary.replace(target)
            assert read(target) == content, ("Replica moved immediately after source write", target)
            print("[DEBUG]", str(target.relative_to(workspace)), hashlib.sha256(content).hexdigest())
