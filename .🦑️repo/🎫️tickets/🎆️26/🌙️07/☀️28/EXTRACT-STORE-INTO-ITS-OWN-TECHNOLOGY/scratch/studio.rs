//#region 🔖️Studio
//#region StudioMember
/// @emoji 🧑️‍🤝️‍🧑️ Object-safe façade over a `DocumentStore<P, Operation>` so a studio host can hold a
/// heterogeneous registry of documents (`HashMap<String, Box<dyn StudioMember>>`) without knowing
/// each member's concrete `P`/`Operation`. Blanket-implemented below by delegating to `dispatch` — never
/// reimplement the underlying VCS mechanics here.
pub trait StudioMember {
    fn document_id(&self) -> &str;
    /// @emoji 🩸️ Whether this member has edits applied since its last checkpoint (mirrors the
    /// `CommitCheckpoint` dispatch's own "nothing to commit" check via `uncommitted_edit_ids`).
    fn is_dirty(&self) -> bool;
    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError>;
    fn current_checkpoint_id(&self) -> Option<String>;
    fn current_alternative_id(&self) -> Option<String>;
    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError>;
    fn create_alternative(&mut self, name: String) -> Result<String, VcsError>;
    // 🎞️ CW3: `protocol::HybridLogicalTimestamp` (not `semio_framework_core`'s local one) — these
    // read `OperationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    fn last_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp>;
    fn last_undone_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp>;
    fn undo(&mut self) -> Result<(), VcsError>;
    fn redo(&mut self) -> Result<(), VcsError>;
    /// @emoji 🪄️ Downcast escape hatch: a studio host UI (or a test) needs the concrete
    /// `DocumentStore<P, Operation>` back out of a `Box<dyn StudioMember>` — e.g. to `Apply` a
    /// technology-specific `Operation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Operation` pair, so this never fails for a genuine member.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<P, Operation> StudioMember for DocumentStore<P, Operation>
where
    P: Clone + Serialize + DeserializeOwned + 'static,
    Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P> + 'static,
{
    fn document_id(&self) -> &str {
        self.envelope().id.as_str()
    }

    fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.envelope, self.applied_edit_ids()).is_empty()
    }

    fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        self.dispatch(DocumentCommand::CommitCheckpoint {
            message: Some(message),
            authors,
        })?;
        // `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
        // trait method — Rust prefers inherent methods over trait methods of the same name.
        self.current_checkpoint_id().map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().map(|id| id.to_string())
    }

    fn current_alternative_id(&self) -> Option<String> {
        self.envelope().active_alternative_id.clone()
    }

    fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self
                .envelope()
                .vcs
                .alternatives
                .iter()
                .find(|alternative| alternative.id == alternative_id)
                .map(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id))
                .unwrap_or(false);
            if is_alternative_tip {
                return self.dispatch(DocumentCommand::SwitchAlternative {
                    alternative_id: alternative_id.to_string(),
                });
            }
        }
        self.dispatch(DocumentCommand::CheckoutCheckpoint {
            checkpoint_id: checkpoint_id.to_string(),
        })
    }

    fn create_alternative(&mut self, name: String) -> Result<String, VcsError> {
        self.dispatch(DocumentCommand::CreateAlternative { name })?;
        self.envelope().active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    fn last_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.applied_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope()
                .vcs
                .edits
                .iter()
                .find(|edit| edit.id == *edit_id)
                .and_then(|edit| edit.operation_meta.last())
                .map(|meta| meta.timestamp)
        })
    }

    fn last_undone_local_edit_timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.redo_edit_ids().iter().rev().find_map(|edit_id| {
            if !self.edit_is_local(edit_id) {
                return None;
            }
            self.envelope()
                .vcs
                .edits
                .iter()
                .find(|edit| edit.id == *edit_id)
                .and_then(|edit| edit.operation_meta.last())
                .map(|meta| meta.timestamp)
        })
    }

    fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Undo)
    }

    fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Redo)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
//#endregion StudioMember

//#region StudioHistoryDocument
/// @emoji 📌️ One member document's position at the moment a `StudioCheckpoint` was recorded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioMemberPin {
    pub document_id: String,
    pub checkpoint_id: String,
    /// @emoji 🌿️ Empty string when the member had no active alternative (its own trunk) at pin time.
    #[serde(default)]
    pub alternative_id: String,
}

