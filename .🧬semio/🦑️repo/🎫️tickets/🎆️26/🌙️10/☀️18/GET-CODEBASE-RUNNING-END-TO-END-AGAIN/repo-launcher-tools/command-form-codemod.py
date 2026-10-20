"""Rewrites the runtime-naming command form in tracked sources, tests and docs to the launcher's script route, so no source names the JavaScript runtime in a command: `bun ./x.ts a` -> `repo ./x.ts a`, `bun "p/📜️script.ts" a` -> `repo "p/📜️script.ts" a`, `bun repo run X` -> `repo run X`, `bun --watch x.ts` -> `repo --watch x.ts`, regex sources `^bun ` -> `^repo `, argv arrays ["bun", "repo", ...] -> ["repo", ...]. Spawns of the runtime by name, `bun test`, `bun install`, `bun -e` and package names are never touched.
Usage: command-form-codemod.py [--root DIR] [--pathspec P ...] [--apply] [--report FILE]. Dry run by default; run together with manifest-neutral.py (same wave) because the consumers parse the form the manifests carry."""
import argparse, json, os, re, subprocess, sys
parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd()); parser.add_argument("--pathspec", action="append", default=[]); parser.add_argument("--apply", action="store_true"); parser.add_argument("--report")
args = parser.parse_args()
root = os.path.abspath(args.root)
files = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", *(args.pathspec or ["."])], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
suffixes = (".ts", ".mjs", ".json", ".md", ".yml", ".yaml", ".feature", ".rs", ".go", ".toml", ".sh", ".ps1")
skip = ("node_modules", "🎫️tickets", ".cursor/", ".codex/plans/", "bun.lock", ".lock", "/💾️development/", "/🖼️assets/📽️")
skip_names = {"📋️project.json", "package.json"}
rules = [
 (re.compile(r"(?<=[`\"'])bun (?=\./|\"|'|\$\{|[^\s\"'`]*📜️script\.ts|[^\s\"'`]*/script\.ts|repo\b|--watch|--cwd)"), "repo "),
 (re.compile(r"\^bun(?=[ (\\])"), "^repo"),
 (re.compile(r"(\[\s*)\"bun\",\s*\"repo\""), r'\1"repo"'),
 (re.compile(r"\bbun repo\b(?=[ `'\")]|$)"), "repo"),
 (re.compile(r"requires one Bun script command"), "requires one script command"),
]
changed, total = {}, 0
for rel in files:
    if not rel or not rel.endswith(suffixes) or any(s in rel for s in skip) or os.path.basename(rel) in skip_names: continue
    full = os.path.join(root, rel)
    try: text = open(full, encoding="utf-8", newline="").read()
    except Exception: continue
    after, count = text, 0
    for pattern, repl in rules:
        after, n = pattern.subn(repl, after); count += n
    if count:
        changed[rel] = count; total += count
        if args.apply: open(full, "w", encoding="utf-8", newline="").write(after)
top = {}
for rel, n in changed.items(): top[rel.split("/")[0] + "/" + (rel.split("/")[1] if "/" in rel else "")] = top.get(rel.split("/")[0] + "/" + (rel.split("/")[1] if "/" in rel else ""), 0) + n
summary = {"dryRun": not args.apply, "filesChanged": len(changed), "replacements": total, "byTopDirectory": dict(sorted(top.items(), key=lambda kv: -kv[1])[:12])}
if args.report: json.dump({**summary, "files": changed}, open(args.report, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps(summary, ensure_ascii=False, indent=1))
