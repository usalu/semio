    struct EmissionProbe<'a>{source:&'a mut OwnedOperationBytes,entered:&'a std::cell::Cell<bool>,length:usize}
    impl crate::os_spr::operation_bytes::OperationByteOutput for EmissionProbe<'_>{
        fn write_bytes(&mut self,bytes:&[u8],control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),crate::os_pack::PackRefusal>{
            if bytes.len()==self.length{self.entered.set(true);}
            crate::os_spr::operation_bytes::OperationByteOutput::write_bytes(self.source,bytes,control)
        }
    }
    let entered=std::cell::Cell::new(false);
    let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress| !(entered.get()&&progress.total==payload.len()&&progress.completed>=256);
    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut cancel);
    let error={let mut probe=EmissionProbe{source:&mut source,entered:&entered,length:payload.len()};variants_binary::encode_op_into(&operation,&Default::default(),&mut probe,&mut control).unwrap_err()};
