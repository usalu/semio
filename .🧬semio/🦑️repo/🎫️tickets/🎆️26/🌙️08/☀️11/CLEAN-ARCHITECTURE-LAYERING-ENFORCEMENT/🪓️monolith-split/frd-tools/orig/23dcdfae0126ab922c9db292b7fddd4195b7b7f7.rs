use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfImage, PdfObject, PdfStreamFilter};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::colour::{extra_entries, lift_colour_space, lower_colour_space, numbers_of, push_opt, raw_stream};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_i64, dict_name};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::modules::xref::ObjectSink;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfImageBody, PdfImageMask};
use crate::standards::v1_7::subsets::base::modules::images::*;

#[test]
fn pdf_semantic_body_image_words_match_neutral_native_and_independent_lopdf() {
    use crate::standards::v1_7::subsets::base::io::export::encode_pdf;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, PdfPage, PdfColorSpace};
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🔨️modules/🖼️images/🧪️tests/🔬️unit/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){let integer=|key:&str|row[key].as_u64().unwrap() as u32;let values=row["values"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u32).collect::<Vec<_>>();let packed=row["packed"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();let (width,height,components,bits)=(integer("width"),integer("height"),integer("components"),integer("bits"));
        assert_eq!(pack_image_samples(width,height,components,bits,&values).unwrap(),packed);assert_eq!(unpack_image_samples(width,height,components,bits,&packed).unwrap(),values);
        let mut snapshot=PdfSnapshot::default();snapshot.pages.push(PdfPage::new(20.0,20.0));snapshot.images.push(PdfImage::samples("I",width,height,if components==3{PdfColorSpace::DeviceRgb}else{PdfColorSpace::DeviceGray},bits,values));let bytes=encode_pdf(&snapshot).unwrap();let reference=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_image_sample_streams(&bytes).unwrap();assert_eq!(reference,vec![(bits,packed)]);
    }
    assert!(unpack_image_samples(3,2,1,1,&[160]).is_err());assert!(pack_image_samples(1,1,1,4,&[16]).is_err());
}

#[test]
fn pdf_semantic_body_foreign_artifact_custody_is_explicit_and_independent() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot,PdfIndirectObject,ObjRef};
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::admit_pdf_artifact;
    let mut snapshot=PdfSnapshot::default();let object=PdfObject::Stream {dict:Vec::new(),data:vec![255,216,255,217],filters:vec![PdfStreamFilter::Dct {color_transform:None}]};let reference=admit_pdf_artifact(&mut snapshot,"s.stdio.jpeg",object.clone()).unwrap();assert_eq!(NativePdfArtifactResources::from_objects(&snapshot.objects).resolve(&reference).unwrap(),object);assert!(NativePdfArtifactResources::default().resolve(&reference).is_err());let bytes=snapshot.objects[0].value.clone();snapshot.objects.clear();snapshot.objects.push(PdfIndirectObject{id:ObjRef{num:99,gen:0},value:bytes});assert_eq!(NativePdfArtifactResources::from_objects(&snapshot.objects).resolve(&reference).unwrap(),object);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