/// @emoji 🗄️ A studio-wide checkpoint: one pin per registered member, so checking it out (or an
/// alternative built on top of it) fans out deterministically to every member's own VCS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioCheckpoint {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub message: String,
    pub authors: Vec<Author>,
    pub timestamp: HybridLogicalTimestamp,
    pub members: Vec<StudioMemberPin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// @emoji 🗄️ Projection of the `"os.studio.history"` meta-document: itself an ordinary `DocumentVcs`
/// document kind (dogfooded — no bespoke transport), holding the studio-level checkpoint/alternative
/// graph that `StudioHost` composes on top of every registered member's own history.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioHistoryProjection {
    pub checkpoints: Vec<StudioCheckpoint>,
    pub alternatives: Vec<StudioAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum StudioHistoryOperation {
    CommitStudioCheckpoint { checkpoint: StudioCheckpoint },
    CreateStudioAlternative { alternative: StudioAlternative },
    SwitchStudioAlternative { alternative_id: String },
    /// @emoji ↩️ Mechanical inverse of `CommitStudioCheckpoint`; never dispatched directly by
    /// `StudioHost` (studio undo is derived and member-local, see `StudioHost::undo`), only
    /// produced by `backwards` for VCS round-trip correctness.
    RemoveStudioCheckpoint { checkpoint_id: String },
    /// @emoji ↩️ Mechanical inverse of `CreateStudioAlternative`; see `RemoveStudioCheckpoint`.
    RemoveStudioAlternative { alternative_id: String },
    /// @emoji ↩️ Mechanical inverse of `SwitchStudioAlternative`; see `RemoveStudioCheckpoint`.
    SetActiveStudioAlternative { alternative_id: Option<String> },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioHistoryDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_checkpoint: Option<StudioCheckpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_alternative: Option<StudioAlternative>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_active_alternative_id: Option<Option<String>>,
}

impl OperationDiff<StudioHistoryProjection> for StudioHistoryDiff {
    fn apply(&self, projection: &StudioHistoryProjection) -> StudioHistoryProjection {
        let mut next = projection.clone();
        if let Some(checkpoint) = &self.add_checkpoint {
            next.checkpoints.push(checkpoint.clone());
        }
        if let Some(checkpoint_id) = &self.remove_checkpoint_id {
            next.checkpoints.retain(|checkpoint| checkpoint.id != *checkpoint_id);
        }
        if let Some(alternative) = &self.add_alternative {
            next.alternatives.push(alternative.clone());
        }
        if let Some(alternative_id) = &self.remove_alternative_id {
            next.alternatives.retain(|alternative| alternative.id != *alternative_id);
        }
        if let Some(active) = &self.set_active_alternative_id {
            next.active_alternative_id = active.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.add_checkpoint.is_some() {
            self.add_checkpoint = other.add_checkpoint;
        }
        if other.remove_checkpoint_id.is_some() {
            self.remove_checkpoint_id = other.remove_checkpoint_id;
        }
        if other.add_alternative.is_some() {
            self.add_alternative = other.add_alternative;
        }
        if other.remove_alternative_id.is_some() {
            self.remove_alternative_id = other.remove_alternative_id;
        }
        if other.set_active_alternative_id.is_some() {
            self.set_active_alternative_id = other.set_active_alternative_id;
        }
    }
}

impl Operation<StudioHistoryProjection> for StudioHistoryOperation {
    type Diff = StudioHistoryDiff;

