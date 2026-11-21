use super::*;
use semio_framework_pack_json::{parse, JsonMemberPolicy};

#[test]
fn whole_control_pages_follow_the_shared_cost_cases() {
    let fixture = parse(include_str!("pages.json"), JsonMemberPolicy::Reject).expect("page fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let fields = case["fields"].as_array().unwrap().iter().flat_map(|row| {
            let field = FieldCost { nodes: row["nodes"].as_u64().unwrap() as usize, section: row["section"].as_u64().map(|index| index as usize) };
            std::iter::repeat_n(field, row["count"].as_u64().unwrap() as usize)
        });
        let actual = form_page(fields, case["budget"].as_u64().unwrap() as usize, case["requested"].as_u64().unwrap() as usize);
        if let Some(index) = case["rejected"].as_u64() {
            assert_eq!(actual, Err(index as usize), "{}", case["name"].as_str().unwrap());
            continue;
        }
        let actual = actual.unwrap();
        let expected = &case["expect"];
        for (key, result) in [("index", actual.index), ("total", actual.total), ("first", actual.fields.start), ("last", actual.fields.end), ("nodes", actual.nodes)] {
            assert_eq!(Some(result as u64), expected[key].as_u64(), "{} {key}", case["name"].as_str().unwrap());
        }
    }
}
