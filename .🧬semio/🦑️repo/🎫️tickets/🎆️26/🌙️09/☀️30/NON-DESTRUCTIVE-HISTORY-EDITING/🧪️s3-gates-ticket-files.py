"""🗂️ S3-GATES: the source files this ticket touched, resolved from every report/design/status markdown in the ticket folder.

Backticked tokens are taken as written, as repo-root paths, or (for abbreviated paths) as a suffix of exactly one tracked file;
alias prefixes (`PLG`, `TT`, `STORE`, `RE`, `PZ`, `FW`, `OS`, `DEV`, `G2`, `G3`) are expanded from the reports' own legends.
Output: one repo-relative path per line, sources only (rs, ts, tsx, py, json, feature, semio) — `🗑️generated/s3-gates/ticket-files.txt`.
"""
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
T = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
OUT = f"{T}/🗑️generated/s3-gates/ticket-files.txt"
ALIASES = {
    "FW/": "🧰️framework/🔨️modules/",
    "OS/": "🧰️framework/🛍️products/💻️os/🔨️modules/",
    "PLG": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "TT": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs",
    "STORE": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs",
    "RE/": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/",
    "PZ/": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/",
}
EXTS = (".rs", ".ts", ".tsx", ".py", ".json", ".feature", ".semio", ".toml")
ROOTS = ("🧰️framework/", "✏️s/", "🌎️hub/", "🎓️teaching/", "♻️mit-bestand/", "📜️script.ts", "package.json", ".vscode/")


def tracked() -> list[str]:
    listed = subprocess.run(["git", "ls-files", "-z", "--", ":!.🧬semio"], cwd=ROOT, capture_output=True, check=True)
    return [p for p in listed.stdout.decode("utf8").split("\0") if p]


def main() -> None:
    files = tracked()
    present = set(files)
    by_tail: dict[str, list[str]] = {}
    for path in files:
        parts = path.split("/")
        for k in range(2, min(len(parts), 9) + 1):
            by_tail.setdefault("/".join(parts[-k:]), []).append(path)
    found: set[str] = set()
    unresolved = 0
    docs = [f for f in os.listdir(os.path.join(ROOT, T)) if f.endswith(".md")]
    for doc in docs:
        text = open(os.path.join(ROOT, T, doc), encoding="utf8").read()
        for token in re.findall(r"`([^`\n]{3,400})`", text):
            token = token.strip().split(" ")[0].split("::")[0]
            token = re.sub(r":[0-9][0-9,\-–]*$", "", token).rstrip(",;)")
            for alias, full in ALIASES.items():
                if (alias.endswith("/") and token.startswith(alias)) or token == alias:
                    token = full + token[len(alias):]
                    break
            token = token.lstrip("./").replace("…/", "").replace("...", "")
            if not token.endswith(EXTS):
                continue
            if token in present:
                found.add(token)
                continue
            candidates = by_tail.get(token, [])
            if 0 < len(candidates) <= 3:
                found.update(candidates)
            else:
                unresolved += 1
    with open(os.path.join(ROOT, OUT), "w", encoding="utf8") as out:
        out.write("\n".join(sorted(found)) + "\n")
    print(f"docs={len(docs)} files={len(found)} unresolved-tokens={unresolved} -> {OUT}", file=sys.stderr)


if __name__ == "__main__":
    main()
