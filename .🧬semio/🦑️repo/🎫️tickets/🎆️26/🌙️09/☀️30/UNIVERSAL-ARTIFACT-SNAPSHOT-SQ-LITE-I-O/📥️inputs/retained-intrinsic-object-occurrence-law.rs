#[test]
fn retained_intrinsic_objects_preserve_all_literal_occurrences_and_words() {
    use semio_framework_value::NativeEncodeControl;
    fn same(a: &DslValue, b: &DslValue) -> bool {
        match (a,b) {
            (DslValue::Null,DslValue::Null) => true,
            (DslValue::Bool(a),DslValue::Bool(b)) => a==b,
            (DslValue::Number(Number::UInt(a)),DslValue::Number(Number::UInt(b))) => a==b,
            (DslValue::Number(Number::Int(a)),DslValue::Number(Number::Int(b))) => a==b,
            (DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b))) => a.to_bits()==b.to_bits(),
            (DslValue::String(a),DslValue::String(b)) => a==b,
            (DslValue::Bytes(a),DslValue::Bytes(b)) => a==b,
            (DslValue::Array(a),DslValue::Array(b)) => a.len()==b.len() && a.iter().zip(b).all(|(a,b)|same(a,b)),
            (DslValue::Object(a),DslValue::Object(b)) => a.len()==b.len() && a.iter().zip(b).all(|((ak,av),(bk,bv))|ak==bk && same(av,bv)),
            _ => false,
        }
    }
    for bits in [0u64,0x8000000000000000,0x3ff0000000000000,0x0000000000000001,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000042,0x7ff0000000000001,0xfff8000000000011] {
        let value=DslValue::Object(vec![
            ("z".into(),DslValue::Null),
            ("a".into(),DslValue::Bool(true)),
            ("z".into(),DslValue::Number(Number::UInt(u64::MAX))),
            ("int".into(),DslValue::Number(Number::Int(i64::MIN))),
            ("positiveInt".into(),DslValue::Number(Number::Int(1))),
            ("float".into(),DslValue::Number(Number::Float(f64::from_bits(bits)))),
            ("text".into(),DslValue::String("NUL\0 世界".into())),
            ("bytes".into(),DslValue::Bytes(vec![0,1,127,128,255])),
            ("array".into(),DslValue::Array(vec![DslValue::Array(vec![]),DslValue::Object(vec![])])),
            ("object".into(),DslValue::Object(vec![("z".into(),DslValue::String("first".into())),("a".into(),DslValue::Null),("z".into(),DslValue::String("last".into()))])),
        ]);
        let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"value",Shape::Value)]);
        let source=||RecordValue{fields:[(0,FieldValue::Value(value.clone()))].into_iter().collect()};
        let expected=print(&source(),&spec(),JoinMode::Document);
        let mut accepted=|_|true;
        let controlled=print_controlled(&source(),&spec(),JoinMode::Document,1000000,&mut NativeEncodeControl::new(10000000,&mut accepted)).unwrap();
        assert_eq!(controlled,expected);
        for grant in [1usize,7,usize::MAX] {
            let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Document,1000000);
            let mut accepted=|_|true;
            let mut control=NativeEncodeControl::new(10000000,&mut accepted);
            let mut turns=0;
            let actual=loop {
                let before=writer.progress();
                assert!(writer.step(0,&mut control).unwrap().is_none());
                assert_eq!(writer.progress(),before);
                turns+=1;assert!(turns<100000);
                if let Some(text)=writer.step(grant,&mut control).unwrap(){break text;}
            };
            assert_eq!(actual,expected);
            let parsed=parse_exact(&actual,&spec(),&ParseOptions::default()).unwrap();
            let FieldValue::Value(restored)=&parsed.fields[&0] else {panic!("intrinsic field changed shape")};
            assert!(same(restored,&value),"complete occurrence or IEEE word changed: {bits:016x}");
        }
    }
}
