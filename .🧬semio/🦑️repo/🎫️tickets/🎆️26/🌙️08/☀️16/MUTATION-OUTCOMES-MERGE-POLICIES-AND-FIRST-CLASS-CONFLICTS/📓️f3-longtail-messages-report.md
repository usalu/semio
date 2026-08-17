# F3 — long-tail message-code remediation (note, shooting, remodel, energy, playbook, forms)

## Gate

`bun ./📜️script.ts verify mutation-outcome-law` breaches in the six leased plugins: **22 → 0**
(`🧪️f3-gate-before.txt`, `🧪️f3-gate-after.txt`). Re-verified with the exact grep from the brief —
0 lines. No new code invented; every leaf uses one of the frozen 7 codes.

## Per-leaf detection (verb family: all are `change/set/update` or `replace`, root-scoped scalars —
none needed `mutation.target-missing`, all targets are document-root singletons that always exist)

**note** (8, `change-*`): `✏️change-pencil-width`, `🌫️change-grid-opacity`, `👁️change-grid-visible`,
`📏️change-grid-spacing`, `📐️change-snap-grid-spacing`, `🔢️change-grid-subdivisions`,
`🧲️change-snap-enabled`, `🧽️change-eraser-radius` — each: `mutation.no-op` (Warning, empty) when
`payload.new_X == base.X`; numeric ones additionally `mutation.invariant` (Fatal, empty) on
non-finite/out-of-domain (`width`/`spacing`/`radius` ≤ 0, `opacity` outside `[0,1]`, `subdivisions` <
1). Booleans (`grid-visible`, `snap-enabled`) get no-op only — no domain to violate.

**shooting** (7, `change-scene-*`): same shape against `base.scene.{sun,shadow,ambient,material}`.
`sun-elevation` Fatal outside `[-90,90]`; `sun-azimuth` Fatal only non-finite (cyclic, no natural
bound); `sun-intensity`/`ambient-intensity` Fatal if negative or non-finite; `material-roughness`
Fatal outside `[0,1]`; `sun-enabled`/`shadow-enabled` no-op only.

**remodel** (4, `replace-*`): `☁️replace-dense`, `⭐replace-sparse`, `🚂replace-tracks`,
`🏗️replace-job` — each: `mutation.no-op` when the incoming value equals `base.results.{dense,
sparse,tracks}` / `base.job` (all derive `PartialEq`, so the equality check is cheap). No Fatal —
these are opaque engine-produced blobs with no local domain invariant to check here.

**energy** (1, `♻️replace-model`): `mutation.no-op` when the parsed `Model` (or the existing
malformed-JSON→`Model::default()` fallback, unchanged behaviour, documented pre-existing honest
degradation) equals `crate::artifacts::model::energy_model(base)`.

**playbook** (1, `✏️change-title`) / **forms** (1, `🏷️change-form-title`): `mutation.no-op` when
`payload.new_title == base.title`. Forms' diff leaf didn't take `base` before — added the parameter
and threaded it through the one call site in `🦠️mutation/🦀️component.rs`.

**Shrink-only allowlist: none.** Every one of the 22 leaves has a cheap equality check available
(all touched fields/values derive or are `PartialEq`), so none qualifies for message-free status.

## In-lease call-site fix

`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:106`
(`apply_remodel_mutation`) still called `protocol::MutationDiff::apply(&mutation.diff(snapshot), ..)`
against a `MutationOutcome<RemodelDiff>` — a leftover from remodel's earlier-wave return-type
migration, not something the message-code fix touches, but it blocked `cargo check` for the whole
crate. Fixed: `.diff(snapshot).into_parts().0` (messages intentionally dropped — this free function's
signature has no message channel, matching how every other call site in this fan-out was told to
adapt).

## Verify (real numbers)

1. Gate: `grep -cE "note|shooting|remodel|energy|playbook|forms"` on
   `mutation-outcome-law` → **0** (`🧪️f3-gate-after.txt`).
