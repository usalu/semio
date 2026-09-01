# ➗️ The mathematical projection EXISTS — my "schema decision" claim was wrong in its premise

## What I claimed, and why it was wrong

I filed the mathematical export as needing a schema decision, on the reasoning that `sequence` had a
`to_fixture()` to serialize while *"`mathematical` has NO `to_fixture()`, so defining the projection —
which of the three children reach the carrier, and in what shape — is a schema decision for that
artifact, not a mechanical substitution."*

The premise is false. The projection exists, under a different name, and a sibling serializer in the
same tree already calls it:

```rust
// ➗️mathematical/🗿️artifacts/➗️mathematical/🦀️component.rs
pub struct MathematicalGraph { directed: bool, nodes: Vec<..>, edges: Vec<..>, algorithm: String, algorithm_seed: Option<String> }
pub struct MathematicalGeometry { points: Vec<MathematicalPoint> }
pub struct MathematicalWorkingScene { graph, geometry }                                   // :301
pub fn mathematical_scene_owner(&MathematicalSnapshot) -> Option<Arc<MathematicalWorkingScene>>  // :336, SYNC
```

Those two structs carry **every field the nine blocked kinds touch** — `edges` (`connect-nodes`,
`disconnect-nodes`), `directed` (`change-graph-directed`), `algorithm` (`update-graph-algorithm`), the
whole graph (`replace-graph`), and `points` (`insert-point`, `move-point`, `remove-point`,
`replace-points`). They are already serialized together elsewhere in the same file —
`mathematical_scene_id` does `serde_json::to_string(&(graph, geometry))`.

So "no carrier can record them" was never a fact about the DATA. It was a fact about which function the
JSON serializer happened to call.

## Why it is still not the one-token fix `sequence` was — for a different reason than I gave

`mathematical_scene_owner` reads `snapshot.results.local_owner::<MathematicalWorkingScene>()` and
returns an **`Option`**. `local_owner` is the `#[serde(skip)]` field of `ArtifactChild`: it is populated
for a snapshot materialized in-process and EMPTY for one decoded from bytes. An export built on it
would carry the full graph when the document is live and silently emit an empty scene after a round
trip — a worse failure than today's honest handle, because it would look like data.

`sequence` has no such hazard: `to_fixture()` is total and infallible.

That `Option` is the real design decision — where the export gets its scene when the owner is absent —
and it is genuinely not mine to guess. The correction is that the decision is about SOURCING the scene,
not about defining a projection that does not exist.

## A second defect found in passing: `MathematicalIntoCsv` appears not to compile

```rust
use crate::artifacts::mathematical::{mathematical_graph, MathematicalSnapshot};
fn serialize(from: &MathematicalSnapshot) -> IoResult<IoPayload> {
    let graph = mathematical_graph(from);      // ← async fn, NOT awaited
    let records = graph.nodes.iter()           // ← field access on a Future
```

`pub async fn mathematical_graph(..) -> MathematicalGraph` (`:342`) is `async`, the call site does not
`.await`, and `.nodes` is then read off the returned future. That cannot compile. It is masked because
`semio-s-plugin-mathematical` depends on `semio-s-plugin-stdio`, which fails with 60 `E0046` before
this file is ever reached.

Both `mathematical_graph` and `mathematical_scene` are `async` with fully synchronous bodies
(`mathematical_scene(s).graph`, `mathematical_scene_owner(s).map(..)`) — the async-convention debt
recorded in `AGENTS.md`. The sync accessor underneath (`mathematical_scene_owner`) is what both the
csv and json serializers actually want.

**Unverified**: this is read from source. `cargo build -p semio-s-plugin-mathematical` cannot reach the
file today, so the claim stands as a source reading, not as a compiler result.

## Status of the nine

Still counted as MISSING, and correctly so — nothing was registered against an export that has not been
run. What changed is the reason, which is now specific enough to act on:

* not "no carrier can record this state" — the state is in two plain `Serialize` structs;
* not "the projection must be designed" — `MathematicalWorkingScene` is the projection;
* but "the only accessor to it returns `Option` because it reads a `#[serde(skip)]` owner, so an
  export built on it is correct in-process and empty after a round trip".
