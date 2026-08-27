# PDF 1.4 Direct Law Runtime Triage

## Observed Failure Boundary

The retained STDIO continuation contains nine failed `language_neutral_forward_and_concrete_inverse` tests in PDF 1.4 Any/A/X. The first observed failing assertions compare serialized Rust `f64` geometry with language-neutral integer JSON literals, for example `792.0` against `792`. This is a representation mismatch in `serde_json::Value`, not evidence that all later assertions pass. The inverse, text/binary and JSON round-trip assertions after those panics were not reached.

The fixture roots exist at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/<subset>/🧬️schema/🧬️mutations/`:

| Subset | Leaf directories |
| --- | --- |
| `✳️any` | `📥️insert-page`, `🔀️move-page`, `🗑️remove-page`, `📝️replace-page-text`, `📐️resize-page` |
| `✳️a` | `🧹️clear-page-text`, `📝️set-page-text` |
| `✳️x` | `📉️collapse-page-size`, `📐️set-page-size` |

Every current source is `🦀️component.rs`; each law reads its direct `🧪️tests/round-trips-the-concrete-inverse/🔣️component.json`. These historical primary filenames still require the separate canonical taxonomy cutover.

## Repair Boundary

Keep authored JSON fixtures and production geometry unchanged. Compare expected snapshots and inverse mutations after deserializing them into their exact production types, retaining the independent JSON Schema/Ajv fixture validation and existing strict payload-field tests. This preserves integer index precision while handling the explicitly floating-point page model correctly. An additional structural JSON representation check must not silently discard undeclared fields.

The existing store `json_values_equal` helper converts every number to `f64`; it is unsuitable as a new general-purpose proof for arbitrary mutation indices because distinct large integers can collapse. Do not broaden it or weaken global assertions for this packet. Re-run all nine original forward/inverse laws and their later codec assertions against rebuilt production sources. Any further failure is a new observed boundary to fix, not an excuse to skip the law.

## Evidence

- Runtime transcript: `🧪️stdio-library-runtime-continuation/🧪️root-runtime-retry.log`, PDF 1.4 failures starting at line700.
- Geometry source: `✳️any/🧬️schema/📸️snapshot/🦀️component.rs`; `PageDoc.width` and `.height` are `f64`, `PdfSnapshot` derives `PartialEq`.
- Comparator inspected: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`, `json_values_equal` and `json_value_to_dsl`.

No production source was changed by this triage. Runtime repair remains queued behind the current disjoint foundation write lanes.
