# 📓️ Resolution of the adversarial gate audit

An independent verifier attacked the claim *"no gate can be satisfied by a run in which nothing was
measured, and no mutation can read as covered unless a QUALIFYING third-party oracle actually produced
its expected result"* and confirmed **8 findings**, each with a runnable demonstration. Seven are fixed;
the eighth is a property of the protocol and is documented rather than patched.

| # | Finding | Status |
| --- | --- | --- |
| 1 | The dependency ratchet compared the committed baseline **against itself** | ✅ fixed |
| 2 | `optional: true` excused a **qualified** probe's hard failure | ✅ fixed |
| 3 | A capability with no `MutationManifest` was **invisible** to the release gate | ✅ fixed |
| 4 | The coverage matrix trusted manifests with **zero validation** | ✅ fixed (via 3 + 7) |
| 5 | A subset that never ran `test inventory` was invisible, not missing | ✅ fixed |
| 6 | The run key was blind to oracle `kind` and probe `qualification` | ✅ fixed |
| 7 | **No validator existed** for pipelines, probes or tolerance profiles | ✅ fixed |
| 8 | `productionDispatch.invoked` and probe qualification are self-reported | 📎️ documented |

## 1 — the ratchet compared the baseline against itself

`DependencyScript` called `ratchetDependencies(sorted, sorted, registry)`: both arguments were the same
array, loaded from the committed `🔒️dependencies.json`. So `newProduction` and `unregisteredTestDeps`
were **provably always empty**, whatever anyone added to a `package.json`, `Cargo.toml`, `go.mod` or
`.csproj`. The ratchet function is sound — the verifier proved it by feeding it a genuinely different
candidate and getting `ok=false` — it simply had nothing to compare against.

Fixed by adding `scanDeclaredDependencies`, which walks the live tree across all five ecosystems and
classifies each declaration production-or-test by whether the declaring file sits in a test-owned
location. The ratchet now runs committed-baseline vs live-scan, and `dependency --scan` prints the
difference. Writing that scanner surfaced three of its own parser bugs, all found by reading its first
output rather than trusting it:

- `[[bench]]` and `[[bin]]` did not reset the TOML section (the heading regex cannot match a
  double-bracket table), so `harness`, `name` and `path` were reported as crates.
- `{ workspace = true }` dependencies were counted as external distributions, reporting the
  repository's own `semio-framework*` crates as third-party.
- Every `key = value` line in a `pyproject.toml` was read as a requirement, so a file declaring **no**
  dependencies yielded four: `name`, `version`, `requires-python`, `package`.

Live scan after the fixes: **131 declared external dependencies, 84 production-reachable, 0 new
production**, and three honestly-reported scan-only entries.

## 2 — `optional` excused a qualified probe

The docstring said `optional` marks "a stage whose probe is not yet qualified", and nothing enforced it:
it was a free boolean. The verifier marked a stage using a `qualification.status === "qualified"` probe
as optional, had that probe report `status: "failed"`, and `evaluatePipeline` still returned
`equal: true` — a qualifying reference reporting failure while the pipeline read green.

`optional` now excuses a stage **only when its probe is registered and not qualified**. An unregistered
probe excuses nothing either, or deleting a registration would become a way to switch a gate off, and
`missingProbes` — computed and then ignored — now fails the verdict. Overclaimed marks are reported in
`PipelineVerdict.overclaimedOptional` and rejected at ingest by `registryRecordBreaches`.

## 3 and 4 — the two mutation registries never met

`mutationCatalogs` (v1's kind/vector vocabulary) and `mutationManifests` (v2's oracle/dispatch/outcome
vocabulary) were independent registries with no cross-check, and `buildCoverageMatrix` reads only the
second. An owner could be 100% v1-complete — every `mutate-<kind>` and `inverse-<kind>` scenario
present — while contributing **zero rows** to `test matrix --enforce`. Because the gated denominators
pool across the whole registry, one properly-manifested owner kept them non-empty, so the omission did
not even trip the empty-denominator guard. The capability was not reported as uncovered; it was absent
from the denominator.

`capabilityManifestBreaches` now requires every declared mutation capability to be owned by a manifest.

## 5 — a subset with no inventory was invisible

`runtimeMutationCoverage`'s denominator came from the inventories that exist. A subset that had never
run `test inventory` contributed nothing, so pooling hid it behind subsets that had. Each manifest with
no inventory is now one uncovered coordinate, named in the gate's `missing` list —
which is why the matrix currently reads `runtimeMutationCoverage 0/1` rather than a vacuous 100%.

## 6 — the run key ignored what a verdict means

Reclassifying an oracle from `cross-semio-implementation` to `third-party-library`, or promoting a probe
from `provisional` to `qualified`, changes what a verdict *means* — and neither was in the key, so the
reclassified run reused the old verdict and the promotion was never measured. Both are now folded in.

## 7 — records nothing validated

`ComparisonPipeline`, `ProbeEntry` and `ToleranceProfile` had no validator at all. `registryRecordBreaches`
now rejects: non-kebab ids, probes with no capabilities or the wrong output schema, a qualification claim
with no evidence, non-finite tolerances, a pipeline with no stages, a pipeline naming an unknown
tolerance profile or an unregistered probe, a stage reading no inputs, `optional` on a qualified probe,
and — the one that matters most — **a pipeline whose every stage is optional**, which would report a
verdict while gating nothing.

## 8 — self-reported fields, stated plainly

`productionDispatch.invoked` is set by the adapter, and a probe's `qualification.status` is set by whoever
registered it. Neither is independently verified, and neither can be from the orchestrator's position:
proving an adapter really reached production dispatch would mean instrumenting the production code, and
proving a qualification would mean re-running the spike.

What they DO buy is real and worth being exact about: an adapter that replays a committed vector must
now **actively assert** that it did not, in a field a reviewer can grep for and a diff will show. That
converts a silent omission into a written claim. It is a weaker guarantee than measurement, and the
specification says so rather than implying otherwise.
