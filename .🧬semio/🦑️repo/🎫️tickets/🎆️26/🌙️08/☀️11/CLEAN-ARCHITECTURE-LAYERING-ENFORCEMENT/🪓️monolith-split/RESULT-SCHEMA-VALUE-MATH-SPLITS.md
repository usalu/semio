# Schema, Value and Math Splits (WP-B), slice SCHEMA-VALUE-MATH-SPLITS, prefix `svm`

Executor: Sonnet. Tools: `svm-tools/` (`split.py`, `split_projection.py`, `oracle.py`, `prune.py`, `sens.py`, `verify.py`, `itemdiff.py`, `plans/*.py`, `derive-corpus/`). Pristine snapshots of the six monoliths: `svm-tools/orig/`. Logs: `🗑️generated/svm-*`. Sandboxes: `🗑️generated/svm-work/`.

## 1. Verdict

All six targets are split and installed. Every verification that can run without a compile passes. **No compile or test was run**: the compile mutex was acquired twice (13:03 and 13:52) and the launcher refused both times with `Insufficient physical compiler initial headroom` (`availableBytes` 4,262,842,368 against 4,294,967,296 required; `svm-cc-probe1.log`, `svm-cc-probe2.log`). Everything below the line "static" is therefore static evidence, and the derive token-stream diff is proven by item identity plus a prepared differential harness, not by executing the expansion (section 4).

| Target (`🧰️framework/🔨️modules/…`) | Before | After: parent + children | Widened to `pub(super)` |
| --- | ---: | --- | ---: |
| `🧬️schema/✅️validator/🦀️.rs` | 1,780 | 422 + `🔨️compile` 522, `🔎️validate` 306, `📃️string-format` 211, `🔤️pattern` 347 | 20 |
| `🧬️schema/📽️projection/🦀️.rs` | 2,264 | 66 + `🧾️rows-a-h` 758, `🎟️rows-h-t` 767, `📖️rows-t-z` 716 | 0 |
| `🌱️value/✨️derive/⚙️expansion/🦀️.rs` | 2,356 | 732 + `🧭️typed-path` 858, `🎯️expand-entries` 574, `🎚️controlled-output` 217 | 9 |
| `🌱️value/💾️resident/🦀️.rs` | 1,712 | 934 + `🪪️consumer-cell` 794 | 37 |
| `🧮️math/🎯️sampling/🎛️processors/🦀️.rs` | 1,797 | 160 + `🌡️warpers` 431, `⚖️penalties` 543, `🎚️logit-controls` 275, `🎯️selection` 214, `🌀️adaptive` 231 | 4 |
| `🧮️math/🎯️sampling/🚧️constraints/🦀️.rs` | 1,599 | 94 + `🚦️stop-conditions` 175, `🤖️regex-automata` 525, `🧱️mask-constraints` 837 | 16 |

Largest child 858 lines, largest parent 934. Longest new path 113 bytes (limit 240). The public paths are unchanged: the sets of `pub` and `pub(crate)` top-level items are identical before and after for all six (`svm-tools/itemdiff.py` plus a pub-set comparison); the parents re-export children with `pub use <child>::*` where a child has `pub` items.

## 2. How each file was cut

Method: line-range cuts on the pristine snapshot by `svm-tools/split.py` (content-anchored: every item must lie wholly in one file, asserted). The tool regenerates imports per file (original `use` text is kept verbatim when a statement is used whole, otherwise trimmed), re-emits `//#region` markers balanced in every file, adds the `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `use <snake>::*;` mount block, and widens only declarations that another file references (receiver-aware scan: `x.field`, `self.field` per impl type, `Type::name`, struct literals; private types named in a widened signature or field are widened too). Children import siblings with explicit `use super::name;`. The parent keeps what children can reach without any widening, because a child sees its ancestors' private items.

- **validator**: the four big regions leave the parent (Compile, Validate, Format, Pattern). Control, Validator and Equality stay.
- **projection**: a pure table. `TYPES` (172 rows) is cut at row boundaries into three `pub(super) const ROWS` tables; the parent concatenates them in a `const fn concat<const N: usize>` and keeps `SchemaMetadata`, `validate` and `render_typescript`. Row lines are byte-identical, including the raw-string TypeScript (nothing was dedented).
- **expansion**: the attribute model, naming and field plans (Case, Attrs, StructPlan, VariantFields) stay in the parent, so only 9 body functions needed widening. `TypedPath`, `Expand` and the controlled helpers become children. `declared_value_crate`, `expand_canonical_tree` and `mod canonical_tree` stay in the parent because `retained-clone`, `factory` and `canonical` reach them as `crate::component::…` and `use super::*`.
- **resident**: an unsafe, tightly coupled ledger. A three-way region split needed 107 widened declarations; a four-way 65. The kept cut is `ConsumerCell` as the only child (37 widenings, each proven necessary by the oracle, see section 3.4); `OriginalRoot` and `TypedRecord` stay with the base vocabulary in the parent (934 lines).
- **processors / constraints**: traits, both `dyn_enum_close!` enums and the `build_*` functions stay in the parent, because `#[dyn_enum]` emits a textually scoped `macro_rules!` that `dyn_enum_close!` must see in the same module. Region markers for the split regions are repeated balanced in parent and child.
- Test-only imports that only the `#[cfg(test)] mod tests` reached through `use super::*` became `#[cfg(test)] use …` lines in the parent so non-test builds gain no unused-import warnings.

