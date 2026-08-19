//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactDsl
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the native codec is one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>`,
//! unsplit. This component owns the real `parse_dsl`/`print_dsl` impl plus the thin artifact-facing
//! wrappers and the canonical example fixtures and their round-trip laws.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::forms::FormsSnapshot;

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `➗️mathematical`'s/`📐️cad`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef`
/// flattened via `to_uri()`), never the child's own content.
async fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
async fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
async fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
async fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
async fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" { Ok(None) } else { Ok(Some(dec_str(s)?)) }
}
async fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
async fn print_forms_snapshot_body(s: &FormsSnapshot) -> String {
    format!("schema={}\nid={}\nversion={}\ntitle={}\nstructure={}\nresults={}", enc_str(&s.schema), enc_str(&s.id), enc_str(&s.version), enc_opt_str(&s.title), enc_child(&s.structure), enc_child(&s.results))
}
async fn parse_forms_snapshot_body(body: &str) -> Result<FormsSnapshot, String> {
    let mut schema = None;
    let mut id = None;
    let mut version = None;
    let mut title = None;
    let mut structure = None;
    let mut results = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("id=") {
            id = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("version=") {
            version = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("title=") {
            title = Some(dec_opt_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("structure=") {
            structure = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("results=") {
            results = Some(dec_child(rest)?);
        } else {
            return Err(format!("forms snapshot: unknown line {line:?}"));
        }
    }
    Ok(FormsSnapshot {
        schema: schema.ok_or_else(|| "forms snapshot: missing schema line".to_string())?,
        id: id.ok_or_else(|| "forms snapshot: missing id line".to_string())?,
        version: version.ok_or_else(|| "forms snapshot: missing version line".to_string())?,
        title: title.unwrap_or(None),
        structure: structure.ok_or_else(|| "forms snapshot: missing structure line".to_string())?,
        results: results.ok_or_else(|| "forms snapshot: missing results line".to_string())?,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ Real hex/bracket text primitives, hand-rolled directly on `FormsSnapshot` — the previous
/// codec bridged through the shared `flow::playbook::PlaybookSpec` grammar (whose `steps` field
/// mapped 1:1 onto this struct's old bare `steps` field); that bridge cannot express a composed
/// child slot (no `dsl::DslField` impl reachable from this crate for `ArtifactChild<S>`), so this
/// upgrade drops it in favor of the same `enc_child`/`dec_child` pattern `➗️mathematical`/`📐️cad`/
/// `✒️writer` established once their own snapshot gained a real child slot.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    async fn envelope_id() -> &'static str {
        crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_forms_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_forms_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

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
pub async fn parse_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub async fn print_dsl(document: &FormsSnapshot) -> String {
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
pub async fn parse_playbook_example_dsl(text: &str) -> Result<FormsSnapshot, store::TextError> {
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
    use crate::artifacts::forms::{forms_children_from_steps, forms_steps, FormStep, FORMS_DOCUMENT_SCHEMA};
    use store::os_store::test_support::assert_dsl_round_trip;

    #[semio_framework_async_macros::async_test]
    async fn snapshot_dsl_round_trips_with_composed_children() {
        let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
        let (structure, results) = forms_children_from_steps(&steps);
        let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: Some("T".into()), structure, results };
        let printed = store::ArtifactDsl::print_dsl(&snapshot);
        let parsed = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parses");
        assert_eq!(parsed, snapshot);
    }

    /// 🩹️ Each test builds `spec` via [`parse_playbook_example_dsl`] (real content, cache-warm in
    /// THIS call) rather than [`parse_dsl`] on the raw example text directly — `FormsSnapshot`'s
    /// own persisted codec is independently proven correct by this facet's own
    /// `snapshot_dsl_round_trips_with_composed_children`; what these three prove is that the
    /// example fixtures parse as real playbook content AND that `assert_dsl_round_trip` (which
    /// exercises `FormsSnapshot::print_dsl`/`parse_dsl` on the resulting cache-warm snapshot) holds
    /// for them too.
    #[semio_framework_async_macros::async_test]
    async fn building_component_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        assert_eq!(spec.id, "building-component");
        assert_eq!(forms_steps(&spec).len(), 2);
        assert_dsl_round_trip(&spec);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        assert_eq!(spec.id, "default");
        assert_eq!(forms_steps(&spec).len(), 1);
        assert_dsl_round_trip(&spec);
    }

    #[semio_framework_async_macros::async_test]
    async fn onboarding_fixture_dsl_round_trips() {
        let spec = parse_playbook_example_dsl(ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        assert_eq!(spec.id, "onboarding");
        assert_eq!(forms_steps(&spec).len(), 3);
        assert_dsl_round_trip(&spec);
    }
}
//#endregion 🧪️Tests
