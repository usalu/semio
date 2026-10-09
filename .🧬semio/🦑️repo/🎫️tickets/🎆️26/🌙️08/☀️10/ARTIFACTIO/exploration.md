# Artifact I/O Exploration

## Decision

Representations belong to `🚪️io/<representation>/<semantic-facet>/[member]`: binary and text precede snapshot, diff, and mutations. Domain schema contains semantic data and actions only. Reuse existing codec traits; remove old module aliases and compatibility paths.

## Verified Scope

Read-only exploration found binary and text representation directories beneath artifact schema mutations throughout plugins. A filesystem traversal counted 1,963 files across 24 plugins at exploration time; a separate `rg --files` inventory counted 1,935, because ignored files differ and other agents are editing concurrently. Counts are observations, not stable acceptance criteria.

The broader schema representation inventory also includes snapshot and diff: `rg` counted 1,579 snapshot and 1,468 diff representation files. These require the same ownership correction. Existing io representation directories occur in artifacts such as animate presentation, so destination collisions need content review rather than blind replacement.

Affected plugins observed: stdio, norm, trinity, puzzle, block, wfc, procedural, gis, fem, space, demonstrator, process, cad, playbook, lowpoly, remodel, raster, architect, shooting, layout, energy, writer, sourcing, and note. Stdio accounts for most mutation representation files (filesystem observation: 1,222); norm contributes 235. Framework artifact schema mutation binary/text directory search returned zero matches.

## Concrete Violation

Generation3d `🧬️schema/🧬️mutations/💾️binary/🦀️.rs` contains substantial OpBinary encoding and decoding, DSL mirror records, protocol metadata, and retained-authority laws. Its TypeScript counterpart is a Uint8Array representation alias. The sibling text Rust file reexports the semantic mutation union and includes grammar metadata. These are I/O concerns despite their current schema ownership.

Generation3d schema mutation root defines the genuine semantic discriminated union and dispatch, but also embeds COMPONENT_GRAMMAR_SEMIO and COMPONENT_GRAMMAR_PATH. Relocating directories alone leaves this representation metadata inside the domain source; audit embedded declarations too.

Do not confuse representation `📝️text` directories with genuine writer domain actions such as edit-text and splice-text. The actions stay in schema; their text/binary codecs move to io.

## Reference Hotspots

- Generation3d artifact root `🦀️.rs` wires nested `schema::{snapshot,diff,mutations}::{binary,text}` modules with explicit path attributes. Both physical paths and module ownership require refactoring. Its existing io root starts near line 357 in the inspected version.
- Generation3d Rust package `📜️script.ts` imports the semantic-wire TypeScript test from schema mutations binary (near line 31 in the inspected version).
- Codec files import `crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation` and snapshot text DSL conversion functions; these should refer to semantic types in schema and conversions in io.
- Explicit include_str protocol/grammar paths, root reexports, package scripts, tests, relative TypeScript imports, and fixture references require validation after moving files.
- Filesystem traversal must supplement rg to cover ignored/untracked representation files.

## Validation Recommendations

Validate that no artifact schema subtree owns binary/text codecs, grammar/protocol files, codec modules, or representation metadata; schema module APIs should expose semantic types only. Validate every path attribute, relative import, and include_str path after relocation. Use existing language-agnostic fixtures and third-party comparisons for codec equivalence, then run relevant nx tasks with bun. Exploration did not execute tests or modify production files.