2. `cargo check` per crate (`🧪️f3-cargo-check.txt` has the combined run):
   - `semio-s-plugin-shooting`: **0 errors** (31 warnings, pre-existing unused-`base`/dead-code, not
     touched).
   - `semio-s-plugin-energy`: **0 errors** (9 warnings, pre-existing).
   - `semio-s-plugin-note`: **4 errors, all outside this lease** — `semio_s_plugin_stdio`'s
     `DwgSnapshot`/`SvgSnapshot` types have been reshaped by the concurrent
     `FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` ticket (confirmed live in this
     session's `git status`): `DwgSnapshot` no longer has a `bytes` field (now a typed `drawing`
     field), `SvgSnapshot` no longer has `lexical`. Sites:
     `✏️s/🔌️plugins/🗒️note/.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs:6`,
     `.../🎨️svg/🔖️1.1/✳️any/🦀️component.rs:23` (import) and the export-side mirror at `:8`. A
     previous wave's own report (`📓️w3-note-shooting-report.md`) already documented this exact
     stdio-churn pattern blocking note/shooting; not re-fixed here, same reasoning.
   - `semio-s-plugin-remodel`: **2 errors, both outside this lease** (the in-lease one is fixed —
     see above) — `Mp4Track`/`Mp4Sample` (also `semio_s_plugin_stdio`-owned) trip
     `E0282 type annotations needed` at
     `✏️s/🔌️plugins/📸️remodel/.../✏️editor/⚙️engine/🎥️video/🦀️component.rs:2938,2962`, same
     concurrent stdio-schema churn.
   - `semio-s-plugin-playbook` / `semio-s-plugin-forms`: **0 own-lease errors** — both are blocked
     transitively because they depend on `semio-framework-os-flow`, which currently fails with
     `E0615` (`widget.id` used as a field, but `id` is a method on `&artifact::Widget`) at
     `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:193`. Confirmed via
     `git log --date=iso` that this file's last touch (`c8a29e41c5`, 2026-08-16 20:26) is a
     repo-wide auto-commit sweep landed ~2h before this check, i.e. genuine concurrent in-flight
     work outside `🧰️framework/**` (outside every plugin lease, not mine to fix).
3. `cargo test --lib`:
   - `semio-s-plugin-energy`: **280 passed, 0 failed, 0 ignored.**
   - `semio-s-plugin-shooting`: `cargo check` is clean, but `--lib` (which also builds `#[cfg(test)]`
     code) fails — `store::ArtifactStore::<ShootingSnapshot, ShootingMutation>::new(..)` now returns
     a `Result` (framework `🏪️store` API change, outside lease), and the pre-existing test at
     `✏️s/🔌️plugins/🎥️shooting/.../🧬️mutations/💾️binary/🦀️component.rs:51-56` still binds it
     unwrapped. Not touched (framework-owned constructor signature, not this ticket's C1-C10 and not
     my lease).
   - `semio-s-plugin-note`, `-remodel`, `-playbook`, `-forms`: test binaries can't link while their
     `cargo check` errors above stand (all outside-lease causes).

## Files touched

- 8 note leaves: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{✏️change-pencil-width,🌫️change-grid-opacity,👁️change-grid-visible,📏️change-grid-spacing,📐️change-snap-grid-spacing,🔢️change-grid-subdivisions,🧲️change-snap-enabled,🧽️change-eraser-radius}/🔺️diff/🦀️component.rs`
- 7 shooting leaves: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{☀️change-scene-sun-enabled,🌅️change-scene-sun-elevation,🌑️change-scene-shadow-enabled,💡️change-scene-sun-intensity,🔅️change-scene-ambient-intensity,🧭️change-scene-sun-azimuth,🪨️change-scene-material-roughness}/🔺️diff/🦀️component.rs`
- 4 remodel leaves + 1 call site: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{☁️replace-dense,⭐replace-sparse,🏗️replace-job,🚂replace-tracks}/🔺️diff/🦀️component.rs` and `.../🧬️mutations/🦀️component.rs`
- 1 energy leaf: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-model/🔺️diff/🦀️component.rs`
- 1 playbook leaf: `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🔺️diff/🦀️component.rs`
- 1 forms leaf (diff signature + mutation call site): `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-form-title/{🔺️diff,🦠️mutation}/🦀️component.rs`

Logs: `🧪️f3-gate-before.txt`, `🧪️f3-gate-after.txt`, `🧪️f3-cargo-check.txt` (all in this folder).
