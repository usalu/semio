# Compiler Dead OnceLock Import Packet

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Compiler component SHA-256: `0be70e2393330cb88d6bf77599e080d9ef42c7c008cc845d86fa2160cf34bff2`; clean.

## Evidence and Lease

The top-level `use std::sync::OnceLock;` has no consumer and is independently reported unused by Rust. The nested `print` module owns a separate `OnceLock` import and live `FONTS` static; preserve both.

Delete only the unused top-level import. Writable paths are the compiler component and one unique Terra acceptance Markdown. Do not edit compiler behavior, nested print imports, Cargo, glue, stdio, or SPR.

The compiler package currently has no Nx project or package script. Validate through package Cargo check as a recorded structural exception plus scoped ordinary/cached diff checks. If the moving SPR channel blocks the package build, preserve source-static acceptance and record the exact external error.
