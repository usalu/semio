# 🩹️ B2 — Post-split fixture URI repair (fem2d/fem3d) + confirmation sweep

Shard B2. Repaired the `asset://` fixture URIs left dangling in `s.fem.2d`/`s.fem.3d`'s
`mutate-fem2d-1`/`mutate-fem3d-1` cases after an earlier shard split their 25 mutations each out of
`✳️any` into five real subsets, then swept the other eleven artifacts split in this ticket wave for
the same class of damage, and fixed the one `fixture-digest-mismatch` straggler in stdio/obj.

## Measured mutation→subset map (authoritative, read off disk)

`find …/🪆️subsets/✳️<subset>/🧬️schema/🧬️mutations -maxdepth 1 -type d`, both artifacts:

| subset     | 2d dirs | 3d dirs | count |
|------------|---------|---------|-------|
| `mesh`     | create/delete/replace-{node,element,section}, create/delete/replace-{region (2d) / solid (3d)} | same shape, `region`→`solid` | 11 |
| `material` | create/delete/replace-material | same | 3 |
| `boundary` | create/delete/replace-support | same | 3 |
| `load`     | change-load-case-self-weight, add-load, remove-load, create/delete-load-case, create/delete-combination | same | 7 |
| `analysis` | update-analysis-settings | same | 1 |

25 mutations each, matching the brief's claimed split (11/3/3/7/1) exactly — measured, not assumed.
`✳️any` still exists in both but its `🧬️schema/🧬️mutations/` now holds only the two non-mutation
codec dirs `💾️binary`/`📝️text` (grammar/binary schema, not moved).

## What was repaired

Both `mutate-fem2d-1/🥒️.feature` and `mutate-fem3d-1/🥒️.feature` had one broken pattern: the
`@id-spec-vector` Scenario Outline's three `Given`/`And` steps hardcoded
`asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/...` while the `<dir>` column
already carried the mutation-directory name — but the OWNING SUBSET (`any`) was baked into the step
text itself, not templated, so all 25×3=75 rows per artifact pointed at a directory that no longer
exists (150 `missing-fixture` breaches total, matching the brief).

Fix: added a `subset` column to each Examples table (values `mesh`/`material`/`boundary`/`load`/
`analysis`, filled from the measured map above) and changed `✳️any` → `✳️<subset>` in the three
`Given`/`And` lines. Script:
`$TICKET/🩹️b2-repoint-fem-mutation-subset-uris.py` (idempotent, `--check` for a dry run).

Checked both sibling `🦀️.rs` adapters for the same hardcoded pattern: their `include_str!` paths were
already correctly repointed to per-subset paths by the earlier shard (only a comment referencing the
still-valid `✳️any/🧬️schema/🧬️mutations/🦀️.rs` aggregator file remained, which is not stale — that
file still lives at that path). The `🐍️.py` adapters don't reference mutation directories at all. No
adapter changes were needed beyond the feature files.

## Confirmation sweep — the other twelve artifacts

`note`, `draw`, `mathematical`, `sequence`, `gltf`, `gif`, `obj`, `dxf`, `las`, `bcf`, `avi`,
`program`. For each: (a) scanned every subset's `🧬️schema/🧬️mutations/` for a mutation-directory
name appearing under more than one subset (a leftover duplicate would let a wrong URI "resolve" by
accident) — **zero duplicates found anywhere**; (b) resolved every `asset://`/`shared://`/`local://`
URI in each artifact's test feature(s) against the on-disk layout, substituting `<column>`
placeholders from their Examples tables (relative-path escapes included, e.g. `note`'s
`../../../✳️document/...` vector cells).

Findings:
- `program` was never split (only `✳️any` exists on disk, 266 mutation dirs still there) — nothing to
  repair, by construction.
- `note`, `draw`, `mathematical` were split, and their features already use the correct
  relative-escape convention from their (artifact-root-owned) case, landing back inside the owner —
  verified against disk, all resolve.
- `gltf`'s case tests only 7 of 120 leaf mutations directly via inline `params`, not via mutation-dir
  `asset://` fixtures — nothing to repoint.
- `gif`/`obj`/`dxf`/`las`/`bcf`/`avi` already had per-subset test CASES (not just per-subset schema)
  with `shared://`/`local://` fixtures resolved at the artifact-root `🧫️fixtures` or subset's own
  `📚️examples` — all confirmed present on disk (`shasum`/`ls`, not assumed from the gate alone).
