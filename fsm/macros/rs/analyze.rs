//! 🔬 Syntax AST → semantic IR — name resolution, dense id assignment, validation.

use crate::parse::{AlwaysAst, EventVariantAst, FinalAst, MachineAst, OnAst, ParallelAst, StateAst, StateItemAst};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::{Ident, Type};

//#region 🔖Ir

pub enum NodeKindIr {
    Atomic,
    Compound,
    Parallel,
    Final,
    HistoryShallow,
    HistoryDeep,
}

pub struct NodeIr {
    pub name: String,
    pub kind: NodeKindIr,
    pub parent: Option<usize>,
    pub initial: Option<usize>,
    pub children: Vec<usize>,
    pub entry_actions: Vec<usize>,
    pub exit_actions: Vec<usize>,
    pub invokes: Vec<usize>,
    pub timers: Vec<(usize, u64)>,
}

pub enum TriggerIr {
    Event(usize),
    Eventless,
    Done(usize),
    Timer(usize),
}

pub struct TransitionIr {
    pub source: usize,
    pub trigger: TriggerIr,
    pub guard: Option<usize>,
    pub targets: Vec<usize>,
    pub internal: bool,
    pub actions: Vec<usize>,
}

pub struct Ir {
    pub machine_name: Ident,
    pub nodes: Vec<NodeIr>,
    pub transitions: Vec<TransitionIr>,
    pub event_name: Ident,
    pub event_variants: Vec<EventVariantAst>,
    pub guards: Vec<Ident>,
    pub actions: Vec<Ident>,
    pub context_ty: Type,
    pub input_ty: Type,
    pub output_ty: Type,
    pub effect_ty: Type,
    pub context_from_input: Ident,
    pub make_output: Option<Ident>,
    pub fingerprint: u64,
    pub manifest_json: String,
}

//#endregion 🔖Ir

//#region 🔖Analyzer

struct PendingTransition {
    source: usize,
    trigger: PendingTrigger,
    guard: Option<usize>,
    target_name: String,
    target_span: Span,
    internal: bool,
    action: Option<usize>,
}

enum PendingTrigger {
    Event(Ident),
    Eventless,
    Done,
    Timer(usize),
}

struct Analyzer {
    nodes: Vec<NodeIr>,
    names: HashMap<String, usize>,
    guards: Vec<Ident>,
    actions: Vec<Ident>,
    outputs: Vec<Ident>,
    pending: Vec<PendingTransition>,
    invoke_counter: usize,
    timer_counter: usize,
}

impl Analyzer {
    fn intern_guard(&mut self, ident: Ident) -> usize {
        if let Some(pos) = self.guards.iter().position(|g| g == &ident) {
            pos
        } else {
            self.guards.push(ident);
            self.guards.len() - 1
        }
    }

    fn intern_action(&mut self, ident: Ident) -> usize {
        if let Some(pos) = self.actions.iter().position(|a| a == &ident) {
            pos
        } else {
            self.actions.push(ident);
            self.actions.len() - 1
        }
    }

    fn add_node(&mut self, name: &Ident, kind: NodeKindIr, parent: usize) -> syn::Result<usize> {
        let key = name.to_string();
        if self.names.contains_key(&key) {
            return Err(syn::Error::new(name.span(), format!("duplicate state name `{key}`")));
        }
        let idx = self.nodes.len();
        self.nodes.push(NodeIr {
            name: key.clone(),
            kind,
            parent: Some(parent),
            initial: None,
            children: Vec::new(),
            entry_actions: Vec::new(),
            exit_actions: Vec::new(),
            invokes: Vec::new(),
            timers: Vec::new(),
        });
        self.names.insert(key, idx);
        self.nodes[parent].children.push(idx);
        Ok(idx)
    }

