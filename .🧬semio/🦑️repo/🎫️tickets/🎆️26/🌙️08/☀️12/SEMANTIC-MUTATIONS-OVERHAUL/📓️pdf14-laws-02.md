# PDF 1.4 Direct Law Assertion Repair

## Scope

Only the `language_neutral_forward_and_concrete_inverse` test sections in the nine direct PDF 1.4 mutation leaves changed:

- `✳️any`: `📥️insert-page`, `🔀️move-page`, `🗑️remove-page`, `📝️replace-page-text`, `📐️resize-page`
- `✳️a`: `🧹️clear-page-text`, `📝️set-page-text`
- `✳️x`: `📉️collapse-page-size`, `📐️set-page-size`

Fixtures, production mutation definitions, geometry, and codecs were not changed.

## Repair

Each law now deserializes fixture `expected` into `PdfSnapshot` and fixture `inverse` into the exact aggregate mutation vector before comparing. This keeps page geometry in the production `f64` model and indices in their typed integer model; it does not introduce the repository-wide JSON-number normalizer.

Each law also recursively checks the serialized mutation, expected snapshot, and inverse for exact object keys, array length/order, and nonnumeric scalar values. Numeric leaves are intentionally compared by the typed production values, allowing JSON integer spelling for a modelled `f64` value without silently accepting an extra or missing field.

The original codec loop and concrete inverse-application loop remain after these checks. The ticket harness verifies that ordering against all nine real sources.

## Evidence

The historical runtime red is retained in [the triage](pdf14-runtime-assertion-triage.md) and its linked STDIO transcript. It identified only the representation-sensitive first assertion; later assertions had not executed then.

The retained [Ajv/source checkpoint](🧪️pdf14-laws-02/🧪️fixture-validation-nx-green.log) ran through Nx:

```text
bun ./📜️script.ts nx exec '--projects=@semio-tech/stdio-plugin' --skipNxCache -- bun /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️pdf14-laws-02/📜️script.ts
```

It passed nine Ajv-validated fixture envelopes and recorded SHA-256 values for every fixture and current direct source. The script additionally asserts the typed snapshot/inverse comparisons, three recursive JSON-shape checks, absence of a redundant fixture-mutation decode, and downstream codec/inverse-loop ordering in all nine real sources. The first Nx invocation exposed the harness's former `cwd` dependency before reading any fixture; that non-behavioral transcript is retained at `🧪️pdf14-laws-02/🧪️fixture-validation-green.log`.

`git diff --check` returned zero for the selected path arguments; its retained output is [here](🧪️pdf14-laws-02/🧪️diff-check.log). Because the direct leaves are currently untracked in this concurrent workspace, the source checkpoint above is the applicable exact-source evidence.

No Cargo/Nx runtime test was run in this packet, per the serialized STDIO gate. The registered replay for the coordinator is:

```text
SEMIO_TEST_BUDGET_MS=180000 SEMIO_TEST_CASE_BUDGET_MS=30000 bun ./📜️script.ts nx run @semio-tech/stdio-plugin:test-quick --skipNxCache -- language_neutral_forward_and_concrete_inverse
```

That replay must establish whether the existing downstream codec and inverse assertions pass. Any resulting failure is a new runtime boundary and is outside this test-only assertion repair until separately authorized.

## Coordinator Runtime Acceptance

After repairing positional Nextest filter routing, the coordinator's fresh registered STDIO invocation compiled and ran exactly9 matching tests:9 passed,5997 skipped, exit0. Transcript: `🧪️pdf14-targeted-registered.log`; binaries metadata: `🧪️pdf14-targeted-registered-artifacts/semio-nextest-qjgb7p/binaries-metadata.json`. The build used `--lib --no-default-features --build-jobs 1 language_neutral_forward_and_concrete_inverse` and an isolated `🧪️pdf14-contract-target`.

An independent listing then verified the exact nine non-ignored PDF1.4 Any/A/X test identities in the retained executable and reran them without fail-fast. All9 passed again; its SHA-256 remained `b1e6539470418a62ce71cf4aea8931ae0f5e2cbebeb7c5fb626133ea73192a7e`. Evidence: `🧪️pdf14-runtime-selection/🧪️root-verified-selection-retry.log`, exact selected roster/results under `🧪️pdf14-runtime-selection/🧫️run-jePvq4`. The initial listing harness failed before test execution because it imported a private capture helper; that invocation error is separately retained in `🧪️root-verified-selection.log` and was corrected using a bounded subprocess.

This accepts the nine assertion repairs and their existing downstream concrete inverse/codec law loops on that compiled source boundary. It is not a full STDIO suite pass, a complete PDF owner conversion, or validation of the subsequent lower metadata/public derive changes. The known wider runtime failures and remaining canonical-filename/metadata/registry work remain open.
