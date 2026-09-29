"""🧪 TS1 one-off: aligns suites with the typed `bun:test` matchers of `@types/bun` (the hand-written env declarations typed every
matcher argument `unknown`). Readonly/literal mismatches compare against their declared shape (`toEqual<readonly string[]>`),
untyped fixture values against their own type (`toEqual<typeof expected>`), `any` handles get the type they produce
(`expect<T>`), possibly-missing expectations are guarded. Idempotent. usage: python3 ts1-bun-matchers.py [--dry-run]"""
import pathlib, sys

R = pathlib.Path("/Users/ueli/Documents/semio")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests"
T = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests"
DRY = "--dry-run" in sys.argv
changed = []

EDITS = {
    f"{L}/🌐️registry-import-language/🟦️.ts": [
        ("else expect(dependencies(row.source, path, row.role), row.id).toEqual(row.expected);", "else expect(dependencies(row.source, path, row.role), row.id).toEqual<typeof row.expected>(row.expected);", 1),
        ('target: "bun", write: false,', 'target: "bun",', 1),
        ('args.kind === "entry-point" ?', 'args.kind === "entry-point-build" ?', 1),
    ],
    f"{L}/🌳️kind-only-basename/🟦️.ts": [
        ("expect(taxonomy.fileKinds[kind.id]).toEqual({ emoji: kind.emoji, extensionChains: kind.extensionChains, role: kind.role });", "expect(taxonomy.fileKinds[kind.id]).toEqual<Pick<typeof kind, \"emoji\" | \"extensionChains\" | \"role\">>({ emoji: kind.emoji, extensionChains: kind.extensionChains, role: kind.role });", 1),
        ("expect(Object.keys(taxonomy.ecosystems)).toEqual(fixture.topology.packageLanguageDirectories);", "expect(Object.keys(taxonomy.ecosystems)).toEqual<readonly string[]>(fixture.topology.packageLanguageDirectories);", 1),
    ],
    f"{L}/🍃️artifact-support-leaf-authority/🟦️.ts": [
        ("expect(taxonomy.mutationPayloadSchemaAuthority).toEqual(vector.payloadAuthority);", "expect(taxonomy.mutationPayloadSchemaAuthority).toEqual<typeof vector.payloadAuthority>(vector.payloadAuthority);", 1),
        ("expect(actual.map((entry) => entry.value), row.id).toEqual(row.values);", "expect(actual.map((entry) => entry.value), row.id).toEqual<readonly string[]>(row.values);", 1),
    ],
    f"{L}/🎟️reference-coverage-selection/🟦️.ts": [
        ("expect(output.slice(0, supported.length)).toEqual(supported);", "expect(output.slice(0, supported.length)).toEqual<typeof supported>(supported);", 1),
    ],
    f"{L}/🏺️historical-package-owner-identity/🟦️.ts": [
        ("expect(actual, row.id).toEqual(coordinates);", "expect(actual, row.id).toEqual<typeof coordinates>(coordinates);", 1),
        ("expect(prior).toEqual(coordinates.filter(", "expect(prior).toEqual<typeof coordinates>(coordinates.filter(", 1),
        ("{ exact: contract })).toEqual([selected]);", "{ exact: contract })).toEqual<readonly (typeof selected)[]>([selected]);", 1),
    ],
    f"{L}/👀️readme-reviewed-fixture-inputs/🟦️.ts": [
        ("expect(value.bytes, path).toEqual(before.bytes);", "expect(value.bytes, path).toEqual<typeof before.bytes>(before.bytes);", 1),
    ],
    f"{L}/🔏️path-emoji-statutes/🟦️.ts": [
        ("expect(identity, row.path).toBe(oracle?.[0]);", "expect(identity, row.path).toBe<string | undefined>(oracle?.[0]);", 1),
    ],
    f"{L}/🔗️markdown-inline-references/🟦️.ts": [
        ("expect(actual.joined(), row.id).toEqual(row.expected);", "expect(actual.joined(), row.id).toEqual<typeof row.expected>(row.expected);", 1),
    ],
    f"{L}/🔬️workspace-contract/🟦️.ts": [
        ("expect(parsed.contributes).toEqual(reference.metadata.semio.contributes);", "expect(parsed.contributes).toEqual<typeof reference.metadata.semio.contributes>(reference.metadata.semio.contributes);", 1),
        ("expect(discovery.parseFixedDirectoryContractSetScope(candidate, parents)).toEqual(candidate);", "expect(discovery.parseFixedDirectoryContractSetScope(candidate, parents)).toEqual<typeof candidate>(candidate);", 1),
        ("expect(oracle, scenario.id).toEqual(expected);", "expect(oracle, scenario.id).toEqual<typeof expected>(expected);", 1),
        ("expect(toml.parse(vector.source), vector.id).toEqual(vector.tomlOracle);", "expect(toml.parse(vector.source), vector.id).toEqual<typeof vector.tomlOracle>(vector.tomlOracle);", 2),
        ("expect(projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.id).toEqual(vector.expected);", "expect(projectCargoProviderManifest({ locator: vector.locator, source: vector.source }), vector.id).toEqual<typeof vector.expected>(vector.expected);", 1),
        ("expect([0, 1]).toContain(listed.status);", "expect<readonly (number | null)[]>([0, 1]).toContain(listed.status);", 1),
        ("    expect(DRAW_SOURCE_SCENARIO.producerContext.compilerRoots).toEqual([nxScript]);", "    if (nxScript === undefined) throw new Error(\"package.json names no `bun ./<script> nx` runner\");\n    expect(DRAW_SOURCE_SCENARIO.producerContext.compilerRoots).toEqual([nxScript]);", 1),
        (".toEqual(DRAW_SOURCE_SCENARIO.oracle.activatedGeneratorIds);", ".toEqual<readonly string[]>(DRAW_SOURCE_SCENARIO.oracle.activatedGeneratorIds);", 2),
        ("expect(inspect(vector.source).map(({ kind }) => kind)).toEqual(vector.expected);", "expect(inspect(vector.source).map(({ kind }) => kind)).toEqual<readonly string[]>(vector.expected);", 1),
        ("expect(Bun.TOML.parse(workspaceText).workspace).toEqual(workspace);", "const parsedWorkspace = Bun.TOML.parse(workspaceText);\n    expect(\"workspace\" in parsedWorkspace ? parsedWorkspace.workspace : undefined).toEqual(workspace);", 1),
        ("expect(Bun.TOML.parse(workspaceBefore.toString(\"utf8\")).workspace).toEqual(workspace);", "const parsedBefore = Bun.TOML.parse(workspaceBefore.toString(\"utf8\"));\n      expect(\"workspace\" in parsedBefore ? parsedBefore.workspace : undefined).toEqual(workspace);", 1),
    ],
    f"{L}/🚚️readme-move-source-authority/🟦️.ts": [
        ("expect(api.canonical(candidate), row.id).toBe(oracleJson(candidate));", "expect<string>(api.canonical(candidate), row.id).toBe(oracleJson(candidate));", 1),
        ("expect(journal.state, row.id).toBe(row.expected === \"rolled-back\" ? \"rolling-back\" : \"editing\");", "expect<string>(journal.state, row.id).toBe(row.expected === \"rolled-back\" ? \"rolling-back\" : \"editing\");", 1),
    ],
    f"{L}/🛤️typescript-path-collection/🟦️.ts": [
        ("expect(implementation(compiler).parse(\"reader.ts\", row.source).filter((token: Token) => token.value === \"🟦️targetsold.ts\"), row.id).toEqual([]);", "expect<readonly Token[]>(implementation(compiler).parse(\"reader.ts\", row.source).filter((token: Token) => token.value === \"🟦️targetsold.ts\"), row.id).toEqual([]);", 1),
    ],
    f"{L}/🟢️readme-current-source-activation/🟦️.ts": [
        ("expect(selected.preimage, String(index)).toEqual(entry.preimage);", "expect(selected.preimage, String(index)).toEqual<typeof entry.preimage>(entry.preimage);", 1),
    ],
    f"{L}/🥒️gherkin-description-inline-code/🟦️.ts": [
        ("expect(actualValues(spans(row.source), row.source), row.id).toEqual(row.expected);", "expect(actualValues(spans(row.source), row.source), row.id).toEqual<typeof row.expected>(row.expected);", 1),
    ],
    f"{L}/🧑‍💻os-dev-composition-ownership/🟦️.ts": [
        ("expected.id).toEqual(actual.inputPatterns);", "expected.id).toEqual<readonly string[]>(actual.inputPatterns);", 1),
    ],
    f"{L}/🧪️storybook-discovery/🟦️.ts": [
        ("expect(stories, expected.path).toEqual(expected.stories);", "expect(stories, expected.path).toEqual<typeof expected.stories>(expected.stories);", 1),
    ],
    f"{L}/🧱️cargo-transaction-command-source/🟦️.ts": [
        ("expect(rows.every(Boolean), owner.path).toBe(true);", "expect<boolean>(rows.every(Boolean), owner.path).toBe(true);", 1),
        ("if (row.accepted) expect(invoke(), row.input).toEqual({ moduleRoot: \"/repo\", packages: row.expected });", "if (row.accepted) expect<{ readonly moduleRoot: string; readonly packages: unknown }>(invoke(), row.input).toEqual({ moduleRoot: \"/repo\", packages: row.expected });", 1),
    ],
    f"{L}/🧱️root-artifact-schema-law-source/🟦️.ts": [
        ("expect(policySourceText(\"/repo\", row.name, operations).state, row.name).toBe(row.expected);", "expect(policySourceText(\"/repo\", row.name, operations).state, row.name).toBe<string>(row.expected);", 1),
    ],
    f"{L}/🧱️root-schema-field-source/🟦️.ts": [
        ("expect(namedDeclarations(resolve(repoRoot, owner.path))).toEqual(owner.declarations);", "expect(namedDeclarations(resolve(repoRoot, owner.path))).toEqual<readonly string[]>(owner.declarations);", 1),
    ],
    f"{L}/🧱️wasm-package-wrappers/🟦️.ts": [
        ("expect(payload.type, row.id).toBe(\"module\");", "expect<string>(payload.type, row.id).toBe(\"module\");", 1),
        ("expect(payload.files, row.id).toEqual([row.wasm, row.module, row.types, row.wasmTypes]);", "expect<readonly string[]>(payload.files, row.id).toEqual([row.wasm, row.module, row.types, row.wasmTypes]);", 1),
        ("expect(project.targets[row.producerTarget].outputs, row.id).toEqual([row.producerOutput]);", "expect<readonly string[]>(project.targets[row.producerTarget].outputs, row.id).toEqual([row.producerOutput]);", 1),
    ],
    f"{L}/🧱️workspace-publication-source/🟦️.ts": [
        ("expect({ ...after, workspaces: before.workspaces }).toEqual(before);", "expect({ ...after, workspaces: before.workspaces }).toEqual<typeof before>(before);", 1),
        ("expect(targets, row.name).toEqual(resolution.targets);", "expect(targets, row.name).toEqual<readonly string[]>(resolution.targets);", 1),
        ("expect(oracle, row.name).toEqual(row.expected);", "expect(oracle, row.name).toEqual<readonly string[]>(row.expected);", 1),
    ],
    f"{L}/🧲️rust-physical-reference-context/🟦️.ts": [
        ("for (const candidate of candidates) expect(row.source.slice(candidate.start, candidate.end), row.id).toBe(candidate.value);", "for (const candidate of candidates) expect<string>(row.source.slice(candidate.start, candidate.end), row.id).toBe(candidate.value);", 1),
    ],
    f"{L}/🧾️source-file-facts/🟦️.ts": [
        ("expect(sourceFileFactReference([nfd], taxonomy)).toEqual([nfd.expected]);", "expect(sourceFileFactReference([nfd], taxonomy)).toEqual<readonly (typeof nfd.expected)[]>([nfd.expected]);", 1),
    ],
    f"{L}/🪶️artifact-empty-facet-authoring/🟦️.ts": [
        ("expect(after[`🧪️workspace/${path}`]).toEqual({ kind: \"file\", ...identity });", "expect(after[`🧪️workspace/${path}`]).toEqual<{ readonly kind: string } & typeof identity>({ kind: \"file\", ...identity });", 2),
    ],
    f"{L}/🫙️artifact-empty-facet-authority/🟦️.ts": [
        ("taxonomy), row.id).toEqual(expected);", "taxonomy), row.id).toEqual<typeof expected>(expected);", 2),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts": [
        ("row.id).toEqual(row.expectedViolations);", "row.id).toEqual<readonly string[]>(row.expectedViolations);", 1),
        (".toBe(row.expectedDispositionRole);", ".toBe<typeof row.expectedDispositionRole>(row.expectedDispositionRole);", 4),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🟦️.ts": [
        ("expect(actual).toEqual(row.expected);", "expect(actual).toEqual<typeof row.expected>(row.expected);", 1),
    ],
    f"{T}/📐️test-layout/🟦️.ts": [
        ("      expect(lines[0]).toBe(expected);", "      expect(lines[0]).toBe<typeof expected>(expected);", 1),
    ],
    f"{T}/🧬️mutation-fixtures/🟦️.ts": [
        (".toEqual(htmlPairs.nodes);", ".toEqual<typeof htmlPairs.nodes>(htmlPairs.nodes);", 1),
    ],
    f"{T}/🧬️schema-invariants/🟦️.ts": [
        ("expect([...SCHEMA_DIAGNOSTIC_CODES].sort()).toEqual([...protocolCodes].sort());", "expect([...SCHEMA_DIAGNOSTIC_CODES].sort()).toEqual<readonly string[]>([...protocolCodes].sort());", 1),
        ("expect([...SCHEMA_DIAGNOSTIC_EMITTERS].sort()).toEqual([...protocolEmitters].sort());", "expect([...SCHEMA_DIAGNOSTIC_EMITTERS].sort()).toEqual<readonly string[]>([...protocolEmitters].sort());", 1),
    ],
    "🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts": [
        ("expect(pid).toBe(child.pid);", "expect(pid).toBe<number | undefined>(child.pid);", 1),
    ],
}

for path, pairs in EDITS.items():
    p = R / path
    before = p.read_text()
    text = before
    for old, new, count in pairs:
        if old not in text:
            assert new in text, (path, old[:90])
            continue
        assert text.count(old) == count, (path, old[:90], text.count(old))
        text = text.replace(old, new)
    if text != before:
        changed.append(path)
        if not DRY:
            p.write_text(text)
print(("would change " if DRY else "changed ") + str(len(changed)) + " files")
