//! 🧪️ Isolated execution of unchanged production Sequence detector bodies over committed language-neutral vectors.
use std::collections::BTreeMap;

type StepParams=BTreeMap<String,String>;
#[derive(Clone,Debug,PartialEq)]
pub struct SequenceStep {pub id:String,pub kind:String,pub params:StepParams,pub x:f64,pub y:f64,pub slot:Option<(String,String)>,pub collapsed:bool}
#[derive(Clone,Debug,PartialEq)]
pub struct SequenceEdge {pub id:String,pub from:String,pub to:String}
#[derive(Clone,Debug,PartialEq)]
pub struct SequenceFixture {pub schema:String,pub steps:Vec<SequenceStep>,pub edges:Vec<SequenceEdge>}

#[derive(Clone,Debug,PartialEq)]
pub struct CreateStep {
    pub step: SequenceStep,
}

#[derive(Clone,Debug,PartialEq)]
pub struct DeleteStep {
    pub id: String,
}

#[derive(Clone,Debug,PartialEq)]
pub struct MoveStep {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone,Debug,PartialEq)]
pub struct EditStepParams {
    pub id: String,
    pub params: StepParams,
}

#[derive(Clone,Debug,PartialEq)]
pub struct ChangeStepCollapsed {
    pub id: String,
    pub collapsed: bool,
}

#[derive(Clone,Debug,PartialEq)]
pub struct ConnectSteps {
    pub id: String,
    pub from: String,
    pub to: String,
}

#[derive(Clone,Debug,PartialEq)]
pub struct DisconnectSteps {
    pub id: String,
}

#[derive(Clone,Debug,PartialEq)]
pub struct DuplicateStep {
    pub source_id: String,
    pub new_id: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone,Debug,PartialEq)]
pub enum SequenceMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    MoveStep(MoveStep),
    EditStepParams(EditStepParams),
    ChangeStepCollapsed(ChangeStepCollapsed),
    ConnectSteps(ConnectSteps),
    DisconnectSteps(DisconnectSteps),
    DuplicateStep(DuplicateStep),
}

/// 🗂️ Indexed before/after scenes shared by independent leaf detection contributions.
pub struct SequenceDetectionContext<'a> {
    pub before: &'a SequenceFixture,
    pub after: &'a SequenceFixture,
    pub before_steps: BTreeMap<&'a str, &'a SequenceStep>,
    pub after_steps: BTreeMap<&'a str, &'a SequenceStep>,
    pub before_edges: BTreeMap<&'a str, &'a SequenceEdge>,
    pub after_edges: BTreeMap<&'a str, &'a SequenceEdge>,
}

/// 🔢️ A leaf-owned mutation with its stable cross-contribution application order.
pub struct SequenceDetectedMutation {
    pub order: (u8, usize, u8),
    pub mutation: SequenceMutation,
}

pub type SequenceMutationDetector = for<'a> fn(&SequenceDetectionContext<'a>) -> Vec<SequenceDetectedMutation>;

/// 🔀️ Assembles ordered leaf-owned detection contributions without concrete mutation branches.
pub fn sequence_snapshot_mutations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceMutation> {
    let context = SequenceDetectionContext {
        before,
        after,
        before_steps: before.steps.iter().rev().map(|step| (step.id.as_str(), step)).collect(),
        after_steps: after.steps.iter().map(|step| (step.id.as_str(), step)).collect(),
        before_edges: before.edges.iter().rev().map(|edge| (edge.id.as_str(), edge)).collect(),
        after_edges: after.edges.iter().map(|edge| (edge.id.as_str(), edge)).collect(),
    };
    let mut detected: Vec<_> = DETECTORS.iter().flat_map(|detect| detect(&context)).collect();
    detected.sort_by_key(|entry| entry.order);
    detected.into_iter().map(|entry| entry.mutation).collect()
}

const DETECTORS:&[SequenceMutationDetector]=&[create_step::detect,delete_step::detect,move_step::detect,edit_step_params::detect,change_step_collapsed::detect,connect_steps::detect,disconnect_steps::detect];

mod create_step {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .steps
        .iter()
        .enumerate()
        .filter(|(_, step)| !context.before_steps.contains_key(step.id.as_str()))
        .map(|(index, step)| SequenceDetectedMutation { order: (1, index, 0), mutation: SequenceMutation::CreateStep(CreateStep { step: step.clone() }) })
        .collect()
}
}

mod delete_step {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .before
        .steps
        .iter()
        .enumerate()
        .filter(|(_, step)| !context.after_steps.contains_key(step.id.as_str()))
        .map(|(index, step)| SequenceDetectedMutation { order: (0, index, 0), mutation: SequenceMutation::DeleteStep(DeleteStep { id: step.id.clone() }) })
        .collect()
}
}

mod move_step {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let before = context.before_steps.get(step.id.as_str())?;
            (before.x != step.x || before.y != step.y).then(|| SequenceDetectedMutation { order: (1, index, 0), mutation: SequenceMutation::MoveStep(MoveStep { id: step.id.clone(), x: step.x, y: step.y }) })
        })
        .collect()
}
}

mod edit_step_params {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let before = context.before_steps.get(step.id.as_str())?;
            (before.params != step.params).then(|| SequenceDetectedMutation { order: (1, index, 1), mutation: SequenceMutation::EditStepParams(EditStepParams { id: step.id.clone(), params: step.params.clone() }) })
        })
        .collect()
}
}

