//! 🦀️ EN 1997 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1997-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1997` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_en1997_mutation` over the full 22-kind `En1997Mutation` vocabulary.
//!
//! Twenty-two document-root scalars and enums, one `change-<field>` each, covering two
//! independent check families that share one ground model. The shallow-footing family carries
//! the actions V_Ed and H_Ed, the footing area, the ground parameters phi, c and gamma, the
//! footing width B, the embedment depth D_f, the stiffness E_s and Poisson's ratio nu, the
//! settlement limit, and the design approach. The pile family carries N_Ed, the shaft factor
//! alpha_s, the pile diameter and length, the shaft and base resistances q_s and q_b, the base
//! area, the profile count and the investigated depth. `change-annex` and
//! `change-design-approach` sit above both.
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
//! (`decode_en1997_snapshot_json`/`encode_en1997_snapshot_json`,
//! `decode_en1997_dsl`/`encode_en1997_dsl`, `decode_en1997_pack`/`encode_en1997_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1997_mutation_json`, `apply_en1997_mutation`, `inverse_en1997_mutation` in
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
/// 🏷️ Mirrors `En1997Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-footing-area-m2",
    "change-phi-deg",
    "change-c-kpa",
    "change-gamma-kn-m3",
    "change-bm",
    "change-dfm",
    "change-es-mpa",
    "change-nu",
    "change-design-approach",
    "change-annex",
    "change-settlement-limit-mm",
    "change-n-pile-ed-kn",
    "change-alpha-s",
    "change-pile-dm",
    "change-qs-kpa",
    "change-pile-lm",
    "change-qb-kpa",
    "change-pile-base-area-m2",
    "change-pile-n-profiles",
    "change-z-investigated-m",
];

