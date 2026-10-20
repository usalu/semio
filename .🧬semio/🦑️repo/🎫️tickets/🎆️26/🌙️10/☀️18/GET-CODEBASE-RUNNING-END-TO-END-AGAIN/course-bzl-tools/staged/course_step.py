"""course-bzl: the course-specific residue after the x-* sets. Anchored, idempotent, all-or-nothing on one root.

G1 `test-e2e --proctor <relative>` resolves against the repository root (audit-proctor P2-7), not the package directory the Bazel and Nx actions run in.
   Source + test + language-agnostic fixture (Python posixpath/ntpath oracle, Ajv schema).
usage: course_step.py [--root DIR] [--check]   (exit 1 = pending, 0 = applied, 2 = anchor refusal, nothing written)"""
import os, sys, json

HERE = os.path.dirname(os.path.abspath(__file__))
FILES = os.path.join(HERE, "files")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 9))
SITE = "🎓️teaching/🏛️architecture/📚️course"
E2E = f"{SITE}/🎭️e2e/🟦️.ts"
TEST = f"{SITE}/🧪️tests/🧱️local-stack/🟦️.ts"
FIXTURE = f"{SITE}/🧫️fixtures/🎭️e2e-proctor/🔣️.json"
SCHEMA = f"{SITE}/🧫️fixtures/🎭️e2e-proctor/🧬️schema/🔣️.json"
NEW = {"e2e-proctor.fixture.json": FIXTURE, "e2e-proctor.schema.json": SCHEMA}

FUNCTION = '''/** 🛤️ Where a `--proctor` executable is: absolute values stay, relative ones resolve against the repository root, never against the package directory the build tool runs the script in. */
export function endToEndProctorPath(repoRoot: string, value: string, paths: Pick<typeof posix, "isAbsolute" | "resolve"> = { isAbsolute, resolve }): string {
  return paths.isAbsolute(value) ? value : paths.resolve(repoRoot, value);
}
'''
TESTCASE = '''  it("resolves --proctor against the repository root on every path flavour", () => {
    const validate = new Ajv({ strict: false }).compile(proctorPathSchema);
    expect(validate(proctorPathCases), JSON.stringify(validate.errors)).toBe(true);
    for (const [paths, rows] of [[posix, proctorPathCases.posix], [win32, proctorPathCases.win32]] as const) {
      for (const row of rows) expect(endToEndProctorPath(row.root, row.value, paths), row.id).toBe(row.expected);
    }
  });
'''

E2E_EDITS = [
    ("import", 'import { dirname, join, resolve } from "node:path";', 'import { dirname, isAbsolute, join, posix, resolve } from "node:path";'),
    ("function", "/** 🧭️ The order the topologies boot in", FUNCTION + "/** 🧭️ The order the topologies boot in"),
    ("say", '    say(plan.proctor === undefined ? "building the proctor (a first build takes minutes)" : `running the proctor ${plan.proctor} as it is`);\n    if (plan.proctor === undefined) artifact',
     '    const given = plan.proctor === undefined ? undefined : endToEndProctorPath(repoRoot, plan.proctor);\n    say(given === undefined ? "building the proctor (a first build takes minutes)" : `running the proctor ${given} as it is`);\n    if (given === undefined) artifact'),
    ("run", "proctor: plan.proctor ?? artifact!, signal }", "proctor: given ?? artifact!, signal }"),
]
TEST_EDITS = [
    ("path import", 'import { dirname, join, resolve, sep } from "node:path";', 'import { dirname, join, posix, resolve, sep, win32 } from "node:path";'),
    ("fixture import", 'import outputCases from "../../🧫️fixtures/🎭️e2e-output/🔣️.json" with { type: "json" };\n',
     'import outputCases from "../../🧫️fixtures/🎭️e2e-output/🔣️.json" with { type: "json" };\nimport proctorPathCases from "../../🧫️fixtures/🎭️e2e-proctor/🔣️.json" with { type: "json" };\nimport proctorPathSchema from "../../🧫️fixtures/🎭️e2e-proctor/🧬️schema/🔣️.json" with { type: "json" };\n'),
    ("plan import", "import { bootOrder, endToEndPlan, endToEndPorts,", "import { bootOrder, endToEndPlan, endToEndPorts, endToEndProctorPath,"),
    ("test", '  it("boots the rehearsal before the dev site, which holds the whole host once it is up", () => {', TESTCASE + '\n  it("boots the rehearsal before the dev site, which holds the whole host once it is up", () => {'),
]


def edit(text, rows, label):
    for name, old, new in rows:
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"refused: {label}: anchor {name!r} occurs {text.count(old)} times")
        text = text.replace(old, new, 1)
    return text


def main():
    args = sys.argv[1:]
    root = os.path.abspath(args[args.index("--root") + 1]) if "--root" in args else DEFAULT_ROOT
    plan = {}
    for rel, rows in ((E2E, E2E_EDITS), (TEST, TEST_EDITS)):
        old = open(os.path.join(root, rel), encoding="utf8").read()
        new = edit(old, rows, rel)
        if new != old:
            plan[rel] = new
    for name, rel in NEW.items():
        data = open(os.path.join(FILES, name), encoding="utf8").read()
        path = os.path.join(root, rel)
        if not os.path.exists(path) or open(path, encoding="utf8").read() != data:
            plan[rel] = data
    for rel in sorted(plan):
        print("pending" if "--check" in args else "write  ", rel)
    if "--check" in args or not plan:
        return 1 if plan else 0
    for rel, text in plan.items():
        path = os.path.join(root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path + ".tmp-course-bzl", "w", encoding="utf8") as handle:
            handle.write(text)
        os.replace(path + ".tmp-course-bzl", path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
