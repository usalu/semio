//! 🦀️ EN 1992 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1992-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1992` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_en1992_mutation` over the full 35-kind `En1992Mutation` vocabulary.
//!
//! Thirty-five document-root scalars, one `change-<field>` each, feeding five distinct EN 1992
//! checks: bending and shear (M_Ed, V_Ed, f_ck, b, d, A_s, f_yk, rho_l, N_Ed, P, A_c, the FEM
//! toggle, span and UDL), fire (rating and provided axis distance), bridge fatigue (concrete
//! stress and steel stress range), the liquid-retaining crack-width check (tightness class,
//! h_D/h ratio, sigma_s, rho_p,eff, f_ct,eff, E_s and s_r,max) and the anchor check (h_ef, the
//! cracked-concrete flag, f_uk, f_yk, A_s, d, c_1, N_Ed and V_Ed).
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
//! (`decode_en1992_snapshot_json`/`encode_en1992_snapshot_json`,
//! `decode_en1992_dsl`/`encode_en1992_dsl`, `decode_en1992_pack`/`encode_en1992_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1992_mutation_json`, `apply_en1992_mutation`, `inverse_en1992_mutation` in
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
/// 🏷️ Mirrors `En1992Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
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
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🎒️liquid-retaining-fem-anchor.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🧪️tests/switches-annex-to-en/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🧪️tests/switches-annex-to-en/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🧪️tests/switches-annex-to-en/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝set-snapshot/🧪️tests/switches-annex-to-en/🎯️outcome/🔣️component.json"),
        ),
        "change-m-ed-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/raises-m-ed-knm-to-187-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/raises-m-ed-knm-to-187-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/raises-m-ed-knm-to-187-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-m-ed-knm/🧪️tests/raises-m-ed-knm-to-187-5/🎯️outcome/🔣️component.json"),
        ),
        "change-v-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/raises-v-ed-kn-to-96-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/raises-v-ed-kn-to-96-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/raises-v-ed-kn-to-96-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦒change-v-ed-kn/🧪️tests/raises-v-ed-kn-to-96-5/🎯️outcome/🔣️component.json"),
        ),
        "change-f-ck" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/raises-f-ck-to-45-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/raises-f-ck-to-45-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/raises-f-ck-to-45-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-f-ck/🧪️tests/raises-f-ck-to-45-0/🎯️outcome/🔣️component.json"),
        ),
        "change-b-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/raises-b-mm-to-375-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/raises-b-mm-to-375-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/raises-b-mm-to-375-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-b-mm/🧪️tests/raises-b-mm-to-375-0/🎯️outcome/🔣️component.json"),
        ),
        "change-d-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/raises-d-mm-to-512-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/raises-d-mm-to-512-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/raises-d-mm-to-512-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-d-mm/🧪️tests/raises-d-mm-to-512-5/🎯️outcome/🔣️component.json"),
        ),
        "change-as-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/raises-a-s-mm2-to-1608-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/raises-a-s-mm2-to-1608-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/raises-a-s-mm2-to-1608-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-as-mm2/🧪️tests/raises-a-s-mm2-to-1608-5/🎯️outcome/🔣️component.json"),
        ),
        "change-f-yk" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/raises-f-yk-to-550-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/raises-f-yk-to-550-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/raises-f-yk-to-550-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-f-yk/🧪️tests/raises-f-yk-to-550-0/🎯️outcome/🔣️component.json"),
        ),
        "change-rho-l" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/raises-rho-l-to-0-015625/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/raises-rho-l-to-0-015625/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/raises-rho-l-to-0-015625/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐘change-rho-l/🧪️tests/raises-rho-l-to-0-015625/🎯️outcome/🔣️component.json"),
        ),
        "change-n-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/raises-n-ed-kn-to-62-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/raises-n-ed-kn-to-62-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/raises-n-ed-kn-to-62-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-n-ed-kn/🧪️tests/raises-n-ed-kn-to-62-5/🎯️outcome/🔣️component.json"),
        ),
        "change-p-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/raises-p-kn-to-45-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/raises-p-kn-to-45-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/raises-p-kn-to-45-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-p-kn/🧪️tests/raises-p-kn-to-45-5/🎯️outcome/🔣️component.json"),
        ),
        "change-ac-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/raises-a-c-mm2-to-168750-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/raises-a-c-mm2-to-168750-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/raises-a-c-mm2-to-168750-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-ac-mm2/🧪️tests/raises-a-c-mm2-to-168750-0/🎯️outcome/🔣️component.json"),
        ),
        "change-use-fem" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🧪️tests/turns-use-fem-on/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🧪️tests/turns-use-fem-on/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🧪️tests/turns-use-fem-on/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐫change-use-fem/🧪️tests/turns-use-fem-on/🎯️outcome/🔣️component.json"),
        ),
        "change-span-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/raises-span-m-to-7-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/raises-span-m-to-7-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/raises-span-m-to-7-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦏change-span-m/🧪️tests/raises-span-m-to-7-5/🎯️outcome/🔣️component.json"),
        ),
        "change-udl-kn-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/raises-udl-kn-m-to-26-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/raises-udl-kn-m-to-26-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/raises-udl-kn-m-to-26-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐪change-udl-kn-m/🧪️tests/raises-udl-kn-m-to-26-25/🎯️outcome/🔣️component.json"),
        ),
        "change-fire-rating" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🧪️tests/switches-fire-rating-to-r120/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🧪️tests/switches-fire-rating-to-r120/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🧪️tests/switches-fire-rating-to-r120/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-fire-rating/🧪️tests/switches-fire-rating-to-r120/🎯️outcome/🔣️component.json"),
        ),
        "change-provided-axis-distance-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/raises-provided-axis-distance-mm-to-42-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/raises-provided-axis-distance-mm-to-42-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/raises-provided-axis-distance-mm-to-42-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-provided-axis-distance-mm/🧪️tests/raises-provided-axis-distance-mm-to-42-5/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-sigma-c-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/raises-bridge-sigma-c-mpa-to-15-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/raises-bridge-sigma-c-mpa-to-15-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/raises-bridge-sigma-c-mpa-to-15-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-bridge-sigma-c-mpa/🧪️tests/raises-bridge-sigma-c-mpa-to-15-75/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-delta-sigma-s-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/raises-bridge-delta-sigma-s-mpa-to-132-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/raises-bridge-delta-sigma-s-mpa-to-132-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/raises-bridge-delta-sigma-s-mpa-to-132-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-bridge-delta-sigma-s-mpa/🧪️tests/raises-bridge-delta-sigma-s-mpa-to-132-5/🎯️outcome/🔣️component.json"),
        ),
        "change-tightness-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🧪️tests/switches-tightness-class-to-tc2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🧪️tests/switches-tightness-class-to-tc2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🧪️tests/switches-tightness-class-to-tc2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦛change-tightness-class/🧪️tests/switches-tightness-class-to-tc2/🎯️outcome/🔣️component.json"),
        ),
        "change-hd-over-h" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/raises-hd-over-h-to-12-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/raises-hd-over-h-to-12-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/raises-hd-over-h-to-12-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-hd-over-h/🧪️tests/raises-hd-over-h-to-12-5/🎯️outcome/🔣️component.json"),
        ),
        "change-liquid-sigma-s-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/raises-liquid-sigma-s-mpa-to-235-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/raises-liquid-sigma-s-mpa-to-235-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/raises-liquid-sigma-s-mpa-to-235-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-liquid-sigma-s-mpa/🧪️tests/raises-liquid-sigma-s-mpa-to-235-5/🎯️outcome/🔣️component.json"),
        ),
        "change-liquid-rho-p-eff" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/raises-liquid-rho-p-eff-to-0-0078125/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/raises-liquid-rho-p-eff-to-0-0078125/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/raises-liquid-rho-p-eff-to-0-0078125/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-liquid-rho-p-eff/🧪️tests/raises-liquid-rho-p-eff-to-0-0078125/🎯️outcome/🔣️component.json"),
        ),
        "change-liquid-f-ct-eff-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/raises-liquid-f-ct-eff-mpa-to-3-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/raises-liquid-f-ct-eff-mpa-to-3-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/raises-liquid-f-ct-eff-mpa-to-3-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-liquid-f-ct-eff-mpa/🧪️tests/raises-liquid-f-ct-eff-mpa-to-3-25/🎯️outcome/🔣️component.json"),
        ),
        "change-liquid-es-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/raises-liquid-e-s-mpa-to-205000-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/raises-liquid-e-s-mpa-to-205000-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/raises-liquid-e-s-mpa-to-205000-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-liquid-es-mpa/🧪️tests/raises-liquid-e-s-mpa-to-205000-0/🎯️outcome/🔣️component.json"),
        ),
        "change-liquid-sr-max-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/raises-liquid-s-r-max-mm-to-312-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/raises-liquid-s-r-max-mm-to-312-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/raises-liquid-s-r-max-mm-to-312-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-liquid-sr-max-mm/🧪️tests/raises-liquid-s-r-max-mm-to-312-5/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-h-ef-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/raises-anchor-h-ef-mm-to-105-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/raises-anchor-h-ef-mm-to-105-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/raises-anchor-h-ef-mm-to-105-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-anchor-h-ef-mm/🧪️tests/raises-anchor-h-ef-mm-to-105-0/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-cracked" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🧪️tests/turns-anchor-cracked-on/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🧪️tests/turns-anchor-cracked-on/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🧪️tests/turns-anchor-cracked-on/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-anchor-cracked/🧪️tests/turns-anchor-cracked-on/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-f-uk-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/raises-anchor-f-uk-mpa-to-900-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/raises-anchor-f-uk-mpa-to-900-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/raises-anchor-f-uk-mpa-to-900-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-anchor-f-uk-mpa/🧪️tests/raises-anchor-f-uk-mpa-to-900-0/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-f-yk-mpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🧪️tests/raises-anchor-f-yk-mpa-to-720-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🧪️tests/raises-anchor-f-yk-mpa-to-720-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🧪️tests/raises-anchor-f-yk-mpa-to-720-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-anchor-f-yk-mpa/🧪️tests/raises-anchor-f-yk-mpa-to-720-0/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-as-mm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/raises-anchor-a-s-mm2-to-157-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/raises-anchor-a-s-mm2-to-157-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/raises-anchor-a-s-mm2-to-157-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-anchor-as-mm2/🧪️tests/raises-anchor-a-s-mm2-to-157-0/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-d-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🧪️tests/raises-anchor-d-mm-to-16-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🧪️tests/raises-anchor-d-mm-to-16-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🧪️tests/raises-anchor-d-mm-to-16-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-anchor-d-mm/🧪️tests/raises-anchor-d-mm-to-16-0/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-c1-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/raises-anchor-c1-mm-to-137-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/raises-anchor-c1-mm-to-137-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/raises-anchor-c1-mm-to-137-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-anchor-c1-mm/🧪️tests/raises-anchor-c1-mm-to-137-5/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-n-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/raises-anchor-n-ed-kn-to-22-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/raises-anchor-n-ed-kn-to-22-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/raises-anchor-n-ed-kn-to-22-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-anchor-n-ed-kn/🧪️tests/raises-anchor-n-ed-kn-to-22-5/🎯️outcome/🔣️component.json"),
        ),
        "change-anchor-v-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/raises-anchor-v-ed-kn-to-11-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/raises-anchor-v-ed-kn-to-11-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/raises-anchor-v-ed-kn-to-11-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-anchor-v-ed-kn/🧪️tests/raises-anchor-v-ed-kn-to-11-25/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1992-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1992::standards::v1::subsets::any::schema::mutations::{apply_en1992_mutation, decode_en1992_mutation_json, inverse_en1992_mutation, En1992Mutation};
    use semio_s_plugin_norm::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::{decode_en1992_dsl, decode_en1992_pack, decode_en1992_snapshot_json, encode_en1992_dsl, encode_en1992_pack, encode_en1992_snapshot_json, En1992Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
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
            let steps = inverse_en1992_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1992_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
