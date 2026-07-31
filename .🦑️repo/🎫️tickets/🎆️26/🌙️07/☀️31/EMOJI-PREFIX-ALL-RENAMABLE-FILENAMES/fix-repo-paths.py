#!/usr/bin/env python3
"""Fix hardcoded .repo paths in repo CLI and TS lib after .🦑️repo migration."""
import os

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
os.chdir(REPO)

GO = "🧰️framework/🛍️product/🦑️repo/🔨️module/💻️client/⌨️cli/⚡️implementation/🐹️go/main.go"
IDX = "🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts"

go_repls = [
    ('filepath.Join(repoRoot, ".repo")', 'filepath.Join(repoRoot, ".🦑️repo")'),
    ('filepath.Join(GetRootDir(), ".repo")', 'filepath.Join(GetRootDir(), ".🦑️repo")'),
    ('filepath.Join(repoRoot, ".repo", "config.toml")', 'filepath.Join(repoRoot, ".🦑️repo", "📋️config.toml")'),
    ('filepath.Join(rootDir, ".repo", "files.json")', 'filepath.Join(rootDir, ".🦑️repo", "files.json")'),
    ('filepath.Join(repoRoot, ".repo", "cache", "breaches")', 'filepath.Join(repoRoot, ".🦑️repo", "⚡️cache", "breaches")'),
    ('filepath.Join(repoRoot, ".repo", "🔀️", yy, mm, dd)', 'filepath.Join(repoRoot, ".🦑️repo", "⚡️cache", "🔀️diff", yy, mm, dd)'),
    ('filepath.Join(repoRoot, ".repo", "compose-micro-commit-bun")', 'filepath.Join(repoRoot, ".🦑️repo", "🐹️compose-micro-commit-bun")'),
    ('filepath.Join(rootDir, ".repo", "⚡️", "🤖️")', 'filepath.Join(rootDir, ".🦑️repo", "⚡️cache", "🤖️generated")'),
    ('filepath.Join(repoRoot, ".repo", "⚡️", "🤖️"', 'filepath.Join(repoRoot, ".🦑️repo", "⚡️cache", "🤖️generated"'),
    ('return filepath.Join(GetRepoMetaDir(), "🎫️")', 'return filepath.Join(GetRepoMetaDir(), "🎫️tickets")'),
    ('return filepath.Join(GetRepoMetaDir(), "🎯️")', 'return filepath.Join(GetRepoMetaDir(), "🎯️goals")'),
    ('filepath.Join(metaRoot, "🎫️", "*", "*", "*", "*")', 'filepath.Join(metaRoot, "🎫️tickets", "*", "*", "*", "*")'),
    ('for _, top := range []string{"🎯️", "👮️", "📝️", "📊️", "✍️"}', 'for _, top := range []string{"🎯️goals", "👮️", "📝️", "📊️metrics", "✍️notes"}'),
    ('if rel == ".repo" || strings.HasPrefix(rel, ".🦑️repo/")', 'if rel == ".🦑️repo" || strings.HasPrefix(rel, ".🦑️repo/")'),
    ('if normalized == ".repo" || strings.HasPrefix(normalized, ".🦑️repo/")', 'if normalized == ".🦑️repo" || strings.HasPrefix(normalized, ".🦑️repo/")'),
    ('entry.Name() != ".repo"', 'entry.Name() != ".🦑️repo"'),
    ('d.Name() != ".repo"', 'd.Name() != ".🦑️repo"'),
    ('".🦑️repo/🧑️\u200d💻️/"', '".🦑️repo/🧑️\u200d💻️devs/"'),
]

idx_repls = [
    ('join(repoRoot, ".repo", "cache", "breaches")', 'join(repoRoot, ".🦑️repo", "⚡️cache", "breaches")'),
    ('join(repoRoot, ".repo", "coverage", kind)', 'join(repoRoot, ".🦑️repo", "📊️metrics", "coverage", kind)'),
    ('ULOC_EXCLUDE_DIRS = [".repo",', 'ULOC_EXCLUDE_DIRS = [".🦑️repo",'),
    ('if (!rel || rel === ".repo" || rel.startsWith(".🦑️repo/"))', 'if (!rel || rel === ".🦑️repo" || rel.startsWith(".🦑️repo/"))'),
    ('/^\\.repo\\/🎫️\\/.+\\/ticket\\.json$/', '/^\\.🦑️repo\\/🎫️tickets\\/.+\\/ticket\\.json$/'),
    ('join(root, ".repo", "🧑️\u200d💻️")', 'join(root, ".🦑️repo", "🧑️\u200d💻️devs")'),
    ('join(root, ".repo", "🧑️\u200d💻️", contributor.alias', 'join(root, ".🦑️repo", "🧑️\u200d💻️devs", contributor.alias'),
    ('mkdirSync(join(root, ".repo")', 'mkdirSync(join(root, ".🦑️repo")'),
    ('join(root, ".repo", MICRO_COMMIT_BUN_PIN)', 'join(root, ".🦑️repo", MICRO_COMMIT_BUN_PIN)'),
    ('ent.name === ".repo"', 'ent.name === ".🦑️repo"'),
    ('ent.name === "node_modules" || ent.name === "dist" || ent.name === ".repo"', 'ent.name === "node_modules" || ent.name === "dist" || ent.name === ".🦑️repo"'),
]

def apply(path, repls):
    t = open(path, encoding="utf-8").read()
    for a, b in repls:
        t = t.replace(a, b)
    open(path, "w", encoding="utf-8").write(t)

apply(GO, go_repls)
apply(IDX, idx_repls)

# nx plugins
for p in [
    "🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/🟨️nx-plugin.mjs",
    "🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/🟨️nx-emoji-project-plugin.mjs",
]:
    t = open(p, encoding="utf-8").read()
    t = t.replace(".repo/cache/breaches", ".🦑️repo/⚡️cache/breaches")
    t = t.replace('includes(".repo")', 'includes(".🦑️repo")')
    open(p, "w", encoding="utf-8").write(t)

# vscode cmake
vs = ".vscode/settings.json"
if os.path.isfile(vs):
    t = open(vs, encoding="utf-8").read().replace(".repo/cache/cmake", ".🦑️repo/⚡️cache/cmake")
    open(vs, "w", encoding="utf-8").write(t)

print("fixed go, index, nx plugins")
