//! 🦀️ EN 1992 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `en1992-1-mutation-semantics` is gone from
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`, because a reference now
//! exists to compare against: `s.norm.en1992` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️component.py` beside this file, registered as the
//! oracle `en1992-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_en1992_mutation` over the full 35-kind `En1992Mutation` vocabulary.
//!
//! Thirty-five document-root scalars, one `change-<field>` each, feeding five distinct EN 1992
//! checks: bending and shear (M_Ed, V_Ed, f_ck, b, d, A_s, f_yk, rho_l, N_Ed, P, A_c, the FEM
//! toggle, span and UDL), fire (rating and provided axis distance), bridge fatigue (concrete
//! stress and steel stress range), the liquid-retaining crack-width check (tightness class,
//! h_D/h ratio, sigma_s, rho_p,eff, f_ct,eff, E_s and s_r,max) and the anchor check (h_ef, the
//! cracked-concrete flag, f_uk, f_yk, A_s, d, c_1, N_Ed and V_Ed).
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
//! (`decode_en1992_snapshot_json`/`encode_en1992_snapshot_json`,
//! `decode_en1992_dsl`/`encode_en1992_dsl`, `decode_en1992_pack`/`encode_en1992_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_en1992_mutation_json`, `apply_en1992_mutation`, `inverse_en1992_mutation` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`), whose
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
/// 🏷️ Mirrors `En1992Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-m-ed-knm",
    "change-v-ed-kn",
    "change-f-ck",
    "change-b-mm",
    "change-d-mm",
    "change-as-mm2",
    "change-f-yk",
    "change-rho-l",
    "change-n-ed-kn",
    "change-p-kn",
    "change-ac-mm2",
    "change-use-fem",
    "change-span-m",
    "change-udl-kn-m",
    "change-fire-rating",
    "change-provided-axis-distance-mm",
    "change-bridge-sigma-c-mpa",
    "change-bridge-delta-sigma-s-mpa",
    "change-tightness-class",
    "change-hd-over-h",
    "change-liquid-sigma-s-mpa",
    "change-liquid-rho-p-eff",
    "change-liquid-f-ct-eff-mpa",
    "change-liquid-es-mpa",
    "change-liquid-sr-max-mm",
    "change-anchor-h-ef-mm",
    "change-anchor-cracked",
    "change-anchor-f-uk-mpa",
    "change-anchor-f-yk-mpa",
    "change-anchor-as-mm2",
    "change-anchor-d-mm",
    "change-anchor-c1-mm",
    "change-anchor-n-ed-kn",
    "change-anchor-v-ed-kn",
];

