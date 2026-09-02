# stdio MeshData call-site conversion — already done when this slice started (2026-09-02)

## Assigned scope

Convert `semio-s-plugin-stdio`'s production `MeshData` call sites off serde, in the 5 window
files named in the brief:
`🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{mesh,model,presentation,table,value}/{👁️viewer/🎭️modes/👁️view,✏️editor/🎭️modes/✏️edit}/🪟️windows/🪟️main/🦀️.rs`.

## Finding: already converted, not by this session

The brief's own finder command
`grep -rn 'MeshData' --include='*.rs' ✏️s/🔌️plugins/🗄️stdio | grep -E 'serde_json|Serialize|Deserialize'`
returned **zero matches** — no production `MeshData` call site in stdio uses `serde_json`,
`Serialize`, or `Deserialize`. All 10 window files (2 modes × 5 subsets) already read:

```rust
("data".to_string(), pack::json_from_dsl_value(&dsl::to_dsl_value(&mesh_from_kind(...)).expect("MeshData serializes"))),
```

— i.e. already on `dsl::to_dsl_value`/`pack::json_from_dsl_value` (the `ToValue`-based path), not
`serde_json`. `MeshData`'s only remaining `serde_json::` hits repo-wide are two unrelated `json!`
report literals and a `#[cfg(test)]` consumer in
`🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️.rs:1183/1186/1969`
(a `validate_sync` diagnostics report, not `MeshData`) — left untouched, out of scope.

No edits were made in this session — the assigned slice was a no-op because a concurrent peer
session had already landed it (consistent with the large in-flight repo-wide rename sweep visible
in `git status` at session start, and with `.../📓️mesh-data-from-value-2026-09-02.md`'s and
`.../📓️shooting-animate-flow-serde-to-value-conversion-2026-09-02.md`'s trail in this same
ticket, which describe the `MeshData::FromValue` groundwork and the shooting/animate/flow waves
that were blocked by the stdio-side gap this slice was meant to close).

## Verification — REAL error counts, isolated target dir, foreground, no Monitor/sub-agents

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target2
export RUSTC_WRAPPER=""
cargo check -p semio-s-plugin-stdio --message-format short
```

Full (untruncated) output captured to a file first, then counted with
`grep -cE ': error(\[|:)'` per the ticket's own anchored-`^error`-undercounts warning:

| crate | before (claimed in brief) | after (measured, this session) |
|---|---|---|
| `semio-s-plugin-stdio` | ~6 `MeshData: Serialize/Deserialize` errors | **0** (exit 0, `Finished` `dev` profile, 1458 warnings only) |

"Before" was not independently reproduced (no stash/checkout available in this shared tree, and
none should be used per CLAUDE.md) — reported as stated in the brief. "After" is directly measured
and real.

## The broader goal (shooting/animate/flow/sourcing unblocked) is NOT yet fully true

Checked all four downstream plugins in one pass (same isolated target dir):

```
cargo check -p semio-s-plugin-shooting -p semio-s-plugin-animate -p semio-s-plugin-flow -p semio-s-plugin-sourcing --message-format short
```

Exit 101, **661 real errors** (`grep -cE ': error(\[|:)'` on the full untruncated output). Of
those, only **7** mention `MeshData` or `semio_s_plugin_stdio` — the rest (654) are unrelated
concurrent churn already tracked elsewhere in this ticket/repo: `E0432` missing
`mutations::*::mutation` submodules, `E0053`/`E0308` "found future" (the repo-wide async-convention
wave), `InputBuilder`/panel-builder API mismatches (a concurrent UI-contract rewrite), etc. — none
of these are serde/MeshData-shaped and none are in this slice's scope.

Of the 7 stdio-family errors:

- **1 genuine remaining `MeshData: serde::Serialize` call site — NOT in stdio, NOT in the assigned
  5 files.** It's in `semio-s-plugin-sourcing`'s own crate:
  `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:242`
  (`kind_mesh_json`): `json!({ "id": kind.id, "data": mesh })` where `mesh: MeshData`, via
  `serde_json::json!`. Same fix pattern as this ticket's precedent (`os-flow`) would apply
  (`pack`/`dsl`'s `ToValue`-based JSON builder instead of `serde_json::json!`), but it is
  `sourcing`'s own production code, not stdio's — out of this slice's assigned scope. Flagged
  separately (see `spawn_task` in the session, title "Convert sourcing's kind_mesh_json off
  serde_json for MeshData").
- **6 are `PdfSnapshot: Serialize`/`DeserializeOwned` errors** in `shooting`'s and `animate`'s own
  PDF import/export bridge files, referencing `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`.
  This is a **different stdio type** (`PdfSnapshot`, not `MeshData`) that still lacks a
  `ToValue`/`FromValue` (or serde) path reachable from those plugins — explicitly out of scope
  (the brief named `MeshData` only). Also flagged separately.

## Files touched this session

None — read-only verification (`grep`, `cargo check`) only, no source edits, per the assigned
slice already being complete. No ticket close/reopen performed, per instructions.
