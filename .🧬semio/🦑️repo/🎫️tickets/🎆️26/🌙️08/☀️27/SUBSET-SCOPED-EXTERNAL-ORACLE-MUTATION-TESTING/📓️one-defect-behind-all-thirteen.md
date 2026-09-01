# 🧬️ One defect behind all thirteen — and a correction to what I wrote into the source

## The thirteen are not two problems

I had filed the remaining gap as two: nine `mathematical` kinds needing a schema decision, four
`sequence` kinds needing a fix already in source. They are **one defect**, in
`ArtifactChild`:

```rust
// 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:2564
pub struct ArtifactChild<S> {
    pub child_id: String,
    pub target: crate::os_io::ArtifactRef,
    #[serde(skip)]
    local_owner: Option<Arc<dyn Any + Send + Sync>>,   // ← the document body lives here
    ...
}
```

The materialized scene is `#[serde(skip)]`. It is populated for a snapshot built in-process and ABSENT
for one decoded from bytes. Every export over a child-backed snapshot inherits that, and the two
plugins differ only in how they swallow it:

| | accessor | absent owner becomes |
|---|---|---|
| `mathematical` | `mathematical_scene_owner(..) -> Option<Arc<MathematicalWorkingScene>>` | `Option::None` — the exporter never used it, so the handle is serialized verbatim |
| `sequence` | `sequence_working_scene_for_handle(..)` = `local_owner().map(..).unwrap_or_default()` | an **empty scene**, silently |

## The correction: I overstated the sequence fix, in a docstring, in the repository

When I changed `SequenceIntoJson` to serialize `from.to_fixture()` I wrote that the hop is now
`IoFidelity::Exact` "for real", and in the ticket that *"`sequence` has no such hazard: `to_fixture()`
is total and infallible."*

`to_fixture()` is total. It is total **by defaulting to empty**, which is not the same thing and is
arguably the worse of the two shapes: `mathematical` at least returns an `Option` a caller could
branch on, while `sequence` cannot distinguish "no steps" from "owner absent". A decoded snapshot
exports as `{schema, steps: [], edges: []}` and still claims `Exact`.

The docstring in that file has been corrected to say so. The change is still an improvement — a live
export now carries steps and edges where before it carried neither — but the `Exact` declaration is
conditional, and the source now says which condition.

## Scope, swept rather than assumed

**29 artifacts** hold `#[child(...)]` handles (`✒️writer`, `➗️mathematical`, `🧩️assembly`, `🌊️flow`,
`🏔️gisterrain`, `🗺️gismap`, `🎬️present`, `🎥️shooting`, `🎬️sequence`, `🏛️program`, `🧊️process3d`,
`💠️lowpoly`, `🔌️wires`, `📋️forms`, `📏️layout`, `📐️cad`, `📘️en1990`, `📙️din18599`, `📖️playbook`,
`📜️imperative`, `📸️remodel`, `🔋️model`, `🔌️jack`, `🕸️dag`, `🖨️raster`, `🧿️semio`, `🖐️5d`, `🧊️3d`,
`🗂️curate`), out of 335 export serializers repo-wide.

Of the eight exporters across the two subsets in question, only three reach the scene at all
(`sequence` csv + json, `mathematical` csv); the txt/md/json remainder serialize the snapshot directly
and therefore emit handles.

**The corpora built in this ticket are NOT affected.** `SemioDrawingSnapshot` holds `canvas`, `styles`
and `layers` as inline `#[state(artifact)]` fields — only `✳️kit` and `✳️object` are child-backed
within `🧿️semio` — so the 17 drawing fixtures, the 5 json ones and the 4 sequence csv ones stand.

## Where this actually bites, stated precisely

It does **not** weaken the 601. `externalOracleCoverage` and `oracleEvidenceCoverage` ask whether a
qualifying oracle is registered and whether a fixture exists to run it against; both are answered by
authored, third-party-written fixtures that never pass through our exporters.

It bites on the SUBJECT side — `runtimeMutationCoverage`, which reads 0.00% (0/40) for every subset
today. When our implementation is finally exported and compared against these fixtures, any
child-backed artifact whose exporter serializes the snapshot directly will produce handles or an empty
scene, and the comparison will fail for a reason that has nothing to do with the mutation under test.

That is the honest shape of it: **13 mutations blocked now, and a latent subject-side blocker across up
to 29 artifacts later** — one root cause, one place to fix it.
