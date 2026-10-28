"""🧱️ Replace standalone native Body admission with one local clause and canonical preparation."""
from pathlib import Path
c = Path("C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/🧩️concepts")
b, old = c / "🧱️body", c / "🔺️tessellation"
source = (old / "🦀️.rs").read_text(encoding="utf-8")
state = source[source.index("#[derive(Default, Clone, Copy"):source.index("#[derive(Default, semio_framework_value::RetireOwned)]")]
state = state.replace("struct State", "pub(super) struct ClauseState").replace("    binding: Option<(usize, usize, u64)>,\n", "").replace("artifact_retire_leaf!(State)", "artifact_retire_leaf!(ClauseState)")
phases = source[source.index("            6 => {", source.index("match self.state.phase {", source.index("pub fn advance"))):source.index('            _ => return Err(invalid("mvd.state").into()),')]
phases = phases.replace("self.input()?", "request")
phases = phases.replace("self.source.as_ref().unwrap().try_authority()?.as_ref()", 'view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?')
phases = phases.replace("self.captured().ok()?", "view?.snapshot(cx).ok()?")
phases = phases.replace("self.captured()?", 'view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?')
phases = phases.replace("self.index.find(id)", "lookup(view, id, cx)?")
phases = phases.replace("revision,cx", "epoch,cx")
phases = phases.replace('else{"Tessellation"}', "else{self.policy.representation_type()}")
phases = phases.replace('progress.copied_bytes += growth.copy_bytes + text.len();', 'progress.copied_bytes += growth.copy_bytes + 2 * text.len();')
start, end = phases.index("            8 => {"), phases.index("            9 => {")
phases = phases[:start] + '''            8 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                if self.member.advance(source) {
                    let result = self.member.result();
                    if result == Some(true) { self.state.phase = 9; }
                    else {
                        self.state.failed = result == Some(false);
                        self.state.incomplete = result.is_none();
                        self.queue(if result.is_none() { "mvd.target.unknown" } else { "mvd.target.type" }, Some(Owner { entity: self.state.position, part: None, argument: usize::MAX }), None, 15);
                    }
                }
                progress.copied_bytes += self.member.copy_bound();
            }
''' + phases[end:]
phases = phases.replace('self.member.start(position, "IFCSHAPEREPRESENTATION");', 'self.member.start(position, "IFCSHAPEREPRESENTATION");\n                        progress.copied_bytes += self.member.copy_bound();')
phases = phases.replace('''                            self.member.start(position, "IFCTESSELLATEDFACESET");
                            self.state.resume = 13;
                            self.state.phase = 8;''', '''                            self.family.start(position);
                            progress.copied_bytes += self.family.copy_bound();
                            self.state.phase = 18;''')