/// 🗣️ The real committed EN 1992 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🧪️liquid-retaining-fem-anchor/🗣️.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🎒️.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-annex" => (
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-annex-to-en/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-annex-to-en/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-annex-to-en/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-annex-to-en/🎯️outcome/🔣️.json"),
        ),
        "change-m-ed-knm" => (
            include_str!("../../🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/🌱️raises-m-ed-knm-to-187-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/🌱️raises-m-ed-knm-to-187-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/🌱️raises-m-ed-knm-to-187-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/🌱️raises-m-ed-knm-to-187-5/🎯️outcome/🔣️.json"),
        ),
        "change-v-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/🌾️raises-v-ed-kn-to-96-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/🌾️raises-v-ed-kn-to-96-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/🌾️raises-v-ed-kn-to-96-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/🌾️raises-v-ed-kn-to-96-5/🎯️outcome/🔣️.json"),
        ),
        "change-f-ck" => (
            include_str!("../../🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/🟫️raises-f-ck-to-45-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/🟫️raises-f-ck-to-45-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/🟫️raises-f-ck-to-45-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/🟫️raises-f-ck-to-45-0/🎯️outcome/🔣️.json"),
        ),
        "change-b-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/🌾️raises-b-mm-to-375-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/🌾️raises-b-mm-to-375-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/🌾️raises-b-mm-to-375-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/🌾️raises-b-mm-to-375-0/🎯️outcome/🔣️.json"),
        ),
        "change-d-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/🟨️raises-d-mm-to-512-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/🟨️raises-d-mm-to-512-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/🟨️raises-d-mm-to-512-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/🟨️raises-d-mm-to-512-5/🎯️outcome/🔣️.json"),
        ),
        "change-as-mm2" => (
            include_str!("../../🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/🐺️raises-a-s-mm2-to-1608-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/🐺️raises-a-s-mm2-to-1608-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/🐺️raises-a-s-mm2-to-1608-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/🐺️raises-a-s-mm2-to-1608-5/🎯️outcome/🔣️.json"),
        ),
        "change-f-yk" => (
            include_str!("../../🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/🌴️raises-f-yk-to-550-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/🌴️raises-f-yk-to-550-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/🌴️raises-f-yk-to-550-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/🌴️raises-f-yk-to-550-0/🎯️outcome/🔣️.json"),
        ),
        "change-rho-l" => (
            include_str!("../../🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/⚪️raises-rho-l-to-0-015625/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/⚪️raises-rho-l-to-0-015625/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/⚪️raises-rho-l-to-0-015625/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/⚪️raises-rho-l-to-0-015625/🎯️outcome/🔣️.json"),
        ),
        "change-n-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/🐼️raises-n-ed-kn-to-62-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/🐼️raises-n-ed-kn-to-62-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/🐼️raises-n-ed-kn-to-62-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/🐼️raises-n-ed-kn-to-62-5/🎯️outcome/🔣️.json"),
        ),
        "change-p-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/🛟️raises-p-kn-to-45-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/🛟️raises-p-kn-to-45-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/🛟️raises-p-kn-to-45-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/🛟️raises-p-kn-to-45-5/🎯️outcome/🔣️.json"),
        ),
        "change-ac-mm2" => (
            include_str!("../../🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/💎️raises-a-c-mm2-to-168750-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/💎️raises-a-c-mm2-to-168750-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/💎️raises-a-c-mm2-to-168750-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/💎️raises-a-c-mm2-to-168750-0/🎯️outcome/🔣️.json"),
        ),
        "change-use-fem" => (
            include_str!("../../🧬️schema/🧬️mutations/🕸️change-use-fem/🧪️tests/🕸️turns-use-fem-on/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕸️change-use-fem/🧪️tests/🕸️turns-use-fem-on/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕸️change-use-fem/🧪️tests/🕸️turns-use-fem-on/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕸️change-use-fem/🧪️tests/🕸️turns-use-fem-on/🎯️outcome/🔣️.json"),
        ),
        "change-span-m" => (
            include_str!("../../🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/🟨️raises-span-m-to-7-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/🟨️raises-span-m-to-7-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/🟨️raises-span-m-to-7-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/🟨️raises-span-m-to-7-5/🎯️outcome/🔣️.json"),
        ),
        "change-udl-kn-m" => (
            include_str!("../../🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/🦅️raises-udl-kn-m-to-26-25/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/🦅️raises-udl-kn-m-to-26-25/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/🦅️raises-udl-kn-m-to-26-25/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/🦅️raises-udl-kn-m-to-26-25/🎯️outcome/🔣️.json"),
        ),
        "change-fire-rating" => (
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-fire-rating/🧪️tests/🔥️switches-fire-rating-to-r120/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-fire-rating/🧪️tests/🔥️switches-fire-rating-to-r120/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-fire-rating/🧪️tests/🔥️switches-fire-rating-to-r120/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔥️change-fire-rating/🧪️tests/🔥️switches-fire-rating-to-r120/🎯️outcome/🔣️.json"),
        ),
        "change-provided-axis-distance-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/🦁️raises-provided-axis-distance-mm-to-42-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/🦁️raises-provided-axis-distance-mm-to-42-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/🦁️raises-provided-axis-distance-mm-to-42-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/🦁️raises-provided-axis-distance-mm-to-42-5/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-sigma-c-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/🌉️raises-bridge-sigma-c-mpa-to-15-75/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/🌉️raises-bridge-sigma-c-mpa-to-15-75/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/🌉️raises-bridge-sigma-c-mpa-to-15-75/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/🌉️raises-bridge-sigma-c-mpa-to-15-75/🎯️outcome/🔣️.json"),
        ),
        "change-bridge-delta-sigma-s-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/🔺️raises-bridge-delta-sigma-s-mpa-to-132-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/🔺️raises-bridge-delta-sigma-s-mpa-to-132-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/🔺️raises-bridge-delta-sigma-s-mpa-to-132-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/🔺️raises-bridge-delta-sigma-s-mpa-to-132-5/🎯️outcome/🔣️.json"),
        ),
        "change-tightness-class" => (
            include_str!("../../🧬️schema/🧬️mutations/💧️change-tightness-class/🧪️tests/💧️switches-tightness-class-to-tc2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-tightness-class/🧪️tests/💧️switches-tightness-class-to-tc2/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-tightness-class/🧪️tests/💧️switches-tightness-class-to-tc2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-tightness-class/🧪️tests/💧️switches-tightness-class-to-tc2/🎯️outcome/🔣️.json"),
        ),
        "change-hd-over-h" => (
            include_str!("../../🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/🌹️raises-hd-over-h-to-12-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/🌹️raises-hd-over-h-to-12-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/🌹️raises-hd-over-h-to-12-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/🌹️raises-hd-over-h-to-12-5/🎯️outcome/🔣️.json"),
        ),
        "change-liquid-sigma-s-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/🖱️raises-liquid-sigma-s-mpa-to-235-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/🖱️raises-liquid-sigma-s-mpa-to-235-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/🖱️raises-liquid-sigma-s-mpa-to-235-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/🖱️raises-liquid-sigma-s-mpa-to-235-5/🎯️outcome/🔣️.json"),
        ),
        "change-liquid-rho-p-eff" => (
            include_str!("../../🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/🖱️raises-liquid-rho-p-eff-to-0-0078125/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/🖱️raises-liquid-rho-p-eff-to-0-0078125/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/🖱️raises-liquid-rho-p-eff-to-0-0078125/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/🖱️raises-liquid-rho-p-eff-to-0-0078125/🎯️outcome/🔣️.json"),
        ),
        "change-liquid-f-ct-eff-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/🖱️raises-liquid-f-ct-eff-mpa-to-3-25/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/🖱️raises-liquid-f-ct-eff-mpa-to-3-25/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/🖱️raises-liquid-f-ct-eff-mpa-to-3-25/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/🖱️raises-liquid-f-ct-eff-mpa-to-3-25/🎯️outcome/🔣️.json"),
        ),
        "change-liquid-es-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/🖱️raises-liquid-e-s-mpa-to-205000-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/🖱️raises-liquid-e-s-mpa-to-205000-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/🖱️raises-liquid-e-s-mpa-to-205000-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/🖱️raises-liquid-e-s-mpa-to-205000-0/🎯️outcome/🔣️.json"),
        ),
        "change-liquid-sr-max-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/🖱️raises-liquid-s-r-max-mm-to-312-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/🖱️raises-liquid-s-r-max-mm-to-312-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/🖱️raises-liquid-s-r-max-mm-to-312-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/🖱️raises-liquid-s-r-max-mm-to-312-5/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-h-ef-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/🐼️raises-anchor-h-ef-mm-to-105-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/🐼️raises-anchor-h-ef-mm-to-105-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/🐼️raises-anchor-h-ef-mm-to-105-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/🐼️raises-anchor-h-ef-mm-to-105-0/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-cracked" => (
            include_str!("../../🧬️schema/🧬️mutations/💥️change-anchor-cracked/🧪️tests/💥️turns-anchor-cracked-on/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💥️change-anchor-cracked/🧪️tests/💥️turns-anchor-cracked-on/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💥️change-anchor-cracked/🧪️tests/💥️turns-anchor-cracked-on/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💥️change-anchor-cracked/🧪️tests/💥️turns-anchor-cracked-on/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-f-uk-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/🛰️raises-anchor-f-uk-mpa-to-900-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/🛰️raises-anchor-f-uk-mpa-to-900-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/🛰️raises-anchor-f-uk-mpa-to-900-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/🛰️raises-anchor-f-uk-mpa-to-900-0/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-f-yk-mpa" => (
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-anchor-f-yk-mpa/🧪️tests/🛡️raises-anchor-f-yk-mpa-to-720-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-anchor-f-yk-mpa/🧪️tests/🛡️raises-anchor-f-yk-mpa-to-720-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-anchor-f-yk-mpa/🧪️tests/🛡️raises-anchor-f-yk-mpa-to-720-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-anchor-f-yk-mpa/🧪️tests/🛡️raises-anchor-f-yk-mpa-to-720-0/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-as-mm2" => (
            include_str!("../../🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/🟪️raises-anchor-a-s-mm2-to-157-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/🟪️raises-anchor-a-s-mm2-to-157-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/🟪️raises-anchor-a-s-mm2-to-157-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/🟪️raises-anchor-a-s-mm2-to-157-0/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-d-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-anchor-d-mm/🧪️tests/⭕️raises-anchor-d-mm-to-16-0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-anchor-d-mm/🧪️tests/⭕️raises-anchor-d-mm-to-16-0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-anchor-d-mm/🧪️tests/⭕️raises-anchor-d-mm-to-16-0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⭕️change-anchor-d-mm/🧪️tests/⭕️raises-anchor-d-mm-to-16-0/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-c1-mm" => (
            include_str!("../../🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/🦁️raises-anchor-c1-mm-to-137-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/🦁️raises-anchor-c1-mm-to-137-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/🦁️raises-anchor-c1-mm-to-137-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/🦁️raises-anchor-c1-mm-to-137-5/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-n-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/🧿️raises-anchor-n-ed-kn-to-22-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/🧿️raises-anchor-n-ed-kn-to-22-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/🧿️raises-anchor-n-ed-kn-to-22-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/🧿️raises-anchor-n-ed-kn-to-22-5/🎯️outcome/🔣️.json"),
        ),
        "change-anchor-v-ed-kn" => (
            include_str!("../../🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/🟩️raises-anchor-v-ed-kn-to-11-25/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/🟩️raises-anchor-v-ed-kn-to-11-25/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/🟩️raises-anchor-v-ed-kn-to-11-25/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/🟩️raises-anchor-v-ed-kn-to-11-25/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-en1992-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1992::standards::v1::subsets::any::schema::mutations::{apply_en1992_mutation, decode_en1992_mutation_json, inverse_en1992_mutation, En1992Mutation};
    use semio_s_plugin_norm::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::{decode_en1992_dsl, decode_en1992_pack, decode_en1992_snapshot_json, encode_en1992_dsl, encode_en1992_pack, encode_en1992_snapshot_json, En1992Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1992Snapshot, String> {
        decode_en1992_snapshot_json(text).map_err(|error| format!("mutate-en1992-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1992Mutation, String> {
        decode_en1992_mutation_json(text).map_err(|error| format!("mutate-en1992-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1992Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1992_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1992Snapshot, expected: &En1992Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1992_snapshot_json(got), encode_en1992_snapshot_json(expected))
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
            let applied = apply_en1992_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1992_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1992_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_en1992_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1992_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1992 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1992_dsl(&text)?;
        let reprinted = encode_en1992_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1992_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1992_pack(&encode_en1992_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1992_snapshot_json(&encode_en1992_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1992_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
