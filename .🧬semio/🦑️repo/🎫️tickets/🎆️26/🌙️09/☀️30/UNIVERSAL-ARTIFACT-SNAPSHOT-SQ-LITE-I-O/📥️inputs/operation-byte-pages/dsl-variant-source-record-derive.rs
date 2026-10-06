        let projected=retained_record_codegen(fields,true);let projected_key=retained_record_key_codegen(fields,true);
        variant_identity_arms.push(quote!{#match_pattern=>(#keyword,#variant_index,Self::#producer_name())});
        variant_projection_arms.push(quote!{#match_pattern=>{#projected}});
        variant_key_arms.push(quote!{#match_pattern=>{#projected_key}});
