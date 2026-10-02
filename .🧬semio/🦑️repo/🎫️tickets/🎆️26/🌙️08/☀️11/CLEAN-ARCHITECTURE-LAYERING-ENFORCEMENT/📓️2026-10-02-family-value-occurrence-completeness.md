# Current family value occurrence completeness

Read-only source inspection; no execution or native proof. Source owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📍️ownership/🟦️.ts`.

The current activation predicate catches reserved six-family identifier tokens (line28), but the ownership inspector can subsequently discard those same actual value occurrences. Its `bare` predecessor whitelist at line104 excludes `{`, `;`, and `return`; `consumer` at111 only includes bare/import/qualified facts; line112 silently continues when declarations are also empty. This is a concrete activation-versus-inspection gap.

| Bounded specimen | Current source-derived result | Required proof outcome |
| --- | --- | --- |
| `fn f(){ EngineKey([0u8;32]); }` with foreign glob import or unavailable binding | EngineKey follows `{`; foreign glob does not name a reserved symbol, no qualified path, no reserved declaration. Consumer false, source skipped. | Explicit `unproven-family-binding`; require occurrence-specific direct canonical constructor binding and mounted provider. |
| `fn f(){ return EngineKey([0u8;32]); }` under the same foreign glob | Predecessor `return` also fails bare whitelist; consumer false. | Same explicit refusal. Return expression syntax must participate independently of annotation/import detection. |
| `fn f(){ EngineKey::new(); }` with unavailable/foreign binding | Binding inspector lines184–188 emits qualified `EngineKey::new`; ownership line76 selects it and line93 refuses missing direct canonical route. | Already explicit refusal by current source branch; retain this positive control when closing constructor/value cases. |

The first two can be native-valid with an external crate exporting tuple struct `EngineKey(pub [u8;32])` and `use foreign::*;`; do not add a same-file reserved declaration because that would mask the skip through declaration-origin detection. Validity is source reasoning only, not a compiler receipt. Ordinary field identifiers or enum variant namespaces need separate classification; this finding does not demand banning them.

Trait-only alias activation remains narrower than actual provider identity: activation line30 recognizes only literal roots `semio_framework_2d`, `semio_framework_os_kernel`, or `store`, not a Cargo-renamed canonical provider. A trait-only consumer `use renamed_compute::compute::Engine; fn f<T: Engine>(){}` therefore needs captured provider-based activation rather than spelling recognition. High owns this correction; no present completion claim.

Minimal closure: classify reserved value-head occurrences at statement/return positions as candidate obligations, with their source offsets and lexical namespace. If classification cannot prove a constructor route, emit explicit unproven rather than allowing line112 to erase the already activated occurrence. Retain qualified associated routes as the third control.
