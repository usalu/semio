//! 🧮️ The symbolic expression tree: an immutable, `Rc`-shared, structurally-hash-cached tree. Every
//! node is built exclusively through `canon.rs`'s smart constructors, which maintain the
//! auto-simplification invariants (flattening, numeric folding, canonical ordering) that the rest of
//! the crate assumes hold everywhere.

use crate::cas::assume::AssumeSet;
use crate::cas::fnkind::FnKind;
use number::{Integer, Rational};
use std::rc::Rc;

// #region 🔖️Symbol
/// 🔤️ A named variable plus the assumption flags fixed at its creation — two symbols with the same
/// name but different flags are deliberately distinct (prevents silently mixing up `x` real vs `x`
/// unconstrained).
#[derive(Clone, Debug)]
pub struct Symbol {
    name: Rc<str>,
    assumptions: AssumeSet,
}

impl Symbol {
    pub(crate) fn new(name: &str, assumptions: AssumeSet) -> Self {
        Self { name: Rc::from(name), assumptions }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assumptions(&self) -> AssumeSet {
        self.assumptions
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.assumptions == other.assumptions
    }
}
impl Eq for Symbol {}
impl std::hash::Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.assumptions.bits().hash(state);
    }
}
impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name).then_with(|| self.assumptions.bits().cmp(&other.assumptions.bits()))
    }
}
impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// #endregion 🔖️Symbol

// #region 🔖️Constant
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Constant {
    Pi,
    E,
    I,
    EulerGamma,
    Inf,
    NegInf,
    ComplexInf,
    Undefined,
}

impl Constant {
    pub fn name(&self) -> &'static str {
        match self {
            Constant::Pi => "pi",
            Constant::E => "e",
            Constant::I => "i",
            Constant::EulerGamma => "EulerGamma",
            Constant::Inf => "oo",
            Constant::NegInf => "-oo",
            Constant::ComplexInf => "zoo",
            Constant::Undefined => "undefined",
        }
    }
}
// #endregion 🔖️Constant

// #region 🔖️RelationalOperator
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum RelationalOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
// #endregion 🔖️RelationalOperator

// #region 🔖️WildKind
#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum WildKind {
    Any,
    Number,
    NotZero,
    /// 🆓️ Matches only subtrees free of the named symbol.
    FreeOf(Rc<str>),
    /// 📚️ Matches a whole sequence of `Add`/`Mul` arguments (at most one per pattern).
    Seq,
}
// #endregion 🔖️WildKind

// #region 🔖️Kind
#[derive(Clone, PartialEq, Debug)]
pub enum Kind {
    Integer(Integer),
    /// ➗️ Invariant (maintained by `canon.rs`): never has an integral value — those fold to `Integer`.
    Rational(Rational),
    Symbol(Symbol),
    Constant(Constant),
    Bool(bool),
    /// ➕️ Flattened, canonically sorted, length `>= 2`; a numeric coefficient (if any) sorts first.
    Add(Vec<Expr>),
    /// ✖️ Same invariants as `Add`.
    Mul(Vec<Expr>),
    Pow(Expr, Expr),
    Fn(FnKind, Vec<Expr>),
    /// 🌱️ The `index`-th real root (ascending) of the integer polynomial with coefficients `coeffs`
    /// (little-endian, degree = `coeffs.len() - 1`); used when `solve` can't produce a closed radical form.
    RootOf {
        coeffs: Vec<Rational>,
        index: u32,
    },
    /// 🔀️ `(value, condition)` pairs; the last condition may be `Bool(true)` for a catch-all default.
    Piecewise(Vec<(Expr, Expr)>),
    Rel(RelationalOperator, Expr, Expr),
    /// 🃏️ Pattern-only placeholder; must never appear in a user-facing expression.
    Wild(u16, WildKind),
}
// #endregion 🔖️Kind

// #region 🔖️Node
struct Node {
    hash: u64,
    kind: Kind,
}

