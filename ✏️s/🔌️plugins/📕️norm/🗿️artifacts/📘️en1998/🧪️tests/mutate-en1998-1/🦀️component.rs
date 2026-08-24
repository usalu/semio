//! 🦀️ EN 1998 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). Recorded
//! no-oracle decision `en1998-1-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.norm.en1998` is a
//! semio-native artifact with no third-party reader or writer, so the `oracle` handlers here
//! read the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no reimplementation of mutation semantics — while `subject` drives this
//! repository's own `apply_en1998_mutation` over the full 49-kind `En1998Mutation` vocabulary.
//!
//! Forty-nine document-root scalars and booleans, one `change-<field>` each — the
//! second-largest vocabulary in the plugin — spanning seven of EN 1998's own structure classes
//! in ONE document: buildings (seismic zone, ground type, importance class, structural system,
//! T_1, mass, V_Rd, drift, height, the multiple-resisting-systems flag), the EN-annex spectrum
//! (a_gR, ground type, spectrum type, period ratio), bridges (V_Rd, bearing displacement demand
//! and capacity), retrofit assessment (knowledge level, limit state, E_d, R_k, gamma_el), silos
//! and tanks (height, radius, N_Rd, V_Ed, V_Rd, behaviour factor q, plus the tank mass and
//! V_Rd), towers and chimneys (M_Ed, M_Rd, the chimney flag, q, mass), foundations (area, p_Rd,
//! H_Ed, H_Rd, the two stiffness factors k) and retaining walls (height, phi, soil gamma, the
//! ductility factor r, H_Rd).
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
//! (`decode_en1998_snapshot_json`/`encode_en1998_snapshot_json`,
//! `decode_en1998_dsl`/`encode_en1998_dsl`, `decode_en1998_pack`/`encode_en1998_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`;
//! `decode_en1998_mutation_json`, `apply_en1998_mutation`, `inverse_en1998_mutation` in
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
/// 🏷️ Mirrors `En1998Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "change-seismic-zone",
    "change-ground-type",
    "change-importance-class",
    "change-structural-system",
    "change-t1-s",
    "change-mass-t",
    "change-v-rd-kn",
    "change-drift-mm",
    "change-height-m",
    "change-multiple-resisting-systems",
    "change-annex",
    "change-en-a-gr",
    "change-en-ground-type",
    "change-en-spectrum-type",
    "change-period-ratio",
    "change-bridge-v-rd-kn",
    "change-bearing-d-ed-mm",
    "change-bearing-d-rd-mm",
    "change-retrofit-knowledge-level",
    "change-retrofit-limit-state",
    "change-retrofit-ed-kn",
    "change-retrofit-rk-kn",
    "change-retrofit-gamma-el",
    "change-silo-height-m",
    "change-silo-radius-m",
    "change-silo-n-rd-kn",
    "change-silo-v-ed-kn",
    "change-silo-v-rd-kn",
    "change-silo-q-nominal",
    "change-tank-height-m",
    "change-tank-radius-m",
    "change-tank-mass-t",
    "change-tank-v-rd-kn",
    "change-tower-m-ed-knm",
    "change-tower-m-rd-knm",
    "change-tower-is-chimney",
    "change-tower-q-nominal",
    "change-tower-mass-t",
    "change-foundation-area-m2",
    "change-foundation-p-rd-kpa",
    "change-foundation-h-ed-kn",
    "change-foundation-h-rd-kn",
    "change-k-foundation",
    "change-k-soil",
    "change-wall-height-m",
    "change-wall-phi-deg",
    "change-wall-soil-gamma-kn-m3",
    "change-wall-r",
    "change-wall-h-rd-kn",
];

