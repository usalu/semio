//! 🦀️ EN 1999 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1999-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1999` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_en1999_mutation` over the full 26-kind `En1999Mutation` vocabulary.
//!
//! Twenty-six document-root scalars and enums, one `change-<field>` each: the design actions
//! N_Ed and M_Ed, the section properties A and W_el, the alloy selection, the buckling
//! reduction chi, the torsion constant I_T and the critical length L_cr, the
//! elevated-temperature theta, the fatigue set (applied and detail stress ranges, the slope m,
//! and the cycle count), the fillet weld set (V_Ed, throat, length and the correlation factor
//! beta_w), the thin-sheet local-buckling set (b, t, k_sigma, W_el and M_Ed) and the shell set
//! (t, r and the applied meridional stress).
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
//! (`decode_en1999_snapshot_json`/`encode_en1999_snapshot_json`,
//! `decode_en1999_dsl`/`encode_en1999_dsl`, `decode_en1999_pack`/`encode_en1999_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1999_mutation_json`, `apply_en1999_mutation`, `inverse_en1999_mutation` in
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
/// 🏷️ Mirrors `En1999Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "change-n-ed-kn",
    "change-m-ed-knm",
    "change-a-mm2",
    "change-w-el-mm3",
    "change-alloy",
    "change-chi",
    "change-it-mm4",
    "change-l-cr-mm",
    "change-theta-c",
    "change-delta-sigma-ed",
    "change-delta-sigma-c",
    "change-fatigue-m",
    "change-n-cycles",
    "change-v-weld-ed-kn",
    "change-weld-throat-mm",
    "change-weld-length-mm",
    "change-beta-w",
    "change-sheet-b-mm",
    "change-sheet-t-mm",
    "change-sheet-k-sigma",
    "change-sheet-w-el-mm3",
    "change-sheet-m-ed-knm",
    "change-shell-t-mm",
    "change-shell-r-mm",
    "change-sigma-ed-shell-mpa",
    "change-annex",
];

