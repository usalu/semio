//! 📋️ Fem2d analysis report bridges: JSON in, JSON out, for language-neutral test adapters and third-party solver oracles.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use protocol::Mutation;

/// 🗂️ The three projection axes of an analysis report, computed from the document so that two
/// implementations order the same values the same way: every node a `bar`/`beam` element references
/// in document order, every restrained-and-active `(node, dof)` pair in that same order, and the
/// `bar`/`beam` members in document order. Regions are outside the projection — a meshed continuum
/// is not something a frame solver can express, and this subset's region geometry already has its own
/// two third-party oracles.
pub fn fem2d_frame_axes(doc: &Fem2dSnapshot) -> (Vec<String>, Vec<(String, String)>, Vec<String>) {
    let ends = |element: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement| match element {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement::Bar { start, end, .. } => (start.clone(), end.clone(), false),
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement::Beam { start, end, .. } => (start.clone(), end.clone(), true),
    };
    let mut active: Vec<(String, [bool; 3])> = Vec::new();
    for element in &doc.elements {
        let (start, end, bends) = ends(element);
        for node in [start, end] {
            match active.iter().position(|(id, _)| id == &node) {
                Some(at) => {
                    active[at].1 = [true, true, active[at].1[2] || bends];
                }
                None => active.push((node, [true, true, bends])),
            }
        }
    }
    let nodes: Vec<String> = doc.nodes.iter().map(|node| node.id.clone()).filter(|id| active.iter().any(|(node, _)| node == id)).collect();
    let mut pairs = Vec::new();
    for node in &nodes {
        let flags = active.iter().find(|(id, _)| id == node).map_or([false; 3], |(_, flags)| *flags);
        for (at, name) in PLANAR_DOFS.iter().enumerate() {
            let held = doc.supports.iter().any(|support| &support.node_id == node && support.fixed.iter().any(|dof| fem2d_dof_name(*dof) == *name));
            if flags[at] && held {
                pairs.push((node.clone(), (*name).to_string()));
            }
        }
    }
    let members: Vec<String> = doc.elements.iter().map(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id).map(str::to_string).collect();
    (nodes, pairs, members)
}

/// 🔤️ A document degree of freedom as the wire spells it.
pub(crate) fn fem2d_dof_name(dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof) -> &'static str {
    match dof {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tx => "Tx",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty => "Ty",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz => "Tz",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Rx => "Rx",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ry => "Ry",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Rz => "Rz",
    }
}

/// 📤️ One solved case, projected onto the three axes.
pub(crate) fn fem2d_case_value(result: &semio_s_fem_model::StaticResult, nodes: &[String], pairs: &[(String, String)], members: &[String]) -> semio_framework_value::DslValue {
    let displacements = nodes
        .iter()
        .map(|node| {
            let found = result.displacements.iter().find(|entry| &entry.node_id == node);
            let values = found.map_or([0.0; 6], |entry| entry.values);
            semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(values[0]), semio_framework_value::DslValue::float(values[1]), semio_framework_value::DslValue::float(values[5])])
        })
        .collect();
    let reactions = pairs
        .iter()
        .map(|(node, dof)| {
            let value = result.reactions.iter().find(|entry| &entry.node_id == node && fem2d_dof_name_of(entry.dof) == dof.as_str()).map_or(0.0, |entry| entry.value);
            semio_framework_value::DslValue::float(value)
        })
        .collect();
    let elements = members
        .iter()
        .map(|member| {
            let six = match result.elements.iter().find(|(id, _)| id == member).map(|(_, value)| value) {
                Some(semio_s_fem_model::ElementResult::Bar { n }) => [*n, 0.0, 0.0, *n, 0.0, 0.0],
                Some(semio_s_fem_model::ElementResult::Beam { stations }) => match (stations.first(), stations.last()) {
                    (Some(head), Some(tail)) => [head.n, head.v, head.m, tail.n, tail.v, tail.m],
                    _ => [0.0; 6],
                },
                _ => [0.0; 6],
            };
            semio_framework_value::DslValue::Array(six.iter().map(|value| semio_framework_value::DslValue::float(*value)).collect())
        })
        .collect();
    semio_framework_value::DslValue::Object(vec![("displacements".to_string(), semio_framework_value::DslValue::Array(displacements)), ("reactions".to_string(), semio_framework_value::DslValue::Array(reactions)), ("elements".to_string(), semio_framework_value::DslValue::Array(elements))])
}