phases += '''            18 => {
                let source = view.ok_or_else(|| invalid("mvd.body.prepared-source"))?.snapshot(cx)?;
                if self.family.advance(source) {
                    match self.family.result() {
                        BodyFamilyResult::Resolved(true) => self.state.phase = 13,
                        BodyFamilyResult::Unknown | BodyFamilyResult::Resolved(false) => {
                            let unknown = self.family.result() == BodyFamilyResult::Unknown;
                            self.state.incomplete |= unknown;
                            self.state.failed |= !unknown;
                            self.queue(if unknown { "mvd.items.unknown" } else { "mvd.items.type" }, self.state.owner, Some(self.state.item), 13);
                        }
                        BodyFamilyResult::Pending => return Err(invalid("mvd.body.family-pending").into()),
                    }
                    self.state.item += 1;
                }
                progress.copied_bytes += self.family.copy_bound();
            }
'''
queue = source[source.index("    fn queue("):source.index("    pub fn advance(")]
clause = '''//! 🧱️ Local Body clauses borrow authenticated request, population and request-index views for one funded turn.
use super::*;
use super::helpers::*;
use super::super::population::{IfcMvdPopulationLookup, IfcMvdPopulationPin, IfcMvdPopulationRequestPin, IfcMvdPopulationRequestView, IfcMvdPopulationStatus, IfcMvdPopulationView};
use super::super::preparation::{IfcMvdRequestIndexPin, IfcMvdRequestIndexView};
use super::super::source_index::IdMatch;
use crate::validation::metadata;
''' + state + '''
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct IfcBodyClause {
    pub(super) report: Option<IfcBodyConceptReport>,
    pub(super) diagnostic: Option<IfcConceptDiagnostic>,
    member: Membership,
    family: BodyFamilyMembership,
    query: IfcAttributeQueryWork,
    state: ClauseState,
    policy: BodyPolicy,
    pin: Option<IfcMvdPopulationPin>,
    request_pin: Option<IfcMvdPopulationRequestPin>,
    requests_pin: Option<IfcMvdRequestIndexPin>,
}
#[cfg(test)]
pub(super) type ClauseMetadata = (ClauseState, BodyPolicy, Option<IfcMvdPopulationPin>, Option<IfcMvdPopulationRequestPin>, Option<IfcMvdRequestIndexPin>);
fn lookup(view: Option<&IfcMvdPopulationView<'_, IfcBodyConceptRequest>>, id: u64, cx: &semio_framework_job::StepContext<'_>) -> Result<IdMatch, ValueError> {
    Ok(match view.ok_or_else(|| invalid("mvd.body.prepared-index"))?.lookup(id, cx)? { IfcMvdPopulationLookup::Missing => IdMatch::Missing, IfcMvdPopulationLookup::Unique(row) => IdMatch::Unique(row), IfcMvdPopulationLookup::Ambiguous => IdMatch::Ambiguous })
}
impl IfcBodyClause {
    pub(super) fn new(policy: BodyPolicy) -> Self { Self { report: None, diagnostic: None, member: Default::default(), family: BodyFamilyMembership::new(policy), query: Default::default(), state: Default::default(), policy, pin: None, request_pin: None, requests_pin: None } }
    fn base() -> RetirementDemand {
        RetirementDemand { copy_bytes: 4 * size_of::<ClauseState>() + 4 * size_of::<IfcConceptTargetReport>() + 2 * size_of::<IfcAttributeQueryWork>() + 2 * size_of::<Membership>() + 2 * size_of::<BodyFamilyMembership>() + 2 * size_of::<BodyPolicy>() + 2 * size_of::<Option<IfcMvdPopulationPin>>() + 2 * size_of::<Option<IfcMvdPopulationRequestPin>>() + 2 * size_of::<Option<IfcMvdRequestIndexPin>>() + 4 * 256, depth: 1, ..Default::default() }
    }
    pub(super) fn demands(&self, request: &IfcBodyConceptRequest) -> Result<RetirementDemand, ValueError> {
        let child = match self.state.phase {
            6 if self.state.target < request.targets.len() => {
                let rows = &self.report.as_ref().unwrap().targets;
                let growth = vector_growth::<IfcConceptTargetReport>(rows.len(), rows.capacity())?;
                let text = if growth.capacity_bytes == 0 { request.targets[self.state.target].representation.len() } else { 0 };
                RetirementDemand { copy_bytes: growth.copy_bytes + 2 * text, capacity_bytes: growth.capacity_bytes + text, ..growth }
            }
            7 | 8 => RetirementDemand { copy_bytes: self.member.copy_bound(), depth: 1, ..Default::default() },
            10 => self.query.demands(),
            13 | 18 => RetirementDemand { copy_bytes: self.family.copy_bound(), depth: 1, ..Default::default() },
            14 => {
                let report = self.report.as_ref().unwrap();
                let list = if self.state.source_invalid && self.state.target == 0 && self.state.resume == 6 { &report.diagnostics } else { &report.targets[self.state.target].diagnostics };
                diagnostic_demand(self.state.pending.unwrap(), self.state.diagnostic_step, list)?
            }
            _ => RetirementDemand::default(),
        };
        let mut demand = Self::base();
        demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("mvd.body.capacity"))?;
        demand.capacity_bytes = child.capacity_bytes;
        demand.release_bytes = child.release_bytes;
        demand.depth = demand.depth.max(child.depth);
        Ok(demand)
    }
    fn check_requests(&self, requests: &IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, cx: &semio_framework_job::StepContext<'_>) -> Result<IfcMvdRequestIndexPin, ValueError> {
        let pin = requests.pin(cx)?;
        if self.requests_pin.is_some_and(|old| !old.matches(requests, cx)) || self.state.authority.is_some_and(|old| !old.matches(cx)) || self.state.donated { return Err(invalid("mvd.body.clause-source")); }
        Ok(pin)
    }
    pub(super) fn advance_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let pin = view.pin(cx)?;
        let requests_pin = self.check_requests(&requests, cx)?;
        if self.request_pin.is_some() || pin.epoch() != epoch || !requests.matches_prepared(&view, cx) || self.pin.is_some_and(|old| !old.matches(&view, cx)) { return Err(invalid("mvd.body.clause-source").into()); }
        let request = view.request(cx)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands(request)?) { return Ok(false); }
        if self.state.done { return Ok(true); }
        self.pin = Some(pin);
        self.requests_pin = Some(requests_pin);
        self.advance_admitted(Some(&view), request, epoch, cx)
    }
    pub(super) fn advance_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let pin = view.pin(cx)?;
        let requests_pin = self.check_requests(&requests, cx)?;
        if self.pin.is_some() || pin.epoch() != epoch || view.status(cx)? != IfcMvdPopulationStatus::Unavailable || !requests.matches_unavailable(&view, cx) || self.request_pin.is_some_and(|old| !old.matches(&view, cx)) { return Err(invalid("mvd.body.clause-source").into()); }
        let request = view.request(cx)?;
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), self.demands(request)?) { return Ok(false); }
        if self.state.done { return Ok(true); }
        self.request_pin = Some(pin);
        self.requests_pin = Some(requests_pin);
        self.advance_admitted(None, request, epoch, cx)
    }
''' + queue + '''
    fn advance_admitted(&mut self, view: Option<&IfcMvdPopulationView<'_, IfcBodyConceptRequest>>, request: &IfcBodyConceptRequest, epoch: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: Self::base().copy_bytes, ..Default::default() };
        match self.state.phase {
            0 => {
                self.state.authority = Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
                self.report = Some(empty_report(self.policy));
                self.state.phase = 6;
                if view.is_none() { self.state.source_invalid = true; self.queue("mvd.source", None, None, 6); }
            }
''' + phases + '''            _ => return Err(invalid("mvd.body.state").into()),
        }
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(self.state.done)
    }
    pub(super) fn take_demands() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<IfcBodyConceptReport>>() + size_of::<bool>(), depth: 1, ..Default::default() } }
    fn take(&mut self, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "mvd.cancelled").into()); }
        if self.state.authority.is_none_or(|old| !old.matches(cx)) { return Err(invalid("mvd.original-source").into()); }
        let demand = Self::take_demands();
        if cx.should_yield() || !retained_clone_grant_admits(cx.retained_grant(), demand) { return Ok(false); }
        if !self.state.done || self.state.donated || output.is_some() { return Err(invalid("mvd.body.output").into()); }
        *output = self.report.take();
        self.state.donated = true;
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    pub(super) fn take_prepared(&mut self, view: IfcMvdPopulationView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.check_requests(&requests, cx)?;
        if self.pin.is_none_or(|old| !old.matches(&view, cx)) || !requests.matches_prepared(&view, cx) { return Err(invalid("mvd.body.clause-source").into()); }
        self.take(output, cx)
    }
    pub(super) fn take_unavailable(&mut self, view: IfcMvdPopulationRequestView<'_, IfcBodyConceptRequest>, requests: IfcMvdRequestIndexView<'_, IfcBodyConceptRequest>, output: &mut Option<IfcBodyConceptReport>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        self.check_requests(&requests, cx)?;
        if self.request_pin.is_none_or(|old| !old.matches(&view, cx)) || view.status(cx)? != IfcMvdPopulationStatus::Unavailable || !requests.matches_unavailable(&view, cx) { return Err(invalid("mvd.body.clause-source").into()); }
        self.take(output, cx)
    }
    #[cfg(test)]
    pub(super) fn metadata(&self) -> ClauseMetadata { (self.state, self.policy, self.pin, self.request_pin, self.requests_pin) }
}
'''
clause = clause.replace("empty_diagnostic(spec)", "empty_diagnostic(self.policy, spec)")
(b / "clause.rs").write_text(clause, encoding="utf-8")
helpers = (old / "helpers.rs").read_text(encoding="utf-8")
helpers = helpers.replace("use super::super::native::*;\n", "")
helpers = helpers.replace("empty_diagnostic(spec:", "empty_diagnostic(policy: BodyPolicy, spec:").replace("labels(spec.code)", "labels(policy, spec.code)")
helpers = helpers.replace("empty_report()", "empty_report(policy: BodyPolicy)")
beg = helpers.index('        concept: "')
end = helpers.index('        schema_conformance:', beg)
helpers = helpers[:beg] + "        concept: policy.concept(),\n        source: policy.source(),\n" + helpers[end:]
helpers = helpers.replace("fn labels(code:", "fn labels(policy: BodyPolicy, code:")
helpers = helpers.replace('"mvd.items.type" => ("A representation item is not a tessellated face set.", "Ein Repräsentationselement ist kein tessellierter Flächensatz."),', '"mvd.items.type" => match policy { BodyPolicy::Tessellation => ("A representation item is not a tessellated face set.", "Ein Repräsentationselement ist kein tessellierter Flächensatz."), BodyPolicy::SweptSolid => ("A representation item is not an untapered extruded or revolved area solid.", "Ein Repräsentationselement ist kein unverjüngter extrudierter oder rotierter Flächenkörper.") },')
(b / "helpers.rs").write_text(helpers, encoding="utf-8")
work = (c / "📐️alignment/direct-work.rs").read_text(encoding="utf-8")
work = work[:work.index('#[cfg(test)]\n#[path = "direct-population-tests.rs"]')]
work = work.replace("use super::clause::IfcAlignmentDirectClause;", "use super::clause::IfcBodyClause;")
work = work.replace("IfcAlignmentDirectClause", "IfcBodyClause").replace("IfcAlignmentDirectWork", "IfcBodyConceptWork").replace("IfcAlignmentDirectRequest", "IfcBodyConceptRequest").replace("IfcAlignmentDirectReport", "IfcBodyConceptReport").replace("mvd.direct.", "mvd.body.")
work = work.replace("#[derive(Default, semio_framework_value::RetireOwned)]", "#[derive(semio_framework_value::RetireOwned)]")
work = work.replace(" { clause: IfcBodyClause,", " { pub(super) clause: IfcBodyClause,")
work = work.replace("impl IfcBodyConceptWork {", '''impl IfcBodyConceptWork {
    fn new(policy: BodyPolicy) -> Self { Self { clause: IfcBodyClause::new(policy), state: Default::default(), preparation: Default::default() } }
    /// 🔺️ Construct the released Body Tessellation clauses with a fixed immutable policy.
    pub fn tessellation() -> Self { Self::new(BodyPolicy::Tessellation) }
    /// 🌊️ Construct the released untapered Body SweptSolid clauses with a fixed immutable policy.
    pub fn swept_solid() -> Self { Self::new(BodyPolicy::SweptSolid) }''')
