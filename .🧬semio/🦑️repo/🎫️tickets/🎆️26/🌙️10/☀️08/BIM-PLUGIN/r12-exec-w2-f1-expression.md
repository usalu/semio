# 🧮️ r12 Execution Report — `w2-f1-expression` (crate `semio-framework-expression`)

`M` = `🧰️framework/🔨️modules/🧮️expression`. Zero runtime dependencies (dev: `serde_json` with `float_roundtrip`). Builds in the **framework workspace**
(`--manifest-path 🧰️framework/Cargo.toml`); the repo-root `Cargo.toml` only lists it under `[workspace.dependencies]` (like its siblings) and does not
resolve `-p semio-framework-expression` until some root-workspace member depends on it. Plugin artifacts depend on it by path, e.g.
`semio-framework-expression = { path = "../../…/🧰️framework/🔨️modules/🧮️expression/📦️packages/🦀️rust" }` in their `[workspace.dependencies]`.

## Layout (domain folders, each `🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`; the package `🦀️.rs` is wiring)

| Folder | Content |
|---|---|
| `🌳️tree` | `Expr`, units, operators, `Function`, `Value`; `🧬️schema/{🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql}` = the AST contract |
| `⚠️errors` | `ExprError`, `ErrorKind` (errors as values, stable codes) |
| `📏️kinds` | `Kind`, `infer`, the dimension tables; `🧬️schema` of the fixtures |
| `🧮️evaluation` | `evaluate`, tolerant comparison |
| `🕸️parameters` | `dependencies`, `plan`, `evaluate_all`, `evaluate_all_declared` |
| `🔤️syntax` | `🔍️lexer`, `🌲️parser` (Pratt), `🖨️printer`, `ParseError`, `Span` |
| `🧫️fixtures/<domain>/🔣️.json` | syntax 371 cases + 28 error cases, kinds 346, evaluation 444 (348 values, 96 errors), parameters 60 |
| `🧪️tests/📜️conformance` | `🥒️.feature`, `🐍️.py` (Python oracle), `🦀️.rs` (Rust conformance test `expression_conformance`) |
| `📦️packages/🦀️rust` | `Cargo.toml`, `📋️project.json` (targets test, test-quick/long/exhaustive, test-oracle, test-oracle-python), `📜️script.ts` |

Generator (kept input script): `T/r12-w2-f1-expression-fixtures.py` writes all schemas, facets and fixtures (seeded, deterministic; expectations come from the
Python oracle). Run: `.venv/Scripts/python.exe T/r12-w2-f1-expression-fixtures.py`.

## Public API (all re-exported at the crate root unless a path is given)

```rust
pub enum Expr { Number(f64), Length(f64, LengthUnit), Angle(f64, AngleUnit), Area(f64, AreaUnit), Volume(f64, VolumeUnit), Bool(bool), Text(String),
    Param(String), Unary(UnaryOp, Box<Expr>), Binary(BinaryOp, Box<Expr>, Box<Expr>), Compare(CompareOp, Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>), Or(Box<Expr>, Box<Expr>), If(Box<Expr>, Box<Expr>, Box<Expr>), Call(Function, Vec<Expr>) }   // Clone, Debug, PartialEq
// helpers: Expr::param(&str), ::binary(op,l,r), ::compare(op,l,r), ::unary(op,e), ::conditional(c,t,e);
//          .children() -> Vec<&Expr>, .at(&[usize]) -> Option<&Expr>, .size(), .is_canonical()
pub enum LengthUnit { Millimetre, Centimetre, Metre, Kilometre, Inch, Foot }   // AngleUnit{Degree,Radian} AreaUnit{SquareMillimetre,SquareCentimetre,SquareMetre} VolumeUnit{CubicMillimetre,CubicCentimetre,CubicMetre,Litre}
pub enum UnaryOp { Negate, Not }   pub enum BinaryOp { Add, Subtract, Multiply, Divide, Power, Min, Max }
pub enum CompareOp { Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual }   pub enum Function { Sqrt, Abs, Round, Floor, Ceil, Sin, Cos, Tan, Atan2 }
pub enum Kind { Number, Length, Angle, Area, Volume, Bool, Text }    // .name() -> &'static str, Kind::from_name, .is_numeric(), Kind::ALL / NUMERIC
pub enum Value { Number(f64), Length(f64), Angle(f64), Area(f64), Volume(f64), Bool(bool), Text(String) }   // SI base units; .kind(), .magnitude() -> Option<f64>, Value::of_kind(Kind, f64)

pub fn parse(source: &str) -> Result<Expr, ParseError>;
pub fn print(expr: &Expr) -> String;                                           // canonical text; parse(print(e)) == e for e.is_canonical()
pub fn print_with_spans(expr: &Expr) -> (String, Vec<(Vec<usize>, Span)>);      // span of every node by child-index path (char offsets), sorted by path
pub fn infer(expr: &Expr, params: &BTreeMap<String, Kind>) -> Result<Kind, ExprError>;
pub fn evaluate(expr: &Expr, env: &BTreeMap<String, Value>) -> Result<Value, ExprError>;      // infer first, then evaluate; lazy and/or/if
pub fn dependencies(expr: &Expr) -> BTreeSet<String>;                           // every Param name, including untaken branches
pub fn evaluate_all(params: &BTreeMap<String, Expr>, overrides: &BTreeMap<String, Expr>) -> Resolved;
pub fn evaluate_all_declared(params, overrides, declared: &BTreeMap<String, Kind>) -> Resolved;   // + DeclaredKind errors
pub struct Resolved { pub order: Vec<String>, pub values: BTreeMap<String, Value>, pub errors: BTreeMap<String, ExprError> }
pub fn plan(graph: &BTreeMap<String, BTreeSet<String>>) -> Plan;                // Plan { order, cycles: Vec<Vec<String>>, blocked }
pub struct ExprError { pub path: Vec<usize>, pub kind: ErrorKind }              // .code() -> "kebab-code"; Display = English fallback; ErrorKind::message()
pub enum ErrorKind { UnknownParam{name}, OperandKind{site,position,expected,found}, MixedKinds{site,left,right}, Arity{site,expected,found},
    Exponent{base,exponent:Option<f64>}, Branches{then,otherwise}, DivisionByZero, Domain{site}, Overflow,
    Cycle{members}, FailedDependency{name}, UnknownOverride, DeclaredKind{declared,found} }
pub struct ParseError { pub span: Span, pub kind: ParseErrorKind }               // .code(), .message(), .render(source) (caret underline), Display
pub struct Span { pub start: usize, pub end: usize }                             // character offsets, half-open
// non-root: evaluation::{evaluate_with(expr, env, kinds), approx_eq, TOLERANCE}, kinds::{binary_kind, call_kind, literal_exponent}, syntax::KEYWORDS, LengthUnit::{symbol, from_symbol, to_si}
```

