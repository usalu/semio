//! 🦀️ DIN 4108 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14 (the no-oracle conversion). The recorded
//! no-oracle decision `din4108-1-mutation-semantics` is gone from
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`, because a reference now
//! exists to compare against: `s.norm.din4108` is a
//! semio-native artifact with no third-party reader or writer, so its reference is a second
//! IMPLEMENTATION: the independent Python `🐍️component.py` beside this file, registered as the
//! oracle `din4108-1-python-independent`. This adapter is the SUBJECT half only — it drives this
//! repository's own `apply_din4108_mutation` over the full 22-kind `Din4108Mutation`
//! vocabulary.
//!
//! `Din4108Snapshot` is seventeen document-root scalars — assembly category, climate zone,
//! airtightness (n50 and class), thermal-bridge sum, indoor relative humidity and design
//! temperature, catalogue and material ids, solar absorptance, design irradiance, interior and
//! exterior vapour-diffusion mu values, envelope area, the Beiblatt-2 conformity flag,
//! application type and declared application class — plus `layers`, an id-less ORDERED
//! construction build-up. That gives seventeen `change-<field>` kinds and five collection
//! kinds: `insert-layer` and `remove-layer` address by index (inserted = final-state index,
//! removed = base-state index), `reorder-layers` moves one layer inside the build-up, and
//! `change-layer-thickness` / `change-layer-lambda` edit one field of one layer by base-state
//! index.
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
//! (`decode_din4108_snapshot_json`/`encode_din4108_snapshot_json`,
//! `decode_din4108_dsl`/`encode_din4108_dsl`, `decode_din4108_pack`/`encode_din4108_pack` in
//! `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`;
//! `decode_din4108_mutation_json`, `apply_din4108_mutation`, `inverse_din4108_mutation` in
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
/// 🏷️ Mirrors `Din4108Mutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "change-category",
    "change-climate",
    "change-airtightness-n50",
    "change-psi-times-l-sum",
    "change-rh-int",
    "change-catalog-id",
    "change-material-id",
    "change-airtightness-class",
    "change-t-int-c",
    "change-solar-absorptance",
    "change-irradiance-wm2",
    "change-moisture-mu-exterior",
    "change-moisture-mu-interior",
    "change-envelope-area-m2",
    "change-bb2-details-conform",
    "change-application-type",
    "change-declared-application-class",
    "insert-layer",
    "remove-layer",
    "reorder-layers",
    "change-layer-thickness",
    "change-layer-lambda",
];

