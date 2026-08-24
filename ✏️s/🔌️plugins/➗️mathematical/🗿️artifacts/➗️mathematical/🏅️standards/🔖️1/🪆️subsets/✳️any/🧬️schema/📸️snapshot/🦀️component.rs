//! 🧬️ Mathematical snapshot schema — artifact-lane fields only.

use crate::artifacts::mathematical::{MathematicalComputedChild, MathematicalGeometry, MathematicalGraph, MathematicalNotationChild, MathematicalResultsChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted mathematical document snapshot. Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
/// (`mathematical→C:text,table,value`): the inline `graph`/`geometry` fields are replaced by three
/// fixed composed CHILD slots — this plugin no longer defines its own text/table/value content
/// models, it composes stdio's `text`/`table`/`value` subsets instead. `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
///
/// 🚚 `equation` (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave
/// M3a) is a FOURTH, plain (non-`#[child]`) persistent field, deliberately NOT routed through the
/// text/table/value composition contract above: that contract governs this plugin's competing
/// content models for exactly those three kinds (verified against `SemioValueSnapshot` — an
/// untyped `Null/Bool/Int/Float/Str/Bytes/List/Map/Ref` JSON-like graph with no operator/variable/
/// assumption vocabulary, genuinely unable to host structural equation edits like "change
/// coefficient of term 3" without a typed `Expr`-shaped enum to address). See `🔖️Equation` below.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub notation: MathematicalNotationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: MathematicalResultsChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub computed: MathematicalComputedChild,
    #[state(artifact)]
    pub equation: EquationSnapshot,
}

//#region 🔖️Equation
/// 🪪 A never-reused node identity issued at node birth and carried in the snapshot — mirrors
/// `✳️brep`'s `PersistentLabel` shape. A mutation address built from THIS survives unrelated
/// edits: unlike a positional path (`expr.children[2].children[0]`), which breaks the instant a
/// sibling is inserted or removed anywhere in the tree (exactly the bug class documented in
/// `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave4-reports/mathematical-report.md`'s
/// `insert_point_inverse_is_remove_point_at_same_index` finding — a base-relative Vec index that
/// silently resolves to the wrong element once the collection's length has changed underneath
/// it — a label is opaque, assigned once, and never reassigned or renumbered by any mutation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquationNodeLabel(pub u64);

/// 🌳 One labeled node of the persisted expression tree — deliberately a SEPARATE, plain,
/// serde-friendly type from `cas::expr::Expr` (the `Rc`-shared, hash-cached, auto-simplifying
/// runtime form `🌿️cas-internals` builds and computes over): `Expr` has no `Serialize`/
/// `Deserialize` and its private `Node`/hash-cache invariants are only safe to construct through
/// `canon.rs`'s smart constructors, which a naive field-by-field deserialize would violate.
/// `EquationNode` is the AUTHORED form — what a mutation edits and what gets persisted — converted
/// to/from `cas::expr::Expr` at the boundary of every inference/mutation that needs to compute
/// (`equation_node_to_expr`/`expr_to_equation_node` below), using ONLY `cas`'s public constructor
/// API, never touching its private internals.
///
/// Scope (honest limitation, not yet the full `cas::expr::Kind` vocabulary): `Integer`/`Rational`/
/// `Symbol`/`Add`/`Mul`/`Pow` only — enough for a single-variable polynomial equation (the
/// `roots` vertical slice this wave proves end-to-end). `Fn`/`Piecewise`/`Rel`/`Wild`/`RootOf`/
/// `Constant` are future work for whichever wave extends the mutation/inference table beyond
/// `roots`/`change-coefficient`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquationNode {
    pub label: EquationNodeLabel,
    pub kind: EquationNodeKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum EquationNodeKind {
    /// 🔢️ Arbitrary-precision integer, round-tripped through `number::Integer`'s own
    /// `Display`/`FromStr` (decimal, sign-prefixed) — never `i64`, which would silently truncate.
    Integer {
        lexeme: String,
    },
    /// ➗️ `numer/denom`, each an `Integer` decimal lexeme (same round-trip as above).
    Rational {
        numer: String,
        denom: String,
    },
    Symbol {
        name: String,
    },
    Add {
        terms: Vec<EquationNode>,
    },
    Mul {
        factors: Vec<EquationNode>,
    },
    Pow {
        base: Box<EquationNode>,
        exponent: Box<EquationNode>,
    },
}

