"""🧩 Compose the guarded natural-PDF source candidates without writing product files."""
from difflib import SequenceMatcher, unified_diff
from hashlib import sha256
from pathlib import Path
import json
import subprocess
import sys

ticket = Path(__file__).resolve().parent
workspace = Path.cwd().resolve()
generated = ticket / "🗑️generated"
full_candidate = "--first-archive" in sys.argv[1:]
fonts = "--base14" in sys.argv[1:]
arguments = tuple(arg for arg in sys.argv[1:] if arg not in ("--first-archive", "--base14"))
sources = (
    "pdf-compressed-byte-install-overlay.json",
    "pdf-paged-decoder-overlay.json",
    "paged-octet-retirement-overlay.json",
)
if full_candidate:
    sources += (
        "natural-source-browser-mounted/overlay.json",
        "browser-first-archive-publication/overlay.json",
        "browser-first-archive-client/overlay.json",
        "pdf-inspector-owner/overlay.json",
    )
else:
    sources += ("natural-source-browser/overlay.json",)
if fonts:
    sources += ("pdf-base14/symbolic-code-overlay.json", "pdf-base14/font-package-overlay.json")
patches = tuple(Path(arg).resolve() for arg in arguments)
files = {}
edits = {}
body = []


def source_path(name):
    """📏 Validates one normalized native source path under the shared replica."""
    relative = Path(name)
    full = (workspace / relative).resolve()
    assert full.is_relative_to(workspace) and len(str(full).encode("utf-16-le")) // 2 <= 256, name
    return relative, full


def merge_disjoint(original, updates, name):
    """🧵 Joins disjoint changes that were independently staged against exactly the same preimage."""
    if len(updates) == 1:
        return updates[0]
    base = original.splitlines(keepends=True)
    hunks = {}
    for update in updates:
        revised = update.splitlines(keepends=True)
        for tag, start, end, first, last in SequenceMatcher(None, base, revised, autojunk=False).get_opcodes():
            if tag == "equal":
                continue
            interval = start, end
            replacement = revised[first:last]
            assert interval not in hunks or hunks[interval] == replacement, (name, interval)
            hunks[interval] = replacement
    cursor = 0
    merged = []
    for (start, end), replacement in sorted(hunks.items()):
        assert start >= cursor, (name, start, cursor)
        merged.extend(base[cursor:start])
        merged.extend(replacement)
        cursor = end
    merged.extend(base[cursor:])
    return "".join(merged)


for source in sources:
    for name, digest, updated in json.loads((generated / source).read_text(encoding="utf8")):
        relative, full = source_path(name)
        files.setdefault(relative, []).append(source)
        current = full.read_bytes() if full.is_file() else None
        assert (sha256(current).hexdigest() if current is not None else None) == digest, (name, source)
        if relative not in edits:
            edits[relative] = current, []
        previous, updates = edits[relative]
        assert previous == current, (name, source)
        assert updated.endswith("\n"), ("Candidate lacks final newline", name)
        updates.append(updated)
for relative, (current, updates) in edits.items():
    existing = "" if current is None else current.decode("utf8")
    assert not existing or existing.endswith("\n"), ("Original lacks final newline", relative)
    assert current is not None or len(updates) == 1, (relative, files[relative])
    updated = merge_disjoint(existing, updates, relative)
    before = "/dev/null" if current is None else f"a/{relative.as_posix()}"
    body.extend(unified_diff(existing.splitlines(True), updated.splitlines(True), before, f"b/{relative.as_posix()}"))
for artifact in patches:
    assert artifact.is_relative_to(generated), artifact
    data = artifact.read_text(encoding="utf8")
    for line in data.splitlines():
        if not line.startswith("+++ b/"):
            continue
        relative, _ = source_path(line[6:].split("\t",1)[0])
        assert relative not in files, (relative, files.get(relative), artifact)
        files[relative] = artifact.name
    body.append(data)
name = "pdf-natural-source-candidate.patch"
if fonts:
    assert full_candidate, "Base 14 overlays require complete first archive composition"
    name = "pdf-natural-first-archive-base14-candidate.patch"
    assets = json.loads((generated / "pdf-base14" / "font-asset-overlay.json").read_text(encoding="utf8"))
    from base64 import b64decode
    for asset in assets:
        relative, full = source_path(asset["path"])
        assert relative not in files and not full.exists(), relative
        assert sha256(b64decode(asset["base64"],validate=True)).hexdigest() == asset["sha256"], relative
    print(f"[DEBUG] Verified {len(assets)} pinned font and license assets.")
elif full_candidate:
    name = "pdf-natural-first-archive-candidate.patch"
destination = generated / name
destination.write_text("".join(body),encoding="utf8")
subprocess.run(["repo","tools","exec","--light","git","--","apply","--check",str(destination)],check=True,cwd=workspace)
print(f"[DEBUG] Composed and checked {len(files)} current source paths without changing the replica.")
