
//#region 🧪️Controlled image natural file

fn sampled_fixture_bytes(spec: &serde_json::Value) -> (Vec<u8>, Vec<(u32, Vec<u8>)>) {
    use serde_json::Value;
    let dimensions = |key| serde_json::from_value::<[usize; 2]>(spec[key].clone()).unwrap();
    let [width, height] = dimensions("imageSize");
    let [page_width, page_height] = dimensions("pageSize");
    let [x, y] = dimensions("imageOrigin");
    let images = spec["images"].as_array().unwrap();
    let samples = |image: &Value| {
        let quadrants: [Vec<u8>; 4] = serde_json::from_value(image["samples"].clone()).unwrap();
        let row = |index: usize| [quadrants[index].repeat(width / 2), quadrants[index + 1].repeat(width / 2)].concat();
        [row(0).repeat(height / 2), row(2).repeat(height / 2)].concat()
    };
    let source_samples: Vec<_> = images.iter().map(|image| (8, samples(image))).collect();
    let resources = |image: &Value| {
        let name = image["name"].as_str().unwrap();
        let object = image["object"].as_u64().unwrap();
        format!("<< /XObject << /{name} {object} 0 R >> >>")
    };
    let page = |contents, entry: &str| {
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {page_width} {page_height}] /Contents {contents} 0 R{entry} >>").into_bytes()
    };
    let stream = |fields: &str, body: &[u8]| {
        let mut stream = format!("<< /Length {}{fields} >>\nstream\n", body.len()).into_bytes();
        stream.extend_from_slice(body);
        stream.extend_from_slice(b"\nendstream");
        stream
    };
    let content = |image: &Value| {
        let name = image["name"].as_str().unwrap();
        stream("", format!("q {width} 0 0 {height} {x} {y} cm /{name} Do Q\n").as_bytes())
    };
    let picture = |index: usize| {
        let image = &images[index];
        let space = image["space"].as_str().unwrap();
        let mut fields = format!(" /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8");
        let samples = &source_samples[index].1;
        let body = if let Some(filter) = image["filter"].as_str() {
            fields.push_str(&format!(" /Filter /{filter}"));
            fixture_stored_zlib(samples)
        } else { samples.clone() };
        stream(&fields, &body)
    };
    let rows = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [3 0 R 6 0 R 9 0 R 13 0 R] /Count 4 /Resources {} >>", resources(&images[0])).into_bytes(),
        page(4, ""), content(&images[0]), picture(0),
        page(7, &format!(" /Resources {}", resources(&images[1]))), content(&images[1]), picture(1),
        page(12, " /Resources 10 0 R"), resources(&images[2]).into_bytes(), picture(2), content(&images[2]),
        page(14, ""), content(&images[0]),
    ];
    (pdf_fixture_file(rows), source_samples)
}

fn fixture_stored_zlib(samples: &[u8]) -> Vec<u8> {
    let length = u16::try_from(samples.len()).unwrap();
    let mut out = vec![0x78, 0x01, 0x01];
    out.extend_from_slice(&length.to_le_bytes());
    out.extend_from_slice(&(!length).to_le_bytes());
    out.extend_from_slice(samples);
    let (a, b) = samples.iter().fold((1u32, 0u32), |(a, b), byte| {
        let a = (a + u32::from(*byte)) % 65_521;
        (a, (b + a) % 65_521)
    });
    out.extend_from_slice(&((b << 16) | a).to_be_bytes());
    out
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_reexports_and_reopens_large_sampled_images() {
    use crate::standards::v1_7::subsets::base::{
        io::import::decode_pdf, schema::{snapshot::{ObjRef, PdfOp}, stream_roles::PdfStreamRoleValue},
    };
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_image_sample_streams;

    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json")).unwrap();
    let (input, samples) = sampled_fixture_bytes(&spec);
    assert!(input.len() > spec["minimumByteLength"].as_u64().unwrap() as usize);
    assert_eq!(reference_image_sample_streams(&input).unwrap(), samples);
    let cold = decode_pdf(&input).unwrap();
    let pages = spec["pages"].as_array().unwrap();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input, pages.len(), &mut scope).await;
    let mounted = app.snapshot().unwrap();
    assert_eq!(mounted.pages, cold.pages);
    assert_eq!(mounted.images, cold.images);
    assert_eq!(mounted.objects, cold.objects);
    assert_eq!(mounted.trailer, cold.trailer);
    assert_eq!(mounted.admitted_stream_roles, cold.admitted_stream_roles);
    for (image, role) in mounted.images.iter().zip(&mounted.admitted_stream_roles) {
        let spec = spec["images"].as_array().unwrap().iter().find(|spec| spec["name"] == image.id).unwrap();
        assert_eq!(role.identity.owner, ObjRef { num: spec["object"].as_u64().unwrap() as u32, gen: 0 });
        assert!(role.identity.path.is_empty());
        assert_eq!(role.dependencies.as_slice(), std::slice::from_ref(&role.identity));
        assert!(matches!(&role.value, PdfStreamRoleValue::Image { image: owned } if owned == image));
    }
    for (page, expected) in mounted.pages.iter().zip(pages) {
        let image = expected["image"].as_u64().unwrap() as usize;
        let name = spec["images"][image]["name"].as_str().unwrap();
        assert!(matches!(&page.content[1..3], [PdfOp::Transform { matrix }, PdfOp::PaintXObject { name: actual }]
            if actual == name && *matrix == [72., 0., 0., 72., 20., 20.]));
    }

    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("mounted PDF reexports").expect("natural PDF output");
    assert_eq!(reference_image_sample_streams(&artifact.data).unwrap(), samples);
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("original image editor closes");
    let mut reopened = open_registered_pdf(artifact.data, pages.len(), &mut scope).await;
    assert_eq!(reopened.snapshot().unwrap().images, cold.images);
    assert_eq!(reopened.snapshot().unwrap().pages, cold.pages);
    laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).expect("reopened image editor closes");
}

//#endregion 🧪️Controlled image natural file
