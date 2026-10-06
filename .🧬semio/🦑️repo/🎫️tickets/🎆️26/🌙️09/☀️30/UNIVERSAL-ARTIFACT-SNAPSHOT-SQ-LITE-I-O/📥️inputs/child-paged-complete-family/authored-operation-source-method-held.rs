impl TestMutation{
    /// 🎞️ Emits this actual roster's canonical operation directly into its caller's retained paged source.
    pub(crate) fn encode_op_into(&self,options:&protocol::codec::PackEncodeOptions,output:&mut dyn protocol::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),protocol::ProtocolError>{
        dsl::variants_binary::encode_op_into(self,options,output,control)
    }
}