/// 🔢️ FNV-1a, computed bottom-up once at construction and cached — equality checks hash first (cheap
/// reject), then `Rc::ptr_eq` (cheap accept, common since subtrees are shared), then a full structural
/// compare only in the rare remaining case.
fn fnv1a_mix(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

fn hash_kind(kind: &Kind) -> u64 {
    let mut h = FNV_OFFSET;
    match kind {
        Kind::Integer(n) => {
            h = fnv1a_mix(h, b"int");
            h = fnv1a_mix(h, n.to_decimal().as_bytes());
        }
        Kind::Rational(r) => {
            h = fnv1a_mix(h, b"rat");
            h = fnv1a_mix(h, r.numer().to_decimal().as_bytes());
            h = fnv1a_mix(h, r.denom().to_decimal().as_bytes());
        }
        Kind::Symbol(s) => {
            h = fnv1a_mix(h, b"sym");
            h = fnv1a_mix(h, s.name.as_bytes());
            h = fnv1a_mix(h, &s.assumptions.bits().to_le_bytes());
        }
        Kind::Constant(c) => {
            h = fnv1a_mix(h, b"const");
            h = fnv1a_mix(h, c.name().as_bytes());
        }
        Kind::Bool(b) => {
            h = fnv1a_mix(h, b"bool");
            h = fnv1a_mix(h, &[*b as u8]);
        }
        Kind::Add(terms) => {
            h = fnv1a_mix(h, b"add");
            for t in terms {
                h = fnv1a_mix(h, &t.hash().to_le_bytes());
            }
        }
        Kind::Mul(factors) => {
            h = fnv1a_mix(h, b"mul");
            for f in factors {
                h = fnv1a_mix(h, &f.hash().to_le_bytes());
            }
        }
        Kind::Pow(base, exp) => {
            h = fnv1a_mix(h, b"pow");
            h = fnv1a_mix(h, &base.hash().to_le_bytes());
            h = fnv1a_mix(h, &exp.hash().to_le_bytes());
        }
        Kind::Fn(kind, args) => {
            h = fnv1a_mix(h, b"fn");
            h = fnv1a_mix(h, kind.name().as_bytes());
            for a in args {
                h = fnv1a_mix(h, &a.hash().to_le_bytes());
            }
        }
        Kind::RootOf { coeffs, index } => {
            h = fnv1a_mix(h, b"rootof");
            for c in coeffs {
                h = fnv1a_mix(h, c.numer().to_decimal().as_bytes());
                h = fnv1a_mix(h, c.denom().to_decimal().as_bytes());
            }
            h = fnv1a_mix(h, &index.to_le_bytes());
        }
        Kind::Piecewise(cases) => {
            h = fnv1a_mix(h, b"piecewise");
            for (v, c) in cases {
                h = fnv1a_mix(h, &v.hash().to_le_bytes());
                h = fnv1a_mix(h, &c.hash().to_le_bytes());
            }
        }
        Kind::Rel(operation, a, b) => {
            h = fnv1a_mix(h, b"rel");
            h = fnv1a_mix(h, &[*operation as u8]);
            h = fnv1a_mix(h, &a.hash().to_le_bytes());
            h = fnv1a_mix(h, &b.hash().to_le_bytes());
        }
        Kind::Wild(id, _) => {
            h = fnv1a_mix(h, b"wild");
            h = fnv1a_mix(h, &id.to_le_bytes());
        }
    }
    h
}
// #endregion 🔖️Node

// #region 🔖️Expr
/// ✨️ An immutable, reference-counted symbolic expression. `!Send`/`!Sync` by design (single-threaded,
/// `Rc`-based sharing — swap to `Arc` in one place, `P<T>` below, if that's ever needed).
#[derive(Clone)]
pub struct Expr(Rc<Node>);

impl Expr {
    /// ⚠️ Only `canon.rs` should call this — every other constructor goes through the smart
    /// constructors there to maintain the auto-simplification invariants.
    pub fn from_kind_unchecked(kind: Kind) -> Self {
        let hash = hash_kind(&kind);
        Self(Rc::new(Node { hash, kind }))
    }

    pub fn kind(&self) -> &Kind {
        &self.0.kind
    }

    pub fn hash(&self) -> u64 {
        self.0.hash
    }

    pub fn symbol(name: &str) -> Self {
        crate::cas::canon::make_symbol(name, AssumeSet::NONE)
    }

    pub fn symbol_with(name: &str, assumptions: AssumeSet) -> Self {
        crate::cas::canon::make_symbol(name, assumptions)
    }

    pub fn integer(value: i64) -> Self {
        crate::cas::canon::make_integer(Integer::from_i64(value))
    }

    pub fn constant(c: Constant) -> Self {
        Self::from_kind_unchecked(Kind::Constant(c))
    }

    pub fn boolean(b: bool) -> Self {
        Self::from_kind_unchecked(Kind::Bool(b))
    }

    pub fn as_integer(&self) -> Option<&Integer> {
        match self.kind() {
            Kind::Integer(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self.kind() {
            Kind::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_zero_literal(&self) -> bool {
        matches!(self.kind(), Kind::Integer(n) if n.is_zero())
    }

    pub fn is_one_literal(&self) -> bool {
        matches!(self.kind(), Kind::Integer(n) if *n == Integer::one())
    }

    // #region 🔖️PublicConstructors
    pub fn add(terms: Vec<Expr>) -> Self {
        crate::cas::canon::make_add(terms)
    }

    pub fn mul(factors: Vec<Expr>) -> Self {
        crate::cas::canon::make_mul(factors)
    }

    pub fn pow(base: Expr, exp: Expr) -> Self {
        crate::cas::canon::make_pow(base, exp)
    }

    pub fn func(kind: FnKind, args: Vec<Expr>) -> Self {
        crate::cas::canon::make_func(kind, args)
    }
    // #endregion 🔖️PublicConstructors
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        if Rc::ptr_eq(&self.0, &other.0) {
            return true;
        }
        if self.0.hash != other.0.hash {
            return false;
        }
        self.0.kind == other.0.kind
    }
}
impl Eq for Expr {}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash.hash(state);
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::cas::fmt::display_string(self))
    }
}

/// 🔢️ Kind-class rank used as the primary key of the canonical total order (before falling back to
/// structural comparison within the same class).
fn kind_rank(k: &Kind) -> u8 {
    match k {
        Kind::Integer(_) => 0,
        Kind::Rational(_) => 0,
        Kind::Constant(_) => 1,
        Kind::Symbol(_) => 2,
        Kind::Pow(..) => 3,
        Kind::Mul(_) => 4,
        Kind::Fn(..) => 5,
        Kind::Add(_) => 6,
        Kind::RootOf { .. } => 7,
        Kind::Piecewise(_) => 8,
        Kind::Rel(..) => 9,
        Kind::Bool(_) => 10,
        Kind::Wild(..) => 11,
    }
}

impl Ord for Expr {
    /// 🔢️ Purely structural — never pointer- or hash-order — so `Display`/canonicalization output is
    /// stable across runs and processes.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        if Rc::ptr_eq(&self.0, &other.0) {
            return Ordering::Equal;
        }
        let (ra, rb) = (kind_rank(self.kind()), kind_rank(other.kind()));
        if ra != rb {
            return ra.cmp(&rb);
        }
        match (self.kind(), other.kind()) {
            (Kind::Integer(a), Kind::Integer(b)) => a.cmp(b),
            (Kind::Integer(a), Kind::Rational(b)) => Rational::from_integer(a.clone()).cmp(b),
            (Kind::Rational(a), Kind::Integer(b)) => a.cmp(&Rational::from_integer(b.clone())),
            (Kind::Rational(a), Kind::Rational(b)) => a.cmp(b),
            (Kind::Constant(a), Kind::Constant(b)) => a.cmp(b),
            (Kind::Symbol(a), Kind::Symbol(b)) => a.cmp(b),
            (Kind::Pow(b1, e1), Kind::Pow(b2, e2)) => b1.cmp(b2).then_with(|| e1.cmp(e2)),
            (Kind::Mul(a), Kind::Mul(b)) => a.cmp(b),
            (Kind::Fn(k1, a1), Kind::Fn(k2, a2)) => k1.name().cmp(&k2.name()).then_with(|| a1.cmp(a2)),
            (Kind::Add(a), Kind::Add(b)) => a.cmp(b),
            (Kind::RootOf { coeffs: c1, index: i1 }, Kind::RootOf { coeffs: c2, index: i2 }) => c1.cmp(c2).then_with(|| i1.cmp(i2)),
            (Kind::Piecewise(a), Kind::Piecewise(b)) => a.cmp(b),
            (Kind::Rel(o1, a1, b1), Kind::Rel(o2, a2, b2)) => o1.cmp(o2).then_with(|| a1.cmp(a2)).then_with(|| b1.cmp(b2)),
            (Kind::Bool(a), Kind::Bool(b)) => a.cmp(b),
            (Kind::Wild(i1, _), Kind::Wild(i2, _)) => i1.cmp(i2),
            _ => Ordering::Equal,
        }
    }
}
impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// #region 🔖️Operators
impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Expr {
        Expr::add(vec![self, rhs])
    }
}
impl std::ops::Add<&Expr> for &Expr {
    type Output = Expr;
    fn add(self, rhs: &Expr) -> Expr {
        Expr::add(vec![self.clone(), rhs.clone()])
    }
}
impl std::ops::Sub for Expr {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Expr {
        Expr::add(vec![self, crate::cas::canon::make_neg(rhs)])
    }
}
impl std::ops::Sub<&Expr> for &Expr {
    type Output = Expr;
    fn sub(self, rhs: &Expr) -> Expr {
        Expr::add(vec![self.clone(), crate::cas::canon::make_neg(rhs.clone())])
    }
}
impl std::ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Expr {
        Expr::mul(vec![self, rhs])
    }
}
impl std::ops::Mul<&Expr> for &Expr {
    type Output = Expr;
    fn mul(self, rhs: &Expr) -> Expr {
        Expr::mul(vec![self.clone(), rhs.clone()])
    }
}
impl std::ops::Div for Expr {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Expr {
        Expr::mul(vec![self, Expr::pow(rhs, Expr::integer(-1))])
    }
}
impl std::ops::Div<&Expr> for &Expr {
    type Output = Expr;
    fn div(self, rhs: &Expr) -> Expr {
        Expr::mul(vec![self.clone(), Expr::pow(rhs.clone(), Expr::integer(-1))])
    }
}
impl std::ops::Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        crate::cas::canon::make_neg(self)
    }
}
impl std::ops::Neg for &Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        crate::cas::canon::make_neg(self.clone())
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::integer(value)
    }
}
impl From<Integer> for Expr {
    fn from(value: Integer) -> Self {
        crate::cas::canon::make_integer(value)
    }
}
impl From<Rational> for Expr {
    fn from(value: Rational) -> Self {
        crate::cas::canon::make_rational(value)
    }
}
// #endregion 🔖️Operators
// #endregion 🔖️Expr

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️expr-unit/🦀️.rs");
// #endregion 🔖️Tests

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EquationNode {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
}

