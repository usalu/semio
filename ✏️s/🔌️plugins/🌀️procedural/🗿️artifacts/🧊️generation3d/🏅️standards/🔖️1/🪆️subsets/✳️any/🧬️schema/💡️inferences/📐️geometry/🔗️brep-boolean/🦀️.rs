//! 🔗️ `brep.boolean` computes: union, difference, intersection and the compound cut as stepped kernel jobs over a fresh session.
//!
//! An operand is a solid or a compound. A compound is the union of its members, so it is fused into one solid first (a chain of boolean jobs of its own); the tools of a compound cut are the members of every tool operand, cut away in one operation. Every kernel job is advanced with the fuel of the grant, so cancelling a widget between two steps drops the session with the job half-way.

use super::brep_curve::{guarded, refusal};
use super::phased_job::{cancelled_fault, Flow, Pipeline, Work};
use super::super::prelude::*;
use semio_framework_3d::brep::engine::{BrepError, BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep, BrepOperation, BrepOperationAdmission, BrepOperationJob, BrepOperationStep, GeometryHandle, GeometryKind};
use semio_framework_3d::brep::operations::boolean::BooleanOp;
use std::collections::VecDeque;
use std::sync::Arc;

//#region 🔖️Plan
/// 🧭️ One kernel job of a boolean, over registers that hold session handles.
enum Task {
    Combine { first: usize, second: usize, op: BooleanOp, into: usize },
    CutAll { target: usize, tools: Vec<usize>, into: usize },
}

enum Outcome {
    Working(f32),
    Ready(Option<GeometryHandle>),
}

enum Running {
    Combine(BrepBooleanJob),
    CutAll(BrepOperationJob),
}

/// 🎯️ What the operands are combined into.
#[derive(Clone, Copy)]
enum Finale {
    Combine(BooleanOp),
    CutAll,
}

/// 🪜️ The operands to import, in order, with the port each came from.
struct Operands {
    ports: Vec<&'static str>,
}

struct Fold {
    operands: Operands,
    finale: Finale,
    registers: Vec<Option<GeometryHandle>>,
    tasks: VecDeque<Task>,
    running: Option<(Running, usize)>,
    planned: bool,
    total: usize,
    result: usize,
}

fn boolean_fault(error: &BrepError) -> WidgetFault {
    match error {
        BrepError::Operation(message) if message.contains("is empty") => refusal("boolean-empty", format!("The operation left no solid ({message})."), format!("Die Operation hat keinen Körper übrig gelassen ({message}).")),
        _ => kernel_fault(error),
    }
}

fn progress(done: usize, total: usize) -> f32 {
    if total == 0 {
        0.0
    } else {
        (done as f32 / total as f32).clamp(0.0, 1.0)
    }
}

impl Fold {
    fn new(ports: Vec<&'static str>, finale: Finale) -> Self {
        Self { operands: Operands { ports }, finale, registers: Vec::new(), tasks: VecDeque::new(), running: None, planned: false, total: 0, result: 0 }
    }

    fn register(&mut self, handle: Option<GeometryHandle>) -> usize {
        self.registers.push(handle);
        self.registers.len() - 1
    }

    fn members(&mut self, work: &mut Work, index: usize) -> Result<Vec<usize>, WidgetFault> {
        let port = self.operands.ports[index];
        let handle = work.handle(index)?;
        let solids = match work.source(index)?.kind() {
            GeometryKind::Solid => vec![handle],
            GeometryKind::Compound => work.session.brep().explode_sync(&handle).map_err(|error| kernel_fault(&error))?,
            _ => return Err(refusal("shape-kind", "This input takes a solid or a compound of solids.", "Dieser Eingang erwartet einen Körper oder eine Gruppe von Körpern.").at(port)),
        };
        if solids.is_empty() {
            return Err(refusal("compound-empty", "The compound holds no solid.", "Die Gruppe enthält keinen Körper.").at(port));
        }
        Ok(solids.into_iter().map(|solid| self.register(Some(solid))).collect())
    }

    fn fused(&mut self, members: &[usize]) -> usize {
        members[1..].iter().fold(members[0], |accumulated, member| {
            let into = self.register(None);
            self.tasks.push_back(Task::Combine { first: accumulated, second: *member, op: BooleanOp::Unite, into });
            into
        })
    }

    fn plan(&mut self, work: &mut Work) -> Result<(), WidgetFault> {
        let mut grouped = Vec::new();
        for index in 0..self.operands.ports.len() {
            grouped.push(self.members(work, index)?);
        }
        let first = self.fused(&grouped[0]);
        let into = self.register(None);
        match self.finale {
            Finale::Combine(op) => {
                let second = self.fused(&grouped[1]);
                self.tasks.push_back(Task::Combine { first, second, op, into });
            }
            Finale::CutAll => {
                let tools = grouped[1..].iter().flatten().copied().collect();
                self.tasks.push_back(Task::CutAll { target: first, tools, into });
            }
        }
        self.total = self.tasks.len();
        self.result = into;
        self.planned = true;
        Ok(())
    }

    fn handle(&self, register: usize) -> Result<GeometryHandle, WidgetFault> {
        self.registers.get(register).and_then(Clone::clone).ok_or_else(|| refusal("pipeline", "The boolean was planned over a value that was never computed.", "Das Boolesche wurde über einen Wert geplant, der nie berechnet wurde."))
    }

