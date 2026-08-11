
#[cfg(test)]
mod media_vocabulary_tests {
    //! 🔀️ Relocated verbatim from 🔺️mesh/🦀️component.rs's own test mod alongside the
    //! 🔖️MediaVocabulary types above (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
    //! wave 4a) — mesh no longer defines MediaType/MediaCompat/Media/MediaPayload/MediaFingerprint/
    //! MediaError, so these tests moved with their types.
    use super::*;

    #[test]
    fn media_types_compatible_covers_direct_any_convert_and_reject() {
        let brep = MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep };
        let mesh_form = MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh };
        let any_3d = MediaType { class: MediaClass::ThreeD, form: MediaForm::Any };
        let vector = MediaType { class: MediaClass::TwoD, form: MediaForm::Vector };
        let raster = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
        let text = MediaType { class: MediaClass::Text, form: MediaForm::Document };
    
        assert_eq!(media_types_compatible(&brep, &brep), MediaCompat::Direct);
        assert_eq!(media_types_compatible(&brep, &any_3d), MediaCompat::Direct, "Any on the accepting side takes anything within the class");
        assert!(matches!(media_types_compatible(&brep, &mesh_form), MediaCompat::Convert { from: MediaForm::Brep, to: MediaForm::Mesh }));
        assert!(matches!(media_types_compatible(&vector, &raster), MediaCompat::Convert { from: MediaForm::Vector, to: MediaForm::Raster }));
        assert_eq!(media_types_compatible(&mesh_form, &brep), MediaCompat::Reject, "mesh->brep has no registered conversion");
        assert_eq!(media_types_compatible(&brep, &text), MediaCompat::Reject, "class mismatch always rejects");
    }
    #[test]
    fn media_fingerprint_structured_hashes_json_binary_reuses_blob_hash() {
        let structured = Media {
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            payload: MediaPayload::Structured { schema: "s".into(), json: "{}".into() },
        };
        let fingerprint = MediaFingerprint::of(&structured);
        assert_eq!(fingerprint, MediaFingerprint::of(&structured), "fingerprint is deterministic");

        let mut changed = structured.clone();
        if let MediaPayload::Structured { json, .. } = &mut changed.payload {
            *json = "{\"a\":1}".into();
        }
        assert_ne!(MediaFingerprint::of(&changed), fingerprint, "different json content hashes differently");

        let binary = Media {
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            payload: MediaPayload::Binary { format: MediaFormat::Glb, blob_hash: "abc123".into() },
        };
        assert_eq!(MediaFingerprint::of(&binary), MediaFingerprint("abc123".into()), "binary payload reuses its blob hash verbatim");
    }
    #[test]
    fn media_error_messages_are_human_readable() {
        assert_eq!(MediaError::UnknownPort("in".into()).to_string(), "unknown media port `in`");
        let incompatible = MediaError::Incompatible {
            port: "out".into(),
            produced: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
            accepted: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        };
        assert!(incompatible.to_string().starts_with("port `out` produced"));
        assert_eq!(MediaError::Payload("p".into(), "bad".into()).to_string(), "media payload error on port `p`: bad");
        assert_eq!(MediaError::NotImplemented.to_string(), "media ports are not implemented for this app");
    }
}
