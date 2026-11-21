//! 🎯 Exercise the staged Store target reader through both real registered PDF mutation vocabularies.

extern crate semio_os_kernel as store;

use serde_json::Value;
use store::ArtifactOperationTargets;

mod v14 {
    pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::{MovePage, PdfMutation as Actual};
    pub struct PdfMutation<'a>(pub &'a Actual);
    mod staged { include!("target14.rs"); }
}

mod v17 {
    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
        mutations::{MovePage, PdfMutation as Actual, RemoveImage, ReplaceContent, SetDictEntry, SetInfo, SetLanguage},
        snapshot::{ObjRef, PdfInfo, PdfObject, PdfOp},
    };
    pub struct PdfMutation<'a>(pub &'a Actual);
    mod staged { include!("target17.rs"); }
}

fn compare(expected: &Value, target: Option<&dyn ArtifactOperationTargets>) {
    let expected = expected.as_array().unwrap();
    let Some(target) = target else { assert!(expected.is_empty()); return; };
    assert_eq!(target.len(), expected.len());
    assert!(target.segment_len(expected.len()).is_err());
    for (index, expected) in expected.iter().enumerate() {
        let bytes = expected.as_str().unwrap().as_bytes();
        assert_eq!(target.segment_len(index).unwrap(), bytes.len());
        let mut read = Vec::new();
        let mut part = [0; 2];
        while read.len() < bytes.len() {
            let copied = target.read_segment(index, read.len(), &mut part).unwrap();
            assert!(copied > 0 && copied <= part.len());
            read.extend_from_slice(&part[..copied]);
        }
        assert_eq!(read, bytes);
        assert_eq!(target.read_segment(index, bytes.len(), &mut part).unwrap(), 0);
        assert!(target.read_segment(index, bytes.len() + 1, &mut part).is_err());
    }
}

fn main() {
    let law: Value = serde_json::from_slice(include_bytes!("law.json")).unwrap();
    let moved = v14::Actual::MovePage(v14::MovePage { from: 4_294_967_295, to: 8 });
    compare(&law["move14"], v14::PdfMutation(&moved).original_conflict_target());
    for (name, operation) in [
        ("move17", v17::Actual::MovePage(v17::MovePage { from: 60, to: 2 })),
        ("operator", v17::Actual::ReplaceContent(v17::ReplaceContent { index: 23, at: 1011, op: v17::PdfOp::Save })),
        ("cos", v17::Actual::SetDictEntry(v17::SetDictEntry { id: v17::ObjRef { num: u32::MAX, gen: u16::MAX }, path: Vec::new(), key: "透明/%".into(), value: v17::PdfObject::Bool(true), index: None })),
        ("image", v17::Actual::RemoveImage(v17::RemoveImage { id: "Deck%Å Ω".into() })),
        ("info", v17::Actual::SetInfo(v17::SetInfo { info: v17::PdfInfo::default() })),
        ("language", v17::Actual::SetLanguage(v17::SetLanguage { language: Some("de-CH".into()) })),
    ] {
        compare(&law[name], v17::PdfMutation(&operation).original_conflict_target());
    }
    println!("[DEBUG] Typed PDF 1.4 and 1.7 history target path laws passed");
}
