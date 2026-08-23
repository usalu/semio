use semio_repo_test_host::oracle::{oracle_create_pdf, oracle_delete_page, oracle_replace_metadata, project_pdf, PdfPageSpec, PdfSpec};

fn main() {
    let a4 = |c: &str| PdfPageSpec { media_box: [0.0, 0.0, 595.0, 842.0], content: c.to_string() };
    let spec = PdfSpec { version: (1, 7), pages: vec![a4(""), a4("")], title: Some("Original Title".into()), author: Some("Original Author".into()) };
    let bytes = oracle_create_pdf(&spec).expect("oracle create");
    if let Some(out) = std::env::args().nth(1) {
        std::fs::write(&out, &bytes).expect("write");
        eprintln!("wrote {} bytes to {}", bytes.len(), out);
    }
    println!("create      -> {}", project_pdf(&bytes).expect("project").to_string());
    let edited = oracle_replace_metadata(&bytes, Some("Replaced Title"), Some("Replaced Author")).expect("metadata");
    println!("metadata    -> {}", project_pdf(&edited).expect("project").to_string());
    let deleted = oracle_delete_page(&bytes, 2).expect("delete");
    println!("delete-page -> {}", project_pdf(&deleted).expect("project").to_string());
}
