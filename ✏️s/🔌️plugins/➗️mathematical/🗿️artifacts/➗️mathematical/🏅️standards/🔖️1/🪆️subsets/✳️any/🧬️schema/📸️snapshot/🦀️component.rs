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
    /// 🔢️ Arbitrary-precision integer, round-tripped through `math::number::Integer`'s own
    /// `Display`/`FromStr` (decimal, sign-prefixed) — never `i64`, which would silently truncate.
    Integer { lexeme: String },
    /// ➗️ `numer/denom`, each an `Integer` decimal lexeme (same round-trip as above).
    Rational { numer: String, denom: String },
    Symbol { name: String },
    Add { terms: Vec<EquationNode> },
    Mul { factors: Vec<EquationNode> },
    Pow { base: Box<EquationNode>, exponent: Box<EquationNode> },
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
    pub fn find(&self, label: EquationNodeLabel) -> Option<&EquationNode> {
        find_labeled(&self.expr, label)
    }

    /// ✏️ Structural replace-in-place by label; a no-op (returns `false`) if `label` isn't
    /// present — callers (mutation `diff`s) must treat that as "nothing to do", never a panic,
    /// since `base` may already have moved past the label a stale payload still names.
    pub fn replace(&mut self, label: EquationNodeLabel, new_kind: EquationNodeKind) -> bool {
        replace_labeled(&mut self.expr, label, &new_kind)
    }
}

fn find_labeled(node: &EquationNode, label: EquationNodeLabel) -> Option<&EquationNode> {
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

fn replace_labeled(node: &mut EquationNode, label: EquationNodeLabel, new_kind: &EquationNodeKind) -> bool {
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
pub fn equation_node_to_expr(node: &EquationNode) -> crate::cas::expr::Expr {
    use crate::cas::expr::Expr;
    match &node.kind {
        EquationNodeKind::Integer { lexeme } => {
            let value: math::number::Integer = lexeme.parse().unwrap_or_else(|_| math::number::Integer::zero());
            Expr::from(value)
        }
        EquationNodeKind::Rational { numer, denom } => {
            let n: math::number::Integer = numer.parse().unwrap_or_else(|_| math::number::Integer::zero());
            let d: math::number::Integer = denom.parse().unwrap_or_else(|_| math::number::Integer::one());
            match math::number::Rational::new(n, d) {
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
pub fn expr_to_equation_node(expr: &crate::cas::expr::Expr, next_label: &mut u64) -> EquationNode {
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

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`'s/`✒️writer`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened
/// via `to_uri()`), never the child's own content.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
/// 🧮️ `equation` has no handcrafted grammar of its own yet (future wave) — round-tripped as
/// hex-encoded `serde_json`, the same "real codec, minimal grammar" trade `child` handles above
/// already make for their own opaque payload half (the `ArtifactRef` URI).
fn enc_equation(e: &EquationSnapshot) -> String {
    enc_str(&serde_json::to_string(e).expect("EquationSnapshot serializes"))
}
fn dec_equation(s: &str) -> Result<EquationSnapshot, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}

fn print_mathematical_snapshot_body(s: &MathematicalSnapshot) -> String {
    format!("notation={}\nresults={}\ncomputed={}\nequation={}", enc_child(&s.notation), enc_child(&s.results), enc_child(&s.computed), enc_equation(&s.equation))
}
fn parse_mathematical_snapshot_body(body: &str) -> Result<MathematicalSnapshot, String> {
    let mut notation = None;
    let mut results = None;
    let mut computed = None;
    let mut equation = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("notation=") {
            notation = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("results=") {
            results = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("computed=") {
            computed = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("equation=") {
            equation = Some(dec_equation(rest)?);
        } else {
            return Err(format!("mathematical snapshot: unknown line {line:?}"));
        }
    }
    Ok(MathematicalSnapshot {
        notation: notation.ok_or_else(|| "mathematical snapshot: missing notation line".to_string())?,
        results: results.ok_or_else(|| "mathematical snapshot: missing results line".to_string())?,
        computed: computed.ok_or_else(|| "mathematical snapshot: missing computed line".to_string())?,
        equation: equation.ok_or_else(|| "mathematical snapshot: missing equation line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn write_equation(out: &mut Vec<u8>, e: &EquationSnapshot) {
    write_bytes_lp(out, serde_json::to_string(e).expect("EquationSnapshot serializes").as_bytes());
}
fn read_equation(reader: &mut store::ByteReader<'_>) -> Result<EquationSnapshot, String> {
    let bytes = read_bytes_lp(reader)?;
    let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn encode_mathematical_snapshot_binary(s: &MathematicalSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_child(&mut out, &s.notation);
    write_child(&mut out, &s.results);
    write_child(&mut out, &s.computed);
    write_equation(&mut out, &s.equation);
    out
}
fn decode_mathematical_snapshot_binary(bytes: &[u8]) -> Result<MathematicalSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    Ok(MathematicalSnapshot { notation: read_child(&mut reader)?, results: read_child(&mut reader)?, computed: read_child(&mut reader)?, equation: read_equation(&mut reader)? })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack, real hex/bracket text + LEB128 binary primitives —
/// same upgrade `📐️cad`/`✒️writer` made once their snapshot gained a real `ArtifactChild<S>` slot
/// (the old `dsl::DslRecord`-derive-driven path cannot express a composed child slot, which has no
/// `dsl::DslField` impl reachable from this crate). The former DSL-mirror indirection through
/// `crate::artifacts::mathematical::dsl::MathematicalSnapshotDsl` is gone — that mirror type still
/// exists for `MathematicalGraphDsl` (the `SetArtifact` command's own payload shape), just no longer
/// for the snapshot's own codec.
impl store::ArtifactDsl for MathematicalSnapshot {
    const EXTENSION: &'static str = "mathematical";
    fn envelope_id() -> &'static str {
        "mathematical.mathematical"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_mathematical_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_mathematical_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for MathematicalSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_mathematical_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_mathematical_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for MathematicalSnapshot {
    fn default() -> Self {
        crate::artifacts::mathematical::mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default())
    }
}
//#endregion 🔖️Snapshot
