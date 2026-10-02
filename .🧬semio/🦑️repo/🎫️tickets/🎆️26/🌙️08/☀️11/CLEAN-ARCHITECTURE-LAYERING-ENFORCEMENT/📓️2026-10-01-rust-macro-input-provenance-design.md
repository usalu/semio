# Rust Macro Compile-Input Provenance Design

Read-only source review, 2026-10-01. No tests, compiler, generation or production edits. This report describes required proofs, not passing results. Paths are repository-relative.

## Current scanner and immediate correction

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:6462–6553` scans include macros even inside token templates and fails closed at 6509 on nonliteral expressions. The current shared source additionally recognizes simple single-arm macro_rules and gathers same-file post-definition invocations. At 6488 it permits literal/expr/tt matchers only; at 6500 it tries to evaluate **every** argument as a string path. This fails both real finite examples: PDF has an ident name plus closure expr, Home has a boolean expr. Resolve only the metavariable dependency closure of the compile-input expression. Validate other matcher arguments structurally without pretending they are paths. Preserve unsupported-source diagnostics until every include template has a closed proof.

Current template implementation is not a full Rust expander. Repeated macro names are rejected; qualified invocations are rejected; no invocation yields unsupported rather than a fictitious empty success. Keep these conservative outcomes unless schema/native oracle cases explicitly extend the contract. Scope shadowing, nested definitions, exported/cross-file calls, repetition, recursion, multiple arms, cfg alternatives and transcriber-created include calls must not silently become empty inventories. Includes inside generated macro definitions need the correct source/span origin, proved by rustc rather than inferred from lexical proximity.

## Finite local macro_rules facts

PDF owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors/🦀️.rs`. Lines 95–100 define `lopdf_vector!($name:ident,$directory:literal,$derive:expr)` with two include_bytes/concat templates. All 44 authored invocations at 106–171 bind directory string literals; each produces before and after PDF inputs, for 88 concrete compile references. Closure bodies and test identifiers affect generated test behavior, not input paths. Relative target base is the actual test source owner, not Cargo manifest or the PDF generator owner. Existing law reads independent lopdf 0.44 before/after assets, derives mutation payload, applies and encodes/decodes forward output, then applies inverse steps and writes back. Keep this exact corpus and original test functions; scanner support is no reason to weaken the runtime oracle.

Concrete directory/test bindings:

- set_page_box → `🖼️set-page-box/{⬅️before.pdf,➡️after.pdf}`
- set_page_user_unit → `📏️set-page-user-unit/{⬅️before.pdf,➡️after.pdf}`
- insert_content → `🖋️insert-content/{⬅️before.pdf,➡️after.pdf}`
- remove_content → `🧻️remove-content/{⬅️before.pdf,➡️after.pdf}`
- replace_content → `🔁️replace-content/{⬅️before.pdf,➡️after.pdf}`
- insert_annotation → `📌️insert-annotation/{⬅️before.pdf,➡️after.pdf}`
- remove_annotation → `📍️remove-annotation/{⬅️before.pdf,➡️after.pdf}`
- set_annotation → `📝️set-annotation/{⬅️before.pdf,➡️after.pdf}`
- set_font → `🔤️set-font/{⬅️before.pdf,➡️after.pdf}`
- remove_font → `🅾️remove-font/{⬅️before.pdf,➡️after.pdf}`
- set_image → `🏞️set-image/{⬅️before.pdf,➡️after.pdf}`
- remove_image → `🌫️remove-image/{⬅️before.pdf,➡️after.pdf}`
- set_form → `📄️set-form/{⬅️before.pdf,➡️after.pdf}`
- remove_form → `🗞️remove-form/{⬅️before.pdf,➡️after.pdf}`
- set_ext_g_state → `🎛️set-ext-g-state/{⬅️before.pdf,➡️after.pdf}`
- remove_ext_g_state → `🎚️remove-ext-g-state/{⬅️before.pdf,➡️after.pdf}`
- set_shading → `🌅️set-shading/{⬅️before.pdf,➡️after.pdf}`
- remove_shading → `🌄️remove-shading/{⬅️before.pdf,➡️after.pdf}`
- set_pattern → `🧩️set-pattern/{⬅️before.pdf,➡️after.pdf}`
- remove_pattern → `🪡️remove-pattern/{⬅️before.pdf,➡️after.pdf}`
- set_color_space → `🌈️set-color-space/{⬅️before.pdf,➡️after.pdf}`
- remove_color_space → `🎨️remove-color-space/{⬅️before.pdf,➡️after.pdf}`
- set_properties → `🏷️set-properties/{⬅️before.pdf,➡️after.pdf}`
- remove_properties → `🔖️remove-properties/{⬅️before.pdf,➡️after.pdf}`
- set_embedded_file → `📎️set-embedded-file/{⬅️before.pdf,➡️after.pdf}`
- remove_embedded_file → `🗃️remove-embedded-file/{⬅️before.pdf,➡️after.pdf}`
- set_outlines → `📑️set-outlines/{⬅️before.pdf,➡️after.pdf}`
- set_named_destination → `🎯️set-named-destination/{⬅️before.pdf,➡️after.pdf}`
- remove_named_destination → `🎪️remove-named-destination/{⬅️before.pdf,➡️after.pdf}`
- set_page_labels → `🔢️set-page-labels/{⬅️before.pdf,➡️after.pdf}`
- set_output_intents → `🏳️set-output-intents/{⬅️before.pdf,➡️after.pdf}`
- set_acro_form → `📋️set-acro-form/{⬅️before.pdf,➡️after.pdf}`
- set_optional_content → `👁️set-optional-content/{⬅️before.pdf,➡️after.pdf}`
- set_page_layout → `📖️set-page-layout/{⬅️before.pdf,➡️after.pdf}`
- set_page_mode → `🖥️set-page-mode/{⬅️before.pdf,➡️after.pdf}`
- set_viewer_preferences → `🛠️set-viewer-preferences/{⬅️before.pdf,➡️after.pdf}`
- set_open_action → `🚪️set-open-action/{⬅️before.pdf,➡️after.pdf}`
- set_language → `🗣️set-language/{⬅️before.pdf,➡️after.pdf}`
- set_mark_info → `🔏️set-mark-info/{⬅️before.pdf,➡️after.pdf}`
- set_metadata → `🧾️set-metadata/{⬅️before.pdf,➡️after.pdf}`
- set_catalog_entry → `🗂️set-catalog-entry/{⬅️before.pdf,➡️after.pdf}`
- remove_catalog_entry → `🧺️remove-catalog-entry/{⬅️before.pdf,➡️after.pdf}`
- set_document_id → `🆔️set-document-id/{⬅️before.pdf,➡️after.pdf}`
- set_encryption → `🔐️set-encryption/{⬅️before.pdf,➡️after.pdf}`

