# Funded Preparation Factory Birth Census

Read-only exact current constructors; none is runtime qualification.

### ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs:129

```rust
    fn begin(&self, request: fixture_store::ArtifactStoreOneItemPreparationRequest<u8, u8>) -> Result<Box<dyn fixture_store::ArtifactStoreOneItemPreparation<u8, u8>>, fixture_store::ArtifactStoreOneItemPreparationRequest<u8, u8>> {
        Err(request)
    }
```

### ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/✏️editor/📬️preparation/🦀️.rs:70

```rust
    fn begin(&self, request: app_store::ArtifactStoreOneItemPreparationRequest<S, M>) -> Result<Box<dyn app_store::ArtifactStoreOneItemPreparation<S, M>>, app_store::ArtifactStoreOneItemPreparationRequest<S, M>> {
        let admitted = request.lane == app_store::HistoryLane::Document
            && request.operation == request.authority.operation()
            && request.generation == request.authority.generation()
            && request.base_revision == request.authority.base_revision()
            && request.authority.actor().len() <= app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            && (self.recognizes)(&request.mutation)
            && (self.preflight)(&request.mutation).is_ok_and(|bytes| bytes <= app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        if !admitted {
            return Err(request);
        }
        Ok(Box::new(StructuralPreparation {
            prefix: self.prefix,
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            copy: Some((self.copy)()),
            mutation_retirement: Arc::clone(&self.mutation_retirement),
            snapshot_retirement: Arc::clone(&self.snapshot_retirement),
            sealer: None,
            external_retirement: None,
            checkpoint: Default::default(),
            seal_base_checkpoint: None,
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs:579

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        let retained_bytes = space_retained_mutation_bytes(&request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err(request);
        }
        Ok(Box::new(SpaceOneItemPreparation {
            prefix: self.prefix,
            maximum_bytes: self.maximum_bytes,
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:394

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>> {
        let Some((mutation_bytes, maximum_bytes)) = home_config_retained_admission(&request.mutation) else {
            return Err(request);
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(HomeConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:691

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != self.lane
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(VcsOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:639

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Gis2dOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
            stamp: self.stamp.clone(),
        }))
    }
```

### ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:586

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SourcingCurationConfig, SourcingCurationConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation>> {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_CONFIG_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SourcingCurationConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false, closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:752

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CurationSnapshot, SourcingMutation>>, store::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation>> {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_DOCUMENT_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SourcingCurationArtifactPreparation {
            base: Some(request.base), mutation: Some(request.mutation), authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false, closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:846

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        let retained_bytes = forms_store_mutation_retained_bytes(&request.mutation).unwrap_or(FORMS_STORE_MUTATION_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > FORMS_STORE_MUTATION_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(FormsStorePreparation {
            prefix: self.prefix,
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:926

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(EquationStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
```

### ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:370

```rust
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(PlaybookOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:18080

```rust
        fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<C, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<C, M>>, store::ArtifactStoreOneItemPreparationRequest<C, M>> {
            let retained_bytes = bounded_config_mutation_retained_bytes(self.prefix, &request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
            if request.lane != HistoryLane::Document
                || request.operation != request.authority.operation()
                || request.generation != request.authority.generation()
                || request.base_revision != request.authority.base_revision()
                || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
                || retained_bytes > self.maximum_bytes
            {
                return Err(request);
            }
            Ok(Box::new(BoundedConfigPreparation::<C, M> {
                prefix: self.prefix,
                maximum_bytes: self.maximum_bytes,
                base: Some(request.base),
                mutation: Some(request.mutation),
                authority: Some(request.authority),
                prepared: None,
                checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
                retained_bytes,
                cancelled: false,
                closing: false,
            }))
        }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17915

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, Mutation>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, ArtifactStoreOneItemPreparationRequest<P, Mutation>> {
        self.as_ref().begin(request)
    }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17935

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire>> {
        self.as_ref().begin(request)
    }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:78

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<P, M>>, ArtifactStoreOneItemPreparationRequest<P, M>> {
        let footprint = match self.edit.preflight(&request.mutation, request.lane) {
            Ok(footprint) if footprint.is_admissible() => footprint,
            _ => return Err(request),
        };
        let ArtifactStoreOneItemPreparationRequest { operation: _, generation: _, base_revision: _, lane: _, authority, base, mutation } = request;
        Ok(Box::new(RetainedClonePreparation::<P, M, E> {
            source: Some(RetainedCloneSource::from_authority(Arc::clone(base.owner.as_ref().expect("live snapshot read owner is present")), base)),
            clone_cursor: Some(P::retained_clone_cursor()),
            clone_handoff: None,
            copied: None,
            edit_cursor: self.edit.begin(),
            inverse: None,
            mutation: Some(RetainedCloneSource::from_authority(Arc::new(mutation),Arc::clone(&authority))),
            authority: Some(authority),
            sealer: None,
            mutation_retirement: Some(Arc::clone(&self.mutation_retirement)),
            snapshot_retirement: Some(Arc::clone(&self.snapshot_retirement)),
            active_retirement: None,
            footprint,
            retained_capacity_bytes: 0,
            maximum_depth: self.maximum_depth,
            clone_turn: 0,
            zero_work_turns: 0,
            checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            seal_base: ArtifactStoreOneItemCheckpoint::default(),
            phase: RetainedClonePreparationPhase::Clone,
            cancelled: false,
            closing: false,
        }))
    }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs:175

```rust
    fn begin(&self, request: super::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn super::ArtifactStoreOneItemPreparation<P, M>>, super::ArtifactStoreOneItemPreparationRequest<P, M>> { self.factory.begin(request) }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:2450

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>> { self.inner.begin(request) }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:2450

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>> { self.inner.begin(request) }
```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:2450

```rust
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>) -> Result<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation>> { self.inner.begin(request) }
```