/// 🗣️ The real committed EN 1998 document, read where the domain already keeps it.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🎒️seismic-rc-frame.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-seismic-zone" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🧪️tests/raises-seismic-zone-to-4/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🧪️tests/raises-seismic-zone-to-4/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🧪️tests/raises-seismic-zone-to-4/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌼change-seismic-zone/🧪️tests/raises-seismic-zone-to-4/🎯️outcome/🔣️component.json"),
        ),
        "change-ground-type" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🧪️tests/switches-ground-type-to-c/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🧪️tests/switches-ground-type-to-c/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🧪️tests/switches-ground-type-to-c/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🍄change-ground-type/🧪️tests/switches-ground-type-to-c/🎯️outcome/🔣️component.json"),
        ),
        "change-importance-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🧪️tests/switches-importance-class-to-cc3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🧪️tests/switches-importance-class-to-cc3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🧪️tests/switches-importance-class-to-cc3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌰change-importance-class/🧪️tests/switches-importance-class-to-cc3/🎯️outcome/🔣️component.json"),
        ),
        "change-structural-system" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🧪️tests/switches-structural-system-to-wall-dcm/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🧪️tests/switches-structural-system-to-wall-dcm/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🧪️tests/switches-structural-system-to-wall-dcm/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊change-structural-system/🧪️tests/switches-structural-system-to-wall-dcm/🎯️outcome/🔣️component.json"),
        ),
        "change-t1-s" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🧪️tests/raises-t1-s-to-0-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🧪️tests/raises-t1-s-to-0-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🧪️tests/raises-t1-s-to-0-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐚change-t1-s/🧪️tests/raises-t1-s-to-0-75/🎯️outcome/🔣️component.json"),
        ),
        "change-mass-t" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🧪️tests/raises-mass-t-to-812-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🧪️tests/raises-mass-t-to-812-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🧪️tests/raises-mass-t-to-812-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪨change-mass-t/🧪️tests/raises-mass-t-to-812-5/🎯️outcome/🔣️component.json"),
        ),
        "change-v-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🧪️tests/raises-v-rd-kn-to-925-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🧪️tests/raises-v-rd-kn-to-925-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🧪️tests/raises-v-rd-kn-to-925-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️change-v-rd-kn/🧪️tests/raises-v-rd-kn-to-925-0/🎯️outcome/🔣️component.json"),
        ),
        "change-drift-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🧪️tests/raises-drift-mm-to-33-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🧪️tests/raises-drift-mm-to-33-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🧪️tests/raises-drift-mm-to-33-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌎️change-drift-mm/🧪️tests/raises-drift-mm-to-33-5/🎯️outcome/🔣️component.json"),
        ),
        "change-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🧪️tests/raises-height-m-to-18-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🧪️tests/raises-height-m-to-18-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🧪️tests/raises-height-m-to-18-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌏️change-height-m/🧪️tests/raises-height-m-to-18-75/🎯️outcome/🔣️component.json"),
        ),
        "change-multiple-resisting-systems" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🧪️tests/turns-multiple-resisting-systems-off/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🧪️tests/turns-multiple-resisting-systems-off/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🧪️tests/turns-multiple-resisting-systems-off/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-multiple-resisting-systems/🧪️tests/turns-multiple-resisting-systems-off/🎯️outcome/🔣️component.json"),
        ),
        "change-annex" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🧪️tests/switches-annex-to-en/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🧪️tests/switches-annex-to-en/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🧪️tests/switches-annex-to-en/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗻change-annex/🧪️tests/switches-annex-to-en/🎯️outcome/🔣️component.json"),
        ),
        "change-en-a-gr" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🧪️tests/raises-en-a-gr-to-0-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🧪️tests/raises-en-a-gr-to-0-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🧪️tests/raises-en-a-gr-to-0-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-en-a-gr/🧪️tests/raises-en-a-gr-to-0-25/🎯️outcome/🔣️component.json"),
        ),
        "change-en-ground-type" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🧪️tests/switches-en-ground-type-to-e/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🧪️tests/switches-en-ground-type-to-e/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🧪️tests/switches-en-ground-type-to-e/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛰️change-en-ground-type/🧪️tests/switches-en-ground-type-to-e/🎯️outcome/🔣️component.json"),
        ),
        "change-en-spectrum-type" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🧪️tests/switches-en-spectrum-type-to-type2/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🧪️tests/switches-en-spectrum-type-to-type2/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🧪️tests/switches-en-spectrum-type-to-type2/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏕️change-en-spectrum-type/🧪️tests/switches-en-spectrum-type-to-type2/🎯️outcome/🔣️component.json"),
        ),
        "change-period-ratio" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🧪️tests/raises-period-ratio-to-3-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🧪️tests/raises-period-ratio-to-3-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🧪️tests/raises-period-ratio-to-3-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏖️change-period-ratio/🧪️tests/raises-period-ratio-to-3-5/🎯️outcome/🔣️component.json"),
        ),
        "change-bridge-v-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🧪️tests/raises-bridge-v-rd-kn-to-725-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🧪️tests/raises-bridge-v-rd-kn-to-725-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🧪️tests/raises-bridge-v-rd-kn-to-725-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏜️change-bridge-v-rd-kn/🧪️tests/raises-bridge-v-rd-kn-to-725-0/🎯️outcome/🔣️component.json"),
        ),
        "change-bearing-d-ed-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🧪️tests/raises-bearing-d-ed-mm-to-165-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🧪️tests/raises-bearing-d-ed-mm-to-165-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🧪️tests/raises-bearing-d-ed-mm-to-165-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏝️change-bearing-d-ed-mm/🧪️tests/raises-bearing-d-ed-mm-to-165-5/🎯️outcome/🔣️component.json"),
        ),
        "change-bearing-d-rd-mm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🧪️tests/raises-bearing-d-rd-mm-to-312-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🧪️tests/raises-bearing-d-rd-mm-to-312-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🧪️tests/raises-bearing-d-rd-mm-to-312-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏞️change-bearing-d-rd-mm/🧪️tests/raises-bearing-d-rd-mm-to-312-5/🎯️outcome/🔣️component.json"),
        ),
        "change-retrofit-knowledge-level" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🧪️tests/switches-retrofit-knowledge-level-to-kl3/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🧪️tests/switches-retrofit-knowledge-level-to-kl3/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🧪️tests/switches-retrofit-knowledge-level-to-kl3/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏟️change-retrofit-knowledge-level/🧪️tests/switches-retrofit-knowledge-level-to-kl3/🎯️outcome/🔣️component.json"),
        ),
        "change-retrofit-limit-state" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🧪️tests/switches-retrofit-limit-state-to-near-collapse/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🧪️tests/switches-retrofit-limit-state-to-near-collapse/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🧪️tests/switches-retrofit-limit-state-to-near-collapse/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪵change-retrofit-limit-state/🧪️tests/switches-retrofit-limit-state-to-near-collapse/🎯️outcome/🔣️component.json"),
        ),
        "change-retrofit-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🧪️tests/raises-retrofit-e-d-kn-to-337-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🧪️tests/raises-retrofit-e-d-kn-to-337-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🧪️tests/raises-retrofit-e-d-kn-to-337-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐝change-retrofit-ed-kn/🧪️tests/raises-retrofit-e-d-kn-to-337-5/🎯️outcome/🔣️component.json"),
        ),
        "change-retrofit-rk-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🧪️tests/raises-retrofit-r-k-kn-to-512-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🧪️tests/raises-retrofit-r-k-kn-to-512-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🧪️tests/raises-retrofit-r-k-kn-to-512-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐞change-retrofit-rk-kn/🧪️tests/raises-retrofit-r-k-kn-to-512-5/🎯️outcome/🔣️component.json"),
        ),
        "change-retrofit-gamma-el" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🧪️tests/raises-retrofit-gamma-el-to-1-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🧪️tests/raises-retrofit-gamma-el-to-1-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🧪️tests/raises-retrofit-gamma-el-to-1-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦋change-retrofit-gamma-el/🧪️tests/raises-retrofit-gamma-el-to-1-25/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🧪️tests/raises-silo-height-m-to-14-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🧪️tests/raises-silo-height-m-to-14-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🧪️tests/raises-silo-height-m-to-14-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐌change-silo-height-m/🧪️tests/raises-silo-height-m-to-14-5/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-radius-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🧪️tests/raises-silo-radius-m-to-6-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🧪️tests/raises-silo-radius-m-to-6-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🧪️tests/raises-silo-radius-m-to-6-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐢change-silo-radius-m/🧪️tests/raises-silo-radius-m-to-6-25/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-n-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🧪️tests/raises-silo-n-rd-kn-to-640-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🧪️tests/raises-silo-n-rd-kn-to-640-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🧪️tests/raises-silo-n-rd-kn-to-640-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐬change-silo-n-rd-kn/🧪️tests/raises-silo-n-rd-kn-to-640-0/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-v-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🧪️tests/raises-silo-v-ed-kn-to-225-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🧪️tests/raises-silo-v-ed-kn-to-225-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🧪️tests/raises-silo-v-ed-kn-to-225-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐳change-silo-v-ed-kn/🧪️tests/raises-silo-v-ed-kn-to-225-5/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-v-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🧪️tests/raises-silo-v-rd-kn-to-412-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🧪️tests/raises-silo-v-rd-kn-to-412-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🧪️tests/raises-silo-v-rd-kn-to-412-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦈change-silo-v-rd-kn/🧪️tests/raises-silo-v-rd-kn-to-412-5/🎯️outcome/🔣️component.json"),
        ),
        "change-silo-q-nominal" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🧪️tests/raises-silo-q-nominal-to-2-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🧪️tests/raises-silo-q-nominal-to-2-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🧪️tests/raises-silo-q-nominal-to-2-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦭change-silo-q-nominal/🧪️tests/raises-silo-q-nominal-to-2-75/🎯️outcome/🔣️component.json"),
        ),
        "change-tank-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🧪️tests/raises-tank-height-m-to-11-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🧪️tests/raises-tank-height-m-to-11-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🧪️tests/raises-tank-height-m-to-11-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐊change-tank-height-m/🧪️tests/raises-tank-height-m-to-11-5/🎯️outcome/🔣️component.json"),
        ),
        "change-tank-radius-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🧪️tests/raises-tank-radius-m-to-5-75/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🧪️tests/raises-tank-radius-m-to-5-75/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🧪️tests/raises-tank-radius-m-to-5-75/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦎change-tank-radius-m/🧪️tests/raises-tank-radius-m-to-5-75/🎯️outcome/🔣️component.json"),
        ),
        "change-tank-mass-t" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🧪️tests/raises-tank-mass-t-to-425-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🧪️tests/raises-tank-mass-t-to-425-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🧪️tests/raises-tank-mass-t-to-425-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐍change-tank-mass-t/🧪️tests/raises-tank-mass-t-to-425-0/🎯️outcome/🔣️component.json"),
        ),
        "change-tank-v-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🧪️tests/raises-tank-v-rd-kn-to-537-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🧪️tests/raises-tank-v-rd-kn-to-537-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🧪️tests/raises-tank-v-rd-kn-to-537-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦂change-tank-v-rd-kn/🧪️tests/raises-tank-v-rd-kn-to-537-5/🎯️outcome/🔣️component.json"),
        ),
        "change-tower-m-ed-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🧪️tests/raises-tower-m-ed-knm-to-1562-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🧪️tests/raises-tower-m-ed-knm-to-1562-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🧪️tests/raises-tower-m-ed-knm-to-1562-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦟change-tower-m-ed-knm/🧪️tests/raises-tower-m-ed-knm-to-1562-5/🎯️outcome/🔣️component.json"),
        ),
        "change-tower-m-rd-knm" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🧪️tests/raises-tower-m-rd-knm-to-2812-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🧪️tests/raises-tower-m-rd-knm-to-2812-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🧪️tests/raises-tower-m-rd-knm-to-2812-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦗change-tower-m-rd-knm/🧪️tests/raises-tower-m-rd-knm-to-2812-5/🎯️outcome/🔣️component.json"),
        ),
        "change-tower-is-chimney" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🧪️tests/turns-tower-is-chimney-off/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🧪️tests/turns-tower-is-chimney-off/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🧪️tests/turns-tower-is-chimney-off/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕷️change-tower-is-chimney/🧪️tests/turns-tower-is-chimney-off/🎯️outcome/🔣️component.json"),
        ),
        "change-tower-q-nominal" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🧪️tests/raises-tower-q-nominal-to-3-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🧪️tests/raises-tower-q-nominal-to-3-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🧪️tests/raises-tower-q-nominal-to-3-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐜change-tower-q-nominal/🧪️tests/raises-tower-q-nominal-to-3-25/🎯️outcome/🔣️component.json"),
        ),
        "change-tower-mass-t" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🧪️tests/raises-tower-mass-t-to-112-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🧪️tests/raises-tower-mass-t-to-112-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🧪️tests/raises-tower-mass-t-to-112-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦔change-tower-mass-t/🧪️tests/raises-tower-mass-t-to-112-5/🎯️outcome/🔣️component.json"),
        ),
        "change-foundation-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🧪️tests/raises-foundation-area-m2-to-144-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🧪️tests/raises-foundation-area-m2-to-144-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🧪️tests/raises-foundation-area-m2-to-144-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦇change-foundation-area-m2/🧪️tests/raises-foundation-area-m2-to-144-0/🎯️outcome/🔣️component.json"),
        ),
        "change-foundation-p-rd-kpa" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🧪️tests/raises-foundation-p-rd-kpa-to-625-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🧪️tests/raises-foundation-p-rd-kpa-to-625-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🧪️tests/raises-foundation-p-rd-kpa-to-625-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦉change-foundation-p-rd-kpa/🧪️tests/raises-foundation-p-rd-kpa-to-625-0/🎯️outcome/🔣️component.json"),
        ),
        "change-foundation-h-ed-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🧪️tests/raises-foundation-h-ed-kn-to-212-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🧪️tests/raises-foundation-h-ed-kn-to-212-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🧪️tests/raises-foundation-h-ed-kn-to-212-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐴change-foundation-h-ed-kn/🧪️tests/raises-foundation-h-ed-kn-to-212-5/🎯️outcome/🔣️component.json"),
        ),
        "change-foundation-h-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🧪️tests/raises-foundation-h-rd-kn-to-475-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🧪️tests/raises-foundation-h-rd-kn-to-475-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🧪️tests/raises-foundation-h-rd-kn-to-475-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐎change-foundation-h-rd-kn/🧪️tests/raises-foundation-h-rd-kn-to-475-0/🎯️outcome/🔣️component.json"),
        ),
        "change-k-foundation" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🧪️tests/raises-k-foundation-to-640000-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🧪️tests/raises-k-foundation-to-640000-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🧪️tests/raises-k-foundation-to-640000-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦄change-k-foundation/🧪️tests/raises-k-foundation-to-640000-0/🎯️outcome/🔣️component.json"),
        ),
        "change-k-soil" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🧪️tests/raises-k-soil-to-262500-0/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🧪️tests/raises-k-soil-to-262500-0/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🧪️tests/raises-k-soil-to-262500-0/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐑change-k-soil/🧪️tests/raises-k-soil-to-262500-0/🎯️outcome/🔣️component.json"),
        ),
        "change-wall-height-m" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🧪️tests/raises-wall-height-m-to-5-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🧪️tests/raises-wall-height-m-to-5-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🧪️tests/raises-wall-height-m-to-5-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐐change-wall-height-m/🧪️tests/raises-wall-height-m-to-5-5/🎯️outcome/🔣️component.json"),
        ),
        "change-wall-phi-deg" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🧪️tests/raises-wall-phi-deg-to-37-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🧪️tests/raises-wall-phi-deg-to-37-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🧪️tests/raises-wall-phi-deg-to-37-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐮change-wall-phi-deg/🧪️tests/raises-wall-phi-deg-to-37-5/🎯️outcome/🔣️component.json"),
        ),
        "change-wall-soil-gamma-kn-m3" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🧪️tests/raises-wall-soil-gamma-kn-m3-to-20-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🧪️tests/raises-wall-soil-gamma-kn-m3-to-20-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🧪️tests/raises-wall-soil-gamma-kn-m3-to-20-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐷change-wall-soil-gamma-kn-m3/🧪️tests/raises-wall-soil-gamma-kn-m3-to-20-5/🎯️outcome/🔣️component.json"),
        ),
        "change-wall-r" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🧪️tests/raises-wall-r-to-2-25/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🧪️tests/raises-wall-r-to-2-25/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🧪️tests/raises-wall-r-to-2-25/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐗change-wall-r/🧪️tests/raises-wall-r-to-2-25/🎯️outcome/🔣️component.json"),
        ),
        "change-wall-h-rd-kn" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🧪️tests/raises-wall-h-rd-kn-to-187-5/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🧪️tests/raises-wall-h-rd-kn-to-187-5/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🧪️tests/raises-wall-h-rd-kn-to-187-5/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦌change-wall-h-rd-kn/🧪️tests/raises-wall-h-rd-kn-to-187-5/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-en1998-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::en1998::standards::v1::subsets::any::schema::mutations::{apply_en1998_mutation, decode_en1998_mutation_json, inverse_en1998_mutation, En1998Mutation};
    use semio_s_plugin_norm::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::{decode_en1998_dsl, decode_en1998_pack, decode_en1998_snapshot_json, encode_en1998_dsl, encode_en1998_pack, encode_en1998_snapshot_json, En1998Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<En1998Snapshot, String> {
        decode_en1998_snapshot_json(text).map_err(|error| format!("mutate-en1998-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<En1998Mutation, String> {
        decode_en1998_mutation_json(text).map_err(|error| format!("mutate-en1998-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &En1998Snapshot) -> Result<Json, String> {
        parse_json(&encode_en1998_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &En1998Snapshot, expected: &En1998Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_en1998_snapshot_json(got), encode_en1998_snapshot_json(expected))
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
            let applied = apply_en1998_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_en1998_snapshot_json(&snapshot))),
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
            let mut current = match apply_en1998_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let steps = inverse_en1998_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_en1998_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed EN 1998 artifact is not UTF-8: {error}"))?;
        let parsed = decode_en1998_dsl(&text)?;
        let reprinted = encode_en1998_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_en1998_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_en1998_pack(&encode_en1998_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_en1998_snapshot_json(&encode_en1998_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        let twin = decode_en1998_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
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