    fn admit(&mut self, work: &mut Work, task: Task) -> Result<(), WidgetFault> {
        match task {
            Task::Combine { first, second, op, into } => {
                let (first, second) = (self.handle(first)?, self.handle(second)?);
                match work.session.brep().boolean_job_sync(&first, &second, op).map_err(|error| boolean_fault(&error))? {
                    BrepBooleanAdmission::Answered(handle) => self.registers[into] = Some(handle),
                    BrepBooleanAdmission::Job(job) => self.running = Some((Running::Combine(job), into)),
                }
            }
            Task::CutAll { target, tools, into } => {
                let tools = tools.into_iter().map(|tool| self.handle(tool)).collect::<Result<Vec<_>, _>>()?;
                let operation = BrepOperation::CompoundCut { target: self.handle(target)?, tools };
                match work.session.brep().operation_job_sync(operation).map_err(|error| boolean_fault(&error))? {
                    BrepOperationAdmission::Answered(handles) => self.registers[into] = handles.into_iter().next(),
                    BrepOperationAdmission::Job(job) => self.running = Some((Running::CutAll(job), into)),
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self, work: &mut Work, fuel: usize) -> Result<Flow, WidgetFault> {
        if !self.planned {
            self.plan(work)?;
        }
        loop {
            if self.running.is_none() {
                let Some(task) = self.tasks.pop_front() else {
                    work.groups = vec![vec![self.handle(self.result)?]];
                    return Ok(Flow::Next);
                };
                self.admit(work, task)?;
                continue;
            }
            let remaining = self.tasks.len();
            let finished = self.total.saturating_sub(remaining + 1);
            let Some((running, into)) = self.running.as_mut() else { continue };
            let into = *into;
            let outcome = match running {
                Running::Combine(job) => match work.session.brep().step_boolean_job_sync(job, fuel).map_err(|error| boolean_fault(&error))? {
                    BrepBooleanStep::Working(step) => Outcome::Working(progress(step.units_done, step.units_total)),
                    BrepBooleanStep::Ready(handle) => Outcome::Ready(Some(handle)),
                    BrepBooleanStep::Cancelled(_) => return Err(cancelled_fault()),
                },
                Running::CutAll(job) => match work.session.brep().step_operation_job_sync(job, fuel).map_err(|error| boolean_fault(&error))? {
                    BrepOperationStep::Working(step) => Outcome::Working(progress(step.done, step.total)),
                    BrepOperationStep::Ready(handles) => Outcome::Ready(handles.into_iter().next()),
                    BrepOperationStep::Cancelled(_) => return Err(cancelled_fault()),
                },
            };
            match outcome {
                Outcome::Working(inner) => return Ok(Flow::Working((finished as f32 + inner) / self.total.max(1) as f32)),
                Outcome::Ready(None) => return Err(refusal("boolean-empty", "The operation left no solid.", "Die Operation hat keinen Körper übrig gelassen.")),
                Outcome::Ready(Some(handle)) => {
                    self.registers[into] = Some(handle);
                    self.running = None;
                    if !self.tasks.is_empty() {
                        return Ok(Flow::Working((finished + 1) as f32 / self.total.max(1) as f32));
                    }
                }
            }
        }
    }
}
//#endregion 🔖️Plan

//#region 🔖️Computes
fn combine(kind: &Kind, inputs: &WidgetInputs, op: BooleanOp) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let (a, b) = (inputs.shape("a")?.clone(), inputs.shape("b")?.clone());
        let mut fold = Fold::new(vec!["a", "b"], Finale::Combine(op));
        Ok(Pipeline::new(kind).import(&a).import(&b).step(move |work, fuel| fold.advance(work, fuel)).exported("shape"))
    })
}

fn fuse(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    combine(kind, &inputs, BooleanOp::Unite)
}

fn cut(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    combine(kind, &inputs, BooleanOp::Cut)
}

fn intersect(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    combine(kind, &inputs, BooleanOp::Intersect)
}

fn compound_cut(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let target = inputs.shape("target")?.clone();
        let tools: Vec<Arc<_>> = inputs.shapes("tools")?.into_iter().cloned().collect();
        if tools.is_empty() {
            return Err(refusal("too-few-tools", "A compound cut needs at least one tool.", "Ein Gruppenschnitt braucht mindestens ein Werkzeug.").at("tools"));
        }
        let mut ports = vec!["target"];
        ports.extend(tools.iter().map(|_| "tools"));
        let mut fold = Fold::new(ports, Finale::CutAll);
        let pipeline = tools.iter().fold(Pipeline::new(kind).import(&target), |pipeline, tool| pipeline.import(tool));
        Ok(pipeline.step(move |work, fuel| fold.advance(work, fuel)).exported("shape"))
    })
}

/// 🗃️ The `brep.boolean` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.boolean.fuse", start: fuse },
    ComputeEntry { id: "brep.boolean.cut", start: cut },
    ComputeEntry { id: "brep.boolean.intersect", start: intersect },
    ComputeEntry { id: "brep.boolean.compoundCut", start: compound_cut },
];
//#endregion 🔖️Computes

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
