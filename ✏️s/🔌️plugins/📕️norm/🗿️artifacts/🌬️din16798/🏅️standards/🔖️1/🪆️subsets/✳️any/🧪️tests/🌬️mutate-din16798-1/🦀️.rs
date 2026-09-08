//! 🦀️ DIN EN 16798-1 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `din16798-1-mutation-semantics` is gone from
//! `../../🔣️oracle.json`, because a reference now
//! exists to compare against: `s.norm.din16798` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️component.py` beside this file, registered as the
//! oracle `din16798-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_din16798_mutation` over the full 62-kind `Din16798Mutation`
//! vocabulary.
//!
//! Sixty-two document-root scalars and not one collection: this is the largest flat mutation
//! vocabulary in the repository, and every kind is a `change-<field>`. The fields group into
//! the standard's own clause families — thermal comfort (operative temperature, humidity,
//! draught air speed, running-mean outdoor temperature), air quality (CO2, IDA class, supply
//! airflow), daylight and acoustics, three separate occupancy models (non-residential persons,
//! dwelling bedrooms, residential occupants) each with their own airflow field, specific fan
//! power, heat recovery (achieved and required efficiency, mass flow, specific heat,
//! temperature lift, operating hours, savings reference), infiltration and blower-door, cellar
//! ventilation, transmission and ventilation heat transfer, cooling (set point, period, gains,
//! utilization factor, reference, chiller type, EER, annual demand), storage and DHW, and duct
//! leakage.
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
//! (`decode_din16798_snapshot_json`/`encode_din16798_snapshot_json`,
//! `decode_din16798_dsl`/`encode_din16798_dsl`, `decode_din16798_pack`/`encode_din16798_pack`
//! in `../../🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_din16798_mutation_json`, `apply_din16798_mutation`, `inverse_din16798_mutation` in
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
/// 🏷️ Mirrors `Din16798Mutation::KINDS` (`../../🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-annex",
    "change-occupancy",
    "change-comfort-category",
    "change-t-op-c",
    "change-rh-percent",
    "change-air-speed-ms",
    "change-theta-rm-c",
    "change-co2-ppm",
    "change-df-percent",
    "change-l-aeq-db",
    "change-persons",
    "change-ida-class",
    "change-ventilation-m3-h",
    "change-floor-area-m2",
    "change-bedrooms",
    "change-dwelling-ventilation-m3-h",
    "change-occupants",
    "change-residential-ventilation-m3-h",
    "change-sfp-wm3-s",
    "change-sfp-required-class",
    "change-heat-recovery-eta",
    "change-heat-recovery-eta-min",
    "change-system-type",
    "change-years-since-inspection",
    "change-humidification-required-kg-h",
    "change-humidification-provided-kg-h",
    "change-fan-qvm3-s",
    "change-fan-t-run-h",
    "change-fan-energy-reference-kwh",
    "change-night-setback-k",
    "change-hr-m-dot-kg-s",
    "change-hr-cp-j-kgk",
    "change-hr-delta-tc",
    "change-hr-th",
    "change-hr-savings-reference-kwh",
    "change-n50-h-inv",
    "change-volume-m3",
    "change-infiltration-allowance-m3-h",
    "change-cellar-area-m2",
    "change-cellar-ventilation-m3-h",
    "change-h-tr-wk",
    "change-h-ve-wk",
    "change-theta-ec",
    "change-theta-set-c",
    "change-cooling-delta-th",
    "change-cooling-gains-kwh",
    "change-cooling-utilization-factor",
    "change-cooling-reference-kwh",
    "change-chiller-type",
    "change-eer-actual",
    "change-qc-kwh",
    "change-generation-reference-kwh",
    "change-data-center-supply-c",
    "change-h-st-wk",
    "change-theta-st-c",
    "change-theta-amb-c",
    "change-storage-th",
    "change-storage-allowance-kwh",
    "change-dhw-delivery-c",
    "change-duct-class",
    "change-duct-test-pressure-pa",
    "change-duct-leakage-m3-sm2",
];

