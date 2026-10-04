    #[semio_framework_async_macros::async_test]
    async fn canonical_confidence_none_is_explicitly_excluded_from_actual_identification() {
        fn absent(_: &IoPayload) -> Confidence { Confidence::None }
        fn low(_: &IoPayload) -> Confidence { Confidence::Low }
        fn medium(_: &IoPayload) -> Confidence { Confidence::Medium }
        fn high(_: &IoPayload) -> Confidence { Confidence::High }
        static NONE: IoEntry = IoEntry { from: CARRIER_TEXT, into: A, fidelity: IoFidelity::Exact, sniff: Some(absent), run: passthrough };
        static LOW: IoEntry = IoEntry { from: CARRIER_TEXT, into: B, fidelity: IoFidelity::Exact, sniff: Some(low), run: passthrough };
        static MEDIUM: IoEntry = IoEntry { from: CARRIER_TEXT, into: C, fidelity: IoFidelity::Exact, sniff: Some(medium), run: passthrough };
        static HIGH: IoEntry = IoEntry { from: CARRIER_TEXT, into: D, fidelity: IoFidelity::Exact, sniff: Some(high), run: passthrough };
        let mut registry: EntryMap = BTreeMap::new();
        for entry in [&NONE, &LOW, &MEDIUM, &HIGH] { registry.insert(key(entry.from, entry.into).await, entry); }
        let found = resolve_identify(&registry, &IoPayload::Text("canonical confidence evidence".into())).await;
        assert_eq!(found, vec![(ArtifactDialect::from(D), Confidence::High), (ArtifactDialect::from(C), Confidence::Medium), (ArtifactDialect::from(B), Confidence::Low)]);
        assert!(!found.iter().any(|(dialect, _)| *dialect == ArtifactDialect::from(A)));
        println!("[DEBUG] actual carrier identification excludes None explicitly and preserves High/Medium/Low evidence ordering");
    }
