# EN 1995 spec-trace under the strict verifier (EN1995-TRACE)

Nothing compiled. `nice -n 19 python -I ⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py`: en1995 68 ok (JSON and Rust twin in sync).

## Root causes
- T-30, T-32, T-33, T-46, T-94, T-95, T-96: the fragments (`en1995.t-30.tapered_edge`, ...) are real check ids, but they are composed at run time by the catalogue bridge (`format!("en1995.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id)` in `⚖️kernel/📇️catalogue/🦀️.rs`), so no literal exists in the sources. Fix: entry-prefix fragments `en1995.t-30.` etc. (the form `emitted()` accepts for catalogue entries, like en1997 `en1997.rw-02.`). At run time `Emitted::with_case_catalogue::<En1995Cases>` seeds every entry, and the entity demo rows (tapers, cutouts, diaphragm, truss, bracing) emit the same ids. T-31 only passed by accident (`"en1995.t-31"` literal in the scene) and is untouched.
- T-71, T-72, T-73: the connection check id was `format!("en1995.{clause}.johansen.{}")`, no literal. Fix in `⚖️timber/🌉️bridge-and-connections`: `(clause, johansen_id)` pair with the two literal prefixes `en1995.8.2.2.johansen` / `en1995.8.2.3.johansen`; the emitted ids are unchanged. Additionally no example had a steel-to-timber connection, so `en1995.8.2.3.johansen` (T-73) was never emitted: new `steel_connection_demo_document()` (compliant beam plus `conn-C3`, double-shear dowels with a centre steel plate, 8 mm), next to `entity_demo_document()`. T-71/T-72 share the 8.2.2 fragment (single and double shear modes in one check).

## assert_traced document list (read)
`🧬️schema/💡️inferences/⚖️checks/🧪️tests/🧭️spec-trace/🦀️.rs`: example documents, compliant beam, noncompliant building, catalogue demo, now also `entity_demo_document()` and `steel_connection_demo_document()` (the entity rows were not part of the list); catalogue seeds cover all `en1995.t-NN.` fragments, `conn-C1` (nail/bolt, no plate) 8.2.2, `conn-C3` 8.2.3. Not run.

## Files
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/`: `🧭️spec-trace/{🔣️.json,🦀️.rs}`, `🧬️schema/⚖️timber/🌉️bridge-and-connections/🦀️.rs`, `🧬️schema/⚖️timber/🏗️structure-and-members/🦀️.rs`, `🧬️schema/💡️inferences/⚖️checks/🧪️tests/🧭️spec-trace/🦀️.rs`.
