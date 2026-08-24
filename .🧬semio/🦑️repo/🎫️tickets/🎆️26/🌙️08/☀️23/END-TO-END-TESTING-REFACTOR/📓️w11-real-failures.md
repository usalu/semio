# The eleven real failures

Making the inverse and round-trip laws assert **in-role** turned a green sweep into a sweep with
eleven genuine failures. This is the most valuable result in the ticket: before this, those scenarios
passed whenever the reference library merely declined to error.

```
[test] level=exhaustive cases=83 executed=1022 passed=1011 failed=11 not-exercised=22
```

Every one is oracle-side — no subject has run — so each says: **the reference implementation's own
forward-then-inverse does not round-trip**, or the inverse spec the adapter computes is wrong. None
of them can be blamed on our codec yet, and none should be closed by weakening the assertion.

## Vertex accounting — 🧊️obj 3.0 (4 failures)

| Scenario | Divergence |
|---|---|
| `inverse-remove-face` | `vertexCount` 8577, expected 8576 |
| `inverse-remove-group` | `vertexCount` 8579, expected 8576 |
| `inverse-remove-object` | `vertexCount` 8579, expected 8576 |
| `inverse-set-object` | `vertexCount` 8579, expected 8576 |

Removing a face, group or object and then undoing it **adds vertices that were not there before** —
one for a face, three for a group or object. That is a systematic off-by-N in the reconstruction, not
noise: the inverse is re-inserting geometry by value rather than restoring the prior index space.
The real fixture has 8,449 declared vertices plus a deliberate orphan duplicate, so this is exactly
the shape of bug an orphan-tolerant reconstruction produces.

## Page content operators — 📄️pdf 1.7 (3 failures)

`inverse-append-page-content`, `inverse-remove-page`, `inverse-set-page-content` all diverge on
`pages.N.contentOperators`. Notably `append-page-content` was the variant whose `inverse()` was
documented from the start as degrading to `SetSnapshot` for lack of a counterpart — this is that
known weakness finally being measured rather than described.

## Single-field divergences (3 failures)

- 📜️docx `inverse-remove-style` — `styles.1.id` is `"Heading1"` after the round trip, not the original.
- 🔣️json `inverse-set-snapshot` — `$.value.models[0].model.geometry.vertices[19].position` differs.
- 🎨️svg `inverse-remove-element` — first divergence at character 9,946 of the real QR-code drawing.

## The tripwire fired — 📰xml 1.0

```
identity-round-trip: byte pass-through — the oracle's re-encoded bytes are bit-identical to the
input, so nothing here proves the document was parsed rather than copied
```

This is the no-byte-pass-through check working exactly as intended, on the one case where it could
not be dismissed. Two readings are possible and they must be distinguished before anything is
changed: either the XML oracle is not genuinely re-serializing, or `quick-xml` round-trips this
particular document byte-exactly, in which case the fixture is too simple to prove parsing and needs
one that exercises attribute ordering, entity forms or self-closing style.

## How these must be handled

1. Triage each to its true owner: our computed inverse spec, the reference library, or the fixture.
2. Fix the cause. Never weaken the assertion, and never swap in a fixture that merely dodges the
   failure.
3. A failure that turns out to be a genuine property of the format — an inverse that cannot exist —
   gets documented as such in the feature, with the scenario retyped honestly rather than deleted.
