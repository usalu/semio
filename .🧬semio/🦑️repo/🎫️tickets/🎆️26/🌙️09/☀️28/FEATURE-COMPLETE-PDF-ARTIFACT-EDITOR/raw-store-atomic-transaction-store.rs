    /// 🖼️ Admits verified media mutation pages with a transaction minted by the actual Store actor and clock.
    pub fn admit_original_media_batch_with_pages(
        &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32],
        actor: &mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        pages: &mut Option<semio_os_store_one_item_publication::ArtifactMutationPages<Mutation>>,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        registered_app: &str, verb: Option<&str>, group: Option<&str>, grant: RetainedCloneGrant,
    ) -> Result<Option<(ArtifactStoreBatchPublication<P, Mutation>, RetainedCloneProgress)>, ValueError>
    where P: Sync, Mutation: Send {
        self.cursor.admit_original_media_batch_with_pages(&self.log, &self.owners, &self.wire, operation,
            expected_generation, expected_revision, actor, pages, factory, registered_app, verb, group, grant)
    }

