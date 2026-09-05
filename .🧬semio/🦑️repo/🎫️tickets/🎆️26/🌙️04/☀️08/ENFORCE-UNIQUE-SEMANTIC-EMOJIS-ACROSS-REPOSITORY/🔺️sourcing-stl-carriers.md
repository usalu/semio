# Sourcing STL Carrier Review

Scope: only the two foreign STL carrier roots beneath Sourcing's curation subset. Both declare `s.stdio.stl`, standard `ascii`, and consume `StlSnapshot`; the triangle meaningfully identifies the triangle-mesh format. Their six siblings currently use 🔣 JSON, 🧊 OBJ, 🎒 ZIP, 🔤 TXT, 📷 PNG, and the old generic 🟪 STL. 🔺 is free in both scopes.

Exact hand-picked changes, relative to `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io`:

- `📥️import/🧩️deserializers/🗿️artifacts/🟪️stl` → `📥️import/🧩️deserializers/🗿️artifacts/🔺️stl`.
- `📤️export/🧵️serializers/🗿️artifacts/🟪️stl` → `📤️export/🧵️serializers/🗿️artifacts/🔺️stl`.

Only the two corresponding `#[path]` mounts in `📦️packages/🦀️rust/🦀️.rs` change. Public identities remain unchanged. The four source payloads under `🔖️ascii/✳️any` are preserved:

| Direction / leaf | Bytes | SHA-256 before move |
| --- | ---: | --- |
| import / 🦀️.rs | 2241 | `899d1c18a792a516c4485ba409d9bdc48a5090cab60475ccf32fff1dac06e65d` |
| export / 🦀️.rs | 1785 | `133855ea46a8e22f25a8678343b689439455cbe96dbd530851309458bf373e0f` |
| import / 🟦️.ts | 11 | `8e609bb71c20b858c77f0e9f90bb1319db8477b13f9f965f1a1e18524bf50881` |
| export / 🟦️.ts | 11 | `8e609bb71c20b858c77f0e9f90bb1319db8477b13f9f965f1a1e18524bf50881` |

Source review explicitly found existing Rust format-mismatch implementations and TypeScript placeholders. This naming repair does not claim functional STL conversion or native-test success and does not alter those implementations.

Verification completed: both current roots exist, both old roots are absent, the two exact Rust mounts use the current names, and all four after-move SHA-256 values equal the recorded originals. Each parent has exactly one triangle-prefixed entry. No other Sourcing paths were changed in this batch.