/// 🔤️ A solver degree of freedom as the wire spells it.
pub(crate) fn fem2d_dof_name_of(dof: semio_s_fem_model::Dof) -> &'static str {
    match dof {
        semio_s_fem_model::Dof::Tx => "Tx",
        semio_s_fem_model::Dof::Ty => "Ty",
        semio_s_fem_model::Dof::Tz => "Tz",
        semio_s_fem_model::Dof::Rx => "Rx",
        semio_s_fem_model::Dof::Ry => "Ry",
        semio_s_fem_model::Dof::Rz => "Rz",
    }
}

/// 🧮️ One JSON report of the linear-static analysis of `snapshot_json` — the production surface the
/// `🧮️solves-fem2d-1-benchmarks` case's third-party solver oracle is compared against.
///
/// Values are SI and in this artifact's own frame: metres, newtons, newton-metres, radians; `x`
/// right and `y` up; `rz` counter-clockwise positive; a reaction is the force the support applies TO
/// the structure; a member's axial force is tension-positive. Cases come first in document order,
/// then combinations. A model whose stiffness matrix is singular reports `error` and no cases at all,
/// which is the only honest answer for a mechanism.
///
/// @see ../../../📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/🥒️.feature
pub fn fem2d_analysis_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (nodes, pairs, members) = fem2d_frame_axes(&doc);
    let axes = |value: semio_framework_value::DslValue| {
        semio_framework_value::DslValue::Object(vec![
            ("nodes".to_string(), semio_framework_value::DslValue::Array(nodes.iter().map(|node| semio_framework_value::DslValue::String(node.clone())).collect())),
            ("reactions".to_string(), semio_framework_value::DslValue::Array(pairs.iter().map(|(node, dof)| semio_framework_value::DslValue::String(format!("{node}.{dof}"))).collect())),
            ("members".to_string(), semio_framework_value::DslValue::Array(members.iter().map(|member| semio_framework_value::DslValue::String(member.clone())).collect())),
            ("cases".to_string(), value),
        ])
    };
    match semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(&doc) {
        Err(error) => Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
        Ok(results) => {
            let mut cases = Vec::new();
            for id in doc.load_cases.iter().map(|case| case.id.clone()).chain(doc.combinations.iter().map(|combination| combination.id.clone())) {
                let Some(result) = results.get(&id) else {
                    return Err(format!("fem2d_solve_all returned no result for {id:?}"));
                };
                let mut entry = vec![("id".to_string(), semio_framework_value::DslValue::String(id.clone()))];
                if let semio_framework_value::DslValue::Object(fields) = fem2d_case_value(result, &nodes, &pairs, &members) {
                    entry.extend(fields);
                }
                cases.push(semio_framework_value::DslValue::Object(entry));
            }
            Ok(semio_framework_pack_json::to_json_string(&axes(semio_framework_value::DslValue::Array(cases))))
        }
    }
}

/// 🧬️ Applies one typed mutation to `base_json` through the SAME production diff/apply path
/// `fem2d_mutation_report_json` uses, then reports the analysis of what it left behind — the
/// mutate-then-solve half of `🧮️solves-fem2d-1-benchmarks`.
pub fn fem2d_mutated_analysis_report_json(base_json: &str, mutation_json: &str) -> Result<String, String> {
    let base: Fem2dSnapshot = semio_framework_pack_json::from_json_str(base_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mutation: Fem2dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, outcome) = store::os_store::replay::apply_outcome(&base, <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(&mutation, &base));
    let faults: Vec<String> = outcome.messages().iter().filter(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).map(|message| format!("{:?}", message.code)).collect();
    if !faults.is_empty() {
        return Err(format!("the mutation was rejected with {faults:?}"));
    }
    fem2d_analysis_report_json(&semio_framework_pack_json::to_json_string(&applied))
}

/// 🎵️ One JSON report of the modal analysis of `snapshot_json` — the natural frequencies in hertz,
/// as many as the document's own `analysis.modalCount` asks for.
pub fn fem2d_modal_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    match semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_modal(&doc) {
        Err(error) => Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
        Ok(result) => {
            Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("frequenciesHz".to_string(), semio_framework_value::DslValue::Array(result.frequencies_hz.iter().take(doc.analysis.modal_count as usize).map(|value| semio_framework_value::DslValue::float(*value)).collect()))])))
        }
    }
}

/// 🏛️ One JSON report of the linear-buckling analysis of `snapshot_json` — the lowest load factor of
/// every load case the document declares, keyed by case id.
pub fn fem2d_buckling_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut factors = Vec::new();
    for case in &doc.load_cases {
        match semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_buckling(&doc, &case.id) {
            Err(error) => return Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
            Ok(result) => match result.factors.first() {
                Some(value) => factors.push((case.id.clone(), semio_framework_value::DslValue::float(*value))),
                None => return Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(format!("no buckling factor for {}", case.id)))]))),
            },
        }
    }
    Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("factors".to_string(), semio_framework_value::DslValue::Object(factors))])))
}
