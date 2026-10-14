# BIM Omitted Field Execution Map

Read-only, no builds. Base is `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. All16 maps are declared persisted original ModelSnapshot authority, absent from current explicit Rust P/R,93-table DDL and TypeScript SQLite P/R names. No default fallback should invent them.

| Field | Exact entity fields | Rust original schema owner | Existing SQL support |
|---|---|---|---|
| supports | name: String; member: String; location: StructuralLocation; offset: Point3; restraints: Restraints | `🧬️schema/📸️snapshot/🦴️structure/🦀️.rs` | No dedicated authored table/visitor |
| load_cases | name: String; category: String; factor: f64 | `🧬️schema/📸️snapshot/🦴️structure/🦀️.rs` | No dedicated authored table/visitor |
| loads | name: String; load_case: String; member: String; location: StructuralLocation; force: Point3; moment: Point3 | `🧬️schema/📸️snapshot/🦴️structure/🦀️.rs` | No dedicated authored table/visitor |
| option_groups | name: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| design_options | group: String; name: String; primary: bool | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| worksets | name: String; default_visible: bool | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| element_options | target: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| element_worksets | target: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| space_conditions | occupancy: Option<String>; occupancy_density: Option<f64>; heating_setpoint: Option<f64>; cooling_setpoint: Option<f64>; ventilation_rate: Option<f64>; lighting_power_density: Option<f64>; equipment_power_density: Option<f64>; schedule: Option<String> | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| clash_sets | name: String; a: ElementSelector; b: ElementSelector; tolerance: f64; clearance: f64 | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| rules | name: String; kind: RuleKind; limit: f64; severity: RuleSeverity; scope: RuleScope | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| issues | title: String; description: String; status: IssueStatus; priority: IssuePriority; assignee: String; author: String; created: String; labels: Vec<String>; elements: Vec<String>; clash: Option<ClashRef>; viewpoint: Option<IssueViewpoint> | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| issue_comments | issue: String; author: String; date: String; text: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| components | storey: String; family: String; position: Point2; elevation: f64; rotation: f64; mirrored: bool; host: Option<String>; system: Option<MepSystem>; name: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| component_overrides | component: String; name: String; value: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |
| mep_elements | storey: String; system: MepSystem; shape: MepShape; path: Vec<Point3>; name: String | `🧬️schema/📸️snapshot/🧱️entities/🦀️.rs` | No dedicated authored table/visitor |

## Relationships and Handcrafted Tables

Every new map needs document_id, stable ordinal and literal map_key. Structural support/load member targets must retain original member IDs and StructuralLocation tag Point(station)/Line(start,end)/Area; force/moment/offset require original Point3 axes and exact scalar word/class. load_cases factor is scalar. DesignOption.group targets option_groups; both membership maps use element map key and target group/workset string. SpaceConditions is keyed by space and all eight fields are independently optional. No missing option should become an invented default.

ClashSet a/b each preserve ElementSelector; Rule scope preserves RuleScope; Issue labels/elements are separate ordered children, optional ClashRef and IssueViewpoint are authored tagged/optional children. IssueComment.issue is an issue key. Component family/storey/optional host/system retain exact strings and tags, position is Point2. ComponentOverride component/name/value preserve literal formula. MepElement.path is ordered Point3 children; MepShape tags and variant parameters must be normalized, not opaque JSON. Value nested owners live in `🧬️schema/📸️snapshot/💠️values/🦀️.rs` (MepShape:543, ElementSelector:891, RuleScope:905, ClashRef:919, IssueViewpoint:941).

## Actual Neutral and Language Mounts

Existing semantic fixture `🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json` names current47maps-allreachablevariants; it is not a complete65-field witness. Existing limit/control/count fixtures sit alongside it. Add a new independently authored complete65 fixture there with every16 map nonempty and connected references; update authored counts and actual input schema. Avoid reuse of Rust-generated snapshot as oracle. Existing inference fixtures under `🧫️fixtures/💡️inferences/` cover structural/family/domain behavior but are not a complete SQLite identity/custody law.

Rust actual semantic provider is `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`; TypeScript implementation is sibling `🟦️.ts` and table schema sibling `🗄️schema/🟦️.ts`; original TS snapshot type is `🧬️schema/📸️snapshot/🟦️.ts`. All require coherent hand edits. Rust ArtifactPack `🚪️io/💾️binary/📸️snapshot/🦀️.rs` also needs actual sqlite_snapshot_codec registration. Existing public Rust laws are `🚪️io/🧪️tests/🪶️sqlite/🦀️.rs`; semantic Source laws are `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`. Neither source presence nor counts imply passes.

First meaningful red witness: complete65 fixture -> exact independent SQLite rows -> TypeScript restore comparing all original fields; require explicit tables for all16 before Rust execution. Mutate a load case factor, design membership target, issue comment literal, component override formula and MEP path point through independent SQL, then actual provider import must yield those edits. Empty maps cannot prove preservation. Original map keys distinct from entity names, optional absent/present and Unicode/NUL strings should be included. Native wholefactory laws follow current authentic build route after bootstrap green; no synthetic narrowed owner.
