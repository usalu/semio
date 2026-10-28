# IFC4x3 Rust Host Audit

Read-only source audit on 2026-10-21. No source, build-system, AGENTS or Git changes were made by this audit. The parent owns fixes. Paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc`; editor paths are under `🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor`.

The cached repo binary's help confirms an old CLI without a general process/source-read command. Read-only PowerShell/rg were used as the existing ticket's documented fallback. Root and ancestor AGENTS, details-implementation.md and ui-audit.md were read. Full compilation remains blocked by the recorded compiler ingress, so the audit does not establish type-check or runtime success.

## Findings

1. **P1: Complex mutations bypass the shared optional-field resolver.** `editor/🦀️.rs` snapshot_edit_special, originally lines 237, 244 and 246, resolves directly through `EDIT_RULES.resolve`. Shared IFC4 `🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs:36` adds `complex_edit` specifically to handle omitted empty complex lists. Normalize the event first, then invoke complex_edit before ordinary resolution. The parent confirmed and is fixing this with tests.

2. **P1: The primary constituent of a complex record uses inherited positions.** `editor/details.rs:115` selects direct attributes only for additional constituents (`start == 5`), while a primary complex record also stores a partial attribute record. This mislabels and mistypes primary attributes whenever an ancestor has explicit attributes. Use direct attributes when the primary entity has complex constituents too; adjust type-switch preservation to the same layout. The parent confirmed and is fixing this with tests.

3. **P1: The complex insertion surface is inaccessible on ordinary entities.** Shared snapshot `📸️snapshot/🦀️.rs:129` omits empty complex vectors during serialization. The provider delegates entity object child_count/object_key and value to the raw provider; `details.rs:408` enables insertion only once already at `/entities/N/complex`. There is no missing-property projection, and insertion on the entity object itself is disabled. An ordinary/new entity therefore has no complex row from which to add its first constituent. Project an empty virtual complex array plus an entity object key (with consistent child_count/value), or expose an allowed missing-property template on the entity object. Resolving commands alone does not fix reachability. Add a host render/control test starting with an empty complex vector.

4. **P2: Optionality leaks from an aggregate attribute to mandatory nested elements.** `details.rs:230` offers the unset variant whenever the enclosing attribute is optional, even after context descended into a mandatory aggregate element or a typed payload. Conversely it ignores current `expression.optional`, so an OPTIONAL ARRAY item on a required attribute has no unset alternative. Use the same root-attribute scope as accepts_reference (`details.rs:54`), together with current expression optionality. Test an optional LIST with required elements and an ARRAY OF OPTIONAL item separately. The existing general SetValue resolver accepts the offered Unset transport; reference-specific validation does not protect scalar aggregate items.

5. **P2: SET/UNIQUE aggregate insertion stops after the default candidate is used.** `details.rs:434-438` tries exactly one item template and returns None when that template already exists. `allows_collection_insert:416` then disables insertion even below the upper bound and when many legal values or reference targets remain. Example: SET OF entity references, containing the first admissible target, cannot append a second different target using controls. Provide an explicit candidate chooser or search unused admissible candidates; do not falsely advertise full aggregate editing based on a single fixed default. Source replacement remains an escape hatch but does not provide the requested field control.

## Contracts Checked

- SnapshotDetailsProvider method signatures, SnapshotDetailValue variants and capability fields match the current shared detail-model contract; no definite trait-method compile mismatch was found statically.
- Current framework detail-tree-and-window renders provider variants on object/array nodes, dispatches indexed variant rows, uses provider child_count/object_key, and emits normalized snapshot-edit action bindings. The provider's tagged IfcValue objects therefore reach variant controls without exposing the ordinary argument `kind` field.
- Exact reference controls convert decimal strings to positive u64 before domain mutations. The current virtual `reference` field covers existing and unset entity-valued slots; direct payload handling also preserves u64. No f64 conversion was found on that path. Context depth is capped and recursion helpers have limits; no obvious infinite recursion was found.
- Root document codec registration, IFC composition editor contribution, package dependency and IFC4x3 document declaration exist. Natural IFC4x3 IO registration is a separately assigned concurrent task, not re-audited here.
- Revisioned host details rendering and centralized snapshot publication are reused. Existing tests cover ordinary nested scalar edits and an already-present complex constituent, but do not cover findings 3-5. Browser accessibility, execution/cancellation bounds, and compiler acceptance remain unverified.

## Scope

This audit excludes full EXPRESS WHERE/UNIQUE/inverse/MVD conformance and the concurrently changing shared Binary/SQLite implementation. Findings describe the source observed before the parent's fixes; rerun focused host tests through authentic compiler ingress when available.
