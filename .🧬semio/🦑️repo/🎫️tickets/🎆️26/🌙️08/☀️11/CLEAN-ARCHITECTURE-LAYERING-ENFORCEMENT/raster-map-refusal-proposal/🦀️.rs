//! 🖨️ Map rejection retains machine cause and returns the exact rejected key/value owner.
use crate::{RasterOwnedMap,RasterOwnedMapInsert};
#[test]
fn map_rejection_retains_kind_and_exact_candidate() {
    let rows: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️map-refusal/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let key=row["key"].as_str().unwrap().to_owned();
        let value=u8::try_from(row["value"].as_u64().unwrap()).unwrap();
        let (kind,reason,actual_key,actual_value)=if row["operation"]=="backing" {
            let mut map=RasterOwnedMap::<[u8;16384]>::new();
            let rejected=map.insert(key,[value;16384]).unwrap_err();
            assert!(rejected.value.iter().all(|byte|*byte==value));
            map.retire();
            (rejected.kind,rejected.reason,rejected.key,value)
        } else {
            let mut map=RasterOwnedMap::<u8>::new();
            let rejected=match row["operation"].as_str().unwrap() {
                "duplicate"=>{map.insert(key.clone(),11).unwrap();map.insert(key,value).unwrap_err()},
                "full"=>{for index in 0..64 {map.insert(format!("key-{index}"),11).unwrap();}map.insert(key,value).unwrap_err()},
                "unadmitted"=>match map.insert_pre_admitted(key,value) {Err(rejected)=>rejected,Ok(RasterOwnedMapInsert::Inserted)=>panic!("unadmitted page cannot accept"),Ok(RasterOwnedMapInsert::Replaced(mut previous))=>{previous.take();panic!("unadmitted page cannot replace")}},
                _=>panic!("closed operation"),
            };
            map.retire();
            (rejected.kind,rejected.reason,rejected.key,rejected.value)
        };
        assert_eq!(kind.as_str(),row["expectedKind"].as_str().unwrap());
        assert_eq!(reason,row["reason"].as_str().unwrap());
        assert_eq!(actual_key,row["key"].as_str().unwrap());
        assert_eq!(actual_value,value);
    }
}
