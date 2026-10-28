"""Batch `procstore`: retire the obsolete command store `🧰️framework/🤖️generated/process` and the `runtimeStateDirectories` mechanism that only existed to keep it out of every cache key.
Proof of obsolescence (aud-disk-g + disk-g): the command configuration (`🧰️framework/⚙️configuration/🏃️process/🔣️.json`) stores into `.cache/command` (working-tree change of 10-26); the only readers of the old path are the policy entry itself and one test fixture; the newest write (captures) is 10-26 08:59 by a process that loaded the old configuration; nothing else references `cargo-target` or `leases` there.
USER DECISION: delete the directory first (193 MiB: captures 365 entries, cargo-target, leases 20 entries). Removing the policy entry while the directory exists would put its 193 MiB into the inputs of every project of the framework owner, so `apply` refuses (exit 3) while it exists."""
import os

L = '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'
D = '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference'
OLD_STORE = "🧰️framework/🤖️generated/process"


def PRECONDITION(root):
    if os.path.exists(os.path.join(root, OLD_STORE)):
        return "the old command store still exists, delete it first (user decision): " + OLD_STORE
    return None


EDITS = [
    (L + "/⚡️caching/🔣️policy.json", [('  "runtimeStateDirectories": [\n    "🧰️framework/🤖️generated/process"\n  ],\n', "")]),
    (L + "/⚡️caching/🧬️schema/🔣️.json", [('    "runtimeStateDirectories": {\n      "description": "Workspace-relative directories that hold runtime state of earlier or older processes (captures, leases, build targets), never an input: every project negates them. They must be ignored by version control and never tracked (a law checks it with git); the command store itself lives in `.cache/command`.",\n      "type": "array",\n      "uniqueItems": true,\n      "items": { "type": "string", "pattern": "^[^/].*[^/]$" }\n    },\n', "")]),
    (L + "/🏗️build/📥️inputs/🟨️.mjs", [('  for (const path of POLICY.runtimeStateDirectories ?? []) inputs.push(`!{workspaceRoot}/${path}/**/*`);\n', "")]),
    (D + "/🟦️.ts", [
        (", runtimeStateDirectories: corpus.runtimeState}, TAXONOMY:", "}, TAXONOMY:"),
        ('    for (const path of corpus.runtimeState) expect((result.default ?? []).includes(`!{workspaceRoot}/${path}/**/*`), `${row.id} negates ${path}`).toBe(true);\n', ""),
    ]),
    (D + "/🔣️.json", [('  ],\n  "runtimeState": [\n    "fw/🤖️generated/process"\n  ]\n', '  ]\n')]),
    (D + "/🧬️schema/🔣️.json", [
        ('    "cases",\n    "runtimeState",\n', '    "cases",\n'),
        (',\n    "runtimeState": {\n      "type": "array",\n      "items": {\n        "type": "string"\n      }\n    }', ""),
    ]),
    ('🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🧪️tests/🏠️runtime-store/🟦️.ts', [
        ('const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));\n', ""),
        ('test("every runtime state directory of the policy is ignored, untracked and no generator writes into it", () => {\n  expect(policy.runtimeStateDirectories.length).toBeGreaterThan(0);\n  const outputs: string[] = Object.values<any>(taxonomy.generatorContracts).flatMap((contract) => (contract.outputRoots ?? []).map((row: {path: string}) => row.path));\n  for (const path of policy.runtimeStateDirectories as string[]) {\n    expect(git("check-ignore", "-q", "--", path).status, `${path} is ignored`).toBe(0);\n    expect(git("ls-files", "--", path).stdout.trim(), `${path} is not tracked`).toBe("");\n    expect(outputs.filter((output) => output === path || output.startsWith(path + "/") || path.startsWith(output + "/")), `${path} is no declared generator output`).toEqual([]);\n  }\n});\n', 'test("the retired command store is declared nowhere: the policy has no runtime state directories and no tracked file names the old store", () => {\n  expect(policy.runtimeStateDirectories).toBeUndefined();\n  const named = git("grep", "-l", "-F", "🤖️generated/process", "--", ".", ":(exclude).🧬semio").stdout.split("\\n").filter((file) => file !== "" && !file.endsWith("🏠️runtime-store/🟦️.ts"));\n  expect(named).toEqual([]);\n}, 60000);\n'),
    ]),
]

NEW = []
