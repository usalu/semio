//! 🦀️ EN 1999 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `en1999-1-mutation-semantics` is gone from
//! `../../🔮️oracle/🔣️.json`, because a reference now
//! exists to compare against: `s.norm.en1999` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️.py` beside this file, registered as the
//! oracle `en1999-1-python-independent`. This adapter is the SUBJECT half only — it drives this
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
//! ⚖️ WHERE THE ASSERTIONS LIVE. Every law this case claims is asserted IN ROLE inside the
//! subject handlers as well as being compared against the oracle's answer, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`,
//! `law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the
//! stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin
//! declares in `✏️s/🔌️plugins/📕️norm/🔣️oracle.json`. What `parity` adds on top is the
//! one thing a single implementation can never provide: that a second implementation, written in
//! another language from the same written specification, reaches the same document.
//!
//! 🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only
//! `semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate;
//! `serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are all
//! unreachable from here. The subset's own production code therefore exports the bridges
//! (`decode_en1999_snapshot_json`/`encode_en1999_snapshot_json`,
//! `decode_en1999_dsl`/`encode_en1999_dsl`, `decode_en1999_pack`/`encode_en1999_pack` in
//! `../../🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_en1999_mutation_json`, `apply_en1999_mutation`, `inverse_en1999_mutation` in
//! `../../🧬️schema/🧬️mutations/🦀️.rs`), whose
//! signatures name only reachable types. This side reaches the committed vectors through
//! `include_str!` and the Python side through the `asset://` URIs the feature declares, so both
//! read the SAME committed bytes and neither holds a Rust or Python literal transcribed beside
//! them that could drift from what the other one read.
//!
//! 🚧️ The Rust SUBJECT phase still cannot run: `semio-s-plugin-norm` does not compile (671 errors
//! at the time of writing — a concurrent session is mid-flight across ~2000 files of this plugin,
//! removing gratuitous `async fn` wrappers). `parity` therefore has nothing to compare the oracle
//! against YET; the moment the crate is green it does, with no further change here. The subject half is
//! written against the SYNC trait surface the fixture tests in this crate already call
//! (`Mutation::diff`, `MutationDiff::apply`, `Mutation::inverse`, `ArtifactDsl`,
//! `ArtifactPack`) rather than against the plugin's async wrappers, and is `sut`-gated so the
//! oracle-only run never links it.

