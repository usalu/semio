# keys5-g batch schema-index-key: input-precision rule `dsl-schema-index` (+ profile `schema-documents`) for the three schema-index targets of @semio-tech/dsl-derive-rs.
# usage: python3 -I apply.py [--root <checkout or mirror>] check|apply|revert      (anchored, idempotent, atomic replace, byte backup in backup/ on apply)
import hashlib, json, os, shutil, sys
args = sys.argv[1:]
root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
mode = [a for a in args if a in ("check", "apply", "revert")][0]
HERE = os.path.dirname(os.path.abspath(__file__))
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json"
path = os.path.join(root, REL)
text = open(path, encoding="utf-8").read()
PROFILE = '''  "schema-documents": {
   "description": "The JSON documents the schema document index parses for their $id: every file below a schema directory of the repository, outside the dot, target, node_modules, dist, generated and package directories its walk skips (the fixture and test collections it skips stay in: they cannot be named without dropping a module called tests).",
   "reads": [
    {
     "globs": [
      "{workspaceRoot}/**/🧬️schema/**/*.json",
      "!{workspaceRoot}/**/{target,node_modules,dist,🗑️generated,📦️packages}/**",
      "!{workspaceRoot}/**/.*/**"
     ]
    }
   ]
  },
'''
RULE = ''',
  {
   "id": "dsl-schema-index",
   "evidence": "static",
   "description": "The schema document index (generate, check and preview) walks the schema directories of the repository and parses every JSON document there for its $id, so it reads those documents and the command closure its script lists, never the Rust crates, TypeScript trees or other-product files `default`, `^default` and the native source groups stand for; its own output (the schema document index) is written, not read.",
   "select": {
    "project": "@semio-tech/dsl-derive-rs",
    "targets": [
     "generate-schema-index",
     "check-schema-index",
     "preview-schema-index"
    ]
   },
   "replaces": [
    "default",
    "^default",
    "nativeSources",
    "^nativeSources",
    "nativeTestSources",
    "^nativeTestSources"
   ],
   "reads": [
    {
     "profile": "schema-documents"
    }
   ]
  }'''
profile_anchor = '  "test-discovery": {\n'
rule_anchor = '    }\n   ]\n  }\n ]\n}\n'
partial = '"id": "dsl-schema-index"' in text or '"schema-documents": {' in text
applied = PROFILE in text and RULE in text
if partial and not applied:
    print("unknown: a different variant of this batch is on disk"); sys.exit(2)
if mode == "check":
    print("applied" if applied else ("base" if text.count(profile_anchor) == 1 and text.endswith(rule_anchor) else "unknown")); sys.exit(0)
def write(new):
    json.loads(new)
    tmp = path + ".k5tmp"
    open(tmp, "w", encoding="utf-8").write(new); os.replace(tmp, path)
if mode == "apply":
    if applied: print("already applied"); sys.exit(0)
    assert text.count(profile_anchor) == 1 and text.endswith(rule_anchor), "anchors not found exactly once"
    os.makedirs(os.path.join(HERE, "backup"), exist_ok=True)
    shutil.copyfile(path, os.path.join(HERE, "backup", "precision.json"))
    new = text.replace(profile_anchor, PROFILE + profile_anchor)
    new = new[: -len('\n ]\n}\n')] + RULE + '\n ]\n}\n'
    write(new); print("apply ok")
elif mode == "revert":
    if not applied: print("not applied"); sys.exit(0)
    new = text.replace(PROFILE, "")
    assert new.endswith(RULE + '\n ]\n}\n'), "rule tail not found"
    new = new[: -len(RULE + '\n ]\n}\n')] + '\n ]\n}\n'
    write(new); print("revert ok")
