//! 🏗️ Semantic IR → Rust code — dense static tables, a `Machine` impl, and a `states` module.

use crate::analyze::{Ir, NodeIr, NodeKindIr, TransitionIr, TriggerIr};
use crate::parse::EventVariantAst;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

//#region 🔖Helpers

fn to_pascal_case(input: &str) -> String {
    input
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn node_id_lit(index: usize) -> TokenStream {
    let lit = index as u16;
    quote! { fsm::NodeId(#lit) }
}

fn node_kind_tokens(kind: &NodeKindIr) -> TokenStream {
    match kind {
        NodeKindIr::Atomic => quote! { fsm::NodeKind::Atomic },
        NodeKindIr::Compound => quote! { fsm::NodeKind::Compound },
        NodeKindIr::Parallel => quote! { fsm::NodeKind::Parallel },
        NodeKindIr::Final => quote! { fsm::NodeKind::Final },
        NodeKindIr::HistoryShallow => quote! { fsm::NodeKind::HistoryShallow },
        NodeKindIr::HistoryDeep => quote! { fsm::NodeKind::HistoryDeep },
    }
}

fn trigger_tokens(trigger: &TriggerIr) -> TokenStream {
    match trigger {
        TriggerIr::Event(idx) => {
            let lit = *idx as u16;
            quote! { fsm::Trigger::Event(fsm::EventId(#lit)) }
        }
        TriggerIr::Eventless => quote! { fsm::Trigger::Eventless },
        TriggerIr::Done(node_idx) => {
            let target = node_id_lit(*node_idx);
            quote! { fsm::Trigger::Done(#target) }
        }
        TriggerIr::Timer(id) => {
            let lit = *id as u16;
            quote! { fsm::Trigger::Timer(fsm::TimerId(#lit)) }
        }
    }
}

//#endregion 🔖Helpers

//#region 🔖NodeDef

fn node_def_tokens(node: &NodeIr, doc_index: usize) -> TokenStream {
    let stable_id = &node.name;
    let kind = node_kind_tokens(&node.kind);
    let parent = match node.parent {
        Some(p) => {
            let id = node_id_lit(p);
            quote! { Some(#id) }
        }
        None => quote! { None },
    };
    let initial = match node.initial {
        Some(i) => {
            let id = node_id_lit(i);
            quote! { Some(#id) }
        }
        None => quote! { None },
    };
    let children = node.children.iter().map(|c| node_id_lit(*c));
    let entry_actions = node.entry_actions.iter().map(|a| {
        let lit = *a as u16;
        quote! { fsm::ActionId(#lit) }
    });
    let exit_actions = node.exit_actions.iter().map(|a| {
        let lit = *a as u16;
        quote! { fsm::ActionId(#lit) }
    });
    let invokes = node.invokes.iter().map(|i| {
        let lit = *i as u16;
        quote! { fsm::InvokeId(#lit) }
    });
    let timers = node.timers.iter().map(|(id, ms)| {
        let idl = *id as u16;
        let msl = *ms;
        quote! { (fsm::TimerId(#idl), #msl) }
    });
    let doc_index = doc_index as u16;
    quote! {
        fsm::NodeDef {
            stable_id: #stable_id,
            kind: #kind,
            parent: #parent,
            initial: #initial,
            children: &[ #(#children),* ],
            entry_actions: &[ #(#entry_actions),* ],
            exit_actions: &[ #(#exit_actions),* ],
            invokes: &[ #(#invokes),* ],
            timers: &[ #(#timers),* ],
            doc_index: #doc_index,
        }
    }
}

//#endregion 🔖NodeDef

//#region 🔖TransitionDef

fn transition_def_tokens(transition: &TransitionIr, doc_index: usize) -> TokenStream {
    let source = node_id_lit(transition.source);
    let trigger = trigger_tokens(&transition.trigger);
    let guard = match transition.guard {
        Some(g) => {
            let lit = g as u16;
            quote! { Some(fsm::GuardId(#lit)) }
        }
        None => quote! { None },
    };
    let targets = transition.targets.iter().map(|t| node_id_lit(*t));
    let kind = if transition.internal {
        quote! { fsm::TransitionKind::Internal }
    } else {
        quote! { fsm::TransitionKind::External }
    };
    let actions = transition.actions.iter().map(|a| {
        let lit = *a as u16;
        quote! { fsm::ActionId(#lit) }
    });
    let doc_index = doc_index as u16;
    quote! {
        fsm::TransitionDef {
            source: #source,
            trigger: #trigger,
            guard: #guard,
            targets: &[ #(#targets),* ],
            kind: #kind,
            actions: &[ #(#actions),* ],
            doc_index: #doc_index,
        }
    }
}

//#endregion 🔖TransitionDef

//#region 🔖Event

fn event_variant_def_tokens(variant: &EventVariantAst) -> TokenStream {
    match variant {
        EventVariantAst::Unit(name) => quote! { #name },
        EventVariantAst::Struct(name, fields) => {
            let field_defs = fields.iter().map(|(fname, fty)| quote! { #fname: #fty });
            quote! { #name { #(#field_defs),* } }
        }
        EventVariantAst::Tuple(name, types) => quote! { #name(#(#types),*) },
    }
}

fn event_id_arm_tokens(event_name: &syn::Ident, index: usize, variant: &EventVariantAst) -> TokenStream {
    let lit = index as u16;
    match variant {
        EventVariantAst::Unit(name) => quote! { #event_name::#name => fsm::EventId(#lit) },
        EventVariantAst::Struct(name, _) => quote! { #event_name::#name { .. } => fsm::EventId(#lit) },
        EventVariantAst::Tuple(name, types) => {
            let underscores = types.iter().map(|_| quote! { _ });
            quote! { #event_name::#name(#(#underscores),*) => fsm::EventId(#lit) }
        }
    }
}

//#endregion 🔖Event

//#region 🔖Emit

pub fn emit(ir: &Ir) -> TokenStream {
    let mod_name = &ir.machine_name;
    let marker_name = format_ident!("{}", to_pascal_case(&ir.machine_name.to_string()));
    let event_name = &ir.event_name;
    let context_ty = &ir.context_ty;
    let input_ty = &ir.input_ty;
    let output_ty = &ir.output_ty;
    let effect_ty = &ir.effect_ty;
    let context_from_input = &ir.context_from_input;
    let machine_name_str = ir.machine_name.to_string();
    let manifest_json = &ir.manifest_json;
    let fingerprint = ir.fingerprint;

    let word_count = ir.nodes.len().div_ceil(64).max(1);

    let event_variant_defs = ir.event_variants.iter().map(event_variant_def_tokens);
    let event_id_arms = ir.event_variants.iter().enumerate().map(|(i, v)| event_id_arm_tokens(event_name, i, v));
    let event_name_arms = ir.event_variants.iter().enumerate().map(|(i, v)| {
        let idx = i as u16;
        let name_str = v.name().to_string();
        quote! { #idx => #name_str }
    });
    let event_count = ir.event_variants.len() as u16;

    let node_defs = ir.nodes.iter().enumerate().map(|(i, n)| node_def_tokens(n, i));
    let transition_defs = ir.transitions.iter().enumerate().map(|(i, t)| transition_def_tokens(t, i));

    let guard_idents = &ir.guards;
    let action_idents = &ir.actions;

    let make_output = match &ir.make_output {
        Some(ident) => quote! { Some(super::#ident) },
        None => quote! { None },
    };

    let state_consts = ir.nodes.iter().enumerate().map(|(i, n)| {
        let const_name = format_ident!("{}", n.name.to_uppercase());
        let id = node_id_lit(i);
        quote! { pub const #const_name: fsm::NodeId = #id; }
    });

    quote! {
        pub mod #mod_name {
            use super::*;

            //#region 🔖Event
            #[derive(Clone, Debug)]
            pub enum #event_name {
                #(#event_variant_defs),*
            }

            impl fsm::StatechartEvent for #event_name {
                const EVENT_COUNT: u16 = #event_count;

                fn event_id(&self) -> fsm::EventId {
                    match self {
                        #(#event_id_arms),*
                    }
                }

                fn event_name(id: fsm::EventId) -> &'static str {
                    match id.0 {
                        #(#event_name_arms,)*
                        _ => "?",
                    }
                }
            }
            //#endregion 🔖Event

            //#region 🔖Machine
            pub struct #marker_name;

            impl fsm::Machine for #marker_name {
                type Context = #context_ty;
                type Event = #event_name;
                type Input = #input_ty;
                type Output = #output_ty;
                type Effect = #effect_ty;
                type Config = fsm::BitSet<#word_count>;

                fn definition() -> &'static fsm::MachineDefinition<Self> {
                    static DEF: fsm::MachineDefinition<#marker_name> = fsm::MachineDefinition {
                        id: #machine_name_str,
                        nodes: &[ #(#node_defs),* ],
                        transitions: &[ #(#transition_defs),* ],
                        context_from_input: super::#context_from_input,
                        make_output: #make_output,
                        guards: &[ #(super::#guard_idents),* ],
                        actions: &[ #(super::#action_idents),* ],
                        fingerprint: #fingerprint,
                        manifest_json: #manifest_json,
                    };
                    &DEF
                }
            }
            //#endregion 🔖Machine

            //#region 🔖States
            pub mod states {
                use super::super::*;
                #(#state_consts)*
            }
            //#endregion 🔖States

            pub type Snapshot = fsm::Snapshot<#marker_name>;
            pub const MANIFEST_JSON: &str = #manifest_json;
        }
    }
}

//#endregion 🔖Emit