    fn build_state(&mut self, ast: StateAst, parent: usize) -> syn::Result<usize> {
        let name = ast.name.clone().expect("nested/top-level states always carry a name");
        let kind = if ast.children.is_empty() { NodeKindIr::Atomic } else { NodeKindIr::Compound };
        let idx = self.add_node(&name, kind, parent)?;

        for entry in &ast.entry {
            let action_idx = self.intern_action(entry.clone());
            self.nodes[idx].entry_actions.push(action_idx);
        }
        for exit in &ast.exit {
            let action_idx = self.intern_action(exit.clone());
            self.nodes[idx].exit_actions.push(action_idx);
        }
        for _invoke in &ast.invokes {
            let invoke_id = self.invoke_counter;
            self.invoke_counter += 1;
            self.nodes[idx].invokes.push(invoke_id);
        }
        for after in &ast.afters {
            let timer_id = self.timer_counter;
            self.timer_counter += 1;
            self.nodes[idx].timers.push((timer_id, after.delay_ms));
            let action = after.action.clone().map(|a| self.intern_action(a));
            self.pending.push(PendingTransition {
                source: idx,
                trigger: PendingTrigger::Timer(timer_id),
                guard: None,
                target_name: after.target.to_string(),
                target_span: after.target.span(),
                internal: false,
                action,
            });
        }
        self.push_on_transitions(idx, &ast.transitions);
        self.push_always_transitions(idx, &ast.always);

        let history_idx = if let Some(h) = &ast.history {
            let kind = if h.deep { NodeKindIr::HistoryDeep } else { NodeKindIr::HistoryShallow };
            Some(self.add_node(&h.name, kind, idx)?)
        } else {
            None
        };

        for child in ast.children {
            match child {
                StateItemAst::State(s) => {
                    self.build_state(s, idx)?;
                }
                StateItemAst::Parallel(p) => {
                    self.build_parallel(p, idx)?;
                }
                StateItemAst::Final(f) => {
                    self.build_final(f, idx)?;
                }
            }
        }

        if matches!(self.nodes[idx].kind, NodeKindIr::Compound) {
            let initial_name = ast
                .initial
                .clone()
                .ok_or_else(|| syn::Error::new(name.span(), format!("compound state `{name}` needs `initial: <child>;`")))?;
            let initial_idx = *self
                .names
                .get(&initial_name.to_string())
                .ok_or_else(|| syn::Error::new(initial_name.span(), format!("unknown initial state `{initial_name}`")))?;
            self.nodes[idx].initial = Some(initial_idx);
        }
        if let Some(h_idx) = history_idx {
            self.nodes[h_idx].initial = self.nodes[idx].initial;
        }

        Ok(idx)
    }

    fn build_parallel(&mut self, ast: ParallelAst, parent: usize) -> syn::Result<usize> {
        if ast.regions.len() < 2 {
            return Err(syn::Error::new(ast.name.span(), format!("parallel state `{}` needs at least two `state` regions", ast.name)));
        }
        let idx = self.add_node(&ast.name, NodeKindIr::Parallel, parent)?;
        for region in ast.regions {
            self.build_state(region, idx)?;
        }
        if let Some((target, action)) = ast.on_done {
            let action_idx = action.map(|a| self.intern_action(a));
            self.pending.push(PendingTransition {
                source: idx,
                trigger: PendingTrigger::Done,
                guard: None,
                target_name: target.to_string(),
                target_span: target.span(),
                internal: false,
                action: action_idx,
            });
        }
        Ok(idx)
    }

    fn build_final(&mut self, ast: FinalAst, parent: usize) -> syn::Result<usize> {
        let idx = self.add_node(&ast.name, NodeKindIr::Final, parent)?;
        if let Some(output_fn) = ast.output {
            self.outputs.push(output_fn);
        }
        Ok(idx)
    }

    fn push_on_transitions(&mut self, source: usize, transitions: &[OnAst]) {
        for on in transitions {
            let guard = on.guard.clone().map(|g| self.intern_guard(g));
            let action = on.action.clone().map(|a| self.intern_action(a));
            self.pending.push(PendingTransition {
                source,
                trigger: PendingTrigger::Event(on.event.clone()),
                guard,
                target_name: on.target.to_string(),
                target_span: on.target.span(),
                internal: on.internal,
                action,
            });
        }
    }

    fn push_always_transitions(&mut self, source: usize, always: &[AlwaysAst]) {
        for al in always {
            let guard = al.guard.clone().map(|g| self.intern_guard(g));
            let action = al.action.clone().map(|a| self.intern_action(a));
            self.pending.push(PendingTransition {
                source,
                trigger: PendingTrigger::Eventless,
                guard,
                target_name: al.target.to_string(),
                target_span: al.target.span(),
                internal: false,
                action,
            });
        }
    }
}

//#endregion 🔖Analyzer

//#region 🔖Fingerprint

fn kind_tag(kind: &NodeKindIr) -> u8 {
    match kind {
        NodeKindIr::Atomic => 0,
        NodeKindIr::Compound => 1,
        NodeKindIr::Parallel => 2,
        NodeKindIr::Final => 3,
        NodeKindIr::HistoryShallow => 4,
        NodeKindIr::HistoryDeep => 5,
    }
}

fn fnv_feed(state: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *state ^= b as u64;
        *state = state.wrapping_mul(0x100000001b3);
    }
}

/// 🔢 Hand-rolled FNV-1a over the compiled structure — no hashing crate dependency.
fn compute_fingerprint(nodes: &[NodeIr], transitions: &[TransitionIr], events: &[EventVariantAst]) -> u64 {
    let mut state: u64 = 0xcbf29ce484222325;
    for node in nodes {
        fnv_feed(&mut state, node.name.as_bytes());
        fnv_feed(&mut state, &[kind_tag(&node.kind)]);
        fnv_feed(&mut state, &(node.parent.map(|p| p as u64).unwrap_or(u64::MAX)).to_le_bytes());
    }
    for t in transitions {
        fnv_feed(&mut state, &(t.source as u64).to_le_bytes());
        for target in &t.targets {
            fnv_feed(&mut state, &(*target as u64).to_le_bytes());
        }
    }
    for v in events {
        fnv_feed(&mut state, v.name().to_string().as_bytes());
    }
    state
}

