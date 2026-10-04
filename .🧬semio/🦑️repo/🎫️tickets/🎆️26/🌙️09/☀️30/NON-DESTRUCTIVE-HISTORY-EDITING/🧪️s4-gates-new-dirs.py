"""🗂️ S4-GATES: the directories this ticket created, for `verify taxonomy report --scope <dir>` (gates §5).

New = an ancestor directory of a file the index added since the last auto-commit before the ticket opened (`3eeee4f9119`,
2026-09-30 00:11) that did not exist in that commit's tree. Attributed to the ticket = the new directory holds a file a ticket
markdown names, or a ticket markdown names the directory itself (backticked path or path suffix). Output: the roots of the
attributed new subtrees, one per line — `🗑️generated/s4-gates/new-dirs.txt` (+ `new-dirs-all.txt`, every attributed new dir).
"""
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
T = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
OUT = f"{T}/🗑️generated/s4-gates"
BASE = "3eeee4f9119"
ALIASES = {
    "FW/": "🧰️framework/🔨️modules/",
    "OS/": "🧰️framework/🛍️products/💻️os/🔨️modules/",
    "OSM/": "🧰️framework/🛍️products/💻️os/🔨️modules/",
    "RE/": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/",
    "PZ/": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/",
}


def git(*args: str) -> list[str]:
    out = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf8")
    return [line for line in out.split("\0") if line]


def tokens() -> set[str]:
    found: set[str] = set()
    for doc in os.listdir(os.path.join(ROOT, T)):
        if not doc.endswith(".md"):
            continue
        for token in re.findall(r"`([^`\n]{3,400})`", open(os.path.join(ROOT, T, doc), encoding="utf8").read()):
            token = re.sub(r":[0-9][0-9,\-–]*$", "", token.strip().split(" ")[0].split("::")[0]).rstrip(",;)")
            for alias, full in ALIASES.items():
                if token.startswith(alias):
                    token = full + token[len(alias):]
                    break
            token = token.lstrip("./").replace("…/", "").replace("...", "").rstrip("/")
            if "/" in token or token.startswith(("🧪️", "🧫️", "🧬️", "🔣️")):
                found.add(token)
    return found


def main() -> None:
    base_dirs = set(git("ls-tree", "-d", "-r", "-z", "--name-only", BASE))
    added = [path for path in git("diff", "--cached", "-z", "--name-only", "--diff-filter=A", BASE, "--", ":!.🧬semio") if os.path.exists(os.path.join(ROOT, path))]
    new_dirs: dict[str, list[str]] = {}
    for path in added:
        parts = path.split("/")[:-1]
        for k in range(1, len(parts) + 1):
            directory = "/".join(parts[:k])
            if directory not in base_dirs:
                new_dirs.setdefault(directory, []).append(path)
    named = tokens()
    named_files = {token for token in named if "." in token.split("/")[-1]}
    by_tail: dict[str, set[str]] = {}
    for directory in new_dirs:
        parts = directory.split("/")
        for k in range(1, min(len(parts), 9) + 1):
            by_tail.setdefault("/".join(parts[-k:]), set()).add(directory)
    attributed: set[str] = set()
    for token in named:
        if token in new_dirs:
            attributed.add(token)
        elif 0 < len(by_tail.get(token, ())) <= 3 and "/" in token:
            attributed.update(by_tail[token])
    tails = {token for token in named_files if token.count("/") >= 2}
    named_paths = {file for file in added if file in named_files or any("/".join(file.split("/")[-k:]) in tails for k in range(3, 10))}
    for directory, files in new_dirs.items():
        if any(file in named_paths for file in files):
            attributed.add(directory)
    roots = sorted(d for d in attributed if not any(d.startswith(other + "/") for other in attributed))
    os.makedirs(os.path.join(ROOT, OUT), exist_ok=True)
    open(os.path.join(ROOT, OUT, "new-dirs-all.txt"), "w", encoding="utf8").write("\n".join(sorted(attributed)) + "\n")
    open(os.path.join(ROOT, OUT, "new-dirs.txt"), "w", encoding="utf8").write("\n".join(roots) + "\n")
    print(f"added={len(added)} new-dirs={len(new_dirs)} attributed={len(attributed)} roots={len(roots)}", file=sys.stderr)


if __name__ == "__main__":
    main()
