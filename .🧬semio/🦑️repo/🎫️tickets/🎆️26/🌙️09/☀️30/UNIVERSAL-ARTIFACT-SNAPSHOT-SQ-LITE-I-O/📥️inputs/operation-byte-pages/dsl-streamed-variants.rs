    fn encode_with_into<T:DslVariants>(op:&T,tag_of:impl Fn(&str,usize)->Result<u64,ProtocolError>,options:&EncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        let(keyword,record)=op.to_named_record_controlled(control).map_err(crate::os_pack::PackRefusal::from)?;
        let variants=T::variants_controlled(control).map_err(crate::os_pack::PackRefusal::from)?;
        let ordinal=variants.iter().position(|(key,_)|key==&keyword).ok_or_else(||ProtocolError::Malformed{what:"op variant",offset:0,detail:format!("keyword '{keyword}' missing from variants()")})?;
        let tag=tag_of(&keyword,ordinal)?;
        let spec=variants[ordinal].1.encode(control).map_err(crate::os_pack::PackRefusal::from)?;
        output.write_bytes(&[OP_BINARY_FORMAT],control)?;
        let mut remaining=tag;
        let mut bytes=[0;10];
        let mut length=0;
        loop{bytes[length]=(remaining as u8)&127;remaining>>=7;if remaining!=0{bytes[length]|=128;}length+=1;if remaining==0{break;}}
        output.write_bytes(&bytes[..length],control)?;
        crate::os_pack::record::encode_record_body_into(&spec,&record,options,output,control)?;
        Ok(())
    }

    /// 🏷️ Appends a declared tagged operation into the same admitted source prefix.
    pub fn encode_tagged_op_into<T:DslVariants>(protocol:&str,op:&T,options:&EncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        encode_with_into(op,|keyword,_|protocol_record::records(protocol).find(|(kind,_)|*kind==keyword).map(|(_,tag)|tag).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("📡️.protocol.semio declares no record for '{keyword}'")}),options,output,control)
    }

    /// 🎞️ Appends the exact ordinal protocol header and direct canonical Record body.
    pub fn encode_op_into<T:DslVariants>(op:&T,options:&EncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
        encode_with_into(op,|_,ordinal|u64::try_from(ordinal).map_err(|_|ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} exceeds the u64 wire range")}),options,output,control)
    }
