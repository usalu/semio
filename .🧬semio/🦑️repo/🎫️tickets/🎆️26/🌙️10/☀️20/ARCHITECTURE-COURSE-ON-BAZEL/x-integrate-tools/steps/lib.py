"""Shared constants and text helpers of the combined apply series (x-integrate)."""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.abspath(os.path.join(HERE, "..", ".."))
LIVE = os.path.abspath(os.path.join(HERE, *[".."] * 9))
SITE = "🎓️teaching/🏛️architecture/📚️course"
PKG = f"{SITE}/📦️packages/🟦️typescript"
COURSE = f"{PKG}/📋️project.json"
PROCTOR = "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"
PRODUCT = "🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts"
BOOTSTRAP = "🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts"
BUILD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BAZEL = f"{BUILD}/🔧️tools/🏗️bazel"
INFERENCE_TESTS = f"{BUILD}/🕸️graph/📥️inference/🧪️tests"
COURSE_INPUTS_FIXTURE = f"{INFERENCE_TESTS}/🎓️course-inputs/🔣️.json"
SITE_TSCONFIG = f"{PKG}/tsconfig.site.json"
SITE_TSCONFIG_TEXT = '{\n  "extends": "./tsconfig.json",\n  "include": ["../../🟦️.ts", "../../🏗️builder/🌐️vite/🟦️.ts"]\n}\n'


def read(root, rel):
    with open(os.path.join(root, rel), encoding="utf8") as handle:
        return handle.read()


def write(root, rel, text):
    path = os.path.join(root, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".tmp-x-integrate"
    with open(temporary, "w", encoding="utf8") as handle:
        handle.write(text)
    os.replace(temporary, path)


def refuse(message):
    raise SystemExit("refused: " + message)


def load_script(path, strip_main=True):
    """Executes a staged apply.py without running its main(), so a step can call its functions in process."""
    source = open(path, encoding="utf8").read()
    if strip_main:
        source = re.sub(r"\n(?:if __name__ == \"__main__\":\n\s+)?(?:sys\.exit\()?main\([^\n]*\)?\n*$", "\n", source)
    namespace = {"__name__": "x_integrate_loaded", "__file__": path}
    exec(compile(source, path, "exec"), namespace)
    return namespace
