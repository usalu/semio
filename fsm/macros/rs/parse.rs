//! ✂️ Token stream → syntax AST for the `statechart!` DSL. Spanned errors only.

use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, LitInt, Token, Type};

//#region 🔖Keywords

mod kw {
    syn::custom_keyword!(machine);
    syn::custom_keyword!(context);
    syn::custom_keyword!(event);
    syn::custom_keyword!(input);
    syn::custom_keyword!(output);
    syn::custom_keyword!(effect);
    syn::custom_keyword!(initial);
    syn::custom_keyword!(state);
    syn::custom_keyword!(parallel);
    syn::custom_keyword!(history);
    syn::custom_keyword!(shallow);
    syn::custom_keyword!(deep);
    syn::custom_keyword!(on);
    syn::custom_keyword!(on_done);
    syn::custom_keyword!(after);
    syn::custom_keyword!(always);
    syn::custom_keyword!(invoke);
    syn::custom_keyword!(entry);
    syn::custom_keyword!(exit);
    syn::custom_keyword!(internal);
    syn::custom_keyword!(context_from_input);
    syn::custom_keyword!(output_from_context);
}

//#endregion 🔖Keywords

//#region 🔖Ast

pub struct MachineAst {
    pub name: Ident,
    pub context_ty: Type,
    pub event_name: Ident,
    pub event_variants: Vec<EventVariantAst>,
    pub input_ty: Type,
    pub output_ty: Type,
    pub effect_ty: Type,
    pub context_from_input: Ident,
    pub output_from_context: Option<Ident>,
    pub initial: Ident,
    pub children: Vec<StateItemAst>,
}

pub enum EventVariantAst {
    Unit(Ident),
    Struct(Ident, Vec<(Ident, Type)>),
    Tuple(Ident, Vec<Type>),
}

impl EventVariantAst {
    pub fn name(&self) -> &Ident {
        match self {
            EventVariantAst::Unit(name) | EventVariantAst::Struct(name, _) | EventVariantAst::Tuple(name, _) => name,
        }
    }
}

pub enum StateItemAst {
    State(StateAst),
    Parallel(ParallelAst),
    Final(FinalAst),
}

#[derive(Default)]
pub struct StateAst {
    pub name: Option<Ident>,
    pub initial: Option<Ident>,
    pub entry: Vec<Ident>,
    pub exit: Vec<Ident>,
    pub invokes: Vec<Ident>,
    pub afters: Vec<AfterAst>,
    pub transitions: Vec<OnAst>,
    pub always: Vec<AlwaysAst>,
    pub history: Option<HistoryAst>,
    pub children: Vec<StateItemAst>,
}

pub struct FinalAst {
    pub name: Ident,
    pub output: Option<Ident>,
}

pub struct ParallelAst {
    pub name: Ident,
    pub regions: Vec<StateAst>,
    pub on_done: Option<(Ident, Option<Ident>)>,
}

pub struct AfterAst {
    pub delay_ms: u64,
    pub target: Ident,
    pub action: Option<Ident>,
}

pub struct OnAst {
    pub event: Ident,
    pub guard: Option<Ident>,
    pub target: Ident,
    pub action: Option<Ident>,
    pub internal: bool,
}

pub struct AlwaysAst {
    pub guard: Option<Ident>,
    pub target: Ident,
    pub action: Option<Ident>,
}

pub struct HistoryAst {
    pub name: Ident,
    pub deep: bool,
}

//#endregion 🔖Ast

//#region 🔖Parse

fn parse_typed_item(input: ParseStream) -> syn::Result<Type> {
    input.parse::<Token![:]>()?;
    let ty = input.parse::<Type>()?;
    input.parse::<Token![;]>()?;
    Ok(ty)
}

fn parse_ident_item(input: ParseStream) -> syn::Result<Ident> {
    input.parse::<Token![:]>()?;
    let ident = input.parse::<Ident>()?;
    input.parse::<Token![;]>()?;
    Ok(ident)
}

fn parse_event_block(input: ParseStream) -> syn::Result<(Ident, Vec<EventVariantAst>)> {
    let name = input.parse::<Ident>()?;
    let content;
    braced!(content in input);
    let mut variants = Vec::new();
    while !content.is_empty() {
        let variant_name = content.parse::<Ident>()?;
        if content.peek(syn::token::Brace) {
            let fields_content;
            syn::braced!(fields_content in content);
            let mut fields = Vec::new();
            while !fields_content.is_empty() {
                let field_name = fields_content.parse::<Ident>()?;
                fields_content.parse::<Token![:]>()?;
                let field_ty = fields_content.parse::<Type>()?;
                fields.push((field_name, field_ty));
                if fields_content.peek(Token![,]) {
                    fields_content.parse::<Token![,]>()?;
                }
            }
            variants.push(EventVariantAst::Struct(variant_name, fields));
        } else if content.peek(syn::token::Paren) {
            let tuple_content;
            syn::parenthesized!(tuple_content in content);
            let mut types = Vec::new();
            while !tuple_content.is_empty() {
                types.push(tuple_content.parse::<Type>()?);
                if tuple_content.peek(Token![,]) {
                    tuple_content.parse::<Token![,]>()?;
                }
            }
            variants.push(EventVariantAst::Tuple(variant_name, types));
        } else {
            variants.push(EventVariantAst::Unit(variant_name));
        }
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }
    Ok((name, variants))
}

