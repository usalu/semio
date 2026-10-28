# IFC Core Audit

## Scope and Method

Read-only inspection of IFC artifact and applicable root, s, and stdio AGENTS instructions. The available repo CLI does not expose general file read/search execution, so PowerShell and rg were used for necessary source inspection; no builds, git mutations, worktrees, or source edits were performed.

## Findings

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📜️artifact-definition.json` registers only `4` and `2x3`; normative sources/checksums are absent and both standards are unverified. No IFC4x3 tree or official IFC4x3 fixture exists.
- Root `🦀️.rs` registers the same two standards and existing document codecs. IFC4X3 mentions are mutation test strings, not a supported schema.
- Under `🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/`, Rust and TS snapshots retain a generic STEP graph: entity id/name/positional args/complex components, three raw header tuples, recursive values. They contain no EXPRESS entity declarations, attribute metadata, subtype/select/enum validation, or IFC4x3 semantics.
- Artifact-level schema JSON still declares `schema` and `text` despite actual snapshot using header/entities; schema consistency must be corrected in the requested new standard.
- Rust text deserializer at `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs` invokes shared Part21 parsing and converts to its own snapshot. It does not check FILE_SCHEMA. Its TS peer is `export {};`.
- Existing Rust sparse diff validates entity ids and argument indices before central `MutationDiff::apply`; supports absorb/inverse and header changes. Positional values are replaced atomically; complex components are whole replacements. Nine mutation kinds cover generic entity/arg and header edits. No typed semantic editing or recursive reference validation occurs.
- Public helper apply methods on argument/entity collections are unchecked; only the central MutationDiff entry validates. New interfaces should funnel all writes through central apply.
- Existing TS snapshot/diff/mutation exports describe shapes/guards without executable parsing, serialization, applying, inverse, or mutation generation.
- Existing IFC4 mutation oracle uses ruststep as grammar reader plus its own writer, explicitly not a third-party differential producer. It includes a sentinel workaround for ruststep apostrophe parsing. Current real fixture is Nakagin Capsule Tower, not official IFC4x3.

## Proposed Ownership and Interfaces

Create sibling `🏅️standards/🔖️4x3/🪆️subsets/✳️any/` rather than reinterpret IFC4 or change IFC2x3. Proposed compact files within new standard: `🧬️schema/📚️catalog/🔣️.json` normative catalog; `🧬️schema/📚️catalog/🦀️.rs` reader and entity/attribute lookup; `🧬️schema/📸️snapshot/🦀️.rs` own graph model; `🧬️schema/🔺️diff/🦀️.rs` checked central apply/inverse; `🚪️io/📝️text/🦀️.rs` parse/write boundary; `🧪️tests/🥒️.feature` language-neutral cases. Avoid deep per-mutation tree duplication to respect path length.

Suggested shared contract: catalog JSON declares schema identifier `IFC4X3_ADD2`, entity name, abstract flag, supertype, flattened explicit attribute names/types/optional/derived, enum/select/type records. Rust `validate_snapshot(&IfcSnapshot) -> Vec<IfcDiagnostic>` validates header, ids, references and catalog attributes; `parse_step(&str) -> Result<IfcSnapshot, IfcError>` and `write_step(&IfcSnapshot) -> Result<String, IfcError>` retain own model; `apply_diff(&IfcSnapshot, &IfcDiff) -> Result<IfcSnapshot, IfcError>` is the sole write authority; `inverse_diff(&IfcSnapshot, &IfcDiff)` uses original base. Main can mirror exact JSON shapes for TS and editor.

No test results claimed; implementation remains pending.
