#[cfg(test)]
mod const_authored_unit_schema_tests{
    use super::*;
    #[test]
    fn borrowed_authored_const_unit_lookup_preserves_all_original_unit_identities(){
        const DEGREE:Option<&UnitSpec>=unit_by_symbol("°");
        const LENGTH:Option<&UnitSpec>=unit_by_symbol("m");
        const REFUSED:Option<&UnitSpec>=unit_by_symbol("°\u{200d}");
        let fixture:serde_json::Value=serde_json::from_str(include_str!("@CONST_UNIT_FIXTURE@")).unwrap();
        let symbols=fixture["symbols"].as_array().unwrap();assert_eq!(UNITS.len(),symbols.len());
        for(index,symbol)in symbols.iter().enumerate(){assert_eq!(UNITS[index].symbol,symbol.as_str().unwrap());}
        assert!(std::ptr::eq(DEGREE.unwrap(),&UNITS[11]));assert!(std::ptr::eq(LENGTH.unwrap(),&UNITS[0]));assert!(REFUSED.is_none());assert!(!std::ptr::eq(unit_by_symbol("deg").unwrap(),DEGREE.unwrap()));
        for query in fixture["queries"].as_array().unwrap(){let actual=unit_by_symbol(query["symbol"].as_str().unwrap());match query["expectedIndex"].as_u64(){Some(index)=>assert!(std::ptr::eq(actual.unwrap(),&UNITS[index as usize])),None=>assert!(actual.is_none())}}
    }
}
