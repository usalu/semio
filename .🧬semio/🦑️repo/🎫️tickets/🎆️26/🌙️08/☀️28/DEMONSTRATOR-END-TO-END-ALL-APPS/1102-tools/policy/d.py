import re
from mk import R
D=[]
M="🧰️framework/🔨️modules"
def lines(rel,a,b):
    L=open(f"{R}/{rel}",encoding="utf8").read().split("\n");return "\n".join(L[a-1:b])
DS=f"{M}/🔀️dispatch/🦀️.rs"
D.append((DS,[
 ("    let item_trait: ItemTrait = syn::parse2(item)?;\n    let dispatch_macro = build_dispatch_macro(&item_trait);\n    let delegate = if delegation { build_typed_delegation(&item_trait)? } else { TokenStream::new() };\n    Ok(quote! {\n        #item_trait\n",
  "    let item_trait: ItemTrait = syn::parse2(item)?;\n    let dispatch_macro = build_dispatch_macro(&item_trait);\n    let delegate = if delegation { build_typed_delegation(&item_trait)? } else { TokenStream::new() };\n    let item_trait = without_async_fn(item_trait);\n    Ok(quote! {\n        #item_trait\n"),
 ("/// 🎟️ A typed owner can customize lifecycle methods while forwarding the original borrowed contract.\n",
  "/// 🪄️ Declares every `async fn` of the re-emitted trait as `fn .. -> impl Future<Output = T>` (and wraps a default body in `async move`), so the trait never trips `async_fn_in_trait`; implementors may still write `async fn`, and `Send` stays structural (R3).\nfn without_async_fn(mut item: ItemTrait) -> ItemTrait {\n    for entry in &mut item.items {\n        let TraitItem::Fn(method) = entry else { continue };\n        if method.sig.asyncness.take().is_none() {\n            continue;\n        }\n        let value: Type = match &method.sig.output {\n            syn::ReturnType::Default => syn::parse_quote!(()),\n            syn::ReturnType::Type(_, ty) => (**ty).clone(),\n        };\n        method.sig.output = syn::parse_quote!(-> impl ::core::future::Future<Output = #value>);\n        if let Some(block) = method.default.take() {\n            method.default = Some(syn::parse_quote!({ async move #block }));\n        }\n    }\n    item\n}\n\n/// 🎟️ A typed owner can customize lifecycle methods while forwarding the original borrowed contract.\n"),
 ("        let (asyncness, output) = match classify_return(method_name, sig)? {\n            ReturnShape::Plain => (sig.asyncness.map(|token| quote!{#token}), quote!{#declared_output}),\n            ReturnShape::FutureOutput(ty) => (Some(quote!{async}), quote!{-> #ty}),\n        };\n        let await_result = asyncness.as_ref().map(|_|quote!{.await});\n        defaults.push(quote!{\n            #(#attributes)*\n            #asyncness fn #method_name #generics (#inputs) #output #where_clause {\n                #name::#method_name(#receiver #(, #args)*) #await_result\n            }\n        });\n",
  "        let shape = classify_return(method_name, sig)?;\n        let (asyncness, output) = match &shape {\n            ReturnShape::Plain => (sig.asyncness.map(|token| quote!{#token}), quote!{#declared_output}),\n            ReturnShape::FutureOutput(ty) => (Some(quote!{async}), quote!{-> #ty}),\n        };\n        let await_result = asyncness.as_ref().map(|_|quote!{.await});\n        let value = match (&shape, &sig.output) {\n            (ReturnShape::FutureOutput(ty), _) => Some(quote!{#ty}),\n            (ReturnShape::Plain, _) if sig.asyncness.is_none() => None,\n            (ReturnShape::Plain, syn::ReturnType::Default) => Some(quote!{()}),\n            (ReturnShape::Plain, syn::ReturnType::Type(_, ty)) => Some(quote!{#ty}),\n        };\n        defaults.push(match value {\n            Some(value) => quote!{\n                #(#attributes)*\n                fn #method_name #generics (#inputs) -> impl ::core::future::Future<Output = #value> #where_clause {\n                    async move { #name::#method_name(#receiver #(, #args)*).await }\n                }\n            },\n            None => quote!{\n                #(#attributes)*\n                #asyncness fn #method_name #generics (#inputs) #output #where_clause {\n                    #name::#method_name(#receiver #(, #args)*) #await_result\n                }\n            },\n        });\n"),
 ("        #[allow(async_fn_in_trait)]\n        #visibility trait #delegate","        #visibility trait #delegate")],
 "dispatch macro: re-emitted and delegate traits declare `fn .. -> impl Future<Output=T>` instead of `async fn`; allow removed; 4 edits"))
PK=f"{M}/🔀️dispatch/📦️packages/🦀️rust/🦀️.rs"
D.append((PK,[(lines(PK,3,12)+"\n","//! `#[dyn_enum]` re-emits every `async fn` of a trait as `fn .. -> impl Future<Output = T>`, so no\n//! consuming crate needs `#![allow(async_fn_in_trait)]`; `Send` stays structural (R3), never a `+ Send` bound.\n")],"dispatch packages: crate-root allow and its R7 doc removed"))
for t in ("🧩️mixed-receivers","🔮️send-future","✍️mutable-receiver","🕳️uninhabited","📈️scale"):
    rel=f"{M}/🔀️dispatch/🧪️tests/{t}/🦀️.rs"
    L=open(f"{R}/{rel}",encoding="utf8").read().split("\n")
    ln=[l for l in L if l.startswith("#![allow(async_fn_in_trait)]")][0]
    D.append((rel,[(ln+"\n","")],"dispatch test crate root allow removed (test file)"))
rel=f"{M}/🔀️dispatch/🧪️tests/🎟️delegation/🦀️.rs"
D.append((rel,[("#![allow(async_fn_in_trait)]\n","")],"dispatch delegation test allow removed (test file)"))
A1=f"{M}/🎭️actor/🎟️retained-turn/📦️packages/🦀️rust/🦀️.rs"
D.append((A1,[("#![allow(async_fn_in_trait)]\n","")],"actor retained-turn: superseded allow (its only trait is sync)"))
A2=f"{M}/🎭️actor/📦️packages/🦀️rust/🦀️.rs"
D.append((A2,[(lines(A2,9,14)+"\n","")],"actor packages: allow and R3/R7 comment removed"))
SH=f"{M}/🎭️actor/🚚️shard-transport-and-kernel-types/🦀️.rs"
D.append((SH,[("    async fn send(&self, bytes: &[u8]);\n    async fn recv(&self) -> Option<Vec<u8>>;\n    async fn heartbeat(&self) -> u64;\n    async fn kill(&self);\n",
 "    fn send(&self, bytes: &[u8]) -> impl ::core::future::Future<Output = ()>;\n    fn recv(&self) -> impl ::core::future::Future<Output = Option<Vec<u8>>>;\n    fn heartbeat(&self) -> impl ::core::future::Future<Output = u64>;\n    fn kill(&self) -> impl ::core::future::Future<Output = ()>;\n")],"actor ShardTransport: RPITIT shape instead of async fn"))
D.append((f"{M}/🌱️value/✨️derive/🛬️receive/🔀️enum/🦀️.rs",[("let Data::Enum(data)=&input.data else{unreachable!()};",'let Data::Enum(data)=&input.data else{return Err(syn::Error::new_spanned(&input.ident,"Receive derive supports enums only"))};')],"syn::Error (fn returns syn::Result<TokenStream>); 1 site"))
