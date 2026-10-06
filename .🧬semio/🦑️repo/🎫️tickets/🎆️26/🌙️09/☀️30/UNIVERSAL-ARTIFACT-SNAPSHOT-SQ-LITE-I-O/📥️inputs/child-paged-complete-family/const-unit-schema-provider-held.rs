pub const fn unit_by_symbol(symbol: &str) -> Option<&'static UnitSpec> {
    let wanted=symbol.as_bytes();
    let mut index=0;
    while index<UNITS.len(){
        let candidate=UNITS[index].symbol.as_bytes();
        if candidate.len()==wanted.len(){
            let mut byte=0;
            while byte<wanted.len()&&candidate[byte]==wanted[byte]{byte+=1;}
            if byte==wanted.len(){return Some(&UNITS[index]);}
        }
        index+=1;
    }
    None
}