fn parse_after(input: ParseStream) -> syn::Result<AfterAst> {
    let delay_lit = input.parse::<LitInt>()?;
    let delay_ms: u64 = delay_lit.base10_parse()?;
    input.parse::<Token![=>]>()?;
    let target = input.parse::<Ident>()?;
    let action = if input.peek(Token![do]) {
        input.parse::<Token![do]>()?;
        Some(input.parse::<Ident>()?)
    } else {
        None
    };
    input.parse::<Token![;]>()?;
    Ok(AfterAst { delay_ms, target, action })
}

fn parse_on(input: ParseStream) -> syn::Result<OnAst> {
    let event = input.parse::<Ident>()?;
    let guard = if input.peek(Token![if]) {
        input.parse::<Token![if]>()?;
        Some(input.parse::<Ident>()?)
    } else {
        None
    };
    input.parse::<Token![=>]>()?;
    let target = input.parse::<Ident>()?;
    let action = if input.peek(Token![do]) {
        input.parse::<Token![do]>()?;
        Some(input.parse::<Ident>()?)
    } else {
        None
    };
    let internal = if input.peek(kw::internal) {
        input.parse::<kw::internal>()?;
        true
    } else {
        false
    };
    input.parse::<Token![;]>()?;
    Ok(OnAst { event, guard, target, action, internal })
}

fn parse_always(input: ParseStream) -> syn::Result<AlwaysAst> {
    let guard = if input.peek(Token![if]) {
        input.parse::<Token![if]>()?;
        Some(input.parse::<Ident>()?)
    } else {
        None
    };
    input.parse::<Token![=>]>()?;
    let target = input.parse::<Ident>()?;
    let action = if input.peek(Token![do]) {
        input.parse::<Token![do]>()?;
        Some(input.parse::<Ident>()?)
    } else {
        None
    };
    input.parse::<Token![;]>()?;
    Ok(AlwaysAst { guard, target, action })
}

fn parse_history(input: ParseStream) -> syn::Result<HistoryAst> {
    let name = input.parse::<Ident>()?;
    let deep = if input.peek(kw::deep) {
        input.parse::<kw::deep>()?;
        true
    } else {
        input.parse::<kw::shallow>()?;
        false
    };
    input.parse::<Token![;]>()?;
    Ok(HistoryAst { name, deep })
}

fn parse_state_body(input: ParseStream, name: Option<Ident>) -> syn::Result<StateAst> {
    let content;
    braced!(content in input);
    let mut state = StateAst { name, ..StateAst::default() };
    while !content.is_empty() {
        if content.peek(kw::initial) {
            content.parse::<kw::initial>()?;
            state.initial = Some(parse_ident_item(&content)?);
        } else if content.peek(kw::entry) {
            content.parse::<kw::entry>()?;
            state.entry.push(content.parse::<Ident>()?);
            content.parse::<Token![;]>()?;
        } else if content.peek(kw::exit) {
            content.parse::<kw::exit>()?;
            state.exit.push(content.parse::<Ident>()?);
            content.parse::<Token![;]>()?;
        } else if content.peek(kw::invoke) {
            content.parse::<kw::invoke>()?;
            state.invokes.push(content.parse::<Ident>()?);
            content.parse::<Token![;]>()?;
        } else if content.peek(kw::after) {
            content.parse::<kw::after>()?;
            state.afters.push(parse_after(&content)?);
        } else if content.peek(kw::on) {
            content.parse::<kw::on>()?;
            state.transitions.push(parse_on(&content)?);
        } else if content.peek(kw::always) {
            content.parse::<kw::always>()?;
            state.always.push(parse_always(&content)?);
        } else if content.peek(kw::history) {
            content.parse::<kw::history>()?;
            state.history = Some(parse_history(&content)?);
        } else if content.peek(kw::state) {
            content.parse::<kw::state>()?;
            let child_name = content.parse::<Ident>()?;
            state.children.push(StateItemAst::State(parse_state_body(&content, Some(child_name))?));
        } else if content.peek(kw::parallel) {
            content.parse::<kw::parallel>()?;
            state.children.push(StateItemAst::Parallel(parse_parallel(&content)?));
        } else if content.peek(Token![final]) {
            content.parse::<Token![final]>()?;
            state.children.push(StateItemAst::Final(parse_final(&content)?));
        } else {
            return Err(content.error("expected initial/entry/exit/invoke/after/on/always/history/state/parallel/final"));
        }
    }
    Ok(state)
}