/// 🗣️ The real committed EN 1997 document, read where the domain already keeps it.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-v-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🧪️tests/raises-the-design-vertical-load-to-750-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🧪️tests/raises-the-design-vertical-load-to-750-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🧪️tests/raises-the-design-vertical-load-to-750-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-v-ed-kn/🧪️tests/raises-the-design-vertical-load-to-750-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-h-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🧪️tests/raises-the-design-horizontal-load-to-120-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🧪️tests/raises-the-design-horizontal-load-to-120-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🧪️tests/raises-the-design-horizontal-load-to-120-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-h-ed-kn/🧪️tests/raises-the-design-horizontal-load-to-120-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-footing-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🧪️tests/enlarges-the-footing-area-to-6-25-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🧪️tests/enlarges-the-footing-area-to-6-25-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🧪️tests/enlarges-the-footing-area-to-6-25-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-footing-area-m2/🧪️tests/enlarges-the-footing-area-to-6-25-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-phi-deg" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🧪️tests/raises-the-friction-angle-to-35-degrees/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🧪️tests/raises-the-friction-angle-to-35-degrees/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🧪️tests/raises-the-friction-angle-to-35-degrees/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-phi-deg/🧪️tests/raises-the-friction-angle-to-35-degrees/🎯️outcome/🔣️component.json"),
        ),
        "change-c-kpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🧪️tests/gives-the-drained-sand-12-5-kpa-of-effective-cohesion/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🧪️tests/gives-the-drained-sand-12-5-kpa-of-effective-cohesion/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🧪️tests/gives-the-drained-sand-12-5-kpa-of-effective-cohesion/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-c-kpa/🧪️tests/gives-the-drained-sand-12-5-kpa-of-effective-cohesion/🎯️outcome/🔣️component.json"),
        ),
        "change-gamma-kn-m3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🧪️tests/raises-the-soil-unit-weight-to-20-kn-m3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🧪️tests/raises-the-soil-unit-weight-to-20-kn-m3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🧪️tests/raises-the-soil-unit-weight-to-20-kn-m3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-gamma-kn-m3/🧪️tests/raises-the-soil-unit-weight-to-20-kn-m3/🎯️outcome/🔣️component.json"),
        ),
        "change-bm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🧪️tests/widens-the-footing-to-2-5-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🧪️tests/widens-the-footing-to-2-5-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🧪️tests/widens-the-footing-to-2-5-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-bm/🧪️tests/widens-the-footing-to-2-5-m/🎯️outcome/🔣️component.json"),
        ),
        "change-dfm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🧪️tests/deepens-the-founding-level-to-2-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🧪️tests/deepens-the-founding-level-to-2-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🧪️tests/deepens-the-founding-level-to-2-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-dfm/🧪️tests/deepens-the-founding-level-to-2-m/🎯️outcome/🔣️component.json"),
        ),
        "change-es-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🧪️tests/stiffens-the-soil-modulus-to-45-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🧪️tests/stiffens-the-soil-modulus-to-45-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🧪️tests/stiffens-the-soil-modulus-to-45-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-es-mpa/🧪️tests/stiffens-the-soil-modulus-to-45-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-nu" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🧪️tests/raises-poissons-ratio-to-0-375/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🧪️tests/raises-poissons-ratio-to-0-375/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🧪️tests/raises-poissons-ratio-to-0-375/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪞change-nu/🧪️tests/raises-poissons-ratio-to-0-375/🎯️outcome/🔣️component.json"),
        ),
        "change-design-approach" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🧪️tests/switches-from-design-approach-1-to-design-approach-2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🧪️tests/switches-from-design-approach-1-to-design-approach-2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🧪️tests/switches-from-design-approach-1-to-design-approach-2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛋️change-design-approach/🧪️tests/switches-from-design-approach-1-to-design-approach-2/🎯️outcome/🔣️component.json"),
        ),
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛏️change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/🎯️outcome/🔣️component.json"),
        ),
        "change-settlement-limit-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🧪️tests/relaxes-the-settlement-limit-to-40-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🧪️tests/relaxes-the-settlement-limit-to-40-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🧪️tests/relaxes-the-settlement-limit-to-40-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚿change-settlement-limit-mm/🧪️tests/relaxes-the-settlement-limit-to-40-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-n-pile-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🧪️tests/raises-the-design-pile-axial-load-to-1200-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🧪️tests/raises-the-design-pile-axial-load-to-1200-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🧪️tests/raises-the-design-pile-axial-load-to-1200-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛁change-n-pile-ed-kn/🧪️tests/raises-the-design-pile-axial-load-to-1200-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-alpha-s" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🧪️tests/lowers-the-shaft-resistance-factor-to-0-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🧪️tests/lowers-the-shaft-resistance-factor-to-0-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🧪️tests/lowers-the-shaft-resistance-factor-to-0-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿change-alpha-s/🧪️tests/lowers-the-shaft-resistance-factor-to-0-5/🎯️outcome/🔣️component.json"),
        ),
        "change-pile-dm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🧪️tests/enlarges-the-pile-diameter-to-0-75-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🧪️tests/enlarges-the-pile-diameter-to-0-75-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🧪️tests/enlarges-the-pile-diameter-to-0-75-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍀change-pile-dm/🧪️tests/enlarges-the-pile-diameter-to-0-75-m/🎯️outcome/🔣️component.json"),
        ),
        "change-qs-kpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🧪️tests/raises-the-unit-shaft-resistance-to-120-kpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🧪️tests/raises-the-unit-shaft-resistance-to-120-kpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🧪️tests/raises-the-unit-shaft-resistance-to-120-kpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌾change-qs-kpa/🧪️tests/raises-the-unit-shaft-resistance-to-120-kpa/🎯️outcome/🔣️component.json"),
        ),
        "change-pile-lm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🧪️tests/lengthens-the-pile-to-15-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🧪️tests/lengthens-the-pile-to-15-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🧪️tests/lengthens-the-pile-to-15-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌵change-pile-lm/🧪️tests/lengthens-the-pile-to-15-m/🎯️outcome/🔣️component.json"),
        ),
        "change-qb-kpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🧪️tests/raises-the-unit-base-resistance-to-3200-kpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🧪️tests/raises-the-unit-base-resistance-to-3200-kpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🧪️tests/raises-the-unit-base-resistance-to-3200-kpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌴change-qb-kpa/🧪️tests/raises-the-unit-base-resistance-to-3200-kpa/🎯️outcome/🔣️component.json"),
        ),
        "change-pile-base-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🧪️tests/doubles-the-pile-base-area-to-0-5-m2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🧪️tests/doubles-the-pile-base-area-to-0-5-m2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🧪️tests/doubles-the-pile-base-area-to-0-5-m2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌳change-pile-base-area-m2/🧪️tests/doubles-the-pile-base-area-to-0-5-m2/🎯️outcome/🔣️component.json"),
        ),
        "change-pile-n-profiles" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🧪️tests/adds-a-third-investigated-ground-profile/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🧪️tests/adds-a-third-investigated-ground-profile/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🧪️tests/adds-a-third-investigated-ground-profile/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌲change-pile-n-profiles/🧪️tests/adds-a-third-investigated-ground-profile/🎯️outcome/🔣️component.json"),
        ),
        "change-z-investigated-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🧪️tests/deepens-the-investigated-depth-to-12-m/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🧪️tests/deepens-the-investigated-depth-to-12-m/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🧪️tests/deepens-the-investigated-depth-to-12-m/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍁change-z-investigated-m/🧪️tests/deepens-the-investigated-depth-to-12-m/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1997-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1997::standards::v1::subsets::any::schema::mutations::{apply_en1997_mutation, decode_en1997_mutation_json, inverse_en1997_mutation, En1997Mutation};
    use semio_s_plugin_norm::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::{decode_en1997_dsl, decode_en1997_pack, decode_en1997_snapshot_json, encode_en1997_dsl, encode_en1997_pack, encode_en1997_snapshot_json, En1997Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1997Snapshot, String> {
        decode_en1997_snapshot_json(text).map_err(|error| format!("mutate-en1997-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1997Mutation, String> {
        decode_en1997_mutation_json(text).map_err(|error| format!("mutate-en1997-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1997Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1997_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1997Snapshot, expected: &En1997Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1997_snapshot_json(got), encode_en1997_snapshot_json(expected))
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
            let applied = apply_en1997_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1997_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1997_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let steps = inverse_en1997_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1997_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1997 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1997_dsl(&text)?;
        let reprinted = encode_en1997_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1997_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1997_pack(&encode_en1997_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1997_snapshot_json(&encode_en1997_snapshot_json(&parsed))?;
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
