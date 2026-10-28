"""Generates batches/procstore.py from the live files (repr of the exact anchor blocks)."""
R = "/Users/ueli/Documents/semio/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
D = L + "/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference"
T = "🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🏠️runtime-store/🟦️.ts"


def rd(path):
    return open(R + path, encoding="utf8").read()


tt = rd(T)
test2 = tt[tt.index('test("every runtime state directory'):]
tax = [line for line in tt.split("\n") if line.startswith("const taxonomy")][0] + "\n"
sch = rd(L + "/⚡️caching/🧬️schema/🔣️.json")
block = sch[sch.index('    "runtimeStateDirectories": {'):sch.index('    "ownerViews": {')]
pol = rd(L + "/⚡️caching/🔣️policy.json")
pblock = pol[pol.index('  "runtimeStateDirectories": ['):pol.index('  "toolchains": {')]
mjs = '  for (const path of POLICY.runtimeStateDirectories ?? []) inputs.push(`!{workspaceRoot}/${path}/**/*`);\n'
assert mjs in rd(L + "/🏗️build/📥️inputs/🟨️.mjs")
ts = rd(D + "/🟦️.ts")
line33 = [line for line in ts.split("\n") if "of corpus.runtimeState)" in line][0] + "\n"
new_test = ('test("the retired command store is declared nowhere: the policy has no runtime state directories and no tracked file names the old store", () => {\n'
            '  expect(policy.runtimeStateDirectories).toBeUndefined();\n'
            '  const named = git("grep", "-l", "-F", "🤖️generated/process", "--", ".", ":(exclude).🧬semio").stdout.split("\\n").filter((file) => file !== "" && !file.endsWith("🏠️runtime-store/🟦️.ts"));\n'
            '  expect(named).toEqual([]);\n'
            '}, 60000);\n')
doc = '''"""Batch `procstore`: retire the obsolete command store `🧰️framework/🤖️generated/process` and the `runtimeStateDirectories` mechanism that only existed to keep it out of every cache key.
Proof of obsolescence (aud-disk-g + disk-g): the command configuration (`🧰️framework/⚙️configuration/🏃️process/🔣️.json`) stores into `.cache/command` (working-tree change of 10-26); the only readers of the old path are the policy entry itself and one test fixture; the newest write (captures) is 10-26 08:59 by a process that loaded the old configuration; nothing else references `cargo-target` or `leases` there.
USER DECISION: delete the directory first (193 MiB: captures 365 entries, cargo-target, leases 20 entries). Removing the policy entry while the directory exists would put its 193 MiB into the inputs of every project of the framework owner, so `apply` refuses (exit 3) while it exists."""
import os

L = %r
D = %r
OLD_STORE = "🧰️framework/🤖️generated/process"


def PRECONDITION(root):
    if os.path.exists(os.path.join(root, OLD_STORE)):
        return "the old command store still exists, delete it first (user decision): " + OLD_STORE
    return None


EDITS = [
    (L + "/⚡️caching/🔣️policy.json", [(%r, "")]),
    (L + "/⚡️caching/🧬️schema/🔣️.json", [(%r, "")]),
    (L + "/🏗️build/📥️inputs/🟨️.mjs", [(%r, "")]),
    (D + "/🟦️.ts", [
        (", runtimeStateDirectories: corpus.runtimeState}, TAXONOMY:", "}, TAXONOMY:"),
        (%r, ""),
    ]),
    (D + "/🔣️.json", [('  ],\\n  "runtimeState": [\\n    "fw/🤖️generated/process"\\n  ]\\n', '  ]\\n')]),
    (D + "/🧬️schema/🔣️.json", [
        ('    "cases",\\n    "runtimeState",\\n', '    "cases",\\n'),
        (',\\n    "runtimeState": {\\n      "type": "array",\\n      "items": {\\n        "type": "string"\\n      }\\n    }', ""),
    ]),
    (%r, [
        (%r, ""),
        (%r, %r),
    ]),
]

NEW = []
''' % (L, D, pblock, block, mjs, line33, T, tax, test2, new_test)
open("batches/procstore.py", "w", encoding="utf8").write(doc)
