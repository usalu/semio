# Private Fixture Reader Module Ownership

Root's exact registered source census refused the finite Home transient and OS UI preferences data readers because the adapters participate through included source contexts. The data declarations will be physically mounted once as private file modules beside the original adapters, with their original function/macro/vector lists and all actual fixture inputs retained. This is a source ownership correction; no macro spelling exemption, runtime compatibility facade, foreign data copy, expected-output reset or filter change is introduced.

The actual existing language-neutral vectors and original native adapter laws are the semantic authority. New module mounts/imports and relocated include paths are the only permitted source deltas. Native runtime and fresh source census admission remain pending. Full original adapter bytes are captured before source edits:

## ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs

SHA256: c13db47b7fd2aaa9be5961c47624a3d4b5ed3537aea19586bfe0ec58ce701748

```rust
//! 🫧️ `s.space.home` ✏️editor/🫧️transient state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `s-home-1-any-editor-transient-state-lane-semantics`: the runner dispatches no oracle role, so
//! every law is asserted inside the subject handler through `semio_repo_test_host::law::vector` over the report
//! of this crate's production bridge `home_transient_mutation_report_json`. The oracle handler answers with the committed
//! after- and before-snapshots read literally, so the reference side exists the moment a second producer does. The one verb
//! is non-invertible (a derived projection page is never undone): its inverse law is asserted on the vectors that fold
//! nothing, where no inverse step may exist and the projection must stay the committed before-snapshot.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_repo_test_host::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ One committed vector of `apply-directory-page`, read literally from `✏️editor/🫧️transient/🧫️fixtures`.
fn vector(id: &str) -> Result<Vector, String> {
    macro_rules! committed {
        ($name:literal, $observable:expr) => {
            Vector {
                before: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/⬅️before/🔣️.json")),
                mutation: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🦠️mutation/🔣️.json")),
                after: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/➡️after/🔣️.json")),
                diff: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🔺️diff/🔣️.json")),
                outcome: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🎯️outcome/🔣️.json")),
                observable: $observable,
            }
        };
    }
    Ok(match id {
        "apply-directory-page-applied" => committed!("✅️apply", true),
        "apply-directory-page-no-op" => committed!("🟰️apply", false),
        "apply-directory-page-rejected" => committed!("🚫️apply", false),
        other => return Err(format!("no committed vector for {other:?}")),
    })
}
//#endregion 🔖️Vectors

//#region 🔖️Oracle
fn literal(text: &str) -> Result<Outcome, String> {
    Ok(Outcome::with_raw(text.as_bytes().to_vec(), parse_json(text)?))
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.after)
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.before)
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::*;
    use semio_s_artifact_space_home::editor::home::transient::mutations::home_transient_mutation_report_json;
    use semio_repo_test_host::law::vector;

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let id = ctx.row()?;
        let committed = vector(id)?;
        let report = home_transient_mutation_report_json(committed.before, committed.mutation, committed.after)?;
        let applied = vector::mutate(id, &report, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let id = ctx.row()?;
        let committed = vector(id)?;
        let report = home_transient_mutation_report_json(committed.before, committed.mutation, committed.after)?;
        let restored = vector::inverse(id, &report)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration

```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🦀️.rs

SHA256: a0ab82f2a321fafa8b3100d044671e8bfc23f2ee3bed557c7a37413a004ae1eb

```rust
//! 🎨️ UI-preferences exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-ui-preferences-mutation-semantics` (`../../../../../🎚️config/🔮️oracles/🔣️.json`):
//! `os.config.ui-preferences` is this operating system's own preference record with no third-party
//! implementation, so `oracle` here reads the committed, independently handcrafted per-kind
//! specification vectors (`../../../../../🎚️config/🧬️schema/🧬️mutations/<slug>/🧫️fixtures/<vector>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_ui_preferences_config_mutation_reporting` over the full nine-kind
//! `UiPreferencesConfigMutation` vocabulary, one `sets-*` (applied) and one `keeps-*` (no-op) vector
//! per kind.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Vectors
/// 🏷️ Every committed vector's scenario id, in the catalog's own order — duplicated, not imported,
/// because the oracle-only build must not link the subject crate. The contract's mutation-coverage
/// gate keeps this list honest against the catalog. `sets-*` vectors are applied, `keeps-*` no-ops.
const VECTORS: &[&str] = &[
    "sets-appearance",
    "keeps-appearance",
    "sets-layout",
    "keeps-layout",
    "sets-driver",
    "keeps-driver",
    "sets-custom-driver",
    "keeps-custom-driver",
    "sets-locale",
    "keeps-locale",
    "sets-terminology",
    "keeps-terminology",
    "sets-theme",
    "keeps-theme",
    "sets-custom-theme",
    "keeps-custom-theme",
    "sets-keybinding",
    "keeps-keybinding",
];
//#endregion 🔖️Vectors