/// 🔌️ JSON-facing edge — plain `source`/`target` id strings for the JS frontend's node-graph payloads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EquationEdge {
    pub id: String,
    pub source: String,
    pub target: String,
}

/// 🕸️ Graph playground state: quadrant toggle, retained layout, and the active algorithm overlay.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EquationGraph {
    pub directed: bool,
    pub nodes: Vec<EquationNode>,
    pub edges: Vec<EquationEdge>,
    pub algorithm: String,
    #[value(default)]
    pub algorithm_seed: Option<String>,
}

impl Default for EquationGraph {
    fn default() -> Self {
        Self {
            directed: true,
            nodes: vec![
                EquationNode { id: "a".into(), label: "A".into(), x: 40.0, y: 60.0 },
                EquationNode { id: "b".into(), label: "B".into(), x: 240.0, y: 20.0 },
                EquationNode { id: "c".into(), label: "C".into(), x: 240.0, y: 180.0 },
                EquationNode { id: "d".into(), label: "D".into(), x: 440.0, y: 100.0 },
            ],
            edges: vec![
                EquationEdge { id: "e1".into(), source: "a".into(), target: "b".into() },
                EquationEdge { id: "e2".into(), source: "a".into(), target: "c".into() },
                EquationEdge { id: "e3".into(), source: "b".into(), target: "d".into() },
                EquationEdge { id: "e4".into(), source: "c".into(), target: "d".into() },
            ],
            algorithm: "topo".into(),
            algorithm_seed: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
pub struct EquationPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for EquationPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<EquationPoint> for (f64, f64) {
    fn from(point: EquationPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐️ Geometry playground state: a point cloud for convex-hull/centroid demonstration.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EquationGeometry {
    pub points: Vec<EquationPoint>,
}

impl Default for EquationGeometry {
    fn default() -> Self {
        Self { points: vec![(40.0, 220.0), (260.0, 40.0), (360.0, 140.0), (300.0, 260.0), (140.0, 300.0), (180.0, 160.0)].into_iter().map(EquationPoint::from).collect() }
    }
}

//#endregion 🔖️Document
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;

//#region 🔖️ChildTypes
pub type EquationNotationChild = store::os_store::artifact_child::ArtifactChild<SemioTextSnapshot>;
pub type EquationResultsChild = store::os_store::artifact_child::ArtifactChild<SemioTableSnapshot>;
pub type EquationComputedChild = store::os_store::artifact_child::ArtifactChild<SemioValueSnapshot>;
//#endregion 🔖️ChildTypes
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{EquationGeometry, EquationGraph};
use crate::{EquationComputedChild, EquationNotationChild, EquationResultsChild};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableCellKind;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableColumn;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableRow;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextRun;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;
pub use crate::{snapshot::schema::EquationExprSnapshot, snapshot::schema::EquationCarrierSnapshot};
pub use crate::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;

//#region 🔖️Composition
/// 🧩️ Model (a) of `📓️s3-math-report.md` §1 (design §20.15): the graph and the point cloud are PARENT-owned persisted state
/// (`EquationSnapshot::graph`/`geometry`); the three composed children — `notation` (node labels as `s.stdio.semio` text
/// runs), `results` (the node table) and `computed` (direction, algorithm, edges and points as one value map) — are DERIVED
/// outputs of that state, addressed by their own content. Every parent leaf decides from `base.graph`/`base.geometry` and
/// has its handles re-derived by `EquationDiff::apply`; the runtime then opens the newly addressed child
/// from the parent's own state (`follow_derivable_children` → [`genesis_equation_child_pack`]) and retires the old one. A
/// derived child is never edited and no parent leaf ever reads it. The converters below are the one-way derivation.

//#region 🔖️Converters
/// 🌉 Derives the notation child: one `text` run per node label, in `graph.nodes` order.
pub fn equation_notation_from_graph(graph: &EquationGraph) -> SemioTextSnapshot {
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs: graph.nodes.iter().map(|node| SemioTextRun { language: String::new(), content: node.label.clone(), marks: Vec::new() }).collect() }
}

/// 📊️ Derives the results child: one `table` row of node `id`/`x`/`y` per node, in `graph.nodes` order (positionally aligned
/// with [`equation_notation_from_graph`]'s runs).
pub fn equation_results_from_graph(graph: &EquationGraph) -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "x".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "y".into(), kind: SemioTableCellKind::Float }],
        rows: graph.nodes.iter().map(|node| SemioTableRow { cells: vec![SemioValue::Str { value: node.id.clone() }, SemioValue::Float { lexeme: format!("{}", node.x) }, SemioValue::Float { lexeme: format!("{}", node.y) }] }).collect(),
    }
}

