# 📓️ H2 — the 143 mutation-vector-registry breaches: causes

These became visible only today. Until the contribution filename was migrated (`🔣️component.json` →
`🔣️.json`), `discoverTestContributions` returned **0** owners, the registry loaded empty, and this
audit had nothing to audit. None of the 143 is new damage.

| Family | Count | Root cause | Remediation | Mechanical | Owner |
| --- | --- | --- | --- | --- | --- |
| `mutation-vector-bundle-invalid` (gltf) | 120 | Half-finished vocabulary rename: `🦠️mutation` → `🧬️operation` | see below — **do not delete** | — | peer wave |
| `mutation-vector-bundle-invalid` (pdf) | 4 | Genuinely missing evidence | populate the bundle | ~30% | pdf owner |
| `mutation-vector-unregistered` (os/config) | 10 | Filename projection mismatch (`🔣️component.json` vs `🔣️.json`) | rename ~60 files | 100% | this wave |
| `mutation-vector-source-id-mismatch` | 9 | Scenario directory name ≠ canonical post-projection id | rename 9 directories | ~70% | mixed |

## 120 gltf bundles — a rename in flight, not damage

Every affected scope has the same shape:

```
bind-default-scene/      🎯️outcome  📸️snapshot/{⬅️before,➡️after}  🔺️diff  🦀️component.rs  🧬️operation
create-accessor/         🎯️outcome  📸️snapshot/{⬅️before,➡️after}  🔺️diff  🦀️component.rs  🧬️operation
change-material-alpha-mode/   … same
add-used-extension/           … same
```

The bundle is complete except that the payload directory is named **`🧬️operation`** where the audit's
frozen constant expects **`🦠️mutation`**. That is a *vocabulary* rename (mutation → operation) being
carried out by a concurrent session, caught mid-flight.

**Deliberately not acted on.** The obvious "remediation" — delete `🧬️operation` — would destroy a peer
session's in-progress work, and the alternative reading is equally live: it may be the audit's expected
bundle constant that is stale, not the tree. Which of `🦠️mutation` and `🧬️operation` is canonical is that
session's decision to land, and the audit constant
(`SOURCE_VECTOR_DIRECTORIES` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts`)
must be updated in the same change. Until then the 120 findings are the correct, visible report of a
half-applied rename.

## 10 os/config bundles — the same filename migration

`🧰️framework/🛍️products/💻️os/🎚️config/…` scopes hold a **complete and correct** structure under the OLD
filenames:

```
sign-out/  🦀️component.rs  🦠️mutation/🔣️component.json
           📸️snapshot/⬅️before/🔣️component.json  📸️snapshot/➡️after/🔣️component.json
           🔺️diff/🔣️component.json  🎯️outcome/🔣️component.json
```

Nothing is missing; the names simply predate the taxonomy's emoji-only `fileKinds`. Same wave as the
contribution rename this ticket completed.

## 9 scenario directories — canonical-name drift

The audit itself states the target for each, e.g.

```
removes-the-selected-generation-2-and-falls-back-to-generation-1
    → removes-generation-2-and-selects-generation-1
drops-the-provided-humidification-to-1-point-25-kg-per-hour
    → provided-humidification-becomes-1-point-25-kg-per-hour
switches-the-primitive-from-implicit-triangles-to-triangle-strip
    → switches-primitive-from-triangles-to-triangle-strip
```

Each rename must be transactional with its catalog reference, which is why the audit calls it a
"transactional rename" rather than a directory move.

## Consequence for this ticket

The vector-registry audit is **not** Protocol v2 surface — it predates it and governs checked-in
physical evidence. What v2 changed is that it can now *see* the tree at all. The platform self-test
that used to pin `144` catalogs and an exact three-element breach list was rewritten to assert the
STRUCTURAL invariants instead, precisely so a peer wave moving these files fails for its own reasons
rather than for this test's frozen constants.