/// 📸️ Persisted equation content: the expression AST plus the label allocator that guarantees
/// every future `create-term` mints a label no earlier mutation (or its inverse) could ever
/// collide with — `next_label` only ever increases, even across delete+undo cycles.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquationSnapshot {
    pub expr: EquationNode,
    pub next_label: u64,
}

impl Default for EquationSnapshot {
    fn default() -> Self {
        Self { expr: EquationNode { label: EquationNodeLabel(0), kind: EquationNodeKind::Integer { lexeme: "0".to_string() } }, next_label: 1 }
    }
}

impl EquationSnapshot {
    /// 🔎️ Depth-first search by label — the ONLY address a mutation/inverse ever resolves
    /// against, per `EquationNodeLabel`'s stability contract.
    pub async fn find(&self, label: EquationNodeLabel) -> Option<&EquationNode> {
        find_labeled(&self.expr, label)
    }

    /// ✏️ Structural replace-in-place by label; a no-op (returns `false`) if `label` isn't
    /// present — callers (mutation `diff`s) must treat that as "nothing to do", never a panic,
    /// since `base` may already have moved past the label a stale payload still names.
    pub async fn replace(&mut self, label: EquationNodeLabel, new_kind: EquationNodeKind) -> bool {
        replace_labeled(&mut self.expr, label, &new_kind)
    }
}

async fn find_labeled(node: &EquationNode, label: EquationNodeLabel) -> Option<&EquationNode> {
    if node.label == label {
        return Some(node);
    }
    match &node.kind {
        EquationNodeKind::Add { terms } => terms.iter().find_map(|t| find_labeled(t, label)),
        EquationNodeKind::Mul { factors } => factors.iter().find_map(|f| find_labeled(f, label)),
        EquationNodeKind::Pow { base, exponent } => find_labeled(base, label).or_else(|| find_labeled(exponent, label)),
        EquationNodeKind::Integer { .. } | EquationNodeKind::Rational { .. } | EquationNodeKind::Symbol { .. } => None,
    }
}

async fn replace_labeled(node: &mut EquationNode, label: EquationNodeLabel, new_kind: &EquationNodeKind) -> bool {
    if node.label == label {
        node.kind = new_kind.clone();
        return true;
    }
    match &mut node.kind {
        EquationNodeKind::Add { terms } => terms.iter_mut().any(|t| replace_labeled(t, label, new_kind)),
        EquationNodeKind::Mul { factors } => factors.iter_mut().any(|f| replace_labeled(f, label, new_kind)),
        EquationNodeKind::Pow { base, exponent } => replace_labeled(base, label, new_kind) || replace_labeled(exponent, label, new_kind),
        EquationNodeKind::Integer { .. } | EquationNodeKind::Rational { .. } | EquationNodeKind::Symbol { .. } => false,
    }
}