work = work.replace("_requests, cx", "requests, cx").replace("clause.advance_prepared(view, epoch, cx)", "clause.advance_prepared(view, requests, epoch, cx)").replace("clause.advance_unavailable(view, epoch, cx)", "clause.advance_unavailable(view, requests, epoch, cx)")
work = work.replace("clause.take_prepared(view, output, cx)", "clause.take_prepared(view, requests, output, cx)").replace("clause.take_unavailable(view, output, cx)", "clause.take_unavailable(view, requests, output, cx)")
work = work.replace("super::clause::ClauseMetadata", "super::clause::ClauseMetadata")
work += '''#[cfg(test)]
impl IfcBodyConceptWork {
    pub(super) fn preparation_stage(&self) -> u8 { self.preparation.stage() }
}
'''
(b / "work.rs").write_text(work, encoding="utf-8")
root = (b / "🦀️.rs").read_text(encoding="utf-8")
root = root.replace("use super::native::Membership;", '''use super::native::*;
use super::concept_types::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, retained_clone_grant_admits};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use crate::catalog::query::{IfcAttributeQueryWork, IfcAttributeQueryStatus, Ifc4x3AttributeMetadata};''')
root = root.replace("use std::mem::size_of;", "use std::{mem::size_of, sync::Arc};")
root = root.replace("mod domain;", "mod domain;\nmod helpers;\nmod clause;\nmod work;\npub use work::IfcBodyConceptWork;")
root = root.replace("impl BodyPolicy {", '''semio_framework_value::artifact_retire_leaf!(BodyPolicy);
fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
impl BodyPolicy {
    fn representation_type(self) -> &'static str { match self { Self::Tessellation => "Tessellation", Self::SweptSolid => "SweptSolid" } }
    fn concept(self) -> &'static str { match self {
        Self::Tessellation => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry",
        Self::SweptSolid => "Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry",
    } }
    fn source(self) -> IfcConceptSource { IfcConceptSource {
        commit: "610ee3f95db9d0380b1eb8de4dc6e17833531518",
        url: match self {
            Self::Tessellation => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry/content.html",
            Self::SweptSolid => "https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_SweptSolid_Geometry/content.html",
        },
        sha256: match self { Self::Tessellation => "b8f905fdf6d7bcb4afa69d20111600589e52f4bef43043b8a7814e0b5da78d32", Self::SweptSolid => "c571f3ac400b669781c09cc666b5cd55031b6a480333df6ac47d8ebb1f8dd7c0" },
    } }''')
