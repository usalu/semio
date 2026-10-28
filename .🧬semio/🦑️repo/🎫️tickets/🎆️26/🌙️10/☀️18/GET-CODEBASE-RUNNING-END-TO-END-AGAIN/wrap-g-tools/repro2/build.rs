use std::process::Command;
fn main() {
    let rustc = std::env::var("RUSTC").unwrap();
    let wrapper = std::env::var("RUSTC_WRAPPER").unwrap();
    for arguments in [vec!["--version"], vec!["-vV"]] {
        let output = Command::new(&wrapper).arg(&rustc).args(&arguments).output().expect("failed to run rustc");
        assert!(output.status.success(), "{:?}", output);
    }
}
