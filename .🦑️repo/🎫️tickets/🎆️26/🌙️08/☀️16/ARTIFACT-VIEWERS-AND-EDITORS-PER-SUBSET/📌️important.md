# Live blockers and parked slices

> Cleared LAST, immediately before `ticket_close` — a non-empty `📌️important.md` blocks the close.

## 1. Two peer sessions are live in this tree (as of ticket start)

- `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` — W1 in flight.
  Owns, in `🔌️plugin/🦀️component.rs`, the regions `🔖️Emit`, `VcsArtifactApp`, `🔖️Exchange`,
  `🧪️testkit`, and `🔌️plugin/🏗️builder/🦀️component.rs`.
- `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` — owns much of `🗄️stdio`
  and `📜️world.wit`.

Mtimes on `🔌️plugin/🦀️component.rs`, `🏗️builder/🦀️component.rs` and `📜️script.ts` were all within
60 s of this ticket's start commit `63686457bdcf0e7ba57a6598a4e224ec6c739f8e`.

## 2. PARKED — `VcsArtifactApp` role guard and `🧪️testkit` (contract §2.3, §2.5)

Lane 0-B was scoped to a **new region only** (`🔖️Surfaces`) because the peer ticket's W1-B holds
`VcsArtifactApp` and `🧪️testkit`. The read-only runtime guard (`viewer.read-only` rejections,
`Rights::Read` store attach, read-only history panel) and the three testkit helpers are therefore
**deferred to the W0 barrier**, to be made once the peer's `📓️w1-b-report.md` exists.

## 3. 18 pre-existing failures in `@semio-tech/repo-lib`

Enumerated in `📓️w0-i-report.md` with evidence. None caused by this ticket (19 → 18 across W0-I).
Not repaired here: several pin vocabulary a live peer session may be mid-rename on, so "fixing" them
could revert in-flight work. Re-check at W4 against a quiet tree.
