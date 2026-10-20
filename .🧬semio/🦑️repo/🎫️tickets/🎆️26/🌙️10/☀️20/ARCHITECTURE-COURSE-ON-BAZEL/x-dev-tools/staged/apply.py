#!/usr/bin/env python3
"""x-dev apply: stages the `stack` dashboard parameter on `@teaching/architecture-course:dev-site` and `@teaching/proctor:dev`, and installs the manifest test.
Idempotent and anchored: a target already carrying the parameter is skipped; an anchor that matches neither state aborts without writing.
usage: python3 apply.py [--check]"""
import os, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = pathlib.Path(os.environ["X_DEV_ROOT"]) if "X_DEV_ROOT" in os.environ else next(p for p in HERE.parents if (p / "AGENTS.md").exists())
CHECK = "--check" in sys.argv
COURSE = ROOT / "🎓️teaching/🏛️architecture/📚️course"
MANIFESTS = [
    (COURSE / "📦️packages/🟦️typescript/📋️project.json", '    "dev-site": {\n', "dev-site"),
    (ROOT / "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json", '    "dev": {\n', "dev"),
]
READY = '''      "metadata": {
        "semio": {
          "dashboard": {
            "ready": {
              "port": %d,
              "portEnv": "%s"
            }
          }
        }
      },
'''
ENVIRONMENT = {
    "TEACHING_ARCHITECTURE_COURSE_PORT": "6063",
    "PROCTOR_PORT": "8793",
    "PROCTOR_DATA": "{workspace}/.🧬semio/🎓️teaching/proctor-beside",
}
KEYS = {
    "dev-site": ["TEACHING_ARCHITECTURE_COURSE_PORT", "PROCTOR_PORT", "PROCTOR_DATA"],
    "dev": ["PROCTOR_PORT", "PROCTOR_DATA"],
}


def parameter(keys):
    lines = ",\n".join('                      "%s": "%s"' % (k, ENVIRONMENT[k]) for k in keys)
    return (HERE / "stack-parameter.json").read_text(encoding="utf8").replace("ENV_LINES", lines)


def edit(path, head, target):
    text = path.read_text(encoding="utf8")
    if text.count(head) != 1:
        sys.exit("anchor %r not unique in %s" % (head, path))
    start = text.index(head)
    block = text[start:text.index('      "executor"', start)]
    if '"id": "stack"' in block:
        print("already applied: %s %s" % (path.name, target))
        return
    port, env = (6061, "TEACHING_ARCHITECTURE_COURSE_PORT") if target == "dev-site" else (8791, "PROCTOR_PORT")
    ready = READY % (port, env)
    if block != head + ready:
        sys.exit("anchor not found (neither applied nor original): %s %s" % (path, target))
    tail = '            }\n          }\n        }\n      },\n'
    replaced = ready.replace(tail, '            },\n' + parameter(KEYS[target]) + '          }\n        }\n      },\n')
    assert replaced != ready
    print("%s: %s %s" % ("would edit" if CHECK else "edit", path.name, target))
    if not CHECK:
        path.write_text(text[:start] + head + replaced + text[start + len(head) + len(ready):], encoding="utf8")


def install():
    source = HERE / "🟦️manifests.test.ts"
    target = COURSE / "🧪️tests/🟢️continuous/🧾️manifests/🟦️.ts"
    if target.exists() and target.read_bytes() == source.read_bytes():
        print("already installed: manifest test")
        return
    print("%s: manifest test" % ("would install" if CHECK else "install"))
    if not CHECK:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())


for manifest in MANIFESTS:
    edit(*manifest)
install()