/// 🗣️ The real committed EN 1999 document, read where the domain already keeps it.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🎒️aluminium-roof-purlin.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-n-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🧪️tests/raises-axial-force-to-180-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🧪️tests/raises-axial-force-to-180-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🧪️tests/raises-axial-force-to-180-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-n-ed-kn/🧪️tests/raises-axial-force-to-180-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-m-ed-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🧪️tests/raises-design-moment-to-9-5-knm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🧪️tests/raises-design-moment-to-9-5-knm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🧪️tests/raises-design-moment-to-9-5-knm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-m-ed-knm/🧪️tests/raises-design-moment-to-9-5-knm/🎯️outcome/🔣️component.json"),
        ),
        "change-a-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🧪️tests/enlarges-section-area-to-2250-mm2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🧪️tests/enlarges-section-area-to-2250-mm2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🧪️tests/enlarges-section-area-to-2250-mm2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-a-mm2/🧪️tests/enlarges-section-area-to-2250-mm2/🎯️outcome/🔣️component.json"),
        ),
        "change-w-el-mm3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🧪️tests/raises-section-modulus-to-40000-mm3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🧪️tests/raises-section-modulus-to-40000-mm3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🧪️tests/raises-section-modulus-to-40000-mm3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-w-el-mm3/🧪️tests/raises-section-modulus-to-40000-mm3/🎯️outcome/🔣️component.json"),
        ),
        "change-alloy" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🧪️tests/switches-alloy-to-aw7020t6/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🧪️tests/switches-alloy-to-aw7020t6/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🧪️tests/switches-alloy-to-aw7020t6/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-alloy/🧪️tests/switches-alloy-to-aw7020t6/🎯️outcome/🔣️component.json"),
        ),
        "change-chi" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🧪️tests/lowers-buckling-chi-to-0-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🧪️tests/lowers-buckling-chi-to-0-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🧪️tests/lowers-buckling-chi-to-0-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-chi/🧪️tests/lowers-buckling-chi-to-0-5/🎯️outcome/🔣️component.json"),
        ),
        "change-it-mm4" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🧪️tests/raises-torsion-constant-to-10240-mm4/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🧪️tests/raises-torsion-constant-to-10240-mm4/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🧪️tests/raises-torsion-constant-to-10240-mm4/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-it-mm4/🧪️tests/raises-torsion-constant-to-10240-mm4/🎯️outcome/🔣️component.json"),
        ),
        "change-l-cr-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🧪️tests/lengthens-buckling-length-to-4000-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🧪️tests/lengthens-buckling-length-to-4000-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🧪️tests/lengthens-buckling-length-to-4000-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-l-cr-mm/🧪️tests/lengthens-buckling-length-to-4000-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-theta-c" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🧪️tests/raises-fatigue-detail-theta-c-to-225-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🧪️tests/raises-fatigue-detail-theta-c-to-225-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🧪️tests/raises-fatigue-detail-theta-c-to-225-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-theta-c/🧪️tests/raises-fatigue-detail-theta-c-to-225-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-delta-sigma-ed" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🧪️tests/raises-fatigue-stress-range-to-62-5-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🧪️tests/raises-fatigue-stress-range-to-62-5-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🧪️tests/raises-fatigue-stress-range-to-62-5-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-delta-sigma-ed/🧪️tests/raises-fatigue-stress-range-to-62-5-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-delta-sigma-c" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🧪️tests/upgrades-detail-category-to-90-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🧪️tests/upgrades-detail-category-to-90-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🧪️tests/upgrades-detail-category-to-90-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-delta-sigma-c/🧪️tests/upgrades-detail-category-to-90-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-fatigue-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🧪️tests/flattens-sn-slope-to-m-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🧪️tests/flattens-sn-slope-to-m-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🧪️tests/flattens-sn-slope-to-m-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-fatigue-m/🧪️tests/flattens-sn-slope-to-m-5/🎯️outcome/🔣️component.json"),
        ),
        "change-n-cycles" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🧪️tests/doubles-fatigue-cycles-to-2000000/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🧪️tests/doubles-fatigue-cycles-to-2000000/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🧪️tests/doubles-fatigue-cycles-to-2000000/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-n-cycles/🧪️tests/doubles-fatigue-cycles-to-2000000/🎯️outcome/🔣️component.json"),
        ),
        "change-v-weld-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🧪️tests/raises-weld-shear-to-48-kn/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🧪️tests/raises-weld-shear-to-48-kn/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🧪️tests/raises-weld-shear-to-48-kn/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-v-weld-ed-kn/🧪️tests/raises-weld-shear-to-48-kn/🎯️outcome/🔣️component.json"),
        ),
        "change-weld-throat-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🧪️tests/thickens-weld-throat-to-6-5-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🧪️tests/thickens-weld-throat-to-6-5-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🧪️tests/thickens-weld-throat-to-6-5-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-weld-throat-mm/🧪️tests/thickens-weld-throat-to-6-5-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-weld-length-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🧪️tests/lengthens-weld-to-200-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🧪️tests/lengthens-weld-to-200-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🧪️tests/lengthens-weld-to-200-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-weld-length-mm/🧪️tests/lengthens-weld-to-200-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-beta-w" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🧪️tests/raises-weld-correlation-beta-w-to-0-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🧪️tests/raises-weld-correlation-beta-w-to-0-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🧪️tests/raises-weld-correlation-beta-w-to-0-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-beta-w/🧪️tests/raises-weld-correlation-beta-w-to-0-75/🎯️outcome/🔣️component.json"),
        ),
        "change-sheet-b-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🧪️tests/widens-sheet-to-320-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🧪️tests/widens-sheet-to-320-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🧪️tests/widens-sheet-to-320-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-sheet-b-mm/🧪️tests/widens-sheet-to-320-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-sheet-t-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🧪️tests/thickens-sheet-to-3-5-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🧪️tests/thickens-sheet-to-3-5-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🧪️tests/thickens-sheet-to-3-5-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-sheet-t-mm/🧪️tests/thickens-sheet-to-3-5-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-sheet-k-sigma" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🧪️tests/raises-sheet-plate-buckling-k-sigma-to-6-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🧪️tests/raises-sheet-plate-buckling-k-sigma-to-6-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🧪️tests/raises-sheet-plate-buckling-k-sigma-to-6-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-sheet-k-sigma/🧪️tests/raises-sheet-plate-buckling-k-sigma-to-6-25/🎯️outcome/🔣️component.json"),
        ),
        "change-sheet-w-el-mm3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🧪️tests/raises-sheet-section-modulus-to-12800-mm3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🧪️tests/raises-sheet-section-modulus-to-12800-mm3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🧪️tests/raises-sheet-section-modulus-to-12800-mm3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-sheet-w-el-mm3/🧪️tests/raises-sheet-section-modulus-to-12800-mm3/🎯️outcome/🔣️component.json"),
        ),
        "change-sheet-m-ed-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🧪️tests/raises-sheet-design-moment-to-1-25-knm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🧪️tests/raises-sheet-design-moment-to-1-25-knm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🧪️tests/raises-sheet-design-moment-to-1-25-knm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-sheet-m-ed-knm/🧪️tests/raises-sheet-design-moment-to-1-25-knm/🎯️outcome/🔣️component.json"),
        ),
        "change-shell-t-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🧪️tests/thickens-shell-to-6-25-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🧪️tests/thickens-shell-to-6-25-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🧪️tests/thickens-shell-to-6-25-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-shell-t-mm/🧪️tests/thickens-shell-to-6-25-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-shell-r-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🧪️tests/widens-shell-radius-to-750-mm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🧪️tests/widens-shell-radius-to-750-mm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🧪️tests/widens-shell-radius-to-750-mm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-shell-r-mm/🧪️tests/widens-shell-radius-to-750-mm/🎯️outcome/🔣️component.json"),
        ),
        "change-sigma-ed-shell-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🧪️tests/raises-shell-design-stress-to-165-mpa/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🧪️tests/raises-shell-design-stress-to-165-mpa/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🧪️tests/raises-shell-design-stress-to-165-mpa/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-sigma-ed-shell-mpa/🧪️tests/raises-shell-design-stress-to-165-mpa/🎯️outcome/🔣️component.json"),
        ),
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🧪️tests/switches-national-annex-to-en/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🧪️tests/switches-national-annex-to-en/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦘change-annex/🧪️tests/switches-national-annex-to-en/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1999-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1999::standards::v1::subsets::any::schema::mutations::{apply_en1999_mutation, decode_en1999_mutation_json, inverse_en1999_mutation, En1999Mutation};
    use semio_s_plugin_norm::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::{decode_en1999_dsl, decode_en1999_pack, decode_en1999_snapshot_json, encode_en1999_dsl, encode_en1999_pack, encode_en1999_snapshot_json, En1999Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1999Snapshot, String> {
        decode_en1999_snapshot_json(text).map_err(|error| format!("mutate-en1999-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1999Mutation, String> {
        decode_en1999_mutation_json(text).map_err(|error| format!("mutate-en1999-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1999Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1999_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1999Snapshot, expected: &En1999Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1999_snapshot_json(got), encode_en1999_snapshot_json(expected))
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
            let applied = apply_en1999_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1999_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1999_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let steps = inverse_en1999_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1999_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1999 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1999_dsl(&text)?;
        let reprinted = encode_en1999_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1999_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1999_pack(&encode_en1999_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1999_snapshot_json(&encode_en1999_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1999_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