/// 🗣️ The real committed DIN 4108 document, read where the domain already keeps it.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — the same committed bytes the independent Python oracle reads through
/// the `asset://` URIs the feature declares, so the two sides can never be comparing different inputs.
/// One `include_str!` per committed file; the subject role decodes all four.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "change-category" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🧪️tests/retypes-the-assembly-as-office/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🧪️tests/retypes-the-assembly-as-office/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🧪️tests/retypes-the-assembly-as-office/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪜change-category/🧪️tests/retypes-the-assembly-as-office/🎯️outcome/🔣️.json"),
        ),
        "change-climate" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🧪️tests/moves-the-building-to-climate-zone-4/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🧪️tests/moves-the-building-to-climate-zone-4/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🧪️tests/moves-the-building-to-climate-zone-4/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️change-climate/🧪️tests/moves-the-building-to-climate-zone-4/🎯️outcome/🔣️.json"),
        ),
        "change-airtightness-n50" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🧪️tests/tightens-n50-to-1-point-5-per-hour/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🧪️tests/tightens-n50-to-1-point-5-per-hour/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🧪️tests/tightens-n50-to-1-point-5-per-hour/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧰change-airtightness-n50/🧪️tests/tightens-n50-to-1-point-5-per-hour/🎯️outcome/🔣️.json"),
        ),
        "change-psi-times-l-sum" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🧪️tests/raises-the-thermal-bridge-sum-to-0-point-05/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🧪️tests/raises-the-thermal-bridge-sum-to-0-point-05/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🧪️tests/raises-the-thermal-bridge-sum-to-0-point-05/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧯change-psi-times-l-sum/🧪️tests/raises-the-thermal-bridge-sum-to-0-point-05/🎯️outcome/🔣️.json"),
        ),
        "change-rh-int" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🧪️tests/raises-indoor-relative-humidity-to-0-point-65/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🧪️tests/raises-indoor-relative-humidity-to-0-point-65/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🧪️tests/raises-indoor-relative-humidity-to-0-point-65/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣change-rh-int/🧪️tests/raises-indoor-relative-humidity-to-0-point-65/🎯️outcome/🔣️.json"),
        ),
        "change-catalog-id" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🧪️tests/repoints-the-catalogue-entry-to-aw-07/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🧪️tests/repoints-the-catalogue-entry-to-aw-07/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🧪️tests/repoints-the-catalogue-entry-to-aw-07/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵change-catalog-id/🧪️tests/repoints-the-catalogue-entry-to-aw-07/🎯️outcome/🔣️.json"),
        ),
        "change-material-id" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🧪️tests/swaps-the-insulation-material-to-eps/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🧪️tests/swaps-the-insulation-material-to-eps/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🧪️tests/swaps-the-insulation-material-to-eps/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪥change-material-id/🧪️tests/swaps-the-insulation-material-to-eps/🎯️outcome/🔣️.json"),
        ),
        "change-airtightness-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🧪️tests/upgrades-the-airtightness-class-to-class1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🧪️tests/upgrades-the-airtightness-class-to-class1/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🧪️tests/upgrades-the-airtightness-class-to-class1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪚change-airtightness-class/🧪️tests/upgrades-the-airtightness-class-to-class1/🎯️outcome/🔣️.json"),
        ),
        "change-t-int-c" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🧪️tests/raises-the-indoor-design-temperature-to-22-point-5-c/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🧪️tests/raises-the-indoor-design-temperature-to-22-point-5-c/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🧪️tests/raises-the-indoor-design-temperature-to-22-point-5-c/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚨change-t-int-c/🧪️tests/raises-the-indoor-design-temperature-to-22-point-5-c/🎯️outcome/🔣️.json"),
        ),
        "change-solar-absorptance" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🧪️tests/lightens-the-facade-to-absorptance-0-point-25/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🧪️tests/lightens-the-facade-to-absorptance-0-point-25/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🧪️tests/lightens-the-facade-to-absorptance-0-point-25/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-solar-absorptance/🧪️tests/lightens-the-facade-to-absorptance-0-point-25/🎯️outcome/🔣️.json"),
        ),
        "change-irradiance-wm2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🧪️tests/raises-design-irradiance-to-750-w-per-m2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🧪️tests/raises-design-irradiance-to-750-w-per-m2/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🧪️tests/raises-design-irradiance-to-750-w-per-m2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲change-irradiance-wm2/🧪️tests/raises-design-irradiance-to-750-w-per-m2/🎯️outcome/🔣️.json"),
        ),
        "change-moisture-mu-exterior" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🧪️tests/raises-the-exterior-mu-value-to-20/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🧪️tests/raises-the-exterior-mu-value-to-20/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🧪️tests/raises-the-exterior-mu-value-to-20/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪛change-moisture-mu-exterior/🧪️tests/raises-the-exterior-mu-value-to-20/🎯️outcome/🔣️.json"),
        ),
        "change-moisture-mu-interior" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🧪️tests/raises-the-interior-mu-value-to-2-point-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🧪️tests/raises-the-interior-mu-value-to-2-point-5/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🧪️tests/raises-the-interior-mu-value-to-2-point-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪝change-moisture-mu-interior/🧪️tests/raises-the-interior-mu-value-to-2-point-5/🎯️outcome/🔣️.json"),
        ),
        "change-envelope-area-m2" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🧪️tests/grows-the-envelope-to-150-m2/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🧪️tests/grows-the-envelope-to-150-m2/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🧪️tests/grows-the-envelope-to-150-m2/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-envelope-area-m2/🧪️tests/grows-the-envelope-to-150-m2/🎯️outcome/🔣️.json"),
        ),
        "change-bb2-details-conform" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🧪️tests/declares-the-beiblatt-2-details-non-conforming/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🧪️tests/declares-the-beiblatt-2-details-non-conforming/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🧪️tests/declares-the-beiblatt-2-details-non-conforming/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀change-bb2-details-conform/🧪️tests/declares-the-beiblatt-2-details-non-conforming/🎯️outcome/🔣️.json"),
        ),
        "change-application-type" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🧪️tests/reclassifies-the-application-type-as-wab/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🧪️tests/reclassifies-the-application-type-as-wab/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🧪️tests/reclassifies-the-application-type-as-wab/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪤change-application-type/🧪️tests/reclassifies-the-application-type-as-wab/🎯️outcome/🔣️.json"),
        ),
        "change-declared-application-class" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🧪️tests/declares-application-class-kh/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🧪️tests/declares-application-class-kh/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🧪️tests/declares-application-class-kh/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧶change-declared-application-class/🧪️tests/declares-application-class-kh/🎯️outcome/🔣️.json"),
        ),
        "insert-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🧪️tests/inserts-an-interior-plaster-layer-at-index-1/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🧪️tests/inserts-an-interior-plaster-layer-at-index-1/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🧪️tests/inserts-an-interior-plaster-layer-at-index-1/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢insert-layer/🧪️tests/inserts-an-interior-plaster-layer-at-index-1/🎯️outcome/🔣️.json"),
        ),
        "remove-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🧪️tests/removes-the-load-bearing-masonry-layer/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🧪️tests/removes-the-load-bearing-masonry-layer/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🧪️tests/removes-the-load-bearing-masonry-layer/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️remove-layer/🧪️tests/removes-the-load-bearing-masonry-layer/🎯️outcome/🔣️.json"),
        ),
        "reorder-layers" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🧪️tests/moves-the-insulation-in-front-of-the-masonry/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🧪️tests/moves-the-insulation-in-front-of-the-masonry/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🧪️tests/moves-the-insulation-in-front-of-the-masonry/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷reorder-layers/🧪️tests/moves-the-insulation-in-front-of-the-masonry/🎯️outcome/🔣️.json"),
        ),
        "change-layer-thickness" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🧪️tests/thickens-the-insulation-layer-to-0-point-2-m/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🧪️tests/thickens-the-insulation-layer-to-0-point-2-m/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🧪️tests/thickens-the-insulation-layer-to-0-point-2-m/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪡change-layer-thickness/🧪️tests/thickens-the-insulation-layer-to-0-point-2-m/🎯️outcome/🔣️.json"),
        ),
        "change-layer-lambda" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🧪️tests/degrades-the-masonry-lambda-to-0-point-5/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🧪️tests/degrades-the-masonry-lambda-to-0-point-5/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🧪️tests/degrades-the-masonry-lambda-to-0-point-5/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪒change-layer-lambda/🧪️tests/degrades-the-masonry-lambda-to-0-point-5/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-din4108-1: no committed fixture is registered for kind {other:?}"),
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
    use semio_s_plugin_norm::artifacts::din4108::standards::v1::subsets::any::schema::mutations::{apply_din4108_mutation, decode_din4108_mutation_json, inverse_din4108_mutation, Din4108Mutation};
    use semio_s_plugin_norm::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::{decode_din4108_dsl, decode_din4108_pack, decode_din4108_snapshot_json, encode_din4108_dsl, encode_din4108_pack, encode_din4108_snapshot_json, Din4108Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<Din4108Snapshot, String> {
        decode_din4108_snapshot_json(text).map_err(|error| format!("mutate-din4108-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<Din4108Mutation, String> {
        decode_din4108_mutation_json(text).map_err(|error| format!("mutate-din4108-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &Din4108Snapshot) -> Result<Json, String> {
        parse_json(&encode_din4108_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &Din4108Snapshot, expected: &Din4108Snapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_din4108_snapshot_json(got), encode_din4108_snapshot_json(expected))
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
            let applied = apply_din4108_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {}", encode_din4108_snapshot_json(&snapshot))),
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
            let mut current = match apply_din4108_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_din4108_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_din4108_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed DIN 4108 artifact is not UTF-8: {error}"))?;
        let parsed = decode_din4108_dsl(&text)?;
        let reprinted = encode_din4108_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_din4108_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_din4108_pack(&encode_din4108_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_din4108_snapshot_json(&encode_din4108_snapshot_json(&parsed))?;
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