/// 🧮️ Derives the computed child: graph direction/algorithm/seed, edges and the point cloud as one structured `value` map.
pub fn equation_computed_from_state(graph: &EquationGraph, geometry: &EquationGeometry) -> SemioValueSnapshot {
    let edges = SemioValue::List {
        items: graph
            .edges
            .iter()
            .map(|edge| SemioValue::Map {
                entries: vec![
                    SemioValueEntry { key: "id".into(), value: SemioValue::Str { value: edge.id.clone() } },
                    SemioValueEntry { key: "source".into(), value: SemioValue::Str { value: edge.source.clone() } },
                    SemioValueEntry { key: "target".into(), value: SemioValue::Str { value: edge.target.clone() } },
                ],
            })
            .collect(),
    };
    let points = SemioValue::List {
        items: geometry
            .points
            .iter()
            .map(|point| SemioValue::Map {
                entries: vec![SemioValueEntry { key: "x".into(), value: SemioValue::Float { lexeme: format!("{}", point.x) } }, SemioValueEntry { key: "y".into(), value: SemioValue::Float { lexeme: format!("{}", point.y) } }],
            })
            .collect(),
    };
    SemioValueSnapshot {
        schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),
        root: SemioValue::Map {
            entries: vec![
                SemioValueEntry { key: "directed".into(), value: SemioValue::Bool { value: graph.directed } },
                SemioValueEntry { key: "algorithm".into(), value: SemioValue::Str { value: graph.algorithm.clone() } },
                SemioValueEntry {
                    key: "algorithmSeed".into(),
                    value: match &graph.algorithm_seed {
                        Some(seed) => SemioValue::Str { value: seed.clone() },
                        None => SemioValue::Null,
                    },
                },
                SemioValueEntry { key: "edges".into(), value: edges },
                SemioValueEntry { key: "points".into(), value: points },
            ],
        },
        nodes: Vec::new(),
    }
}

