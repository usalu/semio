# Nine Path Law Original Inputs

These complete original authored input bytes were freshly read immediately before the exact canonical binding replacement. Each source is retained verbatim in its fence, including its trailing newline; SHA checks apply to the source body only. No historical unavailable witness is invented.

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌀️mutate-procedural-2d-1/🦀️.rs

SHA-256 d403760e4fe7b6b02e29b34873488285c0a0f51cc175e019aa54d66070acbd61

```rust
//! 🦀️ Generation2d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `procedural-2d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Generation2dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Generation2dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `procedural-2d-1-any` catalog.
const KINDS: &[&str] = &[
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Generation2dDiff` mirrors `Generation2dSnapshot`'s two fields name for name, so the table is empty; the sibling `🀄️wfc` subset, whose diff splits every collection into a `<name>Removed`/`<name>Upserted` pair, carries real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Generation2dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-widget` → `CreateWidget`, the Rust variant name this subset's EXTERNALLY tagged enum writes as the
/// payload's sole object key.
fn discriminant(kind: &str) -> String {
    kind.split('-').map(|word| format!("{}{}", word[..1].to_uppercase(), &word[1..])).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags EXTERNALLY, so the
/// discriminant is the payload's single member name and a payload of any other arity is malformed.
fn declared_kind(mutation: &Json) -> String {
    match mutation {
        Json::Object(entries) if entries.len() == 1 => entries[0].0.clone(),
        _ => String::new(),
    }
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the two-widget graph with its two-generation history, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/➖delete-generation/removes-the-selected-generation-2-and-falls-back-to-generation-1/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    let fixture_widgets = reparsed.get("fixture").map(|fixture| fixture.array("widgets").len()).unwrap_or(0);
    let fixture_synapses = reparsed.get("fixture").map(|fixture| fixture.array("synapses").len()).unwrap_or(0);
    let generations = reparsed.get("generation").map(|generation| generation.array("generations").len()).unwrap_or(0);
    if fixture_widgets < 2 || fixture_synapses == 0 || generations < 2 {
        return Err(format!("the committed round-trip snapshot is the two-widget, one-synapse graph with a two-generation history this scenario describes, but it carries {fixture_widgets}/{fixture_synapses}/{generations}"));
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-procedural-3d-1/🦀️.rs

SHA-256 165ffe22fb9e09fa63b4a5b970350e6e7e446da11d33aadc6acd2d75fb739b66

```rust
//! 🦀️ Generation3d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `procedural-3d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Generation3dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Generation3dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `procedural-3d-1-any` catalog.
const KINDS: &[&str] = &[
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Generation3dDiff` mirrors `Generation3dSnapshot`'s two fields name for name, so the table is empty; the sibling `🀄️wfc` subset, whose diff splits every collection into a `<name>Removed`/`<name>Upserted` pair, carries real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Generation3dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-widget` → `CreateWidget`, the Rust variant name this subset's EXTERNALLY tagged enum writes as the
/// payload's sole object key.
fn discriminant(kind: &str) -> String {
    kind.split('-').map(|word| format!("{}{}", word[..1].to_uppercase(), &word[1..])).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags EXTERNALLY, so the
/// discriminant is the payload's single member name and a payload of any other arity is malformed.
fn declared_kind(mutation: &Json) -> String {
    match mutation {
        Json::Object(entries) if entries.len() == 1 => entries[0].0.clone(),
        _ => String::new(),
    }
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the two-widget graph with its two-generation history, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/🗑️delete/removes-the-selected-generation-2-and-falls-back/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    let fixture_widgets = reparsed.get("fixture").map(|fixture| fixture.array("widgets").len()).unwrap_or(0);
    let fixture_synapses = reparsed.get("fixture").map(|fixture| fixture.array("synapses").len()).unwrap_or(0);
    let generations = reparsed.get("generation").map(|generation| generation.array("generations").len()).unwrap_or(0);
    if fixture_widgets < 2 || fixture_synapses == 0 || generations < 2 {
        return Err(format!("the committed round-trip snapshot is the two-widget, one-synapse graph with a two-generation history this scenario describes, but it carries {fixture_widgets}/{fixture_synapses}/{generations}"));
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/💠️mutate-lowpoly-1/🦀️.rs

SHA-256 607038696551a91adc6fb18ec85c8a902816df31b7fcc3929aa840418475f2f7

```rust
//! 🦀️ Lowpoly 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `lowpoly-mutation-semantics`
//! (`../../🔮️oracles/🔣️.json`): this is a semio-NATIVE
//! document and `LowpolyMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `LowpolyMutation::KINDS`
/// (`../../🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `lowpoly-1-any` catalog.
const KINDS: &[&str] = &[
    "create-object",
    "delete-object",
    "reorder-objects",
    "rename-object",
    "change-object-smooth-shading",
    "move-object",
    "rotate-object",
    "scale-object",
    "create-mesh",
    "delete-mesh",
    "insert-paint-layer",
    "remove-paint-layer",
    "rename-paint-layer",
    "change-paint-layer-visible",
    "change-paint-layer-opacity",
    "change-paint-layer-blend-mode",
    "edit-paint-layer",
    "apply-paint-stroke",
    "move-selection",
    "rotate-selection",
    "scale-selection",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `LowpolyDiff` mirrors `LowpolySnapshot` name for name, so the table is empty and every field is matched by its own name; the sibling `🀄️wfc` and `🖐️5d`/`🧊️3d` block subsets, whose diffs split, rename or FOLD their fields, carry real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared LowpolyMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-object` → `CreateObject`, the Rust variant name this subset's EXTERNALLY tagged enum writes as the
/// payload's sole object key.
fn discriminant(kind: &str) -> String {
    kind.split('-').map(|word| format!("{}{}", word[..1].to_uppercase(), &word[1..])).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags EXTERNALLY, so the
/// discriminant is the payload's single member name and a payload of any other arity is malformed.
fn declared_kind(mutation: &Json) -> String {
    match mutation {
        Json::Object(entries) if entries.len() == 1 => entries[0].0.clone(),
        _ => String::new(),
    }
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the two-object lowpoly document that carries stacked paint layers, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/➖️remove-paint-layer/drops-the-detail-layer-at-index-1/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.array("objects").len() != 2 || reparsed.array("objects").iter().map(|object| object.array("paintLayers").len()).sum::<usize>() < 2 {
        return Err("the committed round-trip snapshot is the two-object, stacked-paint-layer document this scenario describes, but it does not carry two objects with at least two paint layers between them".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📐️mutate-cad-1/🦀️.rs

SHA-256 a2fa13dcd14b9103c4791639e769c1ca0fc97c5d5bab7504a88a4e5a12daa3a8

```rust
//! 🦀️ CAD 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `cad-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `CadMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `CadMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `cad-1-any` catalog.
const KINDS: &[&str] = &[
    "create-shape-model",
    "delete-shape-model",
    "create-building-model",
    "delete-building-model",
    "create-energy-model",
    "delete-energy-model",
    "create-structure-classic-model",
    "delete-structure-classic-model",
    "create-drawing",
    "delete-drawing",
    "create-node",
    "delete-node",
    "rename-node",
    "change-reference-hidden",
    "change-reference-locked",
    "change-reference-width",
    "move-reference",
    "replace-reference-media",
    "replace-references",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `CadDiff` mirrors `CadSnapshot` name for name, so the table is empty and every field is matched by its own name; the `🀄️wfc`, `🖐️5d` and `🧊️3d` subsets, whose diffs split, rename or fold their fields, carry real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state is inexpressible on the JSON wire — `Option<Option<T>>` renders
/// `Some(None)` as `null`, exactly like an untouched field. The footprint law accepts an undeclared
/// change on these ONLY when the new value is itself `null`, so a field that changed to anything
/// else is still a failure rather than an exemption.
const VACATE_COLLAPSES: &[&str] = &["shapeModel", "buildingModel", "energyModel", "structureClassicModel"];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared CadMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-shape-model` → `createShapeModel`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the reference-bearing CAD composition, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/📎replace-references/swaps-the-shape-reference-list/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.get("referencesByModelDefinitionId").is_none() || reparsed.array("drawings").is_empty() || reparsed.array("nodes").len() < 2 {
        return Err("the committed round-trip snapshot is the reference-bearing CAD composition this scenario describes — reference planes filed per model definition, a drawing child and a node tree — but at least one of those is missing".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-5d-1/🦀️.rs

SHA-256 9b2e8fec04f3857d8bf21079df1281978d285a6d50e6c844482628c69e7ae900

```rust
//! 🦀️ Block 5d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `block-5d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Block5dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Block5dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `block-5d-1-any` catalog.
const KINDS: &[&str] = &[
    "rename-part-kind",
    "change-part-kind-label",
    "change-part-kind-variant",
    "change-part-kind-description",
    "change-part-kind-icon",
    "change-part-kind-unit",
    "update-part2d",
    "update-part3d",
    "create-representation",
    "delete-representation",
    "rename-representation",
    "change-representation-mesh-url",
    "change-representation-lod",
    "change-representation-description",
    "add-representation-tag",
    "remove-representation-tag",
    "add-representation-attribute",
    "remove-representation-attribute",
    "create-grip-kind",
    "delete-grip-kind",
    "rename-grip-kind",
    "change-grip-kind-label",
    "change-grip-kind-color",
    "change-grip-kind-default-rope-kind",
    "create-grip",
    "delete-grip",
    "move-grip2d",
    "move-grip3d",
    "resize-grip3d",
    "change-grip-grip-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "move-camera3d",
    "scale-camera3d",
    "change-meta-description",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Block5dDiff` does not mirror its snapshot for the two facet fields: the snapshot names them `2d` and `3d` (leading digits, which no Rust identifier can carry) while the diff names them `part2d` and `part3d`. Without these two rows the footprint law would report `update-part-2d` as an undeclared change on every run.
const DIFF_ALIASES: &[(&str, &[&str])] = &[
    ("2d", &["part2d"]),
    ("3d", &["part3d"]),
];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Block5dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `rename-part-kind` → `renamePartKind`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the fully populated part-kind definition with both facets, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/✏️rename-part-kind/renames-part-kind-to-pod/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.get("2d").is_none() || reparsed.get("3d").is_none() || reparsed.array("gripKinds").len() < 2 || reparsed.array("representations").is_empty() {
        return Err("the committed round-trip snapshot is the two-facet part-kind definition this scenario describes — a 2d facet, a 3d facet, two grip kinds and a representation — but at least one of those is missing".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🦀️.rs

SHA-256 1665ffbe0df25732aca6d433b9aaf393af7e86b9c9d494c8e94ccbf203da5e4f

```rust
//! 🦀️ Puzzle 2d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `puzzle-2d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Puzzle2dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it. `spec-vector-<id>` carries every vector
//! the two exhaustive tables cannot: the refusals, whose declared code and contract-D6
//! `🔺️diff/🚫️.absent` sentinel it checks and which have no diff to measure a footprint against, and
//! the second and third vectors of a kind, which cannot ride a `mutate-<kind>` id because the
//! completeness gate reads that id as a claim about the KIND itself.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Puzzle2dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `puzzle-2d-1-any` catalog.
const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "move-node",
    "replace-node-geometry",
    "change-node-kind",
    "edit-node-text",
    "change-node-icon",
    "scale-node",
    "change-node-visible",
    "change-node-locked",
    "change-node-root",
    "change-node-anchor",
    "add-node-handle",
    "remove-node-handle",
    "replace-node-handle",
    "connect-handles",
    "disconnect-handles",
    "replace-edge-geometry",
    "change-edge-kind",
    "change-edge-tips",
    "change-edge-visible",
    "change-edge-locked",
    "change-manifest-id",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "create-target-region",
    "delete-target-region",
    "move-target-region",
    "resize-target-region",
    "edit-target-region-label",
    "change-target-region-hidden",
    "change-target-region-locked",
    "drag-selection",
    "rotate-selection",
    "scale-selection",
];

/// 🧾️ The row ids of the third Examples table, in its own order. They are NOT kinds: the completeness
/// gate reads a `mutate-<kind>` scenario id as a claim about that kind's vocabulary entry, so a second
/// row per kind has to carry its own identity. `<kind>-alpha` is a synthetic two-node-board vector kept
/// from before this corpus was rebuilt on the shipped examples, `<kind>-refused` a contract-D6 refusal,
/// `<kind>-duplicate`/`<kind>-cleared` the two warning-level branches. The three selection transforms
/// add `<kind>-mixed` (nodes and target regions in one target set), `<kind>-partial` (missing and locked
/// members skipped as `mutation.partial`) and `<kind>-unchanged` (identity parameters, a `no-op`).
const SPEC_VECTORS: &[&str] = &[
    "create-node-alpha",
    "create-node-refused",
    "delete-node-alpha",
    "delete-node-refused",
    "move-node-alpha",
    "move-node-refused",
    "replace-node-geometry-alpha",
    "replace-node-geometry-refused",
    "change-node-kind-alpha",
    "change-node-kind-refused",
    "edit-node-text-alpha",
    "edit-node-text-refused",
    "change-node-icon-alpha",
    "change-node-icon-refused",
    "scale-node-alpha",
    "scale-node-refused",
    "change-node-visible-alpha",
    "change-node-visible-refused",
    "change-node-locked-alpha",
    "change-node-locked-refused",
    "change-node-root-alpha",
    "change-node-root-refused",
    "change-node-anchor-alpha",
    "change-node-anchor-refused",
    "add-node-handle-alpha",
    "add-node-handle-refused",
    "remove-node-handle-alpha",
    "remove-node-handle-refused",
    "replace-node-handle-refused",
    "connect-handles-alpha",
    "connect-handles-duplicate",
    "disconnect-handles-alpha",
    "disconnect-handles-refused",
    "replace-edge-geometry-alpha",
    "replace-edge-geometry-refused",
    "change-edge-kind-alpha",
    "change-edge-kind-refused",
    "change-edge-tips-alpha",
    "change-edge-tips-refused",
    "change-edge-visible-alpha",
    "change-edge-visible-refused",
    "change-edge-locked-alpha",
    "change-edge-locked-refused",
    "change-manifest-id-alpha",
    "connect-kind-compatibility-alpha",
    "disconnect-kind-compatibility-alpha",
    "disconnect-kind-compatibility-refused",
    "replace-kind-catalogs-alpha",
    "replace-kind-catalogs-cleared",
    "create-target-region-alpha",
    "create-target-region-refused",
    "delete-target-region-refused",
    "move-target-region-refused",
    "resize-target-region-refused",
    "edit-target-region-label-refused",
    "change-target-region-hidden-refused",
    "change-target-region-locked-refused",
    "drag-selection-mixed",
    "drag-selection-partial",
    "drag-selection-refused",
    "drag-selection-unchanged",
    "rotate-selection-mixed",
    "rotate-selection-partial",
    "rotate-selection-refused",
    "rotate-selection-unchanged",
    "scale-selection-mixed",
    "scale-selection-partial",
    "scale-selection-refused",
    "scale-selection-unchanged",
    "drag-selection-invariant",
    "rotate-selection-invariant",
    "scale-selection-invariant",
    "scale-selection-invariant-negative",
    "scale-node-invariant",
    "replace-node-geometry-invariant",
    "create-node-invariant",
    "add-node-handle-invariant",
    "replace-node-handle-invariant",
    "replace-kind-catalogs-invariant",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Puzzle2dDiff` mirrors `Puzzle2dSnapshot` name for name, so the table is empty and every field is matched by its own name; the sibling `🀄️wfc` and `🧱️block` subsets, whose diffs split, rename or fold their fields, carry real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Puzzle2dMutation kind"));
    }
    Ok(Vector {
        kind,
        before: ctx.fixture_json(&spec.str("before"))?,
        mutation: ctx.fixture_json(&spec.str("mutation"))?,
        diff: ctx.fixture_json(&spec.str("diff"))?,
        after: ctx.fixture_json(&spec.str("after"))?,
        outcome: ctx.fixture_json(&spec.str("outcome"))?,
    })
}

/// 🐫️ `create-node` → `createNode`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🧾️ Every vector the two exhaustive tables do NOT carry: the synthetic alpha-board vectors kept
/// from before this corpus was rebuilt on the shipped examples, the twenty-nine refusals, and the two
/// warning-level branches. They cannot ride the `mutate`/`inverse` tables because the completeness
/// gate reads a `mutate-<kind>` id as a claim about the KIND, so a second row per kind would report
/// an undeclared kind; and because a refusal has no diff to measure a footprint against.
fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Puzzle2dMutation kind"));
    }
    let verdict = spec.str("verdict");
    let before = ctx.fixture_json(&spec.str("before"))?;
    let after = ctx.fixture_json(&spec.str("after"))?;
    let outcome = ctx.fixture_json(&spec.str("outcome"))?;
    let mutation = ctx.fixture_json(&spec.str("mutation"))?;
    let declared = declared_kind(&mutation);
    if declared != discriminant(&kind) {
        return Err(format!("the committed mutation payload filed under {kind:?} declares {declared:?} — the vector does not exercise the kind this row claims"));
    }
    let status = outcome.str("status");
    match verdict.as_str() {
        "refused" => {
            if status != "rejected" {
                return Err(format!("the {kind:?} refusal vector declares status {status:?}, not \"rejected\""));
            }
            if outcome.str("code").is_empty() {
                return Err(format!("the {kind:?} refusal vector declares no machine-readable code, so nothing states WHICH refusal it pins"));
            }
            if !spec.str("diff").ends_with("🚫️.absent") || !ctx.fixture_bytes(&spec.str("diff"))?.is_empty() {
                return Err(format!("contract D6: the {kind:?} refusal vector must commit an EMPTY 🔺️diff/🚫️.absent sentinel instead of an invented empty patch"));
            }
            if !changed_fields(&before, &after).is_empty() {
                return Err(format!("the {kind:?} refusal vector moved the document, so it did not refuse"));
            }
        }
        "noop" => {
            if status != "no-op" || !declares_no_op(&outcome) {
                return Err(format!("the {kind:?} no-op vector must declare the no-op outcome class carrying mutation.no-op, got {status:?}"));
            }
            no_op_law(&Vector { kind: kind.clone(), before: before.clone(), mutation, diff: ctx.fixture_json(&spec.str("diff"))?, after: after.clone(), outcome })?;
        }
        "applied" => {
            if status != "applied" || declares_no_op(&outcome) {
                return Err(format!("the {kind:?} applied vector must declare an applied outcome with no mutation.no-op, got {status:?}"));
            }
            let vector = Vector { kind: kind.clone(), before: before.clone(), mutation, diff: ctx.fixture_json(&spec.str("diff"))?, after: after.clone(), outcome };
            law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
            footprint_law(&vector)?;
        }
        other => return Err(format!("unknown verdict {other:?}; this table declares applied, noop or refused")),
    }
    Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
}

/// 🔁️ Decode and re-encode the real First-Storey-Tambour subgraph of the Nakagin Capsule Tower,
/// through the platform's own dependency-free JSON reader and writer: the document must survive
/// unchanged, and the re-serialized bytes must NOT be the committed bytes — the committed file is
/// pretty-printed and the writer is compact, so a handler that returned the input unread would be
/// caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/🌱create-node/🌱️appends/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.array("nodes").len() < 2 || reparsed.array("edges").is_empty() || reparsed.get("meta").map(|meta| meta.array("kindCompatibility").len()).unwrap_or(0) == 0 {
        return Err("the committed round-trip snapshot is the real tower subgraph this scenario describes — nodes, edges and a kind-compatibility relation — but it does not carry all three".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    for vector in SPEC_VECTORS {
        built = built.subject(&format!("spec-vector-{vector}"), spec_vector);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🖐️mutate-puzzle-5d-1/🦀️.rs

SHA-256 533e88d3887e6b8dfd78f6317f626404426f7971f0297183af28e25177974199

```rust
//! 🦀️ Puzzle 5d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `puzzle-5d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Puzzle5dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Puzzle5dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `puzzle-5d-1-any` catalog.
const KINDS: &[&str] = &[
    "create-part",
    "delete-part",
    "move-part2d",
    "replace-part2d-geometry",
    "edit-part2d-text",
    "change-part2d-icon",
    "change-part2d-hidden",
    "change-part2d-locked",
    "move-part3d",
    "rotate-part3d",
    "scale-part3d",
    "change-part3d-mesh",
    "edit-part3d-label",
    "change-part-kind",
    "change-part-anchor",
    "add-part-grip",
    "remove-part-grip",
    "replace-part-grip",
    "connect-grips",
    "disconnect-grips",
    "replace-fastener-geometry",
    "change-fastener-kind",
    "rename-puzzle5d",
    "change-domain",
    "change-description",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "drag-selection2d",
    "drag-selection3d",
    "rotate-selection3d",
    "scale-selection3d",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Puzzle5dDiff` mirrors `Puzzle5dSnapshot` name for name — including the `parts`/`fasteners` pair — so the table is empty; the sibling `🖐️5d` BLOCK subset, whose diff renames the same two facets to `part2d`/`part3d`, carries real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Puzzle5dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-part` → `createPart`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the two-part, one-fastener puzzle assembly, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/🌱create-part/appends-part-c/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.array("parts").len() < 2 || reparsed.array("fasteners").is_empty() || reparsed.array("kindCompatibility").is_empty() {
        return Err("the committed round-trip snapshot is the two-part, one-fastener, compatibility-carrying assembly this scenario describes, but it does not carry all three".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-block-3d-1/🦀️.rs

SHA-256 6339263a8914d5dcabfd6ac2ecce967c9e69adfcf4cff965bd6171a87a71b7e5

```rust
//! 🦀️ Block 3d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `block-3d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Block3dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Block3dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `block-3d-1-any` catalog.
const KINDS: &[&str] = &[
    "rename-object-kind",
    "change-object-kind-label",
    "change-object-kind-variant",
    "change-object-kind-description",
    "change-object-kind-icon",
    "change-object-kind-unit",
    "create-representation",
    "delete-representation",
    "rename-representation",
    "change-representation-mesh-url",
    "change-representation-lod",
    "change-representation-description",
    "add-representation-tag",
    "remove-representation-tag",
    "add-representation-attribute",
    "remove-representation-attribute",
    "create-vortex-kind",
    "delete-vortex-kind",
    "rename-vortex-kind",
    "change-vortex-kind-label",
    "change-vortex-kind-color",
    "change-vortex-kind-default-cable-kind",
    "create-vortex",
    "delete-vortex",
    "move-vortex",
    "resize-vortex",
    "change-vortex-vortex-kind",
    "change-vortex-label",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera3d",
    "scale-camera3d",
    "change-meta-description",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Block3dDiff` FOLDS two snapshot fields into one: the snapshot keeps the shared kind catalogue child in `catalog` and this document's own additions in `vortexKindExtra`, while the diff declares both through a single `vortexKinds` field. Both rows therefore point at the same diff field — a many-to-one alias no other subset in this scope needs.
const DIFF_ALIASES: &[(&str, &[&str])] = &[
    ("catalog", &["vortexKinds"]),
    ("vortexKindExtra", &["vortexKinds"]),
];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Block3dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `rename-object-kind` → `renameObjectKind`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the object-kind definition with its catalogue child and local extras, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/✏️rename-object-kind/renames-object-kind-to-pod/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.get("catalog").is_none() || reparsed.array("vortexKindExtra").len() < 2 || reparsed.array("vortices").is_empty() || reparsed.array("representations").is_empty() {
        return Err("the committed round-trip snapshot is the object-kind definition this scenario describes — a catalogue child, two local vortex kinds, a placed vortex and a representation — but at least one of those is missing".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```

## ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧊️mutate-puzzle-3d-1/🦀️.rs

SHA-256 185ee4ba75ef15057ef8a8187f6b4d46c7e8aa7eae650a73a900e18b5dd5d972

```rust
//! 🦀️ Puzzle 3d 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `puzzle-3d-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): this is a semio-NATIVE
//! document and `Puzzle3dMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/<fixture>/`, replayed
//! through the platform, plus two metamorphic laws asserted IN ROLE.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law is asserted inside the SUBJECT handler. A handler that merely read the vectors and
//! returned would report a pass having checked nothing.
//!
//! **What the laws are.** `mutate-<kind>` asserts the committed payload really declares that kind
//! and that the vector MOVES the document (the observability law), unless the committed outcome
//! itself declares `mutation.no-op`, in which case the opposite is asserted: nothing moved and the
//! diff declares nothing. `inverse-<kind>` asserts FOOTPRINT COMPLETENESS — `before` and `after`
//! differ on exactly the fields the committed diff declares — which is the precondition that makes a
//! mutation undoable at all, and the strongest inverse property a reader that does not link this
//! subset's own codec can establish: the committed diff's collection arms record removals as bare
//! ids, so a removed record is not reconstructable from the diff alone and the full law
//! `apply(inverse(m), apply(m, base)) == base` stays with the production `inverse()` implementation
//! and the per-leaf fixture tests that already exercise it.
//!
//! @see ../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../../../../../🗄️stdio/🔮️oracles/⚖️law/🦀️.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `Puzzle3dMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because this host must not link the plugin crate.
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps the const honest against the
/// enum and the manifest; the contract's coverage gate keeps this list honest against the
/// `puzzle-3d-1-any` catalog.
const KINDS: &[&str] = &[
    "create-object",
    "delete-object",
    "move-object",
    "rotate-object",
    "scale-object",
    "change-object-mesh",
    "edit-object-label",
    "change-object-kind",
    "change-object-anchor",
    "change-object-hidden",
    "change-object-locked",
    "add-object-vortex",
    "remove-object-vortex",
    "replace-object-vortex",
    "connect-vortices",
    "disconnect-vortices",
    "replace-attraction-geometry",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "create-reference",
    "delete-reference",
    "move-reference",
    "resize-reference",
    "replace-reference-source",
    "change-reference-hidden",
    "change-reference-locked",
    "change-domain",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "drag-selection",
    "rotate-selection",
    "scale-selection",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `Puzzle3dDiff` mirrors `Puzzle3dSnapshot` name for name across all four collections, so the table is empty; the sibling `🀄️wfc` subset, whose diff splits every collection into a `<name>Removed`/`<name>Upserted` pair, carries real rows here.
const DIFF_ALIASES: &[(&str, &[&str])] = &[];

/// 🕳️ Fields whose CLEARED state would be inexpressible on the JSON wire (an `Option<Option<T>>`
/// whose vacated arm renders as `null`, indistinguishable from untouched). This subset's diff carries
/// no doubly-optional field, so the footprint law grants no exemption at all here.
const VACATE_COLLAPSES: &[&str] = &[];
//#endregion 🔖️Vocabulary

//#region 🔖️Vector
/// 🧫️ One committed specification vector, read from the five files the scenario's own doc string
/// names — no recomputation, no transcription into Rust literals.
struct Vector {
    kind: String,
    before: Json,
    mutation: Json,
    diff: Json,
    after: Json,
    outcome: Json,
}

fn vector(ctx: &Context) -> Result<Vector, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("scenario doc string names {kind:?}, which is not a declared Puzzle3dMutation kind"));
    }
    Ok(Vector { kind, before: ctx.fixture_json(&spec.str("before"))?, mutation: ctx.fixture_json(&spec.str("mutation"))?, diff: ctx.fixture_json(&spec.str("diff"))?, after: ctx.fixture_json(&spec.str("after"))?, outcome: ctx.fixture_json(&spec.str("outcome"))? })
}

/// 🐫️ `create-object` → `createObject`, the discriminant this subset's
/// `#[serde(tag = "mutation", rename_all = "camelCase")]` enum writes.
fn discriminant(kind: &str) -> String {
    kind.split('-').enumerate().map(|(index, word)| if index == 0 { word.to_string() } else { format!("{}{}", word[..1].to_uppercase(), &word[1..]) }).collect()
}

/// 🏷️ The kind a committed payload actually declares: this subset tags INTERNALLY, so the
/// discriminant is the payload's own `mutation` member.
fn declared_kind(mutation: &Json) -> String {
    mutation.str("mutation")
}

/// 🚦️ Whether the committed outcome itself declares this vector a no-op — the fixture's own record
/// that the mutation had nothing to do, which inverts what the observability law must demand.
fn declares_no_op(outcome: &Json) -> bool {
    outcome.array("messages").iter().any(|message| message.str("code") == "mutation.no-op")
}

fn field_names(value: &Json) -> Vec<String> {
    match value {
        Json::Object(entries) => entries.iter().map(|(key, _)| key.clone()).collect(),
        _ => Vec::new(),
    }
}

fn member(value: &Json, key: &str) -> Json {
    value.get(key).cloned().unwrap_or(Json::Null)
}

/// 🔎️ The union of both committed snapshots' field names, in `before`'s own order.
fn snapshot_fields(before: &Json, after: &Json) -> Vec<String> {
    let mut names = field_names(before);
    for key in field_names(after) {
        if !names.contains(&key) {
            names.push(key);
        }
    }
    names
}

/// 🔎️ Snapshot fields whose value moved between the two committed snapshots.
fn changed_fields(before: &Json, after: &Json) -> Vec<String> {
    snapshot_fields(before, after).into_iter().filter(|key| member(before, key) != member(after, key)).collect()
}

/// 🔎️ Diff fields the committed diff actually populates. Every arm of this subset's diff is always
/// on the wire — an untouched field is `null` and an untouched list arm is `[]` — so those two
/// shapes declare nothing, while a whole-value replacement that happens to carry an empty list
/// (`{"values": []}`) declares plenty and must not be mistaken for an untouched field.
fn declared_fields(diff: &Json) -> Vec<String> {
    field_names(diff)
        .into_iter()
        .filter(|key| match member(diff, key) {
            Json::Null => false,
            Json::Array(items) => !items.is_empty(),
            _ => true,
        })
        .collect()
}

fn diff_names_for(field: &str) -> Vec<String> {
    let mut names = vec![field.to_string()];
    for (snapshot_field, aliases) in DIFF_ALIASES {
        if *snapshot_field == field {
            names.extend(aliases.iter().map(|alias| alias.to_string()));
        }
    }
    names
}
//#endregion 🔖️Vector

//#region 🔖️Laws
/// ⚖️ Footprint completeness: `before` and `after` differ on exactly the fields the committed diff
/// declares. The forward half catches a snapshot that drifted outside the diff — a change the undo
/// history would silently lose; the reverse half catches a diff that claims a field it never
/// touched — an undo that would rewrite something the mutation left alone.
fn footprint_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    let declared = declared_fields(&vector.diff);
    for field in &changed {
        if diff_names_for(field).iter().any(|name| declared.contains(name)) {
            continue;
        }
        if VACATE_COLLAPSES.contains(&field.as_str()) && member(&vector.after, field) == Json::Null {
            continue;
        }
        return Err(format!(
            "footprint law violated: {:?} changed the snapshot field {field:?} to {} without declaring it in the committed diff, so an undo built from that diff would not restore it",
            vector.kind,
            member(&vector.after, field).to_string()
        ));
    }
    let fields = snapshot_fields(&vector.before, &vector.after);
    for field in &declared {
        let owners: Vec<&String> = fields.iter().filter(|name| diff_names_for(name).contains(field)).collect();
        if owners.is_empty() || owners.iter().any(|owner| changed.contains(owner)) {
            continue;
        }
        return Err(format!("footprint law violated: the committed diff for {:?} declares {field:?}, yet the snapshot field it governs is identical in both committed snapshots", vector.kind));
    }
    Ok(())
}

/// ⚖️ A committed no-op vector must BE a no-op: nothing moved and nothing declared. Where a kind
/// ships only a no-op vector this arm is what keeps the row from reporting the green a real
/// mutation vector would have earned.
fn no_op_law(vector: &Vector) -> Result<(), String> {
    let changed = changed_fields(&vector.before, &vector.after);
    if !changed.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet the snapshot fields {changed:?} moved", vector.kind));
    }
    let declared = declared_fields(&vector.diff);
    if !declared.is_empty() {
        return Err(format!("no-op law violated: the committed outcome for {:?} declares mutation.no-op, yet its diff declares {declared:?}", vector.kind));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🔖️Handlers
/// 🎯️ The committed vector really exercises the kind the row claims, and it moves the document.
fn conformance(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    let declared = declared_kind(&vector.mutation);
    if declared != discriminant(&vector.kind) {
        return Err(format!("the committed mutation payload filed under {:?} declares {declared:?}, not {:?} — the vector does not exercise the kind this row claims", vector.kind, discriminant(&vector.kind)));
    }
    let status = vector.outcome.str("status");
    if status != "applied" {
        return Err(format!("the committed outcome for {:?} declares status {status:?}; this feature replays applied vectors only", vector.kind));
    }
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        law::mutation_is_observable(&vector.kind, &vector.after, &vector.before, &[])?;
    }
    Ok(Outcome::with_raw(vector.after.to_string().into_bytes(), vector.after))
}

/// ↩️ The undo precondition: the change is entirely inside the committed diff's footprint.
fn footprint(ctx: &Context) -> Result<Outcome, String> {
    let vector = vector(ctx)?;
    if declares_no_op(&vector.outcome) {
        no_op_law(&vector)?;
    } else {
        footprint_law(&vector)?;
    }
    Ok(Outcome::with_raw(vector.before.to_string().into_bytes(), vector.before))
}

/// 🔁️ Decode and re-encode the four-collection puzzle scene, through the platform's own dependency-free JSON reader and writer: the
/// document must survive unchanged, and the re-serialized bytes must NOT be the committed bytes —
/// the committed file is pretty-printed and the writer is compact, so a handler that returned the
/// input unread would be caught here.
fn round_trip(ctx: &Context) -> Result<Outcome, String> {
    const SNAPSHOT: &str = "shared://🧬️mutations/🌱create-object/appends-object-c/📸️snapshot/⬅️before/🔣️.json";
    let committed = ctx.fixture_bytes(SNAPSHOT)?;
    let parsed = ctx.fixture_json(SNAPSHOT)?;
    let reserialized = parsed.to_string();
    law::reparsed_not_copied(reserialized.as_bytes(), &committed)?;
    let reparsed = semio_repo_test_host::parse_json(&reserialized)?;
    law::round_trip_preserves(&reparsed, &parsed)?;
    if reparsed.array("objects").len() < 2 || reparsed.array("attractions").is_empty() || reparsed.array("targetVolumes").is_empty() || reparsed.array("references").is_empty() {
        return Err("the committed round-trip snapshot is the four-collection scene this scenario describes — objects, attractions, target volumes and references — but at least one of them is empty".to_string());
    }
    Ok(Outcome::with_raw(reserialized.into_bytes(), reparsed))
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration is by FULL expanded scenario id, so the loop mirrors the feature's `Examples`
/// tables exactly. Every handler is registered in the SUBJECT role: a recorded no-oracle case runs
/// no oracle role at all, so a handler registered there would never execute.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), conformance).subject(&format!("inverse-{kind}"), footprint);
    }
    built.subject("identity-round-trip", round_trip)
}
//#endregion 🔖️Registration
```
