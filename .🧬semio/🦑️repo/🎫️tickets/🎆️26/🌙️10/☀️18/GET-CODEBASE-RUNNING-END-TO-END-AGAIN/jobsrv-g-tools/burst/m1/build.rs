use std::io::Write;
fn main() {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let files: usize = std::env::var("BURST_FILES").ok().and_then(|v| v.parse().ok()).unwrap_or(8);
    let mut build = cc::Build::new();
    for i in 0..files {
        let path = out.join(format!("f{}.c", i));
        writeln!(std::fs::File::create(&path).unwrap(), "int f{}_{}(void) {{ return {}; }}", i, env!("CARGO_PKG_NAME"), i).unwrap();
        build.file(path);
    }
    build.warnings(false).compile("burst");
    println!("cargo:rerun-if-env-changed=BURST_FILES");
}
