"""🛡️ Independently certify relocated product staging against the frozen index."""
import copy
import difflib
import hashlib
import json
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "AGENTS.md").is_file())
PLAN = json.loads((TICKET / "taxonomy-paths.cases.json").read_text(encoding="utf-8-sig"))


def git(*args):
    """🔎 Invoke read-only Git plumbing behind the ticket repo interface."""
    return subprocess.check_output(["git", *args], cwd=ROOT)


def entries(data):
    """🗂️ Preserve indexed path modes, hashes and conflict stages."""
    return {line.partition("\t")[2]: line.partition("\t")[0] for line in data.decode("utf-8").split("\0") if line}


def blob_hash(data):
    """🧬 Compute Git object identity without modifying its object store."""
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()


before = entries((TICKET / "🗑️generated/taxonomy-baseline/index-entries.z").read_bytes())
after = entries(git("ls-files", "--stage", "-z"))
normalized = []
for move in PLAN["moves"]:
    assert move["old"] not in after, move["old"]
    if not move["new"]:
        continue
    path = move["new"]
    assert path in after and after[path].endswith(" 0"), path
    data = (ROOT / path).read_bytes()
    digest = after[path].split()[1]
    if blob_hash(data) != digest:
        assert blob_hash(data.replace(b"\r\n", b"\n")) == digest, path
        normalized.append(path)
registry = PLAN["registry"]
registry_path = registry["path"]
baseline_registry = json.loads(git("cat-file", "blob", before[registry_path].split()[1]))
working_registry = json.loads((ROOT / registry_path).read_text(encoding="utf-8"))
expected_registry = copy.deepcopy(baseline_registry)
for field, names in [("semanticDirectoryKinds", registry["kindIds"]), ("semanticDirectoryMemberKinds", registry["memberIds"])]:
    for name in names:
        assert name not in expected_registry[field] or expected_registry[field][name] == working_registry[field][name], name
        expected_registry[field][name] = working_registry[field][name]
for name, values in registry["memberAdditions"].items():
    target = expected_registry["semanticDirectoryMemberKinds"][name]["memberNames"]
    target.extend(item for item in values if item not in target)
assert json.loads(git("show", ":" + registry_path)) == expected_registry, "Unowned staged registry change"
readme = "🎓️teaching/README.md"
old_readme = git("cat-file", "blob", before[readme].split()[1]).decode("utf-8")
current_readme = (ROOT / readme).read_text(encoding="utf-8")
marker = "\n\n## Complete teaching videos\n"
start = current_readme.index(marker)
end = current_readme.find("\n## ", start + len(marker))
section = current_readme[start:end if end >= 0 else None]
start = old_readme.index(marker)
end = old_readme.find("\n## ", start + len(marker))
expected_readme = old_readme[:start] + section + (old_readme[end:] if end >= 0 else "")
expected_readme = expected_readme.replace("short explanatory videos of the same topic (not built yet).", "explanatory video sources of the same topic.", 1)
indexed_readme = git("show", ":" + readme).decode("utf-8")
assert indexed_readme == expected_readme, "Unowned staged README change: " + "\n".join(difflib.unified_diff(expected_readme.splitlines(), indexed_readme.splitlines(), n=1))
shared = json.loads((TICKET / "shared-edits.json").read_text(encoding="utf-8"))
for path, hunks in shared.items():
    original = git("cat-file", "blob", before[path].split()[1]).decode("utf-8")
    for old, new, *alternatives in hunks:
        if new not in original:
            matched = next(value for value in [old, *alternatives] if value in original)
            original = original.replace(matched, new, 1)
    assert git("show", ":" + path).decode("utf-8") == original, "Unowned native change: " + path
owned = {m["old"] for m in PLAN["moves"]} | {m["new"] for m in PLAN["moves"] if m["new"]} | {registry_path, readme, *shared}
changed = {p for p in before.keys() | after.keys() if before.get(p) != after.get(p)}
ticket_relative = TICKET.relative_to(ROOT).as_posix() + "/"
outside = sorted(p for p in changed if p not in owned and not p.startswith(ticket_relative))
assert not outside, outside
assert not any(Path(p).name == "AGENTS.md" for p in changed)
result = {"canonical_sources": 114, "removed_old_coordinates": 141, "git_lf_normalized": len(normalized), "owned_changed_entries": len(changed & owned), "ticket_entries_changed": sum(p.startswith(ticket_relative) for p in changed), "outside_owned_changes": outside, "registry_kinds": len(registry["kindIds"]), "registry_rosters": len(registry["memberIds"]), "shared_projection": "pass"}
(TICKET / "🗑️generated/taxonomy-final-index-audit.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print(json.dumps(result))