fn build_manifest_json(name: &Ident, nodes: &[NodeIr], transitions: &[TransitionIr], events: &[EventVariantAst]) -> String {
    let mut json = format!("{{\"id\":\"{name}\",\"states\":[");
    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        let parent = node.parent.map(|p| p.to_string()).unwrap_or_else(|| "null".to_string());
        json.push_str(&format!("{{\"id\":\"{}\",\"parent\":{}}}", node.name, parent));
    }
    json.push_str("],\"events\":[");
    for (i, event) in events.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!("\"{}\"", event.name()));
    }
    json.push_str("],\"transitionCount\":");
    json.push_str(&transitions.len().to_string());
    json.push('}');
    json
}

//#endregion 🔖Fingerprint

//#region 🔖Entry

pub fn analyze(ast: MachineAst) -> syn::Result<Ir> {
    let mut az = Analyzer {
        nodes: Vec::new(),
        names: HashMap::new(),
        guards: Vec::new(),
        actions: Vec::new(),
        outputs: Vec::new(),
        pending: Vec::new(),
        invoke_counter: 0,
        timer_counter: 0,
    };
    az.nodes.push(NodeIr {
        name: "root".to_string(),
        kind: NodeKindIr::Compound,
        parent: None,
        initial: None,
        children: Vec::new(),
        entry_actions: Vec::new(),
        exit_actions: Vec::new(),
        invokes: Vec::new(),
        timers: Vec::new(),
    });
    az.names.insert("root".to_string(), 0);

    for child in ast.children {
        match child {
            StateItemAst::State(s) => {
                az.build_state(s, 0)?;
            }
            StateItemAst::Parallel(p) => {
                az.build_parallel(p, 0)?;
            }
            StateItemAst::Final(f) => {
                az.build_final(f, 0)?;
            }
        }
    }

    let initial_idx = *az
        .names
        .get(&ast.initial.to_string())
        .ok_or_else(|| syn::Error::new(ast.initial.span(), format!("unknown initial state `{}`", ast.initial)))?;
    az.nodes[0].initial = Some(initial_idx);

    let mut transitions = Vec::new();
    for p in az.pending {
        let target_idx = *az
            .names
            .get(&p.target_name)
            .ok_or_else(|| syn::Error::new(p.target_span, format!("unknown transition target `{}`", p.target_name)))?;
        let trigger = match p.trigger {
            PendingTrigger::Event(ident) => {
                let event_idx = ast
                    .event_variants
                    .iter()
                    .position(|v| v.name() == &ident)
                    .ok_or_else(|| syn::Error::new(ident.span(), format!("unknown event variant `{ident}`")))?;
                TriggerIr::Event(event_idx)
            }
            PendingTrigger::Eventless => TriggerIr::Eventless,
            PendingTrigger::Done => TriggerIr::Done(p.source),
            PendingTrigger::Timer(id) => TriggerIr::Timer(id),
        };
        transitions.push(TransitionIr {
            source: p.source,
            trigger,
            guard: p.guard,
            targets: vec![target_idx],
            internal: p.internal,
            actions: p.action.into_iter().collect(),
        });
    }

    let make_output = match (ast.output_from_context, az.outputs.len()) {
        (Some(ident), 0) => Some(ident),
        (None, 0) => None,
        (None, 1) => Some(az.outputs[0].clone()),
        (None, _) => {
            return Err(syn::Error::new(
                ast.name.span(),
                "at most one `final` state may declare `output:`; use machine-level `output_from_context:` for more complex cases",
            ))
        }
        (Some(_), _) => {
            return Err(syn::Error::new(
                ast.name.span(),
                "declare output via either machine-level `output_from_context:` or a single `final { output: .. }`, not both",
            ))
        }
    };

    let fingerprint = compute_fingerprint(&az.nodes, &transitions, &ast.event_variants);
    let manifest_json = build_manifest_json(&ast.name, &az.nodes, &transitions, &ast.event_variants);

    Ok(Ir {
        machine_name: ast.name,
        nodes: az.nodes,
        transitions,
        event_name: ast.event_name,
        event_variants: ast.event_variants,
        guards: az.guards,
        actions: az.actions,
        context_ty: ast.context_ty,
        input_ty: ast.input_ty,
        output_ty: ast.output_ty,
        effect_ty: ast.effect_ty,
        context_from_input: ast.context_from_input,
        make_output,
        fingerprint,
        manifest_json,
    })
}

//#endregion 🔖Entry