/// 🗣️ The real committed DIN EN 16798-1 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
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
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-the-check-fc8da5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-the-check-fc8da5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-the-check-fc8da5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌍️change-annex/🧪️tests/🌍️switches-the-check-fc8da5/🎯️outcome/🔣️.json"),
        ),
        "change-occupancy" => (
            include_str!("../../🧬️schema/🧬️mutations/🏢️change-occupancy/🧪️tests/🏢️reclassifies-the-62ec97/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏢️change-occupancy/🧪️tests/🏢️reclassifies-the-62ec97/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏢️change-occupancy/🧪️tests/🏢️reclassifies-the-62ec97/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏢️change-occupancy/🧪️tests/🏢️reclassifies-the-62ec97/🎯️outcome/🔣️.json"),
        ),
        "change-comfort-category" => (
            include_str!("../../🧬️schema/🧬️mutations/🛋️change-comfort-category/🧪️tests/🛋️tightens-the-2ac419/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛋️change-comfort-category/🧪️tests/🛋️tightens-the-2ac419/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛋️change-comfort-category/🧪️tests/🛋️tightens-the-2ac419/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛋️change-comfort-category/🧪️tests/🛋️tightens-the-2ac419/🎯️outcome/🔣️.json"),
        ),
        "change-t-op-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-t-op-c/🧪️tests/🌡️raises-the-c09dc0/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-t-op-c/🧪️tests/🌡️raises-the-c09dc0/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-t-op-c/🧪️tests/🌡️raises-the-c09dc0/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌡️change-t-op-c/🧪️tests/🌡️raises-the-c09dc0/🎯️outcome/🔣️.json"),
        ),
        "change-rh-percent" => (
            include_str!("../../🧬️schema/🧬️mutations/💧️change-rh-percent/🧪️tests/💧️drops-indoor-9a0a67/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-rh-percent/🧪️tests/💧️drops-indoor-9a0a67/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-rh-percent/🧪️tests/💧️drops-indoor-9a0a67/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💧️change-rh-percent/🧪️tests/💧️drops-indoor-9a0a67/🎯️outcome/🔣️.json"),
        ),
        "change-air-speed-ms" => (
            include_str!("../../🧬️schema/🧬️mutations/🪁️change-air-speed-ms/🧪️tests/🪁️doubles-the-e80788/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪁️change-air-speed-ms/🧪️tests/🪁️doubles-the-e80788/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪁️change-air-speed-ms/🧪️tests/🪁️doubles-the-e80788/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪁️change-air-speed-ms/🧪️tests/🪁️doubles-the-e80788/🎯️outcome/🔣️.json"),
        ),
        "change-theta-rm-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🔄️change-theta-rm-c/🧪️tests/🔬️t020/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔄️change-theta-rm-c/🧪️tests/🔬️t020/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔄️change-theta-rm-c/🧪️tests/🔬️t020/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔄️change-theta-rm-c/🧪️tests/🔬️t020/🎯️outcome/🔣️.json"),
        ),
        "change-co2-ppm" => (
            include_str!("../../🧬️schema/🧬️mutations/🌫️change-co2-ppm/🧪️tests/🌫️raises-the-e2d85a/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌫️change-co2-ppm/🧪️tests/🌫️raises-the-e2d85a/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌫️change-co2-ppm/🧪️tests/🌫️raises-the-e2d85a/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌫️change-co2-ppm/🧪️tests/🌫️raises-the-e2d85a/🎯️outcome/🔣️.json"),
        ),
        "change-df-percent" => (
            include_str!("../../🧬️schema/🧬️mutations/☀️change-df-percent/🧪️tests/☀️raises-the-d09f1b/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☀️change-df-percent/🧪️tests/☀️raises-the-d09f1b/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☀️change-df-percent/🧪️tests/☀️raises-the-d09f1b/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☀️change-df-percent/🧪️tests/☀️raises-the-d09f1b/🎯️outcome/🔣️.json"),
        ),
        "change-l-aeq-db" => (
            include_str!("../../🧬️schema/🧬️mutations/🔊️change-l-aeq-db/🧪️tests/🔊️raises-the-3478d8/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔊️change-l-aeq-db/🧪️tests/🔊️raises-the-3478d8/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔊️change-l-aeq-db/🧪️tests/🔊️raises-the-3478d8/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔊️change-l-aeq-db/🧪️tests/🔊️raises-the-3478d8/🎯️outcome/🔣️.json"),
        ),
        "change-persons" => (
            include_str!("../../🧬️schema/🧬️mutations/👥️change-persons/🧪️tests/👥️raises-the-d27cc4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👥️change-persons/🧪️tests/👥️raises-the-d27cc4/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👥️change-persons/🧪️tests/👥️raises-the-d27cc4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👥️change-persons/🧪️tests/👥️raises-the-d27cc4/🎯️outcome/🔣️.json"),
        ),
        "change-ida-class" => (
            include_str!("../../🧬️schema/🧬️mutations/🫁️change-ida-class/🧪️tests/🫁️relaxes-the-37997b/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫁️change-ida-class/🧪️tests/🫁️relaxes-the-37997b/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫁️change-ida-class/🧪️tests/🫁️relaxes-the-37997b/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫁️change-ida-class/🧪️tests/🫁️relaxes-the-37997b/🎯️outcome/🔣️.json"),
        ),
        "change-ventilation-m3-h" => (
            include_str!("../../🧬️schema/🧬️mutations/💨️change-ventilation-m3-h/🧪️tests/💨️raises-the-5feae1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💨️change-ventilation-m3-h/🧪️tests/💨️raises-the-5feae1/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💨️change-ventilation-m3-h/🧪️tests/💨️raises-the-5feae1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💨️change-ventilation-m3-h/🧪️tests/💨️raises-the-5feae1/🎯️outcome/🔣️.json"),
        ),
        "change-floor-area-m2" => (
            include_str!("../../🧬️schema/🧬️mutations/📐️change-floor-area-m2/🧪️tests/📐️grows-the-b01a82/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-floor-area-m2/🧪️tests/📐️grows-the-b01a82/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-floor-area-m2/🧪️tests/📐️grows-the-b01a82/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📐️change-floor-area-m2/🧪️tests/📐️grows-the-b01a82/🎯️outcome/🔣️.json"),
        ),
        "change-bedrooms" => (
            include_str!("../../🧬️schema/🧬️mutations/🛏️change-bedrooms/🧪️tests/🛏️adds-a-fourth-bedroom/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛏️change-bedrooms/🧪️tests/🛏️adds-a-fourth-bedroom/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛏️change-bedrooms/🧪️tests/🛏️adds-a-fourth-bedroom/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛏️change-bedrooms/🧪️tests/🛏️adds-a-fourth-bedroom/🎯️outcome/🔣️.json"),
        ),
        "change-dwelling-ventilation-m3-h" => (
            include_str!("../../🧬️schema/🧬️mutations/🏡️change-dwelling-ventilation-m3-h/🧪️tests/🔬️t016/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏡️change-dwelling-ventilation-m3-h/🧪️tests/🔬️t016/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏡️change-dwelling-ventilation-m3-h/🧪️tests/🔬️t016/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏡️change-dwelling-ventilation-m3-h/🧪️tests/🔬️t016/🎯️outcome/🔣️.json"),
        ),
        "change-occupants" => (
            include_str!("../../🧬️schema/🧬️mutations/👪️change-occupants/🧪️tests/👪️raises-the-0ba810/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👪️change-occupants/🧪️tests/👪️raises-the-0ba810/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👪️change-occupants/🧪️tests/👪️raises-the-0ba810/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/👪️change-occupants/🧪️tests/👪️raises-the-0ba810/🎯️outcome/🔣️.json"),
        ),
        "change-residential-ventilation-m3-h" => (
            include_str!("../../🧬️schema/🧬️mutations/🏘️change-residential-ventilation-m3-h/🧪️tests/🔬️t015/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏘️change-residential-ventilation-m3-h/🧪️tests/🔬️t015/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏘️change-residential-ventilation-m3-h/🧪️tests/🔬️t015/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏘️change-residential-ventilation-m3-h/🧪️tests/🔬️t015/🎯️outcome/🔣️.json"),
        ),
        "change-sfp-wm3-s" => (
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-sfp-wm3-s/🧪️tests/🌀️improves-the-14eb93/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-sfp-wm3-s/🧪️tests/🌀️improves-the-14eb93/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-sfp-wm3-s/🧪️tests/🌀️improves-the-14eb93/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌀️change-sfp-wm3-s/🧪️tests/🌀️improves-the-14eb93/🎯️outcome/🔣️.json"),
        ),
        "change-sfp-required-class" => (
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-sfp-required-class/🧪️tests/🏷️tightens-the-da6640/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-sfp-required-class/🧪️tests/🏷️tightens-the-da6640/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-sfp-required-class/🧪️tests/🏷️tightens-the-da6640/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏷️change-sfp-required-class/🧪️tests/🏷️tightens-the-da6640/🎯️outcome/🔣️.json"),
        ),
        "change-heat-recovery-eta" => (
            include_str!("../../🧬️schema/🧬️mutations/♻️change-heat-recovery-eta/🧪️tests/♻️raises-the-35a023/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/♻️change-heat-recovery-eta/🧪️tests/♻️raises-the-35a023/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/♻️change-heat-recovery-eta/🧪️tests/♻️raises-the-35a023/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/♻️change-heat-recovery-eta/🧪️tests/♻️raises-the-35a023/🎯️outcome/🔣️.json"),
        ),
        "change-heat-recovery-eta-min" => (
            include_str!("../../🧬️schema/🧬️mutations/🚧️change-heat-recovery-eta-min/🧪️tests/🔬️t022/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚧️change-heat-recovery-eta-min/🧪️tests/🔬️t022/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚧️change-heat-recovery-eta-min/🧪️tests/🔬️t022/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚧️change-heat-recovery-eta-min/🧪️tests/🔬️t022/🎯️outcome/🔣️.json"),
        ),
        "change-system-type" => (
            include_str!("../../🧬️schema/🧬️mutations/⚙️change-system-type/🧪️tests/⚙️switches-to-a-8cbac2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚙️change-system-type/🧪️tests/⚙️switches-to-a-8cbac2/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚙️change-system-type/🧪️tests/⚙️switches-to-a-8cbac2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚙️change-system-type/🧪️tests/⚙️switches-to-a-8cbac2/🎯️outcome/🔣️.json"),
        ),
        "change-years-since-inspection" => (
            include_str!("../../🧬️schema/🧬️mutations/📅️change-years-since-inspection/🧪️tests/📅️ages-the-last-e4ba7d/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📅️change-years-since-inspection/🧪️tests/📅️ages-the-last-e4ba7d/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📅️change-years-since-inspection/🧪️tests/📅️ages-the-last-e4ba7d/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📅️change-years-since-inspection/🧪️tests/📅️ages-the-last-e4ba7d/🎯️outcome/🔣️.json"),
        ),
        "change-humidification-required-kg-h" => (
            include_str!("../../🧬️schema/🧬️mutations/☁️change-humidification-required-kg-h/🧪️tests/☁️required-3-00eb85-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☁️change-humidification-required-kg-h/🧪️tests/☁️required-3-00eb85-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☁️change-humidification-required-kg-h/🧪️tests/☁️required-3-00eb85-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/☁️change-humidification-required-kg-h/🧪️tests/☁️required-3-00eb85-5/🎯️outcome/🔣️.json"),
        ),
        "change-humidification-provided-kg-h" => (
            include_str!("../../🧬️schema/🧬️mutations/💦️change-humidification-provided-kg-h/🧪️tests/💦️provided-4f88c7-25/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💦️change-humidification-provided-kg-h/🧪️tests/💦️provided-4f88c7-25/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💦️change-humidification-provided-kg-h/🧪️tests/💦️provided-4f88c7-25/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💦️change-humidification-provided-kg-h/🧪️tests/💦️provided-4f88c7-25/🎯️outcome/🔣️.json"),
        ),
        "change-fan-qvm3-s" => (
            include_str!("../../🧬️schema/🧬️mutations/🪭️change-fan-qvm3-s/🧪️tests/🪭️raises-the-fan-58c455/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪭️change-fan-qvm3-s/🧪️tests/🪭️raises-the-fan-58c455/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪭️change-fan-qvm3-s/🧪️tests/🪭️raises-the-fan-58c455/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪭️change-fan-qvm3-s/🧪️tests/🪭️raises-the-fan-58c455/🎯️outcome/🔣️.json"),
        ),
        "change-fan-t-run-h" => (
            include_str!("../../🧬️schema/🧬️mutations/⏰️change-fan-t-run-h/🧪️tests/⏰️extends-the-daily-6af5a4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏰️change-fan-t-run-h/🧪️tests/⏰️extends-the-daily-6af5a4/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏰️change-fan-t-run-h/🧪️tests/⏰️extends-the-daily-6af5a4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏰️change-fan-t-run-h/🧪️tests/⏰️extends-the-daily-6af5a4/🎯️outcome/🔣️.json"),
        ),
        "change-fan-energy-reference-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/🔌️change-fan-energy-reference-kwh/🧪️tests/🔌️raises-the-fan-fc91eb/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔌️change-fan-energy-reference-kwh/🧪️tests/🔌️raises-the-fan-fc91eb/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔌️change-fan-energy-reference-kwh/🧪️tests/🔌️raises-the-fan-fc91eb/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔌️change-fan-energy-reference-kwh/🧪️tests/🔌️raises-the-fan-fc91eb/🎯️outcome/🔣️.json"),
        ),
        "change-night-setback-k" => (
            include_str!("../../🧬️schema/🧬️mutations/🌙️change-night-setback-k/🧪️tests/🌙️deepens-the-bb5719/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌙️change-night-setback-k/🧪️tests/🌙️deepens-the-bb5719/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌙️change-night-setback-k/🧪️tests/🌙️deepens-the-bb5719/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌙️change-night-setback-k/🧪️tests/🌙️deepens-the-bb5719/🎯️outcome/🔣️.json"),
        ),
        "change-hr-m-dot-kg-s" => (
            include_str!("../../🧬️schema/🧬️mutations/⚖️change-hr-m-dot-kg-s/🧪️tests/🔬️t013/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚖️change-hr-m-dot-kg-s/🧪️tests/🔬️t013/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚖️change-hr-m-dot-kg-s/🧪️tests/🔬️t013/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚖️change-hr-m-dot-kg-s/🧪️tests/🔬️t013/🎯️outcome/🔣️.json"),
        ),
        "change-hr-cp-j-kgk" => (
            include_str!("../../🧬️schema/🧬️mutations/🥵️change-hr-cp-j-kgk/🧪️tests/🥵️corrects-the-air-3d6338/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥵️change-hr-cp-j-kgk/🧪️tests/🥵️corrects-the-air-3d6338/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥵️change-hr-cp-j-kgk/🧪️tests/🥵️corrects-the-air-3d6338/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥵️change-hr-cp-j-kgk/🧪️tests/🥵️corrects-the-air-3d6338/🎯️outcome/🔣️.json"),
        ),
        "change-hr-delta-tc" => (
            include_str!("../../🧬️schema/🧬️mutations/↕️change-hr-delta-tc/🧪️tests/↕️drops-the-heat-0cb1cd/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-hr-delta-tc/🧪️tests/↕️drops-the-heat-0cb1cd/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-hr-delta-tc/🧪️tests/↕️drops-the-heat-0cb1cd/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/↕️change-hr-delta-tc/🧪️tests/↕️drops-the-heat-0cb1cd/🎯️outcome/🔣️.json"),
        ),
        "change-hr-th" => (
            include_str!("../../🧬️schema/🧬️mutations/⏳️change-hr-th/🧪️tests/⏳️extends-the-heat-9aebd7/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏳️change-hr-th/🧪️tests/⏳️extends-the-heat-9aebd7/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏳️change-hr-th/🧪️tests/⏳️extends-the-heat-9aebd7/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⏳️change-hr-th/🧪️tests/⏳️extends-the-heat-9aebd7/🎯️outcome/🔣️.json"),
        ),
        "change-hr-savings-reference-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/💡️change-hr-savings-reference-kwh/🧪️tests/🔬️t017/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💡️change-hr-savings-reference-kwh/🧪️tests/🔬️t017/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💡️change-hr-savings-reference-kwh/🧪️tests/🔬️t017/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/💡️change-hr-savings-reference-kwh/🧪️tests/🔬️t017/🎯️outcome/🔣️.json"),
        ),
        "change-n50-h-inv" => (
            include_str!("../../🧬️schema/🧬️mutations/🏠️change-n50-h-inv/🧪️tests/🏠️loosens-the-b5e8ca/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏠️change-n50-h-inv/🧪️tests/🏠️loosens-the-b5e8ca/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏠️change-n50-h-inv/🧪️tests/🏠️loosens-the-b5e8ca/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏠️change-n50-h-inv/🧪️tests/🏠️loosens-the-b5e8ca/🎯️outcome/🔣️.json"),
        ),
        "change-volume-m3" => (
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-volume-m3/🧪️tests/🧊️grows-the-air-978ca5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-volume-m3/🧪️tests/🧊️grows-the-air-978ca5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-volume-m3/🧪️tests/🧊️grows-the-air-978ca5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧊️change-volume-m3/🧪️tests/🧊️grows-the-air-978ca5/🎯️outcome/🔣️.json"),
        ),
        "change-infiltration-allowance-m3-h" => (
            include_str!("../../🧬️schema/🧬️mutations/🚪️change-infiltration-allowance-m3-h/🧪️tests/🚪️allowance-d598d5-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚪️change-infiltration-allowance-m3-h/🧪️tests/🚪️allowance-d598d5-5/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚪️change-infiltration-allowance-m3-h/🧪️tests/🚪️allowance-d598d5-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚪️change-infiltration-allowance-m3-h/🧪️tests/🚪️allowance-d598d5-5/🎯️outcome/🔣️.json"),
        ),
        "change-cellar-area-m2" => (
            include_str!("../../🧬️schema/🧬️mutations/🏚️change-cellar-area-m2/🧪️tests/🏚️grows-the-cellar-b21e11/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏚️change-cellar-area-m2/🧪️tests/🏚️grows-the-cellar-b21e11/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏚️change-cellar-area-m2/🧪️tests/🏚️grows-the-cellar-b21e11/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🏚️change-cellar-area-m2/🧪️tests/🏚️grows-the-cellar-b21e11/🎯️outcome/🔣️.json"),
        ),
        "change-cellar-ventilation-m3-h" => (
            include_str!("../../🧬️schema/🧬️mutations/🪟️change-cellar-ventilation-m3-h/🧪️tests/🔬️t023/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪟️change-cellar-ventilation-m3-h/🧪️tests/🔬️t023/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪟️change-cellar-ventilation-m3-h/🧪️tests/🔬️t023/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🪟️change-cellar-ventilation-m3-h/🧪️tests/🔬️t023/🎯️outcome/🔣️.json"),
        ),
        "change-h-tr-wk" => (
            include_str!("../../🧬️schema/🧬️mutations/🧱️change-h-tr-wk/🧪️tests/🧱️improves-the-460fbf/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧱️change-h-tr-wk/🧪️tests/🧱️improves-the-460fbf/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧱️change-h-tr-wk/🧪️tests/🧱️improves-the-460fbf/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧱️change-h-tr-wk/🧪️tests/🧱️improves-the-460fbf/🎯️outcome/🔣️.json"),
        ),
        "change-h-ve-wk" => (
            include_str!("../../🧬️schema/🧬️mutations/🔃️change-h-ve-wk/🧪️tests/🔃️raises-the-0eb6b7/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔃️change-h-ve-wk/🧪️tests/🔃️raises-the-0eb6b7/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔃️change-h-ve-wk/🧪️tests/🔃️raises-the-0eb6b7/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🔃️change-h-ve-wk/🧪️tests/🔃️raises-the-0eb6b7/🎯️outcome/🔣️.json"),
        ),
        "change-theta-ec" => (
            include_str!("../../🧬️schema/🧬️mutations/🌤️change-theta-ec/🧪️tests/🌤️raises-the-35ebb1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌤️change-theta-ec/🧪️tests/🌤️raises-the-35ebb1/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌤️change-theta-ec/🧪️tests/🌤️raises-the-35ebb1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌤️change-theta-ec/🧪️tests/🌤️raises-the-35ebb1/🎯️outcome/🔣️.json"),
        ),
        "change-theta-set-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🎯️change-theta-set-c/🧪️tests/🎯️lowers-the-357920/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎯️change-theta-set-c/🧪️tests/🎯️lowers-the-357920/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎯️change-theta-set-c/🧪️tests/🎯️lowers-the-357920/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎯️change-theta-set-c/🧪️tests/🎯️lowers-the-357920/🎯️outcome/🔣️.json"),
        ),
        "change-cooling-delta-th" => (
            include_str!("../../🧬️schema/🧬️mutations/⌛️change-cooling-delta-th/🧪️tests/⌛️extends-the-601ca1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⌛️change-cooling-delta-th/🧪️tests/⌛️extends-the-601ca1/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⌛️change-cooling-delta-th/🧪️tests/⌛️extends-the-601ca1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⌛️change-cooling-delta-th/🧪️tests/⌛️extends-the-601ca1/🎯️outcome/🔣️.json"),
        ),
        "change-cooling-gains-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/📈️change-cooling-gains-kwh/🧪️tests/🔬️t018/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📈️change-cooling-gains-kwh/🧪️tests/🔬️t018/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📈️change-cooling-gains-kwh/🧪️tests/🔬️t018/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📈️change-cooling-gains-kwh/🧪️tests/🔬️t018/🎯️outcome/🔣️.json"),
        ),
        "change-cooling-utilization-factor" => (
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-cooling-utilization-factor/🧪️tests/🔬️t014/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-cooling-utilization-factor/🧪️tests/🔬️t014/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-cooling-utilization-factor/🧪️tests/🔬️t014/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🎚️change-cooling-utilization-factor/🧪️tests/🔬️t014/🎯️outcome/🔣️.json"),
        ),
        "change-cooling-reference-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/📊️change-cooling-reference-kwh/🧪️tests/📊️raises-the-0959e4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-cooling-reference-kwh/🧪️tests/📊️raises-the-0959e4/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-cooling-reference-kwh/🧪️tests/📊️raises-the-0959e4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📊️change-cooling-reference-kwh/🧪️tests/📊️raises-the-0959e4/🎯️outcome/🔣️.json"),
        ),
        "change-chiller-type" => (
            include_str!("../../🧬️schema/🧬️mutations/🥶️change-chiller-type/🧪️tests/🥶️switches-to-a-34114c/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥶️change-chiller-type/🧪️tests/🥶️switches-to-a-34114c/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥶️change-chiller-type/🧪️tests/🥶️switches-to-a-34114c/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🥶️change-chiller-type/🧪️tests/🥶️switches-to-a-34114c/🎯️outcome/🔣️.json"),
        ),
        "change-eer-actual" => (
            include_str!("../../🧬️schema/🧬️mutations/✅️change-eer-actual/🧪️tests/✅️raises-the-9ac5d6/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✅️change-eer-actual/🧪️tests/✅️raises-the-9ac5d6/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✅️change-eer-actual/🧪️tests/✅️raises-the-9ac5d6/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/✅️change-eer-actual/🧪️tests/✅️raises-the-9ac5d6/🎯️outcome/🔣️.json"),
        ),
        "change-qc-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/❄️change-qc-kwh/🧪️tests/❄️raises-the-annual-cd8096/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/❄️change-qc-kwh/🧪️tests/❄️raises-the-annual-cd8096/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/❄️change-qc-kwh/🧪️tests/❄️raises-the-annual-cd8096/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/❄️change-qc-kwh/🧪️tests/❄️raises-the-annual-cd8096/🎯️outcome/🔣️.json"),
        ),
        "change-generation-reference-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/⚡️change-generation-reference-kwh/🧪️tests/⚡️raises-the-27980b/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚡️change-generation-reference-kwh/🧪️tests/⚡️raises-the-27980b/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚡️change-generation-reference-kwh/🧪️tests/⚡️raises-the-27980b/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/⚡️change-generation-reference-kwh/🧪️tests/⚡️raises-the-27980b/🎯️outcome/🔣️.json"),
        ),
        "change-data-center-supply-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🖥️change-data-center-supply-c/🧪️tests/🖥️raises-the-data-54f207/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🖥️change-data-center-supply-c/🧪️tests/🖥️raises-the-data-54f207/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🖥️change-data-center-supply-c/🧪️tests/🖥️raises-the-data-54f207/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🖥️change-data-center-supply-c/🧪️tests/🖥️raises-the-data-54f207/🎯️outcome/🔣️.json"),
        ),
        "change-h-st-wk" => (
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-h-st-wk/🧪️tests/🧮️raises-the-aa7ee8/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-h-st-wk/🧪️tests/🧮️raises-the-aa7ee8/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-h-st-wk/🧪️tests/🧮️raises-the-aa7ee8/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧮️change-h-st-wk/🧪️tests/🧮️raises-the-aa7ee8/🎯️outcome/🔣️.json"),
        ),
        "change-theta-st-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🫙️change-theta-st-c/🧪️tests/🫙️lowers-the-f78ef4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫙️change-theta-st-c/🧪️tests/🫙️lowers-the-f78ef4/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫙️change-theta-st-c/🧪️tests/🫙️lowers-the-f78ef4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🫙️change-theta-st-c/🧪️tests/🫙️lowers-the-f78ef4/🎯️outcome/🔣️.json"),
        ),
        "change-theta-amb-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🌬️change-theta-amb-c/🧪️tests/🌬️lowers-the-e2ef9a/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌬️change-theta-amb-c/🧪️tests/🌬️lowers-the-e2ef9a/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌬️change-theta-amb-c/🧪️tests/🌬️lowers-the-e2ef9a/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🌬️change-theta-amb-c/🧪️tests/🌬️lowers-the-e2ef9a/🎯️outcome/🔣️.json"),
        ),
        "change-storage-th" => (
            include_str!("../../🧬️schema/🧬️mutations/🕒️change-storage-th/🧪️tests/🕒️shortens-the-eca611/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕒️change-storage-th/🧪️tests/🕒️shortens-the-eca611/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕒️change-storage-th/🧪️tests/🕒️shortens-the-eca611/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕒️change-storage-th/🧪️tests/🕒️shortens-the-eca611/🎯️outcome/🔣️.json"),
        ),
        "change-storage-allowance-kwh" => (
            include_str!("../../🧬️schema/🧬️mutations/📉️change-storage-allowance-kwh/🧪️tests/🔬️t019/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-storage-allowance-kwh/🧪️tests/🔬️t019/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-storage-allowance-kwh/🧪️tests/🔬️t019/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/📉️change-storage-allowance-kwh/🧪️tests/🔬️t019/🎯️outcome/🔣️.json"),
        ),
        "change-dhw-delivery-c" => (
            include_str!("../../🧬️schema/🧬️mutations/🚿️change-dhw-delivery-c/🧪️tests/🚿️raises-the-dhw-57354d/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚿️change-dhw-delivery-c/🧪️tests/🚿️raises-the-dhw-57354d/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚿️change-dhw-delivery-c/🧪️tests/🚿️raises-the-dhw-57354d/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🚿️change-dhw-delivery-c/🧪️tests/🚿️raises-the-dhw-57354d/🎯️outcome/🔣️.json"),
        ),
        "change-duct-class" => (
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-duct-class/🧪️tests/🛡️upgrades-the-2bc881/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-duct-class/🧪️tests/🛡️upgrades-the-2bc881/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-duct-class/🧪️tests/🛡️upgrades-the-2bc881/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🛡️change-duct-class/🧪️tests/🛡️upgrades-the-2bc881/🎯️outcome/🔣️.json"),
        ),
        "change-duct-test-pressure-pa" => (
            include_str!("../../🧬️schema/🧬️mutations/🧭️change-duct-test-pressure-pa/🧪️tests/🧭️raises-the-duct-999865/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧭️change-duct-test-pressure-pa/🧪️tests/🧭️raises-the-duct-999865/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧭️change-duct-test-pressure-pa/🧪️tests/🧭️raises-the-duct-999865/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🧭️change-duct-test-pressure-pa/🧪️tests/🧭️raises-the-duct-999865/🎯️outcome/🔣️.json"),
        ),
        "change-duct-leakage-m3-sm2" => (
            include_str!("../../🧬️schema/🧬️mutations/🕳️change-duct-leakage-m3-sm2/🧪️tests/🔬️t021/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕳️change-duct-leakage-m3-sm2/🧪️tests/🔬️t021/🦠️mutation/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕳️change-duct-leakage-m3-sm2/🧪️tests/🔬️t021/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧬️schema/🧬️mutations/🕳️change-duct-leakage-m3-sm2/🧪️tests/🔬️t021/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-din16798-1: no committed fixture is registered for kind {other:?}"),
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
    use crate::standards::v1::subsets::any::schema::mutations::{apply_din16798_mutation, decode_din16798_mutation_json, inverse_din16798_mutation, Din16798Mutation};
    use crate::standards::v1::subsets::any::schema::snapshot::{decode_din16798_dsl, decode_din16798_pack, decode_din16798_snapshot_json, encode_din16798_dsl, encode_din16798_pack, encode_din16798_snapshot_json, Din16798Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<Din16798Snapshot, String> {
        decode_din16798_snapshot_json(text).map_err(|error| format!("mutate-din16798-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<Din16798Mutation, String> {
        decode_din16798_mutation_json(text).map_err(|error| format!("mutate-din16798-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &Din16798Snapshot) -> Result<Json, String> {
        parse_json(&encode_din16798_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &Din16798Snapshot, expected: &Din16798Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_din16798_snapshot_json(got), encode_din16798_snapshot_json(expected))
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
            let applied = apply_din16798_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_din16798_snapshot_json(&snapshot))),
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
            let mut current = match apply_din16798_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_din16798_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_din16798_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed DIN EN 16798-1 artifact is not UTF-8: {error}"))?;
        let parsed = decode_din16798_dsl(&text)?;
        let reprinted = encode_din16798_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_din16798_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_din16798_pack(&encode_din16798_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_din16798_snapshot_json(&encode_din16798_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
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