Home owner: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs`. Lines 18–31 define local committed! with $name:literal and $observable:expr; lines 35–37 invoke it for ✅️apply/true, 🟰️apply/false, 🚫️apply/false. Five include_str templates per invocation resolve to 15 files under `../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/<name>/`: snapshot before, mutation, snapshot after, diff, outcome. These are source-owner fixture inputs. All three match arms compile even if a runtime row chooses only one. Do not census only successful runtime rows. Subject calls the real `home_transient_mutation_report_json`; literal oracle handlers retain before/after snapshots. This case explicitly records a no-oracle decision; its snapshot constants are not an independent third-party proof of Home semantics. Use rustc as the independent oracle for macro expansion/input provenance, and retain the recorded runtime decision rather than mislabeling it a third-party algorithm oracle.

## Procedural quote is a different authority

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:550–600` emits MutationLeaf includes for taxonomy, leaf descriptor and payload schema; 1923 emits the Mutations aggregate taxonomy dependency. The #ident tokens are quote interpolation, not Rust macro_rules metavariables. The derive crate itself does not compile these includes: real consumer derive invocations do.

Leaf expansion obtains the local compiler source from input.ident.span().local_file() (556), computes source authority (558), validates/reads descriptor (559–560), and computes payload schema (566 onward). Authority at 93–122 demands a taxonomy canonical primary under a flat mutation owner or an explicitly registered domain-operation root, rejects mismatched ownership and follows no symlinks. Aggregate authority at 125 onwards demands a canonical collection source and validates component roots. Aggregate expansion 1764–1789 requires nonempty concrete variants, rejects conditional aggregate metadata and emits its taxonomy dependency. Portable absolute paths in generated tokens bind validated consumer authority; they are not derive-source-relative include paths.

Schema already provides MutationSourceAuthorityProjectionV1, MutationLeafDeriveV1 and MandatoryMutationsV1 at derive `🧬️schema/🔣️.json`; source-authority corpus is `🧫️fixtures/🛂️mutation-source-authority/{🔣️.json,🧭️domains.json}`, native source-authority/mandatory/leaf laws and TS authority tests live under `🧪️tests`. Reuse these declared facts to derive finite consumer inputs. A template-only success with no real consumer closure is a blind spot. Do not convert #taxonomy_dependency into an authored literal or special-case this pathname out of scanning.

## Closed schema-first rule

Extend the existing Rust source-direction schema/corpus with a discriminated provenance union:

- Literal input: existing exact expression, raw path/base, source origin, line/module context and ordered required directory prefixes.
- Local finite template input: exact definition source/range/name and matcher arm; include expression origin; the referenced metavariable names; invocation source/range and arguments; substituted path; compile-configuration coverage. Facts must be derived from tokenized authored bytes. Reject unresolved referenced parameters, unsupported referenced expressions, ambiguous binding scopes or unaccounted reachable invocations. Irrelevant parameters are valid token fragments, not fake path values.
- Consumer-derived procedural input: recognized macro identity/export and producer template range; actual consumer source/derive span; validated canonical workspace/taxonomy/owner; exact descriptor/payload lookup inputs; concrete expanded include target; configuration/feature scope. Each producer include site creates an obligation resolved by real consumer invocations or remains an explicit unresolved problem. Producer/template records never themselves authorize filesystem edges.

All concrete inputs feed the existing physical resolver: raw segments and ordered prefixes, symlink refusal, physical leaf existence, exact owner/direction checks, manifest-context restrictions. Do not let template provenance bypass those checks. Include-byte/text edges count separately from module code edges, and each expansion retains its own provenance even when targets deduplicate in the physical inventory. Maintain explicit counters for templates, invocations, expansions, unresolved obligations and physical targets so partial substitution cannot produce a misleading GREEN.

## Genuine proof shape and execution recommendation

First close finite local macros using a language-neutral fixture matrix and tiny native rustc oracle cases: literal directory/name substitution with irrelevant ident/closure/boolean args; all invocations discovered; raw-string literals; distinct include kinds; missing expanded target; symlink-prefix and parent traversal; unbound parameter, repeated/exported/cross-file/recursive/multi-arm forms rejected unless implemented. Native binary prints exact included bytes/text and generated test count; failing native cases establish refusal. A generic third-party parser may validate token ranges behind a test-only interface, but rustc remains semantic expansion authority. No fixture-text-only substitute proves the original macro-generated test bodies.

Then implement procedural consumer provenance as a separate law using existing taxonomy authority fixtures plus a real minimal MutationLeaf and aggregate consumer. Compare emitted dependency facts against rustc dep-info/actual included-byte changes; validate descriptor/payload/taxonomy target edits affect consumer compilation/output, wrong consumer owner and symlink targets fail, and no consumer leaves an unresolved obligation. Consumer census must traverse actual registered packages/features/configurations rather than fabricate consumers under the derive directory. Dep-info alone does not provide ownership authority: compare compiler paths with independently validated consumer scope and authored authority rules.

Execution recommendation: finish the finite macro_rules support first with exact PDF/Home dependency subsets; retain the derive source as RED until its consumer-provenance union and native authority proof land. The three source problems have two different semantic mechanisms and should not be cleared by one exemption.

## Closure argument boundary correction

Current whole-gate census reported by Root after duplicate-attribute closure: 24,082 files, 61,123 references, 86 forbidden edges and four problems. This audit did not run that census.

The PDF expr argument starts with bare `|b, a|` or typed `|b: &PdfSnapshot, a: &PdfSnapshot|`. Plain top-level comma splitting treats the comma in this closure header as an invocation separator. Use an owned, deliberately closed matcher-fragment grammar, not a heuristic that suppresses every comma between arbitrary pipe tokens:

1. At each expected expr argument start, recognize an optional `move` followed by `|...|` (and explicitly test or reject the lexer's `||` zero-argument token). Consume the closure header before recognizing invocation separators. Header grammar sufficient for the real sources is comma-separated identifier/underscore parameters with optional `: &? qualified::Type` annotations. No nesting-independent pipe toggle across the rest of the expression. Unsupported destructuring, generic type punctuation, higher-ranked bounds or return syntax should fail closed until covered by schema/native vectors.
2. After the closing header pipe, consume either one balanced block, or a balanced expression until the next top-level invocation comma. Skip paired (), [], {} delimiters. A block has a definite end, allowing exact trailing-comma/end-of-invocation validation. All real PDF closure bodies are expressions or balanced blocks. Do not confuse inner parameter commas with outer macro commas; add native cases for both typed and untyped headers, closure block versus expression, trailing comma, nested tuple/call, malformed or missing closing pipe.
3. Validate ident fragments as exactly one identifier and literal fragments as valid literal tokens. An irrelevant expr must still be a supported nonempty fragment with exact arity and consumed boundaries; true/false are the real Home case. Once a bound parameter participates in include/concat, it must independently evaluate to an accepted string/base expression; closure, boolean, integer or unknown path input remains rejected. Never turn an irrelevant unparsable fragment into silently discarded source.
4. For these single-arm macros, match fragments in declared order. Parse only the grammar needed by the closed schema, and return unsupported for ambiguity. A general Rust expr parser is a separate extension; tests should not claim one. The last expr cannot simply swallow an arbitrary remainder without checking closure boundaries and permitted trailing tokens: arity mismatches must remain observable.

The closure header rule changes argument boundaries, not path provenance. Every PDF directory literal still expands into exactly two source-relative inputs; every Home name literal into five. Producer quote interpolation remains a separate consumer-authority obligation.
