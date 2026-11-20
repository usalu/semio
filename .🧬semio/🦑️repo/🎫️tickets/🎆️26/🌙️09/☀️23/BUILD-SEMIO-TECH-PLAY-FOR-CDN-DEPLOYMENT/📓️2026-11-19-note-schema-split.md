# Note schema crate split under the source-boundary ceiling

## Blocker

`semio-s-artifact-note-note-schema` (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust`) was refused by native source admission (`process.source.read-files-exhausted`): closure 36 files / 265,017 bytes against the hard ceiling 32 files / 262,144 bytes.

## Cause

Besides the schema facets (`🦀️.rs`, `📸️snapshot`, `🔺️diff`, `🧬️mutations`, `💡️inferences`, ...), the schema crate path-mounted the three io snapshot codecs of `✳️any/🚪️io` (`💾️binary`, `📝️text`, `🪶️sqlite` including `🔢️number`, `🗄️.sql`, `📡️.protocol.semio`, `📖️.grammar.semio` and the demo `🗣️.dsl.semio`; sqlite alone is about 85 KB). Splitting `💡️inferences` would have saved only 7 files / 12 KB, which is not enough.

## Change

- New crate `semio-s-artifact-note-note-io-snapshot` at `✳️any/🚪️io/📸️snapshot/📦️packages/🦀️rust` (same workspace and depth as the sibling `🔺️diff` and `🧬️mutations` crates). Its lib mounts the binary, text and sqlite snapshot sources by path at their unchanged module paths (`standards::v1::subsets::any::io::{binary,text,sqlite}::snapshot`) and explicitly re-exports `NoteSnapshot`, `NOTE_DOCUMENT_SCHEMA`, the node types and `any::schema::*` from the schema crate, so the moved sources keep their `crate::` paths verbatim.
- Schema crate lib: the `any::io` mount tree removed (the 45-byte `block::io::binary` mount stays, the hub still re-exports it).
- `🧬️mutations` crate: depends on the new crate; its `🪞️mirror` re-exports `io::{binary,text,sqlite}::snapshot` from it instead of from the schema crate. Every other consumer reaches those modules through the mutations crate, so no other `use` path changed.
- Workspace members of `🗒️note/Cargo.toml` list the new crate. Cargo.lock is left to cargo's normal resolution.

## Result

Schema crate closure: 28 files, well under 262,144 bytes (about 187 KB at most). New io snapshot crate closure: 9 files. The ticket preflight prints no OVER line for `✏️s/🔌️plugins/🗒️note`.
