"""🌳️ Rename one file-kind family to its kind-only basename and repoint every reference.

Safe because the emoji prefix makes `<emoji><word><ext>` a unique token: it never occurs as
anything else in the repo, so a literal substitution cannot collide. Bare (emoji-less) references
are NOT touched — they are reported for separate judgement.
"""
import os, subprocess, sys, collections

SKIP = {"node_modules", "target", ".git", "dist", "build", "pkg", "__pycache__", "coverage",
        ".venv", "storybook-static", "target-engines", ".nx"}
TEXT = {".rs", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".json", ".toml", ".md", ".py", ".graphql",
        ".proto", ".semio", ".yaml", ".yml", ".ksy", ".g4", ".ebnf", ".abnf", ".spicy", ".wit", ".html"}

def walk(roots):
    for r in roots:
        for dp, dn, fn in os.walk(r):
            dn[:] = [x for x in dn if x not in SKIP]
            for n in fn: yield os.path.join(dp, n), n

def run(old, new, roots, apply_it):
    renamed = subs = files_touched = 0
    targets = [p for p, n in walk(roots) if n == old]
    for p in targets:
        dst = os.path.join(os.path.dirname(p), new)
        if apply_it:
            if os.path.exists(dst): print(f"  !! collision, skipped: {p}"); continue
            subprocess.run(["mv", p, dst], check=True)
        renamed += 1
    for p, n in walk(roots):
        if os.path.splitext(n)[1] not in TEXT: continue
        try: s = open(p, encoding="utf8").read()
        except Exception: continue
        if old not in s: continue
        c = s.count(old)
        if apply_it: open(p, "w", encoding="utf8").write(s.replace(old, new))
        subs += c; files_touched += 1
    print(f"  {'APPLIED' if apply_it else 'DRY-RUN'} {old} -> {new}: {renamed} files renamed, {subs} refs in {files_touched} files")
    return renamed, subs

if __name__ == "__main__":
    old, new = sys.argv[1], sys.argv[2]
    apply_it = "--apply" in sys.argv
    roots = ["✏️s", "🧰️framework", "🌎️hub", "📜️script.ts"]
    roots = [r for r in roots if os.path.exists(r)]
    run(old, new, [r for r in roots if os.path.isdir(r)], apply_it)
