    #[test]
    fn child_complete_group_source_candidate_refuses_partial_last_locale_before_commit(){
        use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress};
        use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
        let fixture:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/@PARTIAL_LABEL_FIXTURE@"))).unwrap();
        assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["partialBytes"],256);assert_eq!(fixture["completeCells"],3);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
        for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
        let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();
        let mut axes=Vec::new();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{axes.push((terminology,locale));}}
        for(terminology,locale)in &axes[..3]{receipt.read_label_from_source(*terminology,*locale,&"first",&mut control).unwrap();}
        let baseline=control.owned_bytes();let continuation=control.pause().unwrap();let mut cancel=|progress:NativeEncodeProgress|progress.owned_bytes<=baseline||progress.completed<256;let mut control=NativeEncodeControl::resume(continuation,&mut cancel).unwrap();let large="x".repeat(8194);assert!(receipt.read_label_from_source(axes[3].0,axes[3].1,&large.as_str(),&mut control).is_err());assert!(control.owned_bytes()>baseline);
        let continuation=control.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();assert!(receipt.commit(&mut control).is_err(),"partial last locale was published despite lacking complete text authority");
        assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain_retained(&mut owner),allocated);
        eprintln!("[DEBUG] three complete locale cells and fourth partial256/8194 retain their real owners; commit refuses and original schema/prefix remain unchanged; all physical owners drain1/4096");
    }