Notes for the families agent:
- `evaluate_all` returns a struct (not a bare tuple): `values` + `errors` + the `order` that was tried. Every parameter appears in exactly one of `values`/`errors`
  (an override of a missing parameter appears only in `errors`, kind `UnknownOverride`, and is otherwise ignored).
- Store formulas as their **canonical text** (`print`) in the snapshot and `parse` on read; there is no serde/Value derive on `Expr` (siblings do not derive
  on geometry types either). The JSON tree contract is `🌳️tree/🧬️schema/🔣️.json` if a JSON form is ever needed.
- Parameter errors have an empty `path`; errors inside a formula carry the child-index path (`Expr::at(path)`, `print_with_spans`) to underline the node.
- Kinds are SI-dimensional: `Length*Length=Area`, `Length*Area=Volume`, `Area/Length=Length`, `x/x=Number`, `Length^2/^3`, `Area^0.5` need a **literal** exponent;
  `sqrt(Area)=Length`; `round/floor/ceil` take `Number` only (round to a grid with `round(w / 100 mm) * 100 mm`); `sin/cos/tan` take `Angle`; `atan2` takes two equal
  numeric kinds and returns `Angle`; `=`/`!=` work on any equal kinds, `< <= > >=` on numeric kinds only. Equality is tolerant (relative 1e-9 against `max(1,|a|,|b|)`).
  A `Material` family parameter (text id) is a `Kind::Text`; `Integer`/`Real` are `Kind::Number`.
- Evaluation order is layered: each round takes every parameter whose dependencies are done, sorted by name; cycle members (sorted strongly connected component)
  fail with `Cycle{members}`, dependants with `FailedDependency{name}` (first failed dependency by name).
- Syntax: `2.4 m`, `90 mm`, `30 deg`, `45°`, `3 m2`/`m²`, `5 l`; `+ - * / ^`, `= != < <= > >=` (also `== ≠ ≤ ≥ × ÷ −`), `and or not`, `if c then a else b`,
  `min(a, b, …)`/`max`, quoted names `` `frame width` ``, text `"…"`. `^` is right associative and binds tighter than prefix `-`; comparisons do not chain.

## Commands and exact results (gate label `w2-f1-expression`, logs in `T/🗑️generated/w2-f1-expression/`)

| Command | Result |
|---|---|
| `cargo test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-expression` | lib **94 passed**, `expression_conformance` **5 passed**, doc 0; no warnings |
| `cargo check … -p semio-framework-expression --lib --target wasm32-wasip2` | OK |
| `.venv/Scripts/python.exe -m pytest -p no:cacheprovider 🧪️tests/📜️conformance/🐍️.py` | **11 passed** (jsonschema validates every fixture; Python `ast` re-derives every kind, value and parameter resolution) |
| `bun ./📜️script.ts test oracle python` in the package dir | 11 passed, ~12.7 s (inside the 15 s budget) |

Differential result: the Rust implementation and the independent Python oracle (own dimension tables, `ast` interpretation, `networkx` SCCs and topological
generations, `Decimal` rounding) agree on all 346 kind cases, 444 evaluation cases (values within 1e-12 relative, identical error code and node path) and 60 parameter
sets (identical order, values and errors); canonical printing and parsing agree on 371 syntax cases whose canonical text was produced by a separate Python printer.
Property tests: 20 000 generated trees satisfy `parse(print(e)) == e`, printing is a fixed point, evaluation results and error paths survive the round trip.

## Deviations and open items

- `evaluate_all` returns `Resolved` (adds `order`) instead of a bare tuple.
- Not run: `clippy` (component not installed for the pinned toolchain); `nx show project` timed out (daemon), so Nx discovery of `project.json` is unverified (the
  file mirrors the geometry project). `.vscode/launch.json` was not touched (sibling framework crates are not registered individually there).
- `🧰️framework/Cargo.toml` (member + `[workspace.dependencies]`) and repo-root `Cargo.toml` (`[workspace.dependencies]`) gained one row each; `🧰️framework/Cargo.lock` was updated by cargo.
- Pre-existing breakage elsewhere (os-kernel, commit 677) did not touch this crate (no dependencies).
