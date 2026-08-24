//! 🦀️ EN 1995 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1995-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1995` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_en1995_mutation` over the full 20-kind `En1995Mutation` vocabulary.
//!
//! Twenty document-root scalars, one `change-<field>` each: national annex, the design actions
//! M_Ed, N_Ed and V_Ed, the section properties W, A, b and h, the characteristic strengths
//! f_m,k, f_c,0,k and f_v,k, the two classification enums that drive k_mod and k_def — service
//! class and load-duration class — the lateral-torsional M_crit, the connection inputs F_Ed and
//! A_ef, the fire inputs (duration and section depth) and the footbridge vibration inputs
//! (vertical acceleration and bridge cycle count).
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
//! (`decode_en1995_snapshot_json`/`encode_en1995_snapshot_json`,
//! `decode_en1995_dsl`/`encode_en1995_dsl`, `decode_en1995_pack`/`encode_en1995_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1995_mutation_json`, `apply_en1995_mutation`, `inverse_en1995_mutation` in
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
/// 🏷️ Mirrors `En1995Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "change-annex",
    "change-m-ed-knm",
    "change-n-ed-kn",
    "change-v-ed-kn",
    "change-w-mm3",
    "change-a-mm2",
    "change-b-mm",
    "change-h-mm",
    "change-fmk",
    "change-fc0-k",
    "change-service-class",
    "change-load-duration",
    "change-m-crit-knm",
    "change-f-ed-kn",
    "change-a-ef-mm2",
    "change-fvk",
    "change-fire-duration-min",
    "change-section-depth-mm",
    "change-a-vert-ms2",
    "change-n-cycles-bridge",
];

