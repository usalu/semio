//! 🦀️ CAD 1 exhaustive mutation case — Rust adapter. Ticket
//! `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `cad-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): this is a semio-NATIVE
//! document and `CadMutation` IS its specification, so there is nothing third-party to register. What
//! stands in for an oracle is named there and exercised here: the committed
//! `(before, mutation, diff, outcome, after)` quintets under
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, replayed
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
//! @see ../../../../../🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs — the shared law helpers.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Shared
/// ⚖️ The repository's shared, dependency-free metamorphic-law helpers, mounted by path rather than
/// linked: a generated test host may not gain a Cargo dependency on another plugin's crate, and the
/// module is deliberately format-neutral — it knows about divergences and laws, not about any
/// document model. `#[path = "."]` re-roots the nested path at THIS file's directory instead of the
/// implicit `🦀️component/` child directory.
#[path = "."]
mod shared {
    #[path = "../../../../../🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs"]
    pub mod law;
}
use shared::law;
//#endregion 🔖️Shared

//#region 🔖️Vocabulary
/// 🏷️ Mirrors `CadMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) — duplicated, not
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
    "change-active-model-definition",
];

/// 🔀️ Snapshot field → the diff field(s) allowed to declare it. `CadDiff` mirrors `CadSnapshot` name for name, so the table is empty and every field is matched by its own name; the `🧩️assembly`, `🖐️5d` and `🧊️3d` subsets, whose diffs split, rename or fold their fields, carry real rows here.
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
    const SNAPSHOT: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎replace-references/🧪️tests/swaps-the-shape-reference-list/📸️snapshot/⬅️before/🔣️component.json";
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
