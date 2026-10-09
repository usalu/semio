# ⚖️ Decision Record — Parametric Families (WP-23), Furniture/Equipment/MEP (WP-10)

Status: **accepted** by the coordinator (2026-10-09). Input: `r11-explore-families.md`, `r9-decisions.md` §4.

## Decision
1. **Families are authored data.** New collection `families: BTreeMap<Id, Family>`. A family has `name`, `category`
   (`Furniture`, `Equipment`, `Casework`, `Plumbing`, `Lighting`, `Mechanical`, `Electrical`, `Generic`, `Profile`),
   `parameters: BTreeMap<Name, FamilyParameter { kind: Length|Angle|Real|Integer|Boolean|Text|Material, value: Expr }>`
   and `solids: BTreeMap<Id, FamilySolid>` (keyed, independently addressable — not a whole list).
   `FamilySolid` = `Extrusion { profile: ParametricProfile, base: Expr, height: Expr }` | `Revolution { profile, axis,
   angle: Expr }` | `Sweep { profile, path: Vec<ExprPoint> }` | `Box { x, y, z, width, depth, height: Expr }`, each with
   `material: Expr` (a Material parameter or literal id), `visible: Expr` (Boolean) and an offset `ExprPoint3`.
   `ParametricProfile` = Rectangle/Circle/Polygon with `Expr` dimensions.
2. **Formulas are authored, values are inferred.** `Expr` is a closed, typed expression tree: `Number`, `Length`,
   `Angle`, `Bool`, `Text`, `Param(name)`, `Unary`, `Binary(+ − × ÷ ^ min max)`, `Compare`, `And/Or/Not`, `If`,
   `Call(sqrt, abs, round, floor, ceil, sin, cos, tan, atan2)`. The resolved parameter values, solid geometry and
   diagnostics (cycle, unknown parameter, unit/kind mismatch, negative dimension, division by zero) are the inference
   field `🧬️families` (per family/type/instance key in the model graph). Nothing evaluated is ever stored.
3. **The expression language is domain-neutral → framework module** `🧰️framework/🔨️modules/🧮️expression` (new crate
   `semio-framework-expression`, zero runtime deps beyond `semio-framework-value`/`-number` if needed): typed AST,
   dimensional kinds (length/angle/area/volume/number/bool/text) with checking, evaluator, `dependencies(expr)`,
   cycle-safe evaluation order over named parameters, and a text syntax parser + printer (round-trip stable) for typing
   formulas in the UI and the DSL text IO. Third-party oracle: python (e.g. `sympy`/`ast`-based evaluator in `.venv`
   test group) produces identical values for a fixture corpus; language-agnostic `.feature`.
4. **Instances.** Furniture, equipment and generic components are placed as `components: BTreeMap<Id, Component {
   storey, family, position, rotation, elevation, mirrored, overrides: BTreeMap<Name, Expr>, host: Option<Id> }>`.
   Overrides replace family parameter formulas per instance; types are families (no separate furniture-type record).
5. **MEP placeholders** are `mep: BTreeMap<Id, MepElement { storey, system: Supply|Return|Exhaust|DomesticWater|Waste|
   Gas|Power|Data|Lighting, shape: Duct{width,height}|Pipe{diameter}|Tray{width,height}, path: Vec<Point3>, name }>`
   plus `mep_terminals` (families with a system connector). Solids/plan inferred; system colours in plan and 3D.
6. **Window/door/column/beam types stay typed records** (fast path). `Profile` gains `Family { family }` (category
   `Profile`) so columns, beams, mullions and railings can use a parametric profile family; the profile outline is the
   family's inferred profile.
7. **Mutations** (approved verbs): `create-family`, `set-family` (name/category), `delete-family` (refuse while used),
   `set-family-parameter`, `remove-family-parameter` (refuse while referenced by another parameter's formula or a solid —
   computed from authored expressions only, no inference in diff), `create-family-solid`, `set-family-solid`,
   `delete-family-solid`; `create-component`, `set-component`, `set-component-override`, `remove-component-override`,
   `delete-component`; `create-mep-element`, `set-mep-element`, `delete-mep-element`. All sparse diffs, concrete inverses,
   sum laws, cross-kind id uniqueness, cascades of properties/classifications.
8. **UI.** Family editor window (parameter table with formula entry + live evaluated value column + diagnostics; solids
   list; 3D preview), component place tool (library of families), override rows in properties, MEP route tool.
9. **IO.** IFC: components → `IfcFurniture`/`IfcFlowTerminal`/`IfcBuildingElementProxy` with `IfcFurnitureType` etc. and
   tessellated geometry; MEP → `IfcDuctSegment`/`IfcPipeSegment`/`IfcCableCarrierSegment`; glTF/SVG via solids.

## Amendment (after F1)
Formulas are stored in the snapshot as canonical text (`semio_framework_expression::print` output); the editor parses
and canonicalises before emitting a mutation (parse errors refuse in the UI, never reach the snapshot); a diff refuses a
formula that does not parse (`mutation.formula-invalid`, pure check, no evaluation). Inference parses + evaluates
(`evaluate_all` → `Resolved { order, values, errors }`); kind/cycle/unknown-parameter errors become diagnostics. API:
`T/r12-exec-w2-f1-expression.md`.

## Consequences
WP-10 and WP-23 are implemented together in wave W2 as three sequential packages: F1 framework expression crate →
F2 families (schema, mutations, inference, family editor) → F3 components + MEP (+ IO). r9-decisions §4 is superseded.
