"""Exact, fail-closed replacements that take `.git/shallow` off the replica's shared allow-list."""
import sys,pathlib
D=pathlib.Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree")
EDITS=[
 ("🚧️boundary/🟦️.ts",'const GIT_SHARED_FILES=new Set(["HEAD","packed-refs","shallow"]);','const GIT_SHARED_FILES=new Set(["HEAD","packed-refs"]);'),
 ("🚧️boundary/🟦️.ts",'the allow-list is `HEAD`, `packed-refs`, `shallow`, everything below `refs` and everything below `objects` except temporary objects (`tmp_` prefix in `objects`), `objects/info/alternates` and every name ending `.lock`; the `.git` directory itself is shared, every other path (index, config, logs, hooks, info, in-progress operation files, IDE files) stays on the machine.','the allow-list is `HEAD`, `packed-refs`, everything below `refs` and everything below `objects` except temporary objects (`tmp_` prefix in `objects`), `objects/info/alternates` and every name ending `.lock`; the `.git` directory itself is shared, every other path (index, config, logs, hooks, info, `shallow`, in-progress operation files, IDE files) stays on the machine: a `shallow` states what the own object store lacks, so a copy elsewhere would cut a full history.'),
 ("🚧️boundary/🧪️tests/🐍️.py",'name in ("HEAD","packed-refs","shallow")','name in ("HEAD","packed-refs")'),
 ("🚧️boundary/🧪️tests/🧫️fixtures/🔣️.json",'{"path": ".git/shallow", "shared": true}','{"path": ".git/shallow", "shared": false}'),
 ("🩺️attest/🟦️.ts",'`.git/packed-refs`, `.git/shallow` and every `.gitignore`','`.git/packed-refs` and every `.gitignore`'),
 ("🩺️attest/🟦️.ts",'path===".git/packed-refs"||path===".git/shallow"||path.startsWith(".git/refs/heads/")','path===".git/packed-refs"||path.startsWith(".git/refs/heads/")'),
 ("🩺️attest/🧪️tests/🐍️agreement.py",'path in (".git/HEAD",".git/packed-refs",".git/shallow")','path in (".git/HEAD",".git/packed-refs")'),
 ("🩺️attest/🧪️tests/🧫️fixtures/🔣️.json",'    "path": ".git/shallow",\n    "git": true','    "path": ".git/shallow",\n    "git": false'),
 ("🔁️replica/🧲️adopt/🟦️.ts",'path===".git/packed-refs"||path===".git/shallow"||path.startsWith(".git/refs/")&&!path.startsWith(".git/refs/heads/")','path===".git/packed-refs"||path.startsWith(".git/refs/")&&!path.startsWith(".git/refs/heads/")'),
 ("🔁️replica/🟦️.ts",'for every shared `.git` path (`HEAD`, `packed-refs`, `shallow`, `refs/**`, `objects/**`)','for every shared `.git` path (`HEAD`, `packed-refs`, `refs/**`, `objects/**`)'),
 ("🔁️replica/🟦️.ts",'a `HEAD`, `packed-refs`, `shallow` or ref file of another content','a `HEAD`, `packed-refs` or ref file of another content'),
 ("🔁️replica/🟦️.ts",'path===".git/packed-refs"||path===".git/shallow"||path.startsWith(".git/refs/")||path.startsWith(objectsPrefix)','path===".git/packed-refs"||path.startsWith(".git/refs/")||path.startsWith(objectsPrefix)'),
 ("README.md",'The root `.git` is shared by an allow-list: `HEAD`, `packed-refs`, `shallow`, everything below `refs`, and everything below `objects`\nexcept names starting `tmp_`, `objects/info/alternates` and every name ending `.lock`. Everything else in `.git` (index, config,\nlogs, hooks, info, `FETCH_HEAD`, `ORIG_HEAD`, in-progress operation files, IDE files) stays on the machine that wrote it.','The root `.git` is shared by an allow-list: `HEAD`, `packed-refs`, everything below `refs`, and everything below `objects`\nexcept names starting `tmp_`, `objects/info/alternates` and every name ending `.lock`. Everything else in `.git` (index, config,\nlogs, hooks, info, `shallow`, `FETCH_HEAD`, `ORIG_HEAD`, in-progress operation files, IDE files) stays on the machine that wrote it.'),
 ("README.md",'compares disk and registers for `HEAD`, `packed-refs`, `shallow`, `refs/**` and `objects/**`','compares disk and registers for `HEAD`, `packed-refs`, `refs/**` and `objects/**`'),
]
texts={}
for name,old,new in EDITS:
 text=texts.setdefault(name,(D/name).read_text(encoding="utf-8"))
 if text.count(old)!=1: sys.exit(f"{name}: expected exactly one match, found {text.count(old)}: {old[:70]}")
 texts[name]=text.replace(old,new)
for name,text in texts.items(): (D/name).write_text(text,encoding="utf-8")
print("edited",len(texts),"files")
