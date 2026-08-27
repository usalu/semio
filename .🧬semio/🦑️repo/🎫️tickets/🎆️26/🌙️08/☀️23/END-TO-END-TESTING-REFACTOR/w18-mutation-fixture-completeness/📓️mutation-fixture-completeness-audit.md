# 🦠️ Mutation-fixture completeness audit

Baseline: `a8d1caf41f68204e73ff5e47ce40c5f543ed442d` (repository HEAD when this audit ran).
Question: *is every single mutation tested with fixtures?*

## 1. What the repository declares

| measure | count |
| --- | --- |
| `🧪️oracle/🔣️.json` manifests carrying `mutationCatalogs` | 171 |
| declared mutation catalogs | 147 |
| declared mutation kinds | 2147 |
| kinds with `deferredKinds` (declared-but-skipped) | **0** |
| feature files tagging `@mutations-<catalog>` | 145 |

Measured with `📜️script.ts` / `📜️detail.ts` / `📜️coverage.ts` in this folder. `📜️coverage.ts`
imports the platform's OWN `parseFeature`, so scenario-id expansion (`mutate-<kind>` /
`inverse-<kind>` from a `Scenario Outline`'s first `Examples` column) is the platform's, not a
re-implementation.

## 2. Feature-level coverage — the repository's own definition

`mutationCoverageBreaches` is the gate that means "every mutation is tested": a feature that tags
`@mutations-<catalog>` must expand to a `mutate-<kind>` AND an `inverse-<kind>` scenario for every
kind the catalog declares.

| measure | count |
| --- | --- |
| kinds with a `mutate-<kind>` scenario | 2144 / 2147 |
| kinds with an `inverse-<kind>` scenario | 2144 / 2147 |
| catalogs no feature claims at all | **2** |
| features claiming a catalog that does not exist | 0 |

The three uncovered kinds were all in one owner, `🧰️framework/🛍️products/💻️os/🎚️config`:

* `os-config-merge-policy-1-any` → `change-merge-policy`
* `os-config-identity-1-any` → `sign-in`, `sign-out`

`sign-in` and `sign-out` additionally had **no committed specification vector at all** — their
`🧬️schema/🧬️mutations/<slug>/` leaves carried no `🧪️tests/` bundle, only in-file unit tests.

## 3. Why those three hid — two independent causes

### 3.1 The catalogs were structurally unrepresentable

`mutationCatalogProblems` required `standardDirectoryName` / `subsetDirectoryName` on every catalog
and required the owner path to END with `/🏅️standards/<std>/🪆️subsets/<subset>`. `🎚️config` is a
framework facet: its vocabulary is versioned with the product, not with a published standard, so it
has no such coordinates to restate. Its three catalogs therefore could not be written in a form
`strictMutationCatalogs` accepts — and since that function THROWS rather than reports, the whole
contribution would have been dropped the moment discovery reached it.

`mutationVectorRegistryBreaches` already handled the profile-less case (`if (markerIndex < 0)
continue`), so only the validator disagreed with the rest of the platform.

### 3.2 Contribution discovery is currently blind repository-wide

**This is a much larger finding and it is NOT this ticket's to fix.** At the pinned baseline:

```
discoverTestContributions(repoRoot)  →  4 contributions
```

All four are stale scratch copies inside
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️energy-support-current-*/`.
Not one real repository owner is discovered.

Cause: `testFilenameForKind` renders the canonical physical leaf name from the taxonomy as
`<emoji><extensionChain>` — the `physicalLeafRendering.filename =
"file-kind-emoji-and-extension-chain"` contract — which yields `🔣️.json` and `🥒️.feature`. The
repository itself still carries the pre-rename stems `🔣️component.json` and `component.feature`
everywhere except those four scratch trees. A raw walk finds **182** `🧪️oracle` directories and
**174** manifests; the platform's own walk matches **4**.

Consequences visible in `📄️contract-baseline.txt` (`bun ./📜️script.ts contract`, 364 high-priority
breaches):

* ~24 real owners report `unregistered-mutation-vocabulary` — "a mutation vocabulary is declared
  here but no catalog registers it" — while their catalog is sitting in a manifest the walk never
  opened.
* `mutation-catalog-unclaimed` can only fire for the 4 scratch catalogs, so the gate that would have
  caught §2's three kinds cannot see them.
* `.🧬semio` reports 409 "unmanaged tests" because the ticket scratch tree is walked as if it were
  repository source.

This belongs to `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` (the half-applied physical-leaf rename)
and to whoever owns `pathExclusions` (ticket scratch trees should not be walked as owners). It is
recorded here rather than patched, because a compatibility shim over the two names is exactly what
`AGENTS.md` forbids and the rename is one atomic repository-wide operation.

## 4. What this ticket changed

1. **`🪪️sign-in` and `🚪️sign-out` now carry committed specification vectors** — the full
   `(before, mutation, after, diff, outcome)` quintet plus a Rust law file, in the same 13-node
   bundle shape as `📌️set-default-app` and `🛡️change-merge-policy`. `sign-in`'s vector deliberately
   starts from an ALREADY signed-in record so it exercises the REPLACEMENT branch, which is the one
   whose inverse must read BASE rather than its own payload. `sign-out`'s pins the signed-out
   spelling as the bare literal `null` that `#[serde(transparent)]` over `Option<Identity>` forces.
2. **Two language-agnostic cases** under `🔌️plugin/🖥️host/🧪️tests/` —
   `mutate-os-config-merge-policy` and `mutate-os-config-identity` — each claiming its catalog, each
   with a Rust adapter that drives production dispatch and asserts inside the subject handler
   (recorded no-oracle decisions run no oracle role). Both add a guard scenario for the outcome class
   no committed vector can express: the merge-policy `mutation.no-op` warning that must hand the
   record back UNCHANGED, and the empty inverse of signing out of an already signed-out record.
3. **Codec bridges** for the merge-policy and identity vocabularies, mirroring the opening
   vocabulary's, so an external adapter can drive production code without reimplementing anything.
4. **`mutationCatalogProblems` now derives profile requirement from the owner**: coordinates are
   required exactly when the owner path carries `/🏅️standards/`, and forbidden otherwise. Schema and
   type follow.
5. **The `🎚️config` manifest declares real `vectors`** for all five kinds of its three vocabularies,
   and its two no-oracle rationales now name the committed vectors they rest on (`substitutes` also
   corrected — `multi-implementation` was not a value the schema's enum allows).

After this change every declared mutation kind in the repository has both a `mutate-<kind>` and an
`inverse-<kind>` scenario in a language-agnostic feature, and every catalog is claimed by exactly one
feature.
