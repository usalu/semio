//! 🦀️ ISO 16757 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `iso16757-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.iso16757` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_iso16757_mutation` over the full 21-kind `Iso16757Mutation`
//! vocabulary.
//!
//! This is the RICHEST document shape in the plugin and the only one besides `📔️vdi3805` whose
//! vocabulary is a lifecycle rather than a parameter form. `Iso16757Snapshot` is a
//! multi-collection document — catalogue, dictionary, geometry, selection, part-number rule,
//! part-number inputs, script limits and exchange process — and the twenty-one kinds split into
//! three genuinely different families: document-root scalars (`change-exchange-process`,
//! `update-script-limits`, `replace-part-number-rule`, `change`/`remove-part-number-input`,
//! `change-selection-class`, `change-selection-series`), ordered constraint edits on the
//! selection facet (`add`/`remove-selection-constraint`), and full create/delete(+rename)
//! lifecycles over four id-keyed collections — the catalogue's `product_groups` and `products`,
//! its `property_definitions`, and the dictionary's `subjects`.
//!
//! ⚖️ WHERE THE ASSERTIONS LIVE. A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has
//! none — so the comparison profile never gets two sides to compare. Every law this case claims
//! is therefore asserted IN ROLE inside the subject handlers, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`,
//! `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the
//! stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin
//! declares in `✏️s/🔌️plugins/📕️norm/🧪️oracle/🔣️component.json`. The oracle handlers still
//! assert what a committed vector can prove on its own: that an `applied` vector genuinely
//! moves the document and a `rejected` one genuinely does not.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only
//! `semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate;
//! `serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are all
//! unreachable from here. The subset's own production code therefore exports the bridges
//! (`decode_iso16757_snapshot_json`/`encode_iso16757_snapshot_json`,
//! `decode_iso16757_dsl`/`encode_iso16757_dsl`, `decode_iso16757_pack`/`encode_iso16757_pack`
//! in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_iso16757_mutation_json`, `apply_iso16757_mutation`, `inverse_iso16757_mutation` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`), whose
//! signatures name only reachable types. Both roles read the SAME committed bytes — the oracle
//! role via `include_str!`, the subject role by decoding that same text — so a fixture can
//! never drift away from a Rust literal transcribed beside it, because there is none.
//!
//! 🚧️ The Rust SUBJECT phase cannot run at the time of writing: `semio-s-plugin-norm` does not
//! compile (a concurrent session is mid-flight removing gratuitous `async fn` wrappers across
//! the crate), and `semio-framework-os-kernel` is red for the same reason. The subject half is
//! written against the SYNC trait surface the fixture tests in this crate already call
//! (`Mutation::diff`, `MutationDiff::apply`, `Mutation::inverse`, `ArtifactDsl`,
//! `ArtifactPack`) rather than against the plugin's async wrappers, and is `sut`-gated so the
//! oracle-only run never links it.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `Iso16757Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "change-exchange-process",
    "update-script-limits",
    "replace-part-number-rule",
    "change-part-number-input",
    "remove-part-number-input",
    "change-selection-class",
    "change-selection-series",
    "add-selection-constraint",
    "remove-selection-constraint",
    "rename-catalogue",
    "rename-manufacturer",
    "create-product-group",
    "delete-product-group",
    "rename-product-group",
    "create-product",
    "delete-product",
    "rename-product",
    "create-property-definition",
    "delete-property-definition",
    "create-subject",
    "delete-subject",
];