- `sequence`: mid-sweep, shard B3's concurrent case-split (artifact-level `mutate-sequence-1` → three
  subset-owned cases: `✳️any/round-trips-the-committed-document`, `✳️dependency/mutate-sequence-1-
  dependency`, `✳️step/mutate-sequence-1-step`) transiently left the latter two referencing
  `asset://../✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` — which the framework's fixture
  resolver (`resolveFixtures` in the test package `🟦️.ts`, ~line 957) explicitly rejects: `asset://`
  is guarded to the case OWNER's own subtree (`resolve(abs).startsWith(guard + sep)`), so a
  subset-owned case can never escape to a sibling subset. This produced exactly 2 fresh
  `missing-fixture` breaches, caught by a gate run mid-sweep. B3 had already committed the correct
  fix (a case-local copy of the 227-byte demo document at each case's own `🧫️fixtures/🗣️.dsl.semio`,
  byte-identical to the `✳️any` original — verified with `diff`) between my detection and my next
  read; by the time I went to repoint the URIs to `local://🗣️.dsl.semio` the feature files already
  read that way. No action needed from B2 here — confirmed via re-read, not assumed.

## The `fixture-digest-mismatch` straggler

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧪️oracle/🔣️.json#pattern-shell/primary-obj`.
The brief described the recorded digest as empty; measured, it was instead stale (a hash and byte
count from before the obj split moved/regenerated the fixture, not empty). Recomputed from the
actual file on disk:

- `sha256`: `caa5195064e5011f97fccc8afedb2d61a7cf5e844b2f9dda9de9dc52e6f32418` (manifest, stale) →
  `1871d6af2e4dae48e66fb028e8767eab71c24804ef096a71bbdabebdad4b2810` (measured via `shasum -a 256`)
- `bytes`: `482` → `487` (measured via `wc -c`)

Scanned every `files[].sha256`/`bytes` entry across every `fixtureManifests[]` block in that same
manifest, and across every other `🧪️oracle/🔣️.json` under the whole `stdio/obj` artifact tree — this
was the only entry off, no sibling digest was lost in the same move.

**Race note**: this edit was silently reverted once by a concurrent auto-commit
(`a807c0706c63eb4527de4541173d4f8fd704471f`, 15:18:09, a huge cross-shard snapshot) that captured a
stale in-flight copy of the manifest from another session touching the same file (shard B1 owns the
`mutationCatalogs` block in this same JSON, not `fixtureManifests`, so this was likely a
read-modify-write race, not an intentional revert). Caught by re-reading the file after the mid-sweep
gate run showed `fixture-digest-mismatch` still at 1 with the OLD hash back in place; reapplied the
edit and it stuck through the final gate run.

## Gate results (`bun ./📜️script.ts test contract`, `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`)

| breach class | before | after |
|---|---|---|
| `missing-fixture` (fem2d/fem3d paths) | 150 (75+75) | 0 |
| `missing-fixture` (all other paths, incl. the transient sequence pair) | 0 | 0 |
| `fixture-digest-mismatch` | 1 | 0 |
| `orphan-fixture` | 0 | 0 (not raised) |

The overall breach total and class mix moved a great deal between runs (e.g. `oracle-in-production`
316→0 shown, `unsplit-artifact-subset` 933 appearing) — that is concurrent shards' work landing in
the shared tree mid-session (confirmed via `git log` on the affected files), not anything B2 touched;
only the classes and paths in this shard's brief are reported above.

## Files touched

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🧪️tests/mutate-fem2d-1/🥒️.feature` — repointed 3 hardcoded
  `✳️any` segments to `✳️<subset>`, added `subset` column to the `id-spec-vector` Examples table.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/mutate-fem3d-1/🥒️.feature` — same.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️geometry/🧪️oracle/🔣️.json` —
  recomputed `sha256`/`bytes` for the `pattern-shell`/`primary-obj` fixture entry.
- `$TICKET/🩹️b2-repoint-fem-mutation-subset-uris.py` — the repair script (kept, per ticket rules).
- Raw gate stdout from each run was written to `$TICKET/🗑️generated/b2-gate-*.txt` and deleted after
  the counts above were extracted, per ticket rules.

No `🧪️oracle/🔣️.json` `mutationCatalogs` blocks were touched (B1's territory). No case directories
were moved or split (B3's territory).
