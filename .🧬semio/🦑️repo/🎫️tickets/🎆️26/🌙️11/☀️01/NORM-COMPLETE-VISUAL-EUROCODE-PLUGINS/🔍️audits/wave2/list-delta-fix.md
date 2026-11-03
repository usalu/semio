# list_delta key fix

Decision: route 2 (local key markers). Changing `list_delta!` so `key: <field>` stops implementing `Keyed` on the row would break every non-norm crate that relies on the row being `Keyed` (bim, procedural, space, ... use `Keyed`/`Parts<..>` with the default `ByKeyed`). A first macro edit (marker = the `$removal` struct) was written and fully reverted; the framework file is unchanged.

Changes:
- 43 `key: id` call sites in norm diff crates (din16798, din18599, din4108, en1993, iso16757 list-row-patches, vdi3805) now use `pub struct <Delta>Key;` + `KeyOf<Row>` + `list: Vec<Row>, key: String = by <Delta>Key`. en1996/en1998 (COMPILE-REVIEW B), en1997, en1999 were already converted. The 3 remaining `key: id` in `norm/compliance/design-case/rows` use a local row, which is fine.
- Generators fixed: en1992-geometry-diff.py, en1993-entity-gen.py, en1993-storey-model-gen.py, en1996_gen_diff.py, field-adder/new_collection_en1998.py (new_collection.py and en1990/s3_diff_rs.py already emitted markers).
- build-loop.md pitfall note updated; "ready for rebuild: list_delta key fix" appended to integrate and build-en* latest.md.
- Not compiled (no builds allowed).