//#region 🔖️Fixtures
/// 🧫️ Embeds one vector's `(before, mutation, after, outcome)` quintet text, read literally.
macro_rules! vector {
    ($leaf:literal, $vector:literal) => {
        (
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/⬅️before/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🦠️mutation/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/➡️after/🔣️.json")),
            include_str!(concat!("../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🎯️outcome/🔣️.json")),
        )
    };
}

/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one scenario —
/// this IS the independently handcrafted vector the no-oracle decision rests on, never recomputed.
fn fixture_text(scenario: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match scenario {
        "sets-appearance" => vector!("🌗️set-appearance", "✏️sets-appearance"),
        "keeps-appearance" => vector!("🌗️set-appearance", "🟰️keeps-appearance"),
        "sets-layout" => vector!("📐️set-layout", "✏️sets-layout"),
        "keeps-layout" => vector!("📐️set-layout", "🟰️keeps-layout"),
        "sets-driver" => vector!("🕹️set-driver", "✏️sets-driver"),
        "keeps-driver" => vector!("🕹️set-driver", "🟰️keeps-driver"),
        "sets-custom-driver" => vector!("🚗️set-custom-driver", "✏️sets-custom-driver"),
        "keeps-custom-driver" => vector!("🚗️set-custom-driver", "🟰️keeps-custom-driver"),
        "sets-locale" => vector!("🗣️set-locale", "✏️sets-locale"),
        "keeps-locale" => vector!("🗣️set-locale", "🟰️keeps-locale"),
        "sets-terminology" => vector!("📖️set-terminology", "✏️sets-terminology"),
        "keeps-terminology" => vector!("📖️set-terminology", "🟰️keeps-terminology"),
        "sets-theme" => vector!("🖼️set-theme", "✏️sets-theme"),
        "keeps-theme" => vector!("🖼️set-theme", "🟰️keeps-theme"),
        "sets-custom-theme" => vector!("🎨️set-custom-theme", "✏️sets-custom-theme"),
        "keeps-custom-theme" => vector!("🎨️set-custom-theme", "🟰️keeps-custom-theme"),
        "sets-keybinding" => vector!("⌨️set-keybinding-override", "✏️sets-keybinding"),
        "keeps-keybinding" => vector!("⌨️set-keybinding-override", "🟰️keeps-keybinding"),
        other => panic!("mutate-os-config-ui-preferences: no specification vector registered for scenario {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER record, read literally.
fn mutate_oracle_for(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(scenario);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE record — undoing a kind must return to
/// exactly where the specification vector started.
fn inverse_oracle_for(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(scenario);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔁️ The identity carrier's reference answer: the committed fully populated record itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("keeps-keybinding");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::UiPreferencesConfigMutation;
    use semio_framework_plugin_host::opening_config::{apply_ui_preferences_config_mutation_reporting, decode_ui_preferences_config_mutation_json, decode_ui_preferences_json, encode_ui_preferences_json, inverse_ui_preferences_config_mutation_steps, UiLocale, UiPreferences};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn record_of(text: &str, label: &str, scenario: &str) -> Result<UiPreferences, String> {
        decode_ui_preferences_json(text).map_err(|error| format!("mutate-os-config-ui-preferences: the committed {label}-record for {scenario:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, scenario: &str) -> Result<UiPreferencesConfigMutation, String> {
        decode_ui_preferences_config_mutation_json(text).map_err(|error| format!("mutate-os-config-ui-preferences: the committed mutation payload for {scenario:?} must decode: {error}"))
    }

    fn projection(record: &UiPreferences) -> Result<Json, String> {
        parse_json(&encode_ui_preferences_json(record))
    }

    /// 🔤️ The projection with every object's members in key order, so member comparisons state the
    /// record's content and never the encoder's map iteration order.
    fn ordered(value: &Json) -> Json {
        match value {
            Json::Object(entries) => {
                let mut sorted: Vec<(String, Json)> = entries.iter().map(|(key, member)| (key.clone(), ordered(member))).collect();
                sorted.sort_by(|left, right| left.0.cmp(&right.0));
                Json::Object(sorted)
            }
            Json::Array(items) => Json::Array(items.iter().map(ordered).collect()),
            other => other.clone(),
        }
    }

    fn disagreement(what: &str, got: &UiPreferences, expected: &UiPreferences) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_ui_preferences_json(got), encode_ui_preferences_json(expected))
    }

    /// 🔎️ The camel-case member names whose projected values differ between two records, read off
    /// the TYPED values' canonical projections — the observable a `field` claim is checked against.
    fn moved_fields(before: &UiPreferences, after: &UiPreferences) -> Result<Vec<String>, String> {
        let (before, after) = (ordered(&projection(before)?), ordered(&projection(after)?));
        let members = ["appearance", "layout", "driverId", "customDrivers", "locale", "terminology", "themeId", "customThemes", "keybindingOverrides"];
        Ok(members.iter().filter(|member| before.get(member) != after.get(member)).map(|member| member.to_string()).collect())
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law: an `applied` vector moves exactly its declared preference and a
    /// `no-op` vector moves nothing. Comparing the whole record against a fixture written from the
    /// same bug would pass an implementation that returned the fixture; naming the member would not.
    fn field_claim_holds(scenario: &str, field: &str, status: &str, base: &UiPreferences, after: &UiPreferences) -> Result<(), String> {
        let moved = moved_fields(base, after)?;
        let expected: Vec<String> = if status == "applied" { vec![field.to_string()] } else { Vec::new() };
        if moved != expected {
            return Err(format!("mutate-{scenario}: the feature declares the {status} vector moves {expected:?}, but the implementation moved {moved:?}"));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: an `applied` vector raises nothing and a `no-op` vector raises
    /// exactly one `mutation.no-op` Warning — the committed outcome's own status and code.
    fn outcome_matches(scenario: &str, declared_status: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        let status = declared.str("status");
        if status != declared_status {
            return Err(format!("mutate-{scenario}: the feature declares a {declared_status:?} vector, but the committed outcome declares {status:?}"));
        }
        let expected: Vec<(String, String)> = if status == "no-op" { vec![("mutation.no-op".to_string(), "Warning".to_string())] } else { Vec::new() };
        if raised != expected.as_slice() {
            return Err(format!("mutate-{scenario}: the committed outcome declares {status:?} with {expected:?}, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the vector's mutation to the committed before-record and asserts, in role, that the
    /// result IS the committed after-record, that exactly the declared preference moved, and that the
    /// reported diagnostics are the committed ones.
    pub fn mutate(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            if row.str("vector") != scenario {
                return Err(format!("mutate-{scenario}: the feature row names vector {:?}", row.str("vector")));
            }
            let (before, mutation, after, outcome) = super::fixture_text(scenario);
            let base = record_of(before, "before", scenario)?;
            let expected = record_of(after, "after", scenario)?;
            let mutation = mutation_of(mutation, scenario)?;
            let mut current = base.clone();
            let raised = apply_ui_preferences_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{scenario}: the applied record does not match the committed after-record"), &current, &expected));
            }
            outcome_matches(scenario, &row.str("status"), &parse_json(outcome)?, &raised)?;
            field_claim_holds(scenario, &row.str("field"), &row.str("status"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-record exactly. Every kind reads its undo off BASE, so a keyed
    /// upsert into an empty map undoes to a removal rather than to the value it just wrote.
    pub fn inverse(scenario: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, _after, _outcome) = super::fixture_text(scenario);
            let base = record_of(before, "before", scenario)?;
            let mutation = mutation_of(mutation, scenario)?;
            let mut current = base.clone();
            let raised = apply_ui_preferences_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{scenario}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{scenario}: the forward mutation left the record untouched, so restoring it proves nothing"));
            }
            for step in inverse_ui_preferences_config_mutation_steps(&mutation, &base) {
                let undone = apply_ui_preferences_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{scenario}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {scenario:?} and then its own inverse did not restore the original"), &current, &base));
            }
            if !moved_fields(&base, &current)?.is_empty() {
                return Err(format!("inverse-{scenario}: {} does not hold its original value again", row.str("field")));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The identity law for a record whose only carrier is its own JSON projection. `os.config`
    /// has no `.dsl.semio` or `.pack.semio` form, so the honest statement is a decode/re-encode that
    /// must reproduce the committed projection exactly. The decode is proven real by reading the
    /// locale, the custom driver's scale and the keybinding back off the TYPED value.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("keeps-keybinding");
        let record = record_of(before, "before", "keeps-keybinding")?;
        let scale = record.custom_drivers.get("studio").and_then(|driver| driver.config.get("scale")).and_then(|scale| scale.as_f64());
        if record.locale != Some(UiLocale::De) || scale != Some(1.25) || record.keybinding_overrides.get("edit.undo").map(String::as_str) != Some("Meta+Z") {
            return Err(format!("identity-round-trip: the committed record holds locale de, driver scale 1.25 and Meta+Z for edit.undo, but the decoded value holds {}", encode_ui_preferences_json(&record)));
        }
        let reencoded = encode_ui_preferences_json(&record);
        let reparsed = record_of(&reencoded, "re-encoded", "keeps-keybinding")?;
        if reparsed != record {
            return Err(disagreement("identity-round-trip: decoding the re-encoded record did not reproduce the typed value", &reparsed, &record));
        }
        let projection = parse_json(&reencoded)?;
        if ordered(&projection) != ordered(&parse_json(before)?) {
            return Err(format!("identity-round-trip: the re-encoded projection differs from the committed record\n     got: {reencoded}\nexpected: {before}"));
        }
        Ok(Outcome::with_raw(reencoded.into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loops mirror the feature's `Examples` tables exactly: every vector is applied, and
/// every applied (`sets-*`) vector is also undone.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for scenario in VECTORS {
        built = built.oracle(&format!("mutate-{scenario}"), mutate_oracle_for(scenario));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{scenario}"), subject::mutate(scenario));
        }
        if scenario.starts_with("sets-") {
            built = built.oracle(&format!("inverse-{scenario}"), inverse_oracle_for(scenario));
            #[cfg(feature = "sut")]
            {
                built = built.subject(&format!("inverse-{scenario}"), subject::inverse(scenario));
            }
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration

```

## Exact Declared Source Moves

Both new private file modules reproduce all original function/macro/vector bytes after only the declared pub(super) visibility and one additional parent component in each include path. Reversing the adapter private mount/import replacements reproduces every original adapter byte. All native assertions, scenario lists, fixture contents, adapter declarations and original target features remain unchanged.

```json
[
  {
    "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🦀️.rs",
    "sha256": "e41fbbf1ff4235186666adf8c29847bade136ad77773126e26a9eebb7ee957ce"
  },
  {
    "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs",
    "sha256": "3e287d3b2aaa8007f44db65170431e444a4228a03f66b3dae2c36599e5199748"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🦀️.rs",
    "sha256": "10595aec51593f7f2eecf777ae56e40e182792bdddd4d5bb28dbee74faa18b91"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures/🦀️.rs",
    "sha256": "dbfe0215fa33b81087b74ad4b80276a3a43fdb954145757a5e7b084255d9c629"
  }
]
```

Native input/refusal/deletion receipts and actual generated host participation remain pending.

## Actual Local Scope And Reference Check

The initial ad hoc collector inspected a nonexistent target property; its null result was not an actual missing-input finding. This corrected check uses the canonical rustSourceTargets resolver and physical lstat for every referenced ancestor and file.

```json
[
  {
    "helper": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs",
    "sha256": "3e287d3b2aaa8007f44db65170431e444a4228a03f66b3dae2c36599e5199748",
    "scope": {
      "state": "resolved",
      "scopes": [
        {
          "kind": "root",
          "modulePath": [],
          "bodyStartOffset": 0,
          "bodyEndOffset": 1444
        }
      ]
    },
    "references": [
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before/🔣️.json",
          "line": 8,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 18,
            "definitionOffset": 224,
            "templateOffset": 341,
            "invocationOffset": 1186,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before/🔣️.json",
          "line": 8,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 19,
            "definitionOffset": 224,
            "templateOffset": 341,
            "invocationOffset": 1255,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before/🔣️.json",
          "line": 8,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 20,
            "definitionOffset": 224,
            "templateOffset": 341,
            "invocationOffset": 1329,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation/🔣️.json",
          "line": 9,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 18,
            "definitionOffset": 224,
            "templateOffset": 497,
            "invocationOffset": 1186,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation/🔣️.json",
          "line": 9,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 19,
            "definitionOffset": 224,
            "templateOffset": 497,
            "invocationOffset": 1255,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation/🔣️.json",
          "line": 9,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 20,
            "definitionOffset": 224,
            "templateOffset": 497,
            "invocationOffset": 1329,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after/🔣️.json",
          "line": 10,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 18,
            "definitionOffset": 224,
            "templateOffset": 641,
            "invocationOffset": 1186,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after/🔣️.json",
          "line": 10,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 19,
            "definitionOffset": 224,
            "templateOffset": 641,
            "invocationOffset": 1255,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after/🔣️.json",
          "line": 10,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 20,
            "definitionOffset": 224,
            "templateOffset": 641,
            "invocationOffset": 1329,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff/🔣️.json",
          "line": 11,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 18,
            "definitionOffset": 224,
            "templateOffset": 792,
            "invocationOffset": 1186,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff/🔣️.json",
          "line": 11,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 19,
            "definitionOffset": 224,
            "templateOffset": 792,
            "invocationOffset": 1255,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff/🔣️.json",
          "line": 11,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 20,
            "definitionOffset": 224,
            "templateOffset": 792,
            "invocationOffset": 1329,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome/🔣️.json",
          "line": 12,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 18,
            "definitionOffset": 224,
            "templateOffset": 934,
            "invocationOffset": 1186,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome/🔣️.json",
          "line": 12,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 19,
            "definitionOffset": 224,
            "templateOffset": 934,
            "invocationOffset": 1255,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome/🔣️.json",
          "line": 12,
          "expansion": {
            "kind": "local-macro",
            "macro": "committed",
            "definitionLine": 5,
            "invocationLine": 20,
            "definitionOffset": 224,
            "templateOffset": 934,
            "invocationOffset": 1329,
            "scope": {
              "kind": "local-block",
              "startOffset": 218,
              "endOffset": 1443
            }
          }
        },
        "directories": [
          "✏️s",
          "✏️s/🔌️plugins",
          "✏️s/🔌️plugins/🪐️space",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
          "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome"
        ],
        "physical": [
          {
            "path": "✏️s",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      }
    ]
  },
  {
    "helper": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures/🦀️.rs",
    "sha256": "dbfe0215fa33b81087b74ad4b80276a3a43fdb954145757a5e7b084255d9c629",
    "scope": {
      "state": "resolved",
      "scopes": [
        {
          "kind": "root",
          "modulePath": [],
          "bodyStartOffset": 0,
          "bodyEndOffset": 3417
        }
      ]
    },
    "references": [
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 41,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 1902,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 42,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 1983,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 43,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2061,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 44,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2130,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 45,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2200,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 46,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2269,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 47,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2346,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 48,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2436,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 49,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2520,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 50,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2589,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 51,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2664,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 52,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2748,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 53,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2827,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 54,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2893,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 55,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 2967,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 56,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 3054,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 57,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 3140,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before/🔣️.json",
          "line": 29,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 58,
            "definitionOffset": 841,
            "templateOffset": 927,
            "invocationOffset": 3229,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 41,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 1902,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 42,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 1983,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 43,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2061,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 44,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2130,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 45,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2200,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 46,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2269,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 47,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2346,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 48,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2436,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 49,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2520,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 50,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2589,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 51,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2664,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 52,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2748,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 53,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2827,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 54,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2893,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 55,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 2967,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 56,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 3054,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 57,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 3140,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation/🔣️.json",
          "line": 30,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 58,
            "definitionOffset": 841,
            "templateOffset": 1084,
            "invocationOffset": 3229,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 41,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 1902,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 42,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 1983,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 43,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2061,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 44,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2130,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 45,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2200,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 46,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2269,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 47,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2346,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 48,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2436,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 49,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2520,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 50,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2589,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 51,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2664,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 52,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2748,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 53,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2827,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 54,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2893,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 55,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 2967,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 56,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 3054,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 57,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 3140,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after/🔣️.json",
          "line": 31,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 58,
            "definitionOffset": 841,
            "templateOffset": 1232,
            "invocationOffset": 3229,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 41,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 1902,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 42,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 1983,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 43,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2061,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 44,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2130,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 45,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2200,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 46,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2269,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 47,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2346,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 48,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2436,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 49,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2520,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 50,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2589,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 51,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2664,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 52,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2748,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 53,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2827,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 54,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2893,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 55,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 2967,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 56,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 3054,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 57,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 3140,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      },
      {
        "to": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome/🔣️.json",
        "reference": {
          "kind": "include_str",
          "path": "../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome/🔣️.json",
          "line": 32,
          "expansion": {
            "kind": "local-macro",
            "macro": "vector",
            "definitionLine": 26,
            "invocationLine": 58,
            "definitionOffset": 841,
            "templateOffset": 1388,
            "invocationOffset": 3229,
            "scope": {
              "kind": "module",
              "modulePath": []
            }
          }
        },
        "directories": [
          "🧰️framework",
          "🧰️framework/🛍️products",
          "🧰️framework/🛍️products/💻️os",
          "🧰️framework/🛍️products/💻️os/🔨️modules",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
          "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
          "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome"
        ],
        "physical": [
          {
            "path": "🧰️framework",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome",
            "kind": "directory"
          },
          {
            "path": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome/🔣️.json",
            "kind": "file"
          }
        ]
      }
    ]
  }
]
```

This checks actual authored helper scope and targets only. The full original host runtimes and fresh whole-tree participation remain pending.

### Literalization Input ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs

SHA-256 3e287d3b2aaa8007f44db65170431e444a4228a03f66b3dae2c36599e5199748

```rust
use semio_repo_test_host::law::vector::Vector;

/// 🧫️ One committed vector of `apply-directory-page`, read literally from `✏️editor/🫧️transient/🧫️fixtures`.
pub(super) fn vector(id: &str) -> Result<Vector, String> {
    macro_rules! committed {
        ($name:literal, $observable:expr) => {
            Vector {
                before: include_str!(concat!("../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/⬅️before/🔣️.json")),
                mutation: include_str!(concat!("../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🦠️mutation/🔣️.json")),
                after: include_str!(concat!("../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/📸️snapshot/➡️after/🔣️.json")),
                diff: include_str!(concat!("../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🔺️diff/🔣️.json")),
                outcome: include_str!(concat!("../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/", $name, "/🎯️outcome/🔣️.json")),
                observable: $observable,
            }
        };
    }
    Ok(match id {
        "apply-directory-page-applied" => committed!("✅️apply", true),
        "apply-directory-page-no-op" => committed!("🟰️apply", false),
        "apply-directory-page-rejected" => committed!("🚫️apply", false),
        other => return Err(format!("no committed vector for {other:?}")),
    })
}

```

### Literalization Input 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures/🦀️.rs

SHA-256 dbfe0215fa33b81087b74ad4b80276a3a43fdb954145757a5e7b084255d9c629

```rust
/// 🏷️ Every committed vector's scenario id, in the catalog's own order — duplicated, not imported,
/// because the oracle-only build must not link the subject crate. The contract's mutation-coverage
/// gate keeps this list honest against the catalog. `sets-*` vectors are applied, `keeps-*` no-ops.
pub(super) const VECTORS: &[&str] = &[
    "sets-appearance",
    "keeps-appearance",
    "sets-layout",
    "keeps-layout",
    "sets-driver",
    "keeps-driver",
    "sets-custom-driver",
    "keeps-custom-driver",
    "sets-locale",
    "keeps-locale",
    "sets-terminology",
    "keeps-terminology",
    "sets-theme",
    "keeps-theme",
    "sets-custom-theme",
    "keeps-custom-theme",
    "sets-keybinding",
    "keeps-keybinding",
];

/// 🧫️ Embeds one vector's `(before, mutation, after, outcome)` quintet text, read literally.
macro_rules! vector {
    ($leaf:literal, $vector:literal) => {
        (
            include_str!(concat!("../../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/⬅️before/🔣️.json")),
            include_str!(concat!("../../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🦠️mutation/🔣️.json")),
            include_str!(concat!("../../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/📸️snapshot/➡️after/🔣️.json")),
            include_str!(concat!("../../../../../../🎚️config/🧬️schema/🧬️mutations/", $leaf, "/🧫️fixtures/", $vector, "/🎯️outcome/🔣️.json")),
        )
    };
}

/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one scenario —
/// this IS the independently handcrafted vector the no-oracle decision rests on, never recomputed.
pub(super) fn fixture_text(scenario: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match scenario {
        "sets-appearance" => vector!("🌗️set-appearance", "✏️sets-appearance"),
        "keeps-appearance" => vector!("🌗️set-appearance", "🟰️keeps-appearance"),
        "sets-layout" => vector!("📐️set-layout", "✏️sets-layout"),
        "keeps-layout" => vector!("📐️set-layout", "🟰️keeps-layout"),
        "sets-driver" => vector!("🕹️set-driver", "✏️sets-driver"),
        "keeps-driver" => vector!("🕹️set-driver", "🟰️keeps-driver"),
        "sets-custom-driver" => vector!("🚗️set-custom-driver", "✏️sets-custom-driver"),
        "keeps-custom-driver" => vector!("🚗️set-custom-driver", "🟰️keeps-custom-driver"),
        "sets-locale" => vector!("🗣️set-locale", "✏️sets-locale"),
        "keeps-locale" => vector!("🗣️set-locale", "🟰️keeps-locale"),
        "sets-terminology" => vector!("📖️set-terminology", "✏️sets-terminology"),
        "keeps-terminology" => vector!("📖️set-terminology", "🟰️keeps-terminology"),
        "sets-theme" => vector!("🖼️set-theme", "✏️sets-theme"),
        "keeps-theme" => vector!("🖼️set-theme", "🟰️keeps-theme"),
        "sets-custom-theme" => vector!("🎨️set-custom-theme", "✏️sets-custom-theme"),
        "keeps-custom-theme" => vector!("🎨️set-custom-theme", "🟰️keeps-custom-theme"),
        "sets-keybinding" => vector!("⌨️set-keybinding-override", "✏️sets-keybinding"),
        "keeps-keybinding" => vector!("⌨️set-keybinding-override", "🟰️keeps-keybinding"),
        other => panic!("mutate-os-config-ui-preferences: no specification vector registered for scenario {other:?}"),
    }
}

```

## Actual Whole-Context Refusal And Direct Fixture Inputs

Third registered whole source census remained RED at both new private helpers: their root-local scope facts resolve, but the macro expansions still lack an exclusive live lexical scope in actual mounted contexts. The earlier local-scope proof did not establish whole-context admission and no runtime pass was claimed. Root now removes only these two fixture-path producer macros and handcrafts the same three Vector records and eighteen tuples with direct literal includes. Every original id/order/observable flag, fallback/error and JSON target is retained; canonical resolver comparison of all87 current targets is required. No source enforcement rule is weakened.

## Canonical Literal Target Preservation

```json
[
  {
    "helper": "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫧️mutate-s-home-1-any-editor-transient/🧫️fixtures/🦀️.rs",
    "sha256": "67bc6e53228e4af5176f5e65ad9fd5c1d47f63e699b7a6c53c559467803d9950",
    "bytes": 3109,
    "literalTargets": 15,
    "allOriginalTargetsPreserved": true,
    "remainingLocalMacroReferences": 0
  },
  {
    "helper": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/🧫️fixtures/🦀️.rs",
    "sha256": "843efcfb53e397771957fa4c17828cd06f2ba9c9f756a311864ffd4e001bc2cf",
    "bytes": 15382,
    "literalTargets": 72,
    "allOriginalTargetsPreserved": true,
    "remainingLocalMacroReferences": 0
  }
]
```

The actual canonical Rust scanner/resolver yields the identical15/72 original JSON target multisets after literalization, no remaining local macro reference, and all referenced files/ancestors are physical and unlinked. This does not claim native execution or whole-context source closure.