root += '''#[cfg(test)]
#[path = "tessellation-tests.rs"]
mod tessellation_tests;
'''
(b / "🦀️.rs").write_text(root, encoding="utf-8")
tests = (old / "🧪️.rs").read_text(encoding="utf-8")
tests = tests.replace("use super::*;", "use super::*;\nuse semio_framework_value::retained_clone::RetainedCloneClose;\nuse semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};")
tests = tests.replace("IfcBodyTessellationWork::default()", "IfcBodyConceptWork::tessellation()").replace("IfcBodyTessellationWork", "IfcBodyConceptWork")
tests = tests.replace('include_str!("cases.json")', 'include_str!("../🔺️tessellation/cases.json")').replace('include_str!("query-cases.json")', 'include_str!("../🔺️tessellation/query-cases.json")')
for name in ["loan", "graph", "close", "request"]:
    tests = tests.replace("fn " + name + "(", "pub(super) fn " + name + "(")
tests = tests.replace("work.diagnostic", "work.clause.diagnostic").replace("work.report", "work.clause.report")
tests = tests.replace("fn body_tessellation_invalid_requests_refuse_before_original_source_capture()", "fn body_tessellation_invalid_requests_refuse_before_population_reads()")
tests = tests.replace("assert!(work.source.is_none());\n        assert_eq!(Arc::strong_count(&source), 1);", "assert!(work.preparation_stage() < 3);")
(b / "tessellation-tests.rs").write_text(tests, encoding="utf-8")
mvd = c.parent / "🦀️.rs"
value = mvd.read_text(encoding="utf-8")
value = value.replace('#[path = "🧩️concepts/🔺️tessellation/🦀️.rs"]\nmod tessellation;\npub use tessellation::IfcBodyTessellationWork;\npub use body::{IfcBodyConceptRequest, IfcBodyConceptReport};', 'pub use body::{IfcBodyConceptRequest, IfcBodyConceptReport, IfcBodyConceptWork};')
mvd.write_text(value, encoding="utf-8")
for name in ["🦀️.rs", "helpers.rs", "🧪️.rs"]:
    (old / name).unlink()
