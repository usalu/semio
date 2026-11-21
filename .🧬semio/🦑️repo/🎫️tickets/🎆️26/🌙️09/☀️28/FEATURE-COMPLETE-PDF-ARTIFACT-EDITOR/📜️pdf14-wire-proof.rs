#[cfg(test)]
mod proof {
    use super::*;
    #[test]
    fn authored_frames_match_independent_little_endian_parser() {
        let file = std::env::var_os("PDF14_WIRE_FRAMES").unwrap();
        let rows: serde_json::Value = serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap();
        for row in rows.as_array().unwrap() {
            let input: Vec<u8> = row["wire"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
            let (fields, bytes) = parse(&input).expect("authored valid frame");
            assert!(parse(&input[..input.len()-1]).is_err());
            assert!(parse(&[input.as_slice(), &[0]].concat()).is_err());
            let text = String::from_utf8(input[bytes].to_vec()).unwrap();
            assert_eq!(text, row["text"].as_str().unwrap_or_default());
            let actual = fields.bind(text);
            let expected = row["values"].as_array().unwrap();
            match (input[1], actual) {
                (0, PdfMutation::InsertPage(item)) => assert_eq!((item.index, item.page.width, item.page.height), (expected[0].as_u64().unwrap() as usize, expected[1].as_f64().unwrap(), expected[2].as_f64().unwrap())),
                (1, PdfMutation::RemovePage(item)) => assert_eq!(item.index, expected[0].as_u64().unwrap() as usize),
                (2, PdfMutation::MovePage(item)) => assert_eq!((item.from, item.to), (expected[0].as_u64().unwrap() as usize, expected[1].as_u64().unwrap() as usize)),
                (3, PdfMutation::ResizePage(item)) => assert_eq!((item.index, item.width, item.height), (expected[0].as_u64().unwrap() as usize, expected[1].as_f64().unwrap(), expected[2].as_f64().unwrap())),
                (4, PdfMutation::ReplacePageText(item)) => assert_eq!(item.index, expected[0].as_u64().unwrap() as usize),
                _ => panic!("wrong PDF 1.4 operation family"),
            }
        }
    }

    #[path = "EFFECT_FIXTURE_FILE"]
    mod fixture_effect;

    #[test]
    fn paid_utf8_pages_deny_short_grant_and_retire_exact_backing() {
        let src = std::env::var_os("PDF14_WIRE_FRAMES").unwrap();
        let rows: serde_json::Value = serde_json::from_slice(&std::fs::read(src).unwrap()).unwrap();
        let row = rows.as_array().unwrap().last().unwrap();
        let input: Vec<u8> = row["wire"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
        let expected = row["text"].as_str().unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 8192, maximum_capacity_bytes: 8192, maximum_release_bytes: 8192, maximum_depth: 8 };
        let mut checkpoint = |_| true;
        let mut native = NativeDecodeControl::new(8192, &mut checkpoint);
        use fixture_effect::{settled, with_effect};
        for cancel in [true, false] {
            let (mut receiver, _) = Pdf14OperationReceiving::admit(&input, (0, input.len()), grant).result.unwrap().unwrap();
            let header = with_effect(grant, 8192, &mut None, |effect| receiver.step_in(&input, &mut native, 8192, effect));
            assert_eq!(header.work, 35);
            header.result.unwrap();
            let mut actual = None;
            let denied = with_effect(RetainedCloneGrant { maximum_capacity_bytes: expected.len()-1, ..grant }, 8192, &mut actual,
                |effect| receiver.step_in(&input, &mut native, 8192, effect));
            assert_eq!((denied.work, denied.result.unwrap(), actual), (0, RetainedCloneProgress::default(), None));
            let allocated = with_effect(grant, 8192, &mut actual, |effect| receiver.step_in(&input, &mut native, 8192, effect));
            assert_eq!(allocated.result.unwrap().retained_capacity_bytes, expected.len());
            assert_eq!(actual, Some(expected.len()));
            let first = with_effect(grant, 8192, &mut None, |effect| receiver.step_in(&input, &mut native, 8192, effect));
            first.result.unwrap();
            assert_eq!(receiver.text.len(), 4095);
            if cancel {
                let mut actual = None;
                let denied = with_effect(RetainedCloneGrant { maximum_release_bytes: expected.len()-1, ..grant }, 8192, &mut actual,
                    |effect| settled(receiver.close_step(effect)));
                assert_eq!(denied.work, 0);
                assert!(!receiver.terminal_is_empty());
                let closed = with_effect(grant, 8192, &mut None, |effect| settled(receiver.close_step(effect)));
                assert_eq!(closed.result.unwrap().progress().released_bytes, expected.len());
            } else {
                let second = with_effect(grant, 8192, &mut None, |effect| receiver.step_in(&input, &mut native, 8192, effect));
                second.result.unwrap();
                let mut output = None;
                let result = receiver.take_into(&mut output, grant);
                assert_eq!(result.work, 1);
                let PdfMutation::ReplacePageText(text) = output.unwrap() else { panic!("incorrect typed operation") };
                assert_eq!(&text.text, expected);
            }
            assert!(receiver.terminal_is_empty());
        }
    }
}
