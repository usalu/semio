    pub fn admit_apply_batch_with_pages(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32],
        actor: &mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations: &mut Vec<Mutation>, pages: &mut Option<semio_os_store_one_item_publication::ArtifactMutationPages<Mutation>>, lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: &mut Option<protocol::TransactionRef>, verb: Option<&str>, group: Option<&str>, outbound: bool, grant: RetainedCloneGrant,
    ) -> Result<Option<(ArtifactStoreBatchPublication<P, Mutation>, RetainedCloneProgress)>, ValueError>
    where P: Sync, Mutation: Send {
        self.admit_apply_batch_for_purpose(store_log, store_owners, store_wire, operation, expected_generation, expected_revision,
            actor, mutations, pages, lane, factory, transaction, verb, group, outbound, None, grant)
    }

    /// 🪪️ Mints the canonical media transaction only for an admitted registered producer.
    pub fn admit_original_media_batch_with_pages(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32],
        actor: &mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        pages: &mut Option<semio_os_store_one_item_publication::ArtifactMutationPages<Mutation>>,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        registered_app: &str, verb: Option<&str>, group: Option<&str>, grant: RetainedCloneGrant,
    ) -> Result<Option<(ArtifactStoreBatchPublication<P, Mutation>, RetainedCloneProgress)>, ValueError>
    where P: Sync, Mutation: Send {
        self.admit_apply_batch_for_purpose(store_log, store_owners, store_wire, operation, expected_generation, expected_revision,
            actor, &mut Vec::new(), pages, HistoryLane::Document, factory, &mut None, verb, group, true, Some(registered_app), grant)
    }

