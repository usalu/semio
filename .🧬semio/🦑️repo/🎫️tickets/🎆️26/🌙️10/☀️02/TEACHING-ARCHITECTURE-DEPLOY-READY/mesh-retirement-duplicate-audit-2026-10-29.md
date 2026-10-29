# Nearby Retirement Duplicate Audit — 2026-10-29

Read-only scan of framework/modules Rust files to directory depth10 checked literal artifact_retire_struct invocations against actual adjacent derive attribute blocks for the same struct identifier. No additional obvious duplicate was found in current Source. The known mesh-engine/canonical/decode State currently has canonical RetireOwned derive at33 and no redundant macro; root repair was already visible when sampled.

MeshData in mesh-engine root is not a duplicate: its actual attributes derive Clone/Debug/PartialEq/Default plus test serialization, while explicit artifact retirement remains separately authored. An initial expression spanning unrelated attributes produced a false candidate and was rejected after checking actual attribute boundaries. No nested-module homonym was promoted into a finding.

Scope is obvious literal structs/macros in nearby framework modules, not expanded Rust syntax, enums, aliases or every deeper mount. Zero remaining textual candidates is not compiler acceptance or a claim that fullsemantic39681 reached TypeScript. No Source edits/builds or duplicate Cargo gate were launched; owning mesh compiler/tests remain root work.
