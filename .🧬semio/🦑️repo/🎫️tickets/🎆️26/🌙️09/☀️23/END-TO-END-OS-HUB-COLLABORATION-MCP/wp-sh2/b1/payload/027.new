//! 🦀️ Local-catalog exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-local-catalog-mutation-semantics` (`../../../../../🎚️config/🔮️oracles/🔣️.json`): `os.config.local-catalog`
//! is this operating system's own device record with no third-party implementation, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures literally — no recomputation. `subject` drives this
//! repository's own `apply_local_catalog_config_mutation_reporting` over the full two-kind vocabulary; every law is asserted
//! INSIDE the subject handler, because a recorded no-oracle case runs no oracle role.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors the derive-generated `LocalCatalogConfigMutation` descriptor order — duplicated, not imported, because the
/// oracle-only build must not link the subject crate.
const KINDS: &[&str] = &["admit-local-document", "retire-local-document"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind, read literally.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "admit-local-document" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📥️admit-local-document/🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/🎯️outcome/🔣️.json"),
        ),
        "retire-local-document" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📤️retire-local-document/🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-os-config-local-catalog: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER catalog, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE catalog.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔮️ The unlisted-retirement guard's reference answer: the retire vector's committed after-catalog, unchanged.
fn unlisted_guard_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (_before, _mutation, after, _outcome) = fixture_text("retire-local-document");
    Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
}

/// 🔁️ The round trip's reference answer: the retire vector's committed two-entry before-catalog itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("retire-local-document");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::{
        apply_local_catalog_config_mutation_reporting, decode_local_catalog_config_mutation_json, decode_local_catalog_json, encode_local_catalog_json, inverse_local_catalog_config_mutation_steps, LocalCatalog, LocalCatalogConfigMutation,
        LocalDocumentStorage,
    };
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn catalog_of(text: &str, label: &str, kind: &str) -> Result<LocalCatalog, String> {
        decode_local_catalog_json(text).map_err(|error| format!("mutate-os-config-local-catalog: the committed {label}-catalog for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<LocalCatalogConfigMutation, String> {
        decode_local_catalog_config_mutation_json(text).map_err(|error| format!("mutate-os-config-local-catalog: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(catalog: &LocalCatalog) -> Result<Json, String> {
        parse_json(&encode_local_catalog_json(catalog))
    }

    fn disagreement(what: &str, got: &LocalCatalog, expected: &LocalCatalog) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_local_catalog_json(got), encode_local_catalog_json(expected))
    }

    fn listed(catalog: &LocalCatalog, document_id: &str) -> bool {
        catalog.documents.iter().any(|entry| entry.document_id == document_id)
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law: the declared document is listed exactly when the row says so, and the sibling entry is
    /// identical before and after.
    fn listing_claims_hold(kind: &str, row: &Json, base: &LocalCatalog, after: &LocalCatalog) -> Result<(), String> {
        let document = row.str("document");
        let sibling = row.str("sibling");
        if base == after {
            return Err(format!("mutate-{kind}: the mutation left the catalog unchanged — the scenario would report a pass for a mutation it never observed"));
        }
        if listed(after, &document) != (row.str("listed") == "yes") {
            return Err(format!("mutate-{kind}: the feature declares {document:?} listed = {:?} afterwards, but the catalog says otherwise", row.str("listed")));
        }
        let before_sibling = base.documents.iter().find(|entry| entry.document_id == sibling);
        let after_sibling = after.documents.iter().find(|entry| entry.document_id == sibling);
        if before_sibling.is_none() || before_sibling != after_sibling {
            return Err(format!("mutate-{kind}: the sibling {sibling:?} must survive untouched"));
        }
        Ok(())
    }

    /// 🎯️ Both committed vectors are clean `applied` vectors, so any diagnostic at all is a divergence.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: both committed local-catalog vectors are clean applied vectors, but this one declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-catalog and asserts, in role, that the result IS the committed
    /// after-catalog, that the listing and sibling claims hold, and that the reported diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = catalog_of(before, "before", kind)?;
            let expected = catalog_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_local_catalog_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied catalog does not match the committed after-catalog"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            listing_claims_hold(kind, &row, &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse restores the committed
    /// before-catalog exactly, admission times included.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = catalog_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_local_catalog_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the catalog untouched, so restoring it proves nothing"));
            }
            for step in inverse_local_catalog_config_mutation_steps(&mutation, &base) {
                let undone = apply_local_catalog_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🚧️ The branch no committed vector can express: retiring an id the catalog does not list changes nothing, raises only
    /// the no-op warning and offers no undo step (an undo would have to invent the entry).
    pub fn unlisted_guard(_ctx: &Context) -> Result<Outcome, String> {
        let (_before, mutation, after, _outcome) = super::fixture_text("retire-local-document");
        let base = catalog_of(after, "after", "retire-local-document")?;
        if listed(&base, "studio-alpha") {
            return Err("unlisted-retirement-has-no-undo: the committed after-catalog of the retire vector must no longer list studio-alpha".to_string());
        }
        let mutation = mutation_of(mutation, "retire-local-document")?;
        let mut current = base.clone();
        let raised = apply_local_catalog_config_mutation_reporting(&mut current, &mutation);
        if raised.iter().map(|(code, _)| code.as_str()).collect::<Vec<_>>() != vec!["mutation.no-op"] {
            return Err(format!("unlisted-retirement-has-no-undo: retiring an unlisted id must raise exactly the no-op warning, but raised {raised:?}"));
        }
        if current != base {
            return Err(disagreement("unlisted-retirement-has-no-undo: retiring an unlisted id must leave the catalog exactly where it was", &current, &base));
        }
        let steps = inverse_local_catalog_config_mutation_steps(&mutation, &base);
        if !steps.is_empty() {
            return Err(format!("unlisted-retirement-has-no-undo: the inverse must be empty, but the implementation offered {} step(s)", steps.len()));
        }
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🔁️ The round trip for a record whose only carrier is its own JSON projection, proven real by reading both ids and
    /// the imported entry's file storage back off the TYPED value.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("retire-local-document");
        let catalog = catalog_of(before, "before", "retire-local-document")?;
        let ids: Vec<&str> = catalog.documents.iter().map(|entry| entry.document_id.as_str()).collect();
        if ids != ["studio-alpha", "studio-imported"] || catalog.documents[1].storage != LocalDocumentStorage::File {
            return Err(format!("local-catalog-round-trip: the committed catalog lists studio-alpha and a file-stored studio-imported, but the decoded value holds {}", encode_local_catalog_json(&catalog)));
        }
        let reencoded = encode_local_catalog_json(&catalog);
        let reparsed = catalog_of(&reencoded, "re-encoded", "retire-local-document")?;
        if reparsed != catalog {
            return Err(disagreement("local-catalog-round-trip: decoding the re-encoded catalog did not reproduce the typed value", &reparsed, &catalog));
        }
        let projection = parse_json(&reencoded)?;
        Ok(Outcome::with_raw(reencoded.into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id (the feature's `Examples` tables).
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("unlisted-retirement-has-no-undo", unlisted_guard_oracle).oracle("local-catalog-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("unlisted-retirement-has-no-undo", subject::unlisted_guard).subject("local-catalog-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
