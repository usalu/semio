//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactDsl
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the native codec is one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>`,
//! unsplit. This component owns the real `parse_dsl`/`print_dsl` impl plus the thin artifact-facing
//! wrappers and the canonical example fixtures and their round-trip laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::FormsSnapshot;

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `FormsSnapshot::__dsl_spec()`, the same record
/// the pack encodes; a parsed document must also pass `FormsSnapshot::validate`.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    fn envelope_id() -> &'static str {
        crate::FORMS_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                    return Err(store::TextError::new("Forms text envelope mismatch", dsl::TextSpan::at(1, 1)));
                }
                rest
            },
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        snapshot.validate().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📄️ The `default` (Contact) fixture — a minimal single-step form, handcrafted in the shared
/// `playbook` kernel's own step/block DSL grammar (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM:
/// this is real authored domain content, loaded via [`parse_playbook_example_dsl`] — NOT
/// `FormsSnapshot`'s own persisted wire format, whose `structure`/`results` child handles are
/// content-addressed opaque references that cannot host hand-authored prose).
pub const DEFAULT_EXAMPLE_TEXT: &str = r##"semio forms.form.dsl v1
schema=forms.form id=default version="1" title=Contact steps=[ id=contact title=Contact blocks=[ id=name label=Name kind=text required=true placeholder="Your name"
condition {
}
id=email label=Email kind=text required=true placeholder="you@example.com"
condition {
}
id=message label=Message kind=longText placeholder="How can we help?"
condition {
}
] ]"##;

/// 📄️ The `onboarding` fixture — a multi-step form exercising every built-in question kind and a
/// conditional block, in the `.forms` DSL.
pub const ONBOARDING_EXAMPLE_TEXT: &str = r##"semio forms.form.dsl v1
schema=forms.form id=onboarding version="1" title="Product Onboarding" steps=[ id=profile title=Profile description="Tell us about yourself." blocks=[ id=full-name label="Full name" kind=text required=true default="Alex Example"
condition {
}
id=bio label=Bio kind=longText placeholder="Short introduction"
condition {
}
id=age label=Age kind=number min=13 max=120 default=28
condition {
}
id=avatar label=Avatar kind=image src=""
condition {
}
id=resume label=Resume kind=file accept=".pdf,.doc,.docx"
condition {
}
] id=preferences title=Preferences description="Customize your experience." blocks=[ id=theme-color label="Accent color" kind=color default="#336699"
condition {
}
id=start-date label="Start date" kind=date default="2026-07-01"
condition {
}
id=notifications label="Enable notifications" kind=boolean default=true
condition {
}
id=volume label="Notification volume" kind=slider min=0 max=100 step=5 unit="%" default=60
condition {
}
id=plan label=Plan kind=single required=true default="pro" options=[ value=free label=Free value=pro label=Pro value=team label=Team ]
condition {
}
id=features label=Features kind=multi default=[ "analytics" ] options=[ value=analytics label=Analytics value=automation label=Automation value=collab label=Collaboration ]
condition {
}
id=offset label="Workspace offset" kind=vector step=0.5 schema=vec3 fields=[ key=x label=X value=0 key=y label=Y value=0 key=z label=Z value=0 ]
condition {
}
id=welcome-note label=Welcome kind=note text="Thanks for trying every question kind in one fixture."
condition {
}
] id=advanced title=Advanced blocks=[ id=show-team-size label="Specify team size" kind=boolean default=false
condition {
}
id=team-size label="Team size" kind=slider min=1 max=50 step=1 default=5
condition {
  truthy
  expr {
    var name=show-team-size
  }
}
id=team-role label="Primary role" kind=single options=[ value=design label=Design value=engineering label=Engineering value=product label=Product ]
condition {
  truthy
  expr {
    var name=show-team-size
  }
}
] ]"##;

/// 📖️ Parses `.forms` DSL text into a `FormsSnapshot` — `FormsSnapshot`'s OWN persisted wire
/// format, the derived text of its own `dsl::DslRecord` spec.
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🌉 Parses the shared `playbook` kernel's own step/block DSL grammar (the human-authored form
/// this facet's three example fixtures have always been handcrafted in) into a real, cache-warm
/// `FormsSnapshot` via [`crate::forms_snapshot_with_state`] — the PERMANENT
/// loading path for `building_component_spec`/`default_example_spec`/`onboarding_example_spec`
/// (`🧬️schema/🦀️component.rs`'s `🔖️DocumentHelpers`), never [`parse_dsl`] above.
///
/// Why: `parse_dsl` decodes `FormsSnapshot`'s OWN persisted wire format — two content-addressed
/// `structure`/`results` handles, no step/block content at all (that content lives in the composed
/// children, resolved through the session-side working-scene cache until a real
/// `ArtifactView::with_children` seam lands — see `crate::🔖️Composition`'s own
/// doc). A handle decoded fresh from a *previous* process (or, as here, from hand-authored example
/// text that was never mint-cached in THIS process) has nothing in the cache to resolve against,
/// so `forms_steps` would read back empty — the same documented staleness gap every composed
/// plugin in this ticket carries for undo-past-history. Loading examples through this function
/// instead sidesteps the gap entirely: it re-derives real step/block content from real playbook
/// grammar text and mints+caches the children in the SAME call, so the returned snapshot's working
/// scene is always warm.
pub fn parse_playbook_example_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
    let body = match store::semio_format::split_text_preamble(text) {
        Ok((_, rest)) => rest,
        Err(_) => text,
    };
    let record = dsl::parse(body, &semio_framework_artifact_playbook_playbook::PlaybookSpec::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    let spec = semio_framework_artifact_playbook_playbook::PlaybookSpec::__dsl_from_record(&record)?;
    Ok(crate::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, &spec.steps))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ExternalBridges
/// 📖️ Parses `.forms` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_forms_dsl(text: &str) -> Result<FormsSnapshot, String> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`FormsSnapshot`] back to `.forms` DSL text under a name an external caller can reach, paired
/// with [`parse_forms_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_forms_dsl(snapshot: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges
