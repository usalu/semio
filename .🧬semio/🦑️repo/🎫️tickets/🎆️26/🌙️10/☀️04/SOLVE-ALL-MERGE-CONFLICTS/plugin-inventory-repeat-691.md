
## 32 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
9
```

end
```
10
```

base
```
use super::*;

```

head
```
use semio_framework_ui_contract::*;

```

stash
```
use super::*;
use store::ErasedSnapshotRetirement;

```


## 33 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
121
```

end
```
132
```

base
```
    /// 🧹️ One bounded close unit of the history-edit ledger: the session vanishes, every typed owner settles against its
    /// store, then one owner retires.
    fn member_subject_retirement_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.time_travel.closing&&self.time_travel.session.stage!=TimeTravelStage::Inactive{return Ok(None)}
        let Some(member)=self.time_travel.member.as_ref()else{return Ok(None)};
        if member.review_children.is_some()||member.children.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),depth:1,..Default::default()}))}
        if let Some(owners)=member.owners.as_ref(){if !owners.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inactive original member retains live typed sources"))}return Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.owners),release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()}))}
        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?}))}}
        Ok(Some(RetirementDemand{copy_bytes:if member.metadata_retired{size_of::<TimeTravelMemberSubject>()}else{size_of::<TimeTravelMemberMetadata>()},depth:1,..Default::default()}))
    }


```

head
```

```

stash
```
    /// 🧹️ One bounded close unit of the history-edit ledger: the session vanishes, every typed owner settles against its
    /// store, then one owner retires.
    fn member_subject_retirement_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.time_travel.closing&&self.time_travel.session.stage!=TimeTravelStage::Inactive{return Ok(None)}
        let Some(member)=self.time_travel.member.as_ref()else{return Ok(None)};
        if member.review_children.is_some()||member.children.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),depth:1,..Default::default()}))}
        if let Some(owners)=member.owners.as_ref(){if !owners.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inactive original member retains live typed sources"))}return Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.owners),release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()}))}
        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?;demand}))}}
        Ok(Some(RetirementDemand{copy_bytes:if member.metadata_retired{size_of::<TimeTravelMemberSubject>()}else{size_of::<TimeTravelMemberMetadata>()},depth:1,..Default::default()}))
    }


```

### Conflict 1

start
```
1075
```

end
```
1076
```

base
```
        self.step_supersede_authoring(self.time_travel_turn_deadline(), identity).await

```

head
```
        self.step_supersede_authoring(self.shared().time_travel_turn_deadline(), identity).await

```

stash
```
        Ok(SupersedeAuthored::Pending)

```

### Conflict 2

start
```
1237
```

end
```
1239
```

base
```
        app.drive_time_travel_turn(crate::plugin_runtime::original_plugin_turn_grant(app.mounted_policy.maintenance, *retained), identity, cx).await?;
        Ok(app.time_travel.has_command_receipt())

```

head
```
        let grant = crate::plugin_runtime::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
        ctx.drive_time_travel_turn(grant, identity, cx).await?;
        Ok(ctx.time_travel.has_command_receipt())

```

stash
```
        app.drive_time_travel_turn(semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(app.mounted_policy.maintenance, *retained), identity, cx).await?;
        Ok(app.time_travel.has_command_receipt())

```


## 34 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
15
```

end
```
16
```

base
```
use super::*;

```

head
```
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use crate::plugin_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_contract::*;

```

stash
```
use super::*;
use store::ErasedSnapshotRetirement;

```


## 35 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🎮️command/📥️admission/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
176
```

end
```
195
```

base
```
pub(crate) struct PendingLatestWinsCommand<A: ArtifactApp> {
    pub(crate) command: ManuallyDrop<Option<Box<A::Command>>>,
    pub(crate) disposer: Box<dyn ArtifactOwnedDisposer<A::Command>>,
    pub(crate) admission: Option<AdmittedToolCommand>,
    pub(crate) meta: ActionMeta,
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub(crate) revision: [u8; 32],
    pub(crate) lease: Option<ToolCancellationLease>,
    pub(crate) key_copy: Option<ToolLatestWinsKeyCopy>,
    pub(crate) key: Option<semio_framework_value::ordered::SharedOwner<String>>,
    pub(crate) retired_key: Option<String>,
    pub(crate) lookup_started: bool,
    pub(crate) accepted: bool,
    pub(crate) restarting: bool,
    pub(crate) closing: bool,
    pub(crate) receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub(crate) fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub(crate) fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,

```

head
```
#[doc(hidden)]
pub struct PendingLatestWinsCommand<A: ArtifactApp> {
    pub command: ManuallyDrop<Option<Box<A::Command>>>,
    pub disposer: Box<dyn ArtifactOwnedDisposer<A::Command>>,
    pub admission: Option<AdmittedToolCommand>,
    pub meta: ActionMeta,
    pub operation: semio_framework_job::Operation,
    pub source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation),
    pub revision: [u8; 32],
    pub lease: Option<ToolCancellationLease>,
    pub key_copy: Option<ToolLatestWinsKeyCopy>,
    pub key: Option<semio_framework_value::ordered::SharedOwner<String>>,
    pub retired_key: Option<String>,
    pub lookup_started: bool,
    pub accepted: bool,
    pub restarting: bool,
    pub closing: bool,
    pub receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,

```

stash
```
pub(crate) struct PendingLatestWinsCommand<A: ArtifactApp> {
    pub(crate) command: ManuallyDrop<Option<Box<A::Command>>>,
    pub(crate) disposer: Box<dyn ArtifactOwnedDisposer<A::Command>>,
    pub(crate) admission: Option<AdmittedToolCommand>,
    pub(crate) meta: ActionMeta,
    pub(crate) operation: semio_framework_job::Operation,
    pub(crate) source_identity:semio_framework_job::OriginalSourceBinding,
    pub(crate) revision: [u8; 32],
    pub(crate) lease: Option<ToolCancellationLease>,
    pub(crate) key_copy: Option<ToolLatestWinsKeyCopy>,
    pub(crate) key: Option<semio_framework_value::ordered::SharedOwner<String>>,
    pub(crate) retired_key: Option<String>,
    pub(crate) lookup_started: bool,
    pub(crate) accepted: bool,
    pub(crate) restarting: bool,
    pub(crate) closing: bool,
    pub(crate) receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub(crate) fault:ManuallyDrop<Option<PluginLifecycleFailure>>,
    pub(crate) fault_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,

```


## 36 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️peer/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
402
```

end
```
404
```

base
```
pub(crate) struct ValidatedPeerRosterCommit {
    pub(crate) generation: u64,

```

head
```
#[doc(hidden)]
pub struct ValidatedPeerRosterCommit {
    pub generation: u64,

```

stash
```
pub(crate) struct ValidatedPeerRosterCommit {
    pub(crate) generation: u64,
}
impl ValidatedPeerRosterCommit {
    /// 👥️ Seals the generation after the mounted instance validated every original roster owner.
    pub(in crate::component) fn admitted(generation: u64) -> Self { Self { generation } }
    /// 👥️ Consumes the original roster admission for its exact generation.
    pub(in crate::component) fn into_generation(self) -> u64 { self.generation }

```


## 37 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
912
```

end
```
915
```

base
```
#[path = "../../🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs"]
mod lifted_route_capacity;
pub use lifted_route_capacity::*;

```

head
```

```

stash
```
#[path = "../../🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs"]
mod lifted_route_capacity;
pub use lifted_route_capacity::*;

#[path="⚙️worker/🦀️.rs"]
pub(crate) mod configuration_worker;

```


## 38 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
543
```

end
```
611
```

base
```
    pub(crate) fn try_new(token: TypedOperationResultToken, lane: TypedOperationResultLane, bytes: &[u8]) -> Result<Self, Fault> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed page authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub(crate) fn try_serialize<T: semio_framework_value::ToValue>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
        let text = semio_framework_pack_json::to_json_string(value);
        let encoded = text.as_bytes();
        if encoded.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed incremental encoder authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: encoded.len() };
        page.bytes[..encoded.len()].copy_from_slice(encoded);
        Ok(page)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn renderer_exchange_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::RENDERER_PAGE_MAGIC.len() + 30 + self.len);
        bytes.extend_from_slice(Self::RENDERER_PAGE_MAGIC);
        bytes.extend_from_slice(&self.token.receiver.to_le_bytes());
        bytes.extend_from_slice(&self.token.operation.to_le_bytes());
        bytes.extend_from_slice(&self.token.generation.to_le_bytes());
        bytes.extend_from_slice(&self.token.sequence.to_le_bytes());
        bytes.push(self.token.attempt);
        bytes.push(match self.lane {
            TypedOperationResultLane::Artifact => 0,
            TypedOperationResultLane::Config => 1,
            TypedOperationResultLane::Draft => 2,
            TypedOperationResultLane::Presence => 3,
            TypedOperationResultLane::Transient => 4,
            TypedOperationResultLane::Child => 5,
            TypedOperationResultLane::Effect => 6,
            TypedOperationResultLane::Event => 7,
            TypedOperationResultLane::Ui => 8,
            TypedOperationResultLane::Download => 9,
            TypedOperationResultLane::Terminal => 10,
            TypedOperationResultLane::Fault => 11,
            TypedOperationResultLane::Interaction => 12,
            TypedOperationResultLane::WindowTransient => 13,
            TypedOperationResultLane::WindowConfig => 14,
        });
        bytes.extend_from_slice(&(self.len as u32).to_le_bytes());
        bytes.extend_from_slice(self.bytes());
        bytes
    }

    pub fn renderer_ack_token(bytes: &[u8]) -> Option<TypedOperationResultToken> {
        let body = bytes.strip_prefix(Self::RENDERER_ACK_MAGIC)?;
        if body.len() != 25 {
            return None;
        }
        Some(TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        })
    }
}

```

head
```
    pub fn try_new(token: TypedOperationResultToken, lane: TypedOperationResultLane, bytes: &[u8]) -> Result<Self, Fault> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed page authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn try_serialize<T: semio_framework_value::ToValue>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
        let text = semio_framework_pack_json::to_json_string(value);
        let encoded = text.as_bytes();
        if encoded.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed incremental encoder authority"));
        }
        let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: encoded.len() };
        page.bytes[..encoded.len()].copy_from_slice(encoded);
        Ok(page)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn renderer_exchange_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::RENDERER_PAGE_MAGIC.len() + 30 + self.len);
        bytes.extend_from_slice(Self::RENDERER_PAGE_MAGIC);
        bytes.extend_from_slice(&self.token.receiver.to_le_bytes());
        bytes.extend_from_slice(&self.token.operation.to_le_bytes());
        bytes.extend_from_slice(&self.token.generation.to_le_bytes());
        bytes.extend_from_slice(&self.token.sequence.to_le_bytes());
        bytes.push(self.token.attempt);
        bytes.push(match self.lane {
            TypedOperationResultLane::Artifact => 0,
            TypedOperationResultLane::Config => 1,
            TypedOperationResultLane::Draft => 2,
            TypedOperationResultLane::Presence => 3,
            TypedOperationResultLane::Transient => 4,
            TypedOperationResultLane::Child => 5,
            TypedOperationResultLane::Effect => 6,
            TypedOperationResultLane::Event => 7,
            TypedOperationResultLane::Ui => 8,
            TypedOperationResultLane::Download => 9,
            TypedOperationResultLane::Terminal => 10,
            TypedOperationResultLane::Fault => 11,
            TypedOperationResultLane::Interaction => 12,
            TypedOperationResultLane::WindowTransient => 13,
            TypedOperationResultLane::WindowConfig => 14,
        });
        bytes.extend_from_slice(&(self.len as u32).to_le_bytes());
        bytes.extend_from_slice(self.bytes());
        bytes
    }

    pub fn renderer_ack_token(bytes: &[u8]) -> Option<TypedOperationResultToken> {
        let body = bytes.strip_prefix(Self::RENDERER_ACK_MAGIC)?;
        if body.len() != 25 {
            return None;
        }
        Some(TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        })
    }
}

```

stash
```

```

### Conflict 1

start
```
935
```

end
```
952
```

base
```
pub(crate) struct AgentLanePreview<A:ArtifactApp>{
    pub(crate) key:Option<crate::tool_cancellation::ToolOperationKey>,
    pub(crate) caller:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub(crate) starting:bool,
    pub(crate) producer:Option<OriginalPreviewDrive<semio_framework::ErasedToolJob>>,
    pub(crate) children:Option<OriginalPreviewDrive<ChildEmissionPreviewJob<A>>>,
    pub(crate) completion:Option<ArtifactToolCompletion<A>>,
    pub(crate) completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub(crate) lease:Option<crate::tool_cancellation::ToolCancellationLease>,
    pub(crate) lease_close:Option<Box<crate::tool_cancellation::ToolCancellationLeaseRetirement>>,
    pub(crate) config:semio_framework_job_worker_authority::BatchDriveConfig,
    pub(crate) maximum_bytes:usize,
    pub(crate) steps:u64,
    pub(crate) publication:Option<ArtifactToolCompletionValue<A>>,
    pub(crate) complete:bool,
    pub(crate) closing:bool,
    pub(crate) publication_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletionValue<A>>>,

```

head
```
#[doc(hidden)]
pub struct AgentLanePreview<A:ArtifactApp>{
    pub key:Option<semio_framework_plugin_operation::ToolOperationKey>,
    pub caller:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub starting:bool,
    pub producer:Option<OriginalPreviewDrive<semio_framework::ErasedToolJob>>,
    pub children:Option<OriginalPreviewDrive<ChildEmissionPreviewJob<A>>>,
    pub completion:Option<ArtifactToolCompletion<A>>,
    pub completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub lease:Option<semio_framework_plugin_operation::ToolCancellationLease>,
    pub lease_close:Option<Box<semio_framework_plugin_operation::ToolCancellationLeaseRetirement>>,
    pub config:semio_framework_job_worker_authority::BatchDriveConfig,
    pub maximum_bytes:usize,
    pub steps:u64,
    pub publication:Option<ArtifactToolCompletionValue<A>>,
    pub complete:bool,
    pub closing:bool,
    pub publication_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletionValue<A>>>,

```

stash
```
pub(crate) struct AgentLanePreview<A:ArtifactApp>{
    pub(crate) key:Option<crate::tool_cancellation::ToolOperationKey>,
    pub(crate) caller:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    pub(crate) starting:bool,
    pub(crate) producer_request:Option<crate::tool_contract::ArtifactOwnedToolJobRequest<A>>,
    pub(crate) producer_request_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<crate::tool_contract::ArtifactOwnedToolJobRequest<A>>>,
    pub(crate) producer:Option<OriginalPreviewDrive<semio_framework::ErasedToolJob>>,
    pub(crate) children:Option<OriginalPreviewDrive<ChildEmissionPreviewJob<A>>>,
    pub(crate) completion:Option<ArtifactToolCompletion<A>>,
    pub(crate) completion_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub(crate) lease:Option<crate::tool_cancellation::ToolCancellationLease>,
    pub(crate) lease_close:Option<Box<crate::tool_cancellation::ToolCancellationLeaseRetirement>>,
    pub(crate) config:semio_framework_job_worker_authority::BatchDriveConfig,
    pub(crate) maximum_bytes:usize,
    pub(crate) steps:u64,
    pub(crate) publication:Option<ArtifactToolCompletionValue<A>>,
    pub(crate) complete:bool,
    pub(crate) closing:bool,
    pub(crate) publication_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactToolCompletionValue<A>>>,

```

### Conflict 2

start
```
954
```

end
```
955
```

base
```
    pub(crate) fn retirement_demands(&self,body:usize,canonical:Option<&crate::tool_cancellation::ToolCancellationHandle>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{

```

head
```
    pub fn retirement_demands(&self,body:usize,canonical:Option<&semio_framework_plugin_operation::ToolCancellationHandle>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{

```

stash
```
    pub(crate) fn retirement_demands(&self,body:usize,canonical:Option<&crate::tool_cancellation::ToolCancellationHandle>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if self.producer_request.is_some()||self.producer_request_retirement.is_some(){return semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_demand(&self.producer_request,&self.producer_request_retirement,body)}

```

### Conflict 3

start
```
963
```

end
```
965
```

base
```
    pub(crate) fn terminal_is_empty(&self)->bool{self.key.is_none()&&self.caller.is_none()&&self.producer.is_none()&&self.children.is_none()&&self.completion.is_none()&&self.completion_close.is_none()&&self.publication.is_none()&&self.publication_close.is_none()&&self.lease.is_none()&&self.lease_close.is_none()}
    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,canonical:Option<&crate::tool_cancellation::ToolCancellationHandle>)->Result<crate::PluginLifecycleStep,crate::PluginLifecycleFailure>{

```

head
```
    pub fn terminal_is_empty(&self)->bool{self.key.is_none()&&self.caller.is_none()&&self.producer.is_none()&&self.children.is_none()&&self.completion.is_none()&&self.completion_close.is_none()&&self.publication.is_none()&&self.publication_close.is_none()&&self.lease.is_none()&&self.lease_close.is_none()}
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,canonical:Option<&semio_framework_plugin_operation::ToolCancellationHandle>)->Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{

```

stash
```
    pub(crate) fn terminal_is_empty(&self)->bool{self.key.is_none()&&self.caller.is_none()&&self.producer_request.is_none()&&self.producer_request_retirement.is_none()&&self.producer.is_none()&&self.children.is_none()&&self.completion.is_none()&&self.completion_close.is_none()&&self.publication.is_none()&&self.publication_close.is_none()&&self.lease.is_none()&&self.lease_close.is_none()}
    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,canonical:Option<&crate::tool_cancellation::ToolCancellationHandle>)->Result<crate::PluginLifecycleStep,crate::PluginLifecycleFailure>{

```

### Conflict 4

start
```
966
```

end
```
969
```

base
```
        self.closing=true;if cx.should_yield(){return Ok(crate::PluginLifecycleStep::Progress(Default::default()))}
        if let Some(owner)=self.producer.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.producer.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>crate::PluginLifecycleStep::Progress(progress),Close::Blocked=>crate::PluginLifecycleStep::Blocked{reason:"original preview producer awaits paid closure"}})}
        if let Some(owner)=self.children.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.children.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>crate::PluginLifecycleStep::Progress(progress),Close::Blocked=>crate::PluginLifecycleStep::Blocked{reason:"original preview child producer awaits paid closure"}})}

```

head
```
        self.closing=true;if cx.should_yield(){return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default()))}
        if let Some(owner)=self.producer.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.producer.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress),Close::Blocked=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"original preview producer awaits paid closure"}})}
        if let Some(owner)=self.children.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.children.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress),Close::Blocked=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"original preview child producer awaits paid closure"}})}

```

stash
```
        self.closing=true;if cx.should_yield(){return Ok(crate::PluginLifecycleStep::Progress(Default::default()))}
        if self.producer_request.is_some()||self.producer_request_retirement.is_some(){let step=semio_framework_plugin_ownership::bounded_ownership::plugin_typed_owner_step(&mut self.producer_request,&mut self.producer_request_retirement,cx.retained_grant())?;cx.consume_retained(step.progress())?;return Ok(crate::PluginLifecycleStep::Progress(step.progress()))}
        if let Some(owner)=self.producer.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.producer.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>crate::PluginLifecycleStep::Progress(progress),Close::Blocked=>crate::PluginLifecycleStep::Blocked{reason:"original preview producer awaits paid closure"}})}
        if let Some(owner)=self.children.as_mut(){let step=owner.cancel_step(cx)?;if owner.cancellation_terminal_is_empty(){self.children.take();}return Ok(match step{Close::Pending{progress}|Close::Complete{progress}=>crate::PluginLifecycleStep::Progress(progress),Close::Blocked=>crate::PluginLifecycleStep::Blocked{reason:"original preview child producer awaits paid closure"}})}

```


## 39 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️runtime-metrics-publisher/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
53
```

end
```
58
```

base
```
/// for why `include_str!` (not `std::fs`) is how a `#[cfg(test)]` block reads it without tripping
/// the crate-purity grep this repo's acceptance criteria runs unconditionally over `component.rs`
/// files (this one is the HOST, not the pure `🎭️actor` crate, but the same discipline is followed
/// here since the file is right next to it and easy to mistake for one).
const SCALE_FIXTURE_REGISTRY_JSON: &str = include_str!("../../../../../🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json");

```

head
```
/// for why the registry is read when the law runs and not embedded: its 1.7 MB would count against the crate's source boundary.
fn scale_fixture_registry_json() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("scale fixture registry {} is not readable: {error}", path.display()))
}

```

stash
```
/// for why `include_str!` (not `std::fs`) is how a `#[cfg(test)]` block reads it without tripping
/// the crate-purity grep this repo's acceptance criteria runs unconditionally over `component.rs`
/// files (this one is the HOST, not the pure `🎭️actor` crate, but the same discipline is followed
/// here since the file is right next to it and easy to mistake for one).
const SCALE_FIXTURE_REGISTRY_JSON: &str = include_str!("../../../../../🧪️testing/⚖️scale/🧪️tests/🧫️fixtures/📇️registry/🔣️.json");

```


## 40 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
0
```

end
```
5
```

base
```
use crate::child_view::PreparedChildContentSlot;
use crate::child_view::PreparedChildContentEntry;
use crate::child_view::PreparedChildContentIdentity;
use crate::app_emission::ChildEmit;
use super::*;

```

head
```
use semio_framework_plugin_decl_views::child_view::PreparedChildContentSlot;
use semio_framework_plugin_decl_views::child_view::PreparedChildContentEntry;
use semio_framework_plugin_decl_views::child_view::PreparedChildContentIdentity;
use semio_framework_plugin_decl_emission::ChildEmit;
use std::sync::Arc;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_decl_views::child_view::{ChildContentRetainers, ChildContentView, MemberKey, MemberKeyRef};
use semio_framework_plugin_child::{ChildMemberAdmission, ChildMemberRegistry};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::mem::ManuallyDrop;
use store::{MemberFactory, SpaceMember};
use semio_framework_plugin_decl_emission::child_emit_preparation::OwnedChildEmit;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::{PrivateChildMemberMetadata, PrivatePublicationMetadata, PrivatePublicationMetadataIssuer, PrivatePublicationMetadataParts, PrivatePublicationMetadataSource};
use crate::group::private_publication_group::child::PrivateChildPublicationInput;
use crate::group::private_publication_group::{PrivateOwnedPublicationLane, child, owner};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;

```

stash
```
use semio_framework_value::ErasedSnapshotRetirement as _;
use crate::child_view::PreparedChildContentSlot;
use crate::child_view::PreparedChildContentEntry;
use crate::child_view::PreparedChildContentIdentity;
use crate::app_emission::ChildEmit;
use super::*;

```


## 41 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
0
```

end
```
3
```

base
```
use crate::app_emission::ChildEmit;
use super::*;
use crate::component::private_child_input::PrivateChildGenesisInput;

```

head
```
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework::kernel::ActorId;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_decl_views::child_view::{ChildContentRetainers, MemberKeyRef};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::mem::ManuallyDrop;
use store::{MemberFactory, SpaceMember};
use semio_framework_plugin_decl_emission::child_emit_preparation::OwnedChildEmit;
use semio_framework_plugin_composition_emission::metadata::private_child_metadata::{PrivateChildMemberMetadata, PrivateChildMemberMetadataIssuer, PrivateChildMemberMetadataSource};
use semio_framework_value::{RetirementDemand, ValueError};
use crate::group::private_publication_group::{child, owner};
use semio_framework_plugin_composition_emission::private_child_input::PrivateChildGenesisInput;

```

stash
```
use semio_framework_value::ErasedSnapshotRetirement as _;
use crate::app_emission::ChildEmit;
use super::*;
use crate::component::private_child_input::PrivateChildGenesisInput;

```


## 42 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
0
```

end
```
2
```

base
```
use mounted_child_group_receipt_group::{MountedGroupMemberReceipts,MountedGroupReceiptOutput};
use mounted_child_group_receipt::MountedArtifactHandleIssuer;

```

