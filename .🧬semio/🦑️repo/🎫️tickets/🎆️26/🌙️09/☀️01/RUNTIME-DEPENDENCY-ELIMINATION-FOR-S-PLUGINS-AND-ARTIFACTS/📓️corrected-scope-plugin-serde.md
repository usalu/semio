# ⚠️ Corrected scope: the plugin-side serde conversion was never done

## What I had been reporting (WRONG)
- "manifests: 119 → 11 entries, 6 real"
- "the serde half is concentrated in 🏪️store's 75 refs"
Both were artifacts of a bad counter. Corrected below.

## Measured truth
**29 of the s plugins still declare `serde` + `serde_json` in `[dependencies]`** (not 6).
With `#[cfg(test)] mod …` blocks stripped by brace-matching and false positives removed
(`VcsError::Serialize` variant names, `*_serde` first-party modules), production serde
references across the plugins total **~10,712**.

Worst: architect 1636 · puzzle 1716 · norm 1595 · block 695 · stdio 582 · procedural 458 · cad 422
Clean: process 0

This is consistent with the crate-count scoreboard (36 crates per plugin, of which 17 are host-only
WASI machinery and the remainder is the serde family): serde is still genuinely LINKED into every
shipped component, because ~10.7k real code sites still require it. The 36-crate convergence was
manifest gating; it was never evidence that serde had left.

## Why the conversion is mechanical
The types already carry first-party derives — `dsl::DslRecord`, `dsl::DslEnum`, `dsl::DslOps`,
`dsl::DslArtifact`, `ArtifactSchema` — and merely still carry `Serialize, Deserialize` alongside.
So each site is: `Serialize, Deserialize` → `ToValue, FromValue`, `#[serde(…)]` → `#[value(…)]`,
retaining `#[cfg_attr(test, …)]` wherever a `serde_json` differential oracle test reads that type.

## The blocker (critical path)
Attribute keys used by those sites vs. what `#[value(…)]` accepts today:
  supported → rename_all 2319 · default 1505 · skip_serializing_if 400 · rename 150 · tag 62 ·
              transparent 6 · serialize_with 4 · deserialize_with 3 · bound 3
  MISSING   → **flatten 75** · with 6 · skip 3 · bare serialize/deserialize 1 each
`flatten` must land in 🌱️value/✨️derive/🦀️.rs before the conversion wave, or 75 sites fail to
compile. Note `flatten` + `deny_unknown_fields` is forbidden in serde and must compile_error! here too.

## Standing rules that still apply
- Do not clear a `Cargo.toml` line that has not been compiled (the 🏗️fem mistake).
- Naive `grep -E 'serde|Serialize'` is unusable in this repo; strip comments AND `#[cfg(test)] mod` blocks.

## Framework side (measured after the plugin correction)
The plugins LINK the framework, so framework serde ships in the component too. Same method:
**~5,692 production refs** across 🧰️framework. Largest: 🖱️ui 900 · 🛂️manifest 508 · 🕸️graph 274 ·
🖼️assets 280 · 🎠️kernel 213 · 🗺️surface 200 · 📡️replication 109 · 🎭️actor 106, plus several
💻️os modules in the 475-570 band.

**Grand total ≈ 16,400 production serde references** (plugins ~10.7k + framework ~5.7k).

### Both totals are UPPER BOUNDS
The brace-matching stripper skips `#[cfg(test)] mod …` blocks but NOT `#[cfg(test)]` applied to a
bare `impl`/`fn`. Validation on 🏪️store: 103 raw → 23 stripped, while careful line-by-line reading
of the same file gives ~8 genuinely production. So the true figure is materially lower than 16.4k;
treat these as a ceiling and an ordering, not a work count.

### Counting recipe that actually works here
1. strip `//` comments; 2. strip `#[cfg(test)] mod …` blocks by brace matching;
3. match only `use serde|serde::|serde_json|#[serde(|derive(… Serialize|Deserialize …)`;
4. exclude `_serde::`, `Error::(Serialize|Deserialize)`, `VcsError::`, `cfg_attr(test`.
Anything less produces numbers that are wrong by 10x in either direction — it has now misled this
ticket three separate times.

## ✅️ The conversion needs NO manifest changes
`semio-framework-os-kernel` re-exports both halves:
    🦀️.rs:337  pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueError};
    🦀️.rs:347  pub use semio_framework_value_derive::{FromValue, ToValue};
Every s plugin already depends on `semio-framework-os-kernel`, so `ToValue`/`FromValue` and their
derives are in reach everywhere WITHOUT adding `semio-framework-value-derive` to any plugin manifest.

Evidence it already works transitively: 🧱️block uses `ToValue` across 177 files while declaring no
value-derive dependency at all. Same for 🌍️gis, 📏️layout, 📸️remodel, 🗒️note, 🎥️shooting, 🎞️animate.

Consequence: agents must NOT add dependency lines for this. Cargo.toml files are shared, contended
files, and a redundant path dep has already broken `cargo metadata` repo-wide twice on this ticket
(the 🏛️architect stdio-oracle dev-dep, and a `semio-framework-hash` path with the wrong `../` depth).
The only manifest edit this goal still wants is REMOVING serde lines, and only after the crate
compiles without them.
