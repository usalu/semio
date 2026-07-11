//! 🪄 Proc macros compiling `statechart!` declarations into `fsm` static tables.
//!
//! Entry points delegate to inner `proc_macro2`-typed functions so the
//! parse → analyze → codegen pipeline is unit-testable without `trybuild`.
//!
//! State/region/history names must be unique across the *whole* machine (no dotted
//! relative/absolute target paths) — a deliberate v1 simplification that keeps name
//! resolution a flat map instead of a scoped path resolver.

mod analyze;
mod codegen;
mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

//#region 🔖Entry

/// 🪄 Compiles a `machine <name> { .. }` declaration into a `pub mod <name> { .. }`
/// with a dense static [`fsm::MachineDefinition`], a generated event enum, and a
/// `states` module of `NodeId` consts.
#[proc_macro]
pub fn statechart(input: TokenStream) -> TokenStream {
    expand_statechart(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn expand_statechart(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ast: parse::MachineAst = syn::parse2(input)?;
    let ir = analyze::analyze(ast)?;
    Ok(codegen::emit(&ir))
}

/// 🪄 Implements `fsm::StatechartEvent` for a consumer-authored enum, for use with
/// hand-authored `MachineDefinition` tables (i.e. without the `statechart!` DSL).
#[proc_macro_derive(StatechartEvent)]
pub fn derive_statechart_event(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    expand_statechart_event(derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn expand_statechart_event(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Enum(e) => e,
        _ => return Err(syn::Error::new_spanned(&input, "StatechartEvent can only be derived for enums")),
    };
    let count = data.variants.len() as u16;
    let mut id_arms = Vec::new();
    let mut name_arms = Vec::new();
    for (i, variant) in data.variants.iter().enumerate() {
        let lit = i as u16;
        let vname = &variant.ident;
        let pattern = match &variant.fields {
            syn::Fields::Named(_) => quote! { #name::#vname { .. } },
            syn::Fields::Unnamed(_) => quote! { #name::#vname(..) },
            syn::Fields::Unit => quote! { #name::#vname },
        };
        id_arms.push(quote! { #pattern => fsm::EventId(#lit) });
        let vname_str = vname.to_string();
        name_arms.push(quote! { #lit => #vname_str });
    }
    Ok(quote! {
        impl fsm::StatechartEvent for #name {
            const EVENT_COUNT: u16 = #count;

            fn event_id(&self) -> fsm::EventId {
                match self {
                    #(#id_arms),*
                }
            }

            fn event_name(id: fsm::EventId) -> &'static str {
                match id.0 {
                    #(#name_arms,)*
                    _ => "?",
                }
            }
        }
    })
}

/// 🪄 Implements `fsm::StatechartSchema` for a consumer-authored context struct —
/// embeds a field name/type manifest consumed by TypeScript generation tooling.
#[proc_macro_derive(StatechartSchema)]
pub fn derive_statechart_schema(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    expand_statechart_schema(derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn expand_statechart_schema(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(s) => s,
        _ => return Err(syn::Error::new_spanned(&input, "StatechartSchema can only be derived for structs")),
    };
    let mut fields_json = Vec::new();
    if let syn::Fields::Named(named) = &data.fields {
        for field in &named.named {
            let fname = field.ident.as_ref().expect("named field").to_string();
            let ty = &field.ty;
            let fty = quote! { #ty }.to_string().replace('"', "\\\"");
            fields_json.push(format!("{{\"name\":\"{fname}\",\"type\":\"{fty}\"}}"));
        }
    }
    let json = format!("{{\"fields\":[{}]}}", fields_json.join(","));
    Ok(quote! {
        impl fsm::StatechartSchema for #name {
            const SCHEMA_JSON: &'static str = #json;
        }
    })
}

//#endregion 🔖Entry

//#region 🧪Tests

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn statechart_expands_minimal_machine_to_valid_rust() {
        let input = quote! {
            machine toggle {
                context: ToggleContext;
                event ToggleEvent { Flip }
                input: ();
                output: ();
                effect: ();
                context_from_input: build_context;
                initial: off;
                state off {
                    on Flip => on;
                }
                state on {
                    on Flip => off;
                }
            }
        };
        let expanded = expand_statechart(input).expect("expansion should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
    }

    #[test]
    fn statechart_rejects_duplicate_state_names() {
        let input = quote! {
            machine dup {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a { on Go => b; }
                state a { on Go => b; }
            }
        };
        let err = expand_statechart(input).expect_err("duplicate state names must be rejected");
        assert!(err.to_string().contains("duplicate state name"));
    }

    #[test]
    fn statechart_rejects_unknown_initial_state() {
        let input = quote! {
            machine bad_initial {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: nonexistent;
                state a { on Go => a; }
            }
        };
        let err = expand_statechart(input).expect_err("unknown initial state must be rejected");
        assert!(err.to_string().contains("unknown initial state"));
    }

    #[test]
    fn statechart_rejects_unknown_transition_target() {
        let input = quote! {
            machine bad_target {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a { on Go => nowhere; }
            }
        };
        let err = expand_statechart(input).expect_err("unknown transition target must be rejected");
        assert!(err.to_string().contains("unknown transition target"));
    }

    #[test]
    fn statechart_rejects_compound_without_initial() {
        let input = quote! {
            machine bad_compound {
                context: Ctx;
                event Ev { Go }
                input: ();
                output: ();
                effect: ();
                context_from_input: build;
                initial: a;
                state a {
                    state nested { on Go => nested; }
                }
            }
        };
        let err = expand_statechart(input).expect_err("compound state without `initial:` must be rejected");
        assert!(err.to_string().contains("needs `initial:"));
    }

    #[test]
    fn statechart_expands_hierarchical_parallel_history_machine() {
        let input = quote! {
            machine media {
                context: MediaContext;
                event MediaEvent { Open, Pause, Play, Stop, Start, AudioDone, VideoDone }
                input: ();
                output: Receipt;
                effect: ();
                context_from_input: build_media;
                output_from_context: make_receipt;
                initial: closed;
                state closed {
                    on Open => open;
                }
                state open {
                    initial: playing;
                    entry on_open_enter;
                    exit on_open_exit;
                    invoke ticker;
                    after 30000 => closed do on_timeout;
                    history opened_history shallow;
                    state playing {
                        on Pause => paused;
                    }
                    state paused {
                        on Play if can_resume => playing;
                    }
                    on Stop => closed;
                }
                parallel recording {
                    state audio {
                        initial: audio_capturing;
                        state audio_capturing { on AudioDone => audio_done; }
                        final audio_done;
                    }
                    state video {
                        initial: video_capturing;
                        state video_capturing { on VideoDone => video_done; }
                        final video_done;
                    }
                    on_done => closed do on_recording_done;
                }
            }
        };
        let expanded = expand_statechart(input).expect("expansion of a hierarchical/parallel/history machine should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
    }

    #[test]
    fn derive_statechart_event_expands_mixed_variant_kinds() {
        let input: DeriveInput = syn::parse_quote! {
            enum Ev {
                Unit,
                Struct { field: i32 },
                Tuple(i32, i32),
            }
        };
        let expanded = expand_statechart_event(input).expect("derive should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
        let text = expanded.to_string();
        assert!(text.contains("EVENT_COUNT"));
        assert!(text.contains("3u16") || text.contains("3"));
    }

    #[test]
    fn derive_statechart_event_rejects_non_enum() {
        let input: DeriveInput = syn::parse_quote! {
            struct NotAnEnum;
        };
        let err = expand_statechart_event(input).expect_err("deriving on a struct must be rejected");
        assert!(err.to_string().contains("enums"));
    }

    #[test]
    fn derive_statechart_schema_expands_named_fields() {
        let input: DeriveInput = syn::parse_quote! {
            struct Ctx {
                order_id: String,
                attempts: u32,
            }
        };
        let expanded = expand_statechart_schema(input).expect("derive should succeed");
        syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
        let text = expanded.to_string();
        assert!(text.contains("SCHEMA_JSON"));
        assert!(text.contains("order_id"));
    }
}

//#endregion 🧪Tests