head
```
use semio_framework::kernel::ArtifactHandle;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::ChildMemberRegistry;
use crate::mounted_child_group_receipt_group::command::CommandAppend;
use semio_framework_plugin_decl_views::history_view::CommandLogEntry;
use semio_framework_value::DslValue;
use store::MemberFactory;
use crate::mounted_child_group_identity::MountedChildGroupIdentityIssuer;
use crate::mounted_child_group_receipt_group::command::entry::MountedCommandEntry;
use crate::mounted_child_group_receipt_group::command::PagedCommandLog;
use semio_framework_plugin_composition_publication::group::private_publication_group::owner::PrivateChildPublicationGroup;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use semio_framework_plugin_decl_views::history_view::ShellHistoryReplay;
use store::SpaceMember;
use semio_framework_value::ValueError;
use crate::mounted_child_group_receipt;
use crate::mounted_child_group_receipt::mounted_nested_retirement_demand;
use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use crate::mounted_child_group_receipt_group::receipt_members::MountedGroupMemberReceipts;
use crate::mounted_child_group_receipt_group::MountedGroupReceiptOutput;
use crate::mounted_child_group_receipt::MountedArtifactHandleIssuer;

```

stash
```
use mounted_child_group_receipt_group::MountedGroupMemberReceipts;
use mounted_child_group_receipt_group::MountedGroupReceiptOutput;
use mounted_child_group_receipt::MountedArtifactHandleIssuer;

```


## 43 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
4
```

end
```
5
```

base
```
pub(crate) struct MountedChildGroupIdentitySource<'a> { pub(crate) parent: &'a str, pub(crate) actor: &'a str, pub(crate) revision: [u8; 32], pub(crate) instance: u32, pub(crate) operation: u64 }

```

head
```
pub struct MountedChildGroupIdentitySource<'a> { pub parent: &'a str, pub actor: &'a str, pub revision: [u8; 32], pub instance: u32, pub operation: u64 }

```

stash
```
pub(crate) struct MountedChildGroupIdentitySource<'a> { pub(crate) parent: &'a str, pub(crate) actor: &'a str, pub(crate) revision: [u8; 32], pub(crate) instance: u32, pub(crate) operation: u64 }

impl<'a> MountedChildGroupIdentitySource<'a> {
    /// 🫳️ Borrows the mounted parent's original coordinates for the issuer to validate and seal.
    pub(crate) fn borrowed(parent: &'a str, actor: &'a str, revision: [u8; 32], instance: u32, operation: u64) -> Self { Self { parent, actor, revision, instance, operation } }
}

```


## 44 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🪪️metadata/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
218
```

end
```
224
```

base
```
        pub(crate) fn from_parts(parts: PrivatePublicationMetadataParts) -> Self { Self { parts: ManuallyDrop::new(Some(parts)) } }
        pub(crate) fn parts(&self) -> Option<&PrivatePublicationMetadataParts> { self.parts.as_ref() }
        pub(crate) fn parts_mut(&mut self) -> Option<&mut PrivatePublicationMetadataParts> { self.parts.as_mut() }
        fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivatePublicationMetadataParts>, ValueError> { if grant.maximum_items == 0 { return Ok(None); } Ok(self.parts.take()) }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.parts.as_ref().and_then(|parts| parts.fields().into_iter().flatten().find_map(|field| (field.capacity() != 0).then_some(field.capacity())).or_else(|| parts.actor.has_owner().then(|| parts.actor.original_allocation_bytes()))).unwrap_or(0) }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.parts.is_none() }

```

head
```
        pub fn from_parts(parts: PrivatePublicationMetadataParts) -> Self { Self { parts: ManuallyDrop::new(Some(parts)) } }
        pub fn parts(&self) -> Option<&PrivatePublicationMetadataParts> { self.parts.as_ref() }
        pub fn parts_mut(&mut self) -> Option<&mut PrivatePublicationMetadataParts> { self.parts.as_mut() }
        pub fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivatePublicationMetadataParts>, ValueError> { if grant.maximum_items == 0 { return Ok(None); } Ok(self.parts.take()) }
        pub fn next_close_byte_demand(&self) -> usize { self.parts.as_ref().and_then(|parts| parts.fields().into_iter().flatten().find_map(|field| (field.capacity() != 0).then_some(field.capacity())).or_else(|| parts.actor.has_owner().then(|| parts.actor.original_allocation_bytes()))).unwrap_or(0) }
        pub fn terminal_is_empty(&self) -> bool { self.parts.is_none() }

```

stash
```
        pub(crate) fn from_parts(parts: PrivatePublicationMetadataParts) -> Self { Self { parts: ManuallyDrop::new(Some(parts)) } }
        pub(crate) fn parts(&self) -> Option<&PrivatePublicationMetadataParts> { self.parts.as_ref() }
        pub(crate) fn parts_mut(&mut self) -> Option<&mut PrivatePublicationMetadataParts> { self.parts.as_mut() }
        pub(in crate::child_content) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivatePublicationMetadataParts>, ValueError> { if grant.maximum_items == 0 { return Ok(None); } Ok(self.parts.take()) }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.parts.as_ref().and_then(|parts| parts.fields().into_iter().flatten().find_map(|field| (field.capacity() != 0).then_some(field.capacity())).or_else(|| parts.actor.has_owner().then(|| parts.actor.original_allocation_bytes()))).unwrap_or(0) }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.parts.is_none() }

```


## 45 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/📨️owned-child-dispatch/🪪️admission/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
14
```

end
```
16
```

base
```
  let admitted=app.test_reserved_emit_admission_case(occupied,&meta());
  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy());

```

head
```
  let admitted=app.dispatch_ctx().test_reserved_emit_admission_case(occupied,&meta());
  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy());

```

stash
```
  let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=caller.context().with_original_decoder(&mut decoder);
  let admitted=app.test_reserved_emit_admission_case(occupied,&meta(),&mut cx);
  drain_and_close_composed_fixture(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut cx);let fuel=cx.fuel_remaining();drop(cx);let fuel=caller.retire_decoder(&mut decoder,fuel).unwrap();caller.retire(fuel).unwrap();

```

### Conflict 1

start
```
24
```

end
```
25
```

base
```
 let retained=app.test_reserved_emit_cancelled_worker_handoff(&meta()).await;

```

head
```
 let retained=app.dispatch_ctx().test_reserved_emit_cancelled_worker_handoff(&meta()).await;

```

stash
```
 let mut caller=crate::app_testing::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();
 let retained=app.test_reserved_emit_cancelled_worker_handoff(&meta()).await;

```


## 46 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
2
```

end
```
3
```

base
```
import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProofCatalogFailures, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobScanThenMonolithRows, toolJobFrameworkReservedRoutesExact, toolJobDecodeAfterAdmission, toolJobLimitsMatch, toolJobExternalCancellationOwned, toolJobRuntimeProofQualified, toolJobQualifiedProofBeforeDecode, toolJobLiveInstanceIsolated, toolJobTypedDispatchExact, toolJobWireDispatchExact, TOOL_JOB_RESERVED_IDS, toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn, toolJobPublicationContracts, toolJobPublicationAuthorityReady, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobDropCancellationBounded, toolJobCancellationScopesBounded, toolJobHardBoundedCloseExact, toolJobErasedCloseTerminalExact, toolJobRuntimeCloseCallbackBounded, toolJobDetachedOutputOwnershipExact, toolJobLiveConstructionCleanupExact, toolJobSnapshotRetirementBounded, toolJobVcsOwnedDisposalExplicit, toolJobRuntimeRegistryFixedClose, toolJobReactorCloseBounded, toolJobOpaqueFutureProductionFailClosed, toolJobSegmentedQueueHardBounded, toolJobSegmentedTerminalDrainExact, toolJobPuzzleReservedRoutesExact, toolJobMediaExportBounded, toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact, toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../../../📜️script.ts";

```

head
```
import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobScanThenMonolithRows, TOOL_JOB_RESERVED_IDS, toolJobPublicationContracts, toolJobPublicationAuthorityReady } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
import { toolJobProofCatalogFailures, toolJobFrameworkReservedRoutesExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🟦️.ts";
import { toolJobDecodeAfterAdmission, toolJobLimitsMatch, toolJobExternalCancellationOwned, toolJobRuntimeProofQualified, toolJobQualifiedProofBeforeDecode, toolJobLiveInstanceIsolated, toolJobTypedDispatchExact, toolJobWireDispatchExact, toolJobDropCancellationBounded, toolJobCancellationScopesBounded, toolJobHardBoundedCloseExact, toolJobErasedCloseTerminalExact, toolJobRuntimeCloseCallbackBounded, toolJobDetachedOutputOwnershipExact, toolJobLiveConstructionCleanupExact, toolJobSnapshotRetirementBounded, toolJobVcsOwnedDisposalExplicit, toolJobRuntimeRegistryFixedClose, toolJobReactorCloseBounded, toolJobOpaqueFutureProductionFailClosed, toolJobMediaExportBounded } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🟦️.ts";
import { toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts";
import { toolJobSegmentedQueueHardBounded, toolJobSegmentedTerminalDrainExact, toolJobPuzzleReservedRoutesExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🛣️routes/🟦️.ts";
import { toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact } from "../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts";

```

stash
```
import fundedBatchFixture from "./🧫️fixtures/🧾️funded-batch.json";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import persistentFoundationFixture from "./🧫️fixtures/🧾️persistent-foundation.json";
import contractExpressionFixture from "./🧫️fixtures/🧾️contract-expression.json";
import { WORKSPACE_ROOT, toolJobContractSource, toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProofCatalogFailures, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobActorProgressOverlayExact, toolJobFem2dMountedSessionExact, toolJobScanThenMonolithRows, toolJobFrameworkReservedRoutesExact, toolJobDecodeAfterAdmission, toolJobLimitsMatch, toolJobExternalCancellationOwned, toolJobRuntimeProofQualified, toolJobQualifiedProofBeforeDecode, toolJobLiveInstanceIsolated, toolJobTypedDispatchExact, toolJobWireDispatchExact, TOOL_JOB_RESERVED_IDS, toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn, toolJobPublicationContracts, toolJobPublicationAuthorityReady, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobDropCancellationBounded, toolJobCancellationScopesBounded, toolJobHardBoundedCloseExact, toolJobErasedCloseTerminalExact, toolJobRuntimeCloseCallbackBounded, toolJobDetachedOutputOwnershipExact, toolJobLiveConstructionCleanupExact, toolJobSnapshotRetirementBounded, toolJobVcsOwnedDisposalExplicit, toolJobRuntimeRegistryFixedClose, toolJobReactorCloseBounded, toolJobOpaqueFutureProductionFailClosed, toolJobSegmentedQueueHardBounded, toolJobSegmentedTerminalDrainExact, toolJobPuzzleReservedRoutesExact, toolJobMediaExportBounded, toolJobImmutableOperationRootsExact, toolJobChildContentRootExact, toolJobMemberStoreOwnerExact, toolJobArtifactStoreStructuralOwnersExact, toolJobHistoryLedgerAdmissionExact, toolJobArtifactEventSourcedMergeExact, toolJobArtifactEditMessageLedgerExact, toolJobArtifactEnvelopeOwnedCodecExact, toolJobPresentationEnvelopeCallerRetainedExact, toolJobWriterEnvelopeCallerRetainedExact, toolJobJackEnvelopeCallerRetainedExact, toolJobTrinityRewriteEnvelopeCallerRetainedExact, toolJobGisMapEnvelopeCallerRetainedExact, toolJobRasterEnvelopeCallerRetainedExact, toolJobRasterEnvelopeSourceEvidence, toolJobRasterEnvelopeCallerFailures, toolJobDrawingEnvelopeCallerRetainedExact, toolJobPeerInteractionRootsExact, toolJobPagedIngressExact, toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../../../📜️script.ts";

```


## 47 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
216
```

end
```
218
```

base
```
    const calls = [...publisher.matchAll(/self\.(store|config_store|draft_store)\.admit_apply_batch\((.*?)\)\.map_err/gu)];
    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,retained))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call[2].includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && !publisher.includes(`emit.${lane}.clear()`) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call[2].includes(`&mutemit.${lanes[call[1] as keyof typeof lanes]},HistoryLane::Document`) && call[2].endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index!+call[0].length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));

```

head
```
    const calls = [...publisher.matchAll(/self\.(store|config_store|draft_store)\.admit_apply_batch\((.*?)\)\.map_err/gu)].map(call => {
      const [text, lane, args] = call;
      if (text === undefined || args === undefined || call.index === undefined || (lane !== "store" && lane !== "config_store" && lane !== "draft_store")) throw new Error("Exact mounted publication source match required");
      return { text, lane, args, index: call.index } as const;
    });
    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,retained))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call.args.includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && !publisher.includes(`emit.${lane}.clear()`) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call.args.includes(`&mutemit.${lanes[call.lane]},HistoryLane::Document`) && call.args.endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index+call.text.length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));

```

stash
```
    const calls = [...publisher.matchAll(/self\.(store|config_store|draft_store)\.admit_apply_batch\((.*?)\)\.map_err/gu)];
    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent)))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call[2].includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && !publisher.includes(`emit.${lane}.clear()`) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call[2].includes(`&mutemit.${lanes[call[1] as keyof typeof lanes]},HistoryLane::Document`) && call[2].endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index!+call[0].length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));

```


## 48 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
65
```

end
```
68
```

base
```
            CloseRung::ToolRuns => self.tool_run_retirement_demand(body),
            CloseRung::TimeTravel => self.time_travel_retirement_demands(body),
            CloseRung::ToolOverlay => self.tool_overlay_retirement_demand(body),

```

head
```
            CloseRung::ToolRuns => self.tool_run_ref().tool_run_retirement_demand(body),
            CloseRung::TimeTravel => self.time_travel_ref().time_travel_retirement_demands(body),
            CloseRung::ToolOverlay => self.render_ref().tool_overlay_retirement_demand(body),

```

stash
```
            CloseRung::ToolRuns => self.tool_run_retirement_demand(body),
            CloseRung::TimeTravel => self.time_travel_close_demands(body),
            CloseRung::ToolOverlay => self.tool_overlay_retirement_demand(body),

```


## 49 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
1387
```

end
```
1396
```

base
```
    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
        if self.refresh_cache().await.is_err() {
            return Vec::new();
        }
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let snapshot_is_admitted = {
            let (_, snapshot, _, _) = self.history.cache.as_ref().expect("cache refreshed above");
            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())

```

head
```
    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return Vec::new();
        }
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let snapshot_is_admitted = {
            let (_, snapshot, _, _) = self.history.cache.as_ref().expect("cache refreshed above");
            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())

```

stash
```
    async fn pending_effects(&mut self,view:Option<&ViewModel>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Vec<Effect>,Fault>{
        if cx.should_yield(){return Ok(Vec::new())}
        self.refresh_cache().await?;
        let render_operation=self.live_render_operation();
        let snapshot_read={let(_,snapshot,_,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;if A::mounted_job_prepare_snapshot_read(render_operation,snapshot.as_ref()){Some(self.stores.store.snapshot_read().map_err(|error|error.into_fault())?)}else{None}};
        let effects={
            let(_,snapshot,config,_)=self.history.cache.as_ref().ok_or_else(||plugin_sdk_fault("pending poll cache unavailable"))?;
            let doc=crate::ArtifactPendingView::new(snapshot.as_ref(),&self.child_composition.child_content_root,render_operation,&self.stores.store.envelope().id,snapshot_read,self.tool_runs.poll_view_for(view.and_then(|view|view.window_id.as_deref())),self.tool_machines.provisional_generation(),&self.tools.instance_operation_owner);
            let cfg=ConfigView{snapshot:config.as_ref(),window:None};
            A::pending_effects(&self.tools.instance_operation_owner,&doc,&cfg,view).await

```


## 50 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs

keys dict_keys(['path', 'relocated'])


## 51 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs

keys dict_keys(['path', 'relocated'])


## 52 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs

keys dict_keys(['path', 'relocated'])


## 53 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
1
```

end
```
853
```

base
```

use super::*;

use super::commands_instance_root::{catalogue_snapshot_from_document, framework_reserved_action_kind, note_shell_command_label};
use super::docs_instance_root::{artifact_handle_of, viewer_read_only_fault};
use super::docs_plugin_app_impl::{history_changed_event};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧮️ One reserved gesture's work, in the STAGED EDIT ROWS every other capacity in the
    /// runtime is counted in — never the bare item count the route used to hand its job, which
    /// no longer matched the two-row footprint its own config preflight declares.
    async fn framework_reserved_work_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        let items = self.framework_reserved_route_items(action, args).await?;
        FRAMEWORK_RESERVED_ROUTE_CAPACITY.rows_for_items(items).ok_or_else(|| plugin_sdk_fault(format!("framework reserved route '{action}' declares {items} items beyond its exact work capacity")))
    }

    async fn framework_reserved_route_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        match action {
            "commitCheckpoint" | "checkoutCheckpoint" => Ok(self.child_composition.children.len().saturating_add(1)),
            REVERT_TO_COMMAND_ACTION_ID => {
                self.refresh_cache().await?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
                let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                match target {
                    Some(edit_id) => {
                        let applied = self.stores.store.applied_edit_ids();
                        Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                    }
                    None => Ok(1),
                }
            }
            "copy" | "cut" => {
                let interaction = self.interaction.interaction_selection_snapshot(&self.stores);
                Ok(interaction.selection.values().map(|selection| selection.ids.len()).sum::<usize>().max(1))
            }
            "paste" => {
                let fragment_bytes = args
                    .and_then(|value| value.get("fragment"))
                    .and_then(|value| <ClipboardFragment as semio_framework_value::FromValue>::from_value(value.clone()).ok())
                    .map(|fragment| fragment.pack_bytes.map_or(fragment.dsl_text.len(), |bytes| bytes.len()))
                    .unwrap_or_default();
                Ok(fragment_bytes.div_ceil(4_096).max(1))
            }
            _ => Ok(1),
        }
    }

    /// 🎰️ `pending_reserved` is DIRECT-MAPPED on `job % ARTIFACT_LIVE_OUTPUT_SLOTS`, and this caller
    /// MINTS the id — so a residue class already held by an unrelated reserved verb (a
    /// `noteShellCommand` the shell records for every user command, an in-flight `undo`) must be
    /// answered by minting into a vacant class, never by faulting the whole route. It used to fault:
    /// a canvas pick landed as `interactive-job.reserved-spawn-capacity` with 63 slots standing empty
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END battery #44-pre: `framework route 'interactionSelect'
    /// has no exact pending spawn slot`). `retire_pending_reserved_latest_wins` still runs first, so
    /// an interaction storm keeps its latest-wins retirement rather than filling the table.
    async fn dispatch_framework_reserved_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if let Some(fault) = InstanceExtensions::reserved_action_refusal(self, action) {
            return Err(fault.into());
        }
        let contract = self.tools.qualified_tool_proof(&self.stores, action)?.contract();
        let decoded_items = bounded_json_items(args, contract.max_decoded_items)?;
        let wire_args = args.cloned();
        let raw = semio_framework_pack_json::to_json_string(&(action, wire_args)).into_bytes();
        let work_items = self.framework_reserved_work_items(action, args).await?;
        if CLIPBOARD_ACTION_IDS.contains(&action) {
            let Some(authority)=self.admit_reserved_emit_producer(action,meta,cx)?else{return Ok(None)};
            let producer = match self.build_artifact_reserved_action_job(action, args, raw.clone(), meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault.into()); } };
            let mut app_completion = None;
            let job = match producer {
                Some((job, completion)) => {
                    app_completion = Some(completion);
                    job
                }
                None => { self.tools.finish_reserved_emit_producer(authority); return Err(plugin_sdk_fault("clipboard producer has no exact installed reserved builder").into()); },
            };
            return self.dispatch_reserved_original_producer(action,&raw,decoded_items,job,app_completion.expect("installed clipboard producer owns its original completion"),meta,None,authority,cx).await.map(Some).map_err(Into::into);
        }
        let Some(permit)=self.admit_framework_reserved_spawn(action,&raw,decoded_items,meta,cx).await?else{return Ok(None)};
        let mut job = permit.operation.operation.0;
        self.pending.retire_pending_reserved_latest_wins(action, job);
        assert!(self.pending.pending_reserved.can_insert(job),"original reserved admission seals its exact vacant spawn slot");
        self.pending.pending_reserved.insert_admitted(job, PendingFrameworkReserved { action: action.to_string(), args: args.cloned(), meta: meta.clone(), permit });
        self.tools.release_reserved_emit_slot(job);
        let mut admitted = ToolOperations::<A>::empty_result(action, meta, vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: encode_framework_reserved_job_input(work_items as u64, &raw), placement: semio_framework::kernel::JobPlacement::Isolated }], Vec::new(), UiDirtyScope::None)
        .await;
        admitted.output = DslValue::Object(vec![("operationId".into(), DslValue::String(job.to_string())), ("generation".into(), DslValue::String(self.stores.store.generation().to_string()))]);
        Ok(Some(admitted))
    }

    async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.tools.qualified_tool_proof(&self.stores, verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
    /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
    /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
    pub(super) async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
        if self.pending.reserved_commit_outcome.is_some() {
            return Ok(());
        }
        let Some(mut commit) = self.pending.reserved_commits.pop_front() else {
            return Ok(());
        };
        match self.run_framework_reserved_commit_unit(&mut commit).await {
            Ok(Some(result)) => {
                let result = self.finish_recorded(commit.log_generation_before, &commit.action, result).await;
                let _ = self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Ok(result));
            }
            Ok(None) => {
                commit.applied = commit.applied.saturating_add(1);
                self.pending.reserved_commits.push_front(commit);
            }
            Err(fault) => {
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Err(fault));
            }
        }
        Ok(())
    }

    async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
        let receipt = commit.permit.identity.take().ok_or_else(|| plugin_sdk_fault("reserved commit lost its authoring receipt"))?;
        let token = commit.permit.lease.token.clone();
        let publication = commit.permit.lease.publication_claim.clone();
        let mut progress = commit.permit.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
            Ok(output) => self.child_composition.follow_derivable_children(&mut self.stores, &mut identity).await.map(|_| output),
            Err(fault) => Err(fault),
        };
        commit.permit.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        commit.permit.identity_progress = progress;
        result
    }

    async fn run_framework_reserved_commit_unit_admitted(&mut self, commit: &mut FrameworkReservedCommit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        if commit.permit.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{}' was cancelled between commit units", commit.action)));
        }
        if matches!(commit.stage, FrameworkReservedCommitStage::Validate) {
            self.stores.validate_framework_reserved_commit(&commit.action, &commit.permit).await?;
            commit.stage = match Self::history_command(&commit.action, commit.args.as_ref()).await {
                Some(ArtifactCommand::CommitCheckpoint { message, authors }) if !self.child_composition.children.is_empty() => {
                    let keys: Vec<MemberKey> = self.child_composition.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
                    self.child_composition.admit_child_content_publication_span(keys.len())?;
                    commit.total = keys.len() as u64 + 1;
                    FrameworkReservedCommitStage::CheckpointChildren { message: message.unwrap_or_else(|| "checkpoint".to_string()), authors, keys, next: 0, pins: Vec::new() }
                }
                _ => {
                    commit.total = 1;
                    FrameworkReservedCommitStage::Route { pins: Vec::new() }
                }
            };
        }
        match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
            FrameworkReservedCommitStage::Validate => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stage"), format!("framework route '{}' re-entered its validation unit", commit.action))),
            FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next, mut pins } => {
                if let Some(key) = keys.get(next) {
                    if let Some(pin) = self.child_composition.checkpoint_child_unit(&message, &authors, key.borrowed(), identity).await? {
                        pins.push(pin);
                    }
                }
                commit.stage = if next + 1 < keys.len() {
                    FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next: next + 1, pins }
                } else {
                    pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
                    FrameworkReservedCommitStage::Route { pins }
                };
                Ok(None)
            }
            FrameworkReservedCommitStage::Route { pins } => {
                let action = commit.action.clone();
                if HISTORY_ACTION_IDS.contains(&action.as_str()) {
                    let result = self.commit_framework_history_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, pins, identity).await?;
                    if action == "checkoutCheckpoint" && result.ui_scope != UiDirtyScope::None {
                        let pins = self.child_composition.checkout_cascade_pins(&mut self.stores)?;
                        if !pins.is_empty() {
                            commit.total = commit.total.saturating_add(pins.len() as u64);
                            commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: 0, result };
                            return Ok(None);
                        }
                    }
                    Ok(Some(result))
                } else if action == REVERT_TO_COMMAND_ACTION_ID {
                    match self.begin_framework_revert_route(commit.args.as_ref(), &commit.meta, identity).await? {
                        Ok(result) => Ok(Some(result)),
                        Err(edit_id) => {
                            commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                            Ok(None)
                        }
                    }
                } else {
                    self.commit_framework_shared_host_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, identity).await.map(Some)
                }
            }
            FrameworkReservedCommitStage::CheckoutChildren { pins, next, result } => {
                let Some(pin) = pins.get(next) else { return Ok(Some(result)) };
                self.child_composition.checkout_child_unit(pin, identity).await?;
                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                Ok(None)
            }
            FrameworkReservedCommitStage::Revert { edit_id } => {
                if self.stores.framework_revert_unit(&edit_id, identity).await? {
                    commit.total = commit.total.saturating_add(1);
                    commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                    return Ok(None);
                }
                self.history.cache = None;
                self.tools.record_command(&mut self.history, REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);
                Ok(Some(ToolOperations::<A>::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
            }
        }
    }

    /// 🧩️ A pending CHILD redo outranks a chrome replay for the same reason a parent one
    /// does: the document stack is reapplied before the shell rows stacked above it.
    ///
    /// 🧩️ A `Child`-lane row is a real undo target too, so chrome undo never jumps OVER
    /// the user's own composed-document edit to replay a panel toggle behind it.
    async fn dispatch_chrome_history_action(&mut self, action: &str, meta: &ActionMeta) -> Result<Option<InvocationResult>, Fault> {
        if action == "redo" {
            if !self.stores.store.redo_edit_ids().is_empty() || self.child_composition.child_history_tails().await.1 {
                return Ok(None);
            }
            let Some(replay) = self.history.shell_redo.pop() else {
                return Ok(None);
            };
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=redo seq={}", replay.seq));
            self.history.shell_undone.remove(&replay.seq);
            self.history.history_dirty_sequences.insert(replay.seq);
            self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
            return Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: replay.redo.action_id, args: replay.redo.args }], vec![history_changed_event().await], UiDirtyScope::Full).await));
        }
        if action != "undo" {
            return Ok(None);
        }
        let (child_applied_tails, _) = self.child_composition.child_history_tails().await;
        let applied: HashSet<&str> = self.stores.store.applied_edit_ids().iter().map(String::as_str).collect();
        let target = self.history.command_log.iter().rev().find(|entry| {
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.history.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            shell || document || child
        });
        let Some(entry) = target else {
            return Ok(None);
        };
        if entry.edit_id.is_some() || entry.inverse.is_none() {
            return Ok(None);
        }
        let seq = entry.seq;
        let inverse = entry.inverse.clone().expect("chrome undo target has inverse");
        let redo = InverseAction { action_id: entry.action_id.clone(), args: None };
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=undo seq={seq} inverse={}", inverse.action_id));
        self.history.shell_undone.insert(seq);
        self.history.shell_redo.push(ShellHistoryReplay { seq, redo });
        self.history.history_dirty_sequences.insert(seq);
        self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
        Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], vec![history_changed_event().await], UiDirtyScope::Full).await))
    }

    async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] history route action={action}"));
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled at commit")));
        }
        if action == "undo" || action == "redo" {
            if let Some(result) = self.dispatch_supersede_history_action(action, meta, identity).await? {
                return Ok(result);
            }
            if let Some(result) = self.dispatch_chrome_history_action(action, meta).await? {
                return Ok(result);
            }
            let parent_group_id = if action == "undo" { self.stores.store.tail_group_id().await } else { self.stores.store.redo_tail().await.and_then(|(_, group_id)| group_id) };
            let group_id = match parent_group_id {
                Some(group_id) => Some(group_id),
                None => self.child_composition.child_group_history_target(&self.history, action).await,
            };
            if let Some(group_id) = group_id {
                let grouped = self.dispatch_group_history_action(action, &group_id, meta, identity).await?;
                if grouped.ui_scope != UiDirtyScope::None {
                    return Ok(grouped);
                }
            }
        }
        let command = Self::history_command(action, args).await.ok_or_else(|| format!("history action {action} missing required argument"))?;
        match self.stores.store.dispatch(command, identity).await {
            Ok(_) => {
                self.stores.stamp_checkpoint_composition_pins(pins).await?;
                self.history.cache = None;
                if self.stores.store.local_step_pending() {
                    self.defer_history_step_row(action);
                } else {
                    self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
                }
                Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await)
            }
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() == vcs::os_store::document_materialize::EMPTY_CHECKPOINT_MESSAGE => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
    /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
    /// answers its complete result.
    async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Result<InvocationResult, String>, Fault> {
        let action = REVERT_TO_COMMAND_ACTION_ID;
        self.refresh_cache().await?;
        let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
        let transition_id = entry_seq.and_then(|seq| self.history.command_log.iter().find(|entry| entry.seq == seq)).and_then(|entry| entry.transition_id.clone());
        if let Some(transition_id) = transition_id {
            return Ok(Ok(self.revert_history_edit_row(&transition_id, meta, identity).await?));
        }
        let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
        let edit_id = match target {
            Some((Some(edit_id), _, _)) => edit_id,
            Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
            _ => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
        };
        if !self.stores.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
            return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
        }
        Ok(Err(edit_id))
    }

    pub(super) async fn commit_framework_clipboard_completion(&mut self,action:&str,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        if let Some(fault)=permit.producer_fault{let mounted=self.tools.tool_operations.get_mut(id).expect("original completion owns its admitted slot");mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original completion lease").cancel();}
        self.tools.finish_mounted_reserved_completion(id);
        Ok(started)
    }

    /// 📥️ Keeps the original producer and completion in their fixed slot through every fallible worker boundary.
    pub(super) async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        let proof=self.tools.qualified_tool_proof(&self.stores, verb)?;
        let key=proof.key();
        let admitted=match admission{Some(admitted)=>admitted,None=>self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
        if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
        let mounted=self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
        let job=mounted.reserved_producer.as_ref().expect("original reserved producer remains retained").clone();
        let dispatched=self.tools.tool_jobs.dispatch(semio_framework::ToolOperationSpec::new(key.controller_id,key.tool_id,proof.schema_id(),job,mounted.operation)).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        mounted.session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatched.job,mounted.source_identity));
        mounted.terminal_seen=false;
        mounted.stage=MountedTypedCommandFullOperationStage::Worker;
        Ok(started)
    }

    async fn commit_framework_shared_host_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.stores.validate_framework_reserved_commit(action, permit).await?;
        if INTERACTION_ACTION_IDS.contains(&action) {
            return self.dispatch_interaction_action(action, args, meta, permit, identity).await;
        }
        if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
            let value = args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or(semio_framework::HISTORY_COMMAND_FILTER_ALL);
            self.history.history_filter = HistoryCommandFilter::from_value(value).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("history-filter.unknown"), format!("'{value}' is no declared history filter option")))?;
            self.history.log_generation += 1;
            self.history.history_dirty_sequences.extend(self.history.command_log.iter().map(|entry| entry.seq));
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false })
            .await);
        }
        if action == NOTE_SHELL_COMMAND_ACTION_ID {
            let command_id = args.and_then(|value| value.get("commandId")).and_then(DslValue::as_str).ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;
            let detail = args.and_then(|value| value.get("detail")).cloned();
            let label = note_shell_command_label(args.and_then(|value| value.get("label")), command_id, detail.as_ref());
            let inverse = args
                .and_then(|value| value.get("inverseCommandId"))
                .and_then(DslValue::as_str)
                .map(|inverse_command_id| InverseAction { action_id: inverse_command_id.to_string(), args: args.and_then(|value| value.get("inverseArgs")).cloned().or(detail) });
            self.stores.validate_framework_reserved_commit(action, permit).await?;
            self.tools.record_command(&mut self.history, command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        if action == RECORD_TUTORIAL_ACTION_ID {
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-shared-host-route"), format!("unknown shared host route '{action}'")))
    }

    /// 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
    /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
    /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard/interaction) —
    /// an app's own behavior is dispatched exclusively through `dispatch_typed_command` now.
    async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;
        let pack = snapshot.encode_pack();
        let spr = store::os_store::text_format::empty_document_spr("", A::DOCUMENT_SCHEMA).await;
        self.dispatch_emit(action, Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }, meta, identity, cx).await
    }

    pub(crate) async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let Some(slot)=self.ingress.stage_action_source(action, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
        let terminal=!matches!(&result,Ok(None));
        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.ingress.pending_action_ingress_identity=None;}
        result
    }

    async fn dispatch_action_received(&mut self,action:&str,source:&mut crate::action_ingress::ActionIngressSource,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if A::ROLE == AppRole::Viewer && viewer_rejects_action(action) {
            return Err((viewer_read_only_fault(action)).into());
        }
        if action == CANCEL_TYPED_OPERATION_ACTION_ID {
            return self.tools.dispatch_operation_cancellation(args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.dismiss_refused_reprojection(action);
        if let Some(fault) = self.ingest.document_loading_refusal(&self.tools, action) {
            return Err((fault).into());
        }
        if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || InstanceExtensions::reads_history(action)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance"))).into());
        }
        Box::pin(self.deliver_utility_host_events(meta, identity, cx)).await?;
        if action == semio_framework::HOST_EVENT_ACTION_ID {
            return Box::pin(self.dispatch_host_event_action(args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Box::pin(self.commit_typing_before(action, args, meta, identity, cx)).await?;
        if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("framework.document-transfer.shell-owned"), format!("{action} is performed by the shell, never by the program"))).into());
        }
        if InstanceExtensions::claims_action(action) {
            return Box::pin(InstanceExtensions::dispatch_action(self, action, args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_framework_reserved_action_id(action) {
            return self.dispatch_framework_reserved_action(action,args,meta,cx).await;
        }
        if action == CATALOGUE_EXAMPLE_ACTION_ID && self.tools.registry.get(action).is_none() {
            let example_id = args.and_then(|value| value.get("exampleId")).and_then(DslValue::as_str).unwrap_or("");
            if let Some(document) = A::catalogue_example_document(example_id) {
                let body = document?;
                return self.dispatch_catalogue_example(action, &body, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        let definition = self.tools.registry.get(action).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{action}' before command construction")))?;
        validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;
        let definition_kind = definition.kind;
        let tool = match Box::pin(self.admit_tool_dispatch(action, args, meta, identity, cx)).await {
            ToolDispatch::Plain => None,
            ToolDispatch::Tagged(tag) => Some(tag),
            ToolDispatch::Settled(result) => return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from),
        };
        if source.assembled.is_none(){match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        }}
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {
            let admission = self.tools.admit_host_configuration_json(&self.stores, action, args).await?;
            self.media.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration action '{action}' exceeds its exact output cap"))).into());
            }
            let mut emit = Emit::config(vec![config_mutation]);
            if let Some(ToolTag::Scrub(tag)) = &tool {
                let base = self.stores.store.content_revision_now();
                let published = self.settle_press(tag, &base, &meta.actor, &mut emit)?;
                self.follow_tool_machines(true);
                if !published {
                    return Ok(Some(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await));
                }
            }
            let previous_kind = self.host_events.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, emit, meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {
            let previous_kind = self.host_events.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, Emit::default(), meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else {
            let command=A::command_from_action(action,if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fault).await?;
            let command=Box::new(command);
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
            let admission = self.tools.admit_command_wire(&self.stores, action, &wire, 1).await?;
            self.media.require_complete_tool_operation_pipeline(&admission)?;
            self.tool_machines.ingress = tool;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await;
            self.tool_machines.ingress = None;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }
    }

    /// 🎯️ Validates manifest structural app/mode ownership, converts the addressed
    /// invocation arguments once into the app's typed command, and enters the typed dispatch channel.
    pub(super) async fn dispatch_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let command_id = address.command_id.as_str();
        let owner_app_id = self.app.instance_id().await.to_string();
        let (owner, definition) = match &address.owner {
            ManifestCommandOwnerAddress::App { app_id, .. } => {
                if app_id != &owner_app_id {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}"))).into());
                }
                (format!("app {owner_app_id}"), self.tools.registry.app_command(command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}")))?)
            }
            ManifestCommandOwnerAddress::Mode { app_id, mode_id, .. } => {
                if app_id != &owner_app_id || active_mode_id != Some(mode_id.as_str()) {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}"))).into());
                }
                (format!("mode {mode_id}"), self.tools.registry.mode_command(mode_id, command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}")))?)
            }
            _ => return Err((Fault::from(format!("command '{command_id}' is not app- or mode-owned"))).into()),
        };
        validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(slot)=self.ingress.stage_action_source(command_id, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=async {
        let args = source.args.as_ref();
        match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        };
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(command_id, args)? {
            let admission = self.tools.admit_host_configuration_json(&self.stores, command_id, args).await?;
            self.media.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration command '{command_id}' exceeds its exact output cap"))).into());
            }
            let previous_kind = self.host_events.invocation_kind.replace(kind);
            let result = self.dispatch_emit(command_id, Emit::config(vec![config_mutation]), meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let crate::action_ingress::ActionIngressSource{assembled,prepared,args,fault,..}=&mut *source;
        let command=A::command_from_action(command_id,if assembled.is_some(){assembled}else if prepared.is_some(){prepared}else{args},fault).await?;
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, command_id, &wire, 1).await?;
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }.await;
        let terminal=!matches!(&result,Ok(None));
        self.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.ingress.pending_action_ingress_identity=None;}
        result
    }

    /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
    /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
    /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
    /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
    /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
    /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
    /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
    async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut BTreeMap<String, DslValue>) -> Result<(), Fault> {
        let declared = match address.window_kind_id.as_str() {
            "*" => self.tools.registry.get(&address.action_id).map(|action| action.args.clone()),
            kind => self.tools.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
        }
        .unwrap_or_default();
        let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
        for arg in omitted.iter() {
            let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
            let token = match format {
                semio_framework::manifest::ArgFormat::DocumentRevision => self.stores.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                semio_framework::manifest::ArgFormat::TargetRevision => {
                    self.refresh_cache().await?;
                    let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
                    let parent_document_id = self.stores.store.envelope().id.clone();
                    let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                    let (_, snapshot, _, history) = self.history.cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_composition.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.tools.instance_operation_owner.clone());
                    A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                        Fault::new(
                            FaultOrigin::Framework,
                            FaultCode::new("agent.target-revision-unresolved"),
                            format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                        )
                    })?
                }
                _ => continue,
            };
            arguments.insert(arg.id.clone(), DslValue::String(token));
        }
        Ok(())
    }

    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
    /// **without applying any of them**.
    ///
    /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
    /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
    /// operation whose publication ladder commits into `self.store` and mints an undo group. An
    /// agent's `action_prepare` must not do that — its own protocol commits separately
    /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
    /// guest instance per plugin, so a prepare that applied would leave the commit applying the
    /// same edit a second time. `history_undo` (which walks back exactly one application) would
    /// then leave half the edit standing — a GREEN gate that lies, which is worse than a red one.
    ///
    /// ✅️ What it shares with the shell lane, deliberately and by call rather than by copy:
    /// the owner-qualified `ManifestActionInvocation` address, `admit_addressed_action_view`'s
    /// window projection, the mode/window-kind ownership checks `handle_action_invocation` runs,
    /// the `windowId` argument it injects, `validate_ui_dispatch_classification`'s interactive-job
    /// discipline, `A::command_from_action`, the exact retained-wire admission `dispatch_action`
    /// begins (`admit_command_wire` + `require_complete_tool_operation_pipeline`) and the app's own
    /// job, built and dispatched through the same factory entry ([`Self::preview_typed_command_job`]).
    /// The one thing it does NOT share is the applying tail: the job runs to the completion it would
    /// publish, and nothing is published. It fails closed ([`agent_lane_carriage`]): a completion that publishes
    /// anything an agent transaction cannot carry is refused by name, never settled without it, and one that carries
    /// nothing but a notice or a selection is refused `app.command.no-effect`, its notices as structured causes.
    ///
    /// 🎟️ The admission it begins is consumed by the previewed job within this same call and retired
    /// through that job's own close protocol, so a prepared handle the caller never invokes leaves
    /// nothing behind.
    ///
    /// 🪟️ `"*"` is the catalog's own marker for an APP-scope verb — one that belongs to no
    /// particular window kind (`🗒️note` declares all 48 of its verbs this way). A concrete
    /// kind must own the verb; `"*"` addresses the app registry directly, exactly as
    /// `dispatch_action` does for every verb the shell sends.
    pub(crate) async fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let address=&invocation.address;
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {owner_app_id}", address.app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() && !self.tools.registry.has_mode(&address.mode_id).await {
            return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
        }
        if address.window_kind_id != "*" && reserved_kind.is_none() && self.tools.registry.window_action(&address.window_kind_id, &address.action_id).await.is_none() {
            return Err((plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id))).into());
        }
        let context = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("addressed action requires the caller's explicit language and terminology context"))?;
        let view = crate::plugin_runtime::admit_addressed_action_view(context, &invocation.address)?;
        let definition = self
            .tools.registry
            .get(&address.action_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{}' before command construction", address.action_id)))?;
        let kind = definition.kind;
        validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
        if self.tools.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
            if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
            if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
            let mut original_args=if self.ingress.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
            Some(DslValue::Object(arguments.into_iter().collect()))};
            let Some(command)=self.ingress.receive_action_command(&address.action_id, &mut original_args, &mut None, cx).await?else{return Ok(None)};
            let wire=<A::Command as protocol::OpBinary>::encode_op(&command).map_err(|source|source.into_fault())?;
            let admission=self.tools.admit_command_wire(&self.stores, &address.action_id, &wire, 1).await?;self.media.require_complete_tool_operation_pipeline(&admission)?;
            self.begin_agent_lane_preview(Box::new(command),admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
            return Ok(None)
        }
        let maximum_bytes=self.tools.agent_lane_preview.as_ref().unwrap().maximum_bytes;
        let Some((emit,(presence,transient,window_transient),job_steps))=self.tools.advance_agent_lane_preview(&address.action_id, cx)?else{return Ok(None)};
        let carriage = agent_lane_carriage(&AgentLanePublication {
            carried_ops: emit.artifact_mutations.len() + emit.config_mutations.len() + emit.draft_mutations.len() + emit.child_emits.len(),
            effects: &emit.effects,
            window_config_ops: emit.window_config_mutations.len(),
            extension_calls: emit.extension_invocations.len(),
            events: emit.events.len(),
            selection_writes: emit.interaction_writes.len(),
            tasks: emit.tasks.len(),
            presence,
            transient,
            window_transient,
        });
        match carriage {
            AgentLaneCarriage::Carried => {}
            AgentLaneCarriage::NoEffect => return Err((agent_lane_no_effect_fault(&address.action_id, &emit.effects)).into()),
            AgentLaneCarriage::Uncarried(lanes) => return Err((agent_lane_uncarried_fault(&address.action_id, &lanes.join(", "))).into()),
        }
        if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
            return Err((viewer_read_only_fault(&address.action_id)).into());
        }
        let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
        for op in emit.artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        for child in emit.child_emits.iter() {
            if !child.owner.is_empty() || self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-child-missing"),
                    format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                )).into());
            }
        }
        let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);
        let mut config_op_bytes = Vec::with_capacity(emit.config_mutations.len());
        for op in emit.config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let mut draft_op_bytes = Vec::with_capacity(emit.draft_mutations.len());
        for op in emit.draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
        if priced > maximum_bytes {
            return Err((Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.preview-output"),
                format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, maximum_bytes),
            )).into());
        }
        self.pending.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });
        let mut result = ToolOperations::<A>::empty_result(&address.action_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output = DslValue::Object(vec![
            ("previewedAction".into(), DslValue::String(address.action_id.clone())),
            ("actionKind".into(), DslValue::String(format!("{kind:?}"))),
            ("documentOps".into(), DslValue::String(artifact_op_bytes.len().to_string())),
            ("configOps".into(), DslValue::String(config_op_bytes.len().to_string())),
            ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
            ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
            ("opBytes".into(), DslValue::String(priced.to_string())),
            ("jobSteps".into(), DslValue::String(job_steps.to_string())),
        ]);
        Ok(Some(result))
    }

    /// 👁️ Builds the SAME app-owned job the shell lane builds for this admitted command (`A::build_tool_job` over
    /// [`Self::capture_typed_command_roots`]), dispatches it through the SAME retained-wire factory entry
    /// `start_typed_command_operation` uses, and drives it through its own steps on this turn
    /// ([`drive_agent_lane_preview`]) to the completion it would publish — publishing nothing. Every job shape an
    /// app owns previews this way: the SDK's retained command, a plugin's own tool-command job (puzzle, writer, …)
    /// and a download job, whose file an agent transaction cannot carry and is refused by name. The run holds a
    /// keyed cancellation lease, so cancelling the document cancels the preview. Answers the emit, how many presence,
    /// transient and window-transient mutations the job's completion would publish (lanes an agent
    /// transaction cannot carry, see [`agent_lane_uncarried_lanes`]) and how many of the job's own steps ran.
    fn admit_agent_lane_preview_key(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.tools.agent_lane_preview.is_none(){
            let grant=cx.retained_grant();let bytes=size_of::<crate::tool_jobs::AgentLanePreview<A>>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let Some(operation_id)=self.media.admit_typed_operation_slot(&self.pending, &self.tools)else{return Ok(false)};
            let revision=self.stores.store.content_revision_now();let key=ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),generation:semio_framework_job::Generation(self.stores.store.generation_now())};
            let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
        }
        let state=self.tools.agent_lane_preview.as_mut().unwrap();
        if state.closing{return Ok(false)}
        state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if key.app_instance_id!=meta.instance_id{return Err(plugin_sdk_fault("original preview operation belongs to another app instance").into())}
        if state.lease.is_none(){state.lease=self.tools.tool_cancellations.begin_keyed(key,cx)?;}
        Ok(state.lease.is_some())
    }

    async fn begin_agent_lane_preview(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let verb = admission.verb.clone();
        let roots = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let state=self.tools.agent_lane_preview.as_ref().ok_or_else(||plugin_sdk_fault("original preview admission source is absent"))?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if !state.starting||state.lease.is_none()||key.base_revision!=roots.base_revision||key.generation!=roots.generation{return Err(plugin_sdk_fault("original preview roots no longer match their admitted operation key").into())}
        let operation_id=key.operation_id;
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_job_controller_id, roots.base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        let completion = ArtifactToolCompletion::<A>::new();
        let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
        let (context, _context_birth) = ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                roots.canonical_base_revision,
                roots.draft_generation,
                roots.transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&roots.children),
                    draft: std::sync::Arc::clone(&roots.draft_snapshot),
                    transient: std::sync::Arc::clone(&roots.transient),
                    window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&roots.presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),
            artifact_owned_tool_birth_grant(),
        );
        let key = admission.proof.key();
        let contract = admission.proof.contract();
        let spec = A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
            command,
            raw_wire: ArtifactToolRawInput::transferred_to_factory(),
            operation,
            controller_id: key.controller_id,
            tool_id: key.tool_id,
            payload_schema_id: admission.proof.schema_id(),
            contract,
            decoded_items: admission.decoded_items,
            app_instance_id: meta.instance_id,
            parent_document_id: self.stores.store.envelope().id.clone(),
            canonical_base_revision: roots.canonical_base_revision,
            authoring_seed: self.stores.authoring_seed(&meta.actor),
            snapshot: roots.snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config: roots.config,
            window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
            history: roots.history,
            interaction_state: roots.interaction_state,
            interaction_hover: std::sync::Arc::new(roots.interaction_hover),
            context,
            instance_operation_owner: self.tools.instance_operation_owner.clone(),
            output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
            completion: completion.clone(),
        })
        .await?
        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?;
        let wire_admission = admission.wire_admission.clone();
        let dispatch =
            self.tools.tool_jobs.dispatch_wire_retained_with_spec(&wire_admission, admission.raw_wire, None, spec).map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?;
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
        let state=self.tools.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer=Some(crate::tool_jobs::OriginalPreviewDrive::new(dispatch.job,config,cx));state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
        Ok(())
    }

    /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`
    /// carries. Decoding it here is what gives the agent lane the same dispatch the shell's UI
    /// lane has; running it through [`Self::preview_addressed_action`] rather than
    /// `handle_action_invocation` is what makes it a PREPARE (see that method's own doc).
    pub(super) async fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let invocation: ManifestActionInvocation = crate::plugin_runtime::decode_wire_serialized(command_bytes).await.map_err(|_| {
            Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.missing-exact-key"),
                "a typed command frame carries one owner-qualified ManifestActionInvocation — the same wire AppCommand::Command carries; this frame decoded as neither, so no command-specific pre-serde envelope can be selected",
            )
        })?;
        self.preview_addressed_action(&invocation, meta, cx).await
    }
}

```

head
```

```

stash
```

use super::*;

use super::commands_instance_root::{catalogue_snapshot_from_document, framework_reserved_action_kind, note_shell_command_label};
use super::docs_instance_root::{artifact_handle_of, viewer_read_only_fault};
use super::docs_plugin_app_impl::{history_changed_event};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧮️ One reserved gesture's work, in the STAGED EDIT ROWS every other capacity in the
    /// runtime is counted in — never the bare item count the route used to hand its job, which
    /// no longer matched the two-row footprint its own config preflight declares.
    async fn framework_reserved_work_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        let items = self.framework_reserved_route_items(action, args).await?;
        FRAMEWORK_RESERVED_ROUTE_CAPACITY.rows_for_items(items).ok_or_else(|| plugin_sdk_fault(format!("framework reserved route '{action}' declares {items} items beyond its exact work capacity")))
    }

    async fn framework_reserved_route_items(&mut self, action: &str, args: Option<&DslValue>) -> Result<usize, Fault> {
        match action {
            "commitCheckpoint" | "checkoutCheckpoint" => Ok(self.child_composition.children.len().saturating_add(1)),
            REVERT_TO_COMMAND_ACTION_ID => {
                self.refresh_cache().await?;
                let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
                let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).and_then(|entry| entry.edit_id.clone()));
                match target {
                    Some(edit_id) => {
                        let applied = self.stores.store.applied_edit_ids();
                        Ok(applied.iter().position(|id| *id == edit_id).map_or(1, |position| applied.len().saturating_sub(position + 1).max(1)))
                    }
                    None => Ok(1),
                }
            }
            "copy" | "cut" => {
                let interaction = self.interaction.interaction_selection_snapshot(&self.stores);
                Ok(interaction.selection.values().map(|selection| selection.ids.len()).sum::<usize>().max(1))
            }
            "paste" => {
                let fragment_bytes = args
                    .and_then(|value| value.get("fragment"))
                    .and_then(|value| <ClipboardFragment as semio_framework_value::FromValue>::from_value(value.clone()).ok())
                    .map(|fragment| fragment.pack_bytes.map_or(fragment.dsl_text.len(), |bytes| bytes.len()))
                    .unwrap_or_default();
                Ok(fragment_bytes.div_ceil(4_096).max(1))
            }
            _ => Ok(1),
        }
    }

    /// 🎰️ `pending_reserved` is DIRECT-MAPPED on `job % ARTIFACT_LIVE_OUTPUT_SLOTS`, and this caller
    /// MINTS the id — so a residue class already held by an unrelated reserved verb (a
    /// `noteShellCommand` the shell records for every user command, an in-flight `undo`) must be
    /// answered by minting into a vacant class, never by faulting the whole route. It used to fault:
    /// a canvas pick landed as `interactive-job.reserved-spawn-capacity` with 63 slots standing empty
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END battery #44-pre: `framework route 'interactionSelect'
    /// has no exact pending spawn slot`). `retire_pending_reserved_latest_wins` still runs first, so
    /// an interaction storm keeps its latest-wins retirement rather than filling the table.
    async fn dispatch_framework_reserved_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if let Some(fault) = InstanceExtensions::reserved_action_refusal(self, action) {
            return Err(fault.into());
        }
        let contract = self.tools.qualified_tool_proof(&self.stores, action)?.contract();
        let decoded_items = bounded_json_items(args, contract.max_decoded_items)?;
        let wire_args = args.cloned();
        let raw = semio_framework_pack_json::to_json_string(&(action, wire_args)).into_bytes();
        let work_items = self.framework_reserved_work_items(action, args).await?;
        if CLIPBOARD_ACTION_IDS.contains(&action) {
            let Some(authority)=self.admit_reserved_emit_producer(action,meta,cx)?else{return Ok(None)};
            let producer = match self.build_artifact_reserved_action_job(action, args, raw.clone(), meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault.into()); } };
            let mut app_completion = None;
            let job = match producer {
                Some((job, completion)) => {
                    app_completion = Some(completion);
                    job
                }
                None => { self.tools.finish_reserved_emit_producer(authority); return Err(plugin_sdk_fault("clipboard producer has no exact installed reserved builder").into()); },
            };
            return self.dispatch_reserved_original_producer(action,&raw,decoded_items,job,app_completion.expect("installed clipboard producer owns its original completion"),meta,None,authority,cx).await.map(Some).map_err(Into::into);
        }
        let Some(permit)=self.admit_framework_reserved_spawn(action,&raw,decoded_items,meta,cx).await?else{return Ok(None)};
        let mut job = permit.operation.operation.0;
        self.pending.retire_pending_reserved_latest_wins(action, job);
        assert!(self.pending.pending_reserved.can_insert(job),"original reserved admission seals its exact vacant spawn slot");
        self.pending.pending_reserved.insert_admitted(job, PendingFrameworkReserved { action: action.to_string(), args: args.cloned(), meta: meta.clone(), permit });
        self.tools.release_reserved_emit_slot(job);
        let mut admitted = ToolOperations::<A>::empty_result(action, meta, vec![Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND.into(), input: encode_framework_reserved_job_input(work_items as u64, &raw), placement: semio_framework::kernel::JobPlacement::Isolated }], Vec::new(), UiDirtyScope::None)
        .await;
        admitted.output = DslValue::Object(vec![("operationId".into(), DslValue::String(job.to_string())), ("generation".into(), DslValue::String(self.stores.store.generation().to_string()))]);
        Ok(Some(admitted))
    }

    async fn admit_framework_reserved_spawn(&mut self, verb: &str, raw: &[u8], decoded_items: usize, meta: &ActionMeta,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<FrameworkReservedCommitPermit>, semio_framework_job::OriginalTurnFailure> {
        let proof = self.tools.qualified_tool_proof(&self.stores, verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory")).into());
        }
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission")).into());
        }
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,false,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🪜️ Runs ONE bounded unit of the commit at the head of the queue. Commits are strictly FIFO,
    /// and the head waits while a finished outcome is still untaken, so at most one outcome is ever
    /// owed to the host. A finished commit leaves its result in `reserved_commit_outcome`.
    pub(super) async fn step_framework_reserved_commit(&mut self) -> Result<(), Fault> {
        if self.pending.reserved_commit_outcome.is_some() {
            return Ok(());
        }
        let Some(mut commit) = self.pending.reserved_commits.pop_front() else {
            return Ok(());
        };
        match self.run_framework_reserved_commit_unit(&mut commit).await {
            Ok(Some(result)) => {
                let result = self.finish_recorded(commit.log_generation_before, &commit.action, result).await;
                let _ = self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: commit.permit.operation.operation.0, ui_scope: result.ui_scope.clone() });
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Ok(result));
            }
            Ok(None) => {
                commit.applied = commit.applied.saturating_add(1);
                self.pending.reserved_commits.push_front(commit);
            }
            Err(fault) => {
                commit.permit.finish();
                self.pending.reserved_commit_outcome = Some(Err(fault));
            }
        }
        Ok(())
    }

    async fn run_framework_reserved_commit_unit(&mut self, commit: &mut FrameworkReservedCommit) -> Result<Option<InvocationResult>, Fault> {
        let receipt = commit.permit.identity.take().ok_or_else(|| plugin_sdk_fault("reserved commit lost its authoring receipt"))?;
        let token = commit.permit.lease.token.clone();
        let publication = commit.permit.lease.publication_claim.clone();
        let mut progress = commit.permit.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.run_framework_reserved_commit_unit_admitted(commit, &mut identity).await {
            Ok(output) => self.child_composition.follow_derivable_children(&mut self.stores, &mut identity).await.map(|_| output),
            Err(fault) => Err(fault),
        };
        commit.permit.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        commit.permit.identity_progress = progress;
        result
    }

    async fn run_framework_reserved_commit_unit_admitted(&mut self, commit: &mut FrameworkReservedCommit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        if commit.permit.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{}' was cancelled between commit units", commit.action)));
        }
        if matches!(commit.stage, FrameworkReservedCommitStage::Validate) {
            self.stores.validate_framework_reserved_commit(&commit.action, &commit.permit).await?;
            commit.stage = match Self::history_command(&commit.action, commit.args.as_ref()).await {
                Some(ArtifactCommand::CommitCheckpoint { message, authors }) if !self.child_composition.children.is_empty() => {
                    let keys: Vec<MemberKey> = self.child_composition.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
                    self.child_composition.admit_child_content_publication_span(keys.len())?;
                    commit.total = keys.len() as u64 + 1;
                    FrameworkReservedCommitStage::CheckpointChildren { message: message.unwrap_or_else(|| "checkpoint".to_string()), authors, keys, next: 0, pins: Vec::new() }
                }
                _ => {
                    commit.total = 1;
                    FrameworkReservedCommitStage::Route { pins: Vec::new() }
                }
            };
        }
        match std::mem::replace(&mut commit.stage, FrameworkReservedCommitStage::Validate) {
            FrameworkReservedCommitStage::Validate => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stage"), format!("framework route '{}' re-entered its validation unit", commit.action))),
            FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next, mut pins } => {
                if let Some(key) = keys.get(next) {
                    if let Some(pin) = self.child_composition.checkpoint_child_unit(&message, &authors, key.borrowed(), identity).await? {
                        pins.push(pin);
                    }
                }
                commit.stage = if next + 1 < keys.len() {
                    FrameworkReservedCommitStage::CheckpointChildren { message, authors, keys, next: next + 1, pins }
                } else {
                    pins.sort_by(|left, right| left.child_ref.artifact_id.cmp(&right.child_ref.artifact_id));
                    FrameworkReservedCommitStage::Route { pins }
                };
                Ok(None)
            }
            FrameworkReservedCommitStage::Route { pins } => {
                let action = commit.action.clone();
                if HISTORY_ACTION_IDS.contains(&action.as_str()) {
                    let result = self.commit_framework_history_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, pins, identity).await?;
                    if action == "checkoutCheckpoint" && result.ui_scope != UiDirtyScope::None {
                        let pins = self.child_composition.checkout_cascade_pins(&mut self.stores)?;
                        if !pins.is_empty() {
                            commit.total = commit.total.saturating_add(pins.len() as u64);
                            commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: 0, result };
                            return Ok(None);
                        }
                    }
                    Ok(Some(result))
                } else if action == REVERT_TO_COMMAND_ACTION_ID {
                    match self.begin_framework_revert_route(commit.args.as_ref(), &commit.meta, identity).await? {
                        Ok(result) => Ok(Some(result)),
                        Err(edit_id) => {
                            commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                            Ok(None)
                        }
                    }
                } else {
                    self.commit_framework_shared_host_route(&action, commit.args.as_ref(), &commit.meta, &commit.permit, identity).await.map(Some)
                }
            }
            FrameworkReservedCommitStage::CheckoutChildren { pins, next, result } => {
                let Some(pin) = pins.get(next) else { return Ok(Some(result)) };
                self.child_composition.checkout_child_unit(pin, identity).await?;
                commit.stage = FrameworkReservedCommitStage::CheckoutChildren { pins, next: next + 1, result };
                Ok(None)
            }
            FrameworkReservedCommitStage::Revert { edit_id } => {
                if self.stores.framework_revert_unit(&edit_id, identity).await? {
                    commit.total = commit.total.saturating_add(1);
                    commit.stage = FrameworkReservedCommitStage::Revert { edit_id };
                    return Ok(None);
                }
                self.history.cache = None;
                self.tools.record_command(&mut self.history, REVERT_TO_COMMAND_ACTION_ID, ActionKind::History, None, None, Vec::new(), None);
                Ok(Some(ToolOperations::<A>::empty_result(REVERT_TO_COMMAND_ACTION_ID, &commit.meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await))
            }
        }
    }

    /// 🧩️ A pending CHILD redo outranks a chrome replay for the same reason a parent one
    /// does: the document stack is reapplied before the shell rows stacked above it.
    ///
    /// 🧩️ A `Child`-lane row is a real undo target too, so chrome undo never jumps OVER
    /// the user's own composed-document edit to replay a panel toggle behind it.
    async fn dispatch_chrome_history_action(&mut self, action: &str, meta: &ActionMeta) -> Result<Option<InvocationResult>, Fault> {
        if action == "redo" {
            if !self.stores.store.redo_edit_ids().is_empty() || self.child_composition.child_history_tails().await.1 {
                return Ok(None);
            }
            let Some(replay) = self.history.shell_redo.pop() else {
                return Ok(None);
            };
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=redo seq={}", replay.seq));
            self.history.shell_undone.remove(&replay.seq);
            self.history.history_dirty_sequences.insert(replay.seq);
            self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
            return Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: replay.redo.action_id, args: replay.redo.args }], vec![history_changed_event().await], UiDirtyScope::Full).await));
        }
        if action != "undo" {
            return Ok(None);
        }
        let (child_applied_tails, _) = self.child_composition.child_history_tails().await;
        let applied: HashSet<&str> = self.stores.store.applied_edit_ids().iter().map(String::as_str).collect();
        let target = self.history.command_log.iter().rev().find(|entry| {
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.history.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            shell || document || child
        });
        let Some(entry) = target else {
            return Ok(None);
        };
        if entry.edit_id.is_some() || entry.inverse.is_none() {
            return Ok(None);
        }
        let seq = entry.seq;
        let inverse = entry.inverse.clone().expect("chrome undo target has inverse");
        let redo = InverseAction { action_id: entry.action_id.clone(), args: None };
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] chrome history action=undo seq={seq} inverse={}", inverse.action_id));
        self.history.shell_undone.insert(seq);
        self.history.shell_redo.push(ShellHistoryReplay { seq, redo });
        self.history.history_dirty_sequences.insert(seq);
        self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
        Ok(Some(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], vec![history_changed_event().await], UiDirtyScope::Full).await))
    }

    async fn commit_framework_history_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, pins: Vec<vcs::CompositionPin>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] history route action={action}"));
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{action}' was cancelled at commit")));
        }
        if action == "undo" || action == "redo" {
            if let Some(result) = self.dispatch_supersede_history_action(action, meta, identity).await? {
                return Ok(result);
            }
            if let Some(result) = self.dispatch_chrome_history_action(action, meta).await? {
                return Ok(result);
            }
            let parent_group_id = if action == "undo" { self.stores.store.tail_group_id().await } else { self.stores.store.redo_tail().await.and_then(|(_, group_id)| group_id) };
            let group_id = match parent_group_id {
                Some(group_id) => Some(group_id),
                None => self.child_composition.child_group_history_target(&self.history, action).await,
            };
            if let Some(group_id) = group_id {
                let grouped = self.dispatch_group_history_action(action, &group_id, meta, identity).await?;
                if grouped.ui_scope != UiDirtyScope::None {
                    return Ok(grouped);
                }
            }
        }
        let command = Self::history_command(action, args).await.ok_or_else(|| format!("history action {action} missing required argument"))?;
        match self.stores.store.dispatch(command, identity).await {
            Ok(_) => {
                self.stores.stamp_checkpoint_composition_pins(pins).await?;
                self.history.cache = None;
                if self.stores.store.local_step_pending() {
                    self.defer_history_step_row(action);
                } else {
                    self.tools.record_command(&mut self.history, action, ActionKind::History, None, None, Vec::new(), None);
                }
                Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), vec![history_changed_event().await], UiDirtyScope::Full).await)
            }
            Err(vcs::VcsError::NothingToUndo) | Err(vcs::VcsError::NothingToRedo) | Err(vcs::VcsError::ForeignEdit(_)) => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(vcs::VcsError::ValidationFailed(ref message)) if message.as_str() == vcs::os_store::document_materialize::EMPTY_CHECKPOINT_MESSAGE => Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️ Resolves a `revertToCommand` target. A document or configuration edit answers the lane and
    /// edit id [`Self::framework_revert_unit`] walks back one undo per commit unit; every other target
    /// answers its complete result.
    async fn begin_framework_revert_route(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Result<InvocationResult, String>, Fault> {
        let action = REVERT_TO_COMMAND_ACTION_ID;
        self.refresh_cache().await?;
        let entry_seq = args.and_then(|value| value.get("entrySeq")).and_then(DslValue::as_f64).map(|seq| seq as u64);
        let transition_id = entry_seq.and_then(|seq| self.history.command_log.iter().find(|entry| entry.seq == seq)).and_then(|entry| entry.transition_id.clone());
        if let Some(transition_id) = transition_id {
            return Ok(Ok(self.revert_history_edit_row(&transition_id, meta, identity).await?));
        }
        let target = entry_seq.and_then(|seq| self.history.cache.as_ref().and_then(|(_, _, _, history)| history.commands.iter().find(|entry| entry.seq == seq && entry.revertible)).map(|entry| (entry.edit_id.clone(), entry.kind, entry.inverse.clone())));
        let edit_id = match target {
            Some((Some(edit_id), _, _)) => edit_id,
            Some((None, ActionKind::Shell, Some(inverse))) => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, vec![Effect::ReplayShellCommand { action_id: inverse.action_id, args: inverse.args }], Vec::new(), UiDirtyScope::None).await)),
            _ => return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
        };
        if !self.stores.store.applied_edit_ids().iter().any(|id| *id == edit_id) {
            return Ok(Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await));
        }
        Ok(Err(edit_id))
    }

    pub(super) async fn commit_framework_clipboard_completion(&mut self,action:&str,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(action,None,None,completion,meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        if let Some(fault)=permit.producer_fault{let mounted=self.tools.tool_operations.get_mut(id).expect("original completion owns its admitted slot");mounted.terminal_fault.get_or_insert(fault);mounted.cancellation_lease.as_ref().expect("original completion lease").cancel();}
        self.tools.finish_mounted_reserved_completion(id);
        Ok(started)
    }

    /// 📥️ Keeps the original producer and completion in their fixed slot through every fallible worker boundary.
    pub(super) async fn dispatch_reserved_original_producer(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:ArtifactReservedToolJob,completion:ArtifactToolCompletion<A>,meta:&ActionMeta,admission:Option<semio_framework::ToolWireAdmission>,mut permit:FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let id=permit.operation.operation.0;
        let started=self.mount_original_reserved_owners(verb,None,Some(job),Some(completion),meta,Some((permit.operation,permit.lease,permit.identity.take().ok_or_else(||plugin_sdk_fault("original reserved producer lost its authoring receipt"))?)), cx).await?;
        self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot").producer_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));
        let validation=(||->Result<(),Fault>{
            let proof=self.tools.qualified_tool_proof(&self.stores,verb)?;
            let key=proof.key();
            let admitted=match admission{Some(admitted)=>admitted,None=>self.tools.tool_jobs.admit_exact_wire(key.controller_id.clone(),key.tool_id.clone(),proof.schema_id(),raw).map_err(|error|plugin_sdk_fault(error.to_string()))?};
            if decoded_items>proof.contract().max_decoded_items||!proof.admits::<A>(&admitted){return Err(plugin_sdk_fault("original reserved producer failed its exact owner/factory/schema admission"));}
            Ok(())
        })();
        let mounted=self.tools.tool_operations.get_mut(id).expect("original producer owns its admitted slot");
        if let Err(fault)=validation{mounted.user_cancel_requested=true;mounted.terminal_seen=true;mounted.stage=MountedTypedCommandFullOperationStage::Publishing;mounted.cancellation_lease.as_ref().expect("original reserved source cancellation lease").cancel();return Err(fault)}
        mounted.terminal_seen=false;
        mounted.stage=MountedTypedCommandFullOperationStage::Worker;
        Ok(started)
    }

    async fn commit_framework_shared_host_route(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, permit: &FrameworkReservedCommitPermit, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        self.stores.validate_framework_reserved_commit(action, permit).await?;
        if INTERACTION_ACTION_IDS.contains(&action) {
            return self.dispatch_interaction_action(action, args, meta, permit, identity).await;
        }
        if action == SET_HISTORY_COMMAND_FILTER_ACTION_ID {
            let value = args.and_then(|value| value.get("value")).and_then(DslValue::as_str).unwrap_or(semio_framework::HISTORY_COMMAND_FILTER_ALL);
            self.history.history_filter = HistoryCommandFilter::from_value(value).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("history-filter.unknown"), format!("'{value}' is no declared history filter option")))?;
            self.history.log_generation += 1;
            self.history.history_dirty_sequences.extend(self.history.command_log.iter().map(|entry| entry.seq));
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false })
            .await);
        }
        if action == NOTE_SHELL_COMMAND_ACTION_ID {
            let command_id = args.and_then(|value| value.get("commandId")).and_then(DslValue::as_str).ok_or_else(|| "noteShellCommand missing required commandId".to_string())?;
            let detail = args.and_then(|value| value.get("detail")).cloned();
            let label = note_shell_command_label(args.and_then(|value| value.get("label")), command_id, detail.as_ref());
            let inverse = args
                .and_then(|value| value.get("inverseCommandId"))
                .and_then(DslValue::as_str)
                .map(|inverse_command_id| InverseAction { action_id: inverse_command_id.to_string(), args: args.and_then(|value| value.get("inverseArgs")).cloned().or(detail) });
            self.stores.validate_framework_reserved_commit(action, permit).await?;
            self.tools.record_command(&mut self.history, command_id, ActionKind::Shell, Some(label), None, Vec::new(), inverse);
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        if action == RECORD_TUTORIAL_ACTION_ID {
            return Ok(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-shared-host-route"), format!("unknown shared host route '{action}'")))
    }

    /// 🕰️ The actual body of `PluginApp::handle_action` — renamed to an inherent method so
    /// `handle_action` itself can stay a thin `finish_recorded` wrapper (see `🔖️CommandLog`). B1:
    /// FRAMEWORK-reserved verbs only (history/revert/filter/noteShellCommand/clipboard/interaction) —
    /// an app's own behavior is dispatched exclusively through `dispatch_typed_command` now.
    async fn dispatch_catalogue_example(&mut self, action: &str, body: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let snapshot = catalogue_snapshot_from_document::<A::Snapshot>(body)?;
        let pack = snapshot.encode_pack();
        let spr = store::os_store::text_format::empty_document_spr("", A::DOCUMENT_SCHEMA).await;
        self.dispatch_emit(action, Emit { effects: vec![Effect::LoadDocument { pack, spr }], ui_scope: UiDirtyScope::Full, ..Emit::default() }, meta, identity, cx).await
    }

    pub(crate) async fn dispatch_action(&mut self, action: &str, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let Some(slot)=self.tool_machines.ingress.stage_action_source(action, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=self.dispatch_action_received(action,&mut source,meta,identity,cx).await;
        let terminal=!matches!(&result,Ok(None));
        self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.tool_machines.ingress.pending_action_ingress_identity=None;}
        result
    }

    async fn dispatch_action_received(&mut self,action:&str,source:&mut crate::action_ingress::ActionIngressSource,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if A::ROLE == AppRole::Viewer && viewer_rejects_action(action) {
            return Err((viewer_read_only_fault(action)).into());
        }
        if action == CANCEL_TYPED_OPERATION_ACTION_ID {
            return self.tools.dispatch_operation_cancellation(args, meta).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        self.dismiss_refused_reprojection(action);
        if let Some(fault) = self.ingest.document_loading_refusal(&self.tools, action) {
            return Err((fault).into());
        }
        if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || InstanceExtensions::reads_history(action)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance"))).into());
        }
        Box::pin(self.deliver_utility_host_events(meta, identity, cx)).await?;
        if action == semio_framework::HOST_EVENT_ACTION_ID {
            return Box::pin(self.dispatch_host_event_action(args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        Box::pin(self.commit_typing_before(action, args, meta, identity, cx)).await?;
        if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("framework.document-transfer.shell-owned"), format!("{action} is performed by the shell, never by the program"))).into());
        }
        if InstanceExtensions::claims_action(action) {
            return Box::pin(InstanceExtensions::dispatch_action(self, action, args, meta, identity, cx)).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if is_framework_reserved_action_id(action) {
            return self.dispatch_framework_reserved_action(action,args,meta,cx).await;
        }
        if action == CATALOGUE_EXAMPLE_ACTION_ID && self.tools.registry.get(action).is_none() {
            let example_id = args.and_then(|value| value.get("exampleId")).and_then(DslValue::as_str).unwrap_or("");
            if let Some(document) = A::catalogue_example_document(example_id) {
                let body = document?;
                return self.dispatch_catalogue_example(action, &body, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
        }
        let definition = self.tools.registry.get(action).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{action}' before command construction")))?;
        validate_ui_dispatch_classification("action", action, definition.semantics.execution.interactive_job)?;
        let definition_kind = definition.kind;
        let tool = match Box::pin(self.admit_tool_dispatch(action, args, meta, identity, cx)).await {
            ToolDispatch::Plain => None,
            ToolDispatch::Tagged(tag) => Some(tag),
            ToolDispatch::Settled(result) => return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from),
        };
        if source.assembled.is_none(){match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        }}
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(action, args)? {
            let admission = self.tools.admit_host_configuration_json(&self.stores, action, args).await?;
            self.media.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration action '{action}' exceeds its exact output cap"))).into());
            }
            let mut emit = Emit::config(vec![config_mutation]);
            if let Some(ToolTag::Scrub(tag)) = &tool {
                let base = self.stores.store.content_revision_now();
                let published = self.settle_press(tag, &base, &meta.actor, &mut emit)?;
                self.follow_tool_machines(true);
                if !published {
                    return Ok(Some(ToolOperations::<A>::empty_result(action, meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await));
                }
            }
            let previous_kind = self.host_events.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, emit, meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | SET_ACTIVE_UTILITY_ACTION_ID) {
            let previous_kind = self.host_events.invocation_kind.replace(definition_kind);
            let result = self.dispatch_emit(action, Emit::default(), meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        } else {
            let command=A::command_from_action(action,if source.assembled.is_some(){&mut source.assembled}else if source.prepared.is_some(){&mut source.prepared}else{&mut source.args},&mut source.fault).await?;
            let command=Box::new(command);
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
            let admission = self.tools.admit_command_wire(&self.stores, action, &wire, 1).await?;
            self.media.require_complete_tool_operation_pipeline(&admission)?;
            self.tool_machines.ingress = tool;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await;
            self.tool_machines.ingress = None;
            result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }
    }

    /// 🎯️ Validates manifest structural app/mode ownership, converts the addressed
    /// invocation arguments once into the app's typed command, and enters the typed dispatch channel.
    pub(super) async fn dispatch_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let command_id = address.command_id.as_str();
        let owner_app_id = self.app.instance_id().await.to_string();
        let (owner, definition) = match &address.owner {
            ManifestCommandOwnerAddress::App { app_id, .. } => {
                if app_id != &owner_app_id {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}"))).into());
                }
                (format!("app {owner_app_id}"), self.tools.registry.app_command(command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by app {owner_app_id}")))?)
            }
            ManifestCommandOwnerAddress::Mode { app_id, mode_id, .. } => {
                if app_id != &owner_app_id || active_mode_id != Some(mode_id.as_str()) {
                    return Err((Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}"))).into());
                }
                (format!("mode {mode_id}"), self.tools.registry.mode_command(mode_id, command_id).await.ok_or_else(|| Fault::from(format!("command '{command_id}' is not owned by active mode {mode_id} of app {owner_app_id}")))?)
            }
            _ => return Err((Fault::from(format!("command '{command_id}' is not app- or mode-owned"))).into()),
        };
        validate_ui_dispatch_classification(&owner, command_id, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(slot)=self.tool_machines.ingress.stage_action_source(command_id, source_args, original_fault, cx)?else{return Ok(None)};
        let Some(mut source)=self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().loan_source(cx)?else{return Ok(None)};
        let result=async {
        let args = source.args.as_ref();
        match self.ingest.import_staging.admit_args(args).map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), refusal.message()))? {
            semio_framework::kernel::ImportArguments::Staged { .. } => return Ok(Some(ToolOperations::<A>::empty_result(command_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await)),
            semio_framework::kernel::ImportArguments::Whole(whole) => source.assembled=Some(whole),
            semio_framework::kernel::ImportArguments::NotAnImport => (),
        };
        let args=source.assembled.as_ref().or(source.prepared.as_ref()).or(source.args.as_ref());
        if let Some(config_mutation) = A::host_configuration_mutation(command_id, args)? {
            let admission = self.tools.admit_host_configuration_json(&self.stores, command_id, args).await?;
            self.media.require_tool_operation_authority(&admission)?;
            let encoded = ::protocol::OpBinary::encode_op(&config_mutation).map_err(|error| error.into_fault())?;
            if encoded.len() > admission.proof.contract().max_output_bytes {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.host-configuration-output"), format!("host configuration command '{command_id}' exceeds its exact output cap"))).into());
            }
            let previous_kind = self.host_events.invocation_kind.replace(kind);
            let result = self.dispatch_emit(command_id, Emit::config(vec![config_mutation]), meta, identity, cx).await;
            self.host_events.invocation_kind = previous_kind;
            return result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let crate::action_ingress::ActionIngressSource{assembled,prepared,args,fault,..}=&mut *source;
        let command=A::command_from_action(command_id,if assembled.is_some(){assembled}else if prepared.is_some(){prepared}else{args},fault).await?;
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, command_id, &wire, 1).await?;
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        result.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
        }.await;
        let terminal=!matches!(&result,Ok(None));
        self.tool_machines.ingress.action_ingress_sources.get_mut(slot).unwrap().return_source(std::mem::ManuallyDrop::into_inner(source),terminal);
        if terminal{self.tool_machines.ingress.pending_action_ingress_identity=None;}
        result
    }

    /// 🔐️ The agent lane's revision fill (ticket 26/09/23, G12 session 14c): a declared
    /// [`ActionArgDef::document_revision`] / [`ActionArgDef::target_revision`] argument the agent omitted is admitted against
    /// the token its rendered binding carries right now — the document's canonical revision (what every document-scoped
    /// verb compares against on this lane), or the app's own [`ArtifactApp::agent_target_revision`]; a target the app cannot
    /// resolve is refused by name. Only this lane fills: the shell lanes pass the binding's token and the app's parser
    /// refuses its absence there; the gateway admits an omission only behind a passed MCP `expectedRevision` (the
    /// document-level stale-write guard), and its prepare→invoke pair refuses a document that moved in between.
    async fn fill_agent_revisions(&mut self, address: &semio_framework::manifest::ActionAddress, arguments: &mut BTreeMap<String, DslValue>) -> Result<(), Fault> {
        let declared = match address.window_kind_id.as_str() {
            "*" => self.tools.registry.get(&address.action_id).map(|action| action.args.clone()),
            kind => self.tools.registry.window_action(kind, &address.action_id).await.map(|action| action.args.clone()),
        }
        .unwrap_or_default();
        let omitted: Vec<_> = declared.into_iter().filter(|arg| !arguments.contains_key(&arg.id)).collect();
        for arg in omitted.iter() {
            let semio_framework::manifest::ArgSchema::String { format: Some(format), .. } = &arg.schema else { continue };
            let token = match format {
                semio_framework::manifest::ArgFormat::DocumentRevision => self.stores.store.content_revision().iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                semio_framework::manifest::ArgFormat::TargetRevision => {
                    self.refresh_cache().await?;
                    let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
                    let parent_document_id = self.stores.store.envelope().id.clone();
                    let staged = DslValue::Object(arguments.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                    let (_, snapshot, _, history) = self.history.cache.as_ref().expect("cache refreshed above");
                    let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(&self.child_composition.child_content_root), render_operation, parent_document_id, None).await.with_instance(self.tools.instance_operation_owner.clone());
                    A::agent_target_revision(&address.action_id, &staged, &doc).await?.ok_or_else(|| {
                        Fault::new(
                            FaultOrigin::Framework,
                            FaultCode::new("agent.target-revision-unresolved"),
                            format!("action '{}' binds its `{}` to one addressed target and this app resolves no such target for an agent — pass the target's revision", address.action_id, arg.id),
                        )
                    })?
                }
                _ => continue,
            };
            arguments.insert(arg.id.clone(), DslValue::String(token));
        }
        Ok(())
    }

    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: validate, price, and produce the ops
    /// **without applying any of them**.
    ///
    /// 🧭️ Why this cannot be `AppCommand::Command`. The shell's dispatch lane APPLIES: it runs
    /// `handle_action_invocation` → `dispatch_typed_command_inner`, which mounts a typed
    /// operation whose publication ladder commits into `self.stores.store` and mints an undo group. An
    /// agent's `action_prepare` must not do that — its own protocol commits separately
    /// (`TransactionPrepare{ops}` → `TransactionCommit`), and `RoutingArtifactChannel` caches ONE
    /// guest instance per plugin, so a prepare that applied would leave the commit applying the
    /// same edit a second time. `history_undo` (which walks back exactly one application) would
    /// then leave half the edit standing — a GREEN gate that lies, which is worse than a red one.
    ///
    /// ✅️ What it shares with the shell lane, deliberately and by call rather than by copy:
    /// the owner-qualified `ManifestActionInvocation` address, `admit_addressed_action_view`'s
    /// window projection, the mode/window-kind ownership checks `handle_action_invocation` runs,
    /// the `windowId` argument it injects, `validate_ui_dispatch_classification`'s interactive-job
    /// discipline, `A::command_from_action`, the exact retained-wire admission `dispatch_action`
    /// begins (`admit_command_wire` + `require_complete_tool_operation_pipeline`) and the app's own
    /// job, built and dispatched through the same factory entry ([`Self::preview_typed_command_job`]).
    /// The one thing it does NOT share is the applying tail: the job runs to the completion it would
    /// publish, and nothing is published. It fails closed ([`agent_lane_carriage`]): a completion that publishes
    /// anything an agent transaction cannot carry is refused by name, never settled without it, and one that carries
    /// nothing but a notice or a selection is refused `app.command.no-effect`, its notices as structured causes.
    ///
    /// 🎟️ The admission it begins is consumed by the previewed job within this same call and retired
    /// through that job's own close protocol, so a prepared handle the caller never invokes leaves
    /// nothing behind.
    ///
    /// 🪟️ `"*"` is the catalog's own marker for an APP-scope verb — one that belongs to no
    /// particular window kind (`🗒️note` declares all 48 of its verbs this way). A concrete
    /// kind must own the verb; `"*"` addresses the app registry directly, exactly as
    /// `dispatch_action` does for every verb the shell sends.
    pub(crate) async fn preview_addressed_action(&mut self, invocation: &ManifestActionInvocation, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let address=&invocation.address;
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {owner_app_id}", address.app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() && !self.tools.registry.has_mode(&address.mode_id).await {
            return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
        }
        if address.window_kind_id != "*" && reserved_kind.is_none() && self.tools.registry.window_action(&address.window_kind_id, &address.action_id).await.is_none() {
            return Err((plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id))).into());
        }
        let context = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("addressed action requires the caller's explicit language and terminology context"))?;
        let view = crate::plugin_runtime::admit_addressed_action_view(context, &invocation.address)?;
        let definition = self
            .tools.registry
            .get(&address.action_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("UI dispatch rejected unknown action key '{}' before command construction", address.action_id)))?;
        let kind = definition.kind;
        validate_ui_dispatch_classification("action", &address.action_id, definition.semantics.execution.interactive_job)?;
        if self.tools.agent_lane_preview.as_ref().is_none_or(|state|state.starting){
            if !self.admit_agent_lane_preview_key(meta,cx)?{return Ok(None)}
            if !self.tool_machines.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
            let mut original_args=if self.tool_machines.ingress.action_source_pending(){None}else{let mut arguments=invocation.arguments.clone();arguments.insert("windowId".into(),DslValue::String(address.window_instance_id.clone()));self.fill_agent_revisions(address,&mut arguments).await?;
            Some(DslValue::Object(arguments.into_iter().collect()))};
            let Some(command)=self.tool_machines.ingress.receive_action_command(&address.action_id, &mut original_args, &mut None, cx).await?else{return Ok(None)};
            let wire=<A::Command as protocol::OpBinary>::encode_op(&command).map_err(|source|source.into_fault())?;
            let admission=self.tools.admit_command_wire(&self.stores, &address.action_id, &wire, 1).await?;self.media.require_complete_tool_operation_pipeline(&admission)?;
            self.begin_agent_lane_preview(Box::new(command),admission,&ActionMeta{view_state:Some(view),..meta.clone()},cx).await?;
            return Ok(None)
        }
        let maximum_bytes=self.tools.agent_lane_preview.as_ref().unwrap().maximum_bytes;
        let Some((emit,(presence,transient,window_transient),job_steps))=self.tools.advance_agent_lane_preview(&address.action_id, cx)?else{return Ok(None)};
        let carriage = agent_lane_carriage(&AgentLanePublication {
            carried_ops: emit.artifact_mutations.len() + emit.config_mutations.len() + emit.draft_mutations.len() + emit.child_emits.len(),
            effects: &emit.effects,
            window_config_ops: emit.window_config_mutations.len(),
            extension_calls: emit.extension_invocations.len(),
            events: emit.events.len(),
            selection_writes: emit.interaction_writes.len(),
            tasks: emit.tasks.len(),
            presence,
            transient,
            window_transient,
        });
        match carriage {
            AgentLaneCarriage::Carried => {}
            AgentLaneCarriage::NoEffect => return Err((agent_lane_no_effect_fault(&address.action_id, &emit.effects)).into()),
            AgentLaneCarriage::Uncarried(lanes) => return Err((agent_lane_uncarried_fault(&address.action_id, &lanes.join(", "))).into()),
        }
        if A::ROLE == AppRole::Viewer && (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty()) {
            return Err((viewer_read_only_fault(&address.action_id)).into());
        }
        let mut artifact_op_bytes = Vec::with_capacity(emit.artifact_mutations.len());
        for op in emit.artifact_mutations.iter() {
            artifact_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        for child in emit.child_emits.iter() {
            if !child.owner.is_empty() || self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())).is_none() {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-child-missing"),
                    format!("action '{}' edits owned child {:?} in slot {:?}, which this instance does not hold", address.action_id, child.child_id, child.slot),
                )).into());
            }
        }
        let child_op_bytes = ChildEmit::encode_groups(&emit.child_emits);
        let mut config_op_bytes = Vec::with_capacity(emit.config_mutations.len());
        for op in emit.config_mutations.iter() {
            config_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let mut draft_op_bytes = Vec::with_capacity(emit.draft_mutations.len());
        for op in emit.draft_mutations.iter() {
            draft_op_bytes.push(::protocol::OpBinary::encode_op(op).map_err(|error| error.into_fault())?);
        }
        let priced = artifact_op_bytes.iter().chain(config_op_bytes.iter()).chain(draft_op_bytes.iter()).map(Vec::len).sum::<usize>();
        if priced > maximum_bytes {
            return Err((Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.preview-output"),
                format!("previewed action '{}' produced {priced} op byte(s); its exact output cap is {}", address.action_id, maximum_bytes),
            )).into());
        }
        self.pending.last_emit_wire = Some(EmitWire { document: protocol::encode_ops_vec(&artifact_op_bytes), config: protocol::encode_ops_vec(&config_op_bytes), draft: protocol::encode_ops_vec(&draft_op_bytes), children: child_op_bytes });
        let mut result = ToolOperations::<A>::empty_result(&address.action_id, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        result.output = DslValue::Object(vec![
            ("previewedAction".into(), DslValue::String(address.action_id.clone())),
            ("actionKind".into(), DslValue::String(format!("{kind:?}"))),
            ("documentOps".into(), DslValue::String(artifact_op_bytes.len().to_string())),
            ("configOps".into(), DslValue::String(config_op_bytes.len().to_string())),
            ("draftOps".into(), DslValue::String(draft_op_bytes.len().to_string())),
            ("childGroups".into(), DslValue::String(emit.child_emits.len().to_string())),
            ("opBytes".into(), DslValue::String(priced.to_string())),
            ("jobSteps".into(), DslValue::String(job_steps.to_string())),
        ]);
        Ok(Some(result))
    }

    /// 👁️ Builds the SAME app-owned job the shell lane builds for this admitted command (`A::build_tool_job` over
    /// [`Self::capture_typed_command_roots`]), dispatches it through the SAME retained-wire factory entry
    /// `start_typed_command_operation` uses, and drives it through its own steps on this turn
    /// ([`drive_agent_lane_preview`]) to the completion it would publish — publishing nothing. Every job shape an
    /// app owns previews this way: the SDK's retained command, a plugin's own tool-command job (puzzle, writer, …)
    /// and a download job, whose file an agent transaction cannot carry and is refused by name. The run holds a
    /// keyed cancellation lease, so cancelling the document cancels the preview. Answers the emit, how many presence,
    /// transient and window-transient mutations the job's completion would publish (lanes an agent
    /// transaction cannot carry, see [`agent_lane_uncarried_lanes`]) and how many of the job's own steps ran.
    fn admit_agent_lane_preview_key(&mut self,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.tools.agent_lane_preview.is_none(){
            let grant=cx.retained_grant();let bytes=size_of::<crate::tool_jobs::AgentLanePreview<A>>();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let Some(operation_id)=self.media.admit_typed_operation_slot(&self.pending, &self.tools)else{return Ok(false)};
            let revision=self.stores.store.content_revision_now();let key=ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision:semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width"))),generation:semio_framework_job::Generation(self.stores.store.generation_now())};
            let Some(caller)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:grant,site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:0,step_budget_us:0};
            self.tools.agent_lane_preview=Some(crate::tool_jobs::AgentLanePreview{key:Some(key),caller:std::mem::ManuallyDrop::new(Some(caller)),starting:true,producer_source:None,producer_request:None,producer_request_retirement:None,producer:None,children:None,completion:None,completion_close:None,lease:None,lease_close:None,config,maximum_bytes:0,steps:0,publication:None,complete:false,closing:false,publication_close:None});
        }
        let state=self.tools.agent_lane_preview.as_mut().unwrap();
        if state.closing{return Ok(false)}
        state.caller.as_ref().ok_or_else(||plugin_sdk_fault("original preview caller source is absent"))?.validate_context(cx)?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if key.app_instance_id!=meta.instance_id{return Err(plugin_sdk_fault("original preview operation belongs to another app instance").into())}
        if state.lease.is_none(){state.lease=self.tools.tool_cancellations.begin_keyed(key,cx)?;}
        Ok(state.lease.is_some())
    }

    async fn begin_agent_lane_preview(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
        let verb = admission.verb.clone();
        let roots = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let state=self.tools.agent_lane_preview.as_ref().ok_or_else(||plugin_sdk_fault("original preview admission source is absent"))?;
        let key=state.key.as_ref().ok_or_else(||plugin_sdk_fault("original preview operation key is absent"))?;
        if !state.starting||state.lease.is_none()||key.base_revision!=roots.base_revision||key.generation!=roots.generation{return Err(plugin_sdk_fault("original preview roots no longer match their admitted operation key").into())}
        let operation_id=key.operation_id;
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_job_controller_id, roots.base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        let completion = ArtifactToolCompletion::<A>::new();
        let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
        let (context, _context_birth) = ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                roots.canonical_base_revision,
                roots.draft_generation,
                roots.transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&roots.children),
                    draft: std::sync::Arc::clone(&roots.draft_snapshot),
                    transient: std::sync::Arc::clone(&roots.transient),
                    window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&roots.presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation()),
            artifact_owned_tool_birth_grant(),
        );
        let key = admission.proof.key();
        let contract = admission.proof.contract();
        let request = ArtifactOwnedToolJobRequest {
            source_binding: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
                retained:self.mounted_policy.maintenance,
            command: crate::tool_contract::ArtifactOriginalToolCommand::from_admitted_op_binary(command,&admission.raw_wire,semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)),
            input: Some(admission.raw_wire),
            checkpoint: None,
            operation,
            controller_id: key.controller_id,
            tool_id: key.tool_id,
            payload_schema_id: admission.proof.schema_id(),
            contract,
            decoded_items: admission.decoded_items,
            app_instance_id: meta.instance_id,
            parent_document_id: self.stores.store.envelope().id.clone(),
            canonical_base_revision: roots.canonical_base_revision,
            authoring_seed: self.authoring_seed(&meta.actor),
            snapshot: roots.snapshot,
            snapshot_read: self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?,
            config: roots.config,
            window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
            history: roots.history,
            interaction_state: roots.interaction_state,
            interaction_hover: std::sync::Arc::new(roots.interaction_hover),
            context,
            instance_operation_owner: self.tools.instance_operation_owner.clone(),
            output_chunks: ArtifactOutputChunks::new(contract.max_output_bytes),
            completion: completion.clone(),
        };
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:cx.retained_grant(),site:"plugin_agent_lane_preview",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:contract.max_work_units_per_step,step_budget_us:u64::from(contract.max_step_micros)};
        let state=self.tools.agent_lane_preview.as_mut().expect("original preview lease remains received");state.producer_request=Some(request);state.completion=Some(completion);state.config=config;state.maximum_bytes=contract.max_output_bytes;state.starting=false;
        Ok(())
    }

    /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`
    /// carries. Decoding it here is what gives the agent lane the same dispatch the shell's UI
    /// lane has; running it through [`Self::preview_addressed_action`] rather than
    /// `handle_action_invocation` is what makes it a PREPARE (see that method's own doc).
    pub(super) async fn dispatch_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let invocation: ManifestActionInvocation = crate::plugin_runtime::decode_wire_serialized(command_bytes).await.map_err(|_| {
            Fault::new(
                FaultOrigin::Framework,
                FaultCode::new("interactive-job.missing-exact-key"),
                "a typed command frame carries one owner-qualified ManifestActionInvocation — the same wire AppCommand::Command carries; this frame decoded as neither, so no command-specific pre-serde envelope can be selected",
            )
        })?;
        self.preview_addressed_action(&invocation, meta, cx).await
    }
}

```


## 54 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs

keys dict_keys(['path', 'relocated'])


## 55 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
1
```

end
```
1415
```

base
```

use super::*;

use super::docs_instance_root::{artifact_handle_of, tool_transaction_shape_fault, typed_operation_document_is_fresh, viewer_read_only_fault};
use super::docs_maintenance_ladder::{removed};
use super::typed_ops_instance_root::{store_publication_fault};
use super::typed_ops_plugin_app_impl::{PUBLICATION_RETURNED_ROOT_ALLOWANCE};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🩺️ How many consecutive publication units advanced the SAME operation without changing any of
    /// [`Self::typed_operation_stall_witness`]'s three facts — `Some(streak)` only on a power of two at
    /// or above [`TYPED_OPERATION_STALL_WITNESS_FLOOR`], so one stalled operation costs at most a
    /// handful of lines however long it spins.
    ///
    /// 🐛️ A mounted operation that never publishes its terminal keeps the actor in `MoreWork` with no
    /// effect, no patch and no fault, which every other instrument reads as silence: the host's own
    /// drain polls forever, the reactor's more-work streak names only `typed_operation` as its source,
    /// and nothing says WHICH arm of the publication ladder is not moving. Measured 2026-09-20 on the
    /// signed-in `s` Home, whose `applyDirectoryEventPage` operation spun for the whole session
    /// (ticket 26/09/18 S8).
    fn typed_operation_stall_streak(before: (u64, u8, u8, u64), after: (u64, u8, u8, u64)) -> Option<u64> {
        static WITNESS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        static STREAK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = (after.0.wrapping_shl(16) | u64::from(after.1).wrapping_shl(8) | u64::from(after.2)) ^ after.3.wrapping_mul(TYPED_OPERATION_STALL_PROGRESS_MIX);
        if before != after || WITNESS.swap(key, std::sync::atomic::Ordering::Relaxed) != key {
            STREAK.store(0, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        let streak = STREAK.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
        (streak >= TYPED_OPERATION_STALL_WITNESS_FLOOR && streak.is_power_of_two()).then_some(streak)
    }

    pub(crate) async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let before = self.tools.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.tools.typed_operation_stall_witness()) {
            if let Some(streak) = Self::typed_operation_stall_streak(before, after) {
                let verb = self.tools.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
                if after.1 == TYPED_OPERATION_STALL_PUBLISHING_STAGE && streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    self.tools.fault_stalled_typed_operation_publication(after.0, streak)?;
                }
            }
        }
        advanced
    }

    /// ♻️ A retiring operation answers `has_runnable_typed_operations`, so the turn driver that reports it
    /// runnable must be the one that releases its slot — see [`Self::retire_typed_operation_run`].
    ///
    /// 🕹️ "After EVERY artifact (document) dispatch, re-derive fresh topology and prune any
    /// selection/hover id no longer present" — the clause `dispatch_emit_inner` carries, which
    /// the migrated publication ladder never reaches (see the emit backstops above it). A
    /// migrated dispatch's document change lands on the Artifact lane, so this is where the
    /// revalidation belongs; `mounted` is out of the registry here, so the async pass is free
    /// to borrow the app, and the flag makes it exactly one pass per operation.
    async fn advance_typed_operation_publication_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if !self.tools.latest_wins_commands.is_empty() && (self.tools.latest_wins_turn || self.tools.tool_operations.is_empty()) {
            self.tools.latest_wins_turn = false;
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.tools.latest_wins_turn = true;
        let Some((index, operation_id)) = self.tools.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
            let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();
            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
            let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
            mounted.cancellation_lease=self.tools.tool_cancellations.begin_keyed(&key,cx)?;
            if let Some(lease)=mounted.cancellation_lease.as_ref(){
                if mounted.terminal_fault.is_some()||mounted.user_cancel_requested{lease.cancel();}
                let maximum=({ let __a0 = &self.tools.tool_operations.get(operation_id).unwrap().verb; self.tools.qualified_tool_proof(&self.stores, __a0) })?.contract().max_output_bytes;
                let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();let lease=mounted.cancellation_lease.as_ref().unwrap();
                let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
                mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);
            }
            return Ok(());
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker) {
            record_typed_operation_unit(TypedOperationUnitKind::Worker);
            return self.tools.drive_typed_operation_worker(&self.mounted_policy, operation_id, cx);
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
            record_typed_operation_unit(TypedOperationUnitKind::Retirement);
            return self.retire_typed_operation_run(operation_id,cx);
        }
        if !self.tools.tool_operations.get_mut(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Publishing) {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        }
        let mut mounted =
            self.tools.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority"), "typed-operation publication owner changed before one-unit transfer"))?;
        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;
        let mut driver_result = Ok(());
        if let Err(failure)=outcome{self.tools.tool_operations.insert_admitted(operation_id,mounted);return Err(failure);}
        mounted.publication_attempt=0;
        if driver_result.is_ok() && mounted.published_artifact && !mounted.interaction_revalidated {
            mounted.interaction_revalidated = true;
            let meta = mounted.meta.clone();
            let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
            let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
            let token = lease.token.clone();
            let publication = lease.publication_claim.clone();
            let mut progress = mounted.identity_progress;
            let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
            driver_result = self.revalidate_interaction_on_document_change(&meta, &mut identity).await;
            mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
            drop(observer);
            mounted.identity_progress = progress;
        }
        self.tools.tool_operations.insert_admitted(operation_id, mounted);
        driver_result.map_err(Into::into)
    }

    /// 🏃️ Drives ONE mounted operation's publication ladder for a bounded slice of this continuation
    /// unit — the exact twin of the slice [`Self::drive_typed_operation_worker`] gives a Worker-stage
    /// operation and [`Self::retire_typed_operation_run`] gives a retiring one, for the publication
    /// state machine that sits between them.
    ///
    /// 🧾️ Every ladder step — install the taken completion, stage the batch, prepare one item, fold
    /// it, retire its preparation owner, validate the cursor, preflight the commit, commit, mint the
    /// receipt — used to be its own host round trip, because the caller is driven once per reactor
    /// turn and the store's grant pages one item per call. One `deleteSelection` on a ONE-object
    /// document therefore cost 104–235 round trips (wave B44 §2.3): a document-INDEPENDENT constant
    /// that alone burns seconds of a 30-second interaction budget, and the reason a `setCamera` on a
    /// document it never reads cost 2.3 s.
    ///
    /// 🛑️ The run stops on exactly the protocol's own bounds and on nothing else: a queued result page
    /// the host must acknowledge before the operation may continue (`stage` leaves `Publishing`), a
    /// completion that is not ready yet so nothing is staged to advance, [`TYPED_OPERATION_PUBLICATION_PUMPS`],
    /// and [`INTERACTIVE_TURN_WORKER_WALL_US`]. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
    pub(crate) async fn publish_mounted_typed_operation_run(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.identity.is_none() {
            let completion = mounted.completion.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original effect completion"))?;
            let (receipt, progress) = completion.take_authoring_identity()?.ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original app-owned effect receipt"))?;
            mounted.identity = Some(receipt);
            mounted.identity_progress = progress;
        }
        let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its admitted authoring receipt"))?;
        let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original cancellation lease"))?;
        let token = lease.token.clone();
        let publication = lease.publication_claim.clone();
        let mut progress = mounted.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.publish_mounted_typed_operation_run_admitted(mounted, &mut identity,cx,retained).await {
            Ok(()) => self.child_composition.follow_derivable_children(&mut self.stores, &mut identity).await.map_err(Into::into),
            Err(fault) => Err(fault),
        };
        mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        mounted.identity_progress = progress;
        result
    }

    async fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.publication_ownership_progress.is_some(){return Ok(());}
        let mut spent = RetainedCloneProgress::default();
        let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
        if spent != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
        }
        outcome
    }

    /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
    async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
        let started_us=cx.now_us();
        let turn = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            if cx.should_yield(){return Ok(())}
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();
            if mounted.owned_child_group.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_owned_child_operation_unit(mounted,cx).await?;
            } else if mounted.pending_child_publication.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_child_operation_unit(mounted, identity).await?;
            } else if Self::mounted_typed_interaction_writes_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity).await?;
            } else if self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await?;
            } else {
                let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                let receipt = mounted.publication_ownership_progress.take();
                if let Some((_, unit)) = receipt {
                    cx.consume_retained(unit)?;
                    *spent = spent.checked_add(unit).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    if unit.copied_items == 0 || crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent).maximum_items == 0 { return Ok(()); }
                }
                if self.stores.store.backbone_ref().is_some() {
                    self.stores.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.stores.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    }
                }
                record_typed_operation_unit(if mounted.result_page.is_some() { TypedOperationUnitKind::Page } else { TypedOperationUnitKind::Store });
                if receipt.is_none() && before == (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage) {
                    return Ok(());
                }
            }
            if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
            if mounted.owned_child_group.is_some(){let delta=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;*spent=spent.checked_add(delta)?;}
            if mounted.stage != MountedTypedCommandFullOperationStage::Publishing {
                return Ok(());
            }
            if mounted.publication.is_none() && mounted.pending_artifact_publication.is_none() && mounted.pending_child_publication.is_none() && mounted.owned_child_group.is_none() {
                return Ok(());
            }
            let Some(started_us) = started_us else { return Ok(()) };
            if cx.now_us().is_some_and(|now_us| now_us.saturating_sub(started_us) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 🕹️ Whether this operation's next publication unit is its [`InteractionWrite`] lane: every
    /// durable store lane of its emit is already drained (a selection may only name ids the
    /// document already holds), and there is at least one write left to apply. Sync and
    /// allocation-free so the async caller can branch on it before mounting anything.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🧺️ A batched durable lane empties its emit vector the moment it is admitted, so the
    /// in-flight publication itself — not the drained vector — is what still owes a turn.
    fn mounted_typed_interaction_writes_are_next(mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        !emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
    }

    /// 🕹️ Publishes this operation's whole [`InteractionWrite`] lane as ONE unit through the same
    /// `apply_interaction_writes` single-writer path `dispatch_emit` uses — the retained-tool twin
    /// of that call. It is one unit rather than one-write-per-turn because a selection is a single
    /// state-machine transition per domain: draining it write-by-write would publish intermediate
    /// selections no user ever asked for. `MountedTypedCommandFullOperation` needs no extra field:
    /// the writes live in the retained emit until taken here, and the lane is only entered once
    /// every durable lane is drained (`mounted_typed_interaction_writes_are_next`).
    async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation interaction publication lost its emit owner"));
        };
        let writes = std::mem::take(&mut emit.interaction_writes);
        let meta = mounted.meta.clone();
        if let Err(fault) = self.apply_interaction_writes(&writes, &meta, identity).await {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();
        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
        mounted.queue_page(page)
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the typed-operation twin of
    /// `dispatch_emit`'s inline interaction fold, for the ladder every app command
    /// (`dispatch_typed_command_inner`) and retained tool publishes through and that never enters
    /// `dispatch_emit`. Peels every interaction verb out of the retained emit's effect list as ONE
    /// unit (the same one-transition-per-domain reasoning as the `InteractionWrite` unit above),
    /// applies each through `apply_interaction_verb`, and widens the emit's `ui_scope` with the
    /// verbs' app-declared refresh scopes so the `Ui` page and the completion witness — published
    /// later in this same ladder — already carry them. No result page is queued: the host has
    /// nothing to acknowledge, because the verb it used to be asked to replay never leaves the
    /// reactor, and the ladder simply continues with the remaining effects next pump.
    ///
    /// 🩹 A verb that could not be applied is reported on the runtime's `[TRACE]` diagnostics line
    /// and dropped — the typed ladder has no `InvocationResult.diagnostics` lane (its only fault
    /// lane is terminal), and failing the carrying command for a pick that already happened is
    /// exactly what `fold_inline_interaction_verbs` exists to prevent.
    async fn publish_mounted_typed_inline_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner"));
        };
        let mut folded: Vec<(String, Option<DslValue>)> = Vec::new();
        if let Some(index) = self.tools.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.tools.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.tools.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub(crate) async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.tools.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.tools.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tools.tool_cancellations,cx);
            self.tools.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.tools.advance_latest_wins_admission_unit(&mut self.stores, &mut pending, retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tools.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tools.tool_operations.can_insert(operation_id) || self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
            let started = self.start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            }
            started?;
            return Ok(());
        }
        if pending.closing && pending.command_owners_are_empty() {
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "latest-wins-admission".into(),
                meta: pending.meta.clone(),
                operation: pending.operation,
                source_identity:pending.source_identity,
                canonical_revision: pending.revision,
                artifact_generation: pending.operation.generation.0,
                config_generation: 0,
                draft_generation: 0,
                presence_generation: 0,
                transient_generation: 0,
                window_config_authority: None,
                window_transient_authority: None,
                publication_lanes: &[],
                session: None,
                session_rejected: None,
                reserved_producer: None,
                completion: None,
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: None,
                cancellation_lease: pending.lease.take(),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: true,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;
            self.tools.tool_operations.insert_admitted(operation_id, mounted);
            self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
    }

    /// 🧩️ The composed lane is PUBLISHED, not discarded: `dispatch_emit_group` has just
    /// built the parent's mutations, one per touched child under the child's own
    /// content-addressed handle, and the single `UndoGroup` that names them all. The
    /// three scalars of `receipt` are what the host ACKs; this is what the host READS.
    async fn publish_mounted_typed_child_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if mounted.pending_child_publication.as_ref().is_some_and(|owner|matches!(owner.phase,PendingChildGroupPublicationPhase::Acknowledged|PendingChildGroupPublicationPhase::Closing)){return Ok(());}
        let Some(mut pending) = mounted.pending_child_publication.take() else {
            return Err(plugin_sdk_fault("typed-operation child publication lost its retained owner"));
        };
        match pending.phase {
            PendingChildGroupPublicationPhase::Acknowledged | PendingChildGroupPublicationPhase::Closing => {
                mounted.pending_child_publication=Some(pending);
                return Err(plugin_sdk_fault("original mounted child publication escaped its full-grant close frontier"));
            }
            PendingChildGroupPublicationPhase::Committed => {
                let receipt = pending.receipt.as_ref().cloned().ok_or_else(|| plugin_sdk_fault("committed child publication lost its exact result receipt"))?;
                mounted.published_artifact = true;
                self.record_typed_operation_lane(mounted);
                mounted.pending_child_publication = Some(pending);
                return mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
            }
            PendingChildGroupPublicationPhase::Faulted => {
                let fault = pending.fault().unwrap_or_else(|| plugin_sdk_fault("typed-operation child publication fault lost its bounded detail"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            PendingChildGroupPublicationPhase::Ready => {}
            PendingChildGroupPublicationPhase::Dispatching => {
                mounted.pending_child_publication = Some(pending);
                return Err(plugin_sdk_fault("typed-operation child publication re-entered a non-dispatchable phase"));
            }
        }
        match self.child_composition.reclaim_child_retirements(1, 0) {
            Ok(true) => {}
            Ok(false) => {
                mounted.pending_child_publication = Some(pending);
                return Ok(());
            }
            Err(error) => {
                let fault = pending.reject_and_fault(&error);
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_composition.child_content_generation && captured.identity_digest() == self.child_composition.child_content_root.identity_digest();
        if !parent_fresh || !child_root_fresh {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale parent or child immutable roots"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let mut seen = HashSet::with_capacity(pending.child_emits.len());
        for child in pending.child_emits.iter() {
            if !seen.insert((child.slot.clone(), child.child_id.clone())) {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication contains a duplicate exact target"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            let expected_revision = match captured.revision(&child.slot, &child.child_id) {
                Ok(revision) => revision,
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            };
            let Some(expected_dialect) = captured.dialect(&child.slot, &child.child_id) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from its captured root"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            let Some(entry) = self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from the live member registry"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            if entry.reference.dialect != expected_dialect || entry.member.content_revision().await != expected_revision {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale member identity, dialect, or revision"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        if mounted.cancellation_lease.as_ref().expect("claimed child publication retains its cancellation lease").is_cancelled().await {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication was cancelled before group linearization"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        if let Err(error) = pending.begin_dispatch() {
            let fault = pending.reject_and_fault(&error);
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.stores.store.generation_now();
                mounted.canonical_revision = self.stores.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_composition.child_content_generation };
                if self.tools.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
                    let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation composed gesture receiver is saturated"));
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                if let Err(error) = pending.commit(receipt.clone()) {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                mounted.pending_child_publication = Some(pending);
                mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
            }
            Err(fault) => {
                let fault = pending.reject_and_fault(&fault);
                mounted.pending_child_publication = Some(pending);
                Err(fault)
            }
        }
    }

    pub(crate) fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.stores.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.tools.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if Self::mounted_typed_interaction_writes_are_next(mounted) || self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.stores.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.stores.advance_mounted_window_config_refresh(mounted, retained); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.stores.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.ingest.reclaim_document_snapshot_read_returns(&self.stores)?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.stores.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.stores.config_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.stores.draft_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.stores.presence_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.stores.transient_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.stores.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.stores.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt) => {
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.stores.store.content_revision_now();
                            mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                            mounted.operation.generation = semio_framework_job::Generation(receipt.generation_after);
                        }
                        TypedOperationResultLane::Config => {
                            mounted.published_config = true;
                            mounted.config_generation = receipt.generation_after;
                        }
                        TypedOperationResultLane::Draft => mounted.draft_generation = receipt.generation_after,
                        TypedOperationResultLane::Presence => mounted.presence_generation = receipt.generation_after,
                        TypedOperationResultLane::Transient => mounted.transient_generation = receipt.generation_after,
                        TypedOperationResultLane::WindowConfig => {
                            mounted.published_window_config = true;
                            mounted.pending_window_config_receipt = Some(receipt);
                            return self.stores.advance_mounted_window_config_refresh(mounted, retained);
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    let token = mounted.next_token();
                    mounted.queue_page(TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)?)
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant"));}
                    assert!(mounted.publication_ownership_progress.is_none(),"typed preparation cannot overwrite an unreturned original receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused => {
                    let reason=pending.preparation_refusal().ok_or_else(||plugin_sdk_fault("typed preparation refused without its original retained failure"))?;
                    let ownership=reason.retained_progress();
                    assert!(mounted.publication_ownership_progress.is_none(),"typed refusal cannot overwrite its unreturned actual receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Err(plugin_sdk_fault("artifact-store publication awaits an ACK without its retained host page")),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK")),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let page = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, &framed[..len])?
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane"));
                }
                if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else {
                    TypedOperationResultPage::try_serialize(
                        token,
                        TypedOperationResultLane::Download,
                        &DownloadResultPayload { filename: download.filename.as_str(), mime_type: download.mime_type.as_str(), encoding: download.encoding.as_deref(), bytes: download.chunks.bytes() },
                    )?
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let preparation=match emit.prepare_child_one(retained){Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy"));}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault);
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy"));}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault)}
                if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
                    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
                    || (!emit.window_config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowConfig))
                    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
                    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                    || ((!emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
                    || (!emit.interaction_writes.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Interaction))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.media.require_operation_emitting_kind(&mounted.verb, self.tools.declared_dispatch_kind(&self.host_events, &mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if let Some(fault) = InstanceExtensions::mutation_refusal(self, &mounted.verb) {
                        return Err(fault);
                    }
                    if let Some(pending_txn_id) = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.txn.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                            emit.artifact_mutations.clear();
                            return Ok(());
                        }
                    }
                }
                if let Some(fault) = tool_transaction_shape_fault(
                    &mounted.verb,
                    emit.transaction.as_ref(),
                    emit.transaction_phase,
                    !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty(),
                    !emit.artifact_mutations.is_empty(),
                    emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps),
                ) {
                    return Err(fault);
                }
                if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty() {
                    if let Some(fault) = InstanceExtensions::document_refusal(self, &mounted.verb) {
                        return Err(fault);
                    }
                }
                self.history.close_streamed_transaction_unit(&mut self.stores, emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.child_composition.admit_private_child_group_frame(operation, captured_millis, u64::MAX, grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.child_composition.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.stores.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.stores.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.stores.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.stores.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.stores.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.stores.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.stores.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner"));
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    MediaTransfers::<A>::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stores.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.tools.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.tools.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.tools.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
                                    return Err(plugin_sdk_fault("typed-operation completion receiver is saturated"));
                                }
                                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, b"typed-operation-complete")?
                            }
                        }
                    }
                }
            }
        };
        mounted.queue_page(page)
    }

    pub(crate) async fn dispatch_typed_command_inner(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if self.tools.live_runtime_instance_id != Some(meta.instance_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.live-instance"), format!("typed command '{}' does not belong to the mounted live app instance", admission.verb)));
        }
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        self.deliver_utility_host_events(meta, identity, cx).await?;
        let verb = A::command_id(&command).await.to_string();
        if admission.verb != verb {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.command-identity"), format!("exact admitted command '{}' decoded as '{verb}'", admission.verb)));
        }
        if let Some(fault) = self.ingest.document_loading_refusal(&self.tools, &verb) {
            return Err(fault);
        }
        let Some(operation_id) = self.media.admit_typed_operation_slot(&self.pending, &self.tools) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"));
        };
        if self.host_events.delivering_host_event {
            self.host_events.host_event_operations.insert(operation_id.0);
        }
        if let Some(tag) = self.tool_machines.ingress.take() {
            self.tool_machines.bind(operation_id.0, tag);
        }
        if let QualifiedToolProof::AppOwned(registration) = &admission.proof {
            {
                let ordinary = (registration.latest_wins_target)(command.as_ref()).is_none();
                let disposer = (registration.latest_wins_command_disposer)()
                    .and_then(|owner| owner.downcast::<Box<dyn ArtifactOwnedDisposer<A::Command>>>().ok())
                    .map(|owner| *owner)
                    .ok_or_else(|| plugin_sdk_fault("latest-wins target requires its exact compiler-owned command retirement factory"))?;
                let revision = self.stores.store.content_revision_now();
                let generation = semio_framework_job::Generation(self.stores.store.generation_now());
                let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")));
                let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, operation_id.0);
                self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0);
                self.tools.latest_wins_commands.insert_admitted(
                    operation_id.0,
                    PendingLatestWinsCommand {
                        command: ManuallyDrop::new(Some(command)),
                        disposer,
                        admission: Some(admission),
                        meta: meta.clone(),
                        operation,
                        source_identity:(cx.operation(),cx.generation()),
                        revision,
                        lease: None,
                        key_copy: None,
                        key: None,
                        retired_key: None,
                        lookup_started: false,
                        accepted: ordinary,
                        restarting: false,
                        closing: false,
                        receipt:None,
                        fault:ManuallyDrop::new(None),fault_retirement:None,
                    },
                );
                self.tools.latest_wins_order.push(operation_id.0).expect("exact FIFO command slot was pre-admitted");
                let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
                started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
                return Ok(started);
            }
        }
        Err(plugin_sdk_fault("typed admission requires its exact retained command owner"))
    }

    /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
    /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
    /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
    /// the same verb against different state.
    pub(super) async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let canonical_base_revision = self.stores.store.content_revision();
        let window_config_authority = self.stores.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.stores.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_composition.child_content_root)),
            draft_snapshot: self.stores.draft_store.snapshot_root(),
            interaction_state: self.stores.interaction_store.snapshot_root(),
            interaction_hover: self.interaction.interaction_hover.clone(),
            presence_peers: self.stores.presence_store.peers_root(),
            transient: self.stores.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.roster.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.stores.store.generation()),
            config_generation: self.stores.config_store.generation(),
            draft_generation: self.stores.draft_store.generation(),
            presence_generation: self.stores.presence_store.generation().await,
            transient_generation: self.stores.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    }

    /// 🧪️ Observes actual capture authority without exposing production dispatch preparation.
    #[cfg(test)]
    pub(crate) async fn test_capture_presence_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<(std::sync::Arc<PeerPresenceRoot>, std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>), Fault> {
        let TypedCommandRoots { peer_presence, presence_peers, .. } = self.capture_typed_command_roots(command, meta).await?;
        Ok((peer_presence, presence_peers))
    }

    /// 🧪️ Mounts an exact typed admission so laws can observe the boundary before producer completion.
    #[cfg(test)]
    pub(crate) async fn test_mount_typed_command(&mut self, command: A::Command, verb: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, verb, &payload, 1).await?;
        self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await
    }

    /// 🧵️ Captures immutable roots and mounts one admitted worker turn. Ephemeral producers
    /// execute inside their app-owned job and publish through its declared completion lanes.
    async fn start_typed_command_operation(
        &mut self,
        command: Box<A::Command>,
        admission: AdmittedToolCommand,
        meta: &ActionMeta,
        operation_id: semio_framework_job::OperationId,
        cancellation_lease: ToolCancellationLease,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        assert!(self.tools.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
        assert_eq!(self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
        let verb = admission.verb.clone();
        let TypedCommandRoots {
            snapshot,
            config,
            history,
            children,
            draft_snapshot,
            interaction_state,
            interaction_hover,
            presence_peers,
            transient,
            peer_presence,
            canonical_base_revision,
            base_revision,
            generation,
            config_generation,
            draft_generation,
            presence_generation,
            transient_generation,
            window_config_authority,
            window_transient_authority,
        } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let parent_document_id = self.stores.store.envelope().id.clone();
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_job_controller_id, base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        assert_eq!(cancellation_lease.key.base_revision, base_revision, "fresh worker receives a revision-rebound cancellation lease");
        assert_eq!(cancellation_lease.key.generation, generation, "fresh worker receives a generation-rebound cancellation lease");
        let payload_schema_id = admission.proof.schema_id();
        let qualified_key = admission.proof.key();
        let publication_lanes = admission.proof.publication_lanes();
        let completion = ArtifactToolCompletion::<A>::new();
        let mut identity_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !cancellation_lease.token.is_cancelled_now() && !cancellation_lease.publication_claim.is_cancelled();
        let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(identity_observer);
        completion.install_authoring_identity(identity, &cancellation_lease)?;
        let retained_wire_admission = admission.wire_admission.clone();
        let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
        let mut retained_wire = Some(admission.raw_wire);
        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
        let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
        let mut request_snapshot_read = if app_owned_retained_route { Some(self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
        let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
        let owned_context = if app_owned_retained_route {
            let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
            let gesture = self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta,artifact_owned_tool_birth_grant(),artifact_owned_tool_handoff_grant())?;
            Some(ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                canonical_base_revision,
                draft_generation,
                transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&children),
                    draft: std::sync::Arc::clone(&draft_snapshot),
                    transient: std::sync::Arc::clone(&transient),
                    window_config: window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())
            .with_gesture(gesture),
            artifact_owned_tool_birth_grant(),
        ).0) } else { None };
        let operation_spec = match admission.proof.clone() {
            QualifiedToolProof::Bounded(_) => {
                let job = TypedCommandFullOperationJob::<A> {
                    operation: None,
                    parent_document_id: Some(parent_document_id.clone()),
                    command: Some(std::sync::Arc::from(command)),
                    snapshot: Some(snapshot),
                    config: Some(config),
                    history: Some(history),
                    children: Some(std::sync::Arc::clone(&children)),
                    draft: Some(draft_snapshot),
                    interaction_state: Some(interaction_state),
                    interaction_hover: Some(std::sync::Arc::new(interaction_hover)),
                    peer_presence: Some(peer_presence),
                    presence_local: Some(self.stores.presence_store.local_read().map_err(Fault::from)?),
                    presence_peers: Some(presence_peers),
                    transient: Some(transient),
                    contract: admission.proof.contract(),
                    decoded_items: admission.decoded_items,
                    phase: TypedCommandFullOperationPhase::Prepare,
                    prepare_cursor: 0,
                    reducer_cursor: 0,
                    output: Some(completion.clone()),
                    state: [0; 3],
                    fault_message: b"",
                    pending: None,
                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
                    closing: false,
                };
                semio_framework::ToolOperationSpec::new(self.tools.tool_job_controller_id.clone(), verb.clone(), payload_schema_id, job, operation)
            }
            QualifiedToolProof::AppOwned(_) => A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
                command,
                raw_wire: raw_input.clone(),
                operation,
                controller_id: qualified_key.controller_id,
                tool_id: qualified_key.tool_id,
                payload_schema_id,
                contract: admission.proof.contract(),
                decoded_items: admission.decoded_items,
                app_instance_id: meta.instance_id,
                parent_document_id,
                canonical_base_revision,
                authoring_seed: self.stores.authoring_seed(&meta.actor),
                snapshot,
                snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                config,
                window_config: request_window_config.take(),
                history,
                interaction_state,
                interaction_hover: std::sync::Arc::new(interaction_hover),
                context: owned_context.expect("app-owned route retains its original admitted context"),
                instance_operation_owner: self.tools.instance_operation_owner.clone(),
                output_chunks: output_chunks.clone(),
                completion: completion.clone(),
            })
            .await?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?,
            QualifiedToolProof::FrameworkOwned(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.framework-route"), format!("framework-owned tool '{verb}' cannot enter the application typed-command reducer")));
            }
        };
        let dispatch = if app_owned_retained_route {
            self.tools.tool_jobs
                .dispatch_wire_retained_with_spec(&retained_wire_admission, retained_wire.take().expect("app-owned retained wire owner transfers once"), None, operation_spec)
                .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?
        } else {
            self.tools.tool_jobs.dispatch(operation_spec).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?
        };
        let session=None;let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,(cx.operation(),cx.generation())));
        self.tools.tool_operations.insert_admitted(
            operation_id.0,
            MountedTypedCommandFullOperation {
                verb: verb.clone(),
                meta: meta.clone(),
                operation,
                source_identity:(cx.operation(),cx.generation()),
                canonical_revision: canonical_base_revision,
                artifact_generation: generation.0,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
                publication_lanes,
                session,
                session_rejected,
                reserved_producer: None,
                completion: Some(completion),
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: (!app_owned_retained_route).then_some(raw_input),
                output_chunks: Some(output_chunks),
                cancellation_lease: Some(cancellation_lease),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: false,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::clone(&children)),
                captured_child_content_generation: self.child_composition.child_content_generation,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Worker,
            },
        );
        self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
        Ok(started)
    }

    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> {
        let command = Box::new(command);
        async move {
            let log_generation_before = self.history.log_generation;
            let verb = A::command_id(command.as_ref()).await;
            let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
            let admission = self.tools.admit_command_wire(&self.stores, verb, &payload, 1).await?;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await?;
            Ok(self.finish_recorded(log_generation_before, verb, result).await)
        }
    }

    /// 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
    ///
    /// 🔒️ Contract §2.3 clause 1 — `import` is not a string action (see `VIEWER_REJECTED_ACTION_IDS`'s
    /// own doc), so it is guarded here instead of `dispatch_action`.
    pub(super) async fn dispatch_import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if A::ROLE == AppRole::Viewer {
            return Err(viewer_read_only_fault("import"));
        }
        let proof = self.tools.qualified_tool_proof(&self.stores, "import-media")?;
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), &[]).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), "media import resolved outside its exact controller/owner/factory/tool/schema proof"));
        }
        let (authority,source)=self.media.pending_media_import_permit.take().ok_or_else(||plugin_sdk_fault("media source requires its original pre-admitted import permit"))?;
        if (source.0,source.1)!=(cx.operation(),cx.generation()){*self.media.pending_media_import_permit=Some((authority,source));return Err(plugin_sdk_fault("media import requires its unchanged original caller"))}
        let (job, completion) = match self.build_artifact_reserved_media_job(port, media, meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault); } };
        let raw = Vec::new();
        self.dispatch_reserved_original_producer("import-media",&raw,1,job,completion,meta,Some(admission),authority, cx).await
    }

    /// 🧮️ B1: dispatches a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` against
    /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    pub(super) async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.receive_configuration_failure(cx)?{return Ok(None)}
        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_completion.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
        let proof=self.tools.qualified_tool_proof(&self.stores, "configuration-binary")?;let key=proof.key();
        let admission=match self.tools.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
        type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
        let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
        let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
        self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));
        self.pending.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
        let authority=self.pending.pending_configuration_permit.take().unwrap();
        let permit=match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut self.stores, "configuration-binary", &[], 1, job, meta, Some(admission), authority, cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending.pending_configuration_permit=Some(permit);*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
        self.pending.pending_configuration_permit=Some(permit);
        let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
        match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
        self.finish_configuration_command(meta,identity,cx).await
    }

    async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration commit requires its unchanged original caller").into())}
        if let Some(owner)=self.pending.pending_configuration_completion.as_mut(){
            let demand=if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.demand(cx.retained_grant().maximum_copy_bytes)?};
            let terminal=owner.terminal_is_empty();let mut removed=false;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{if terminal{removed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{owner.step(grant)}})?;
            if removed{self.pending.pending_configuration_completion.take();}return Ok(None)
        }
        let permit=self.pending.pending_configuration_permit.as_ref().expect("configuration commit retains its admitted permit");self.stores.validate_framework_reserved_commit("configuration-binary", permit).await?;
        if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let command=self.pending.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
    }

    /// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a
    /// command-log row for EVERY dispatch, view actions included (`ActionKind::View` is also the
    /// fallback for an unresolved verb), so the log generation advancing is not by itself evidence that
    /// anything a user would call history happened. A `View` verb paints, it does not command; an
    /// `Interaction` verb moves selection/hover, which `dispatch_interaction_action` has always
    /// declared as kept out of the panel. Handing either one a history patch made every background
    /// tick a history update — measured 2026-09-09 20:55 as 90 host `readHistory` round trips beside
    /// 252 `fillBuildTick`s in 35 s.
    /// 🚫️async: E1 pure classification consumed by sync-only `matches!` call sites — see R9
    fn verb_dirties_history_panel(kind: Option<ActionKind>) -> bool {
        !matches!(kind, Some(ActionKind::View | ActionKind::Interaction))
    }

    /// 🧾️ Attaches the command-log delta directly to the accepted invocation response. History is
    /// no longer a plugin-rendered panel body, so this deliberately leaves `ui_scope` untouched.
    /// `verb` is the DECLARED action/command id — every caller must pass the real one, never a generic
    /// placeholder, or the kind lookup below silently resolves to nothing and the skip is unreachable.
    pub(super) async fn finish_recorded(&mut self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
        if self.history.log_generation == log_generation_before {
            return result;
        }
        let kind = self.tools.registry.get(verb).map(|definition| definition.kind).or_else(|| self.tools.registry.get_command(verb).map(|command| command.kind));
        if !Self::verb_dirties_history_panel(kind) {
            return result;
        }
        result.history_patch = self.history_patch(false).await.ok();
        result
    }

    pub(super) fn retained_fields_terminal_is_empty(&self) -> bool {
        self.retained_fields_own_terminal_is_empty() && self.child_composition.children.is_empty() && self.interaction.local_interaction_query.is_none() && self.ingest.snapshot_read_returns_terminal_is_empty(&self.stores, &self.interaction)
    }
}

```

head
```

```

stash
```

use super::*;

use super::docs_instance_root::{artifact_handle_of, tool_transaction_shape_fault, typed_operation_document_is_fresh, viewer_read_only_fault};
use super::docs_maintenance_ladder::{removed};
use super::typed_ops_instance_root::{store_publication_fault};
use super::typed_ops_plugin_app_impl::{PUBLICATION_RETURNED_ROOT_ALLOWANCE};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🩺️ How many consecutive publication units advanced the SAME operation without changing any of
    /// [`Self::typed_operation_stall_witness`]'s three facts — `Some(streak)` only on a power of two at
    /// or above [`TYPED_OPERATION_STALL_WITNESS_FLOOR`], so one stalled operation costs at most a
    /// handful of lines however long it spins.
    ///
    /// 🐛️ A mounted operation that never publishes its terminal keeps the actor in `MoreWork` with no
    /// effect, no patch and no fault, which every other instrument reads as silence: the host's own
    /// drain polls forever, the reactor's more-work streak names only `typed_operation` as its source,
    /// and nothing says WHICH arm of the publication ladder is not moving. Measured 2026-09-20 on the
    /// signed-in `s` Home, whose `applyDirectoryEventPage` operation spun for the whole session
    /// (ticket 26/09/18 S8).
    fn typed_operation_stall_streak(before: (u64, u8, u8, u64), after: (u64, u8, u8, u64)) -> Option<u64> {
        static WITNESS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        static STREAK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = (after.0.wrapping_shl(16) | u64::from(after.1).wrapping_shl(8) | u64::from(after.2)) ^ after.3.wrapping_mul(TYPED_OPERATION_STALL_PROGRESS_MIX);
        if before != after || WITNESS.swap(key, std::sync::atomic::Ordering::Relaxed) != key {
            STREAK.store(0, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        let streak = STREAK.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
        (streak >= TYPED_OPERATION_STALL_WITNESS_FLOOR && streak.is_power_of_two()).then_some(streak)
    }

    pub(crate) async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let before = self.tools.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.tools.typed_operation_stall_witness()) {
            if let Some(streak) = Self::typed_operation_stall_streak(before, after) {
                let verb = self.tools.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
                if after.1 == TYPED_OPERATION_STALL_PUBLISHING_STAGE && streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    self.tools.fault_stalled_typed_operation_publication(after.0, streak)?;
                }
            }
        }
        advanced
    }

    /// ♻️ A retiring operation answers `has_runnable_typed_operations`, so the turn driver that reports it
    /// runnable must be the one that releases its slot — see [`Self::retire_typed_operation_run`].
    ///
    /// 🕹️ "After EVERY artifact (document) dispatch, re-derive fresh topology and prune any
    /// selection/hover id no longer present" — the clause `dispatch_emit_inner` carries, which
    /// the migrated publication ladder never reaches (see the emit backstops above it). A
    /// migrated dispatch's document change lands on the Artifact lane, so this is where the
    /// revalidation belongs; `mounted` is out of the registry here, so the async pass is free
    /// to borrow the app, and the flag makes it exactly one pass per operation.
    async fn advance_typed_operation_publication_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if !self.tools.latest_wins_commands.is_empty() && (self.tools.latest_wins_turn || self.tools.tool_operations.is_empty()) {
            self.tools.latest_wins_turn = false;
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.tools.latest_wins_turn = true;
        let Some((index, operation_id)) = self.tools.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
            let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();
            if !mounted.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
            let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
            mounted.cancellation_lease=self.tools.tool_cancellations.begin_keyed(&key,cx)?;
            if let Some(lease)=mounted.cancellation_lease.as_ref(){
                if mounted.terminal_fault.is_some()||mounted.user_cancel_requested{lease.cancel();}
                let maximum=({ let __a0 = &self.tools.tool_operations.get(operation_id).unwrap().verb; self.tools.qualified_tool_proof(&self.stores, __a0) })?.contract().max_output_bytes;
                let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();let lease=mounted.cancellation_lease.as_ref().unwrap();
                let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
                mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);
            }
            return Ok(());
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker) {
            record_typed_operation_unit(TypedOperationUnitKind::Worker);
            return self.tools.drive_typed_operation_worker(&self.mounted_policy, operation_id, cx);
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
            record_typed_operation_unit(TypedOperationUnitKind::Retirement);
            return self.retire_typed_operation_run(operation_id,cx);
        }
        if !self.tools.tool_operations.get_mut(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Publishing) {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        }
        let mut mounted =
            self.tools.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority"), "typed-operation publication owner changed before one-unit transfer"))?;
        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;
        let mut driver_result = Ok(());
        if let Err(failure)=outcome{self.tools.tool_operations.insert_admitted(operation_id,mounted);return Err(failure);}
        mounted.publication_attempt=0;
        if driver_result.is_ok() && mounted.published_artifact && !mounted.interaction_revalidated {
            mounted.interaction_revalidated = true;
            let meta = mounted.meta.clone();
            let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
            let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
            let token = lease.token.clone();
            let publication = lease.publication_claim.clone();
            let mut progress = mounted.identity_progress;
            let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
            driver_result = self.revalidate_interaction_on_document_change(&meta, &mut identity).await;
            mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
            drop(observer);
            mounted.identity_progress = progress;
        }
        self.tools.tool_operations.insert_admitted(operation_id, mounted);
        driver_result.map_err(Into::into)
    }

    /// 🏃️ Drives ONE mounted operation's publication ladder for a bounded slice of this continuation
    /// unit — the exact twin of the slice [`Self::drive_typed_operation_worker`] gives a Worker-stage
    /// operation and [`Self::retire_typed_operation_run`] gives a retiring one, for the publication
    /// state machine that sits between them.
    ///
    /// 🧾️ Every ladder step — install the taken completion, stage the batch, prepare one item, fold
    /// it, retire its preparation owner, validate the cursor, preflight the commit, commit, mint the
    /// receipt — used to be its own host round trip, because the caller is driven once per reactor
    /// turn and the store's grant pages one item per call. One `deleteSelection` on a ONE-object
    /// document therefore cost 104–235 round trips (wave B44 §2.3): a document-INDEPENDENT constant
    /// that alone burns seconds of a 30-second interaction budget, and the reason a `setCamera` on a
    /// document it never reads cost 2.3 s.
    ///
    /// 🛑️ The run stops on exactly the protocol's own bounds and on nothing else: a queued result page
    /// the host must acknowledge before the operation may continue (`stage` leaves `Publishing`), a
    /// completion that is not ready yet so nothing is staged to advance, [`TYPED_OPERATION_PUBLICATION_PUMPS`],
    /// and [`INTERACTIVE_TURN_WORKER_WALL_US`]. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
    pub(crate) async fn publish_mounted_typed_operation_run(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.identity.is_none() {
            let completion = mounted.completion.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original effect completion"))?;
            let (receipt, progress) = completion.take_authoring_identity()?.ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original app-owned effect receipt"))?;
            mounted.identity = Some(receipt);
            mounted.identity_progress = progress;
        }
        let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its admitted authoring receipt"))?;
        let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original cancellation lease"))?;
        let token = lease.token.clone();
        let publication = lease.publication_claim.clone();
        let mut progress = mounted.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.publish_mounted_typed_operation_run_admitted(mounted, &mut identity,cx,retained).await {
            Ok(()) => self.child_composition.follow_derivable_children(&mut self.stores, &mut identity).await.map_err(Into::into),
            Err(fault) => Err(fault),
        };
        mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        mounted.identity_progress = progress;
        result
    }

    async fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.publication_ownership_progress.is_some(){return Ok(());}
        let mut spent = RetainedCloneProgress::default();
        let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
        if spent != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
        }
        outcome
    }

    /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
    async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
        let started_us=cx.now_us();
        let turn = semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            if cx.should_yield(){return Ok(())}
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();
            if mounted.owned_child_group.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_owned_child_operation_unit(mounted,cx).await?;
            } else if mounted.pending_child_publication.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_child_operation_unit(mounted, identity).await?;
            } else if Self::mounted_typed_interaction_writes_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity).await?;
            } else if self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await?;
            } else {
                let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                self.publish_mounted_typed_operation_unit(mounted,semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                let receipt = mounted.publication_ownership_progress.take();
                if let Some((_, unit)) = receipt {
                    cx.consume_retained(unit)?;
                    *spent = spent.checked_add(unit).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    if unit.copied_items == 0 || crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent).maximum_items == 0 { return Ok(()); }
                }
                if self.stores.store.backbone_ref().is_some() {
                    self.stores.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.stores.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    }
                }
                record_typed_operation_unit(if mounted.result_page.is_some() { TypedOperationUnitKind::Page } else { TypedOperationUnitKind::Store });
                if receipt.is_none() && before == (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage) {
                    return Ok(());
                }
            }
            if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
            if mounted.owned_child_group.is_some(){let delta=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;*spent=spent.checked_add(delta)?;}
            if mounted.stage != MountedTypedCommandFullOperationStage::Publishing {
                return Ok(());
            }
            if mounted.publication.is_none() && mounted.pending_artifact_publication.is_none() && mounted.pending_child_publication.is_none() && mounted.owned_child_group.is_none() {
                return Ok(());
            }
            let Some(started_us) = started_us else { return Ok(()) };
            if cx.now_us().is_some_and(|now_us| now_us.saturating_sub(started_us) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 🕹️ Whether this operation's next publication unit is its [`InteractionWrite`] lane: every
    /// durable store lane of its emit is already drained (a selection may only name ids the
    /// document already holds), and there is at least one write left to apply. Sync and
    /// allocation-free so the async caller can branch on it before mounting anything.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🧺️ A batched durable lane empties its emit vector the moment it is admitted, so the
    /// in-flight publication itself — not the drained vector — is what still owes a turn.
    fn mounted_typed_interaction_writes_are_next(mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        !emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
    }

    /// 🕹️ Publishes this operation's whole [`InteractionWrite`] lane as ONE unit through the same
    /// `apply_interaction_writes` single-writer path `dispatch_emit` uses — the retained-tool twin
    /// of that call. It is one unit rather than one-write-per-turn because a selection is a single
    /// state-machine transition per domain: draining it write-by-write would publish intermediate
    /// selections no user ever asked for. `MountedTypedCommandFullOperation` needs no extra field:
    /// the writes live in the retained emit until taken here, and the lane is only entered once
    /// every durable lane is drained (`mounted_typed_interaction_writes_are_next`).
    async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation interaction publication lost its emit owner"));
        };
        let writes = std::mem::take(&mut emit.interaction_writes);
        let meta = mounted.meta.clone();
        if let Err(fault) = self.apply_interaction_writes(&writes, &meta, identity).await {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();
        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
        mounted.queue_page(page)
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the typed-operation twin of
    /// `dispatch_emit`'s inline interaction fold, for the ladder every app command
    /// (`dispatch_typed_command_inner`) and retained tool publishes through and that never enters
    /// `dispatch_emit`. Peels every interaction verb out of the retained emit's effect list as ONE
    /// unit (the same one-transition-per-domain reasoning as the `InteractionWrite` unit above),
    /// applies each through `apply_interaction_verb`, and widens the emit's `ui_scope` with the
    /// verbs' app-declared refresh scopes so the `Ui` page and the completion witness — published
    /// later in this same ladder — already carry them. No result page is queued: the host has
    /// nothing to acknowledge, because the verb it used to be asked to replay never leaves the
    /// reactor, and the ladder simply continues with the remaining effects next pump.
    ///
    /// 🩹 A verb that could not be applied is reported on the runtime's `[TRACE]` diagnostics line
    /// and dropped — the typed ladder has no `InvocationResult.diagnostics` lane (its only fault
    /// lane is terminal), and failing the carrying command for a pick that already happened is
    /// exactly what `fold_inline_interaction_verbs` exists to prevent.
    async fn publish_mounted_typed_inline_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner"));
        };
        let mut folded: Vec<(String, Option<DslValue>)> = Vec::new();
        if let Some(index) = self.tools.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.tools.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.tools.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub(crate) async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.tools.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.tools.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tools.tool_cancellations,cx);
            self.tools.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.tools.advance_latest_wins_admission_unit(&mut self.stores, &mut pending, retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tools.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tools.tool_operations.can_insert(operation_id) || self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
            let started = self.start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            }
            started?;
            return Ok(());
        }
        if pending.closing && pending.command_owners_are_empty() {
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "latest-wins-admission".into(),
                meta: pending.meta.clone(),
                operation: pending.operation,
                source_identity:pending.source_identity,
                canonical_revision: pending.revision,
                artifact_generation: pending.operation.generation.0,
                config_generation: 0,
                draft_generation: 0,
                presence_generation: 0,
                transient_generation: 0,
                window_config_authority: None,
                window_transient_authority: None,
                publication_lanes: &[],
                session: None,
                session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                producer_source:None,producer_request:None,producer_request_retirement:None,
                reserved_producer: None,
                completion: None,
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: None,
                cancellation_lease: pending.lease.take(),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: true,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;
            self.tools.tool_operations.insert_admitted(operation_id, mounted);
            self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
    }

    /// 🧩️ The composed lane is PUBLISHED, not discarded: `dispatch_emit_group` has just
    /// built the parent's mutations, one per touched child under the child's own
    /// content-addressed handle, and the single `UndoGroup` that names them all. The
    /// three scalars of `receipt` are what the host ACKs; this is what the host READS.
    async fn publish_mounted_typed_child_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if mounted.pending_child_publication.as_ref().is_some_and(|owner|matches!(owner.phase,PendingChildGroupPublicationPhase::Acknowledged|PendingChildGroupPublicationPhase::Closing)){return Ok(());}
        let Some(mut pending) = mounted.pending_child_publication.take() else {
            return Err(plugin_sdk_fault("typed-operation child publication lost its retained owner"));
        };
        match pending.phase {
            PendingChildGroupPublicationPhase::Acknowledged | PendingChildGroupPublicationPhase::Closing => {
                mounted.pending_child_publication=Some(pending);
                return Err(plugin_sdk_fault("original mounted child publication escaped its full-grant close frontier"));
            }
            PendingChildGroupPublicationPhase::Committed => {
                let receipt = pending.receipt.as_ref().cloned().ok_or_else(|| plugin_sdk_fault("committed child publication lost its exact result receipt"))?;
                mounted.published_artifact = true;
                self.record_typed_operation_lane(mounted);
                mounted.pending_child_publication = Some(pending);
                return mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
            }
            PendingChildGroupPublicationPhase::Faulted => {
                let fault = pending.fault().unwrap_or_else(|| plugin_sdk_fault("typed-operation child publication fault lost its bounded detail"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            PendingChildGroupPublicationPhase::Ready => {}
            PendingChildGroupPublicationPhase::Dispatching => {
                mounted.pending_child_publication = Some(pending);
                return Err(plugin_sdk_fault("typed-operation child publication re-entered a non-dispatchable phase"));
            }
        }
        match self.child_composition.reclaim_child_retirements(1, 0) {
            Ok(true) => {}
            Ok(false) => {
                mounted.pending_child_publication = Some(pending);
                return Ok(());
            }
            Err(error) => {
                let fault = pending.reject_and_fault(&error);
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_composition.child_content_generation && captured.identity_digest() == self.child_composition.child_content_root.identity_digest();
        if !parent_fresh || !child_root_fresh {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale parent or child immutable roots"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let mut seen = HashSet::with_capacity(pending.child_emits.len());
        for child in pending.child_emits.iter() {
            if !seen.insert((child.slot.clone(), child.child_id.clone())) {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication contains a duplicate exact target"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            let expected_revision = match captured.revision(&child.slot, &child.child_id) {
                Ok(revision) => revision,
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            };
            let Some(expected_dialect) = captured.dialect(&child.slot, &child.child_id) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from its captured root"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            let Some(entry) = self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from the live member registry"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            if entry.reference.dialect != expected_dialect || entry.member.content_revision().await != expected_revision {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale member identity, dialect, or revision"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        if mounted.cancellation_lease.as_ref().expect("claimed child publication retains its cancellation lease").is_cancelled().await {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication was cancelled before group linearization"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        if let Err(error) = pending.begin_dispatch() {
            let fault = pending.reject_and_fault(&error);
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.stores.store.generation_now();
                mounted.canonical_revision = self.stores.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_composition.child_content_generation };
                if self.tools.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
                    let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation composed gesture receiver is saturated"));
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                if let Err(error) = pending.commit(receipt.clone()) {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                mounted.pending_child_publication = Some(pending);
                mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
            }
            Err(fault) => {
                let fault = pending.reject_and_fault(&fault);
                mounted.pending_child_publication = Some(pending);
                Err(fault)
            }
        }
    }

    pub(crate) fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.stores.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.tools.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if Self::mounted_typed_interaction_writes_are_next(mounted) || self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.stores.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.stores.advance_mounted_window_config_refresh(mounted, retained); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.stores.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.ingest.reclaim_document_snapshot_read_returns(&self.stores)?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.stores.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.stores.config_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.stores.draft_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.stores.presence_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.stores.transient_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.stores.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.stores.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt) => {
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.stores.store.content_revision_now();
                            mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                            mounted.operation.generation = semio_framework_job::Generation(receipt.generation_after);
                        }
                        TypedOperationResultLane::Config => {
                            mounted.published_config = true;
                            mounted.config_generation = receipt.generation_after;
                        }
                        TypedOperationResultLane::Draft => mounted.draft_generation = receipt.generation_after,
                        TypedOperationResultLane::Presence => mounted.presence_generation = receipt.generation_after,
                        TypedOperationResultLane::Transient => mounted.transient_generation = receipt.generation_after,
                        TypedOperationResultLane::WindowConfig => {
                            mounted.published_window_config = true;
                            mounted.pending_window_config_receipt = Some(receipt);
                            return self.stores.advance_mounted_window_config_refresh(mounted, retained);
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    let token = mounted.next_token();
                    mounted.queue_page(TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)?)
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant"));}
                    assert!(mounted.publication_ownership_progress.is_none(),"typed preparation cannot overwrite an unreturned original receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused => {
                    let reason=pending.preparation_refusal().ok_or_else(||plugin_sdk_fault("typed preparation refused without its original retained failure"))?;
                    let ownership=reason.retained_progress();
                    assert!(mounted.publication_ownership_progress.is_none(),"typed refusal cannot overwrite its unreturned actual receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Err(plugin_sdk_fault("artifact-store publication awaits an ACK without its retained host page")),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK")),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let page = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, &framed[..len])?
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane"));
                }
                if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else {
                    TypedOperationResultPage::try_serialize(
                        token,
                        TypedOperationResultLane::Download,
                        &DownloadResultPayload { filename: download.filename.as_str(), mime_type: download.mime_type.as_str(), encoding: download.encoding.as_deref(), bytes: download.chunks.bytes() },
                    )?
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let preparation=match emit.prepare_child_one(retained){Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy"));}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault);
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy"));}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault)}
                if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
                    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
                    || (!emit.window_config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowConfig))
                    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
                    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                    || ((!emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
                    || (!emit.interaction_writes.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Interaction))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.media.require_operation_emitting_kind(&mounted.verb, self.tools.declared_dispatch_kind(&self.host_events, &mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if let Some(fault) = InstanceExtensions::mutation_refusal(self, &mounted.verb) {
                        return Err(fault);
                    }
                    if let Some(pending_txn_id) = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff_cold(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.txn.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                            emit.artifact_mutations.clear();
                            return Ok(());
                        }
                    }
                }
                if let Some(fault) = tool_transaction_shape_fault(
                    &mounted.verb,
                    emit.transaction.as_ref(),
                    emit.transaction_phase,
                    !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty(),
                    !emit.artifact_mutations.is_empty(),
                    emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps),
                ) {
                    return Err(fault);
                }
                if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty() {
                    if let Some(fault) = InstanceExtensions::document_refusal(self, &mounted.verb) {
                        return Err(fault);
                    }
                }
                self.history.close_streamed_transaction_unit(&mut self.stores, emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.child_composition.admit_private_child_group_frame(operation, captured_millis, u64::MAX, grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.child_composition.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.stores.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.stores.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.stores.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.stores.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.stores.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.stores.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.stores.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner"));
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    MediaTransfers::<A>::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stores.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.tools.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.tools.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.tools.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
                                    return Err(plugin_sdk_fault("typed-operation completion receiver is saturated"));
                                }
                                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, b"typed-operation-complete")?
                            }
                        }
                    }
                }
            }
        };
        mounted.queue_page(page)
    }

    pub(crate) async fn dispatch_typed_command_inner(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if self.tools.live_runtime_instance_id != Some(meta.instance_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.live-instance"), format!("typed command '{}' does not belong to the mounted live app instance", admission.verb)));
        }
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        self.deliver_utility_host_events(meta, identity, cx).await?;
        let verb = A::command_id(&command).await.to_string();
        if admission.verb != verb {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.command-identity"), format!("exact admitted command '{}' decoded as '{verb}'", admission.verb)));
        }
        if let Some(fault) = self.ingest.document_loading_refusal(&self.tools, &verb) {
            return Err(fault);
        }
        let Some(operation_id) = self.media.admit_typed_operation_slot(&self.pending, &self.tools) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"));
        };
        if self.host_events.delivering_host_event {
            self.host_events.host_event_operations.insert(operation_id.0);
        }
        if let Some(tag) = self.tool_machines.ingress.take() {
            self.tool_machines.bind(operation_id.0, tag);
        }
        if let QualifiedToolProof::AppOwned(registration) = &admission.proof {
            {
                let ordinary = (registration.latest_wins_target)(command.as_ref()).is_none();
                let disposer = (registration.latest_wins_command_disposer)()
                    .and_then(|owner| owner.downcast::<Box<dyn ArtifactOwnedDisposer<A::Command>>>().ok())
                    .map(|owner| *owner)
                    .ok_or_else(|| plugin_sdk_fault("latest-wins target requires its exact compiler-owned command retirement factory"))?;
                let revision = self.stores.store.content_revision_now();
                let generation = semio_framework_job::Generation(self.stores.store.generation_now());
                let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")));
                let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, operation_id.0);
                self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0);
                self.tools.latest_wins_commands.insert_admitted(
                    operation_id.0,
                    PendingLatestWinsCommand {
                        command: ManuallyDrop::new(Some(command)),
                        disposer,
                        admission: Some(admission),
                        meta: meta.clone(),
                        operation,
                        source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                        revision,
                        lease: None,
                        key_copy: None,
                        key: None,
                        retired_key: None,
                        lookup_started: false,
                        accepted: ordinary,
                        restarting: false,
                        closing: false,
                        receipt:None,
                        fault:ManuallyDrop::new(None),fault_retirement:None,
                    },
                );
                self.tools.latest_wins_order.push(operation_id.0).expect("exact FIFO command slot was pre-admitted");
                let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
                started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
                return Ok(started);
            }
        }
        Err(plugin_sdk_fault("typed admission requires its exact retained command owner"))
    }

    /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
    /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
    /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
    /// the same verb against different state.
    pub(super) async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let canonical_base_revision = self.stores.store.content_revision();
        let window_config_authority = self.stores.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.stores.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_composition.child_content_root)),
            draft_snapshot: self.stores.draft_store.snapshot_root(),
            interaction_state: self.stores.interaction_store.snapshot_root(),
            interaction_hover: self.interaction.interaction_hover.clone(),
            presence_peers: self.stores.presence_store.peers_root(),
            transient: self.stores.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.roster.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.stores.store.generation()),
            config_generation: self.stores.config_store.generation(),
            draft_generation: self.stores.draft_store.generation(),
            presence_generation: self.stores.presence_store.generation().await,
            transient_generation: self.stores.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    }

    /// 🧪️ Observes actual capture authority without exposing production dispatch preparation.
    #[cfg(test)]
    pub(crate) async fn test_capture_presence_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<(std::sync::Arc<PeerPresenceRoot>, std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>), Fault> {
        let TypedCommandRoots { peer_presence, presence_peers, .. } = self.capture_typed_command_roots(command, meta).await?;
        Ok((peer_presence, presence_peers))
    }

    /// 🧪️ Mounts an exact typed admission so laws can observe the boundary before producer completion.
    #[cfg(test)]
    pub(crate) async fn test_mount_typed_command(&mut self, command: A::Command, verb: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, verb, &payload, 1).await?;
        self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await
    }

    /// 🧵️ Captures immutable roots and mounts one admitted worker turn. Ephemeral producers
    /// execute inside their app-owned job and publish through its declared completion lanes.
    async fn start_typed_command_operation(
        &mut self,
        command: Box<A::Command>,
        admission: AdmittedToolCommand,
        meta: &ActionMeta,
        operation_id: semio_framework_job::OperationId,
        cancellation_lease: ToolCancellationLease,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        assert!(self.tools.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
        assert_eq!(self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
        if !matches!(&admission.proof,QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("typed worker handoff requires its exact registered application producer"));}
        let verb = admission.verb.clone();
        let TypedCommandRoots {
            snapshot,
            config,
            history,
            children,
            draft_snapshot,
            interaction_state,
            interaction_hover,
            presence_peers,
            transient,
            peer_presence,
            canonical_base_revision,
            base_revision,
            generation,
            config_generation,
            draft_generation,
            presence_generation,
            transient_generation,
            window_config_authority,
            window_transient_authority,
        } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let parent_document_id = self.stores.store.envelope().id.clone();
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_job_controller_id, base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        assert_eq!(cancellation_lease.key.base_revision, base_revision, "fresh worker receives a revision-rebound cancellation lease");
        assert_eq!(cancellation_lease.key.generation, generation, "fresh worker receives a generation-rebound cancellation lease");
        let payload_schema_id = admission.proof.schema_id();
        let qualified_key = admission.proof.key();
        let publication_lanes = admission.proof.publication_lanes();
        let completion = ArtifactToolCompletion::<A>::new();
        let mut identity_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !cancellation_lease.token.is_cancelled_now() && !cancellation_lease.publication_claim.is_cancelled();
        let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(identity_observer);
        completion.install_authoring_identity(identity, &cancellation_lease)?;
        let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
        let mut retained_wire = Some(admission.raw_wire);
        let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
        let mut request_snapshot_read = if app_owned_retained_route { Some(self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
        let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
        let owned_context = if app_owned_retained_route {
            let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
            let gesture = self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta,artifact_owned_tool_birth_grant(),artifact_owned_tool_handoff_grant())?;
            Some(ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                canonical_base_revision,
                draft_generation,
                transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&children),
                    draft: std::sync::Arc::clone(&draft_snapshot),
                    transient: std::sync::Arc::clone(&transient),
                    window_config: window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())
            .with_gesture(gesture),
            artifact_owned_tool_birth_grant(),
        ).0) } else { None };
        let producer_request = Some(ArtifactOwnedToolJobRequest {
            source_binding: semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
                retained:self.mounted_policy.maintenance,
                command: crate::tool_contract::ArtifactOriginalToolCommand::from_admitted_op_binary(command,retained_wire.as_ref().expect("original typed wire remains retained"),semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx)),
                input: retained_wire.take(),
                checkpoint: None,
                operation,
                controller_id: qualified_key.controller_id,
                tool_id: qualified_key.tool_id,
                payload_schema_id,
                contract: admission.proof.contract(),
                decoded_items: admission.decoded_items,
                app_instance_id: meta.instance_id,
                parent_document_id,
                canonical_base_revision,
                authoring_seed: self.authoring_seed(&meta.actor),
                snapshot,
                snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                config,
                window_config: request_window_config.take(),
                history,
                interaction_state,
                interaction_hover: std::sync::Arc::new(interaction_hover),
                context: owned_context.expect("app-owned route retains its original admitted context"),
                instance_operation_owner: self.tools.instance_operation_owner.clone(),
                output_chunks: output_chunks.clone(),
                completion: completion.clone(),
            });
        let session=None;let session_rejected=None;
        self.tools.tool_operations.insert_admitted(
            operation_id.0,
            MountedTypedCommandFullOperation {
                verb: verb.clone(),
                meta: meta.clone(),
                operation,
                source_identity:semio_framework_job::OriginalSourceBinding::borrow(cx),
                canonical_revision: canonical_base_revision,
                artifact_generation: generation.0,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
                publication_lanes,
                session,
                session_rejected,
                producer_source: producer_request.as_ref().map(|request|request.source_binding),
                producer_request,
                producer_request_retirement: None,
                reserved_producer: None,
                completion: Some(completion),
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: Some(output_chunks),
                cancellation_lease: Some(cancellation_lease),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: false,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::clone(&children)),
                captured_child_content_generation: self.child_composition.child_content_generation,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Worker,
            },
        );
        self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
        Ok(started)
    }

    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> {
        let command = Box::new(command);
        async move {
            let log_generation_before = self.history.log_generation;
            let verb = A::command_id(command.as_ref()).await;
            let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
            let admission = self.tools.admit_command_wire(&self.stores, verb, &payload, 1).await?;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await?;
            Ok(self.finish_recorded(log_generation_before, verb, result).await)
        }
    }

    /// 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
    ///
    /// 🔒️ Contract §2.3 clause 1 — `import` is not a string action (see `VIEWER_REJECTED_ACTION_IDS`'s
    /// own doc), so it is guarded here instead of `dispatch_action`.
    pub(super) async fn dispatch_import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if A::ROLE == AppRole::Viewer {
            return Err(viewer_read_only_fault("import"));
        }
        let proof = self.tools.qualified_tool_proof(&self.stores, "import-media")?;
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), &[]).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), "media import resolved outside its exact controller/owner/factory/tool/schema proof"));
        }
        let (authority,source)=self.media.pending_media_import_permit.take().ok_or_else(||plugin_sdk_fault("media source requires its original pre-admitted import permit"))?;
        if (source.0,source.1)!=(cx.operation(),cx.generation()){*self.media.pending_media_import_permit=Some((authority,source));return Err(plugin_sdk_fault("media import requires its unchanged original caller"))}
        let (job, completion) = match self.build_artifact_reserved_media_job(port, media, meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault); } };
        let raw = Vec::new();
        self.dispatch_reserved_original_producer("import-media",&raw,1,job,completion,meta,Some(admission),authority, cx).await
    }

    /// 🧮️ B1: dispatches a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` against
    /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    pub(super) async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.pending_configuration_source.is_some_and(|source|!source.matches(cx)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration source refused different original roots or clock").into())}
        if self.pending.pending_configuration_rejected{self.tools.close_configuration_failure(&mut self.pending,cx,self.mounted_policy.close).map_err(semio_framework_job::OriginalTurnFailure::from)?;return Ok(None)}
        if cx.is_cancelled(){if self.pending.pending_configuration_source.is_some(){self.pending.pending_configuration_rejected=true;self.tools.close_configuration_failure(&mut self.pending,cx,self.mounted_policy.close).map_err(semio_framework_job::OriginalTurnFailure::from)?;return Ok(None)}return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"configuration caller cancelled before any source was admitted").into())}
        if self.pending.pending_configuration_worker.is_some(){
            let permit=self.pending.pending_configuration_permit.as_ref().expect("configuration source retains its original permit");
            let done=self.pending.pending_configuration_worker.as_mut().unwrap().advance(cx,permit.lease.cancel_token())?;
            if !done{return Ok(None)}
            let mut worker=self.pending.pending_configuration_worker.take().unwrap();let operation=worker.semantic_operation();let failure=worker.take_failure();assert!(worker.terminal_is_empty());drop(worker);
            if let Some(failure)=failure{self.pending.pending_configuration_rejected=true;*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(failure));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}
            let live=self.stores.store.content_revision();let revision=semio_framework_job::RevisionId(u64::from_be_bytes(live[..8].try_into().expect("revision lane width")));
            if !matches!(semio_framework_job::validate_commit(&operation,revision,semio_framework_job::Generation(self.stores.store.generation())),semio_framework_job::CommitValidation::Accepted){self.pending.pending_configuration_rejected=true;*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration worker completed against a stale revision or generation").into()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}
            let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration binary job completed without a decoded command"))?;
            match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{self.pending.pending_configuration_rejected=true;self.pending.pending_configuration_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
            return self.finish_configuration_command(meta,identity,cx).await
        }
        if self.pending.receive_configuration_failure(cx)?{return Ok(None)}
        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_command.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
        let proof=self.tools.qualified_tool_proof(&self.stores,"configuration-binary")?;let key=proof.key();
        let admission=match self.tools.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_))||proof.contract().max_decoded_items<1||!proof.admits::<A>(&admission){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration producer failed its exact owner/factory/schema/contract admission").into())}
        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_source=Some(semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
        type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
        let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
        let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
        self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));
        self.pending.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:Some(self.pending.pending_configuration_permit.as_ref().unwrap().operation),decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
        let operation=self.pending.pending_configuration_permit.as_ref().unwrap().operation;
        self.pending.pending_configuration_worker=Some(crate::reserved_jobs::configuration_worker::ConfigurationWorker::new(job,operation,proof.contract().max_output_bytes,proof.contract().max_work_units_per_step,self.mounted_policy.preparation,self.mounted_policy.close,cx));
        Ok(None)
    }

    async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration commit requires its unchanged original caller").into())}
        if let Some(owner)=self.pending.pending_configuration_completion.as_mut(){
            let demand=if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.demand(cx.retained_grant().maximum_copy_bytes)?};
            let terminal=owner.terminal_is_empty();let mut removed=false;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{if terminal{removed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{owner.step(grant)}})?;
            if removed{self.pending.pending_configuration_completion.take();}return Ok(None)
        }
        let permit=self.pending.pending_configuration_permit.as_ref().expect("configuration commit retains its admitted permit");self.stores.validate_framework_reserved_commit("configuration-binary", permit).await?;
        if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let command=self.pending.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.pending.pending_configuration_source=None;self.pending.pending_configuration_rejected=false;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
    }

    /// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a
    /// command-log row for EVERY dispatch, view actions included (`ActionKind::View` is also the
    /// fallback for an unresolved verb), so the log generation advancing is not by itself evidence that
    /// anything a user would call history happened. A `View` verb paints, it does not command; an
    /// `Interaction` verb moves selection/hover, which `dispatch_interaction_action` has always
    /// declared as kept out of the panel. Handing either one a history patch made every background
    /// tick a history update — measured 2026-09-09 20:55 as 90 host `readHistory` round trips beside
    /// 252 `fillBuildTick`s in 35 s.
    /// 🚫️async: E1 pure classification consumed by sync-only `matches!` call sites — see R9
    fn verb_dirties_history_panel(kind: Option<ActionKind>) -> bool {
        !matches!(kind, Some(ActionKind::View | ActionKind::Interaction))
    }

    /// 🧾️ Attaches the command-log delta directly to the accepted invocation response. History is
    /// no longer a plugin-rendered panel body, so this deliberately leaves `ui_scope` untouched.
    /// `verb` is the DECLARED action/command id — every caller must pass the real one, never a generic
    /// placeholder, or the kind lookup below silently resolves to nothing and the skip is unreachable.
    pub(super) async fn finish_recorded(&mut self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
        if self.history.log_generation == log_generation_before {
            return result;
        }
        let kind = self.tools.registry.get(verb).map(|definition| definition.kind).or_else(|| self.tools.registry.get_command(verb).map(|command| command.kind));
        if !Self::verb_dirties_history_panel(kind) {
            return result;
        }
        result.history_patch = self.history_patch(false).await.ok();
        result
    }

    pub(super) fn retained_fields_terminal_is_empty(&self) -> bool {
        self.retained_fields_own_terminal_is_empty() && self.child_composition.children.is_empty() && self.interaction.local_interaction_query.is_none() && self.ingest.snapshot_read_returns_terminal_is_empty(&self.stores, &self.interaction)
    }
}

```


## 56 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪆️child/🦀️.rs

keys dict_keys(['path', 'conflicts', 'merged'])

### Conflict 0

start
```
1
```

end
```
24
```

base
```
use crate::child_content::mounted_child_group_receipt::mounted_nested_retirement_demand;
use crate::child_content::mounted_child_group_receipt::MOUNTED_RECEIPT_MAXIMUM_BYTES;
use crate::child_content::mounted_child_group_receipt::MountedReceiptBytes;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntrySource;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntry;
use crate::child_content::mounted_child_group_receipt_group::command::CommandAppend;
pub(crate) use crate::child_content::mounted_child_group_receipt_group::command::PagedCommandLog;
#[cfg(test)] use crate::child_view::PreparedChildContentEntry;
#[cfg(test)] use crate::child_view::child_content_arc_bytes;
#[cfg(test)] use crate::child_view::ChildContentRoot;
use crate::child_view::PreparedChildContentIdentity;
use crate::child_view::CHILD_CONTENT_SLOTS;
use crate::child_view::MemberKey;
use crate::child_view::MemberKeyRef;
use crate::child_view::ChildContentEntry;
use crate::child_view::ChildContentPage;
use crate::child_view::ChildContentView;
use crate::child_view::ChildContentTake;
use crate::child_view::ChildContentRetainers;
use crate::child_view::ChildContentOwners;
use super::child_emit_preparation::OwnedChildEmit;
use semio_framework_value::DslValue;
use semio_framework::{kernel::{ActorId, ArtifactDiff, ArtifactHandle, ArtifactVersion, EditRef, HybridLogicalTimestamp, InverseMutation, InvocationId, KernelMutation, MutationId, SchemaId, UndoGroup, UndoPolicy}, ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};

```

head
```
extern crate semio_framework_os_kernel as store;
use std::mem::ManuallyDrop;
use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_SLOTS;
use semio_framework_plugin_decl_views::child_view::MemberKey;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_plugin_decl_views::child_view::ChildContentEntry;
use semio_framework_plugin_decl_views::child_view::ChildContentPage;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_views::child_view::ChildContentTake;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_views::child_view::ChildContentOwners;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};

```

stash
```
use semio_framework_value::ErasedSnapshotRetirement as _;
use crate::child_content::mounted_child_group_receipt::mounted_nested_retirement_demand;
use crate::child_content::mounted_child_group_receipt::MOUNTED_RECEIPT_MAXIMUM_BYTES;
use crate::child_content::mounted_child_group_receipt::MountedReceiptBytes;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntrySource;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntry;
use crate::child_content::mounted_child_group_receipt_group::command::CommandAppend;
pub(crate) use crate::child_content::mounted_child_group_receipt_group::command::PagedCommandLog;
#[cfg(test)] use crate::child_view::PreparedChildContentEntry;
#[cfg(test)] use crate::child_view::child_content_arc_bytes;
#[cfg(test)] use crate::child_view::ChildContentRoot;
use crate::child_view::PreparedChildContentIdentity;
use crate::child_view::CHILD_CONTENT_SLOTS;
use crate::child_view::MemberKey;
use crate::child_view::MemberKeyRef;
use crate::child_view::ChildContentEntry;
use crate::child_view::ChildContentPage;
use crate::child_view::ChildContentView;
use crate::child_view::ChildContentTake;
use crate::child_view::ChildContentRetainers;
use crate::child_view::ChildContentOwners;
use super::child_emit_preparation::OwnedChildEmit;
use semio_framework_value::DslValue;
use semio_framework::{kernel::{ActorId, ArtifactDiff, ArtifactHandle, ArtifactVersion, EditRef, HybridLogicalTimestamp, InverseMutation, InvocationId, KernelMutation, MutationId, SchemaId, UndoGroup, UndoPolicy}, ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};

```

### Conflict 1

start
```
808
```

end
```
820
```

base
```
#[cfg(test)]
include!("../🧪️tests/🔬️app-child-member-registry/🦀️.rs");

include!("../🧩️composition/🪪️metadata/🦀️.rs");

include!("../🧩️composition/📬️publication/🤝️group/🦀️.rs");

mod mounted_child_group_identity { include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs"); }

pub(crate) use mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};

pub(crate) mod mounted_child_group_receipt { use super::*; include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs"); }

```

head
```

```

stash
```
#[cfg(test)]
include!("../🧪️tests/🔬️app-child-member-registry/🦀️.rs");

include!("../🧩️composition/🪪️metadata/🦀️.rs");

include!("../🧩️composition/📬️publication/🤝️group/🦀️.rs");

pub(crate) mod mounted_child_group_identity { include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs"); }

pub(crate) use mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};

pub(crate) mod mounted_child_group_receipt { use super::*; include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs"); }

```
