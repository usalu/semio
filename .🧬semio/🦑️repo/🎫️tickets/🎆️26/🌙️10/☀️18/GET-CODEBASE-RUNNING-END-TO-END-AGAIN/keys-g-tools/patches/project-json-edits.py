"""python3 project-json-edits.py [--check] : the keys-g project declaration edits (anchored, idempotent, all-or-nothing).
course-react: the `reads` tree of the repository library (5,068 files) is replaced by the declared runtime entry of its vitest config (the import closure of the config is inferred: workspace test aliases, churn observation);
dashboard: `test-language` reads only what its oracle run proved (the launcher runtime pin, the language modules and the axes schema) instead of the whole library."""
import json, sys
ROOT = "/Users/ueli/Documents/semio/"
CR = ROOT + "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json"
DB = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json"
files = {"cr": open(CR, encoding="utf-8").read(), "db": open(DB, encoding="utf-8").read()}

def sub(key, anchor, new, marker):
    if marker in files[key]:
        return
    if files[key].count(anchor) != 1:
        sys.exit(f"{key}: anchor not unique ({files[key].count(anchor)}): {anchor[:80]}")
    files[key] = files[key].replace(anchor, new)

CONFIG = "{workspaceRoot}/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"
files["cr"] = files["cr"].replace('''    "reads": [
      "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**/*"
    ],
''', "")
sub("cr", '''      "inputs": [
        "default",
        "^default",
        "reads",
        { "dependentTasksOutputFiles": "**/*" }
      ],
      "outputs": [],
''', '''      "inputs": [
        "default",
        "^default",
        { "dependentTasksOutputFiles": "**/*" }
      ],
      "outputs": [],
      "metadata": {
        "semio": {
          "runtimeSourceEntries": [
            "''' + CONFIG + '''"
          ]
        }
      },
''', '{ "dependentTasksOutputFiles": "**/*" }\n      ],\n      "outputs": [],\n      "metadata"')
sub("cr", '''      "inputs": [
        "default",
        "^default",
        "testEnv",
        "reads"
      ],
      "outputs": [],
''', '''      "inputs": [
        "default",
        "^default",
        "testEnv"
      ],
      "outputs": [],
      "metadata": {
        "semio": {
          "runtimeSourceEntries": [
            "''' + CONFIG + '''"
          ]
        }
      },
''', '"testEnv"\n      ],\n      "outputs": [],\n      "metadata"')
db = json.loads(files["db"])
ni = db["namedInputs"]
if "tsTestReads" not in ni:
    lib = "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
    proc = "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/"
    ni["tsTestReads"] = [
        "{workspaceRoot}/🧰️framework/🔨️modules/📚️compiler/**/*",
        lib + "🎛️dashboard/**/*",
        lib + "🚀️launcher/**/*",
        proc + "🏗️build/🗣️language/**/*",
        proc + "📦️packages/🟦️typescript/🏗️build/🗣️language/**/*",
        "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🎚️axes/🔣️.json",
    ]
    inputs = db["targets"]["test-language"]["inputs"]
    db["targets"]["test-language"]["inputs"] = ["tsTestReads" if item == "reads" else item for item in inputs]
files["db"] = json.dumps(db, ensure_ascii=False, indent=2) + "\n"
if "--check" in sys.argv:
    print("anchors ok")
    sys.exit(0)
open(CR, "w", encoding="utf-8").write(files["cr"])
open(DB, "w", encoding="utf-8").write(files["db"])
print("applied project declarations")
