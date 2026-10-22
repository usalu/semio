# 🪝 The anchored edits of the langs-conc set, grouped by concern; each entry is (path, fn(text) -> (text, changed)).
from layout import *
from patchlib import chain
import patches_deps
import patches_python
import patches_cmake
import patches_bazel
import patches_keys
import patches_root
import patches_hermetic
def exports_process(text):
    return chain(text, [("process-exports", '    "./compiler-state": "./🏗️build/🗃️state/🟦️.ts",\n', '    "./compiler-state": "./🏗️build/🗃️state/🟦️.ts",\n    "./toolchains": "./🏗️build/🧰️toolchains/🟦️.ts",\n')])
def edits():
    return [
        (PKG + "/package.json", exports_process),
    ] + patches_deps.edits() + patches_python.edits() + patches_cmake.edits() + patches_bazel.edits() + patches_keys.edits() + patches_root.edits() + patches_hermetic.edits()
def writes(): return []