## 3. Static verification (Wave 6)

### 3.1 Line multiset lossless (`svm-tools/verify.py`, logs `🗑️generated/svm-<name>-verify.log`)

| Target | Lines removed from original (distinct) | Lines added |
| --- | --- | --- |
| validator | 0 | 24 (docstrings, mount lines, regenerated imports) |
| projection | 1 (`pub const TYPES … = &[`) | 35 (3 doc, 3 mount, `ROW_COUNT`, `concat`, three table heads) |
| expansion | 0 | 21 |
| resident | 0 | 14 |
| processors | 3 (the three original `use` statements, now trimmed per file) | 51 |
| constraints | 2 (the same, trimmed) | 28 |

Added lines are only docstrings, mount lines, regenerated `use` lines and (for the split regions) duplicated region markers. Nothing else differs apart from the `pub(super) ` prefix.

### 3.2 Item identity

`svm-tools/itemdiff.py` hashes every top-level item (fn, struct, enum, impl, trait, const, macro call), ignoring region markers and the `pub(super) ` prefix, and compares the multisets. Result: identical for validator (89 items), constraints (66), processors (100), resident (80) and expansion (59). Projection: the concatenation of the three child tables equals the original row block line by line (2,223 lines, `IDENTICAL SEQUENCE`), and `validate` / `render_typescript` are unchanged text.

### 3.3 Scoped patched differential (`svm-tools/oracle.py`, rules of `📓️RESOLUTION-ORACLE.md`)

`rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null --cap-lints allow`, sandbox A = pristine monolith, sandbox B = the installed tree (`--installed`), both with the extern-crate and external-glob patches. Errors about unresolved external crates and attributes are dropped from both sides (their count changes with the number of `use` lines). Run on the installed tree:

| Crate | cfgs | baseline errors = split errors | Verdict |
| --- | --- | --- | --- |
| validator | lib, test | 14, 28 | IDENTICAL |
| projection (synthetic root `svm-root/🦀️.rs` mounting the module and its test; crate `semio_framework`) | lib, test, wasm32-unknown-unknown, wasm32-wasip2 | 1 each (only the `/dev/rmeta` write error) | IDENTICAL |
| expansion (crate type `proc-macro`) | lib, test | 393, 433 | IDENTICAL |
| resident | lib, test, wasm, p2 | 1, 6, 1, 1 | IDENTICAL |
| processors | lib, test, wasm, p2 | 42, 97, 42, 42 | IDENTICAL |
| constraints | lib, test, wasm, p2 | 13, 42, 13, 13 | IDENTICAL |

Fidelity, stated honestly:
- `resident` and `projection` depend only on `std`, so typeck, privacy and const evaluation run in full. Proof of sensitivity: removing the `pub(super)` of `ConsumerHeader.aliases` gives `E0616`, and shortening `ROW_COUNT` gives `E0080 index out of bounds` at `schema_metadata::TYPES`. So `TYPES` and `concat` evaluate without error in the split tree.
- The other four crates import crates that are absent from the sandbox, which suppresses most typeck. Resolution and item privacy (`E0425`, `E0432`, `E0433`, `E0603`) are still checked; field and method privacy only in bodies that are not tainted. `svm-tools/sens.py` removes each added `pub(super)` in turn: validator 19 of 20 are detected as necessary by the oracle, expansion 9 of 9. The undetected ones (validator `Scope.base`, 11 of 16 in constraints, 4 of 4 in processors) are covered only by the receiver-aware reference scan and, for processors and constraints, by the test files that name them (`penalty.counts`, `mirostat.mu`, `cache.entries`, `ac.step`, `validates_json_schema`, `constraint.mode`). This is the residual risk for the real check.
- One defect found by the oracle during the work and fixed before install: a child using a type through `use super::*` of the parent (`Data` in `canonical`); the splitter treated every `#[path]` descendant as test-only. It now separates production descendants from `#[cfg(test)]` ones.

