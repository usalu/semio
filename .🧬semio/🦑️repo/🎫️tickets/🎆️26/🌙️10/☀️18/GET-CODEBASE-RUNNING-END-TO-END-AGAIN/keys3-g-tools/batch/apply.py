#!/usr/bin/env python3
"""keys3-g batch: check | apply | revert | stage <dir>. Anchored (every anchor must occur exactly once), idempotent (a marker says "already applied"), byte-backed (backup/ keeps the pre-apply bytes of each file once), re-reads each file right before editing.
`stage <dir>` writes the patched COPIES into <dir> (same relative paths) and never touches the live tree: the laws run against them before the batch lands (KEYS3_ROOT=<dir>).
The batch is independent of undeclared-g's batch (other anchors in the same course project.json; no whole-file hash refusal), so the two land in either order.
usage: python3 apply.py check|apply|revert|stage <dir>"""
import json, os, shutil, sys

W = os.environ.get("KEYS3_LIVE", "/Users/ueli/Documents/semio")
HERE = os.path.dirname(os.path.abspath(__file__))
COURSE_CSS = "🎓️teaching/🏛️architecture/📚️course/🎨️.css"
COURSE_PROJECT = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"
DASHBOARD_PROJECT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json"

UI, COURSE_TARGET, COURSE_SITE = "🧰️framework/🔨️modules/🖱️ui", "🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react", "🎓️teaching/🏛️architecture/📚️course"
EXCLUDED_DIRECTORIES = "🧪️tests,🧫️fixtures,🧫️examples,🧪️fixtures,🧪️examples,📖️stories,🎭️e2e"
EXCLUDED_FILES = "feature,stories.*,story.*,mdx"
GENERATED_DIRECTORIES = "node_modules,.git,.nx,.cache,target,dist,📤️dist,build,out,coverage,test-results,playwright-report,storybook-static,.venv,__pycache__,bin,obj,⚡️cache,🗑️generated,.lake,pkg"
ROOTS = f"{{{UI},{COURSE_TARGET},{COURSE_SITE}}}"

CSS_ANCHOR = '@source "../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react";\n'
CSS_NEW = CSS_ANCHOR + f'''/* 🧪️ Product CSS never depends on tests, fixtures, examples, e2e specs or stories: Tailwind scans no file of them, so a class used only there cannot change the served stylesheet.
   `tailwindScan` in `📦️packages/🟦️typescript/📋️project.json` declares exactly the remaining scan region as a cache input; the law `keys3-g-tools/batch/laws/tailwind-scan.test.ts` keeps both in step. */
@source not "../../../**/{{{EXCLUDED_DIRECTORIES}}}";
@source not "../../../**/*.{{{EXCLUDED_FILES}}}";
'''
SCAN = [
    f'"{{workspaceRoot}}/{UI}/**/*"',
    f'"{{workspaceRoot}}/{COURSE_TARGET}/**/*"',
    f'"{{workspaceRoot}}/{COURSE_SITE}/**/*"',
    f'"!{{workspaceRoot}}/{ROOTS}/**/{{{GENERATED_DIRECTORIES}}}/**/*"',
    f'"!{{workspaceRoot}}/{ROOTS}/**/{{{EXCLUDED_DIRECTORIES}}}/**/*"',
    f'"!{{workspaceRoot}}/{ROOTS}/**/*.{{{EXCLUDED_FILES}}}"',
]
PROJECT_EDITS = [
    ("scan-named-input", '    "workspaceManifests": [\n',
     '    "tailwindScan": [\n      ' + ",\n      ".join(SCAN) + '\n    ],\n    "workspaceManifests": [\n', '"tailwindScan": ['),
    ("build-inputs", '        "{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/**/*"\n      ],\n      "outputs": [\n        "{projectRoot}/.🧾️store/build.json"',
     '        "{workspaceRoot}/🧰️framework/🔨️modules/🖼️assets/**/*",\n        "tailwindScan"\n      ],\n      "outputs": [\n        "{projectRoot}/.🧾️store/build.json"', '"tailwindScan"\n      ],\n      "outputs": ['),
    ("proctor-cargo-identity", '    "proctorCargoManifests": [\n',
     '    "proctorCargoManifests": [\n      { "input": "cargoToolchain", "projects": ["@teaching/proctor"] },\n', '{ "input": "cargoToolchain", "projects": ["@teaching/proctor"] }'),
]
CSS_EDITS = [("course-css-source-not", CSS_ANCHOR, CSS_NEW, '@source not "../../../'),]
DASHBOARD_EDITS = [("ci-source-project-json", '        "{workspaceRoot}/.github/workflows/build-system.yml"\n      ],\n      "outputs": [],\n      "options": {\n        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",\n        "command": "repo ./📜️script.ts test ci-source"',
                    '        "{workspaceRoot}/.github/workflows/build-system.yml",\n        "{projectRoot}/📋️project.json"\n      ],\n      "outputs": [],\n      "options": {\n        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",\n        "command": "repo ./📜️script.ts test ci-source"',
                    '"{projectRoot}/📋️project.json"\n      ],\n      "outputs": [],\n      "options": {\n        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",\n        "command": "repo ./📜️script.ts test ci-source"')]
FILES = {COURSE_CSS: CSS_EDITS, COURSE_PROJECT: PROJECT_EDITS, DASHBOARD_PROJECT: DASHBOARD_EDITS}


def read(root, path):
    return open(os.path.join(root, path), encoding="utf8").read()


def state(text, edits):
    rows = {}
    for name, old, new, marker in edits:
        if marker in text: rows[name] = "applied"
        elif text.count(old) == 1: rows[name] = "base"
        else: rows[name] = f"ANCHOR-MISSING ({text.count(old)} occurrences)"
    return rows


def patched(text, edits):
    for name, old, new, marker in edits:
        if marker in text: continue
        assert text.count(old) == 1, f"{name}: anchor occurs {text.count(old)} times"
        text = text.replace(old, new)
    return text


def backup(path): return os.path.join(HERE, "backup", path.replace("/", "__"))


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"
    if cmd == "check":
        bad = False
        for path, edits in FILES.items():
            rows = state(read(W, path), edits); bad |= any(v.startswith("ANCHOR") for v in rows.values()); print(path, json.dumps(rows, ensure_ascii=False))
        sys.exit(1 if bad else 0)
    if cmd == "stage":
        out = sys.argv[2]
        for path, edits in FILES.items():
            target = os.path.join(out, path); os.makedirs(os.path.dirname(target), exist_ok=True)
            open(target, "w", encoding="utf8").write(patched(read(W, path), edits)); print("staged", path)
    elif cmd == "apply":
        plan = {}
        for path, edits in FILES.items():
            text = read(W, path); rows = state(text, edits)
            if any(v.startswith("ANCHOR") for v in rows.values()): sys.exit(f"refused, anchor changed in {path}: {rows}")
            plan[path] = text
        os.makedirs(os.path.join(HERE, "backup"), exist_ok=True)
        for path, text in plan.items():
            if not os.path.exists(backup(path)): shutil.copyfile(os.path.join(W, path), backup(path))
            fresh = read(W, path)
            open(os.path.join(W, path), "w", encoding="utf8").write(patched(fresh, FILES[path])); print("applied", path)
    elif cmd == "revert":
        for path in FILES:
            if os.path.exists(backup(path)):
                shutil.copyfile(backup(path), os.path.join(W, path)); print("reverted", path)
            else: print("nothing to revert", path)
    else:
        sys.exit("usage: check|apply|revert|stage <dir>")


main()
