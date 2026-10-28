"""obsrows-g: derives the staged genuine-birth files (a birth that prepares the target execution inventory the execution law consumes) from the live ones by exact, unique replacements (re-runnable)."""
import json, os, sys
LIB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
CA = os.environ["CA"]
G = "🧪️testing/🌱️genuine-birth/"

def derive(rel, edits):
    text = open(LIB + rel, encoding="utf8").read()
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit("anchor count %d != 1 in %s: %s" % (text.count(old), rel, old[:90]))
        text = text.replace(old, new)
    out = CA + "/obsrows-g-tools/staged/" + rel
    os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, "w", encoding="utf8").write(text)
    print("ok", rel)

derive(G + "🔣️.json", [(
    '''    { "id": "model", "arguments": ["run", "@semio-tech/repo-lib:test-build-model"], "sdk": true, "environment": {}, "after": "Repo-to-Nx|Undeclared" }
''',
    '''    { "id": "model", "arguments": ["run", "@semio-tech/repo-lib:test-build-model"], "sdk": true, "environment": {}, "after": "Repo-to-Nx|Undeclared" },
    { "id": "target-execution", "arguments": ["tools", "setup", "all"], "sdk": false, "environment": {}, "preparation": "target-execution" }
''')])

derive(G + "🧬️schema/🔣️.json", [(
    '''          "environment": { "type": "object", "additionalProperties": { "type": "string" } },
          "after": { "type": "string", "minLength": 1 }''',
    '''          "environment": { "type": "object", "additionalProperties": { "type": "string" } },
          "after": { "type": "string", "minLength": 1 },
          "preparation": { "description": "Work done once beneath this birth before the file starts: `target-execution` prepares the inventory of all emitted targets and hands its path and sha256 to the file as SEMIO_TARGET_EXECUTION_INVENTORY_FILE and SEMIO_TARGET_EXECUTION_INVENTORY_SHA256, then verifies that every consumed fact and the producer still hold.", "enum": ["target-execution"] }''')])

derive(G + "🟦️.ts", [
    ('import { basename, isAbsolute, join, relative, resolve } from "node:path";',
     'import { basename, isAbsolute, join, relative, resolve } from "node:path";'),
    ('export type GenuineBirthV1 = Readonly<{ id: string; arguments: readonly string[]; sdk: boolean; environment: Readonly<Record<string, string>>; after?: string }>;',
     'export type GenuineBirthV1 = Readonly<{ id: string; arguments: readonly string[]; sdk: boolean; environment: Readonly<Record<string, string>>; after?: string; preparation?: "target-execution" }>;'),
    ('/** 🧪️ Describes one test file run inside a genuine Original birth. */',
     '''/** 🧾️ Prepares the inventory of all emitted targets once beneath the birth, hands its path and sha256 to `run` and verifies afterwards that every consumed source fact and the producer still hold; a failing `run` skips the verification. */
export async function withTargetExecutionInventoryV1(root: string, environment: NodeJS.ProcessEnv, original: ScriptInvocation, maximumElapsedMilliseconds: number, run: (inventory: Readonly<Record<string, string>>) => Promise<void>): Promise<void> {
  const { targetExecutionPreparationControlV1, prepareTargetExecutionInventoryV1, verifyTargetExecutionInventoryV1 } = await import("../../🏗️build/🏃️execution/🧪️tests/📥️preparation/🟦️.ts");
  const artifacts = environment.SEMIO_TEST_ARTIFACT_DIR;
  if (artifacts === undefined) throw Error("Test artifact directory required");
  const control = targetExecutionPreparationControlV1(original, { version: 1, maximumElapsedMilliseconds }, () => performance.now()), output = join(artifacts, `execution-inventory-${process.pid}.json`);
  const { digest } = await prepareTargetExecutionInventoryV1(root, output, control);
  await run({ SEMIO_TARGET_EXECUTION_INVENTORY_FILE: output, SEMIO_TARGET_EXECUTION_INVENTORY_SHA256: digest });
  await verifyTargetExecutionInventoryV1(output, digest, control);
}

/** 🧪️ Describes one test file run inside a genuine Original birth. */'''),
    ('  await withScriptProcessEnvelope(envelope, async original => {\n    if (!birth.sdk) return launch(original);',
     '  const start = (original: ScriptInvocation, extra: Readonly<Record<string, string | undefined>> = {}): Promise<void> => birth.preparation === undefined ? launch(original, extra) : withTargetExecutionInventoryV1(root, environment, original, declared.childTimeoutMilliseconds, inventory => launch(original, { ...extra, ...inventory }));\n  await withScriptProcessEnvelope(envelope, async original => {\n    if (!birth.sdk) return start(original);'),
    ('() => launch(original, { SEMIO_NX_TOOLING_ROOT: toolkitRoot, SEMIO_NX_TOOLING_IDENTITY: identity }));',
     '() => start(original, { SEMIO_NX_TOOLING_ROOT: toolkitRoot, SEMIO_NX_TOOLING_IDENTITY: identity }));'),
])

derive(G + "🧪️tests/🧫️probe/🟦️.ts", [
    ('import { expect, test } from "bun:test";',
     'import { expect, test } from "bun:test";\nimport { createHash } from "node:crypto";\nimport { readFileSync } from "node:fs";'),
    ('test.skipIf(!live)("probe a.b(c)+[d]", () => { expect(1).toBe(1); });',
     '''/** 🧾️ The prepared execution inventory reaches the file exactly when the birth declares its preparation, and its digest is the independent sha256 of the file. */
test.skipIf(!live)("probe inventory reaches the file exactly when the birth prepares it", () => {
  const birth = selectGenuineBirthV1(process.env.SEMIO_GENUINE_BIRTH), path = process.env.SEMIO_TARGET_EXECUTION_INVENTORY_FILE, digest = process.env.SEMIO_TARGET_EXECUTION_INVENTORY_SHA256;
  if (birth.preparation === undefined) { expect(path).toBeUndefined(); expect(digest).toBeUndefined(); return; }
  if (path === undefined || digest === undefined) throw Error("Prepared execution inventory required");
  const bytes = readFileSync(path);
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(digest);
  expect(JSON.parse(bytes.toString("utf8")).value.producer).toMatch(/^[a-f0-9]{64}$/u);
});

test.skipIf(!live)("probe a.b(c)+[d]", () => { expect(1).toBe(1); });'''),
])

derive(G + "🧪️tests/🧫️fixtures/🔣️.json", [(
    '''    { "birth": "setup", "pattern": "^probe a\\\\.b\\\\(c\\\\)\\\\+\\\\[d\\\\]$", "pass": 1 }
''',
    '''    { "birth": "setup", "pattern": "^probe a\\\\.b\\\\(c\\\\)\\\\+\\\\[d\\\\]$", "pass": 1 },
    { "birth": "setup", "pattern": "^probe inventory reaches the file exactly when the birth prepares it$", "pass": 1 },
    { "birth": "target-execution", "pattern": "^probe inventory reaches the file exactly when the birth prepares it$", "pass": 1 }
''')])

derive(G + "🧪️tests/🟦️.ts", [(
    '''  expect(validate({ ...policy, births: [] })).toBe(false);''',
    '''  expect(validate({ ...policy, births: [] })).toBe(false);
  expect(validate({ ...policy, births: [{ ...policy.births[0], preparation: "unknown" }] })).toBe(false);
  expect(policy.births.filter(row => "preparation" in row).map(row => row.id)).toEqual(["target-execution"]);''')])