    fn diff(&self, _projection: &StudioHistoryProjection) -> StudioHistoryDiff {
        match self {
            StudioHistoryOperation::CommitStudioCheckpoint { checkpoint } => StudioHistoryDiff {
                add_checkpoint: Some(checkpoint.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::CreateStudioAlternative { alternative } => StudioHistoryDiff {
                add_alternative: Some(alternative.clone()),
                set_active_alternative_id: Some(Some(alternative.id.clone())),
                ..Default::default()
            },
            StudioHistoryOperation::SwitchStudioAlternative { alternative_id } => StudioHistoryDiff {
                set_active_alternative_id: Some(Some(alternative_id.clone())),
                ..Default::default()
            },
            StudioHistoryOperation::RemoveStudioCheckpoint { checkpoint_id } => StudioHistoryDiff {
                remove_checkpoint_id: Some(checkpoint_id.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::RemoveStudioAlternative { alternative_id } => StudioHistoryDiff {
                remove_alternative_id: Some(alternative_id.clone()),
                ..Default::default()
            },
            StudioHistoryOperation::SetActiveStudioAlternative { alternative_id } => StudioHistoryDiff {
                set_active_alternative_id: Some(alternative_id.clone()),
                ..Default::default()
            },
        }
    }

    fn backwards(&self, projection: &StudioHistoryProjection) -> Vec<Self> {
        match self {
            StudioHistoryOperation::CommitStudioCheckpoint { checkpoint } => {
                vec![StudioHistoryOperation::RemoveStudioCheckpoint {
                    checkpoint_id: checkpoint.id.clone(),
                }]
            }
            StudioHistoryOperation::CreateStudioAlternative { alternative } => vec![
                StudioHistoryOperation::SetActiveStudioAlternative {
                    alternative_id: projection.active_alternative_id.clone(),
                },
                StudioHistoryOperation::RemoveStudioAlternative {
                    alternative_id: alternative.id.clone(),
                },
            ],
            StudioHistoryOperation::SwitchStudioAlternative { .. } => vec![StudioHistoryOperation::SetActiveStudioAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
            StudioHistoryOperation::RemoveStudioCheckpoint { checkpoint_id } => projection
                .checkpoints
                .iter()
                .find(|checkpoint| checkpoint.id == *checkpoint_id)
                .map(|checkpoint| vec![StudioHistoryOperation::CommitStudioCheckpoint { checkpoint: checkpoint.clone() }])
                .unwrap_or_default(),
            StudioHistoryOperation::RemoveStudioAlternative { alternative_id } => projection
                .alternatives
                .iter()
                .find(|alternative| alternative.id == *alternative_id)
                .map(|alternative| vec![StudioHistoryOperation::CreateStudioAlternative { alternative: alternative.clone() }])
                .unwrap_or_default(),
            StudioHistoryOperation::SetActiveStudioAlternative { .. } => vec![StudioHistoryOperation::SetActiveStudioAlternative {
                alternative_id: projection.active_alternative_id.clone(),
            }],
        }
    }
}
//#endregion StudioHistoryDocument

//#region StudioHost
/// @emoji 🏛️ Composes many `StudioMember` documents under one studio-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `"os.studio.history"` meta-document. App-agnostic: this
/// crate has no notion of what a member document *is*, only that it satisfies `StudioMember`.
pub struct StudioHost {
    meta: DocumentStore<StudioHistoryProjection, StudioHistoryOperation>,
    members: HashMap<String, Box<dyn StudioMember>>,
}

impl StudioHost {
    pub fn new(meta_envelope: DocumentEnvelope<StudioHistoryProjection, StudioHistoryOperation>) -> Self {
        Self {
            meta: DocumentStore::new(meta_envelope),
            members: HashMap::new(),
        }
    }

    pub fn register_member(&mut self, member: Box<dyn StudioMember>) {
        self.members.insert(member.document_id().to_string(), member);
    }

    pub fn unregister_member(&mut self, document_id: &str) -> Option<Box<dyn StudioMember>> {
        self.members.remove(document_id)
    }

    pub fn member(&self, document_id: &str) -> Option<&dyn StudioMember> {
        self.members.get(document_id).map(|member| member.as_ref())
    }

    pub fn member_mut<'a>(&'a mut self, document_id: &str) -> Option<&'a mut (dyn StudioMember + 'a)> {
        match self.members.get_mut(document_id) {
            Some(member) => Some(member.as_mut()),
            None => None,
        }
    }

    pub fn meta_projection(&self) -> Result<StudioHistoryProjection, VcsError> {
        self.meta.projection()
    }

    /// @emoji 🔗️ Attaches a backbone to the studio-wide meta-document, same runtime-attach/detach
    /// contract as any other `DocumentStore` — default is unattached, this is always an
    /// explicit call.
    pub fn attach_backbone(&mut self, backbone: Box<dyn Backbone>) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone)
    }

    /// @emoji ✂️ Detaches the meta-document's backbone; the studio history stays in memory.
    pub fn detach_backbone(&mut self) -> Option<Box<dyn Backbone>> {
        self.meta.detach_backbone()
    }

    pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
        self.meta.backbone_ref()
    }

