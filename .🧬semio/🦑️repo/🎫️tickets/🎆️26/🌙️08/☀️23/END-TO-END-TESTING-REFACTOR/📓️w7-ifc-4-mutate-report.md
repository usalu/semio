# Wave 7 — IFC 4/✳️any exhaustive mutation oracle

Assignment: 🏗️ifc standard 🔖️4 subset ✳️any. Reference: `ruststep` 0.4 (already linked). Verified with
`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-ifc-4` from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

## Result

**23/23** oracle scenarios green. Contract check: **0 breaches**, none naming `ifc`.

```
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0
```

Rust SUBJECT phase does not compile this wave — confirmed independently (not just taken from the
brief): `cargo check -p semio-s-plugin-stdio --lib` fails at workspace-manifest load, unrelated to
this ticket (`✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml` fails `workspace.dependencies.js-sys`
inheritance). The subject half is written and `sut`-gated regardless (see files below); only the
oracle phase is claimed green.

## KEY INSIGHT confirmed, not assumed

IFC4 is a real ISO 10303-21 (STEP physical file / Part-21) exchange structure whose DATA section
carries IFC4's own EXPRESS schema instead of an AP-series one. `ruststep`'s `ast` module parses the
Part-21 GRAMMAR only (no generated IFC4 schema module exists in the crate), so the same reader this
wave's `mutate-step-ap214` case registered applies here unchanged — confirmed by feeding it this
subset's real 24792-entity Nakagin Capsule Tower fixture and observing zero errors after the fix
below.

## §6 applies: ruststep is the independent READER only

`ruststep` 0.4 has no writer (confirmed independently, same finding as `mutate-step-ap214`'s own
report: no `Display`/`fmt::Formatter` impl on `Exchange`/`DataSection`/`Record`/`Parameter` anywhere
in the crate). Every scenario is typed `@mode-property`/`@mode-round-trip`, never
`@mode-differential`. The oracle dispatcher performs every mutation via ruststep's real parse plus
this module's own from-scratch Part-21 writer (a deliberate, brief-compliant duplication of
`mutate-step-ap214`'s own writer — STEP is not one of the fleet brief's six named shared-family
modules, and editing another artifact's file to extract one is out of bounds for this ticket).

## A second, genuine ruststep defect found and worked around

Beyond the reader/no-writer gap already known from the STEP AP214 case, this real fixture surfaced a
SECOND, independent confirmed defect in `ruststep` 0.4's tokenizer (`src/parser/token.rs::string`):
it never implements the doubled-apostrophe escape (`''` inside a string = one literal `'`) that the
Part-21 grammar itself defines, and that real IfcOpenShell-exported content legitimately uses —
entity `#17012` (`IFCPROPERTYSINGLEVALUE('composePort',$,IFCLABEL('{''guid'':
''019ab243-...''}'),$)`) carries an embedded-JSON string escaped exactly this way. Reproduced
standalone in `ifc-4-oracle-scratch/` before being worked around: the tokenizer silently terminates
the string at the first embedded apostrophe and the failure cascades until the whole `DATA` section
fails to match, surfacing as a misleading "expected `END-ISO-10303-21;`, found `DATA;`" error (the
same class of finding as this wave's GIF 89a defects — documented, not hidden by loosening the
projection).

Worked around with a real string-delimiter-aware single pass
(`escape_doubled_apostrophes`/`unescape_exchange` in the oracle module), not a blind
`text.replace("''", ...)`  — that naive approach was tried first and confirmed WRONG, because `('')`
(a list holding one EMPTY string) is also two adjacent apostrophes at the character level and this
fixture's own real `FILE_NAME` header record carries it twice (`(''),('')`  for empty author/
organization). The correct pass tracks whether the cursor is already inside an open string: a `''`
pair seen mid-string is an escaped apostrophe (sentinel-substituted, then restored after parsing); a
`''` pair seen from a closed state is a fresh empty string and is left untouched. Verified via the
scratch probe against the real fixture: 24792 entities, entity `#17012`'s JSON content recovered
exactly, `FILE_NAME`'s two empty aggregates preserved.

## Deliberate integrity exercise: removing a referenced entity

`insert-entity`/`remove-entity`/`set-entity-arg` are exercised on real building entities, not
synthetic ones. `remove-entity` is deliberately targeted at the real capsule proxy `#16976`
(`IFCBUILDINGELEMENTPROXY`, name `'b'`) BECAUSE the real `IFCRELAGGREGATES` `#16991` references it by
id inside its 180-member aggregate list — the integrity question the assignment calls out. The
dispatcher removes only the one targeted DATA record and does not rewrite `#16991`'s reference,
leaving it dangling; this matches this subset's own production `IfcMutation::RemoveEntity` semantics
(`schema::diff::diff_remove_entity`, confirmed by reading that file), which do not cascade either. A
dedicated test (`remove_and_reinsert_the_real_referenced_entity_16976`) asserts the reference is
present before removal, present-and-dangling after, and that reinserting `#16976` at its original
position with its original args restores the pristine projection exactly.

## Files touched (this ticket subset only)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `pub const KINDS` (11 kebab-case kinds) and the
  `kinds_const_matches_enum_variants_in_declaration_order` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — full
  oracle dispatcher, from-scratch Part-21 writer, apostrophe-escape workaround, semantic projection,
  9 `#[cfg(test)]` unit tests against the real fixture.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` —
  new: oracle registration (`ruststep-ifc-4-any-mutate`), mutation catalog (`ifc-4-any`, 11 kinds),
  `semantic-ifc-v1` comparison profile.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` — new: real 2.5 MB
  IfcOpenShell 0.8.4.post1 export of the Nakagin Capsule Tower, copied byte-identical (md5 confirmed)
  from ticket `26/03/20/EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS/test-nakagin.ifc`. Header
  confirmed `FILE_SCHEMA(('IFC4'))`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/mutate-ifc-4/component.feature` — new: 11 kinds ×
  (mutate + inverse) + 1 identity round trip, 23 scenarios.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/mutate-ifc-4/🦀️component.rs` — new: adapter (oracle
  handlers + `sut`-gated subject module, unverified against the compiler this wave for the reason
  above).

Scratch probes (not part of the shipped implementation) live in this ticket's own
`ifc-4-oracle-scratch/`.

## Not done / out of scope

- No `noOracleDecision` needed — `ruststep` is a credible, actively-maintained reader once its two
  confirmed defects are worked around.
- Did not touch `Cargo.toml`, `📦️lib.rs`, or any file outside this subset's own directories, the new
  fixture, and the new case directory.
