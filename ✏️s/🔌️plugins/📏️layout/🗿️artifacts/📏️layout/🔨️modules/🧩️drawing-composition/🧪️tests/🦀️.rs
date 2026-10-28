//! 🧪️ The composition exposes the exact actual Draw-owned producers with no alternate payload owner.
use super::*;
#[test]fn genuine_owner_producers_are_original(){let contract:serde_json::Value=serde_json::from_str(include_str!("../📜️contract.json")).unwrap();assert_eq!(contract["owner"],"s.draw.drawing@1/*");assert!(std::ptr::eq(&DRAWING_ADMITTER,&semio_s_artifact_draw_drawing_model::validation::DRAWING_ADMITTER));assert!(std::ptr::eq(&DRAWING_VECTOR_SOURCE,&semio_s_artifact_draw_drawing_model::vector_source::DRAWING_VECTOR_SOURCE));}