fn parse_final(input: ParseStream) -> syn::Result<FinalAst> {
    let name = input.parse::<Ident>()?;
    if input.peek(Token![;]) {
        input.parse::<Token![;]>()?;
        return Ok(FinalAst { name, output: None });
    }
    let content;
    braced!(content in input);
    let mut output = None;
    while !content.is_empty() {
        if content.peek(kw::output) {
            content.parse::<kw::output>()?;
            output = Some(parse_ident_item(&content)?);
        } else {
            return Err(content.error("expected `output` inside a `final` block"));
        }
    }
    Ok(FinalAst { name, output })
}

fn parse_parallel(input: ParseStream) -> syn::Result<ParallelAst> {
    let name = input.parse::<Ident>()?;
    let content;
    braced!(content in input);
    let mut regions = Vec::new();
    let mut on_done = None;
    while !content.is_empty() {
        if content.peek(kw::state) {
            content.parse::<kw::state>()?;
            let region_name = content.parse::<Ident>()?;
            regions.push(parse_state_body(&content, Some(region_name))?);
        } else if content.peek(kw::on_done) {
            content.parse::<kw::on_done>()?;
            content.parse::<Token![=>]>()?;
            let target = content.parse::<Ident>()?;
            let action = if content.peek(Token![do]) {
                content.parse::<Token![do]>()?;
                Some(content.parse::<Ident>()?)
            } else {
                None
            };
            content.parse::<Token![;]>()?;
            on_done = Some((target, action));
        } else {
            return Err(content.error("expected `state` region or `on_done` inside a `parallel` block"));
        }
    }
    Ok(ParallelAst { name, regions, on_done })
}

impl Parse for MachineAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::machine>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);

        let mut context_ty = None;
        let mut event_decl = None;
        let mut input_ty = None;
        let mut output_ty = None;
        let mut effect_ty = None;
        let mut context_from_input = None;
        let mut output_from_context = None;
        let mut initial = None;
        let mut children = Vec::new();

        while !content.is_empty() {
            if content.peek(kw::context_from_input) {
                content.parse::<kw::context_from_input>()?;
                context_from_input = Some(parse_ident_item(&content)?);
            } else if content.peek(kw::context) {
                content.parse::<kw::context>()?;
                context_ty = Some(parse_typed_item(&content)?);
            } else if content.peek(kw::event) {
                content.parse::<kw::event>()?;
                event_decl = Some(parse_event_block(&content)?);
            } else if content.peek(kw::input) {
                content.parse::<kw::input>()?;
                input_ty = Some(parse_typed_item(&content)?);
            } else if content.peek(kw::output_from_context) {
                content.parse::<kw::output_from_context>()?;
                output_from_context = Some(parse_ident_item(&content)?);
            } else if content.peek(kw::output) {
                content.parse::<kw::output>()?;
                output_ty = Some(parse_typed_item(&content)?);
            } else if content.peek(kw::effect) {
                content.parse::<kw::effect>()?;
                effect_ty = Some(parse_typed_item(&content)?);
            } else if content.peek(kw::initial) {
                content.parse::<kw::initial>()?;
                initial = Some(parse_ident_item(&content)?);
            } else if content.peek(kw::state) {
                content.parse::<kw::state>()?;
                let child_name = content.parse::<Ident>()?;
                children.push(StateItemAst::State(parse_state_body(&content, Some(child_name))?));
            } else if content.peek(kw::parallel) {
                content.parse::<kw::parallel>()?;
                children.push(StateItemAst::Parallel(parse_parallel(&content)?));
            } else if content.peek(Token![final]) {
                content.parse::<Token![final]>()?;
                children.push(StateItemAst::Final(parse_final(&content)?));
            } else {
                return Err(content.error(
                    "expected context/event/input/output/effect/context_from_input/output_from_context/initial/state/parallel/final",
                ));
            }
        }

        let (event_name, event_variants) = event_decl.ok_or_else(|| input.error("machine must declare `event <Name> { .. }`"))?;

        Ok(MachineAst {
            name,
            context_ty: context_ty.ok_or_else(|| input.error("machine must declare `context: Type;`"))?,
            event_name,
            event_variants,
            input_ty: input_ty.ok_or_else(|| input.error("machine must declare `input: Type;`"))?,
            output_ty: output_ty.ok_or_else(|| input.error("machine must declare `output: Type;`"))?,
            effect_ty: effect_ty.ok_or_else(|| input.error("machine must declare `effect: Type;`"))?,
            context_from_input: context_from_input.ok_or_else(|| input.error("machine must declare `context_from_input: ident;`"))?,
            output_from_context,
            initial: initial.ok_or_else(|| input.error("machine must declare `initial: ident;`"))?,
            children,
        })
    }
}

//#endregion 🔖Parse