//#endregion 🔖️Converters

//#region 🔖️DerivedChildren
/// 🧮️ The content-addressed handle of one derived child: its id is `content_id(prefix, pack)` of the derived content, the same
/// as its target `artifact_id` (what `ChildRestoreProjection::child` demands), so equal state addresses equal children.
pub fn equation_derived_child<S>(prefix: &str, subset: &str, pack: &[u8]) -> store::os_store::artifact_child::ArtifactChild<S> {
    let child_id = store::content_id(prefix, pack);
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    store::os_store::artifact_child::ArtifactChild::new(child_id.clone(), semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id, dialect })
}

/// 📦️ The derived pack of composed slot `slot` from `(graph, geometry)`, with the address prefix and subset it is minted
/// under; `None` for a slot this artifact does not derive.
pub fn equation_derived_pack(slot: &str, graph: &EquationGraph, geometry: &EquationGeometry) -> Option<(&'static str, &'static str, Vec<u8>)> {
    use pack::snapshot::ArtifactPack;
    match slot {
        "notation" => Some(("equation-notation", "text", <SemioTextSnapshot as ArtifactPack>::encode_pack(&equation_notation_from_graph(graph)))),
        "results" => Some(("equation-results", "table", <SemioTableSnapshot as ArtifactPack>::encode_pack(&equation_results_from_graph(graph)))),
        "computed" => Some(("equation-computed", "value", <SemioValueSnapshot as ArtifactPack>::encode_pack(&equation_computed_from_state(graph, geometry)))),
        _ => None,
    }
}

