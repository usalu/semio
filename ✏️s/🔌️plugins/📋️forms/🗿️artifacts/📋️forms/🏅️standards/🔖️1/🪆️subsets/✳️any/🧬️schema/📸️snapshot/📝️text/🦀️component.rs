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
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../../../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📄️ The `default` (Contact) fixture — a minimal single-step form, in the `.forms` DSL.
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

/// 📖️ Parses `.forms` DSL text into a `FormsSnapshot`.
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::os_store::test_support::assert_dsl_round_trip;

    #[test]
    fn building_component_fixture_dsl_round_trips() {
        let spec = parse_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        assert_eq!(spec.id, "building-component");
        assert_eq!(spec.steps.len(), 2);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn default_fixture_dsl_round_trips() {
        let spec = parse_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        assert_eq!(spec.id, "default");
        assert_eq!(spec.steps.len(), 1);
        assert_dsl_round_trip(&spec);
    }

    #[test]
    fn onboarding_fixture_dsl_round_trips() {
        let spec = parse_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        assert_eq!(spec.id, "onboarding");
        assert_eq!(spec.steps.len(), 3);
        assert_dsl_round_trip(&spec);
    }
}
//#endregion 🧪️Tests
