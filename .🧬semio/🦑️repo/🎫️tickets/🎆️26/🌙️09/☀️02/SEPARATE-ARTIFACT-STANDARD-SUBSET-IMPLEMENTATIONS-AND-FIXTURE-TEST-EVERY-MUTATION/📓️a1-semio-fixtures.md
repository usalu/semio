# A1 — 🗄️stdio/🧿️semio fixture breaches

## Before / after (measured with `bun ./📜️script.ts test contract`, scoped to `🧿️semio`)

| breach id | before | after |
| --- | --- | --- |
| `missing-fixture` | 268 | 0 |
| `orphan-fixture` | 234 | 0 |
| `fixture-tolerance-profile-unknown` | 154 | 0 |
| `fixture-comparison-profile-unknown` | 154 | 0 |
| `fixture-generator-unregistered` | 65 | 0 |
| **total** | **875** | **0** |

Confirmed by re-running the gate three times across the session (after the reference repair, after
the profile/oracle contributions, and a final stability check) — each time filtering
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` to `id` in the five above and `scope` containing
`🧿️semio`.

No new breaches were introduced in `🧿️semio` outside these five ids: the 43 remaining breaches
under `🧿️semio` after this shard's work (`binary-protocol-drift` 19, `capability-without-manifest`
14, `mutation-kind-undeclared` 3, `runtime-inventory-missing` 3, `mutation-kind-uncovered` 1,
`mutation-inverse-uncovered` 1, `unknown-oracle` 1, `stub-serializer` 1) are pre-existing and out
of this shard's five breach classes.

## Root cause

Two independent drifts, both mechanical, together accounted for essentially all 875 breaches.

1. **`missing-fixture` + `orphan-fixture` (502 of 875) were the two halves of one reference
   drift**, exactly as the ticket hypothesized. A kind-only-basename migration moved every
   case-local mutation vector from a flat named file (`local://🦠️insert-channel.json`,
   `local://🗣️artifact.dsl.semio`, …) to a kind-only file nested one directory deeper
   (`🧫️insert-channel/🦠️mutation/🔣️.json`, `🖼️assets/🗣️.dsl.semio`, …) without ever touching the
   `local://`/`asset://` references inside the nineteen `mutate-semio-<subset>/🥒️.feature` files
   that name them. Every `missing-fixture` breach was a reference to the OLD flat name; every
   `orphan-fixture` breach was the file the migration actually left behind, now unreferenced by
   anything. Verified case by case (not assumed): for every one of the 268 missing references I
   located the file the migration actually produced and confirmed its content matched what the
   scenario needed (e.g. the "combined vector" files contain `{kind, params/mutation, before,
   after}` together; the presentation/model "split triple" files are three separate
   `{🦠️mutation,⬅️before,➡️after}/🔣️.json` siblings) before repointing the reference — nothing
   was invented.

2. **`fixture-tolerance-profile-unknown` + `fixture-comparison-profile-unknown` +
   `fixture-generator-unregistered` (373 of 875) were the same shape one level up**: three
   subsets' (`✳️brep`, `✳️drawing`, `✳️mesh`) own `🧪️oracle/🔣️.json` fixture manifests named
   comparison/tolerance profiles and a generator-oracle id that the SAME file's own
   `comparisonProfiles`/`toleranceProfiles`/`oracles` arrays never defined — the profiles were
   written into the fixture records but the corresponding declarations were never contributed (or,
   for two brep tolerance names, were near-synonyms of profiles the framework or the subset itself
   already owns under a slightly different name).

## What was changed and why

### Reference repairs (`missing-fixture` / `orphan-fixture`)

- Rewrote the `local://`/`asset://` templates in all 19 `mutate-semio-<subset>/🥒️.feature` files to
  point at the file the kind-only migration actually left on disk. Three shapes covered nearly
  every case, applied per subset only after confirming the shape on disk:
  - Combined single-vector subsets (`animation`, `audio`, `cad`, `document`, `drawing`, `flow`,
    `image`, `mesh`, `presentation`'s spec-vector role): `local://🦠️<id>.json` →
    `local://🧫️<id>/🦠️mutation/🔣️.json`.
  - Split-triple subsets (`model`, `presentation`'s spec-vector role): `local://<id>/⬅️before.json`
    / `🦠️mutation.json` / `➡️after.json` → the same names nested one directory under
    `<id>/…/🔣️.json`.
  - Root-level real-artifact assets that were renamed from `<name>.<ext>` to a case-named directory
    with a kind-only leaf (`graph`, `kit`, `text`, `table`, `value`, `audio`/`video`'s shared clip,
    `brep`, `presentation`'s `🗣️talk.dsl.semio`) or simply moved to the fixtures root
    (`drawing`/`image`/`mesh`'s `🗣️artifact.dsl.semio` → `🗣️.dsl.semio`; `value`'s real
    424,392-byte `hexagonal-cut-concrete-forest-left.model.json` source, already committed as a
    bare `🔣️.json` at the fixtures root).
  - `asset://…/📚️examples/…/🖼️assets/🗣️example.dsl.semio` → `…/🗣️.dsl.semio` (24 occurrences
    across `any`, `animation`, `audio`, `brep`, `cad`, `document`, `flow`, `graph`, `kit`, `model`,
    `object`, `presentation`, `table`, `text`, `value`, `video`) — same drift, one level up in the
    artifact's shared example corpus.
  - `drawing`'s spec-vector Examples table named 6 mutation directories without the `-node`/`-nodes`
    suffix their real directories under `✳️drawing/🧬️schema/🧬️mutations/` actually carry
    (`flatten`→`🫓flatten-node`, `group`→`🧷group-nodes`, `rotate`→`🔄rotate-node`,
    `scale`→`📏scale-node`, `unflatten`→`🎈unflatten-node`, `ungroup`→`💫ungroup-node`) — a second,
    smaller instance of the same "renamed but not repointed" pattern, confirmed against the real
    directory names and their `🧪️tests/<slug>` children.
- Relocated a `no-mutation` vector 8 migrations had each left at the fixtures ROOT
  (`🧫️fixtures/🦠️mutation/🔣️.json`, content-verified as the `no-mutation` vector for that subset)
  into `🧫️no-mutation/🦠️mutation/🔣️.json` in `animation`, `audio`, `cad`, `document`, `flow`,
  `image`, `video` — so the one uniform `<id>` template covers `no-mutation` too instead of needing
  a special case. `presentation`'s equivalent bare `no-mutation/🦠️mutation/🔣️.json` (needed for its
  own spec-vector scenario) was additionally COPIED — not moved — to
  `🧫️no-mutation/🦠️mutation/🔣️.json`, because presentation genuinely needs the trivial
  `{"mutation":"noMutation"}` payload at both locations for two different scenarios (mutate/inverse
  against the real derived deck vs. the synthetic spec-vector).
- Deleted one confirmed-dead duplicate: `mutate-semio-presentation/🧫️fixtures/🦠️mutation/🔣️.json`,
  byte-for-byte the same content (whitespace only differs) as the properly-located
  `no-mutation/🦠️mutation/🔣️.json`, referenced by nothing after the repair above.

### Profile / generator-oracle contributions (`fixture-*-profile-unknown` / `fixture-generator-unregistered`)

All in `✳️brep`, `✳️drawing`, `✳️mesh`'s `🧪️oracle/🔣️.json`:

- **`✳️brep`**: contributed `semantic-brep-kernel-edit-v1` (a real comparison profile describing
  the STEP + tessellated-mesh + measured-metrics bundle brepjs-occt and the independent Python
  reader actually compare) and `geometry-tessellated` (a real tolerance profile for the
  curve-replacement family, sized wider than `mechanical-standard` because an arc→spline edit
  carries legitimate chord error by construction). Repointed the 18 `boolean-standard` and 38
  `topology-exact` fixture-manifest entries to the framework's own `mechanical-standard` and
  `analytic-strict` CORE tolerance profiles — genuine synonyms, not stubs:
  `mechanical-standard`'s own description literally reads "the default for the Boolean corpus", and
  the `topology-exact` fixtures are all exact closed-form vertex repositioning, which is exactly
  `analytic-strict`'s stated scope.
- **`✳️drawing`**: contributed `xml-element-tree` (the 17 quick-xml-generated SVG fixtures compare
  one parsed SVG document's element tree — order-significant, since SVG order is paint order —
  which is distinct from the subset's existing whole-bundle `semantic-drawing-carrier-v1` profile
  that compares SVG+DXF+PDF together). Repointed the 17 `exact` tolerance names to the subset's
  already-contributed `drawing-exact` profile (same near-machine-precision semantics for a 2D
  vector carrier with no tessellation freedom, just named `exact` at the fixture and `drawing-exact`
  at the contribution).
- **`✳️mesh`**: contributed `semantic-mesh-manifold-v1` (measurement-based comparison — relative
  volume/area error and normalized Hausdorff distance via manifold-3d, matching the
  `manifold-mesh-measure` oracle's own documented baseline: 0.000e+00 on a round-trip vs.
  1.073e-01/9.988e-01 for a genuinely different solid) and four tolerance profiles
  (`mesh-exact`, `mesh-tessellated`, `mesh-degenerate`, `mesh-scale-relative`) whose boundaries were
  read directly off how the fixture corpus already partitions itself (2 analytic-primitive
  fixtures, 46 ordinary boolean/topology fixtures, 9 `degenerate-*` sliver/hairline fixtures, 8
  `scale-*` fixtures spanning 1e-3 to 1e6 length units). Registered the missing generator oracle
  `manifold3d-three` as a real third-party-library entry (`manifold-3d` 3.5.1 + `three` 0.182.0,
  Apache-2.0/MIT, `testOnly: true`) — distinct from the already-registered `manifold-mesh-measure`
  and `three-carrier-reader` runtime-comparison oracles because a fixture's authority is the tool
  that GENERATED it, and all 65 `third-party-generated` mesh fixtures record that pairing, used
  together, as their generator.

## Deliberately not fixed, and why

- The `comparisonProfile.pipeline`/`toleranceProfile` fields I contributed for
  `semantic-brep-kernel-edit-v1` were left WITHOUT a matching `comparisonPipelines`/`probes`
  registration (structural fields only: `arrays: "ordered"`). Building a real probe-backed
  pipeline is separate infrastructure work outside these five breach ids — and the artifact's own
  conforming exemplar (`📷️png/✳️any`'s `png-1-2-crate-compare-v1`) has exactly the same shape: a
  profile naming a pipeline id with no `comparisonPipelines` entry anywhere in the repository. My
  three new profiles avoid the `pipeline` field entirely for the same reason, using only the
  structural fields the `fixture-comparison-profile-unknown` check actually validates.
- `mutate-semio-mesh/🥒️.feature`'s `@oracle-semio-mesh-typescript-three-independent` tag resolving
  to an unregistered oracle (`unknown-oracle`, 1 breach) was left alone — different breach class,
  pre-existing, not one of the five in this shard's mandate.
- The other 42 pre-existing `🧿️semio` breaches (`binary-protocol-drift`,
  `capability-without-manifest`, `mutation-kind-undeclared`, `runtime-inventory-missing`,
  `mutation-kind-uncovered`, `mutation-inverse-uncovered`, `stub-serializer`) are untouched —
  outside the five ids assigned to this shard.

## Files touched

- 19 `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-*/🥒️.feature` (reference repairs).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{✳️brep,✳️drawing,✳️mesh}/🧪️oracle/🔣️.json`
  (contributed profiles/oracle; repointed fixture-manifest profile names).
- Moved `🧫️fixtures/🦠️mutation/🔣️.json` → `🧫️fixtures/🧫️no-mutation/🦠️mutation/🔣️.json` in
  `mutate-semio-{animation,audio,cad,document,flow,image,video}`.
- Copied `mutate-semio-presentation/🧫️fixtures/no-mutation/🦠️mutation/🔣️.json` →
  `.../🧫️no-mutation/🦠️mutation/🔣️.json`; deleted the now-fully-dead
  `mutate-semio-presentation/🧫️fixtures/🦠️mutation/🔣️.json`.
- Scripts kept in this ticket folder: `🔍️a1-analyze-missing.py`, `🔍️a1-analyze-missing2.py`
  (diagnosis), `🩹️a1-repair-fixture-references.py`, `🩹️a1-contribute-profiles.py` (the two repairs
  above, re-runnable).
