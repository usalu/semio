    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[dsl(extension = "demo")]
    struct DemoProjection {
        n: i32,
    }

    // `impl store::DocumentPack for DemoProjection` is now generated automatically by
    // `#[derive(dsl::DslDocument)]` above (see dsl/derive/rs/lib.rs's `🔖️DslDocument` region) —
    // same seam as its `impl store::DocumentDsl for DemoProjection` sibling.

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoDiff {
        n: Option<i32>,
    }

    impl OperationDiff<DemoProjection> for DemoDiff {
        fn apply(&self, projection: &DemoProjection) -> DemoProjection {
            DemoProjection {
                n: self.n.unwrap_or(projection.n),
            }
        }

        fn absorb(&mut self, other: Self) {
            if other.n.is_some() {
                self.n = other.n;
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
    #[serde(tag = "operation")]
    enum DemoOperation {
        #[dsl(key = "set-n")]
        SetN { n: i32 },
    }

    impl Operation<DemoProjection> for DemoOperation {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                DemoOperation::SetN { n } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![DemoOperation::SetN { n: projection.n }]
        }
    }

    /// @emoji 🛰️ Builds a foreign {@link OperationEnvelope} (as if authored by `actor` on another peer) by

    /// applying `operation` in a throwaway peer store and stamping the envelope's actor id.
    fn foreign_operation_envelope(actor: &str, operation: DemoOperation) -> OperationEnvelope {
        let mut peer = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "demo",
            DemoProjection { n: 0 },
            None,
        ));
        peer.dispatch(DocumentCommand::Apply {
            operations: vec![operation],
            description: None,
        })
        .expect("peer apply");
        let edit = peer.envelope().vcs.edits.last().expect("peer edit").clone();
        let mut envelope = operation_envelope_from_edit(peer.envelope(), &edit, Vec::new()).expect("operation envelope");
        envelope.actor = ActorId(actor.to_string());
        envelope
    }

    #[test]
    fn materialize_replays_forward_operations() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").n, 1);
        assert_eq!(store.envelope().vcs.edits.len(), 1);
    }

    #[test]
    fn undo_redo_round_trip() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").n, 0);
        store.dispatch(DocumentCommand::Redo).expect("redo");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn apply_computes_backwards_from_pre_state() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let edit = &store.envelope().vcs.edits[0];
        assert_eq!(edit.backwards, vec![DemoOperation::SetN { n: 0 }]);
    }

    #[test]
    fn commit_checkpoint_wraps_edits_into_change() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("init".into()),
                authors: vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            })
            .expect("commit");
        assert_eq!(store.envelope().vcs.changes.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
        assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
    }

    #[test]
    fn checkout_checkpoint_restores_applied_edits() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit");
        let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply2");
        assert_eq!(store.projection().expect("projection").n, 9);
        store
            .dispatch(DocumentCommand::CheckoutCheckpoint {
                checkpoint_id,
            })
            .expect("checkout");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn alternatives_switch_restores_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative {
                name: "branch-a".into(),
            })
            .expect("create alternative");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on branch");
        store
            .dispatch(DocumentCommand::SwitchAlternative {
                alternative_id: alt_id,
            })
            .expect("switch");
        assert_eq!(store.projection().expect("projection").n, 1);
    }

    #[test]
    fn checkout_old_checkpoint_then_commit_creates_a_fork() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        let c1 = store.envelope().vcs.checkpoints[0].id.clone();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() })
            .expect("checkout c1");
        assert_eq!(store.current_checkpoint_id(), Some(c1.as_str()));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("fork".into()),
                authors: Vec::new(),
            })
            .expect("commit fork");
        let children: Vec<&Checkpoint> = store
            .envelope()
            .vcs
            .checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str()))
            .collect();
        assert_eq!(children.len(), 2, "checking out an old checkpoint before committing must fork, not extend the trunk");
    }

    #[test]
    fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create alternative");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("branch commit".into()),
                authors: Vec::new(),
            })
            .expect("commit on branch");
        assert_eq!(store.envelope().vcs.alternatives[0].checkpoint_ids.len(), 2);
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
    }

    #[test]
    fn history_columns_orders_newest_first_and_labels_trunk_root() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit c1");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c2".into()),
                authors: Vec::new(),
            })
            .expect("commit c2");
        let columns = store.history_columns();
        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
        assert_eq!(columns[0].lane, 0);
        assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
        assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
        let json = serde_json::to_string(&columns[0]).expect("serialize");
        assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
    }

    #[test]
    fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("root".into()),
                authors: Vec::new(),
            })
            .expect("commit root");
        let root = store.envelope().vcs.checkpoints[0].id.clone();

        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() })
            .expect("create feature-a");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("a1".into()),
                authors: Vec::new(),
            })
            .expect("commit a1");

        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature-b".into() })
            .expect("create feature-b");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("b1".into()),
                authors: Vec::new(),
            })
            .expect("commit b1");

        store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root.clone() })
            .expect("checkout root again");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("main resumed".into()),
                authors: Vec::new(),
            })
            .expect("commit main resumed");

        let columns = store.history_columns();
        assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
        let by_message: HashMap<String, &HistoryColumn> = columns
            .iter()
            .filter_map(|column| column.description.clone().map(|description| (description, column)))
            .collect();
        assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
        assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
        let a_lane = by_message["a1"].lane;
        let b_lane = by_message["b1"].lane;
        assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
        assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
        assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

        let root_children: Vec<&HistoryColumn> = columns
            .iter()
            .filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str()))
            .collect();
        assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
    }

    #[test]
    fn no_backbone_by_default() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
        let store = DocumentStore::new(envelope);
        assert!(store.backbone_ref().is_none());
    }

    #[test]
    fn memory_backbone_pair_propagates_edits_bidirectionally() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope_a: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let envelope_b: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentStore::new(envelope_a);
        let mut store_b = DocumentStore::new(envelope_b);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        store_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply on a");
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 1, "b receives a's edit");

        store_b
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply on b");
        store_a.tick().expect("tick a");
        assert_eq!(store_a.projection().expect("projection a").n, 2, "a receives b's edit");
    }

    #[test]
    fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store_a = DocumentStore::new(envelope.clone());
        let mut store_b = DocumentStore::new(envelope);
        store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        store_a.detach_backbone();
        assert!(store_a.backbone_ref().is_none());

        store_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 9 }],
                description: None,
            })
            .expect("apply after detach still works on the in-memory graph");
        assert_eq!(store_a.projection().expect("projection a").n, 9);
        store_b.tick().expect("tick b");
        assert_eq!(store_b.projection().expect("projection b").n, 0, "detached edits never reach the peer");
    }

    #[test]
    fn deserialized_envelope_with_stale_backbone_ref_never_auto_attaches() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut stale_json: serde_json::Value =
            serde_json::to_value(&envelope).expect("serialize envelope");
        stale_json["backbone"] = serde_json::json!({ "uri": "folder:///nonexistent/path" });
        let stale_envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            serde_json::from_value(stale_json).expect("deserialize envelope with stale backbone ref");

        let mut store = DocumentStore::new(stale_envelope.clone());
        assert!(
            store.tick().expect("tick with no live backbone is a no-operation") == false,
            "no backbone was ever attached, so there is nothing to pump"
        );
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply works purely against the in-memory graph");
        assert_eq!(store.projection().expect("projection").n, 1);

        store.set_state(stale_envelope, Vec::new(), Vec::new());
        assert!(
            store.tick().expect("tick after set_state with no live backbone is a no-operation") == false,
            "set_state must not resurrect IO from a stale backbone descriptor either"
        );
    }





    #[test]
    fn document_codec_of_round_trips_pack_and_dsl() {
        let codec = DocumentCodec::of::<DemoProjection, DemoOperation>("demo/v1");
        assert_eq!(codec.schema, "demo/v1");
        assert_eq!(codec.extension, "demo");

        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 4 }, None);
        let envelope_json = serde_json::to_string(&envelope).expect("envelope json");

        let (pack_files, dsl_mirror) = (codec.print)(&envelope_json).expect("codec print");
        assert_eq!(dsl_mirror, DemoProjection { n: 4 }.print_dsl(), "dsl mirror matches the initial projection's print_dsl");

        let parsed_json = (codec.parse)(&pack_files.pack, &pack_files.ops).expect("codec parse");
        let parsed: DocumentEnvelope<DemoProjection, DemoOperation> = serde_json::from_str(&parsed_json).expect("parse envelope json");
        assert_eq!(parsed.vcs.initial_projection.n, 4, "codec.parse round trips through pack bytes");

        let parsed_dsl_json = (codec.parse_dsl)(&dsl_mirror, &pack_files.ops).expect("codec parse_dsl");
        let parsed_dsl: DocumentEnvelope<DemoProjection, DemoOperation> = serde_json::from_str(&parsed_dsl_json).expect("parse envelope json (dsl path)");
        assert_eq!(
            parsed.vcs.initial_projection, parsed_dsl.vcs.initial_projection,
            "codec.parse and codec.parse_dsl agree on the same document"
        );

        register_document_codec(codec);
        assert!(document_codec("demo/v1").is_some(), "registered codec is discoverable by schema string");
        assert!(document_codec("no-such-schema").is_none());
    }


    #[test]
    fn attach_reconciles_a_pushed_snapshot() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let seeded: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut seed_store = DocumentStore::new(seeded);
        seed_store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        remote
            .push(BackboneMessage::Snapshot {
                envelope_json: seed_store.envelope_json().expect("seed json"),
            })
            .expect("push snapshot");

        let fresh: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(fresh);
        store.attach_backbone(Box::new(channel)).expect("attach reconciles the pushed snapshot");
        assert_eq!(store.projection().expect("projection").n, 5, "adopted the pushed snapshot's edit");
    }

    #[test]
    fn channel_backbone_round_trips_between_store_and_actor() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let attach_flush = remote.drain().expect("drain attach");
        assert!(
            attach_flush.iter().any(|message| matches!(message, BackboneMessage::Snapshot { .. })),
            "attach flushes a snapshot to the actor end: {attach_flush:?}"
        );

        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 4 }],
                description: None,
            })
            .expect("apply");
        let outbound = remote.drain().expect("drain apply");
        assert!(
            outbound.iter().any(|message| matches!(message, BackboneMessage::Operations { .. })),
            "a local apply is sent outbound as operations: {outbound:?}"
        );

        remote
            .push(BackboneMessage::Operations {
                envelopes: vec![foreign_operation_envelope("peer", DemoOperation::SetN { n: 8 })],
            })
            .expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 8, "store ingests the actor's inbound operations");
    }

    #[test]
    fn pump_acks_ingested_operations() {
        let (channel, remote) = ChannelBackbone::pair("chan");
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote.drain().expect("drain attach snapshot");

        let inbound = foreign_operation_envelope("peer", DemoOperation::SetN { n: 7 });
        let operation_id = inbound.id.0.clone();
        remote
            .push(BackboneMessage::Operations { envelopes: vec![inbound] })
            .expect("push inbound operations");
        store.tick().expect("tick");
        assert_eq!(store.projection().expect("projection").n, 7, "ingested the inbound operation");

        let outbound = remote.drain().expect("drain ack");
        assert!(
            outbound
                .iter()
                .any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![operation_id.clone()])),
            "successful operations ingest emits an Ack for the ingested operation ids: {outbound:?}"
        );
    }

    #[test]
    fn exact_base_only_undo_refuses_a_foreign_tail() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        store
            .ingest_remote(foreign_operation_envelope("peer", DemoOperation::SetN { n: 2 }))
            .expect("ingest foreign");
        assert_eq!(store.projection().expect("projection").n, 2, "foreign edit sits at the tail");

        let error = store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::ExactBaseOnly,
                semantic_command: None,
            })
            .expect_err("undo must refuse a foreign tail");
        assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
        assert_eq!(store.projection().expect("projection").n, 2, "the timeline is untouched after refusal");
    }

    #[test]
    fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("local apply");
        let local_edit_id = store.applied_edit_ids()[0].clone();
        let foreign = foreign_operation_envelope("peer", DemoOperation::SetN { n: 2 });
        let foreign_id = foreign.id.0.clone();
        store.ingest_remote(foreign).expect("ingest foreign");
        assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

        store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::TransformAgainstConcurrent,
                semantic_command: None,
            })
            .expect("transform undo removes the local edit from mid-timeline");
        assert_eq!(
            store.applied_edit_ids(),
            std::slice::from_ref(&foreign_id),
            "only the local edit is removed; the concurrent foreign edit stays applied"
        );
        assert_eq!(
            store.redo_edit_ids(),
            std::slice::from_ref(&local_edit_id),
            "the local edit is on the redo stack"
        );
        assert_eq!(store.projection().expect("projection").n, 2, "projection re-materializes from the foreign edit alone");

        store.dispatch(DocumentCommand::Redo).expect("redo brings the local edit back");
        assert_eq!(store.applied_edit_ids().len(), 2);
        assert_eq!(store.projection().expect("projection").n, 1, "redo re-applies the local edit at the tail");
    }

    #[test]
    fn compensating_undo_dispatches_semantic_command() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let undo_apply = serde_json::to_string(&DocumentCommand::Apply {
            operations: vec![DemoOperation::SetN { n: 0 }],
            description: Some("compensate".into()),
        })
        .expect("serialize undo apply");
        store
            .dispatch(DocumentCommand::UndoWithPolicy {
                policy: UndoPolicy::CompensatingAction,
                semantic_command: Some(undo_apply),
            })
            .expect("compensating undo");
        assert_eq!(store.projection().expect("projection").n, 0);
    }

    #[test]
    fn edit_operations_exposes_the_latest_edit() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert!(store.edit_operations().is_none(), "no edits yet");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply");
        let (forwards, backwards, meta) = store.edit_operations().expect("edit operations");
        assert_eq!(forwards, &[DemoOperation::SetN { n: 5 }]);
        assert_eq!(backwards, &[DemoOperation::SetN { n: 0 }], "backwards restores the pre-state");
        assert_eq!(meta.len(), 1);
    }

    #[test]
    fn amend_last_absorbs_into_matching_coalesce_key() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("first amend");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("second amend");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced into a single edit");
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("projection after undo").n,
            0,
            "undo restores pre-gesture state in one step"
        );
    }

    #[test]
    fn amend_last_incremental_path_matches_full_replay_over_many_amends() {
        // 🪢️ Regression guard for the incremental `AmendLast` path (see `AmendCache`): many sequential
        // amends into the same coalesced edit — e.g. a long slider drag — must still produce exactly the
        // same edit (forwards/backwards/operation_meta length, final projection, one-step undo) as the
        // previous full-replay-every-time implementation, just without re-replaying history each time.
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        for n in 1..=50 {
            store
                .dispatch(DocumentCommand::AmendLast {
                    operations: vec![DemoOperation::SetN { n }],
                    coalesce_key: Some("drag".into()),
                })
                .expect("amend");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still a single coalesced edit");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        assert_eq!(edit.forwards.len(), 50);
        assert_eq!(edit.backwards.len(), 50);
        assert_eq!(edit.operation_meta.len(), 50);
        assert_eq!(store.projection().expect("projection").n, 50);
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("projection after undo").n,
            0,
            "one undo reverts the whole 50-step coalesced gesture"
        );
    }

    #[test]
    fn amend_last_incremental_cache_survives_undo_redo_round_trip() {
        // 🪢️ Undo/redo only move edit ids between `applied_edit_ids`/`redo_edit_ids` — they never mutate
        // an edit's own `forwards`, so a cached post-projection keyed by `(edit_id, forwards_len)` stays
        // valid across an undo immediately followed by a redo of the very same coalesced edit.
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("first amend");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        store.dispatch(DocumentCommand::Redo).expect("redo");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend after undo/redo");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "still coalesced into the original edit");
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Undo).expect("undo again");
        assert_eq!(store.projection().expect("projection after undo").n, 0);
    }

    #[test]
    fn amend_last_starts_new_edit_when_coalesce_key_differs() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag-a".into()),
            })
            .expect("first drag");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag-b".into()),
            })
            .expect("second drag");
        assert_eq!(store.envelope().vcs.edits.len(), 2, "distinct gestures are separate edits");
    }

    #[test]
    fn amend_last_does_not_absorb_into_committed_edit() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 1 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: None,
                authors: Vec::new(),
            })
            .expect("commit");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 2 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend after commit");
        assert_eq!(
            store.envelope().vcs.edits.len(),
            2,
            "committed edits are never amended, even with a matching coalesce key"
        );
    }

    #[test]
    fn test_support_round_trip_helpers_pass_for_demo_operation() {
        test_support::assert_operation_round_trip(&DemoProjection { n: 4 }, DemoOperation::SetN { n: 9 });
        test_support::assert_store_roundtrip(DemoProjection { n: 4 }, DemoOperation::SetN { n: 9 });

        let edit = Edit::<DemoOperation> {
            id: "edit-command-envelope".into(),
            actor: Some("actor-fallback".into()),
            forwards: vec![DemoOperation::SetN { n: 9 }],
            backwards: vec![DemoOperation::SetN { n: 4 }],
            operation_meta: vec![OperationMeta {
                operation_id: Some(OperationId("op-a".into())),
                dependencies: vec![OperationId("op-0".into())],
                base_version: 0,
                author_id: Some(ActorId("actor-explicit".into())),
                timestamp: protocol::HybridLogicalTimestamp::new(1, 1000),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
            }],
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoProjection, DemoOperation>(&edit, &DocumentId("doc-command-envelope".into()));
    }

    /// @emoji 🪤️ Proves `assert_command_envelope_round_trip` is not a trivially-true check: a hand-rolled
    /// `Operation` whose `Deserialize` impl silently drops its own field (encodes `n` faithfully but
    /// always decodes to `n: 0`) must trip law (2) of the doc comment on
    /// `assert_command_envelope_round_trip` — the same "deliberately lossy impl" pattern
    /// `protocol_testkit`'s `op_text_round_trip_panics_on_a_lossy_impl` uses for `assert_op_text_round_trip`.
    #[test]
    #[should_panic(expected = "did not decode back into an equal forward operation")]
    fn command_envelope_round_trip_panics_on_a_lossy_operation() {
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        struct LossyDiff;

        impl OperationDiff<DemoProjection> for LossyDiff {
            fn apply(&self, projection: &DemoProjection) -> DemoProjection {
                projection.clone()
            }
            fn absorb(&mut self, _other: Self) {}
        }

        #[derive(Clone, Debug, PartialEq)]
        struct LossyOperation {
            n: i32,
        }

        impl Serialize for LossyOperation {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_i32(self.n)
            }
        }

        impl<'de> Deserialize<'de> for LossyOperation {
            fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
                Ok(LossyOperation { n: 0 })
            }
        }

        impl Operation<DemoProjection> for LossyOperation {
            type Diff = LossyDiff;
            fn diff(&self, _projection: &DemoProjection) -> LossyDiff {
                LossyDiff
            }
            fn backwards(&self, _projection: &DemoProjection) -> Vec<Self> {
                vec![self.clone()]
            }
        }

        let edit = Edit::<LossyOperation> {
            id: "edit-lossy".into(),
            actor: None,
            forwards: vec![LossyOperation { n: 7 }],
            backwards: vec![],
            operation_meta: vec![],
            description: None,
            coalesce_key: None,
            sequence_number: 0,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        test_support::assert_command_envelope_round_trip::<DemoProjection, LossyOperation>(&edit, &DocumentId("doc-lossy".into()));
    }

    // `DemoProjection`'s `store::DocumentDsl` impl and `DemoOperation`'s `store::OpText` impl are now
    // generated by `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type definitions
    // themselves (see `DemoProjection`/`DemoOperation` above) — the `dsl_schema` grammar replaces
    // this crate's own hand-rolled `"n <value>"`/`"set-n <value>"` printer/parser.

    #[test]
    fn demo_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&DemoProjection { n: 42 });
    }

    #[test]
    fn demo_dsl_pack_equivalence() {
        test_support::assert_dsl_pack_equivalence(&DemoProjection { n: 42 });
    }

    #[test]
    fn demo_op_text_round_trips() {
        test_support::assert_op_line_round_trip(&DemoOperation::SetN { n: 7 });
    }

    #[test]
    fn print_edit_lines_emits_one_indented_line_per_forward_op() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply");
        let edit = store.envelope().vcs.edits.last().expect("edit");
        let printed = print_edit_lines(edit).expect("print edit lines");
        assert!(printed.starts_with("edit "), "got {printed:?}");
        assert!(printed.contains("\n  set-n n=1\n"));
    }

    #[test]
    fn document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: Some("bump".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            })
            .expect("commit");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn parse_document_text_rejects_invalid_op_line_with_span() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "doc demo schema=demo/v1\nedit e1 started=\"1\"\n  not-an-op\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert_eq!(error.span.line, 3);
    }

    /// @emoji 🩺️ Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, amend
    /// gestures, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
    /// full-replay differential oracle, so any divergence between the incremental paths and a
    /// from-scratch replay fails loudly here rather than surfacing as a silent projection bug later.
    #[test]
    fn stateful_current_matches_full_replay_across_interleaved_commands() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);

        // Multi-operation edit: current must fold both ops, matching a from-scratch replay.
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }, DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply multi-op edit");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 2);

        // Amend gesture: the first `AmendLast` cannot merge into the preceding `Apply`-created edit
        // (`Apply` never sets a `coalesce_key`, so it can never match), so it starts a NEW edit; the
        // second `AmendLast` shares that edit's key and merges into it — two edits total, the second
        // one carrying two coalesced increments (3 then 4).
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 3 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend 1");
        store
            .dispatch(DocumentCommand::AmendLast {
                operations: vec![DemoOperation::SetN { n: 4 }],
                coalesce_key: Some("drag".into()),
            })
            .expect("amend 2");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);
        assert_eq!(store.envelope().vcs.edits.len(), 2, "the amend gesture started its own edit, not a third");

        // Undo the whole amended edit (O(1) tail-cache path) restores the `Apply`-edit's state, not
        // the initial projection — only the amend gesture's edit is undone here.
        store.dispatch(DocumentCommand::Undo).expect("undo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 2);
        store.dispatch(DocumentCommand::Redo).expect("redo");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);

        // Checkpoint (cold path through `checkout_checkpoint_internal` is NOT exercised by commit
        // itself, but a following apply + a second, older undo still must agree with replay).
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: Vec::new(),
            })
            .expect("commit");
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply after checkpoint");
        test_support::assert_live_equals_replay(&store);
        store.dispatch(DocumentCommand::Undo).expect("undo after checkpoint");
        test_support::assert_live_equals_replay(&store);
        assert_eq!(store.projection().expect("projection").n, 4);
    }

    //#region 🏛️StudioTests
    /// @emoji ⏱️ Like `DemoOperation` but with an explicit, test-controlled `timestamp()` override, so
    /// undo-ordering-by-HLT tests don't depend on real wall-clock resolution.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation")]
    enum TimestampedOperation {
        SetN { n: i32, physical_ms: u64 },
    }

    impl Operation<DemoProjection> for TimestampedOperation {
        type Diff = DemoDiff;

        fn diff(&self, _projection: &DemoProjection) -> DemoDiff {
            match self {
                TimestampedOperation::SetN { n, .. } => DemoDiff { n: Some(*n) },
            }
        }

        fn backwards(&self, projection: &DemoProjection) -> Vec<Self> {
            vec![TimestampedOperation::SetN {
                n: projection.n,
                physical_ms: 0,
            }]
        }

        fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
            match self {
                TimestampedOperation::SetN { physical_ms, .. } => Some(protocol::HybridLogicalTimestamp::new(0, *physical_ms)),
            }
        }
    }

    /// @emoji 🪄️ Downcasts a registered `dyn StudioMember` back to its concrete demo store.
    fn demo_member<'a, Operation: crate::Operation<DemoProjection> + 'static>(
        host: &'a mut StudioHost,
        document_id: &str,
    ) -> &'a mut DocumentStore<DemoProjection, Operation> {
        host.member_mut(document_id)
            .expect("member registered")
            .as_any_mut()
            .downcast_mut::<DocumentStore<DemoProjection, Operation>>()
            .expect("concrete member type matches")
    }

    #[test]
    fn studio_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
        let mut member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        member_a
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply a");

        let mut member_b = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-b",
            DemoProjection { n: 0 },
            None,
        ));
        member_b
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 5 }],
                description: None,
            })
            .expect("apply b");
        member_b
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("b-init".into()),
                authors: Vec::new(),
            })
            .expect("commit b upfront, so it starts clean");
        let member_b_checkpoint = member_b.current_checkpoint_id().expect("b checkpoint").to_string();

        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));
        host.register_member(Box::new(member_b));

        let studio_checkpoint_id = host
            .commit_studio_checkpoint(
                "studio init".into(),
                vec![Author {
                    id: "a1".into(),
                    name: "Alice".into(),
                    avatar: None,
                }],
            )
            .expect("commit studio checkpoint");

        let projection = host.meta_projection().expect("meta projection");
        assert_eq!(projection.checkpoints.len(), 1);
        let checkpoint = &projection.checkpoints[0];
        assert_eq!(checkpoint.id, studio_checkpoint_id);
        assert_eq!(checkpoint.members.len(), 2, "pins one entry per registered member");
        let pin_b = checkpoint.members.iter().find(|pin| pin.document_id == "member-b").expect("pin b");
        assert_eq!(pin_b.checkpoint_id, member_b_checkpoint, "clean member reuses its existing checkpoint");
        assert!(
            !host.member("member-a").expect("member a").is_dirty(),
            "dirty member-a is committed (and therefore clean) by the studio checkpoint"
        );
    }

    #[test]
    fn studio_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
        let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b");
        let meta_envelope: DocumentEnvelope<StudioHistoryProjection, StudioHistoryOperation> =
            create_document_envelope("os.studio.history/v1", "studio", StudioHistoryProjection::default(), None);
        let mut host_a = StudioHost::new(meta_envelope.clone());
        let mut host_b = StudioHost::new(meta_envelope);
        assert!(host_a.backbone_ref().is_none(), "default is unattached, like any other DocumentStore");

        host_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        host_b.attach_backbone(Box::new(backbone_b)).expect("attach b");
        assert!(host_a.backbone_ref().is_some());

        let mut member = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        member
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply on member, so it's dirty and can be committed");
        host_a.register_member(Box::new(member));
        host_a
            .commit_studio_checkpoint("studio init".into(), Vec::new())
            .expect("commit studio checkpoint on a");

        host_b.tick().expect("tick b");
        assert_eq!(
            host_b.meta_projection().expect("meta projection b").checkpoints.len(),
            1,
            "the studio-wide checkpoint replicates through the meta-document's backbone"
        );

        host_a.detach_backbone();
        assert!(host_a.backbone_ref().is_none());
        host_a
            .commit_studio_checkpoint("studio offline".into(), Vec::new())
            .expect("meta history keeps working purely in memory once detached");
        host_b.tick().expect("tick b again");
        assert_eq!(
            host_b.meta_projection().expect("meta projection b unchanged").checkpoints.len(),
            1,
            "detached studio edits never reach the peer"
        );
    }

    #[test]
    fn studio_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
        let member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply 1");
        let studio_checkpoint_1 = host.commit_studio_checkpoint("first".into(), Vec::new()).expect("commit 1");

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2");
        host.commit_studio_checkpoint("second".into(), Vec::new()).expect("commit 2");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            2,
            "member reflects the second studio checkpoint before checking out the first"
        );

        host.checkout_studio_checkpoint(&studio_checkpoint_1).expect("checkout studio checkpoint 1");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            1,
            "checking out the first studio checkpoint fans out and restores member-a's pinned state"
        );
    }

    #[test]
    fn studio_switch_alternative_fans_out_and_restores_pinned_member_state() {
        let member_a = DocumentStore::new(create_document_envelope::<DemoProjection, DemoOperation>(
            "demo/v1",
            "member-a",
            DemoProjection { n: 0 },
            None,
        ));
        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_a));

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: None,
            })
            .expect("apply 1");
        host.commit_studio_checkpoint("root".into(), Vec::new()).expect("commit root");

        let alt_id = host.create_studio_alternative("branch-a".into()).expect("create alternative");

        demo_member::<DemoOperation>(&mut host, "member-a")
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 2 }],
                description: None,
            })
            .expect("apply 2 (uncommitted at the studio level)");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            2,
            "uncommitted edit is live before switching"
        );

        host.switch_studio_alternative(&alt_id).expect("switch alternative fans out to its pinned checkpoint");
        assert_eq!(
            demo_member::<DemoOperation>(&mut host, "member-a").projection().expect("projection").n,
            1,
            "switching alternatives restores each member to its pinned checkpoint, discarding the uncommitted edit"
        );
    }

    #[test]
    fn studio_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
        let mut member_early = DocumentStore::new(create_document_envelope::<DemoProjection, TimestampedOperation>(
            "demo-ts/v1",
            "member-early",
            DemoProjection { n: 0 },
            None,
        ));
        member_early
            .dispatch(DocumentCommand::Apply {
                operations: vec![TimestampedOperation::SetN { n: 1, physical_ms: 1_000 }],
                description: None,
            })
            .expect("apply early");

        let mut member_late = DocumentStore::new(create_document_envelope::<DemoProjection, TimestampedOperation>(
            "demo-ts/v1",
            "member-late",
            DemoProjection { n: 0 },
            None,
        ));
        member_late
            .dispatch(DocumentCommand::Apply {
                operations: vec![TimestampedOperation::SetN { n: 9, physical_ms: 2_000 }],
                description: None,
            })
            .expect("apply late");

        let mut host = StudioHost::new(create_document_envelope(
            "os.studio.history/v1",
            "studio",
            StudioHistoryProjection::default(),
            None,
        ));
        host.register_member(Box::new(member_early));
        host.register_member(Box::new(member_late));

        host.undo().expect("studio undo targets the member with the higher HLT");
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-early").projection().expect("early projection").n,
            1,
            "earlier local edit (lower HLT) is untouched"
        );
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-late").projection().expect("late projection").n,
            0,
            "later local edit (higher HLT) is the one undone"
        );

        host.redo().expect("studio redo targets the most recently undone edit");
        assert_eq!(
            demo_member::<TimestampedOperation>(&mut host, "member-late").projection().expect("late projection after redo").n,
            9,
            "redo restores the member's most recently undone edit"
        );
    }

    #[test]
    fn default_reconcile_hook_is_a_no_op_for_existing_document_kinds() {
        let projection = DemoProjection { n: 4 };
        let (reconciled, conflicts) = DemoOperation::SetN { n: 4 }.reconcile(projection.clone());
        assert_eq!(reconciled, projection, "default reconcile leaves the projection untouched");
        assert!(conflicts.is_empty(), "default reconcile reports no conflicts");

        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 3 }],
                description: None,
            })
            .expect("apply");
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed.n, 3, "materialize_document_projection is unaffected by the no-operation default reconcile hook");
        let (with_conflicts, conflicts) = store.projection_with_conflicts().expect("projection with conflicts");
        assert_eq!(with_conflicts.n, 3);
        assert!(conflicts.is_empty());
        assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
    }

    #[test]
    fn studio_history_op_round_trips() {
        let checkpoint = StudioCheckpoint {
            id: "sc-1".into(),
            parent_id: None,
            message: "root".into(),
            authors: Vec::new(),
            timestamp: HybridLogicalTimestamp::new(0, 1),
            members: vec![StudioMemberPin {
                document_id: "member-a".into(),
                checkpoint_id: "cp-1".into(),
                alternative_id: String::new(),
            }],
        };
        test_support::assert_operation_round_trip(
            &StudioHistoryProjection::default(),
            StudioHistoryOperation::CommitStudioCheckpoint {
                checkpoint: checkpoint.clone(),
            },
        );

        let with_checkpoint = StudioHistoryProjection {
            checkpoints: vec![checkpoint],
            alternatives: Vec::new(),
            active_alternative_id: None,
        };
        let alternative = StudioAlternative {
            id: "sa-1".into(),
            name: "branch".into(),
            checkpoint_ids: vec!["sc-1".into()],
        };
        test_support::assert_operation_round_trip(
            &with_checkpoint,
            StudioHistoryOperation::CreateStudioAlternative { alternative },
        );

        let with_alternative_active = StudioHistoryProjection {
            active_alternative_id: Some("sa-1".into()),
            ..with_checkpoint
        };
        test_support::assert_operation_round_trip(
            &with_alternative_active,
            StudioHistoryOperation::SwitchStudioAlternative {
                alternative_id: "sa-other".into(),
            },
        );
    }

    //#endregion 🏛️StudioTests

    //#region 🔖️TextFormatHelpers
    #[test]
    fn ops_author_conversion_drops_avatar_matching_the_ops_text_format() {
        let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
        let round_tripped: Author = OpsAuthor::from(&author).into();
        assert_eq!(round_tripped, Author { id: "a1".into(), name: "Alice".into(), avatar: None }, "OpsAuthor never carries avatar — it is not part of the .ops text format");
    }

    #[test]
    fn ops_header_line_checkpoint_round_trips_including_delimiter_and_quote_characters_in_authors() {
        let header = OpsHeaderLine::Checkpoint {
            id: "c1".to_string(),
            at: "18".to_string(),
            changes: vec!["ch1".to_string(), "ch2".to_string()],
            parent: None,
            by: vec![
                OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string() },
                OpsAuthor { id: "b2".to_string(), name: "Bob".to_string() },
            ],
            message: Some("first \"checkpoint\"".to_string()),
        };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Checkpoint round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_edit_round_trips_including_a_quoted_description() {
        let header = OpsHeaderLine::Edit {
            id: "e1".to_string(),
            started: "1".to_string(),
            actor: None,
            finished: None,
            key: None,
            description: Some("hello \"world\"".to_string()),
        };
        let printed = header.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
        assert!(!printed.contains("actor="), "an absent optional field must be omitted: {printed}");
        let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
        assert_eq!(parsed, header, "OpsHeaderLine::Edit round trip diverged for {printed:?}");
    }

    #[test]
    fn ops_header_line_parse_op_rejects_a_line_with_no_known_keyword() {
        let error = OpsHeaderLine::parse_op("not a structural line").unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
    }

    #[test]
    fn parse_document_text_rejects_a_header_line_missing_its_required_positional_id() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "active\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("expected Text"), "got {error:?}");
        assert_eq!(error.span.line, 1);
    }

    #[test]
    fn parse_document_text_rejects_an_unknown_header_line_keyword() {
        let files = DocumentTextFiles {
            dsl: "n=0\n".to_string(),
            ops: "doc demo schema=demo/v1\nbogus id=x\n".to_string(),
        };
        let error = parse_document_text::<DemoProjection, DemoOperation>(&files.dsl, &files.ops).unwrap_err();
        assert!(error.message.contains("unknown operation line"), "got {error:?}");
        assert_eq!(error.span.line, 2);
    }

    #[test]
    fn document_text_round_trips_with_an_active_alternative_and_a_quoted_description() {
        let envelope = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![DemoOperation::SetN { n: 1 }],
                description: Some("said \"hi\" and used a \\ backslash".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "branch \"a\"".into() })
            .expect("create alternative (auto-commits and activates it)");
        assert!(store.envelope().active_alternative_id.is_some(), "precondition: an alternative is active");
        let files = print_document_text(store.envelope()).expect("print document text");
        assert!(files.ops.lines().any(|line| line.starts_with("active ")), "an active alternative must print an `active` header line: {}", files.ops);
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }

    //#endregion 🔖️TextFormatHelpers

    //#region 🔖️CommandErrorPaths
    #[test]
    fn apply_with_no_operations_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::Apply { operations: Vec::new(), description: None })
            .unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn amend_last_with_no_operations_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::AmendLast { operations: Vec::new(), coalesce_key: None })
            .unwrap_err();
        assert_eq!(error, VcsError::EmptyApply);
    }

    #[test]
    fn undo_with_nothing_applied_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert_eq!(store.dispatch(DocumentCommand::Undo).unwrap_err(), VcsError::NothingToUndo);
    }

    #[test]
    fn redo_with_nothing_undone_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        assert_eq!(store.dispatch(DocumentCommand::Redo).unwrap_err(), VcsError::NothingToRedo);
    }

    #[test]
    fn checkout_of_an_unknown_checkpoint_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: "nope".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::UnknownChange("nope".into()));
    }

    #[test]
    fn switch_to_an_unknown_alternative_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::SwitchAlternative { alternative_id: "nope".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::UnknownAlternative("nope".into()));
    }

    #[test]
    fn switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected() {
        let mut envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        envelope.vcs.alternatives.push(Alternative {
            id: "alt-dangling".into(),
            name: "dangling".into(),
            checkpoint_ids: vec!["checkpoint-that-was-never-recorded".into()],
        });
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::SwitchAlternative { alternative_id: "alt-dangling".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the alternative's pinned checkpoint id must actually exist");
    }

    #[test]
    fn create_alternative_with_no_edits_and_no_checkpoints_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let error = store
            .dispatch(DocumentCommand::CreateAlternative { name: "x".into() })
            .unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint, "the auto-commit has nothing pending, so there is still no checkpoint to branch from");
    }

    #[test]
    fn compensating_undo_without_a_semantic_command_is_rejected() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        let error = store
            .dispatch(DocumentCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: None })
            .unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    }

    #[test]
    fn materialize_document_projection_rejects_an_unknown_edit_id() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let error = materialize_document_projection(&envelope, &["missing-edit".to_string()]).unwrap_err();
        assert_eq!(error, VcsError::UnknownEdit("missing-edit".into()));
    }

    #[test]
    fn dispatch_json_applies_a_serialized_command_and_projection_json_reflects_it() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        let command_json = serde_json::to_string(&DocumentCommand::Apply {
            operations: vec![DemoOperation::SetN { n: 7 }],
            description: None,
        })
        .expect("serialize command");
        store.dispatch_json(&command_json).expect("dispatch json");
        assert_eq!(store.projection_json().expect("projection json"), serde_json::to_string(&DemoProjection { n: 7 }).unwrap());

        let error = store.dispatch_json("not json").unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(_)), "got {error:?}");
    }

    //#endregion 🔖️CommandErrorPaths

    //#region 🔖️ReconcileAlternative
    #[test]
    fn reconcile_alternative_requires_an_existing_checkpoint() {
        let mut envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new()).unwrap_err();
        assert_eq!(error, VcsError::NoCheckpoint);
    }

    #[test]
    fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() })
            .expect("commit");
        let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

        let mut without_message = store.envelope().clone();
        let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new()).expect("reconcile without message");
        assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
        assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
        assert!(!alt_id.is_empty());

        let mut with_message = store.envelope().clone();
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
        reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone())
            .expect("reconcile with message");
        assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
        let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
        assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
        assert_eq!(recorded_checkpoint.authors, authors);
        assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
        assert_eq!(
            with_message.vcs.changes.last().unwrap().description,
            Some("merged concurrent work".into()),
            "the passed checkpoint_message becomes the change's description"
        );
    }

    #[test]
    fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply 1");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }).expect("commit 1");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None }).expect("apply 2");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }).expect("commit 2");

        let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
        assert!(ids.iter().all(|id| id.starts_with("ck-")));
    }

    #[test]
    fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply root");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit root");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        store.dispatch(DocumentCommand::CreateAlternative { name: "feature-a".into() }).expect("create feature-a");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None }).expect("apply a");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).expect("commit a1");
        let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        store.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }).expect("checkout root");
        store.dispatch(DocumentCommand::CreateAlternative { name: "feature-b".into() }).expect("create feature-b");
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 3 }], description: None }).expect("apply b");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).expect("commit b1");
        let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

        assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id), Some(root_id.clone()), "a1 and b1 forked at root");
        assert_eq!(merge_base(store.envelope(), &a1_id, &root_id), Some(root_id.clone()), "root is its own descendant's merge-base");
        assert_eq!(merge_base(store.envelope(), &root_id, &root_id), Some(root_id), "a checkpoint is its own merge-base");
    }

    #[test]
    fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> = create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store.dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None }).expect("apply");
        store.dispatch(DocumentCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).expect("commit");
        let root_id = store.envelope().vcs.checkpoints[0].id.clone();

        assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint"), None, "an id absent from the checkpoint list shares no ancestry with anything");
    }

    //#endregion 🔖️ContentAddressedCheckpointAndMergeBase

    //#region 🔖️RemoteSnapshotMerge
    #[test]
    fn snapshot_merge_into_a_nonempty_store_adds_only_the_new_remote_edits_and_records() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("local apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() })
            .expect("local commit");

        let mut remote_store = DocumentStore::new(store.envelope().clone());
        remote_store.set_state(store.envelope().clone(), store.applied_edit_ids().to_vec(), Vec::new());
        remote_store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None })
            .expect("remote apply");
        remote_store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() })
            .expect("remote commit");

        let (channel, remote_end) = ChannelBackbone::pair("chan");
        store.attach_backbone(Box::new(channel)).expect("attach");
        let _ = remote_end.drain().expect("drain attach snapshot");
        remote_end
            .push(BackboneMessage::Snapshot { envelope_json: remote_store.envelope_json().expect("remote json") })
            .expect("push snapshot");
        store.tick().expect("tick merges the pushed snapshot");

        assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduped, only the new remote edit is added");
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's new checkpoint is merged in by id");
        assert_eq!(store.projection().expect("projection").n, 2, "current folds in the newly merged edit's forwards");
    }

    //#endregion 🔖️RemoteSnapshotMerge

    //#region 🔖️StudioMemberCheckoutRouting
    #[test]
    fn studio_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
        let envelope: DocumentEnvelope<DemoProjection, DemoOperation> =
            create_document_envelope("demo/v1", "demo", DemoProjection { n: 0 }, None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 1 }], description: None })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CreateAlternative { name: "feature".into() })
            .expect("create alternative (auto-commits since no checkpoint existed yet)");
        let alt_id = store.envelope().vcs.alternatives[0].id.clone();
        let tip = store.envelope().vcs.alternatives[0].checkpoint_ids.last().expect("alt has a tip").clone();

        StudioMember::checkout(&mut store, &tip, &alt_id).expect("checkout at the tip routes through SwitchAlternative");
        assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

        store
            .dispatch(DocumentCommand::Apply { operations: vec![DemoOperation::SetN { n: 2 }], description: None })
            .expect("apply on branch");
        store
            .dispatch(DocumentCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() })
            .expect("commit c2, advancing the alt's tip past `tip`");

        StudioMember::checkout(&mut store, &tip, &alt_id).expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
        assert_eq!(store.projection().expect("projection").n, 1, "restored the old checkpoint's state");
        assert_eq!(
            store.envelope().active_alternative_id, None,
            "the checked-out checkpoint is no longer any alternative's tip, so nothing is active"
        );
    }

    //#endregion 🔖️StudioMemberCheckoutRouting

    //#region 🔖️BackbonePorts
    #[test]
    fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
        let port = MemoryBackbonePort::new();
        let error = port.read("file://nowhere").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("file://a", "payload-1").expect("write");
        assert_eq!(port.read("file://a").expect("read"), "payload-1");
        port.write("file://a", "payload-2").expect("overwrite");
        assert_eq!(port.read("file://a").expect("read after overwrite"), "payload-2", "write is an upsert");
    }

    #[test]
    fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
        let port = LocalStorageBackbonePort::new();
        let error = port.read("local://missing").unwrap_err();
        assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
        port.write("local://a", "value").expect("write falls back to the in-memory store");
        assert_eq!(port.read("local://a").expect("read falls back too"), "value");

        let defaulted = LocalStorageBackbonePort::default();
        assert!(defaulted.read("local://a").is_err(), "Default constructs its own independent fallback store");
    }

    //#endregion 🔖️BackbonePorts
}
//#endregion 🧪️Tests


