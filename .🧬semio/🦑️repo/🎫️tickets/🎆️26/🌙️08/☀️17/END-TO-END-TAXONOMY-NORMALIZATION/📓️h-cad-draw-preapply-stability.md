# CAD and Draw Pre-Apply Stability Audit

## Outcome

The frozen CAD/Draw mapping packet remains physically and semantically stable, but the live production apply is **not ready** at this snapshot.

- The language-neutral golden still derives exactly 209 CAD and 11 Draw mappings, and both mapping digests recompute exactly.
- All 220 source leaves exist as regular mode `100644` files. Their current path/mode/content manifest digest is `5f50eb62ba0e506c47d5b63f4b7568611d52edebc3e455b548dec9423ca83662` under `sha256-path-nul-mode-nul-content-sha256-lines-v1`.
- All 220 destination leaves remain absent and unoccupied under exact, NFC, Unicode case-fold, and VS16-fold comparisons.
- Destination path maxima remain 237 bytes for CAD and 210 bytes for Draw, both within the 240-byte budget.
- The frozen reference census remains 76 CAD occurrences plus the adjacent root join, and 23 Draw occurrences. The two Draw configurable-entry preimages still match their exact golden SHA-256 values.
- The pure authority, exact mapping/reference-plan, negative-authority, strict-schema, and registered-golden tests pass.
- Two fixture apply boundaries repeatedly fail because the submitted and rederived `sourceTreeDigest` differ. A third cancellation boundary timed out at five seconds and its interrupted Git ancestor probe returned no status.
- Narrow exact-source plans are projection-clean, but their external edit owner set differs from the broader plugin-scoped plan. The broader plan includes the complete authored/external owner surface but also admits unrelated pre-existing projection blockers. There is not yet one zero-error production plan containing the exact complete reference set.

No CAD or Draw move was applied. No production, taxonomy, normalizer, test, golden, Git, actual Compose, or temp/Compose path was modified. This report is the only write.

## Frozen authority