use semio_repo_test_host::{digest, parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `En1999Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
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
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🏠️aluminium-roof-purlin/🖼️assets/🏠️aluminium-roof-purlin/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://📚️examples/🏠️aluminium-roof-purlin/🖼️assets/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-n-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🏋️change-n-ed-kn/🧪️tests/🏋️raises-axial-force-1357d7/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏋️change-n-ed-kn/🧪️tests/🏋️raises-axial-force-1357d7/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏋️change-n-ed-kn/🧪️tests/🏋️raises-axial-force-1357d7/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏋️change-n-ed-kn/🧪️tests/🏋️raises-axial-force-1357d7/🎯️outcome/🔣️.json"),
        ),
        "change-m-ed-knm" => (
            include_str!("../../🧬️schema/🧬️mutations/⤴️change-m-ed-knm/🧪️tests/⤴️raises-design-2699b1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⤴️change-m-ed-knm/🧪️tests/⤴️raises-design-2699b1/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⤴️change-m-ed-knm/🧪️tests/⤴️raises-design-2699b1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⤴️change-m-ed-knm/🧪️tests/⤴️raises-design-2699b1/🎯️outcome/🔣️.json"),
        ),
        "change-a-mm2" => (
            include_str!("../../🧬️schema/🧬️mutations/📐️change-a-mm2/🧪️tests/📐️enlarges-section-87627e/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-a-mm2/🧪️tests/📐️enlarges-section-87627e/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-a-mm2/🧪️tests/📐️enlarges-section-87627e/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-a-mm2/🧪️tests/📐️enlarges-section-87627e/🎯️outcome/🔣️.json"),
        ),
        "change-w-el-mm3" => (
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-w-el-mm3/🧪️tests/🧊️raises-section-549f53/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-w-el-mm3/🧪️tests/🧊️raises-section-549f53/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-w-el-mm3/🧪️tests/🧊️raises-section-549f53/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-w-el-mm3/🧪️tests/🧊️raises-section-549f53/🎯️outcome/🔣️.json"),
        ),
        "change-alloy" => (
            include_str!("../../🧬️schema/🧬️mutations/⚗️change-alloy/🧪️tests/⚗️switches-alloy-to-aw7020t6/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚗️change-alloy/🧪️tests/⚗️switches-alloy-to-aw7020t6/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚗️change-alloy/🧪️tests/⚗️switches-alloy-to-aw7020t6/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚗️change-alloy/🧪️tests/⚗️switches-alloy-to-aw7020t6/🎯️outcome/🔣️.json"),
        ),
        "change-chi" => (
            include_str!("../../🧬️schema/🧬️mutations/⬇️change-chi/🧪️tests/⬇️lowers-buckling-chi-to-0-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⬇️change-chi/🧪️tests/⬇️lowers-buckling-chi-to-0-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⬇️change-chi/🧪️tests/⬇️lowers-buckling-chi-to-0-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⬇️change-chi/🧪️tests/⬇️lowers-buckling-chi-to-0-5/🎯️outcome/🔣️.json"),
        ),
        "change-it-mm4" => (
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-it-mm4/🧪️tests/🌀️raises-torsion-05cfed/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-it-mm4/🧪️tests/🌀️raises-torsion-05cfed/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-it-mm4/🧪️tests/🌀️raises-torsion-05cfed/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-it-mm4/🧪️tests/🌀️raises-torsion-05cfed/🎯️outcome/🔣️.json"),
        ),
        "change-l-cr-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/📏️change-l-cr-mm/🧪️tests/📏️lengthens-buckling-2d5120/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📏️change-l-cr-mm/🧪️tests/📏️lengthens-buckling-2d5120/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📏️change-l-cr-mm/🧪️tests/📏️lengthens-buckling-2d5120/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📏️change-l-cr-mm/🧪️tests/📏️lengthens-buckling-2d5120/🎯️outcome/🔣️.json"),
        ),
        "change-theta-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-theta-c/🧪️tests/🌡️raises-temperature-485796/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-theta-c/🧪️tests/🌡️raises-temperature-485796/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-theta-c/🧪️tests/🌡️raises-temperature-485796/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-theta-c/🧪️tests/🌡️raises-temperature-485796/🎯️outcome/🔣️.json"),
        ),
        "change-delta-sigma-ed" => (
            include_str!("../../🧬️schema/🧬️mutations/↕️change-delta-sigma-ed/🧪️tests/↕️raises-fatigue-efde64/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-delta-sigma-ed/🧪️tests/↕️raises-fatigue-efde64/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-delta-sigma-ed/🧪️tests/↕️raises-fatigue-efde64/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-delta-sigma-ed/🧪️tests/↕️raises-fatigue-efde64/🎯️outcome/🔣️.json"),
        ),
        "change-delta-sigma-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-delta-sigma-c/🧪️tests/🏷️upgrades-detail-29b434/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-delta-sigma-c/🧪️tests/🏷️upgrades-detail-29b434/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-delta-sigma-c/🧪️tests/🏷️upgrades-detail-29b434/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-delta-sigma-c/🧪️tests/🏷️upgrades-detail-29b434/🎯️outcome/🔣️.json"),
        ),
        "change-fatigue-m" => (
            include_str!("../../🧬️schema/🧬️mutations/📉️change-fatigue-m/🧪️tests/📉️flattens-sn-slope-to-m-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-fatigue-m/🧪️tests/📉️flattens-sn-slope-to-m-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-fatigue-m/🧪️tests/📉️flattens-sn-slope-to-m-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-fatigue-m/🧪️tests/📉️flattens-sn-slope-to-m-5/🎯️outcome/🔣️.json"),
        ),
        "change-n-cycles" => (
            include_str!("../../🧬️schema/🧬️mutations/🔁️change-n-cycles/🧪️tests/🔁️doubles-fatigue-deabf2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔁️change-n-cycles/🧪️tests/🔁️doubles-fatigue-deabf2/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔁️change-n-cycles/🧪️tests/🔁️doubles-fatigue-deabf2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔁️change-n-cycles/🧪️tests/🔁️doubles-fatigue-deabf2/🎯️outcome/🔣️.json"),
        ),
        "change-v-weld-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/✂️change-v-weld-ed-kn/🧪️tests/✂️raises-weld-shear-1aa664/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✂️change-v-weld-ed-kn/🧪️tests/✂️raises-weld-shear-1aa664/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✂️change-v-weld-ed-kn/🧪️tests/✂️raises-weld-shear-1aa664/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✂️change-v-weld-ed-kn/🧪️tests/✂️raises-weld-shear-1aa664/🎯️outcome/🔣️.json"),
        ),
        "change-weld-throat-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-weld-throat-mm/🧪️tests/🔥️thickens-weld-f6bb0e/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-weld-throat-mm/🧪️tests/🔥️thickens-weld-f6bb0e/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-weld-throat-mm/🧪️tests/🔥️thickens-weld-f6bb0e/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-weld-throat-mm/🧪️tests/🔥️thickens-weld-f6bb0e/🎯️outcome/🔣️.json"),
        ),
        "change-weld-length-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🧵️change-weld-length-mm/🧪️tests/🧵️lengthens-weld-67bc92/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧵️change-weld-length-mm/🧪️tests/🧵️lengthens-weld-67bc92/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧵️change-weld-length-mm/🧪️tests/🧵️lengthens-weld-67bc92/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧵️change-weld-length-mm/🧪️tests/🧵️lengthens-weld-67bc92/🎯️outcome/🔣️.json"),
        ),
        "change-beta-w" => (
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-beta-w/🧪️tests/🧮️raises-weld-d69034/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-beta-w/🧪️tests/🧮️raises-weld-d69034/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-beta-w/🧪️tests/🧮️raises-weld-d69034/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-beta-w/🧪️tests/🧮️raises-weld-d69034/🎯️outcome/🔣️.json"),
        ),
        "change-sheet-b-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/↔️change-sheet-b-mm/🧪️tests/↔️widens-sheet-to-320-mm/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↔️change-sheet-b-mm/🧪️tests/↔️widens-sheet-to-320-mm/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↔️change-sheet-b-mm/🧪️tests/↔️widens-sheet-to-320-mm/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↔️change-sheet-b-mm/🧪️tests/↔️widens-sheet-to-320-mm/🎯️outcome/🔣️.json"),
        ),
        "change-sheet-t-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/📑️change-sheet-t-mm/🧪️tests/📑️thickens-sheet-7a8128/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📑️change-sheet-t-mm/🧪️tests/📑️thickens-sheet-7a8128/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📑️change-sheet-t-mm/🧪️tests/📑️thickens-sheet-7a8128/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📑️change-sheet-t-mm/🧪️tests/📑️thickens-sheet-7a8128/🎯️outcome/🔣️.json"),
        ),
        "change-sheet-k-sigma" => (
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-sheet-k-sigma/🧪️tests/🎚️raises-sheet-46106e/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-sheet-k-sigma/🧪️tests/🎚️raises-sheet-46106e/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-sheet-k-sigma/🧪️tests/🎚️raises-sheet-46106e/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-sheet-k-sigma/🧪️tests/🎚️raises-sheet-46106e/🎯️outcome/🔣️.json"),
        ),
        "change-sheet-w-el-mm3" => (
            include_str!("../../🧬️schema/🧬️mutations/📊️change-sheet-w-el-mm3/🧪️tests/📊️raises-sheet-e2adfa/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-sheet-w-el-mm3/🧪️tests/📊️raises-sheet-e2adfa/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-sheet-w-el-mm3/🧪️tests/📊️raises-sheet-e2adfa/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-sheet-w-el-mm3/🧪️tests/📊️raises-sheet-e2adfa/🎯️outcome/🔣️.json"),
        ),
        "change-sheet-m-ed-knm" => (
            include_str!("../../🧬️schema/🧬️mutations/🌊️change-sheet-m-ed-knm/🧪️tests/🌊️raises-sheet-f095d9/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌊️change-sheet-m-ed-knm/🧪️tests/🌊️raises-sheet-f095d9/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌊️change-sheet-m-ed-knm/🧪️tests/🌊️raises-sheet-f095d9/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌊️change-sheet-m-ed-knm/🧪️tests/🌊️raises-sheet-f095d9/🎯️outcome/🔣️.json"),
        ),
        "change-shell-t-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🐚️change-shell-t-mm/🧪️tests/🐚️thickens-shell-b4701c/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐚️change-shell-t-mm/🧪️tests/🐚️thickens-shell-b4701c/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐚️change-shell-t-mm/🧪️tests/🐚️thickens-shell-b4701c/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐚️change-shell-t-mm/🧪️tests/🐚️thickens-shell-b4701c/🎯️outcome/🔣️.json"),
        ),
        "change-shell-r-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-shell-r-mm/🧪️tests/🐚️widens-shell-3f9379/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-shell-r-mm/🧪️tests/🐚️widens-shell-3f9379/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-shell-r-mm/🧪️tests/🐚️widens-shell-3f9379/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-shell-r-mm/🧪️tests/🐚️widens-shell-3f9379/🎯️outcome/🔣️.json"),
        ),
        "change-sigma-ed-shell-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🗜️change-sigma-ed-shell-mpa/🧪️tests/🐚️raises-shell-19c224/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🗜️change-sigma-ed-shell-mpa/🧪️tests/🐚️raises-shell-19c224/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🗜️change-sigma-ed-shell-mpa/🧪️tests/🐚️raises-shell-19c224/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🗜️change-sigma-ed-shell-mpa/🧪️tests/🐚️raises-shell-19c224/🎯️outcome/🔣️.json"),
        ),
        "change-annex" => (
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-national-41f642/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-national-41f642/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-national-41f642/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-national-41f642/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1999-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The status the committed `🎯️outcome/🔣️.json` declares for one kind — `applied` or
/// `rejected` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
#[cfg(feature = "sut")]
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}
//#endregion 🔖️Fixtures

//#region 🔖️Carrier
/// 🧵️ The canonical carrier bytes as a comparable projection: the envelope preamble, every body line
/// as written, and the digest and length of what was emitted. `.dsl.semio` has no grammar document in
/// this repository — the committed `📖️.grammar.semio` is the repository-wide `payload = OCTET+`
/// placeholder — so the identity scenario compares the two implementations at the carrier level rather
/// than mapping carrier tokens onto the snapshot's enum spellings, a mapping nothing states. The
/// independent Python implementation builds the identical shape from ITS re-emission, and `digest` is
/// the coordinator's own sha256, so the two languages' bytes are directly comparable.
#[cfg(feature = "sut")]
fn carrier_projection(text: &str) -> Json {
    let (preamble, body) = text.split_once('\n').unwrap_or((text, ""));
    let body = body.strip_suffix('\n').unwrap_or(body);
    let lines = if body.is_empty() { Vec::new() } else { body.split('\n').map(|line| Json::String(line.to_string())).collect::<Vec<Json>>() };
    Json::Object(vec![
        ("preamble".to_string(), Json::String(preamble.to_string())),
        ("lines".to_string(), Json::Array(lines)),
        ("dslDigest".to_string(), Json::String(digest(text.as_bytes()))),
        ("dslLength".to_string(), Json::Number(text.as_bytes().len() as f64)),
    ])
}
//#endregion 🔖️Carrier

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use crate::standards::v1::subsets::any::schema::mutations::{apply_en1999_mutation, decode_en1999_mutation_json, inverse_en1999_mutation, En1999Mutation};
    use crate::standards::v1::subsets::any::schema::snapshot::{decode_en1999_dsl, decode_en1999_pack, decode_en1999_snapshot_json, encode_en1999_dsl, encode_en1999_pack, encode_en1999_snapshot_json, En1999Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
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
    /// The projection carries BOTH the mutated and the restored document: projecting only the restored
    /// one would make every row of the table project the same value and the differential vacuous.
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
            let mutated = projection(&current)?;
            let steps = inverse_en1999_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1999_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), restored)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
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
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), carrier_projection(&reprinted)))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. SUBJECT role only: the reference
/// answer now comes from the independent Python implementation registered as this subset's oracle, and
/// registering this repository's own answer on the oracle side as well would compare it with
/// itself.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
