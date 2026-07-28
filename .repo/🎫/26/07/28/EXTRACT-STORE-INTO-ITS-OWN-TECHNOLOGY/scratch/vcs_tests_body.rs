    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct DemoItem {
        id: String,
        value: i32,
    }

    impl Identified<String> for DemoItem {
        fn id(&self) -> &String {
            &self.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    struct DemoItemPatch {
        value: Option<i32>,
    }

    impl Patchable<DemoItemPatch> for DemoItem {
        fn apply_patch(&mut self, patch: &DemoItemPatch) -> DemoItemPatch {
            let inverse = DemoItemPatch { value: Some(self.value) };
            if let Some(value) = patch.value {
                self.value = value;
            }
            inverse
        }
    }

    #[test]
    fn collection_diff_from_op_projects_each_variant() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
        ];
        let added = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(
            &items,
            &CollectionOperation::Add {
                index: 0,
                item: DemoItem { id: "c".into(), value: 3 },
            },
        );
        assert_eq!(added.added.len(), 1);
        assert!(added.removed.is_empty() && added.modified.is_empty());

        let removed = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(&items, &CollectionOperation::Remove { id: "a".into() });
        assert_eq!(removed.removed, vec!["a".to_string()]);

        let patched = collection_diff_from_operation(
            &items,
            &CollectionOperation::Patch {
                id: "b".into(),
                patch: DemoItemPatch { value: Some(9) },
            },
        );
        assert_eq!(patched.modified.len(), 1);
        assert_eq!(patched.modified[0].id, "b");

        let moved = collection_diff_from_operation::<String, DemoItem, DemoItemPatch>(&items, &CollectionOperation::Move { id: "a".into(), to_index: 1 });
        assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
        assert_eq!(moved.added.len(), 1);
        assert_eq!(moved.added[0].id, "a");
    }

    #[test]
    fn collection_op_add_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem {
            id: "a".into(),
            value: 1,
        }];
        let operation = CollectionOperation::Add {
            index: 1,
            item: DemoItem {
                id: "b".into(),
                value: 2,
            },
        };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[1].id, "b");
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_move_and_invert() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
            DemoItem { id: "c".into(), value: 3 },
        ];
        let operation = CollectionOperation::Move {
            id: "a".into(),
            to_index: 2,
        };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_patch_and_invert() {
        let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
        let operation = CollectionOperation::Patch {
            id: "a".into(),
            patch: DemoItemPatch { value: Some(9) },
        };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied[0].value, 9);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    #[test]
    fn collection_op_remove_and_invert() {
        let items: Vec<DemoItem> = vec![
            DemoItem { id: "a".into(), value: 1 },
            DemoItem { id: "b".into(), value: 2 },
        ];
        let operation = CollectionOperation::Remove { id: "a".into() };
        let mut applied = items.clone();
        apply_collection_operation(&mut applied, &operation);
        assert_eq!(applied.len(), 1);
        let inverse = invert_collection_operation(&items, &operation);
        apply_collection_operation(&mut applied, &inverse);
        assert_eq!(applied, items);
    }

    //#endregion 🔖ReconcileAlternative

    //#region 🔖ContentAddressedCheckpointAndMergeBase
    #[test]
    fn content_addressed_checkpoint_id_is_deterministic_and_content_sensitive() {
        let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
        let changes = vec![root_change];
        let change_ids = vec!["change-root".to_string()];
        let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];

        let id_a = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        let id_b = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        assert_eq!(id_a, id_b, "identical inputs converge on the identical id");
        assert!(id_a.starts_with("ck-"), "got {id_a}");

        let id_different_message = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("other message"), &authors, "2026-07-27T00:00:01Z");
        assert_ne!(id_a, id_different_message, "a different message must change the id");

        let id_different_parent = content_addressed_checkpoint_id(Some("ck-parent"), &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");
        assert_ne!(id_a, id_different_parent, "a different parent must change the id");

        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z");
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    }

