# W6 g2 — stdio artifact declarative conversion: zip, deflate, bcf, xlsx, pptx, docx, pdf

All 7 assigned artifacts converted to `declaration()` at their artifact root; all imperative
`register()` lines removed from `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`, replaced by
`.artifact(...)` in the builder chain. `zip`/`bcf` fully declarative. `xlsx`/`pptx`/`docx` fully
declarative, `xlsx`/`docx` additionally gained `.subset_validators()` (strict+transitional) that
their old `register()` expressed via a side-effecting `io::register()` call. `pdf` needed TWO
declarations (`declaration()` = 1.7/canonical, `declaration_1_4()` = the frozen 1.4 stub) — verified,
not assumed, per this ticket's own warning: they share one `Dialect.artifact_kind`
(`"s.stdio.pdf"`, hence one shared `kind` in both builders) but are otherwise fully independent
(distinct schema ids, document-codec schema strings, language ids). The old double `crate::
artifacts::pdf::engine::register()` + `standards::v1_7::engine::register()` plugin-root calls
registered 1.7 twice (harmless, since every underlying registry is idempotent-by-key) — the two
declarations now register each standard exactly once, net-equal end state, redundancy removed.

**Left imperative, reported not invented**: `deflate` and `pdf` 1.4 each had a
`register_schema_specs()` call (`dsl::registry::register_schema_spec`, the P2-M3 `FullResolver`
insertion API — a registry distinct from `.languages()`'s `dsl::register_language`, and not one of
`ArtifactDeclaration`'s fields). Not invented, not dropped — both now run via `.setup(fn)` on the
plugin root, matching this ticket's own W1d precedent (puzzle's B2 case: a genuine gap survives on
`.setup()` while everything else moves to `.artifact()`). `zip`/`bcf`/`xlsx`/`pptx`/`docx` never
called `register_schema_spec` (all hand-rolled types, no derivable `RecordSpec` — their own doc
comments already said so), so those five have zero residual `.setup()`.

`xlsx`/`docx` `.subset_validators()` re-derive their strict/transitional `SubsetValidatorEntry` rows
via the same side-effect-free `subset_validator_entry_of::<V>()` constructor their subsets'
(module-private) `validator_entry()` fns already called — no visibility widening into `🚪️io/`
needed anywhere. `pdf` similarly re-derives 8 rows total (1.4: a/x; 1.7: a/x/e/ua/vt/h).

**Silent-rebind check**: every `.composers(...)` call fully qualifies to each artifact's ENGINE-own
`io_registry::entries()` (owned `&'static [ComposerEntry]`), never the artifact root's own
shadowing `io_registry` module (which returns `&'static [&'static ComposerEntry]` references and
would silently rebind under a bare call). `grep -rn "io_registry::entries"` across all 7 files:
zero bare calls, all fully qualified — pasted output above shows each hit prefixed with
`crate::artifacts::<x>::engine::...` or `crate::artifacts::pdf::standards::v1_{4,7}::engine::...`.

**Verify — `cargo check -p semio-s-plugin-stdio --all-targets`, run 3 times (once green, then two
retries after upstream churn touched the crate):**

Run 1 (`scratch-w6-g2-check-1.txt`), taken right after all 7 artifacts + the plugin root were
converted, before any other session's concurrent edit landed:
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-stdio --all-targets
Finished `dev` profile [unoptimized] target(s) in 1m 20s
```
Exit 0, `grep -c "^error"` on the full log: 0. Only pre-existing dead-code warnings (unused demo/
sweep/fixture helpers across docx/pptx/xlsx/bcf/json/semio schema files — none touched by this
pass, none inside my `declaration()` regions).

Between runs, a repo-wide edit briefly inserted a bogus `subsets::any::` segment into several
artifacts' `engine::` paths (json/md/svg/png/jpg/mp4/avi/mp3 — none of them mine, all owned by
other groups' already-landed declarations) AND transiently touched my own `pdf::component.rs`'s
three `standards::v1_7::engine::…` references the same way. Verified against `📦️glue.rs` (`engine`
sits directly under `v1_7`, never under `subsets::any`) that this was wrong; by the time I re-read
the file to fix it, it had already reverted to my original correct paths (confirmed:
`grep -n "subsets::any::engine" pdf/🦀️component.rs` → no match) — a concurrent session's transient
edit-in-flight, not a change I needed to make myself.

Run 3 (`scratch-w6-g2-check-final.txt`, final, stable across two consecutive runs 30s apart):
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo check -p semio-s-plugin-stdio --all-targets
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 603 warnings emitted
error: could not compile `semio-s-plugin-stdio` (lib test) due to 9 previous errors; 737 warnings emitted
```
Exit 101, 11 error lines — **every one** inside `crate::artifacts::semio::standards::v1::engine`
(missing `demo_mesh_snapshot`/`print_mesh_dsl`/`parse_mesh_dsl`/`encode_mesh_pack`, referenced from
`🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/🦀️component.rs`). Proved upstream,
not mine: (1) `🧿️semio` is not one of my 7 assigned artifacts and I never opened any file under
`🗿️artifacts/🧿️semio/`; (2) `stat -f '%Sm'` on the failing snapshot file reports `Aug 13 00:59:01`,
seconds before this check ran — live in-progress editing, not stable churn I could wait out; (3)
zero error lines mention any of `zip`/`deflate`/`bcf`/`xlsx`/`pptx`/`docx`/`pdf`. Classified (c)
upstream/live-churn per this ticket's own protocol ("retry-and-wait, do not patch") — `🧿️semio` is
not mine to fix.

**Files touched**: the 7 artifact roots (`🎒️zip`, `🗜️deflate`, `💬️bcf`, `📕️xlsx`, `🎞️pptx`,
`📜️docx`, `📄️pdf` — `🦀️component.rs` each, `declaration()`/`declaration_1_4()` regions only,
existing regions/tests untouched) and `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (7 `register()`
lines removed [8 counting pdf's second line], 8 `.artifact(...)` calls + 2 `.setup(...)` calls
added, only within my artifacts' own lines — re-read immediately before each edit since 5 siblings
edit the same file concurrently). Nothing inside any `⚙️engine/` directory was moved, pruned, or
renamed — only additive `declaration()`/helper fns at artifact roots, matching the "add only, move
nothing" constraint. `🧬️mutations/**` untouched throughout.