/// 🗣️ The real committed ISO 16757 document, read where the domain already keeps it.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-exchange-process" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🧪️tests/advances-the-exchange-stage-to-determine-product/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🧪️tests/advances-the-exchange-stage-to-determine-product/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🧪️tests/advances-the-exchange-stage-to-determine-product/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍃change-exchange-process/🧪️tests/advances-the-exchange-stage-to-determine-product/🎯️outcome/🔣️component.json"),
        ),
        "update-script-limits" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🧪️tests/doubles-the-step-budget-and-quintuples-the-timeout/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🧪️tests/doubles-the-step-budget-and-quintuples-the-timeout/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🧪️tests/doubles-the-step-budget-and-quintuples-the-timeout/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌷update-script-limits/🧪️tests/doubles-the-step-budget-and-quintuples-the-timeout/🎯️outcome/🔣️component.json"),
        ),
        "replace-part-number-rule" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🧪️tests/swaps-the-literal-rule-for-a-height-driven-script/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🧪️tests/swaps-the-literal-rule-for-a-height-driven-script/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🧪️tests/swaps-the-literal-rule-for-a-height-driven-script/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍂replace-part-number-rule/🧪️tests/swaps-the-literal-rule-for-a-height-driven-script/🎯️outcome/🔣️component.json"),
        ),
        "change-part-number-input" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🧪️tests/raises-the-height-part-number-input-to-750/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🧪️tests/raises-the-height-part-number-input-to-750/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🧪️tests/raises-the-height-part-number-input-to-750/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱change-part-number-input/🧪️tests/raises-the-height-part-number-input-to-750/🎯️outcome/🔣️component.json"),
        ),
        "remove-part-number-input" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🧪️tests/drops-the-length-part-number-input/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🧪️tests/drops-the-length-part-number-input/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🧪️tests/drops-the-length-part-number-input/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿remove-part-number-input/🧪️tests/drops-the-length-part-number-input/🎯️outcome/🔣️component.json"),
        ),
        "change-selection-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🧪️tests/retargets-the-selection-at-the-towel-radiator-class/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🧪️tests/retargets-the-selection-at-the-towel-radiator-class/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🧪️tests/retargets-the-selection-at-the-towel-radiator-class/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-selection-class/🧪️tests/retargets-the-selection-at-the-towel-radiator-class/🎯️outcome/🔣️component.json"),
        ),
        "change-selection-series" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🧪️tests/narrows-the-selection-to-the-pr-plus-series/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🧪️tests/narrows-the-selection-to-the-pr-plus-series/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🧪️tests/narrows-the-selection-to-the-pr-plus-series/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-selection-series/🧪️tests/narrows-the-selection-to-the-pr-plus-series/🎯️outcome/🔣️component.json"),
        ),
        "add-selection-constraint" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🧪️tests/appends-a-width-under-800-constraint/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🧪️tests/appends-a-width-under-800-constraint/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🧪️tests/appends-a-width-under-800-constraint/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁add-selection-constraint/🧪️tests/appends-a-width-under-800-constraint/🎯️outcome/🔣️component.json"),
        ),
        "remove-selection-constraint" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🧪️tests/drops-the-trailing-length-constraint/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🧪️tests/drops-the-trailing-length-constraint/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🧪️tests/drops-the-trailing-length-constraint/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️remove-selection-constraint/🧪️tests/drops-the-trailing-length-constraint/🎯️outcome/🔣️component.json"),
        ),
        "rename-catalogue" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🧪️tests/restamps-the-catalogue-as-the-2026-edition/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🧪️tests/restamps-the-catalogue-as-the-2026-edition/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🧪️tests/restamps-the-catalogue-as-the-2026-edition/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲rename-catalogue/🧪️tests/restamps-the-catalogue-as-the-2026-edition/🎯️outcome/🔣️component.json"),
        ),
        "rename-manufacturer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🧪️tests/adds-the-ag-suffix-to-the-manufacturer/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🧪️tests/adds-the-ag-suffix-to-the-manufacturer/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🧪️tests/adds-the-ag-suffix-to-the-manufacturer/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳rename-manufacturer/🧪️tests/adds-the-ag-suffix-to-the-manufacturer/🎯️outcome/🔣️component.json"),
        ),
        "create-product-group" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🧪️tests/appends-a-towel-radiators-group/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🧪️tests/appends-a-towel-radiators-group/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🧪️tests/appends-a-towel-radiators-group/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀create-product-group/🧪️tests/appends-a-towel-radiators-group/🎯️outcome/🔣️component.json"),
        ),
        "delete-product-group" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🧪️tests/removes-the-radiators-group-and-strands-its-class/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🧪️tests/removes-the-radiators-group-and-strands-its-class/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🧪️tests/removes-the-radiators-group-and-strands-its-class/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌹delete-product-group/🧪️tests/removes-the-radiators-group-and-strands-its-class/🎯️outcome/🔣️component.json"),
        ),
        "rename-product-group" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🧪️tests/renames-the-radiators-group-to-panel-radiators/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🧪️tests/renames-the-radiators-group-to-panel-radiators/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🧪️tests/renames-the-radiators-group-to-panel-radiators/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿rename-product-group/🧪️tests/renames-the-radiators-group-to-panel-radiators/🎯️outcome/🔣️component.json"),
        ),
        "create-product" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🧪️tests/appends-a-pr900-product-to-the-existing-series/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🧪️tests/appends-a-pr900-product-to-the-existing-series/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🧪️tests/appends-a-pr900-product-to-the-existing-series/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁create-product/🧪️tests/appends-a-pr900-product-to-the-existing-series/🎯️outcome/🔣️component.json"),
        ),
        "delete-product" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🧪️tests/removes-the-pr600-product-from-the-catalogue/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🧪️tests/removes-the-pr600-product-from-the-catalogue/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🧪️tests/removes-the-pr600-product-from-the-catalogue/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌸delete-product/🧪️tests/removes-the-pr600-product-from-the-catalogue/🎯️outcome/🔣️component.json"),
        ),
        "rename-product" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🧪️tests/renames-pr600-to-the-compact-variant-name/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🧪️tests/renames-pr600-to-the-compact-variant-name/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🧪️tests/renames-pr600-to-the-compact-variant-name/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️rename-product/🧪️tests/renames-pr600-to-the-compact-variant-name/🎯️outcome/🔣️component.json"),
        ),
        "create-property-definition" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🧪️tests/appends-a-selection-scoped-length-property/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🧪️tests/appends-a-selection-scoped-length-property/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🧪️tests/appends-a-selection-scoped-length-property/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾create-property-definition/🧪️tests/appends-a-selection-scoped-length-property/🎯️outcome/🔣️component.json"),
        ),
        "delete-property-definition" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🧪️tests/removes-the-height-property-definition/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🧪️tests/removes-the-height-property-definition/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🧪️tests/removes-the-height-property-definition/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌺delete-property-definition/🧪️tests/removes-the-height-property-definition/🎯️outcome/🔣️component.json"),
        ),
        "create-subject" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🧪️tests/appends-a-towel-radiator-subject-under-the-radiator-parent/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🧪️tests/appends-a-towel-radiator-subject-under-the-radiator-parent/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🧪️tests/appends-a-towel-radiator-subject-under-the-radiator-parent/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵create-subject/🧪️tests/appends-a-towel-radiator-subject-under-the-radiator-parent/🎯️outcome/🔣️component.json"),
        ),
        "delete-subject" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🧪️tests/removes-the-radiator-subject-from-the-dictionary/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🧪️tests/removes-the-radiator-subject-from-the-dictionary/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🧪️tests/removes-the-radiator-subject-from-the-dictionary/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌻delete-subject/🧪️tests/removes-the-radiator-subject-from-the-dictionary/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-iso16757-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️component.json` declares for one kind — `applied` or
/// `rejected` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. The one law a
/// committed pair can carry on its own is asserted here in role — an `applied` vector must MOVE the
/// document and a `rejected` vector must leave it identical — so a placeholder fixture that changed
/// nothing could not sit in this table unnoticed.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, after, _outcome) = fixture_text(kind);
        let (base, projection) = (canonical(before), canonical(after));
        match committed_status(kind).as_str() {
            "applied" => law::mutation_is_observable(kind, &projection, &base, &[])?,
            "rejected" if law::divergence(&projection, &base).is_some() => {
                return Err(format!("mutate-{kind}: the committed outcome declares this vector rejected, so its after-snapshot must be identical to its before-snapshot"));
            }
            "rejected" => {}
            other => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
        }
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), projection))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started. The inverse LAW itself cannot be
/// asserted from the committed vectors alone (nothing here computes an inverse), which is precisely
/// why it is asserted in the subject handler below instead.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_norm::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::{apply_iso16757_mutation, decode_iso16757_mutation_json, inverse_iso16757_mutation, Iso16757Mutation};
    use semio_s_plugin_norm::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::{decode_iso16757_dsl, decode_iso16757_pack, decode_iso16757_snapshot_json, encode_iso16757_dsl, encode_iso16757_pack, encode_iso16757_snapshot_json, Iso16757Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<Iso16757Snapshot, String> {
        decode_iso16757_snapshot_json(text).map_err(|error| format!("mutate-iso16757-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<Iso16757Mutation, String> {
        decode_iso16757_mutation_json(text).map_err(|error| format!("mutate-iso16757-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &Iso16757Snapshot) -> Result<Json, String> {
        parse_json(&encode_iso16757_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &Iso16757Snapshot, expected: &Iso16757Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_iso16757_snapshot_json(got), encode_iso16757_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot, under whichever contract the committed `🎯️outcome` declares: an `applied`
    /// vector must be accepted without a diagnostic and must move the projection (`law::
    /// mutation_is_observable`), a `rejected` one must raise a diagnostic and leave the document
    /// bit-identical. A handler that merely returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let status = super::committed_status(kind);
            let applied = apply_iso16757_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_iso16757_snapshot_json(&snapshot))),
                ("rejected", Ok((snapshot, _messages))) => snapshot,
                ("rejected", Err(_error)) => base.clone(),
                (other, _) => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
            };
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(kind, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{kind}: a rejected mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// ↩️ The metamorphic inverse law, asserted in role through `law::inverse_restores`: applying the
    /// kind and then its OWN computed inverse must restore the committed before-snapshot exactly —
    /// collection POSITION included, not merely membership. A kind the committed outcome declares
    /// `applied` must additionally produce a non-empty inverse, because a mutation that changes the
    /// document and reports nothing to undo silently breaks the event-sourced undo history.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let original = projection(&base)?;
            let mut current = match apply_iso16757_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let steps = inverse_iso16757_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_iso16757_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            law::inverse_restores(kind, &projection(&current)?, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            Ok(Outcome::with_raw(original.to_string().into_bytes(), original))
        }
    }

    /// 🔁️ The real committed document through every encoding it has. The DSL carrier is deliberately
    /// byte-preserving here — the committed file IS this printer's own canonical output — so
    /// `law::carrier_is_exact` is the correct half of the identity law and the usual
    /// no-byte-pass-through inequality would be the wrong claim. What proves the document was PARSED
    /// rather than copied is the agreement of three independently written codecs: the hand-written
    /// DSL grammar, the hand-written binary pack protocol, and the JSON projection. A shortcut that
    /// handed back its input bytes could not survive the pack leg.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed ISO 16757 artifact is not UTF-8: {error}"))?;
        let parsed = decode_iso16757_dsl(&text)?;
        let reprinted = encode_iso16757_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_iso16757_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_iso16757_pack(&encode_iso16757_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_iso16757_snapshot_json(&encode_iso16757_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let projection = projection(&parsed)?;
        law::round_trip_preserves(&projection(&repacked)?, &projection)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// document the oracle role can read literally, but the real artifact is committed as DSL bytes
/// ONLY, and turning those into a document needs this subset's own codec — which the oracle-only
/// build must not link.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
