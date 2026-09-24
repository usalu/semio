"""⚠️ M5b — inserts `.action_destructive("<id>")` directly after the verb's own unique
`.action_interactive_job("<id>", …)` chain step (the same chain, the same id), keeping `.await` when
that step carries one. Refuses unless exactly one such line exists in the plugin tree (or in the
artifact subtree given as a path, for a plugin that declares the same id once per artifact)."""
import glob, re, sys
plugin, ids = sys.argv[1], sys.argv[2:]
files = [f for f in glob.glob(f"✏️s/🔌️plugins/{plugin}/**/🦀️.rs", recursive=True) if "🧪️" not in f] if plugin.count("/") == 0 else [f for f in glob.glob(f"{plugin}/**/🦀️.rs", recursive=True) if "🧪️" not in f]
for verb in ids:
    pattern = re.compile(r'^(\s*)\.action_interactive_job\("' + re.escape(verb) + r'",.*?(\.await)?\s*$')
    hits = [(f, i) for f in files for i, line in enumerate(open(f, encoding="utf8").read().split("\n")) if pattern.match(line)]
    if len(hits) != 1:
        print(f"REFUSED {plugin}.{verb}: {len(hits)} anchor(s) {hits[:3]}")
        continue
    f, i = hits[0]
    lines = open(f, encoding="utf8").read().split("\n")
    if any(f'.action_destructive("{verb}")' in line for line in lines):
        print(f"SKIP {plugin}.{verb}: already declared")
        continue
    m = pattern.match(lines[i])
    lines.insert(i + 1, f'{m.group(1)}.action_destructive("{verb}")' + (".await" if m.group(2) else ""))
    open(f, "w", encoding="utf8").write("\n".join(lines))
    print(f"OK {plugin}.{verb} -> {f}:{i + 2}")
