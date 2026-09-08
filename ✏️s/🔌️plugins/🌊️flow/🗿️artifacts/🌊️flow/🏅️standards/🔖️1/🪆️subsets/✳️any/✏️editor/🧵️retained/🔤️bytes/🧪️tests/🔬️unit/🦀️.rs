
use super::*;

#[test]
fn semantic_text_copy_and_equality_obey_one_byte_and_production_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let source = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
        let grant = row["grantBytes"].as_u64().unwrap() as usize;
        let mut copy = TextCopy::default();
        let mut copied = 0;
        while !copy.complete() {
            let bytes = copy.advance(&source, grant).unwrap().unwrap();
            assert!(bytes <= grant);
            copied += bytes;
        }
        let target = copy.take().unwrap();
        assert_eq!(target, source);
        assert_eq!(copied, source.len());
        let mut equality = Equality::default();
        let mut compared = 0;
        loop {
            let (result, bytes) = equality.advance(&source, &target, grant);
            assert!(bytes <= grant);
            compared += bytes;
            if let Some(result) = result {
                assert!(result);
                break;
            }
        }
        assert_eq!(compared, source.len() * 2);
        assert_eq!(Equality::default().advance(&source, "", grant), (Some(false), 0));
    }
}