Golden:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json`

Golden SHA-256: `1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0`.

| Contract | Mappings | Mapping digest | Recomputed | Max destination bytes |
| --- | ---: | --- | --- | ---: |
| `artifact-example-model-catalog-v1` | 209 | `a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6` | exact | 237 |
| `artifact-editor-command-bundle-v1` | 11 | `1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b` | exact | 210 |

## Source preimages

The exact 220 source paths from the golden were read directly. The current boundary is:

```json
{
  "sources": 220,
  "regularFiles": 220,
  "mode100644": 220,
  "missing": 0,
  "nonFiles": 0,
  "manifestDigestAlgorithm": "sha256-path-nul-mode-nul-content-sha256-lines-v1",
  "manifestDigest": "5f50eb62ba0e506c47d5b63f4b7568611d52edebc3e455b548dec9423ca83662"
}
```

The golden does not store 220 per-file content hashes. Reconciliation against pinned baseline `9f449b10659b95148c8bcb3f91ce583bf7446973` found 216 byte-identical source blobs and four exact post-baseline deltas. The golden was not changed:

| Source | Baseline Git blob | Current Git blob | Current SHA-256 | Mode |
| --- | --- | --- | --- | --- |
| CAD concrete model manifest | absent | `eca02aaf430e06264c51cfcb5b842ceb3153c823` | `b90bae55fc9ccb0616c474cc07f2fcece194c8c4ba403758c996a0362cac5a03` | `100644` |
| Draw FSM `Cargo.toml` | `90ba73b08733ae0d3effb3630069a82386341e99` | `ec0cba8b72c2395839599c6a321cd914192897e2` | `35f3abecfcdfac2a01a433fdb61718ddc1802e5a5dcc05a413467d7afb18eaac` | `100644` |
| Draw FSM `📦️glue.rs` | `b8fc4af7e333a31e2e17a50f9e2739f58dab06c2` | `ac9221fb1903d3070766fdd6f50c96cb52163af7` | `d53720b8187b87ec55e2273b9b05569d982b1d3167daeea3c0caa6921c3af616` | `100644` |
| Draw command `🦀️component.rs` | `af40b0f76926f6f9f53fb8979a1d6432981ae076` | `0e732184548f7cbaffc06b2c7e471d46cd60baa2` | `41910ee6cbf411d7a2e5615fe01d414ba65aac97ad43576385501f5afd350b9e` | `100644` |

The CAD concrete manifest is the frozen packet's explicitly validated ninth model. The three Draw content deltas preserve the exact projection shape and mode; the pure Draw authority test derives the same 11-file union and exact configuration preimages from them.

## Destination occupancy and path budget

The audit walked only the two exact non-Compose artifact roots and observed 1,860 existing nodes. It compared every golden destination leaf against all observed paths using four separate keys:

- exact bytes;
- NFC;
- NFC plus `toLocaleLowerCase("und")`;
- NFC with every U+FE0F removed.

All four occupancy result sets are empty. No destination exists exactly or under a folded identity. CAD's maximum destination is 237 bytes and Draw's is 210 bytes; no destination exceeds 240 bytes.

## Reference surface and preimages

The targeted current lexical census remains exact:

- CAD trailing-root marker: 76 occurrences across the four frozen owner files: 49 interaction Rust, 13 runtime TypeScript, 13 interaction-spec Rust, and one spatial-kernel TypeScript.
- CAD marker without the trailing slash: 77 occurrences, adding the adjacent `Path::join` root.
- Draw old command-root marker: 23 occurrences across the seven frozen owner files: 8 dependency registry, 5 plus 5 moving Nx manifests, 2 root Cargo, 1 root script, 1 Draw package Cargo, and 1 Draw package Rust glue.

The two golden configurable-entry preimages remain exact:

| Owner | Golden/current SHA-256 |
| --- | --- |
| FSM Cargo `lib.path` | `35f3abecfcdfac2a01a433fdb61718ddc1802e5a5dcc05a413467d7afb18eaac` |
| FSM macros Cargo `lib.path` | `47213c84c9999d121abd74998de513b6d45c5838bcdcaf27e1dcf673acc01024` |

Current narrow live plans after the scope release are internally clean:

| Scope | Entries | Projection moves | Planned edits | Projection/reference/collision/old-token findings | All errors |
| --- | ---: | ---: | ---: | ---: | ---: |
| exact CAD source root | 312 | 209 | 1 registered spatial-kernel marker | 0 | 0 |
| exact Draw source root | 29 | 11 | 25 | 0 | 0 |

The exact CAD narrow plan expands the registered spatial-kernel consumer but intentionally does not inventory the three authored CAD owners outside the source root. The exact Draw narrow plan's 25 edits do not have the same owner set as a Draw-plugin-scoped plan.

A read-only `✏️s/🔌️plugins/🖍️draw` plan completed in 7.53 seconds with 705 entries, 11 Draw moves, and 25 selected Draw edits. Its owner set includes the two Draw package consumers plus the registered root consumers and moving command files. However, that scope also reports 65 inventory violations and 94 total errors, including unrelated mutation old-token findings and other incomplete Draw command projections. It therefore cannot be the atomic zero-error CAD/Draw apply plan.

This scope-dependent owner-set difference is a live pre-apply blocker even though the frozen fixture reference assertions pass.

## Focused test matrix

Command:

```text
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='artifact-example-model-catalog-projection is schema-owned|artifact-example-model-catalog-projection fails closed|artifact-editor-command-projection|projects every registered golden bundle|plans the exact CAD and Draw authority mappings|rejects unowned artifact prose|rolls back and atomically applies CAD and Draw projections|normalization rejects malformed projection consumers|cancellation rolls back and a successful retry converges|plans, applies, verifies, and converges an exact Nx-owned preview'
```

Released-state result:

```text
7 pass
248 filtered out
3 fail
279 expect() calls
51.44s
```

Passing boundaries include the three pure CAD/Draw authorities, registered golden projection, exact 209/11 plan plus structured cross-profile references, negative counterfeit/unowned cases, and strict normalization schema rejection.

Exact remaining failures:

1. `rolls back and atomically applies CAD and Draw projections to an empty second plan` fails at `🧹️normalization/🟦️.ts:7767`: `Plan source-tree digest cannot be rederived exactly from current schema-owned authority`.
2. `plans, applies, verifies, and converges an exact Nx-owned preview` fails at the same revalidation boundary and message.
3. `cancellation rolls back and a successful retry converges to an empty second plan` exceeds its five-second test timeout. Its interrupted `git rev-parse` ancestor probe returns `status === null`, producing `Git index ancestor probe failed for 🧪️tests: exit unknown`; the test reports killing one dangling process.

An earlier post-scope/pre-ticket run was 8 pass and 2 fail with the same two source-tree digest mismatches. The released run adds the host-sensitive five-second cancellation timeout. A final process census found no remaining matching fixture, inventory, or Nx process.

## Readiness decision

The frozen mapping and physical preimage packet is ready. The production transaction is not ready because:

1. apply revalidation cannot rederive the planned source-tree digest in two independent fixture workflows;
2. the narrow zero-error scope and broader complete-reference scope do not currently yield the same reference owner set;
3. the required full 10-test pre-apply matrix is not green.

Do not apply the 220 production moves until one plan contains the exact 209 CAD plus 11 Draw moves, the complete 75 CAD plus 25 Draw structured edit records, zero projection/reference/collision/stale-token errors, and the full focused matrix passes.

## Commands used

All commands were read-only except creation of this ticket report:

```text
shasum -a 256 <golden>
bun -e '<exact golden source/mode/hash and destination-fold occupancy audit>' <golden>
rg --count-matches -F '🖼️assets/🏗️modelDefinitions/' <four exact CAD owners>
rg --count-matches -F '🖼️assets/🏗️modelDefinitions' <four exact CAD owners>
rg --count-matches -F '<old Draw command-root marker>' <seven exact Draw owners>
bun -e '<inventoryTaxonomy + planTaxonomy summary>'
bun test <index.test.ts> --test-name-pattern='<frozen ten-case matrix>'
ps -ax -o pid=,ppid=,command= | rg '<exact audit process patterns>'
```
