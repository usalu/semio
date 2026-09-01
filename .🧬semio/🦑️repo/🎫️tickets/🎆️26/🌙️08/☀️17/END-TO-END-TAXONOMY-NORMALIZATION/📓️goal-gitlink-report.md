# Gitlink Terminal-Traversal Fix — Report

## Result

The gitlink boundary crossing (`assertTransactionRepositoryPath` refusing every apply with
`Generator input crosses an index-owned repository boundary: ♻️mit-bestand (♻️mit-bestand/recherche)`)
is **fixed and independently verified**. A real `clean taxonomy apply` for scope
`🧰️framework/🔨️modules/🔢️number` now proceeds through staging, both moves, all 5 edits, and the
`plugin-registry` regeneration (which itself re-invoked `nx run @semio-tech/plugin-registry:generate`
and `:check-generated` successfully). It then rolled back at a **separate, unrelated, pre-existing**
verification gate — see "Downstream blocker" below.

## Root cause and fix

The 40k+ generator inputs for the repo-wide `plugin-registry` regeneration were enumerated by
`registryCatalogInputView`'s `entries()` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` (used by `scanRepo`,
`registryExampleCatalog`, `resumeGeneratorInputView`, and N's own wildcard `generatorInputPaths`
visitor) via plain `readdirSync`, with no awareness of git-index gitlink boundaries. It recursed past
`♻️mit-bestand/recherche` into the submodule's checked-out content.

Fix, confined to the generator-input enumeration path (admission-side
`sourceAdmissionRepositoryFences`/the admission guard were **not** touched, per scope discipline —
a concurrent session owns that side):

- **D** (`🔍️discovery/🟦️component.ts`): added `registryCatalogGitlinkBoundaries(repoRoot)` — a
  cached, one-shot `git ls-files --stage -z` read, filtered to stage-0 `160000` entries, NFC-normalized.
  `registryCatalogInputView.entries(path)` now returns `[]` for any path in that set: the boundary is
  still recorded as a directory node (`kind()` unchanged, one `lstat`, no recursion), but nothing
  beneath it is ever `readdirSync`'d, hashed, or admitted. This is the single choke point shared by
  every consumer above, so the fix is general, not a special case for this one submodule.
- **N** (`🧹️normalization/🟦️.ts`): `assertTransactionRepositoryPath` gained a third access role,
  `"input"`, alongside `"point"`/`"subtree"`. Generator-input accesses (`transactionRepositoryBootstrapPaths`)
  now use `"input"` uniformly instead of `nodeKind === "directory" ? "subtree" : "point"`. `"input"`
  rejects only paths *strictly below* a fence (`sourceAdmissionContainingRepository(path, fences, false)`),
  admitting the fence path itself and any of its ancestors — because a terminal generator-input node is
  recorded once, never descended. `"subtree"` (moves, embedded roots, symlink edits, generator outputs,
  reference edits) is untouched and still rejects on any overlap, since a real recursive filesystem
  operation over an ancestor of a gitlink actually would relocate/touch the nested repository — that
  guarantee had to stay exactly as strict as before.

## Test

New suite `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️registry-catalog-gitlink-boundary/`
(`🔣️.json` fixture + `🟦️.test.ts`, mirroring `🧪️cargo-discovery-exclusions`'s
source-extraction-plus-virtual-filesystem shape). Registered as `test-registry-catalog-gitlink-boundary`
in `📋️project.json`, `📜️script.ts`, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (order
`410.2152`, right after the neighboring `historical-document-evidence` gate). 3 tests, all passing via
`bun nx run @semio-tech/repo-lib:test-registry-catalog-gitlink-boundary --skip-nx-cache`:

1. `registryCatalogGitlinkBoundaries` parses only stage-zero `160000` rows (a conflicted non-zero
   stage is ignored), NFC-normalizes, and caches (verified against a synthetic git-index byte stream,
   not a real subprocess spawn — `bun test`'s own subprocess handling is broken in this sandbox,
   confirmed independently of this change: even `spawnSync("/bin/echo", ...)` returns status 1 with
   empty stdout when invoked from inside `bun test`, but works under plain `bun run` and under
   `bun nx run`).
2. `registryCatalogInputView` treats a declared boundary as a terminal leaf: `kind()` still reports
   `"directory"`, `entries()` on it is `[]`, a full recursive walk visits the boundary itself plus its
   ancestor and a sibling file, but never any of two seeded nested files, and no `fs.reads`/`fs.fileReads`
   entry starts with the boundary path.
3. Launch/Nx registration consistency (mirrors the existing pattern in the neighboring suite).

`🧪️cargo-discovery-exclusions/🟦️.test.ts`'s existing extraction-based test for `registryCatalogInputView`
was updated to inject a `registryCatalogGitlinkBoundaries` stub (`() => new Set()`) since it re-extracts
that function's literal source text and constructs it via `new Function(...)` with an explicit,
now-one-larger global list; still 5/5 passing.

## Downstream blocker (separate, pre-existing, NOT part of this fix)

Post-apply verification (`projectionStaleViolations`) rolled the transaction back:

    Projection verification found 1 stale old-hierarchy token(s): ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml

Diagnosed with temporary `[DEBUG]` instrumentation (added and fully reverted — `git diff` on the
normalization file carries no debug residue). Root cause: taxonomy schema declares a
projection `artifact-editor-command-bundle-v1` for draw's `🖱️canvas-pointer-down` editor command
(the only one of ~28 sibling commands with its own nested `🔄️fsm` sub-crate) that was **never
executed** — the destination (`✏️editor/🪆️1-any/🎮️commands/...`) exists nowhere in the repo, but five
schema-registered "external consumer" contracts already assert the old literal path must be gone from
`Cargo.toml` (root + draw's own), `📦️glue.rs`, `🔒️dependencies.json`, `📜️script.ts`. This is
presence-based drift detection working as designed (confirmed against `declaredProjectionConsumerPaths`'s
inventory branch, which activates a group the same way), not a verifier bug — and it fires for
**every** scope, not just this one, because a framework-module scope's verification candidate set
turned out to span ~35k paths (near-full-repo), which includes the entire draw plugin tree. No apply
has ever reached this gate before (this ticket's own status doc: "no scope applies today"), so this is
newly-observed, not newly-introduced.

Spawned as a separate follow-up (`task_5cc29252`, "Complete or gate the stale draw editor-command-bundle
projection") with the full diagnosis, repo-wide marker sweep, and both remediation options — it is a
real plugin-specific migration (or a narrower verifier gating change), outside this ticket's
generator-input-enumeration scope, and not something to force through by weakening the stale-token
guarantee.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` — gitlink boundary cache + terminal `entries()`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — `"input"` access role.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️registry-catalog-gitlink-boundary/🔣️.json`, `🟦️.test.ts` — new.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️cargo-discovery-exclusions/🟦️.test.ts` — updated stub injection.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`, `📜️script.ts` — new test target.
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — new gate entry.
