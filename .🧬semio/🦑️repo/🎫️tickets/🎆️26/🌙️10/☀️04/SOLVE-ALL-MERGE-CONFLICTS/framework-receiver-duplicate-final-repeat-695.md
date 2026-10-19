# Final Framework Receiver and Declaration Audit

Source lexical inspection only. Receivers inside quoted generated code are manual-review candidates; this does not claim Rust compilation.

{
  "checked": 1204,
  "receiverCandidates": [
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎚️controlled-output/🦀️.rs",
      "line": 188,
      "function": "controlled_named_output",
      "statement": "let ident=field.ident.as_ref().unwrap();let flag=format_ident!(\"__emit_{index}\");let attrs=parse_field_attrs(&field.attrs)?;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let alias=format_ident!(\"__source_field_{index}\");let access=if source_self{quote!{&self.#ident}}else{quote!{#"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎚️controlled-output/🦀️.rs",
      "line": 205,
      "function": "controlled_to_body",
      "statement": "let field=data.fields.iter().next().ok_or_else(||syn::Error::new_spanned(input,\"transparent output requires field\"))?;let access=if let Some(ident)=&field.ident{quote!{&self.#ident}}else{quote!{&self.0}};controlled_field_encode(&parse_field_attrs(&field.attrs)?,access,c)"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs",
      "line": 19,
      "function": "expand_to_value",
      "statement": "quote! { #value_crate::ToValue::to_value(&self.#ident) }"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs",
      "line": 557,
      "function": "expand_from_value",
      "statement": "quote! { #value_crate::FromValue::edit_value_at_path(&mut self.#ident, path, edit) }"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs",
      "line": 497,
      "function": "to_value_object_entries",
      "statement": "quote! { #path(&self.#ident) }"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🔀️enum/🦀️.rs",
      "line": 20,
      "function": "expand",
      "statement": "let count=input.get().record_len()?;if self.scan==count{let index=self.tag.ok_or_else(#tree::invalid)?;let text=input.get().record_child(index)?.text()?;self.selected=match text.as_str(){#(#wires=>#indices,)*_=>return Err(#tree::invalid())};self.phase=2;}else{let key=input.get().record_key(self.scan"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs",
      "line": 46,
      "function": "expand",
      "statement": "unsafe{::std::ptr::write(::std::ptr::addr_of_mut!((*self.partial.as_mut().ok_or_else(#value::native_decoding::owned::tree::invalid)?.as_mut_ptr()).#id),field)};"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs",
      "line": 170,
      "function": "close_demand_methods",
      "statement": "if !#value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { return #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_body_bytes); }"
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs",
      "line": 220,
      "function": "expand_struct",
      "statement": "if self.draining { return if #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { Ok(#value_crate::RetirementDemand {depth:1,..Default::default()}) } else { #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_copy_bytes) }; "
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs",
      "line": 410,
      "function": "expand_enum",
      "statement": "if self.draining { return if #value_crate::retained_clone::RetainedCloneCursor::terminal_is_empty(&self.#cursor) { Ok(#value_crate::RetirementDemand {depth:1,..Default::default()}) } else { #value_crate::retained_clone::nested_retained_clone_close_demands::<#ty>(&self.#cursor,maximum_copy_bytes) }; "
    },
    {
      "path": "🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs",
      "line": 578,
      "function": "construct_fields",
      "statement": "let owners = values.iter().map(|value| if variant.is_some() { quote!(*self.#value.take().expect(\"retained field owner\")) } else { quote!(self.#value.take().expect(\"retained field owner\")) }).collect::<Vec<_>>();"
    },
    {
      "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🏗️record-field-codegen/🦀️.rs",
      "line": 146,
      "function": "record_codegen",
      "statement": "quote! { ::semio_framework_dsl_record::DslField::to_value(&self.#ident) },"
    },
    {
      "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📥️owned/🦀️.rs",
      "line": 8,
      "function": "fields_cursor",
      "statement": "let demands_active=plans.iter().enumerate().filter(|(_,plan)|!plan.skip).map(|(index,plan)|{let child=quote::format_ident!(\"Field{}\",index);let field=&plan.ident;let ty=&plan.field_ty;quote!(#active::#child(child)=>{if self.fields.value.as_ref().unwrap().#field.is_some(){return Ok(#v::RetirementDema"
    },
    {
      "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🧹️lifecycle-codegen/🦀️.rs",
      "line": 35,
      "function": "encoding_record_codegen",
      "statement": "let source=if bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};"
    },
    {
      "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🧹️lifecycle-codegen/🦀️.rs",
      "line": 88,
      "function": "retained_record_codegen",
      "statement": "let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};"
    },
    {
      "path": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🧹️lifecycle-codegen/🦀️.rs",
      "line": 112,
      "function": "retained_record_key_codegen",
      "statement": "let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧳️retained/🦀️.rs",
      "line": 20,
      "function": "expand",
      "statement": "select.push(quote!(#name::#variant_name(_)=>{*self.state=#state::#variant_name(::std::boxed::Box::new(#ports::new()));}));"
    }
  ],
  "duplicateCandidates": [
    {
      "path": "🧰️framework/🔨️modules/⏱️trace/🧮️memory/🦀️.rs",
      "name": "guest_linear_memory_bytes",
      "line": 213,
      "first": 207
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/⚙️engine/🎟️icon-paint-cache/🦀️.rs",
      "name": "get_or_build",
      "line": 455,
      "first": 366
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/🚫️rejection/🦀️.rs",
      "name": "next_demand",
      "line": 97,
      "first": 92
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🧪️member-identity-and-group-testing-helpers/🦀️.rs",
      "name": "durable_group_test_outcome",
      "line": 67,
      "first": 43
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📜️space-history/🦀️.rs",
      "name": "apply",
      "line": 213,
      "first": 193
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs",
      "name": "process_resident_bytes",
      "line": 670,
      "first": 664
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs",
      "name": "process_resident_bytes",
      "line": 697,
      "first": 664
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta/🧪️tests/🔬️unit/🦀️.rs",
      "name": "apply",
      "line": 70,
      "first": 58
    }
  ]
}

## Manual Classification

All 17 receiver candidates are inside authored quote!/procedural codegen. No invalid actual source receiver was found. Four duplicate groups are immutable HEAD cfg alternatives (trace guest memory, icon cache and native process memory). The remaining four groups are already authored in immutable stash, documented with source hashes in framework-authored-source-issues-repeat-695.md. They were preserved rather than compile-repaired.
