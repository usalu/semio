//! 🧭 Traceability — trace chains and audit trail queries.

use crate::kernel::{EntityId, TraceKind, TraceLink};
use crate::program::Program;
use crate::registers::AuditEvent;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

// #region 🔖TraceChain
/// @emoji ⛓️ Ordered chain of trace links from a root entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceChain {
    pub root_id: EntityId,
    pub links: Vec<TraceLink>,
}

/// @emoji 📜 Filtered audit trail slice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditTrail {
    pub subject_id: Option<EntityId>,
    pub events: Vec<AuditEvent>,
}

/// @emoji 💥 Reverse impact set from trace links pointing at an entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpactTrace {
    pub target_id: EntityId,
    pub upstream_ids: Vec<EntityId>,
    pub links: Vec<TraceLink>,
}
// #endregion

// #region 🔖TraceQueries
/// @emoji 🔗 Builds a forward trace chain from `root_id` following kind-appropriate links.
pub fn trace_chain(program: &mut Program, root_id: &EntityId) -> TraceChain {
    embed_requirement_traces(program);
    let adjacency = trace_adjacency(&program.traces);
    let mut visited = HashSet::new();
    let mut links = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_id.clone());
    visited.insert(root_id.clone());
    while let Some(current) = queue.pop_front() {
        if let Some(outgoing) = adjacency.get(&current) {
            for link in outgoing {
                if !follows_kind_chain(&link.kind) {
                    continue;
                }
                links.push(link.clone());
                if visited.insert(link.to_id.clone()) {
                    queue.push_back(link.to_id.clone());
                }
            }
        }
    }
    TraceChain {
        root_id: root_id.clone(),
        links,
    }
}

/// @emoji 🔍 Finds trace links touching `entity_id` (from or to).
pub fn trace_links_for(program: &mut Program, entity_id: &EntityId) -> Vec<TraceLink> {
    embed_requirement_traces(program);
    program
        .traces
        .iter()
        .filter(|link| &link.from_id == entity_id || &link.to_id == entity_id)
        .cloned()
        .collect()
}

/// @emoji ↩️ Reverse impact trace — entities that depend on or satisfy `target_id`.
pub fn trace_impact(program: &mut Program, target_id: &EntityId) -> ImpactTrace {
    embed_requirement_traces(program);
    let mut upstream = HashSet::new();
    let mut links = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(target_id.clone());
    while let Some(current) = queue.pop_front() {
        for link in &program.traces {
            if &link.to_id != &current {
                continue;
            }
            if matches!(
                link.kind,
                TraceKind::ObjectiveToRequirement
                    | TraceKind::StakeholderToRequirement
                    | TraceKind::FunctionToProgramElement
                    | TraceKind::RequirementToDecision
                    | TraceKind::RequirementToRisk
                    | TraceKind::ConstraintToImpact
            ) {
                links.push(link.clone());
                if upstream.insert(link.from_id.clone()) {
                    queue.push_back(link.from_id.clone());
                }
            }
        }
    }
    ImpactTrace {
        target_id: target_id.clone(),
        upstream_ids: upstream.into_iter().collect(),
        links,
    }
}

/// @emoji 📋 Returns audit events for an optional subject, newest first.
pub fn audit_trail(program: &Program, subject_id: Option<&EntityId>) -> AuditTrail {
    let mut events: Vec<AuditEvent> = program
        .audit_events
        .iter()
        .filter(|event| subject_id.is_none_or(|id| &event.subject_id == id))
        .cloned()
        .collect();
    events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    AuditTrail {
        subject_id: subject_id.cloned(),
        events,
    }
}

/// @emoji ➕ Appends a trace link to the program trace register.
pub fn add_trace_link(program: &mut Program, from_id: EntityId, to_id: EntityId, kind: TraceKind) {
    program.traces.push(TraceLink::new(from_id, to_id, kind));
}

/// @emoji 🔁 Resolves superseded requirements to their terminal replacement.
pub fn resolve_supersedes(program: &Program, requirement_id: &EntityId) -> EntityId {
    let mut current = requirement_id.clone();
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current.clone()) {
            break;
        }
        let Some(next) = program
            .requirements
            .iter()
            .find(|r| &r.header.id == &current)
            .and_then(|r| r.superseded_by.clone())
        else {
            break;
        };
        current = next;
    }
    current
}

/// @emoji 🧷 Copies requirement-embedded trace links into the program trace register.
fn embed_requirement_traces(program: &mut Program) {
    for requirement in &program.requirements {
        for link in &requirement.trace_links {
            if program.traces.iter().any(|t| t.id == link.id) {
                continue;
            }
            program.traces.push(link.clone());
        }
    }
}

fn follows_kind_chain(kind: &TraceKind) -> bool {
    !matches!(kind, TraceKind::FullAuditTrail)
}

fn trace_adjacency(traces: &[TraceLink]) -> HashMap<EntityId, Vec<TraceLink>> {
    let mut map: HashMap<EntityId, Vec<TraceLink>> = HashMap::new();
    for link in traces {
        map.entry(link.from_id.clone()).or_default().push(link.clone());
    }
    map
}
// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::TraceKind;
    use crate::program::sample_program;

    #[test]
    fn trace_chain_follows_links() {
        let mut program = sample_program();
        let a = program.elements[0].header.id.clone();
        let b = program.elements[1].header.id.clone();
        add_trace_link(&mut program, a.clone(), b.clone(), TraceKind::FunctionToProgramElement);
        let chain = trace_chain(&mut program, &a);
        assert_eq!(chain.links.len(), 1);
        assert_eq!(chain.links[0].to_id, b);
    }

    #[test]
    fn audit_trail_sorted_newest_first() {
        let mut program = sample_program();
        program.audit_events.push(AuditEvent {
            header: crate::kernel::EntityHeader::new(EntityId::new_serial("audit"), "older"),
            action: crate::registers::AuditAction::Created,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: "2020-01-01T00:00:00Z".into(),
            details: crate::kernel::TextField::plain("old"),
            before_state: None,
            after_state: None,
            ip_address: None,
            client: None,
            session_id: None,
            change_record_id: None,
            trace_link: None,
            success: true,
            error_message: None,
            correlation_id: None,
            compliance_tags: Vec::new(),
            retention_until: None,
        });
        program.audit_events.push(AuditEvent {
            header: crate::kernel::EntityHeader::new(EntityId::new_serial("audit"), "newer"),
            action: crate::registers::AuditAction::Updated,
            actor_id: None,
            subject_id: program.elements[0].header.id.clone(),
            subject_kind: "element".into(),
            timestamp: "2025-01-01T00:00:00Z".into(),
            details: crate::kernel::TextField::plain("new"),
            before_state: None,
            after_state: None,
            ip_address: None,
            client: None,
            session_id: None,
            change_record_id: None,
            trace_link: None,
            success: true,
            error_message: None,
            correlation_id: None,
            compliance_tags: Vec::new(),
            retention_until: None,
        });
        let trail = audit_trail(&program, None);
        assert!(trail.events[0].timestamp > trail.events[1].timestamp);
    }

    #[test]
    fn trace_impact_collects_upstream() {
        let mut program = sample_program();
        let req_id = EntityId::new_serial("requirement");
        let elem_id = program.elements[0].header.id.clone();
        add_trace_link(&mut program, req_id.clone(), elem_id.clone(), TraceKind::ObjectiveToRequirement);
        let impact = trace_impact(&mut program, &elem_id);
        assert!(impact.upstream_ids.contains(&req_id));
    }
}