/// 🗣️ The real committed EN 1995 document, read where the domain already keeps it.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️glulam-footbridge/🖼️assets/🗣️glulam-footbridge.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️glulam-footbridge/🖼️assets/🎒️glulam-footbridge.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-annex/🧪️tests/switches-from-the-german-na-to-the-recommended-en-annex/🎯️outcome/🔣️component.json"),
        ),
        "change-m-ed-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🧪️tests/raises-the-design-bending-moment-to-32-knm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🧪️tests/raises-the-design-bending-moment-to-32-knm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🧪️tests/raises-the-design-bending-moment-to-32-knm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧴change-m-ed-knm/🧪️tests/raises-the-design-bending-moment-to-32-knm/🎯️outcome/🔣️component.json"),
        ),
        "change-n-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🧪️tests/raises-the-design-axial-force-to-75-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🧪️tests/raises-the-design-axial-force-to-75-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🧪️tests/raises-the-design-axial-force-to-75-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽change-n-ed-kn/🧪️tests/raises-the-design-axial-force-to-75-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-v-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🧪️tests/raises-the-design-shear-force-to-22-5-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🧪️tests/raises-the-design-shear-force-to-22-5-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🧪️tests/raises-the-design-shear-force-to-22-5-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺change-v-ed-kn/🧪️tests/raises-the-design-shear-force-to-22-5-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-w-mm3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🧪️tests/raises-the-section-modulus-to-4000000-mm3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🧪️tests/raises-the-section-modulus-to-4000000-mm3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🧪️tests/raises-the-section-modulus-to-4000000-mm3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪑change-w-mm3/🧪️tests/raises-the-section-modulus-to-4000000-mm3/🎯️outcome/🔣️component.json"),
        ),
        "change-a-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🧪️tests/enlarges-the-gross-area-to-72000-mm2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🧪️tests/enlarges-the-gross-area-to-72000-mm2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🧪️tests/enlarges-the-gross-area-to-72000-mm2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-a-mm2/🧪️tests/enlarges-the-gross-area-to-72000-mm2/🎯️outcome/🔣️component.json"),
        ),
        "change-b-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🧪️tests/widens-the-beam-to-240-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🧪️tests/widens-the-beam-to-240-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🧪️tests/widens-the-beam-to-240-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-b-mm/🧪️tests/widens-the-beam-to-240-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-h-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🧪️tests/deepens-the-beam-to-360-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🧪️tests/deepens-the-beam-to-360-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🧪️tests/deepens-the-beam-to-360-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-h-mm/🧪️tests/deepens-the-beam-to-360-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-fmk" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🧪️tests/upgrades-the-bending-strength-class-to-28-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🧪️tests/upgrades-the-bending-strength-class-to-28-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🧪️tests/upgrades-the-bending-strength-class-to-28-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-fmk/🧪️tests/upgrades-the-bending-strength-class-to-28-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-fc0-k" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🧪️tests/raises-the-parallel-compressive-strength-to-26-5-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🧪️tests/raises-the-parallel-compressive-strength-to-26-5-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🧪️tests/raises-the-parallel-compressive-strength-to-26-5-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-fc0-k/🧪️tests/raises-the-parallel-compressive-strength-to-26-5-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-service-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🧪️tests/moves-the-beam-from-service-class-1-to-service-class-2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🧪️tests/moves-the-beam-from-service-class-1-to-service-class-2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🧪️tests/moves-the-beam-from-service-class-1-to-service-class-2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹change-service-class/🧪️tests/moves-the-beam-from-service-class-1-to-service-class-2/🎯️outcome/🔣️component.json"),
        ),
        "change-load-duration" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🧪️tests/shortens-the-load-duration-class-from-medium-to-short/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🧪️tests/shortens-the-load-duration-class-from-medium-to-short/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🧪️tests/shortens-the-load-duration-class-from-medium-to-short/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-load-duration/🧪️tests/shortens-the-load-duration-class-from-medium-to-short/🎯️outcome/🔣️component.json"),
        ),
        "change-m-crit-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🧪️tests/raises-the-critical-buckling-moment-to-96-knm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🧪️tests/raises-the-critical-buckling-moment-to-96-knm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🧪️tests/raises-the-critical-buckling-moment-to-96-knm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-m-crit-knm/🧪️tests/raises-the-critical-buckling-moment-to-96-knm/🎯️outcome/🔣️component.json"),
        ),
        "change-f-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🧪️tests/raises-the-design-fastener-force-to-24-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🧪️tests/raises-the-design-fastener-force-to-24-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🧪️tests/raises-the-design-fastener-force-to-24-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-f-ed-kn/🧪️tests/raises-the-design-fastener-force-to-24-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-a-ef-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🧪️tests/enlarges-the-effective-connection-area-to-16000-mm2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🧪️tests/enlarges-the-effective-connection-area-to-16000-mm2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🧪️tests/enlarges-the-effective-connection-area-to-16000-mm2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-a-ef-mm2/🧪️tests/enlarges-the-effective-connection-area-to-16000-mm2/🎯️outcome/🔣️component.json"),
        ),
        "change-fvk" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🧪️tests/lowers-the-characteristic-shear-strength-to-3-5-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🧪️tests/lowers-the-characteristic-shear-strength-to-3-5-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🧪️tests/lowers-the-characteristic-shear-strength-to-3-5-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-fvk/🧪️tests/lowers-the-characteristic-shear-strength-to-3-5-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-fire-duration-min" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🧪️tests/raises-the-fire-exposure-from-r30-to-r60/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🧪️tests/raises-the-fire-exposure-from-r30-to-r60/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🧪️tests/raises-the-fire-exposure-from-r30-to-r60/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-fire-duration-min/🧪️tests/raises-the-fire-exposure-from-r30-to-r60/🎯️outcome/🔣️component.json"),
        ),
        "change-section-depth-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🧪️tests/raises-the-size-effect-depth-to-360-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🧪️tests/raises-the-size-effect-depth-to-360-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🧪️tests/raises-the-size-effect-depth-to-360-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪠change-section-depth-mm/🧪️tests/raises-the-size-effect-depth-to-360-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-a-vert-ms2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🧪️tests/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🧪️tests/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🧪️tests/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-a-vert-ms2/🧪️tests/doubles-the-vertical-footfall-acceleration-to-0-5-m-s2/🎯️outcome/🔣️component.json"),
        ),
        "change-n-cycles-bridge" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🧪️tests/quadruples-the-bridge-fatigue-cycles-to-2000000/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🧪️tests/quadruples-the-bridge-fatigue-cycles-to-2000000/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🧪️tests/quadruples-the-bridge-fatigue-cycles-to-2000000/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧼change-n-cycles-bridge/🧪️tests/quadruples-the-bridge-fatigue-cycles-to-2000000/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1995-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1995::standards::v1::subsets::any::schema::mutations::{apply_en1995_mutation, decode_en1995_mutation_json, inverse_en1995_mutation, En1995Mutation};
    use semio_s_plugin_norm::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::{decode_en1995_dsl, decode_en1995_pack, decode_en1995_snapshot_json, encode_en1995_dsl, encode_en1995_pack, encode_en1995_snapshot_json, En1995Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1995Snapshot, String> {
        decode_en1995_snapshot_json(text).map_err(|error| format!("mutate-en1995-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1995Mutation, String> {
        decode_en1995_mutation_json(text).map_err(|error| format!("mutate-en1995-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1995Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1995_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1995Snapshot, expected: &En1995Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1995_snapshot_json(got), encode_en1995_snapshot_json(expected))
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
            let applied = apply_en1995_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1995_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1995_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let steps = inverse_en1995_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1995_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1995 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1995_dsl(&text)?;
        let reprinted = encode_en1995_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1995_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1995_pack(&encode_en1995_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1995_snapshot_json(&encode_en1995_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1995_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if twin != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact", &twin, &parsed));
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
/// document the oracle role can read literally, but the real artifact is committed as DSL and pack bytes
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
