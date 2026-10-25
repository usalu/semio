#!/usr/bin/env python3
"""Restores the cargo `test` target and TestScript of one repo-*-rs crate exactly as its siblings declare them.

usage: restore_target.py <module-dir-name> <project-name-suffix>   e.g. 🎯️goals goals
script.ts comes back from HEAD (the removal of dtest2-d was its only difference); project.json gets the
`namedInputs.testEnv` block and the `test` target re-inserted by text, other edits are kept.
"""
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
module = sys.argv[1]
base = f"{MODULES}/{module}/📦️packages/🦀️rust"
script = ROOT / base / "📜️script.ts"
project = ROOT / base / "📋️project.json"

head = subprocess.run(["git", "show", f"HEAD:{base}/📜️script.ts"], cwd=ROOT, capture_output=True, text=True, check=True).stdout
current = script.read_text(encoding="utf8")
if "TestScript" not in current:
    script.write_text(head, encoding="utf8")

text = project.read_text(encoding="utf8")
if '"test"' not in text:
    named = '''  "namedInputs": {
    "testEnv": [
      { "env": "SEMIO_TEST_LEVEL" },
      { "env": "SEMIO_TEST_BUDGET_MS" },
      { "env": "SEMIO_BUILD_BUDGET_MS" }
    ]
  },
'''
    anchor = '  "targets": {\n'
    assert text.count(anchor) == 1
    text = text.replace(anchor, named + anchor)
    tail = '      "cache": true\n    }\n  }\n}'
    assert text.rstrip().endswith(tail), "unexpected project.json tail"
    test_target = f'''      "cache": true
    }},
    "test": {{
      "executor": "repo:script",
      "cache": true,
      "inputs": [
        "default",
        "^default",
        "testEnv",
        "cargoToolchain"
      ],
      "outputs": [],
      "options": {{
        "cwd": "{base}",
        "command": "repo ./📜️script.ts test"
      }}
    }}
  }}
}}'''
    text = text.rstrip()[: -len(tail)] + test_target + "\n"
    project.write_text(text, encoding="utf8")
print("restored", module)
