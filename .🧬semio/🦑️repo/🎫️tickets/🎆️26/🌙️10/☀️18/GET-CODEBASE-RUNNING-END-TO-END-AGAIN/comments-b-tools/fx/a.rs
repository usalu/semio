//! module doc
use a::b; // top trailing

/// doc
pub fn f() { // open
    // standalone
    let s = "// not"; // trailing
    let r = r#"// raw "#;
    /* block */ let c = 'x';
    // SAFETY: because
    //   continued
    unsafe { x() }

    // gap

    g(); /* inline */ h();
}

mod tests { // 🔖️Tests
    // module level stays
    #[test]
    fn t() {
        // inner
        assert!(true);
    }
}

struct S {
    a: u8, // field
    // sep
    b: u8,
}