### 3.4 Minimal widening on `resident` (`svm-tools/prune.py`)

Because the oracle is exact for `resident`, every auto-widened declaration was tried removed, one by one, against the lib and test baseline. Removable ones were dropped (e.g. `ConsumerHeader.type_id`, `ConsumerHeader.registration`, `ConsumerNode.source`, `ResidentConsumer.pointer`); the 37 left each produce a real error when removed (`🗑️generated/svm-resident-prune.json`). Type privacy forced the structs named by widened fields to be widened too (`error: type … is private`).

### 3.5 Other static checks

- rustfmt parse (`rustfmt +nightly-2026-07-20 --check --config skip_children=true`): every new and changed file parses, no `error:`.
- `#[path]` resolution: every mount target exists (checked by `verify.py`, including `../🧵️canonical/🦀️.rs` and the test mounts).
- Regions: balanced in every file. Where each original region lives: validator `Control`, `Validator`, `Equality` parent, `Compile` / `Validate` / `Format` / `Pattern` children; processors and constraints repeat `Warpers`, `Selection`, `Adaptive`, `Stops`, `Constraints` in parent and child; resident `ConsumerCell` child, the rest parent; expansion `TypedPath` and `Expand` children, the rest parent.
- Unused-import scan (`verify.py`): none besides names kept for `use super::*` consumers (`Data` for `canonical`; `cfg(test)` groups for the unit tests).
- Duplicate-mount scan: each child mounted exactly once, and no other crate mounts any of the six files (`git grep` for the six paths: only docs, fixtures and the one facade mount for projection).

## 4. Derive expansion: token-stream identity

What was proven (all static):
1. Every one of the 59 top-level items of the original expansion (all `quote!`-carrying functions included) reappears byte-identical (only `pub(super) ` added on 9 functions). Since the functions are unchanged and every call resolves to the same definition (oracle, section 3.3), the emitted `TokenStream` of every input is the same.
2. Paths emitted by the expansion and by `canonical`: `quote!` blocks carry only these path roots: `#value_crate` (304 uses; default `::semio_framework_value`, overridable by `#[value(crate = "…")]`), `#c` and `#owner` (the same value-crate path, passed in), `::core`, `::std`, and `::semio_framework_value`. There is no kernel alias (`os_kernel`, `store`, `dsl`, `spr`, `semio_format`, `dsl_grammar`) anywhere in the expansion or `canonical`. `git grep` of `semio_` in both files finds `semio_framework_value`, one doc mention of `semio_framework_value_derive` and the hygiene prefix `semio_value_field_`. The `DslArtifact` path-root fix of `RESULT-OS-KERNEL-ENGINE.md` lives in the `dsl` derive, a different crate, and is untouched.
3. The only other crate-level coupling stays valid: `crate::component::declared_value_crate`, `component::expand_to_value`, `expand_from_value`, `expand_canonical_tree` (all still reachable by the same paths), and `canonical`'s `use super::*`.