mod change_step_collapsed {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let before = context.before_steps.get(step.id.as_str())?;
            (before.collapsed != step.collapsed).then(|| SequenceDetectedMutation { order: (1, index, 2), mutation: SequenceMutation::ChangeStepCollapsed(ChangeStepCollapsed { id: step.id.clone(), collapsed: step.collapsed }) })
        })
        .collect()
}
}

mod connect_steps {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    context
        .after
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| context.before_edges.get(edge.id.as_str()).map_or(true, |before| before.from != edge.from || before.to != edge.to))
        .map(|(index, edge)| SequenceDetectedMutation { order: (3, index, 1), mutation: SequenceMutation::ConnectSteps(ConnectSteps { id: edge.id.clone(), from: edge.from.clone(), to: edge.to.clone() }) })
        .collect()
}
}

mod disconnect_steps {
use super::*;
pub fn detect(context: &SequenceDetectionContext<'_>) -> Vec<SequenceDetectedMutation> {
    let removed = context
        .before
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| !context.after_edges.contains_key(edge.id.as_str()) && ![edge.from.as_str(), edge.to.as_str()].iter().any(|id| context.before_steps.contains_key(id) && !context.after_steps.contains_key(id)))
        .map(|(index, edge)| SequenceDetectedMutation { order: (2, index, 0), mutation: SequenceMutation::DisconnectSteps(DisconnectSteps { id: edge.id.clone() }) });
    let retargeted = context
        .after
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| context.before_edges.get(edge.id.as_str()).is_some_and(|before| before.from != edge.from || before.to != edge.to))
        .map(|(index, edge)| SequenceDetectedMutation { order: (3, index, 0), mutation: SequenceMutation::DisconnectSteps(DisconnectSteps { id: edge.id.clone() }) });
    removed.chain(retargeted).collect()
}
}

fn main(){
let before=SequenceFixture { schema:"sequence.fixture".into(),steps:vec![SequenceStep { id:"a".into(), kind:"log.print".into(), params:BTreeMap::from([("message".into(),"\"old\"".into())]), x:0.0,y:0.0,slot:None,collapsed:false },SequenceStep { id:"b".into(), kind:"log.print".into(), params:BTreeMap::from([]), x:10.0,y:0.0,slot:None,collapsed:false },SequenceStep { id:"c".into(), kind:"log.print".into(), params:BTreeMap::from([]), x:20.0,y:0.0,slot:None,collapsed:false }],edges:vec![SequenceEdge {id:"cascade".into(),from:"c".into(),to:"a".into()},SequenceEdge {id:"removed".into(),from:"a".into(),to:"b".into()},SequenceEdge {id:"retargeted".into(),from:"b".into(),to:"a".into()}] };
let after=SequenceFixture { schema:"sequence.fixture".into(),steps:vec![SequenceStep { id:"a".into(), kind:"log.print".into(), params:BTreeMap::from([("message".into(),"\"new\"".into())]), x:1.0,y:2.0,slot:None,collapsed:true },SequenceStep { id:"b".into(), kind:"log.print".into(), params:BTreeMap::from([]), x:11.0,y:0.0,slot:None,collapsed:false },SequenceStep { id:"d".into(), kind:"log.print".into(), params:BTreeMap::from([]), x:30.0,y:0.0,slot:None,collapsed:false }],edges:vec![SequenceEdge {id:"retargeted".into(),from:"b".into(),to:"d".into()},SequenceEdge {id:"created".into(),from:"a".into(),to:"d".into()}] };
let expected=vec![SequenceMutation::DeleteStep(DeleteStep {id:"c".into()}),SequenceMutation::MoveStep(MoveStep {id:"a".into(),x:1.0,y:2.0}),SequenceMutation::EditStepParams(EditStepParams {id:"a".into(),params:BTreeMap::from([("message".into(),"\"new\"".into())])}),SequenceMutation::ChangeStepCollapsed(ChangeStepCollapsed {id:"a".into(),collapsed:true}),SequenceMutation::MoveStep(MoveStep {id:"b".into(),x:11.0,y:0.0}),SequenceMutation::CreateStep(CreateStep {step:SequenceStep { id:"d".into(), kind:"log.print".into(), params:BTreeMap::from([]), x:30.0,y:0.0,slot:None,collapsed:false }}),SequenceMutation::DisconnectSteps(DisconnectSteps {id:"removed".into()}),SequenceMutation::DisconnectSteps(DisconnectSteps {id:"retargeted".into()}),SequenceMutation::ConnectSteps(ConnectSteps {id:"retargeted".into(),from:"b".into(),to:"d".into()}),SequenceMutation::ConnectSteps(ConnectSteps {id:"created".into(),from:"a".into(),to:"d".into()})];
assert_eq!(sequence_snapshot_mutations(&before,&after),expected,"ordered-step-edits-and-cascade-aware-edge-changes");
let before=SequenceFixture { schema:"sequence.fixture".into(),steps:vec![],edges:vec![] };
let after=SequenceFixture { schema:"sequence.fixture".into(),steps:vec![],edges:vec![] };
let expected=vec![];
assert_eq!(sequence_snapshot_mutations(&before,&after),expected,"no-change-is-an-empty-plan");
}
