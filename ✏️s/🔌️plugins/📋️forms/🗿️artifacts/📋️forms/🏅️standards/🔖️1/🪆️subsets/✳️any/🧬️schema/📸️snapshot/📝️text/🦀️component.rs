//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::ArtifactDsl for FormsSnapshot` lives in `📸️snapshot/🧬️schema`. This component adds the thin
//! artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example fixtures and their
//! round-trip laws.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::forms::FormsSnapshot;

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

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
/// format (hand-rolled directly on the composed `structure`/`results` child slots since ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM; see `📸️snapshot/🧬️schema`'s `🔖️HandcraftedArtifactCodecs`).
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

/// 🌉 Parses the shared `playbook` kernel's own step/block DSL grammar (the human-authored form
/// this facet's three example fixtures have always been handcrafted in) into a real, cache-warm
/// `FormsSnapshot` via [`crate::artifacts::forms::forms_snapshot_with_state`] — the PERMANENT
/// loading path for `building_component_spec`/`default_example_spec`/`onboarding_example_spec`
/// (`🧬️schema/🦀️component.rs`'s `🔖️DocumentHelpers`), never [`parse_dsl`] above.
///
/// Why: `parse_dsl` decodes `FormsSnapshot`'s OWN persisted wire format — two content-addressed
/// `structure`/`results` handles, no step/block content at all (that content lives in the composed
/// children, resolved through the session-side working-scene cache until a real
/// `ArtifactView::with_children` seam lands — see `crate::artifacts::forms::🔖️Composition`'s own
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
    let record = dsl::parse(body, &flow::playbook::PlaybookSpec::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    let spec = flow::playbook::PlaybookSpec::__dsl_from_record(&record)?;
    Ok(crate::artifacts::forms::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, spec.steps))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::forms_steps;
    use store::os_store::test_support::assert_dsl_round_trip;

    /// 🩹️ Each test builds `spec` via [`parse_playbook_example_dsl`] (real content, cache-warm in
    /// THIS call) rather than [`parse_dsl`] on the raw example text directly — `FormsSnapshot`'s
    /// own persisted codec is independently proven correct by `📸️snapshot/🧬️schema`'s
    /// `snapshot_dsl_round_trips_with_composed_children`; what these three prove is that the
    /// example fixtures parse as real playbook content AND that `assert_dsl_round_trip` (which
    /// exercises `FormsSnapshot::print_dsl`/`parse_dsl` on the resulting cache-warm snapshot) holds
    /// for them too.
    #[test]
    fn building_component_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        assert_eq!(spec.id, "building-component");
        assert_eq!(forms_steps(&spec).len(), 2);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn default_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        assert_eq!(spec.id, "default");
        assert_eq!(forms_steps(&spec).len(), 1);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn onboarding_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        assert_eq!(spec.id, "onboarding");
        assert_eq!(forms_steps(&spec).len(), 3);
        assert_dsl_round_trip(&spec);
    }
}
//#endregion 🧪️Tests
