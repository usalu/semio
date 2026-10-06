    /// 🛂️ Captures the complete original caller limits before any source read or destination allocation.
    pub fn new_with_limits(source:&'source S,policy:super::JsonMemberPolicy,limits:JsonReadLimits)->Result<Self,semio_framework_value::ValueError>{
        if source.byte_len()as u128>u128::from(limits.maximum_bytes){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON complete source exceeds caller byte limit"));}
        let mut parser=super::JsonGrammarCursor::new(policy);parser.limits=limits;Ok(Self{source,parser})
    }
