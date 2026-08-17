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