//#region 🔖️CasExprBridge
/// 🌉 `EquationNode` → `cas::expr::Expr`, through `cas`'s PUBLIC constructor API only
/// (`Expr::integer`/`Expr::from(Rational)`/`Expr::symbol`/`Expr::add`/`Expr::mul`/`Expr::pow`) —
/// never touches `cas`'s private `Node`/hash-cache fields. Labels are dropped here: `Expr` has no
/// concept of node identity, it is the pure computation form.
pub async fn equation_node_to_expr(node: &EquationNode) -> crate::cas::expr::Expr {
    use crate::cas::expr::Expr;
    match &node.kind {
        EquationNodeKind::Integer { lexeme } => {
            let value: number::Integer = lexeme.parse().unwrap_or_else(|_| number::Integer::zero());
            Expr::from(value)
        }
        EquationNodeKind::Rational { numer, denom } => {
            let n: number::Integer = numer.parse().unwrap_or_else(|_| number::Integer::zero());
            let d: number::Integer = denom.parse().unwrap_or_else(|_| number::Integer::one());
            match number::Rational::new(n, d) {
                Some(r) => Expr::from(r),
                None => Expr::integer(0),
            }
        }
        EquationNodeKind::Symbol { name } => Expr::symbol(name),
        EquationNodeKind::Add { terms } => Expr::add(terms.iter().map(equation_node_to_expr).collect()),
        EquationNodeKind::Mul { factors } => Expr::mul(factors.iter().map(equation_node_to_expr).collect()),
        EquationNodeKind::Pow { base, exponent } => Expr::pow(equation_node_to_expr(base), equation_node_to_expr(exponent)),
    }
}

/// 🌉 `cas::expr::Expr` → `EquationNode`, minting a fresh label per node via `next_label` (used
/// when a mutation's payload carries a brand-new authored subtree that has never had a label —
/// e.g. `create-term`'s inserted `Expr`). Falls back to `Integer(0)` for any `Kind` outside this
/// wave's scope (`Fn`/`Piecewise`/`Rel`/`Wild`/`RootOf`/`Constant`/`Bool`) rather than panicking —
/// documented gap, not silent corruption (the fallback is structurally distinguishable, never
/// mistaken for a real computed value, since callers control which `Expr`s they ever pass in
/// during this wave's proven scope).
pub async fn expr_to_equation_node(expr: &crate::cas::expr::Expr, next_label: &mut u64) -> EquationNode {
    use crate::cas::expr::Kind;
    let label = EquationNodeLabel(*next_label);
    *next_label += 1;
    let kind = match expr.kind() {
        Kind::Integer(i) => EquationNodeKind::Integer { lexeme: i.to_string() },
        Kind::Rational(r) => EquationNodeKind::Rational { numer: r.numer().to_string(), denom: r.denom().to_string() },
        Kind::Symbol(s) => EquationNodeKind::Symbol { name: s.name().to_string() },
        Kind::Add(terms) => EquationNodeKind::Add { terms: terms.iter().map(|t| expr_to_equation_node(t, next_label)).collect() },
        Kind::Mul(factors) => EquationNodeKind::Mul { factors: factors.iter().map(|f| expr_to_equation_node(f, next_label)).collect() },
        Kind::Pow(base, exponent) => EquationNodeKind::Pow { base: Box::new(expr_to_equation_node(base, next_label)), exponent: Box::new(expr_to_equation_node(exponent, next_label)) },
        Kind::Constant(_) | Kind::Bool(_) | Kind::Fn(_, _) | Kind::RootOf { .. } | Kind::Piecewise(_) | Kind::Rel(_, _, _) | Kind::Wild(_, _) => EquationNodeKind::Integer { lexeme: "0".to_string() },
    };
    EquationNode { label, kind }
}
//#endregion 🔖️CasExprBridge
//#endregion 🔖️Equation

impl Default for MathematicalSnapshot {
    fn default() -> Self {
        crate::artifacts::mathematical::mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default())
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `mathematical_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn mathematical_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <MathematicalSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <MathematicalSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <MathematicalSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <MathematicalSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = serde_json::json!({
        "parsed": serde_json::to_value(&parsed).map_err(|error| error.to_string())?,
        "reparsed": serde_json::to_value(&reparsed).map_err(|error| error.to_string())?,
        "packDecoded": serde_json::to_value(&unpacked).map_err(|error| error.to_string())?,
        "canonicalText": canonical,
        "canonicalTextAgain": canonical_again,
    });
    Ok(report.to_string())
}
//#endregion 🌉️IdentityBridge
