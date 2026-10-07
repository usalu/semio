/// 🧬️ Produces static field identities and finite lazy record edges from the actual authored plan.
fn borrowed_record_codegen(fields:&Fields,owner:&syn::Ident)->Vec<proc_macro2::TokenStream>{
    let plans=plan_fields(fields);
    plans.iter().map(|plan|{
        let FieldPlan{id,key,positional,optional,kind,elem_ty,block,unit,angle,refs,defines,lang,lang_from,coord,dir,..}=plan;
        let refinement=if let Some(symbol)=unit{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Quantity(::semio_framework_dsl_record::borrowed_unit(#symbol))})}else if let Some(symbol)=angle{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Angle(::semio_framework_dsl_record::borrowed_unit(#symbol))})}else if let Some(kind)=refs{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Ref(#kind)})}else if let Some(from)=lang_from{let key=plans.iter().find(|plan|plan.ident==from.as_str()).map_or_else(||to_kebab(from),|plan|plan.key.clone());Some(quote!{::semio_framework_dsl_record::BorrowedShape::EmbedFrom(#key)})}else if let Some(lang)=lang{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Embed(#lang)})}else if *coord{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Coord(3)})}else if *dir{Some(quote!{::semio_framework_dsl_record::BorrowedShape::Dir})}else{None};
        let elem_ty=borrowed_owner_type(elem_ty,owner);
        let shape=match kind{
            FieldKind::Scalar|FieldKind::OptionScalar(_)=>refinement.unwrap_or_else(||quote!{<#elem_ty as ::semio_framework_dsl_record::BorrowedDslField>::SHAPE}),
            FieldKind::VecList(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::List(::semio_framework_dsl_record::borrowed_field_shape::<#inner>)}},
            FieldKind::VecTuple(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Tuple(::semio_framework_dsl_record::borrowed_field_shape::<#inner>,None)}},
            FieldKind::VecStatements(inner)|FieldKind::OptionStatements(inner)|FieldKind::RequiredStatements(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Statements(<#inner as ::semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS)}},
            FieldKind::VecBlockStatements(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Block(||::semio_framework_dsl_record::BorrowedShape::Statements(<#inner as ::semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS))}},
            FieldKind::MapField(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Map(::semio_framework_dsl_record::borrowed_field_shape::<#inner>)}},
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::BorrowedShape::Bytes64},
            FieldKind::VecTable(inner)=>{let inner=borrowed_owner_type(inner,owner);quote!{::semio_framework_dsl_record::BorrowedShape::Table(::semio_framework_dsl_record::borrowed_record::<#inner>)}},
        };
        let shape=if *block{quote!{::semio_framework_dsl_record::BorrowedShape::Block(||#shape)}}else{shape};
        let position=match positional{Some(position)=>quote!{Some(#position as u8)},None=>quote!{None}};
        let defines=match defines{Some(defines)=>quote!{Some(#defines)},None=>quote!{None}};
        quote!{::semio_framework_dsl_record::BorrowedFieldSpec{id:#id,key:#key,position:#position,shape:#shape,optional:#optional,flatten:false,defines:#defines,is_call_name:false}}
    }).collect()
}

/// 🔁️ Resolves an authored Self edge to its concrete type in a uniquely named static table.
fn borrowed_owner_type(ty:&Type,owner:&syn::Ident)->proc_macro2::TokenStream{
    fn resolve(tokens:proc_macro2::TokenStream,owner:&syn::Ident)->proc_macro2::TokenStream{tokens.into_iter().map(|token|match token{proc_macro2::TokenTree::Ident(ident)if ident=="Self"=>proc_macro2::TokenTree::Ident(owner.clone()),proc_macro2::TokenTree::Group(group)=>{let mut next=proc_macro2::Group::new(group.delimiter(),resolve(group.stream(),owner));next.set_span(group.span());proc_macro2::TokenTree::Group(next)},other=>other}).collect()}
    resolve(quote!{#ty},owner)
}

/// 📑️ Adds only the explicit static metadata roles owned by this record composition.
fn borrowed_record_owner_codegen(input:&DeriveInput,field_role:bool)->proc_macro2::TokenStream{
    let name=&input.ident;
    let Data::Struct(data)=&input.data else{return syn::Error::new_spanned(input,"borrowed record metadata requires a struct").to_compile_error()};
    let container=parse_container_attrs(input);
    let keyword=match container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let layout=if container.lines_layout{quote!{::semio_framework_dsl_record::RecordLayout::Lines}}else{quote!{::semio_framework_dsl_record::RecordLayout::Inline}};
    let fields=borrowed_record_codegen(&data.fields,name);
    let count=fields.len();let table=quote::format_ident!("__DSL_BORROWED_{}_FIELDS",name);
    let field=if field_role{quote!{impl ::semio_framework_dsl_record::BorrowedDslField for #name{const SHAPE : ::semio_framework_dsl_record::BorrowedShape=::semio_framework_dsl_record::BorrowedShape::Record(::semio_framework_dsl_record::borrowed_record::<Self>);}}}else{quote!{}};
    quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;#count]=[#(#fields),*];impl ::semio_framework_dsl_record::BorrowedDslRecord for #name{const RECORD : ::semio_framework_dsl_record::BorrowedRecordSpec=::semio_framework_dsl_record::BorrowedRecordSpec{keyword:#keyword,layout:#layout,fields:&#table};}#field}
}

/// 🏷️ Compiles literal scalar tags into static enum metadata without owned label copies.
fn borrowed_scalar_owner_codegen(input:&DeriveInput)->proc_macro2::TokenStream{
    let name=&input.ident;
    let Data::Enum(data)=&input.data else{return syn::Error::new_spanned(input,"borrowed scalar metadata requires an enum").to_compile_error()};
    let mut labels=Vec::new();
    for(index,variant)in data.variants.iter().enumerate(){if !matches!(variant.fields,Fields::Unit){return syn::Error::new_spanned(variant,"borrowed scalar metadata requires unit variants").to_compile_error()}let ordinal=index as u32;let attrs=parse_field_attrs(&variant.attrs);let label=attrs.key.unwrap_or_else(||to_kebab(&variant.ident.to_string()));labels.push(quote!{(#label,#ordinal)});}
    let count=labels.len();let table=quote::format_ident!("__DSL_BORROWED_{}_ENUM",name);
    quote!{static #table:[(&'static str,u32);#count]=[#(#labels),*];impl ::semio_framework_dsl_record::BorrowedDslField for #name{const SHAPE : ::semio_framework_dsl_record::BorrowedShape=::semio_framework_dsl_record::BorrowedShape::Enum(&#table);}}
}

/// 🌿️ Preserves literal tagged identities and delegates newtypes to their actual static record owner.
fn borrowed_variants_owner_codegen(name:&syn::Ident,data:&syn::DataEnum)->proc_macro2::TokenStream{
    let mut methods=Vec::new();let mut entries=Vec::new();let mut identities=Vec::new();let mut tables=Vec::new();
    for(index,variant)in data.variants.iter().enumerate(){
        let tag=&variant.ident;let attrs=parse_field_attrs(&variant.attrs);let keyword=attrs.key.unwrap_or_else(||to_kebab(&tag.to_string()));let make=quote::format_ident!("__dsl_borrowed_variant_{}",index);let spec=quote::format_ident!("__DSL_BORROWED_VARIANT_SPEC_{}",index);
        let(body,pattern)=match &variant.fields{
            Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let inner=borrowed_owner_type(&fields.unnamed[0].ty,name);(quote!{<#inner as ::semio_framework_dsl_record::BorrowedDslRecord>::RECORD},quote!{Self::#tag(_)})},
            Fields::Named(_)=>{let fields=borrowed_record_codegen(&variant.fields,name);let count=fields.len();let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANT_{}_FIELDS",name,index);tables.push(quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;#count]=[#(#fields),*];});(quote!{::semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some(#keyword),layout : ::semio_framework_dsl_record::RecordLayout::Inline,fields:&#table}},quote!{Self::#tag{..}})},
            Fields::Unit=>{let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANT_{}_FIELDS",name,index);tables.push(quote!{static #table:[::semio_framework_dsl_record::BorrowedFieldSpec;0]=[];});(quote!{::semio_framework_dsl_record::BorrowedRecordSpec{keyword:Some(#keyword),layout : ::semio_framework_dsl_record::RecordLayout::Inline,fields:&#table}},quote!{Self::#tag})},
            _=>return syn::Error::new_spanned(variant,"borrowed tagged metadata requires named, unit, or one record newtype variant").to_compile_error(),
        };
        methods.push(quote!{const #spec : ::semio_framework_dsl_record::BorrowedRecordSpec=#body;fn #make()->::semio_framework_dsl_record::BorrowedRecordSpec{Self::#spec}});entries.push(quote!{(#keyword,#name::#make as fn()->::semio_framework_dsl_record::BorrowedRecordSpec)});identities.push(quote!{#pattern=>(#keyword,#index,Self::#make())});
    }
    let count=entries.len();let table=quote::format_ident!("__DSL_BORROWED_{}_VARIANTS",name);
    quote!{#(#tables)*impl #name{#(#methods)*}static #table:[(&'static str,fn()->::semio_framework_dsl_record::BorrowedRecordSpec);#count]=[#(#entries),*];impl ::semio_framework_dsl_record::BorrowedDslVariants for #name{const VARIANTS:&'static[(&'static str,fn()->::semio_framework_dsl_record::BorrowedRecordSpec)]=&#table;fn projected_borrowed_variant_identity(&self)->(&'static str,usize,::semio_framework_dsl_record::BorrowedRecordSpec){match self{#(#identities),*}}}}
}
