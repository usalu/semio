
    /// 🎞️ Appends the exact declared intrinsic field to the caller's owned operation sink.
    pub fn encode_wire_value_into(value:&DslValue,options:&PackEncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
        crate::os_pack::record::encode_value_record_body_into(VALUE_BRIDGE_FIELD_ID,value,options,output,control)
    }

    /// 🎞️ Decodes the exact declared intrinsic field by borrowing the complete caller source.
    pub fn decode_wire_value_span(bytes:crate::os_pack::ByteSpan<'_>,options:&PackDecodeOptions,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
        crate::os_pack::record::decode_value_record_body_span_exact_controlled(bytes,VALUE_BRIDGE_FIELD_ID,options,control)
    }
