//! 🧪️ Exercises the real PDF preview sampler using the shared MuPDF fixture.

use serde_json::Value;

mod standards {
    pub mod v1_7 {
        pub mod subsets {
            pub mod base {
                pub mod schema {
                    pub mod snapshot {
                        #[derive(PartialEq)]
                        pub enum PdfColorSpace { DeviceGray, DeviceRgb, Other }
                        pub enum PdfImageBody { Samples { values: Vec<u32> }, Retained }
                        pub enum PdfImageMask { ColorKey { ranges: Vec<u32> }, Stencil { image: String } }
                        pub struct PdfInlineImage {
                            pub image_mask: bool, pub color_space: Option<PdfColorSpace>, pub width: u32,
                            pub height: u32, pub bits_per_component: u32, pub decode: Vec<f64>, pub body: PdfImageBody,
                        }
                        pub struct PdfImage {
                            pub image_mask: bool, pub color_space: Option<PdfColorSpace>, pub width: u32,
                            pub height: u32, pub bits_per_component: u32, pub decode: Vec<f64>, pub body: PdfImageBody,
                            pub mask: Option<PdfImageMask>, pub soft_mask: Option<String>,
                            pub soft_mask_in_data: Option<u32>, pub matte: Option<Vec<f64>>,
                        }
                    }
                }
            }
        }
    }
}

#[path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🎭️sampled-alpha/🦀️.rs"]
mod sampled_alpha;

use sampled_alpha::PdfImagePreviewSamples;
use standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfImage, PdfImageBody, PdfImageMask};

fn samples(width: u32, height: u32, bits: u32, color_space: PdfColorSpace, data: &Value) -> PdfImage {
    PdfImage {
        image_mask: false, color_space: Some(color_space), width, height, bits_per_component: bits,
        decode: Vec::new(), body: PdfImageBody::Samples { values: serde_json::from_value(data.clone()).unwrap() },
        mask: None, soft_mask: None, soft_mask_in_data: None, matte: None,
    }
}

fn verify(image: PdfImagePreviewSamples<'_>, expected: &Value) {
    let coords = [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)];
    let actual: Vec<u8> = coords.into_iter().flat_map(|(u, v)| image.rgba(u, v)).collect();
    assert_eq!(actual, serde_json::from_value::<Vec<u8>>(expected.clone()).unwrap());
}

#[test]
fn image_samples_match_color_key_stencil_soft_alpha_and_matte() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🧪️image-paint.json")).unwrap();
    let mut parent = samples(2, 2, 8, PdfColorSpace::DeviceRgb, &fixture["before"]);
    parent.decode = serde_json::from_value(fixture["decode"].clone()).unwrap();
    parent.mask = Some(PdfImageMask::ColorKey { ranges: serde_json::from_value(fixture["mask"].clone()).unwrap() });
    verify(PdfImagePreviewSamples::xobject(&parent, |_| None).unwrap(), &fixture["maskedRgba"]);
    for case in fixture["referencedMasks"].as_array().unwrap() {
        let mut parent = samples(2, 2, 8, PdfColorSpace::DeviceRgb, case.get("source").unwrap_or(&fixture["before"]));
        let number = |key: &str| case[key].as_u64().unwrap().try_into().unwrap();
        let mut mask = samples(number("width"), number("height"), number("bits"), PdfColorSpace::DeviceGray, &case["samples"]);
        mask.decode = serde_json::from_value(case["decode"].clone()).unwrap();
        mask.matte = serde_json::from_value(case["matte"].clone()).unwrap();
        if case["kind"] == "stencil" {
            mask.image_mask = true;
            mask.color_space = None;
            parent.mask = Some(PdfImageMask::Stencil { image: "mask".into() });
        } else {
            if case["overridesColorKey"] == true {
                parent.mask = Some(PdfImageMask::ColorKey { ranges: vec![0, 255, 0, 255, 0, 255] });
            }
            parent.soft_mask = Some("mask".into());
        }
        let image = PdfImagePreviewSamples::xobject(&parent, |id| (id == "mask").then_some(&mask)).unwrap();
        assert_eq!(image.dimensions(), (2, 2));
        verify(image, &case["expectedRgba"]);
    }
}