What was prepared but could not be executed: `svm-tools/derive-corpus/` is an in-process differential. `before/🦀️.rs` is the pristine expansion (only the `#[path]` to `canonical` re-pointed), `🦀️.rs` is a test that parses every `ToValue` / `FromValue` / `CanonicalJsonTree` input from `corpus-files.txt` (the derive crate's own test files and their neighbours plus the 387 files in `🧰️framework` that use the derives) and asserts the string form of `before::expand_*` equals `crate::component::expand_*` for `to_value`, `from_value`, `canonical_tree` and `declared_value_crate`, optionally dumping both streams to `$SVM_CORPUS_OUT.{before,after}.txt`. Compile-queue run:

```
# temporarily add to 🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs (remove afterwards, prefix [DEBUG]):
#[cfg(test)] #[path = "../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/svm-tools/derive-corpus/🦀️.rs"] mod svm_corpus;
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' SVM_CORPUS_OUT=<ticket>/🗑️generated/svm-derive-corpus perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- test -p semio-framework-value-derive --manifest-path 🧰️framework/Cargo.toml --lib svm_split_expansion -- --nocapture
```

Expected: `[DEBUG] svm corpus inputs=… compared token streams=…`, test passes, `diff svm-derive-corpus.before.txt svm-derive-corpus.after.txt` empty.

## 5. Source-text readers and neighbours re-pointed

- `🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts` read the expansion file text for three strings (`container.crate_path…`, `#value_crate::ToValue`, `#value_crate::FromValue`), which now live in different files. It reads the whole `readRustSourceClosureV1` closure of the expansion instead. `bun test ./…/🧱️owners/🟦️.ts`: 7 pass, 0 fail, 4,527 expects (includes the changed test).
- `💾️resident/📦️packages/🦀️rust/📋️project.json` and the fixture `🧪️tests/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` (which must equal the project file) each gained the `🪪️consumer-cell/🦀️.rs` named input so the Nx hasher sees the child; the fixture is valid JSON.
- Doc comments pointing at `ASSERTED_STRING_FORMATS` in `✅️validator/🦀️.rs` now name `✅️validator/📃️string-format/🦀️.rs` (`🔮️oracles/✅️validator/🟦️.ts`, `🧪️tests/✅️draft07-oracle/🟦️.ts`). `framework-source-topology` and `framework-root-source-topology` fixtures stay valid: their anchors (`SchemaValidator`, `pub struct SchemaMetadata`) remain in the parents.

## 6. P11 hygiene edges (`framework-schema`, `schema-validator-*`, `value-*`)

Command (the `DEPENDENCY-DIRECTION-AUDIT-2.md` section 1 run, whole tree, run at 13:41 before any TS edit of mine): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (exit 35, 13,347 error lines, `🗑️generated/svm-depcruise-before.txt`).

| Rule | Edges now (13:41) | of which outside `🧪️tests` | dda2 snapshot 04:59 (all / prod) |
| --- | ---: | ---: | --- |
| `no-cross-package-relative-@semio-tech/framework-schema` | 102 | 36 | 259 / 104 |
| `no-cross-package-relative-@semio-tech/schema-validator-ts` | 238 | 161 | 228 / 152 |
| `no-cross-package-relative-@semio-tech/schema-validator-rs` | 238 | 161 | 228 / 152 |
| `no-cross-package-relative-@semio-tech/value-ts` | 146 | 49 | 210 / 84 |
| `no-cross-package-relative-@semio-tech/value-rs` | 146 | 49 | 210 / 84 |

The `-ts` and `-rs` rules of one package count the same edges twice (both packages share the owner directory), so the distinct production edges are 36 + 161 + 49 = 246. The drift against 04:59 is peer work (other slices moved tests and importers).

**After: unchanged, 0 edges fixed.** What was measured and why:
- Edges whose importer lies in the six touched module directories: **0** in all five rules. Importers in the wider `🧬️schema`, `🌱️value`, `🧮️math` trees: 9 (schema oracle to the validator `🏷️format-policy` JSON, the schema package script to `✅️validator/🧪️tests/🟦️.ts`, a composition test to `🧬️schema/🟦️.ts`, four `🌱️value` files to `🧬️schema/🧾️record/🟦️.ts`). Edges whose target lies in the touched directories: 531, all from outside (the validator TS module alone has about 226 importers: 124 in `repo/📚️library`, 10 in the os plugin tree, 8 in `🏃️process`, 19 in `📓️print` and so on).
- The rule resolves export targets relative to the package directory (`exports["."] = "./🟦️.ts"` means `✅️validator/📦️packages/🟦️typescript/🟦️.ts`, a two-line barrel), and an export target may not contain `..`. So the validator's real module file `✅️validator/🟦️.ts` can never become an export target; the only fix is for each importer to use `@semio-tech/schema-validator-ts` (already linked in `node_modules/@semio-tech`, already used by a few files) or the barrel. Doing that for the 226 importers is a cross-slice mass rewrite (the importers sit in `repo`, `os`, `print`, `process`, `hub`), and it would trade one finding for another: each importing package must also list the dependency or `not-to-unlisted` grows by the same amount, which needs `package.json`, `bun.lock` and `bun install` changes. The value-ts targets (33 files: `🛬️decode`, retained-clone wire schemas, resident, `🧬️schema`) have the same shape, and the `framework-schema` package exports only `./📜️script.ts` from a `🦀️rust` package directory, where a thin TS barrel would breach the package-language purity rule. I judged none of these clean inside the touched modules, so no import or `exports` entry was changed. I did not edit the policy.
- Recipe for the owners, in order: (1) complete the validator barrel with `export * from "../../🟦️.ts";` (it exports 4 of the 11 names importers use: `compileJsonSchemaSubset` 10 importers, `requireString` 4, `requireStringArray` 4, `requireLiteral` 2 are missing); (2) rewrite the importers to `@semio-tech/schema-validator-ts` and add the dependency to each importing package; (3) add thin barrels plus `exports` for `value-ts` (`./resident`, `./decode`, `./schema`, `./retained-clone/wire`) and re-point the 20 `💾️resident/🟦️.ts` importers (8 in the renderer, 3 in actor instance, 2 in plugin); (4) give `framework-schema` a TS package for `./record`, `./artifact-reference` and the oracles.
- My TS edits (one test line and two doc comments) cannot change an edge. Check: scoped cruiser over those 3 files after the edit, `svm-depcruise-scoped-after.txt` (exit 36 for the unrelated rules): their 10 error edges are identical to the 10 in the 13:41 run. A whole-tree "after" run was attempted three times and each aborted with `ENOENT` on a file a peer deleted mid-run (`svm-depcruise-after.err`); the per-file comparison above is the after evidence.


## 7. Real-check commands for the compile queue

Run each under the fleet mutex, one launcher call, `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- <cmd>`:

```
check -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints -p semio-framework-value-derive --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
check -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
check -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --lib --target wasm32-unknown-unknown --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
check -p semio-framework --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
test -p semio-framework --lib exports_typescript_bindings --manifest-path 🧰️framework/Cargo.toml
test -p semio-framework-schema-validator -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --manifest-path 🧰️framework/Cargo.toml
test -p semio-framework-value-derive --manifest-path 🧰️framework/Cargo.toml        (the derive's integration tests: newtype_transparent, variant_field_casing, deny_unknown_fields_enums, typed_path, flatten_with_skip, owned_json, generic_retained_owner)
```

Plus the derive corpus run of section 4. Dependents need no re-check for paths (public paths unchanged), but the crates that derive `ToValue` / `FromValue` (e.g. `semio-framework-ui-contract-*`, `semio-framework-os-*`) exercise the new expansion in any workspace check. Things most likely to fail on the first real compile, in order: a missing `pub(super)` on a field or method that only an untainted-body oracle could see (validator `Scope`, constraints `Frag` / `NfaNode`, processors `counts` / `mu`); a trait method call that needs a trait import in a child (`use super::{LogitsProcessor,…}` is added by identifier use only); the `ambiguous_glob_reexports` or `unused_imports` warnings on the `use <child>::*;` lines.

## 8. Hand-offs

- **Taxonomy sweep**: register these new child folder names (row `crate-module-children`, see `RESULT-TAXONOMY-SWEEP.md`): `🔨️compile`, `🔎️validate`, `📃️string-format`, `🔤️pattern`, `🧾️rows-a-h`, `🎟️rows-h-t`, `📖️rows-t-z`, `🧭️typed-path`, `🎯️expand-entries`, `🎚️controlled-output`, `🪪️consumer-cell`, `🌡️warpers`, `⚖️penalties`, `🎚️logit-controls`, `🎯️selection`, `🌀️adaptive`, `🚦️stop-conditions`, `🤖️regex-automata`, `🧱️mask-constraints`. `verify taxonomy` was not run.
- **Compile window**: section 7.
- **Headroom**: `⚡️cache/rust/fixed/m` is 32 MB short of the 4 GiB initial headroom; nothing inside it was touched.

## 9. Files touched

Created (19 child modules): `🧬️schema/✅️validator/{🔨️compile,🔎️validate,📃️string-format,🔤️pattern}/🦀️.rs`; `🧬️schema/📽️projection/{🧾️rows-a-h,🎟️rows-h-t,📖️rows-t-z}/🦀️.rs`; `🌱️value/✨️derive/⚙️expansion/{🧭️typed-path,🎯️expand-entries,🎚️controlled-output}/🦀️.rs`; `🌱️value/💾️resident/🪪️consumer-cell/🦀️.rs`; `🧮️math/🎯️sampling/🎛️processors/{🌡️warpers,⚖️penalties,🎚️logit-controls,🎯️selection,🌀️adaptive}/🦀️.rs`; `🧮️math/🎯️sampling/🚧️constraints/{🚦️stop-conditions,🤖️regex-automata,🧱️mask-constraints}/🦀️.rs` (all under `🧰️framework/🔨️modules/`).
Rewritten parents: the six target files.
Edited: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts`, `🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json`, `🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json`, `🧬️schema/🔮️oracles/✅️validator/🟦️.ts` (doc comment), `🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts` (doc comment).
No `AGENTS.md`, policy, taxonomy, `launch.json` or ticket tool was touched; no modifying git command was run.