/// 🏗️ The three derived-child handles of `(graph, geometry)`.
pub fn equation_children(graph: &EquationGraph, geometry: &EquationGeometry) -> (EquationNotationChild, EquationResultsChild, EquationComputedChild) {
    let handle = |slot: &str| equation_derived_pack(slot, graph, geometry).expect("a derived slot");
    let ((notation_prefix, notation_subset, notation), (results_prefix, results_subset, results), (computed_prefix, computed_subset, computed)) = (handle("notation"), handle("results"), handle("computed"));
    (equation_derived_child(notation_prefix, notation_subset, &notation), equation_derived_child(results_prefix, results_subset, &results), equation_derived_child(computed_prefix, computed_subset, &computed))
}

/// 🌱️ `ArtifactApp::genesis_child_pack`: the derived content of the composed member `slot` a document names, minted from the
/// parent's own `graph`/`geometry` — the boot store, every whole-document load (`Effect::LoadDocument` ships no member) and
/// every derived child the runtime follows after a parent edit. A coordinate that is not the content address of the parent's
/// own derivation is no child this document derives.
pub fn genesis_equation_child_pack(snapshot: &EquationSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    let declared = match slot {
        "notation" => &snapshot.notation.child_id,
        "results" => &snapshot.results.child_id,
        "computed" => &snapshot.computed.child_id,
        _ => return None,
    };
    let (prefix, _, pack) = equation_derived_pack(slot, &snapshot.graph, &snapshot.geometry)?;
    (child_id == declared && store::content_id(prefix, &pack) == child_id).then_some(pack)
}

/// 📤️ Every equation field for a foreign carrier.
pub fn equation_carrier_snapshot(snapshot: &EquationSnapshot) -> EquationCarrierSnapshot {
    EquationCarrierSnapshot { graph: snapshot.graph.clone(), geometry: snapshot.geometry.clone(), equation: snapshot.equation.clone() }
}

/// 🏗️ A full `EquationSnapshot` from a `(graph, geometry)` pair with its derived handles and the default equation.
pub fn equation_snapshot_with_state(graph: &EquationGraph, geometry: &EquationGeometry) -> EquationSnapshot {
    let (notation, results, computed) = equation_children(graph, geometry);
    EquationSnapshot { graph: graph.clone(), geometry: geometry.clone(), notation, results, computed, equation: EquationExprSnapshot::default() }
}

/// 📥️ A full `EquationSnapshot` from a complete carrier fixture.
pub fn equation_snapshot_from_host_snapshot(fixture: EquationCarrierSnapshot) -> EquationSnapshot {
    let mut snapshot = equation_snapshot_with_state(&fixture.graph, &fixture.geometry);
    snapshot.equation = fixture.equation;
    snapshot
}
//#endregion 🔖️DerivedChildren
//#endregion 🔖️Composition
}
//#endregion 🧩️ArtifactItems4