    /// @emoji 📡️ Drains inbound backbone messages into the meta-document's edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick()
    }

    /// @emoji 💾️ Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `StudioCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the studio history itself is
    /// durable the moment this returns.
    pub fn commit_studio_checkpoint(&mut self, message: String, authors: Vec<Author>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty() {
                member.commit_checkpoint(message.clone(), authors.clone())?;
            }
            let checkpoint_id = member.current_checkpoint_id().ok_or(VcsError::NoCheckpoint)?;
            pins.push(StudioMemberPin {
                document_id: document_id.clone(),
                checkpoint_id,
                alternative_id: member.current_alternative_id().unwrap_or_default(),
            });
        }
        let checkpoint_id = create_document_vcs_id("studio-checkpoint");
        let parent_id = self
            .meta
            .projection()?
            .checkpoints
            .last()
            .map(|checkpoint| checkpoint.id.clone());
        let checkpoint = StudioCheckpoint {
            id: checkpoint_id.clone(),
            parent_id,
            message: message.clone(),
            authors,
            timestamp: HybridLogicalTimestamp::new(0, now_ms()),
            members: pins,
        };
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::CommitStudioCheckpoint { checkpoint }],
            description: Some(message),
        })?;
        self.meta.dispatch(DocumentCommand::CommitCheckpoint {
            message: None,
            authors: Vec::new(),
        })?;
        Ok(checkpoint_id)
    }

    /// @emoji 🌿️ Records a `StudioAlternative` pinned at the current studio checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub fn create_studio_alternative(&mut self, name: String) -> Result<String, VcsError> {
        let checkpoint_id = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let alternative_id = create_document_vcs_id("studio-alternative");
        let alternative = StudioAlternative {
            id: alternative_id.clone(),
            name,
            checkpoint_ids: checkpoint_id.into_iter().collect(),
        };
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::CreateStudioAlternative { alternative }],
            description: None,
        })?;
        Ok(alternative_id)
    }

    /// @emoji 🔀️ Fans out to every member pinned by `checkpoint_id`'s `StudioCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub fn checkout_studio_checkpoint(&mut self, checkpoint_id: &str) -> Result<(), VcsError> {
        let projection = self.meta.projection()?;
        let checkpoint = projection
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.id == checkpoint_id)
            .ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id)?;
            }
        }
        Ok(())
    }

    /// @emoji 🔀️ Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub fn switch_studio_alternative(&mut self, alternative_id: &str) -> Result<(), VcsError> {
        let projection = self.meta.projection()?;
        let alternative = projection
            .alternatives
            .iter()
            .find(|alternative| alternative.id == alternative_id)
            .ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(DocumentCommand::Apply {
            operations: vec![StudioHistoryOperation::SwitchStudioAlternative {
                alternative_id: alternative_id.to_string(),
            }],
            description: None,
        })?;
        self.checkout_studio_checkpoint(&checkpoint_id)
    }

    /// @emoji ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — studio-level undo has no `StudioHistoryOperation`
    /// of its own, it is purely a cross-member ordering policy.
    pub fn undo(&mut self) -> Result<(), VcsError> {
        let target = self
            .members
            .iter()
            .filter_map(|(document_id, member)| {
                member.last_local_edit_timestamp().map(|timestamp| (timestamp.cmp_key(), document_id.clone()))
            })
            .max_by_key(|(cmp_key, _)| *cmp_key)
            .map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo()
    }

    /// @emoji ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub fn redo(&mut self) -> Result<(), VcsError> {
        let target = self
            .members
            .iter()
            .filter_map(|(document_id, member)| {
                member
                    .last_undone_local_edit_timestamp()
                    .map(|timestamp| (timestamp.cmp_key(), document_id.clone()))
            })
            .max_by_key(|(cmp_key, _)| *cmp_key)
            .map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo()
    }
}
//#endregion StudioHost
//#endregion 🔖️Studio
