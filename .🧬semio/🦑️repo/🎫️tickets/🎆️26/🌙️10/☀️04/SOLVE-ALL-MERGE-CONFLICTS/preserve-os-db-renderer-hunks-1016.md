# OS DB and Renderer Pending Hunk Evidence

## Hunk 1 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts
Base 101–102; stash 102–103; scope ``

Base:
```rust
      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_COMPILER_FONTS: join(repo, COMPILER_FONTS_DIRECTORY) }, repo, controller.signal, row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
```
Stash:
```rust
      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], nativeLaunchEnvironmentV1(this.invocation, { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_COMPILER_FONTS: join(repo, COMPILER_FONTS_DIRECTORY) }, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()), repo, signal, row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
    }
```

## Hunk 2 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts
Base 25–25; stash 28–28; scope ``

Base:
```rust
  const dependencies:any={COMPILER_FONTS_DIRECTORY,scriptCompilerAdmissionControlV1:()=>({signal:originalController.signal}),runtimeCatalogBuildInventoryV1:()=>({moduleDirectories:[{pluginId:corpus.entry.pluginId,directoryName:corpus.entry.directoryName}]}),BundleScript:class{repoRoot=root;invocation={control:{signal:originalController.signal}};},buildComponentClosureV1:async(...args:any[])=>{built=args;},TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE:trusted.TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE,TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE:trusted.TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE,TRUSTED_PLUGIN_MODULE_RECEIPT_FILE:trusted.TRUSTED_PLUGIN_MODULE_RECEIPT_FILE,join,resolve,relative,isAbsolute,pathToFileURL,readFileSync,readGeneratedCatalogProjection,AbortController,process,console,registryPath,ownerPath:"owner",COMPONENT_MODULE_DIRECTORIES:[{pluginId:corpus.entry.pluginId,directoryName:corpus.entry.directoryName}],moduleDirectoryName:()=>corpus.entry.directoryName,pluginModulesRootIn:(repo:string)=>join(repo,"modules"),COMPONENT_PROFILE_POLICIES:{"wasm-release":{directory:"dist/components",maximumBytes:268435456,shipped:true}},SHIPPED_COMPONENT_PROFILE:"wasm-release",nativeRuntimeDirectory:(path:string)=>path==="."?path:join(root,"runtime"),publishNativeRuntime:async(...args:any[])=>{published=args;},existsSync:()=>true,playgroundNativeHostArtifactV1:()=>process.execPath,nativeRendererBinary:()=>{throw Error("Unexpected fallback binary");},runNativeSession:async(...args:any[])=>{launched=args;},createAssetHttpServerV1:()=>{throw Error("No assets requested");},PLAYGROUND_ASSET_PROVIDERS_V1:[]};
```
Stash:
```rust
  const dependencies:any={nativeLaunchEnvironmentV1,readNativeLaunchDeclarationV1,nativeLaunchDeclaration,COMPILER_FONTS_DIRECTORY,scriptCompilerAdmissionControlV1:()=>({signal:originalController.signal}),runtimeCatalogBuildInventoryV1:()=>({moduleDirectories:[{pluginId:corpus.entry.pluginId,directoryName:corpus.entry.directoryName}]}),BundleScript:class{repoRoot=root;get invocation(){return nativeInvocation??{control:{signal:originalController.signal}};}},buildComponentClosureV1:async(...args:any[])=>{built=args;},TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE:trusted.TRUSTED_PLUGIN_MODULE_DESCRIPTOR_JSON_FILE,TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE:trusted.TRUSTED_PLUGIN_MODULE_DESCRIPTOR_PACK_FILE,TRUSTED_PLUGIN_MODULE_RECEIPT_FILE:trusted.TRUSTED_PLUGIN_MODULE_RECEIPT_FILE,join,resolve,relative,isAbsolute,pathToFileURL,readFileSync,readGeneratedCatalogProjection,AbortController,process,console,registryPath,ownerPath:"owner",COMPONENT_MODULE_DIRECTORIES:[{pluginId:corpus.entry.pluginId,directoryName:corpus.entry.directoryName}],moduleDirectoryName:()=>corpus.entry.directoryName,pluginModulesRootIn:(repo:string)=>join(repo,"modules"),COMPONENT_PROFILE_POLICIES:{"wasm-release":{directory:"dist/components",maximumBytes:268435456,shipped:true}},SHIPPED_COMPONENT_PROFILE:"wasm-release",nativeRuntimeDirectory:(path:string)=>path==="."?path:join(root,"runtime"),publishNativeRuntime:async(...args:any[])=>{published=args;},existsSync:()=>true,playgroundNativeHostArtifactV1:()=>process.execPath,nativeRendererBinary:()=>{throw Error("Unexpected fallback binary");},runNativeSession:async(...args:any[])=>{launched=args;},createAssetHttpServerV1:()=>{throw Error("No assets requested");},PLAYGROUND_ASSET_PROVIDERS_V1:[]};
```

## Hunk 3 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs
Base 369–369; stash 367–367; scope `fn close_asset_world_step(&mut self, runtime: &RuntimeMailbox) {`

Base:
```rust
    fn close_asset_world_step(&mut self, runtime: &RuntimeMailbox) {
```
Stash:
```rust
    fn close_asset_world_step(&mut self, runtime: &RuntimeMailbox, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
```

## Hunk 4 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs
Base 399–399; stash 395–403; scope `fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter) -> bool {`

Base:
```rust
    fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter) -> bool {
```
Stash:
```rust
    fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if matches!(self.phase, ComponentSurfaceClosePhase::Asset | ComponentSurfaceClosePhase::World) {
            self.close_asset_world_step(runtime,cx)?;
            return Ok(self.phase == ComponentSurfaceClosePhase::Terminal);
        }
        Ok(self.close_engine_step(runtime,presenter,cx))
    }

    fn close_engine_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter, cx: &mut semio_framework_job::StepContext<'_>) -> bool {
```

## Hunk 5 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs
Base 511–511; stash 509–509; scope `fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter) -> bool {`

Base:
```rust
    fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter) -> bool {
```
Stash:
```rust
    fn close_step(&mut self, runtime: &RuntimeMailbox, presenter: &mut AppPresenter, cx: &mut semio_framework_job::StepContext<'_>) -> bool {
```

## Hunk 6 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 235–237; stash 233–238; scope `pub(crate) fn drive_one(&mut self) {`

Base:
```rust
    pub(crate) fn drive_one(&mut self) {
        if (self.session_closing || matches!(self.session.as_ref(), Some(SurfaceResizeSession::Rejected(_)))) && self.drive_session_close_one() {
            return;
```
Stash:
```rust
    pub(crate) fn drive_one(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.session_closing { self.drive_session_close_one(cx)?; return Ok(()); }
        if let Some(source) = self.source.as_mut() {
            let config = BatchDriveConfig { retained: cx.retained_grant(), site: "os_renderer_surface_resize", stage: InteractiveStage::InteractiveStep, fuel_per_step: SURFACE_RESIZE_STEP_FUEL, step_budget_us: SURFACE_RESIZE_STEP_BUDGET_US };
            if let Some(session) = source.admit_pending(config, cx)? { self.session = Some(session); assert!(source.terminal_is_empty()); self.source = None; }
            return Ok(());
```

## Hunk 7 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 258–259; stash 255–257; scope `pub(crate) fn drive_one(&mut self) {`

Base:
```rust
            let _ = session.pump_one(&crate::renderer_worker_pool(), Lane::Interactive);
            return;
```
Stash:
```rust
            if session.checked_out_job().is_some() { session.resume().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resize lost its empty-outcome worker"))?; }
            else { session.pump_one_for_interactive_turn(cx)?; }
            return Ok(());
```

## Hunk 8 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 336–336; stash 314–314; scope `pub(crate) fn close_step(&mut self) -> bool {`

Base:
```rust
    pub(crate) fn close_step(&mut self) -> bool {
```
Stash:
```rust
    pub(crate) fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
```

## Hunk 9 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 362–376; stash 332–332; scope `pub(crate) fn close_abandoned_step() -> bool {`

Base:
```rust
    pub(crate) fn close_abandoned_step() -> bool {
        let Some((slot, pointer)) = SURFACE_LANE_ABANDONMENT.iter().enumerate().find_map(|(slot, owner)| {
            let pointer = owner.swap(std::ptr::null_mut(), Ordering::AcqRel);
            (!pointer.is_null()).then_some((slot, pointer))
        }) else {
            return true;
        };
        let mut lane = unsafe { Box::from_raw(pointer) };
        if lane.close_step() && lane.terminal_is_empty() {
            drop(lane);
        } else {
            SURFACE_LANE_ABANDONMENT[slot].store(Box::into_raw(lane), Ordering::Release);
        }
        false
    }
```
Stash:
```rust

```

## Hunk 10 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 399–402; stash 355–357; scope `pub(crate) fn drive_one(&mut self) {`

Base:
```rust
    pub(crate) fn drive_one(&mut self) {
        if let Self::Mounted(lane) = self {
            lane.drive_one();
        }
```
Stash:
```rust
    pub(crate) fn drive_one(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if let Self::Mounted(lane) = self { lane.drive_one(cx)?; }
        Ok(())
```

## Hunk 11 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs
Base 432–432; stash 387–387; scope `pub(crate) fn close_step(&mut self) -> bool {`

Base:
```rust
    pub(crate) fn close_step(&mut self) -> bool {
```
Stash:
```rust
    pub(crate) fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
```

## Hunk 12 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs
Base 701–701; stash 705–706; scope `async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, crate::program_bridge::ProgramFault> {`

Base:
```rust
    async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, crate::program_bridge::ProgramFault> {
```
Stash:
```rust
    async fn create_app(&mut self, owner:CreateAppRequestOwner) -> Result<u32, crate::program_bridge::ProgramFault> {
        let (wasm_path,plugin_id,app_id,artifact_schema)=(owner.path(),owner.plugin(),owner.app(),owner.schema());
```

## Hunk 13 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs
Base 1246–1247; stash 1246–1248; scope `pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, crate::program_bridge::ProgramFault> {`

Base:
```rust
    pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, crate::program_bridge::ProgramFault> {
        match self.submit(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(wasm_path, plugin_id, app_id, artifact_schema) }).await {
```
Stash:
```rust
    pub(crate) async fn create_app(&self, wasm_path: &std::path::Path, plugin_id: &str, app_id: &str, artifact_schema: &str) -> Result<u32, crate::program_bridge::ProgramFault> {
        let owner=open_create_app_source(CreateAppBorrowed{path:wasm_path,plugin:plugin_id,app:app_id,schema:artifact_schema}).await.map_err(crate::program_bridge::ProgramFault::from)?;
        match self.submit(KernelRequest::CreateApp { owner }).await {
```

## Hunk 14 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs
Base 424–430; stash 424–424; scope `pub(super) async fn find_wasm_artifact(dir: &std::path::Path) -> Option<PathBuf> {`

Base:
```rust
pub(super) async fn find_wasm_artifact(dir: &std::path::Path) -> Option<PathBuf> {
    let request = semio_framework_os_services::NativeIoRequest::ScanDirectory { path: dir.to_path_buf(), directories_only: false, extension: Some("wasm".into()), first_only: true };
    match crate::run_renderer_io(request).await.ok()? {
        semio_framework_os_services::NativeIoValue::Paths(mut paths) => paths.pop(),
        _ => None,
    }
}
```
Stash:
```rust
pub(super) async fn find_wasm_artifact(dir:&std::path::Path)->Result<Option<semio_framework_os_services::OriginalSuppliedNativePath>,crate::RendererSourceIoError>{crate::find_renderer_wasm_path(dir).await}
```

## Hunk 15 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs
Base 434–487; stash 428–430; scope `pub(super) struct CreateAppRequestOwner {`

Base:
```rust
pub(super) struct CreateAppRequestOwner {
    pub(super) wasm_path: Option<PathBuf>,
    pub(super) plugin_id: Option<String>,
    pub(super) app_id: Option<String>,
    pub(super) artifact_schema: Option<String>,
}

impl CreateAppRequestOwner {
    pub(super) fn new(wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Self {
        Self { wasm_path: Some(wasm_path), plugin_id: Some(plugin_id), app_id: Some(app_id), artifact_schema: Some(artifact_schema) }
    }

    pub(super) fn into_parts(mut self) -> (PathBuf, String, String, String) {
        (
            self.wasm_path.take().expect("create request path is present"),
            self.plugin_id.take().expect("create request plugin is present"),
            self.app_id.take().expect("create request app is present"),
            self.artifact_schema.take().expect("create request schema is present"),
        )
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(length) = self.wasm_path.as_ref().map(|path| path.as_os_str().len()) {
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.wasm_path.take().expect("create request path is present"));
            return (self.terminal_is_empty(), 1, length);
        }
        let mut released = None;
        for field in [&mut self.plugin_id, &mut self.app_id, &mut self.artifact_schema] {
            if let Some(length) = field.as_ref().map(String::len) {
                if length > maximum_bytes {
                    return (false, 0, 0);
                }
                drop(field.take().expect("create request string is present"));
                released = Some(length);
                break;
            }
        }
        match released {
            Some(length) => (self.terminal_is_empty(), 1, length),
            None => (true, 0, 0),
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.wasm_path.is_none() && self.plugin_id.is_none() && self.app_id.is_none() && self.artifact_schema.is_none()
    }

    fn remaining_bytes(&self) -> usize {
        self.wasm_path.as_ref().map_or(0, |path| path.as_os_str().len()) + self.plugin_id.as_ref().map_or(0, String::len) + self.app_id.as_ref().map_or(0, String::len) + self.artifact_schema.as_ref().map_or(0, String::len)
    }
}
```
Stash:
```rust
#[path="🐣️create-app/🎟️source/🦀️.rs"]
mod create_app_source;
pub(crate) use create_app_source::{CreateAppBorrowed,CreateAppRequestOwner,CreateAppSourceFailure,CreateAppSourceFailureKind,open_create_app_source};
```

## Hunk 16 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs
Base 139–139; stash 286–286; scope `pub(crate) fn close_world3d_dynamic_step(&self,presenter:&mut AppPresenter,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
        if !self.close_renderer_asset_step() {
```
Stash:
```rust
        if !self.close_renderer_asset_step(cx)? {
```

## Hunk 17 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs
Base 240–241; stash 372–383; scope `pub(crate) fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
        if let Some((index, mut completion)) = returned {
            let original=completion.apply.close_returned_interaction_step(&mut runtime,cx);completion.restores_interaction=completion.apply.restores_interaction();if !matches!(original,Ok(true)){self.0.completions.lock().expect("runtime completion mailbox lock").restore_at(index,completion)}return original.map(|_|false);
```
Stash:
```rust
        if let Some((index, mut completion, checkout)) = returned {
            if matches!(completion.apply, RuntimeApply::Applied) {
                let original = frame_action_metadata(cx);
                let mut queue = self.0.completions.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if matches!(original, Ok(true)) { queue.finish_checkout(checkout); } else { queue.return_checkout(index, completion, checkout); }
                return original.map(|_| false);
            }
            let original = completion.apply.close_returned_interaction_step(&mut runtime, cx);
            completion.restores_interaction = completion.apply.restores_interaction();
            if matches!(original, Ok(true)) { completion.apply = RuntimeApply::Applied; }
            self.0.completions.lock().unwrap_or_else(std::sync::PoisonError::into_inner).return_checkout(index, completion, checkout);
            return original.map(|_| false);
```

## Hunk 18 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs
Base 244–252; stash 386–386; scope `pub(crate) fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
        if !runtime.interaction_available() && runtime.pending_frame_deferred.is_none() {
            if let Some(mut owner) = self.try_take_abandoned_frame_deferred() {
                owner.cancel_current();
                let Some((interaction, cursor)) = owner.take_pair() else { return Ok(false) };
                runtime.return_interaction(interaction);
                runtime.pending_frame_deferred = Some(cursor);
                return Ok(false);
            }
        }
```
Stash:
```rust
        if self.recover_original_frame_step(&mut runtime, cx)? { return Ok(false); }
```

## Hunk 19 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs
Base 255–257; stash 389–389; scope `pub(crate) fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
            if cursor.close_step() {
                runtime.pending_frame_deferred = None;
            }
```
Stash:
```rust
            if cursor.close_step(cx)? && frame_action_metadata(cx)? { runtime.pending_frame_deferred = None; }
```

## Hunk 20 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs
Base 260–262; stash 392–392; scope `pub(crate) fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
        if !runtime.frame_actions.close_step() {
            return Ok(false);
        }
```
Stash:
```rust
        if !runtime.frame_actions.close_step(cx)? { return Ok(false); }
```

## Hunk 21 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 276–277; stash 276–275; scope `pub(crate) struct RendererAssetDecodeJob {`

Base:
```rust
    pub(crate) handback: Arc<Mutex<Option<RendererAssetProbe>>>,
    pub(crate) cancelled: Arc<AtomicBool>,
```
Stash:
```rust

```

## Hunk 22 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 284–285; stash 282–283; scope `pub(crate) fn new(probe: RendererAssetProbe, cancelled: Arc<AtomicBool>) -> Self {`

Base:
```rust
    pub(crate) fn new(probe: RendererAssetProbe, cancelled: Arc<AtomicBool>) -> Self {
        Self { probe: RefCell::new(Some(probe)), handback: Arc::new(Mutex::new(None)), cancelled, result: None, steps: 0, closing: false }
```
Stash:
```rust
    pub(crate) fn new(probe: RendererAssetProbe) -> Self {
        Self { probe: RefCell::new(Some(probe)), result: None, steps: 0, closing: false }
```

## Hunk 23 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 385–388; stash 356–357; scope `pub(crate) struct RendererAssetDecodeSession {`

Base:
```rust
    pub(crate) session: Option<semio_framework_job_worker::WorkerJobSession<RendererAssetDecodeJob>>,
    pub(crate) rejected: Option<semio_framework_job_worker::WorkerJobSessionAdmissionRejected<RendererAssetDecodeJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    pub(crate) handback: Arc<Mutex<Option<RendererAssetProbe>>>,
```
Stash:
```rust
    pub(crate) session: Option<semio_framework_job_worker::MountedWorkerJobSession<RendererAssetDecodeJob>>,
    pub(crate) source: Option<semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource<RendererAssetDecodeJob>>,
```

## Hunk 24 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 390–390; stash 359–358; scope `pub(crate) struct RendererAssetDecodeSession {`

Base:
```rust
    pub(crate) cancelled: Arc<AtomicBool>,
```
Stash:
```rust

```

## Hunk 25 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 393–395; stash 361–362; scope `pub(crate) struct RendererAssetDecodeSession {`

Base:
```rust
    pub(crate) fault: Option<&'static str>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) waker: Option<std::task::Waker>,
```
Stash:
```rust
    cancelled: AtomicBool,
    closing: bool,
```

## Hunk 26 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 399–405; stash 365–366; scope `pub(crate) fn new(probe: RendererAssetProbe, wake: Option<RuntimeHostWaker>) -> Self {`

Base:
```rust
    pub(crate) fn new(probe: RendererAssetProbe, wake: Option<RuntimeHostWaker>) -> Self {
        use semio_framework_job::{allocate_operation_id,root_cancel_token,Generation,InteractiveStage};
use semio_framework_job_worker::{WorkerJobSession};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
        let surface_token = match probe.owner() {
            RendererAssetFetchOwner::World { surface_token, .. } => Some(*surface_token),
            RendererAssetFetchOwner::Shared(_) => None,
        };
```
Stash:
```rust
    pub(crate) fn new(probe: RendererAssetProbe, cx: &semio_framework_job::StepContext<'_>) -> Self {
        let surface_token = match probe.owner() { RendererAssetFetchOwner::World { surface_token, .. } => Some(*surface_token), RendererAssetFetchOwner::Shared(_) => None };
```

## Hunk 27 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 441–562; stash 423–422; scope `pub(crate) fn new(probe: RendererAssetProbe, wake: Option<RuntimeHostWaker>) -> Self {`

Base:
```rust

    pub(crate) fn cancel(&self) {
        if !self.cancelled.swap(true, Ordering::AcqRel) {
            self.wake();
        }
    }

    pub(crate) fn wake(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    pub(crate) fn pump_one(&mut self) -> bool {
        use semio_framework_job::{StepOutcome};
use semio_framework_job_worker::{WorkerJobPoll};
        if self.boundary.is_none() {
            if let Ok(mut handback) = self.handback.try_lock() {
                self.boundary = handback.take().map(|probe| (probe, Some(self.fault.map_or(RendererAssetDecodeResult::Cancelled, RendererAssetDecodeResult::Fault))));
            }
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let demand = match rejected.retirement_demands(ui_contract::UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes) { Ok(demand) => demand, Err(_) => { self.fault = Some("original rejected job close demand refused"); return false; } };
            if !ui_contract::ui_worker_retirement_permits(demand) { self.fault = Some("original worker owner exceeds fixed retirement policy"); return false; }
            let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
            let step = rejected.close_step(grant).admit(grant, rejected.terminal_is_empty());
            if matches!(step, semio_framework_job::InteractiveJobCloseStep::Refused { .. }) { self.fault = Some("original rejected job close receipt refused"); return false; }
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return true;
        }
        let Some(session) = self.session.as_ref() else { return false };
        let _ = session.take_wake();
        #[cfg(not(target_arch = "wasm32"))]
        if self.waker.as_ref().is_some_and(|waker| session.register_wake(waker).is_err()) {
            return true;
        }
        match session.poll() {
            WorkerJobPoll::Idle => {
                #[cfg(not(target_arch = "wasm32"))]
                match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Maintenance) {
                    Ok(ticket) => {
                        self.ticket = Some(ticket);
                        return false;
                    }
                    Err(semio_framework_job_worker::WorkerJobSubmitFault::SequenceExhausted) => {
                        self.fault = Some("asset decoder session sequence exhausted");
                        let _ = session.begin_close();
                    }
                    Err(_) => {}
                }
                #[cfg(target_arch = "wasm32")]
                match session.try_step_on_worker() {
                    Ok((ticket, _)) => self.ticket = Some(ticket),
                    Err(semio_framework_job_worker::WorkerJobContention::WakeExhausted(_)) => {
                        self.fault = Some("asset decoder session wake sequence exhausted");
                        let _ = session.begin_close();
                    }
                    Err(_) => {}
                }
            }
            WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket else {
                    self.fault = Some("asset decoder outcome lost its exact ticket");
                    let _ = session.begin_close();
                    return true;
                };
                if let Ok(mut owner) = session.take_outcome(ticket) {
                    self.ticket = None;
                    if matches!(owner.outcome(), StepOutcome::Yield) {
                        let _ = owner.take_outcome();
                        if let Err(owner) = owner.resume() {
                            self.fault = Some("asset decoder yield could not resume its exact owner");
                            owner.begin_close();
                        }
                    } else {
                        self.fault = Some("asset decoder exposed a nonterminal payload");
                        owner.begin_close();
                    }
                }
            }
            WorkerJobPoll::Terminal => {
                if let Ok(owner) = session.take_terminal() {
                    let result = match owner.outcome() {
                        StepOutcome::Complete(_) => owner.job().result.unwrap_or(RendererAssetDecodeResult::Fault("asset decoder completed without its typed result")),
                        StepOutcome::Cancelled => RendererAssetDecodeResult::Cancelled,
                        _ => RendererAssetDecodeResult::Fault("asset decoder worker terminated with a fault"),
                    };
                    self.boundary = owner.job().take_probe().map(|probe| (probe, Some(result)));
                    self.ticket = None;
                    owner.begin_close();
                }
            }
            WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    match rejected.kind() {
                        semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended => rejected.resume(),
                        _ => {
                            self.fault = Some("asset decoder worker pool refused the exact submission");
                            self.boundary = rejected.job().take_probe().map(|probe| (probe, Some(RendererAssetDecodeResult::Fault("asset decoder worker pool refused the exact submission"))));
                            rejected.begin_close();
                        }
                    }
                }
            }
            WorkerJobPoll::Closing | WorkerJobPoll::TerminalEmpty => {
                let demand = match session.retirement_demands(ui_contract::UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes) { Ok(demand) => demand, Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => return false, Err(_) => { self.fault = Some("original worker job close demand refused"); return false; } };
                if !ui_contract::ui_worker_retirement_permits(demand) { self.fault = Some("original worker owner exceeds fixed retirement policy"); return false; }
                let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
            let step = session.close_step(grant);
            if !step.progress().fits(grant) || matches!(step, semio_framework_job_worker_authority::WorkerJobCloseStep::Refused(_)) { self.fault = Some("original worker job close receipt refused"); return false; }
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            WorkerJobPoll::Submitted | WorkerJobPoll::CheckedOut => return false,
        }
        true
    }
```
Stash:
```rust

```

## Hunk 28 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 570–573; stash 428–428; scope `pub(crate) fn take_boundary(&mut self) -> Option<(RendererAssetProbe, Option<RendererAssetDecodeResult>)> {`

Base:
```rust

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.boundary.is_none() && self.handback.try_lock().is_ok_and(|owner| owner.is_none())
    }
```
Stash:
```rust
    pub(crate) fn terminal_is_empty(&self) -> bool { self.session.is_none() && self.source.is_none() && self.boundary.is_none() }
```

## Hunk 29 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs
Base 615–978; stash 470–469; scope `pub(crate) fn renderer_worker_pool() -> semio_framework_async::WorkerPool {`

Base:
```rust
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct RendererIoHandle {
    slot: usize,
    pub(crate) generation: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoHandle {
    fn try_take(&mut self) -> Option<Result<semio_framework_os_services::NativeIoValue, String>> {
        let result = renderer_io_with_node(self.slot, self.generation, |node| {
            let result = node.result.take()?;
            node.result_taken = true;
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            Some(result)
        })
        .flatten();
        self.completed |= result.is_some();
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for RendererIoHandle {
    type Output = Result<semio_framework_os_services::NativeIoValue, String>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return std::task::Poll::Ready(result);
        }
        let registered = renderer_io_with_node(self.slot, self.generation, |node| {
            node.waker = Some(cx.waker().clone());
            if let Some(session) = node.session.as_ref() {
                let _ = session.register_wake(cx.waker());
            }
        })
        .is_some();
        if registered {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            std::task::Poll::Pending
        } else if renderer_io_generation_live(self.slot, self.generation) {
            RENDERER_IO_WAKE.store(true, Ordering::Release);
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            self.completed = true;
            std::task::Poll::Ready(Err("native I/O mounted session generation is stale".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for RendererIoHandle {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(slot) = RENDERER_IO_SLOTS.get(self.slot) {
            if slot.generation.load(Ordering::Acquire) == self.generation {
                slot.cancel_requested.store(true, Ordering::Release);
            }
        }
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS + 1;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_VACANT: u8 = 0;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_LIVE: u8 = 1;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const RENDERER_IO_CHECKED_OUT: u8 = 2;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_EXHAUSTED: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoSlot {
    generation: AtomicU64,
    state: std::sync::atomic::AtomicU8,
    node: std::sync::atomic::AtomicPtr<RendererIoNode>,
    cancel_requested: AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoSlot {
    const fn vacant() -> Self {
        Self { generation: AtomicU64::new(0), state: std::sync::atomic::AtomicU8::new(RENDERER_IO_VACANT), node: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()), cancel_requested: AtomicBool::new(false) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoNode {
    session: Option<semio_framework_job_worker::WorkerJobSession<semio_framework_os_services::NativeIoJob>>,
    rejected: Option<semio_framework_job_worker::WorkerJobSessionAdmissionRejected<semio_framework_os_services::NativeIoJob>>,
    ticket: Option<semio_framework_job_worker::WorkerJobTicket>,
    result: Option<Result<semio_framework_os_services::NativeIoValue, String>>,
    result_taken: bool,
    detached: bool,
    cancel: semio_framework_async::CancelToken,
    waker: Option<std::task::Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoNode {
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn close_detached_result(&mut self) -> bool {
        if !self.detached {
            return false;
        }
        let Some(result) = self.result.as_mut() else { return false };
        match result {
            Ok(value) if !value.terminal_is_empty() => {
                let _ = value.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return true;
            }
            Ok(_) => {}
            Err(error) if !error.is_empty() => {
                if error.len() > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return false;
                }
                error.clear();
                return true;
            }
            Err(_) => {}
        }
        self.result = None;
        self.result_taken = true;
        true
    }

    fn pump_one(&mut self) {
        if self.close_detached_result() {
            self.wake();
            return;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            self.wake();
            return;
        }
        let Some(session) = self.session.as_ref() else {
            self.wake();
            return;
        };
        let _ = session.take_wake();
        if self.result.is_some() || self.detached {
            if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                self.wake();
                return;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            self.wake();
            return;
        }
        match session.poll() {
            semio_framework_job_worker::WorkerJobPoll::Idle => match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Io) {
                Ok(ticket) => self.ticket = Some(ticket),
                Err(semio_framework_job_worker::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                Err(error) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.begin_close();
                    } else {
                        let _ = session.begin_close();
                    }
                    self.result = Some(Err(format!("native I/O worker submission failed: {error:?}")));
                }
            },
            semio_framework_job_worker::WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket.take() else {
                    self.result = Some(Err("native I/O outcome lost its exact ticket".into()));
                    let _ = session.begin_close();
                    self.wake();
                    return;
                };
                match session.take_outcome(ticket) {
                    Ok(mut owner) => {
                        if matches!(owner.take_outcome(), semio_framework_job::StepOutcome::Yield) {
                            if owner.resume().is_err() {
                                self.result = Some(Err("native I/O yield could not resume exact owner".into()));
                            }
                        } else {
                            owner.begin_close();
                            self.result = Some(Err("native I/O published a nonterminal retained payload".into()));
                        }
                    }
                    Err(_) => self.result = Some(Err("native I/O outcome handback contention".into())),
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Terminal => match session.take_terminal() {
                Ok(mut owner) => {
                    self.result = owner.job_mut().take_result().or_else(|| Some(Err("native I/O terminal owner had no structured result".into())));
                    owner.begin_close();
                }
                Err(_) => self.result = Some(Err("native I/O terminal handback contention".into())),
            },
            semio_framework_job_worker::WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.resume();
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Closing => {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            semio_framework_job_worker::WorkerJobPoll::Submitted | semio_framework_job_worker::WorkerJobPoll::CheckedOut | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty => {}
        }
        self.wake();
    }

    fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.result.is_none() && (self.detached || self.result_taken)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) static RENDERER_IO_SLOTS: [RendererIoSlot; RENDERER_IO_SESSION_SLOTS] = [const { RendererIoSlot::vacant() }; RENDERER_IO_SESSION_SLOTS];
#[cfg(not(target_arch = "wasm32"))]
pub(crate) static RENDERER_IO_WAKE: AtomicBool = AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
static RENDERER_IO_PUMP_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_io_with_node<R>(index: usize, generation: u64, use_node: impl FnOnce(&mut RendererIoNode) -> R) -> Option<R> {
    let slot = RENDERER_IO_SLOTS.get(index)?;
    if slot.generation.load(Ordering::Acquire) != generation || slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    if slot.generation.load(Ordering::Acquire) != generation {
        slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        return None;
    }
    let pointer = slot.node.load(Ordering::Acquire);
    let result = (!pointer.is_null()).then(|| use_node(unsafe { &mut *pointer }));
    slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
    result
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_io_generation_live(index: usize, generation: u64) -> bool {
    RENDERER_IO_SLOTS.get(index).is_some_and(|slot| slot.generation.load(Ordering::Acquire) == generation && matches!(slot.state.load(Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT) && !slot.node.load(Ordering::Acquire).is_null())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn pump_renderer_io_sessions(maximum_sessions: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let start = RENDERER_IO_PUMP_CURSOR.fetch_add(maximum_sessions, Ordering::AcqRel);
    let mut advanced = 0;
    for offset in 0..RENDERER_IO_SESSION_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let index = start.wrapping_add(offset) % RENDERER_IO_SESSION_SLOTS;
        let slot = &RENDERER_IO_SLOTS[index];
        if slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            continue;
        }
        let pointer = slot.node.load(Ordering::Acquire);
        if pointer.is_null() {
            slot.state.store(RENDERER_IO_VACANT, Ordering::Release);
            continue;
        }
        let node = unsafe { &mut *pointer };
        let cancel_advanced = slot.cancel_requested.swap(false, Ordering::AcqRel);
        if cancel_advanced {
            node.detached = true;
            node.cancel.cancel_now();
            if let Some(session) = node.session.as_ref() {
                let _ = session.begin_close();
            }
            if let Some(rejected) = node.rejected.as_mut() {
                rejected.begin_close();
            }
            node.wake();
        }
        if !cancel_advanced {
            node.pump_one();
        }
        if node.terminal_is_empty() {
            slot.node.store(std::ptr::null_mut(), Ordering::Release);
            slot.cancel_requested.store(false, Ordering::Release);
            drop(unsafe { Box::from_raw(pointer) });
            slot.state.store(if slot.generation.load(Ordering::Acquire) == u64::MAX { RENDERER_IO_EXHAUSTED } else { RENDERER_IO_VACANT }, Ordering::Release);
        } else {
            slot.state.store(RENDERER_IO_LIVE, Ordering::Release);
        }
        advanced += 1;
    }
    if RENDERER_IO_SLOTS.iter().any(|slot| slot.state.load(Ordering::Acquire) == RENDERER_IO_LIVE) {
        RENDERER_IO_WAKE.store(true, Ordering::Release);
    }
    advanced
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn submit_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<RendererIoHandle, String> {
    use semio_framework_job::{allocate_operation_id,root_cancel_token,Generation,InteractiveStage,INTERACTIVE_LANE_FUEL,INTERACTIVE_LANE_WALL_US};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
    let Some((slot_index, generation)) = RENDERER_IO_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        if slot.state.compare_exchange(RENDERER_IO_VACANT, RENDERER_IO_CHECKED_OUT, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let Some(generation) = slot.generation.load(Ordering::Acquire).checked_add(1) else {
            slot.state.store(RENDERER_IO_EXHAUSTED, Ordering::Release);
            return None;
        };
        slot.generation.store(generation, Ordering::Release);
        slot.cancel_requested.store(false, Ordering::Release);
        Some((index, generation))
    }) else {
        return Err("native I/O mounted session registry is exhausted".into());
    };
    let job = semio_framework_os_services::NativeIoJob::new(request);
    let cancel = root_cancel_token();
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(generation),
        cancel: cancel.clone(),
        config: BatchDriveConfig { retained: semio_framework_job::RetainedCloneGrant::default(), site: "os_renderer_native_io", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: semio_framework_job::default_now_us,
    };
    let (session, rejected, result) = match semio_framework_job_worker::WorkerJobSession::try_new(job, params) {
        Ok(session) => (Some(session), None, None),
        Err(mut rejected) => {
            rejected.begin_close();
            (None, Some(rejected), Some(Err("native I/O retained session registry is full".into())))
        }
    };
    let node = Box::new(RendererIoNode { session, rejected, ticket: None, result, result_taken: false, detached: false, cancel, waker: None });
    RENDERER_IO_SLOTS[slot_index].node.store(Box::into_raw(node), Ordering::Release);
    RENDERER_IO_SLOTS[slot_index].state.store(RENDERER_IO_LIVE, Ordering::Release);
    RENDERER_IO_WAKE.store(true, Ordering::Release);
    Ok(RendererIoHandle { slot: slot_index, generation, completed: false })
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn run_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<semio_framework_os_services::NativeIoValue, String> {
    submit_renderer_io(request)?.await
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs"]
mod renderer_io_retained_tests;
```
Stash:
```rust

```

## Hunk 30 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 823–829; stash 927–949; scope `pub(crate) fn close_returned_interaction_step(&mut self,runtime:&mut AppRuntime,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{`

Base:
```rust
    pub(crate) fn close_returned_interaction_step(&mut self,runtime:&mut AppRuntime,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        match self{
            Self::ResumeFrameDeferred{interaction,cursor}=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){owner.begin_close();if !owner.close_step(){return Ok(false)}}cursor.take();Ok(true)},
            Self::DispatchEvents(cursor)=>{if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            Self::ResumeDispatch{interaction,cursor}=>{if let Some(returned)=interaction.as_mut(){if let Some(original)=cursor.as_ref(){original.validate_original(cx)?}if !interpreter::close_clipboard_io_step(&mut returned.input,cx)?{return Ok(false)}}if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            #[cfg(not(target_arch="wasm32"))]Self::RestoreInteraction(interaction)=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned)}Ok(true)},
            _=>Ok(false),
```
Stash:
```rust
    pub(crate) fn close_returned_interaction_step(&mut self, runtime: &mut AppRuntime, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        match self {
            Self::Applied => Ok(true),
            #[cfg(not(target_arch = "wasm32"))]
            Self::FrameExecutionReturned => Ok(true),
            Self::ResumeFrameDeferred { interaction, cursor } => {
                if interaction.is_some() { if !frame_action_metadata(cx)? { return Ok(false); } runtime.return_interaction(interaction.take().unwrap()); return Ok(false); }
                if let Some(owner) = cursor.as_mut() { owner.begin_close(); if !owner.close_step(cx)? { return Ok(false); } if !frame_action_metadata(cx)? { return Ok(false); } cursor.take(); return Ok(false); }
                Ok(true)
            }
            Self::DispatchEvents(cursor) => {
                if let Some(owner) = cursor.as_mut() { if !owner.close_step(cx)? { return Ok(false); } if !frame_action_metadata(cx)? { return Ok(false); } cursor.take(); return Ok(false); }
                Ok(true)
            }
            Self::ResumeDispatch { interaction, cursor } => {
                if let Some(returned) = interaction.as_mut() { if let Some(original) = cursor.as_ref() { original.validate_original(cx)?; } if !interpreter::close_clipboard_io_step(&mut returned.input, cx)? { return Ok(false); } }
                if interaction.is_some() { if !frame_action_metadata(cx)? { return Ok(false); } runtime.return_interaction(interaction.take().unwrap()); return Ok(false); }
                if let Some(owner) = cursor.as_mut() { if !owner.close_step(cx)? { return Ok(false); } if !frame_action_metadata(cx)? { return Ok(false); } cursor.take(); return Ok(false); }
                Ok(true)
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => { if interaction.is_some() { if !frame_action_metadata(cx)? { return Ok(false); } runtime.return_interaction(interaction.take().unwrap()); return Ok(false); } Ok(true) }
            _ => Ok(false),
```

## Hunk 31 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 847–847; stash 967–967; scope `fn restore_interaction_only(&mut self, runtime: &mut AppRuntime) -> bool {`

Base:
```rust
    fn restore_interaction_only(&mut self, runtime: &mut AppRuntime) -> bool {
```
Stash:
```rust
    pub(crate) fn restore_interaction_only(&mut self, runtime: &mut AppRuntime) -> bool {
```

## Hunk 32 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 905–916; stash 1025–1031; scope `pub(crate) fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {`

Base:
```rust
    pub(crate) fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        if cursor.is_none() && !runtime.interaction_available() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                if let Some(mut owner) = mailbox.try_take_abandoned_frame_deferred() {
                    owner.cancel_current();
                    let Some((interaction, returned_cursor)) = owner.take_pair() else { return false };
                    runtime.return_interaction(interaction);
                    *cursor = Some(returned_cursor);
                    return false;
                }
            }
```
Stash:
```rust
    pub(crate) fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if let Some(owner) = cursor.as_mut() { if !owner.actions.close_retained_action_step(cx)? { return Ok(false); } }
        if cursor.as_ref().is_some_and(|owner| owner.closing) {
            if let Some(owner) = cursor.as_mut() { if !owner.close_step(cx)? { return Ok(false); } }
            if !frame_action_metadata(cx)? { return Ok(false); }
            cursor.take();
            return Ok(true);
```

## Hunk 33 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 918–959; stash 1033–1037; scope `pub(crate) fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {`

Base:
```rust
        #[cfg(not(target_arch = "wasm32"))]
        if runtime.pending_frame_maintenance_refusal.is_none() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                runtime.pending_frame_maintenance_refusal = mailbox.try_take_abandoned_frame_maintenance();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal.take() {
            let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            if refusal.reservation_live {
                if !mailbox.cancel_interaction_future_reservation() {
                    runtime.pending_frame_maintenance_refusal = Some(refusal);
                    return false;
                }
                refusal.reservation_live = false;
            }
            let Some(mut owner) = refusal.try_take() else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            let Some((interaction, mut returned_cursor)) = owner.take_pair() else {
                if let Err(owner) = refusal.owner.try_restore(owner) {
                    refusal.owner = Arc::new(FrameExecutionOwnerCell::new(owner));
                }
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            returned_cursor.return_shell_maintenance();
            runtime.return_interaction(interaction);
            *cursor = Some(returned_cursor);
            return false;
        }
        if cursor.as_ref().is_some_and(|cursor| cursor.closing) {
            let complete = cursor.as_mut().is_some_and(FrameDeferredCursor::close_step);
            if complete {
                cursor.take();
            }
            return complete;
        }
```
Stash:
```rust
        if cx.should_yield() { return Ok(false); }
        Ok(Self::dispatch_frame_deferred(cursor, runtime, handle))
    }

    fn dispatch_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
```

## Hunk 34 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 1066–1066; stash 1144–1144; scope `fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
    fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
```
Stash:
```rust
    pub(crate) fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
```

## Hunk 35 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 1075–1076; stash 1163–1166; scope `fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
```
Stash:
```rust
                if interaction.is_some() {
                    if !frame_action_metadata(cx)? { return Ok(false); }
                    runtime.return_interaction(interaction.take().unwrap());
                    return Ok(false);
```

## Hunk 36 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 1082–1083; stash 1172–1175; scope `fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
```
Stash:
```rust
                if interaction.is_some() {
                    if !frame_action_metadata(cx)? { return Ok(false); }
                    runtime.return_interaction(interaction.take().unwrap());
                    return Ok(false);
```

## Hunk 37 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs
Base 1086–1085; stash 1179–1190; scope `fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {`

Base:
```rust

```
Stash:
```rust
            _ => {}
        }

        Ok((||{match self {
            Self::Applied => return true,
            #[cfg(not(target_arch = "wasm32"))]
            Self::FrameExecutionReturned => return true,
            Self::DispatchEvents(_)|Self::ResumeDispatch{..}=>unreachable!(),
            Self::Resize { width, height, dpr } => runtime.resize(*width, *height, *dpr),
            Self::ResumeFrameDeferred { .. } => unreachable!(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(_) => unreachable!(),
```

## Hunk 38 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
Base 170–169; stash 170–172; scope `mod native_caller;`

Base:
```rust

```
Stash:
```rust
#[cfg(not(target_arch = "wasm32"))]
#[path = "../⌨️native-entrypoint/🎟️launch/🦀️.rs"]
mod native_launch;
```

## Hunk 39 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
Base 237–236; stash 240–244; scope `mod asset_decode_jobs;`

Base:
```rust

```
Stash:
```rust
#[cfg(not(target_arch = "wasm32"))]
#[path = "🚪️native-io/🦀️.rs"]
mod native_io;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native_io::*;
```

## Hunk 40 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs
Base 183–191; stash 156–157; scope `fn poll_native_plugin_hot_swap(&mut self) {`

Base:
```rust
        let cursor = self.native_hot_swap_cursor % self.shell.plugins.len().max(1);
        self.native_hot_swap_cursor = self.native_hot_swap_cursor.wrapping_add(1);
        let Some(program) = self.shell.plugins.get(cursor) else { return };
        let Some(path) = program.wasm_artifact_path().map(std::path::Path::to_path_buf) else { return };
        let mut paths = semio_framework_os_services::NativePathSet::new();
        if paths.try_push(path).is_err() {
            return;
        }
        self.native_hot_swap_scan = submit_renderer_io(semio_framework_os_services::NativeIoRequest::Modified(paths)).ok();
```
Stash:
```rust
        let index=self.native_hot_swap_program.unwrap();let interaction=&mut self.interaction;let answer=self.native_hot_swap_scan.as_mut().unwrap().try_modified(|observation|observation.is_some_and(|(path,time)|interaction.as_mut().and_then(|state|state.shell.plugins.get_mut(index)).is_some_and(|program|program.observe_native_modification(path,time))));
        match answer{None=>{},Some(Ok(changed))=>{self.native_reload_pending|=changed;self.native_hot_swap_scan.take();self.native_hot_swap_program=None},Some(Err(kind))=>{let error=self.native_hot_swap_scan.take().unwrap().failure(kind);self.native_hot_swap_program=None;log_debug(&format!("plugin hot-swap scan failed: {error}"));}}
```

## Hunk 41 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs
Base 1019–1019; stash 1019–1019; scope `pub(crate) fn step(&mut self, runtime: &RuntimeMailbox, handle: &AppHandle, context: &mut semio_framework_job::StepContext<'_>) -> AppFrameTransactionStep {`

Base:
```rust
                    return AppFrameTransactionStep::Pending;
```
Stash:
```rust
                    match original { Ok(true) => {}, Ok(false) => return AppFrameTransactionStep::Pending, Err(failure) => return AppFrameTransactionStep::CloseFailure(failure) }
```

## Hunk 42 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs
Base 680–892; stash 616–615; scope `pub(crate) fn new(retained: RetainedCloneGrant) -> Self {`

Base:
```rust
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_completion_waker(&mut self, waker: Arc<dyn Fn() + Send + Sync>) {
        if !self.closing {
            self.completion_waker = Some(waker);
        }
    }

    fn admit_active(&mut self, active: ActiveFrameBuild) {
        let params = batch_params(active.operation, active.generation, active.cancel.clone(), active.retained);
        match semio_framework_job_worker::WorkerJobSession::try_new(active, params) {
            Ok(session) => self.session = Some(session),
            Err(mut rejected) => {
                rejected.begin_close();
                self.rejected = Some(rejected);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> Option<crate::AppFramePresentation> {
        if self.closing {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(self.retained);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return None;
        }
        if let Some(session) = self.session.as_mut() {
            if session.generation() != generation {
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                } else {
                    let _ = session.close_step(self.retained);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                return None;
            }
            if let Some(callback) = self.completion_waker.as_ref() {
                let waker = std::task::Waker::from(Arc::new(FrameCompletionWake(Arc::clone(callback))));
                let _ = session.register_wake(&waker);
            }
            match session.poll() {
                semio_framework_job_worker::WorkerJobPoll::Idle => match session.try_submit_step(&crate::renderer_worker_pool(), Lane::Interactive, self.retained) {
                    Ok(ticket) => self.ticket = Some(ticket),
                    Err(semio_framework_job_worker::WorkerJobSubmitFault::Pool(kind)) => {
                        if let Ok(rejected) = session.take_rejected() {
                            if matches!(kind, semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended) {
                                rejected.resume();
                            } else {
                                rejected.begin_close();
                            }
                        }
                    }
                    Err(_) => {
                        let _ = session.begin_close();
                    }
                },
                semio_framework_job_worker::WorkerJobPoll::Outcome => {
                    if let Some(ticket) = self.ticket {
                        if let Ok(mut owner) = session.take_outcome(ticket) {
                            match receive_frame_outcome(&mut owner, self.retained, &mut self.retained_progress) {
                                Ok(Some(JobOutcomeKind::Yield)) => { if owner.resume().is_ok() { self.ticket = None; } }
                                Ok(None) => {},
                                _ => { self.ticket = None; owner.begin_close(); },
                            }
                        }
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Terminal => {
                    if let Ok(mut owner) = session.take_terminal() {
                        if receive_frame_outcome(&mut owner, self.retained, &mut self.retained_progress).ok().flatten() != Some(JobOutcomeKind::Complete) { return None; }
                        let frame_generation = owner.job().generation;
                        let frame = if generation_is_fresh(generation, frame_generation) { owner.job_mut().ok().and_then(|job| job.completed.take()) } else { None };
                        owner.begin_close();
                        return frame;
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Rejected => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Closing => {
                    let _ = session.close_step(self.retained);
                    if session.terminal_is_empty() {
                        self.session = None;
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Submitted | semio_framework_job_worker::WorkerJobPoll::CheckedOut | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty => {}
            }
            return None;
        }
        if self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone(), self.retained));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    /// 🌀️ A superseded build is RETIRED HERE, in this Worker turn — never parked.
    ///
    /// 🩸️ This used to cancel the session and `return None`, which drove the close ladder by one
    /// step per CALL. On the browser the caller is an event-driven tick, so a shell that is
    /// settled between inputs ticks about once a second: one superseded build then held
    /// `self.session` for tens of seconds, no replacement build could be admitted
    /// (`self.session.is_none()` is the admission gate), and therefore NOTHING pumped the runtime
    /// mailbox — measured on 6118 as `frame build cancelled: session generation Generation(3) !=
    /// requested Generation(4)` followed by 74 seconds with no further `frame build admitted`,
    /// no `render begin`, and 75 undelivered `DispatchEvents`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
    /// Falling through spends one bounded Worker turn retiring it, so a later callback can admit
    /// a fresh build without borrowing work from the page or running an uncharged close loop.
    ///
    /// 🌐️ The dedicated `semio-frame-worker` scheduler owns this opportunity. Wasm has no
    /// second thread for the `Send`-gated pool path, so `try_step_on_worker` executes one exact
    /// retained owner turn in this isolate. Checkout and resume are ownership bookkeeping for
    /// that SAME turn; they do not run another job unit. A later scheduler callback owns every
    /// subsequent unit or close step.
    ///
    /// 🔁️ No live session means the previous frame finished (or never started), and the caller
    /// already refuses to produce while a presentation is pending — so the next frame build starts
    /// HERE, every opportunity.
    ///
    /// 🩸️ This used to admit only when `generation` differed from the last admitted one. The browser
    /// frame generation advances on host EVENTS, not on redraws, so an idle shell admitted exactly
    /// one build for its whole life: the first frame completed with its retained window bodies still
    /// mid-ingress, and no second frame was ever built to finish them — a permanently blank canvas
    /// that still asked for frames (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-blank-paint-2026-09-12.md`). One-build-at-a-time is enforced by `self.session`, which
    /// is the authority that gate was standing in for.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> Option<crate::AppFramePresentation> {
        if self.closing || web_sys::window().is_some() {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(self.retained);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return None;
        }
        if let Some(session) = self.session.as_mut() {
            if session.generation() != generation {
                crate::log_debug_once_per_transition("frame-session-generation", true, &format!("[TRACE] frame build superseded: session generation {:?} != requested {generation:?}", session.generation()));
                self.cancel.cancel_now();
                if !matches!(session.poll(), semio_framework_job_worker::WorkerJobPoll::Closing | semio_framework_job_worker::WorkerJobPoll::TerminalEmpty) {
                    let _ = session.begin_close();
                }
            } else {
                crate::log_debug_once_per_transition("frame-session-generation", false, "[TRACE] frame build session generation matches the requested one again");
            }
            let poll = match session.poll() {
                semio_framework_job_worker::WorkerJobPoll::Idle => match session.try_step_on_worker(self.retained) {
                    Ok((ticket, poll)) => {
                        self.ticket = Some(ticket);
                        poll
                    }
                    Err(_) => return None,
                },
                poll => poll,
            };
            let mut presentation = None;
            let mut retire_session = false;
            match poll {
                semio_framework_job_worker::WorkerJobPoll::Outcome => {
                    if let Some(ticket) = self.ticket {
                        if let Ok(mut owner) = session.take_outcome(ticket) {
                            match receive_frame_outcome(&mut owner, self.retained, &mut self.retained_progress) {
                                Ok(Some(JobOutcomeKind::Yield)) => { if owner.resume().is_ok() { self.ticket = None; } }
                                Ok(None) => {},
                                _ => { self.ticket = None; owner.begin_close(); },
                            }
                        }
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Terminal => {
                    if let Ok(mut owner) = session.take_terminal() {
                        if receive_frame_outcome(&mut owner, self.retained, &mut self.retained_progress).ok().flatten() != Some(JobOutcomeKind::Complete) { return None; }
                        let frame_generation = owner.job().generation;
                        let frame = if generation_is_fresh(generation, frame_generation) { owner.job_mut().ok().and_then(|job| job.completed.take()) } else { None };
                        owner.begin_close();
                        presentation = frame;
                    }
                }
                semio_framework_job_worker::WorkerJobPoll::Closing => {
                    let _ = session.close_step(self.retained);
                    retire_session = session.terminal_is_empty();
                }
                semio_framework_job_worker::WorkerJobPoll::TerminalEmpty => retire_session = true,
                _ => {}
            }
            if retire_session {
                self.session = None;
            }
            return presentation;
        }
        crate::log_debug_diagnostic(&format!("[TRACE] frame build admitted generation={generation:?}"));
        self.cancel = root_cancel_token();
        self.admit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, self.cancel.clone(), self.retained));
        self.last_submitted_generation = Some(generation);
        None
```
Stash:
```rust

```

## Hunk 43 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs
Base 230–230; stash 238–238; scope `fn close_active_frame(mut active: ActiveFrameBuild) -> crate::RuntimeMailbox {`

Base:
```rust
fn close_active_frame(mut active: ActiveFrameBuild) -> crate::RuntimeMailbox {
```
Stash:
```rust
fn close_active_frame(mut active: ActiveFrameBuild,original:&mut crate::fixture_original_turn::FixtureTurnV1) -> crate::RuntimeMailbox {
```

## Hunk 44 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs
Base 2282–2282; stash 2303–2303; scope `fn a_fetched_glb_becomes_the_resident_world_mesh_its_url_names() {`

Base:
```rust
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state,terrain_grant,&mut context);
```
Stash:
```rust
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state,terrain_grant,context);
```

## Hunk 45 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs
Base 2355–2355; stash 2368–2368; scope `fn a_real_catalogued_glb_streams_through_the_surfaces_own_asset_lane_into_its_mesh_table() {`

Base:
```rust
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state,terrain_grant,&mut context);
```
Stash:
```rust
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state,terrain_grant,context);
```

## Hunk 46 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🗨️chrome-overlays-and-tour/🦀️.rs
Base 433–433; stash 435–439; scope `pub(crate) fn refused_guest_call(&mut self, refusal: crate::program_bridge::ProgramFault, app: &AppDefinition) -> String {`

Base:
```rust
    pub(crate) fn refused_guest_call(&mut self, refusal: crate::program_bridge::ProgramFault, app: &AppDefinition) -> String {
```
Stash:
```rust
    pub(crate) fn refused_guest_call(&mut self, mut refusal: crate::program_bridge::ProgramFault, app: &AppDefinition) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        let detail=if refusal.frame.is_some()||refusal.create_app_source.is_some(){program_fault_text(&refusal,self.active_terminology(),self.active_locale())}else{std::mem::take(&mut refusal.text)};
        #[cfg(target_arch = "wasm32")]
        let detail=std::mem::take(&mut refusal.text);
```

## Hunk 47 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🗨️chrome-overlays-and-tour/🦀️.rs
Base 448–447; stash 451–453; scope `pub(crate) fn note_refused_open(&mut self, refusal: crate::program_bridge::ProgramFault) -> String {`

Base:
```rust

```
Stash:
```rust
        let detail = program_fault_text(&refusal, self.active_terminology(), self.active_locale());
        #[cfg(not(target_arch = "wasm32"))]
        let source_refusal=refusal.create_app_source.is_some();
```

## Hunk 48 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🗨️chrome-overlays-and-tour/🦀️.rs
Base 453–453; stash 458–458; scope `pub(crate) fn note_refused_open(&mut self, refusal: crate::program_bridge::ProgramFault) -> String {`

Base:
```rust
        if refusal.frame.is_some() { self.note_dispatch_fault(&detail); return detail; }
```
Stash:
```rust
        if refusal.frame.is_some() || source_refusal { self.note_dispatch_fault(&detail); return detail; }
```

## Hunk 49 — 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs
Base 95–95; stash 95–95; scope `fn a_refused_guest_channel_is_told_as_its_localized_notice() {`

Base:
```rust
            let detail = shell.note_refused_open(crate::program_bridge::ProgramFault { fault: Some(fault.clone()), frame: None,kernel:None,actor_call:None, text: format!("create_app promise failed: {}", fault.describe()) });
```
Stash:
```rust
            let detail = shell.note_refused_open(crate::program_bridge::ProgramFault { fault: Some(fault.clone()), frame: None,kernel:None,actor_call:None,create_app_source:None, text: format!("create_app promise failed: {}", fault.describe()) });
```

## Hunk 50 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs
Base 1214–1214; stash 1214–1214; scope `fn touched_command(command_id: &str, actor: &str, kind: &str, hlc_actor: u64, paths: &str) -> db::conflict::CommandTouch {`

Base:
```rust
    let touch = db::conflict::CommandTouch::new(protocol::MutationId(command_id.to_string()), protocol::ActorId(actor.to_string()), db::conflict::CommandKind::from(kind), timestamp);
```
Stash:
```rust
    let touch = db::conflict::CommandTouch::new(protocol::MutationId(command_id.to_string()), protocol::ActorId(actor.into()), db::conflict::CommandKind::from(kind), timestamp);
```

## Hunk 51 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs
Base 494–494; stash 525–525; scope `async fn handle_hello_bootstraps_a_fresh_replica_via_tail_and_issues_a_resume_token() {`

Base:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
```
Stash:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string().into()), 64 * 1024)).unwrap();
```

## Hunk 52 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs
Base 517–517; stash 548–548; scope `async fn handle_hello_reports_no_follow_up_for_an_already_caught_up_replica() {`

Base:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(hello_frontier), "session-2".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
```
Stash:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(hello_frontier), "session-2".to_string(), protocol::ActorId("semio_hub".to_string().into()), 64 * 1024)).unwrap();
```

## Hunk 53 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs
Base 538–538; stash 569–569; scope `async fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {`

Base:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(stale_hello_frontier), "session-3".to_string(), protocol::ActorId("semio_hub".to_string()), 4)).unwrap();
```
Stash:
```rust
    let response = db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, Some(stale_hello_frontier), "session-3".to_string(), protocol::ActorId("semio_hub".to_string().into()), 4)).unwrap();
```

## Hunk 54 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs
Base 555–555; stash 586–586; scope `async fn handle_hello_rejects_zero_snapshot_chunk_bytes() {`

Base:
```rust
    assert!(matches!(db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "s".to_string(), protocol::ActorId("semio_hub".to_string()), 0)), Err(DbError::InvalidArgument(_))));
```
Stash:
```rust
    assert!(matches!(db_actor::block_on(handle_hello(db_storage::db_io_test_pool(), storage, document, None, "s".to_string(), protocol::ActorId("semio_hub".to_string().into()), 0)), Err(DbError::InvalidArgument(_))));
```

## Hunk 55 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3832–3831; stash 3834–3835; scope `fn from_db_error(error: &DbError) -> Self {`

Base:
```rust

```
Stash:
```rust
            DbError::Pack(error) => Self::Pack(error.cause_kind()),
            DbError::Protocol(_) => Self::Protocol,
```

## Hunk 56 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3856–3855; stash 3901–3901; scope `pub struct DbIoFault {`

Base:
```rust

```
Stash:
```rust
    structured: Option<DbError>,
```

## Hunk 57 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3857–3856; stash 3903–3903; scope `pub struct DbIoFault {`

Base:
```rust

```
Stash:
```rust
    custody: Option<DbIoFaultCustody>,
```

## Hunk 58 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3869–3868; stash 3920–3920; scope `pub fn into_db_error(mut self) -> DbError {`

Base:
```rust

```
Stash:
```rust
            DbIoFaultCause::Pack(_) | DbIoFaultCause::Protocol => panic!("typed DB I/O fault lost its original error owner"),
```

## Hunk 59 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3899–3899; stash 3951–3956; scope `pub fn close_step(&mut self) -> bool {`

Base:
```rust
        false
```
Stash:
```rust
        if self.structured.is_some() {
            let progressed = db_io_close_structured_fault(&mut self.structured);
            if self.structured.is_none() && self.result_handback.is_none() { self.custody.take(); }
            return progressed;
        }
        self.result_handback.is_none() && self.custody.take().is_some()
```

## Hunk 60 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 3962–3961; stash 4043–4045; scope `struct DbIoTaskSlot {`

Base:
```rust

```
Stash:
```rust
    fault_custody: Option<DbIoFaultCustody>,
    poll_custody: Option<DbIoFaultCustody>,
    cleanup_custody: Option<DbIoFaultCustody>,
```

## Hunk 61 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4067–4066; stash 4156–4156; scope `fn db_io_note_backend_cleanup(control: DbIoBackendControl, outcome: Option<&DbError>) {`

Base:
```rust

```
Stash:
```rust
    let fault = outcome.map(|(error, custody)| db_io_task_fault(custody, DbIoFaultKind::Backend, error));
```

## Hunk 62 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4525–4525; stash 4623–4623; scope `fn db_io_finish_async_driver(handle: DbIoTaskHandle, backend: DbIoBackendControl, executor: Box<dyn DbIoTaskExecutor>, task: DbIoTask, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {`

Base:
```rust
        Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
```
Stash:
```rust
        Err(error) => DbIoTerminal::Fault(db_io_task_fault(owner.fault_custody.take().expect("preborn task fault custody"), DbIoFaultKind::Backend, error)),
```

## Hunk 63 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4529–4529; stash 4627–4627; scope `fn db_io_finish_async_driver(handle: DbIoTaskHandle, backend: DbIoBackendControl, executor: Box<dyn DbIoTaskExecutor>, task: DbIoTask, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {`

Base:
```rust
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
```
Stash:
```rust
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(owner.fault_custody.take().expect("preborn task fault custody"), DbIoFaultKind::Backend, error)),
```

## Hunk 64 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4603–4602; stash 4701–4701; scope `pub struct DbIoTaskOperation {`

Base:
```rust

```
Stash:
```rust
    custody: Option<DbIoFaultCustody>,
```

## Hunk 65 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4636–4635; stash 4735–4735; scope `enum DbIoLostOwner {`

Base:
```rust

```
Stash:
```rust
static DB_IO_LOST_OWNER_CURSOR: std::sync::Mutex<usize> = std::sync::Mutex::new(0);
```

## Hunk 66 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4726–4726; stash 4831–4838; scope `fn db_io_lost_owner_close_step() -> Result<bool, DbError> {`

Base:
```rust
    Ok(true)
```
Stash:
```rust
    Ok(false)
}

fn db_io_lost_owner_tier_close_step<const N: usize>(tier: &std::sync::Mutex<[Option<DbIoLostOwner>; N]>, start: usize, count: usize) -> Option<(usize, Result<(), DbError>)> {
    let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let index = start + owners[start..start + count].iter().position(Option::is_some)?;
    let outcome = db_io_lost_owner_close_opportunity(owners[index].as_mut().expect("selected original lost owner"));
    Some((index, outcome.map(|terminal| { if terminal { owners[index] = None; } })))
```

## Hunk 67 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 4888–4888; stash 5000–5000; scope `pub fn complete(mut self, terminal: Result<DbIoResult, DbError>) -> Result<(), DbError> {`

Base:
```rust
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(DbIoFaultKind::Backend, &error)),
```
Stash:
```rust
            Err(error) => DbIoTerminal::Fault(db_io_task_fault(owner.fault_custody.take().expect("preborn task fault custody"), DbIoFaultKind::Backend, error)),
```

## Hunk 68 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 5485–5485; stash 5600–5600; scope `pub fn db_io_task_close_step() -> Result<Option<usize>, DbError> {`

Base:
```rust
        match &result {
```
Stash:
```rust
        match result {
```

## Hunk 69 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs
Base 5545–5544; stash 5661–5661; scope `pub fn db_io_task_close_step() -> Result<Option<usize>, DbError> {`

Base:
```rust

```
Stash:
```rust
        let refused_custody = if result.is_ok() { owner.cleanup_custody = Some(cleanup_custody); None } else { Some(cleanup_custody) };
```

## Hunk 70 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs
Base 1533–1533; stash 1518–1521; scope `fn db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate() {`

Base:
```rust
    assert!(db_io_lost_owner_close_step().unwrap());
```
Stash:
```rust
    for _ in 0..2 * (DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS) {
        assert!(db_io_lost_owner_close_step().unwrap());
        if DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().any(Option::is_none) { break; }
    }
```

## Hunk 71 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️unit/🦀️.rs
Base 2–1; stash 2–282; scope ``

Base:
```rust

```
Stash:
```rust

#[semio_framework_async_macros::async_test]
async fn db_io_fault_returns_original_typed_pack_error_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🆔️ids/🧪️tests/🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    let row = &fixture["cases"][0];
    let error = semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, row["message"].as_str().unwrap());
    let pointer = error.message.as_ptr();
    let error = DbError::from(pack::PackRefusal::ValueRefusal(error));
    let fault = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, error);
    assert_eq!(fault.cause, DbIoFaultCause::Pack(pack::PackCauseKind::Refusal(semio_framework_value::ValueRefusalKind::InvalidValue)));
    let DbError::Pack(pack::PackError::Refusal(pack::PackRefusal::ValueRefusal(error))) = fault.into_db_error() else { panic!("original typed Pack error") };
    assert_eq!(error.message.as_ptr(), pointer);
    assert_eq!(serde_json::json!({"message":error.message}), serde_json::json!({"message":row["message"]}));
    eprintln!("[DEBUG] DB I/O terminal returns the actual Pack refusal with its original owned message allocation");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_retirement_releases_one_original_semantic_allocation_at_a_time() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🆔️ids/🧪️tests/🧫️fixtures/⚠️error/🎒️pack/🔣️.json")).unwrap();
    let mut message = String::with_capacity(256);
    message.push_str(fixture["cases"][0]["message"].as_str().unwrap());
    let capacity = message.capacity();
    let error = semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message);
    let mut owner = Some(DbError::from(pack::PackRefusal::ValueRefusal(error)));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(progressed);
    assert!(owner.is_some());
    assert_eq!((born, released), (0, capacity));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(progressed);
    assert!(owner.is_none());
    assert_eq!((born, released), (0, 0));
    let mut message = String::with_capacity(DB_IO_STRUCTURED_FAULT_CLOSE_GRANT.maximum_release_bytes + fixture["retirement"]["deniedExtraBytes"].as_u64().unwrap() as usize);
    message.push_str(fixture["cases"][0]["message"].as_str().unwrap());
    let pointer = message.as_ptr();
    let capacity = message.capacity();
    let mut owner = Some(DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
    let (progressed, born, released) = crate::test_allocation::observe_backing(|| db_io_close_structured_fault(&mut owner));
    assert!(!progressed);
    assert_eq!(born, fixture["retirement"]["deniedNewBytes"].as_u64().unwrap() as usize);
    assert_eq!(released, fixture["retirement"]["deniedReleasedBytes"].as_u64().unwrap() as usize);
    let Some(DbError::Protocol(protocol::ProtocolError::Malformed { what, offset, detail })) = owner else { panic!("original oversized cause") };
    assert_eq!((what, offset, detail.as_ptr(), detail.capacity()), ("fixture", 42, pointer, capacity));
    assert_eq!(serde_json::json!({"message":detail}), serde_json::json!({"message":fixture["cases"][0]["message"]}));
    eprintln!("[DEBUG] DB structured fault obeys independent finite parent release authority and preserves one-over original causes with heap0");
}

struct FaultCustodyTransportPolicy;
impl pack::PackTransportPolicy for FaultCustodyTransportPolicy {
    fn checkpoint(&self, _: pack::PackTransportProgress) -> Result<(), semio_framework_value::ValueError> { Ok(()) }
    fn progress(&self, _: pack::PackTransportProgress) {}
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_maintenance_serves_every_tier_while_retaining_unsupported_original() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(4096, 0), FaultCustodyTransportPolicy).ok().unwrap();
    let original = context.reserve::<std::io::Error>(pack::PackTransportCategory::NativeIo, pack::PackRetryDisposition::Never).unwrap().publish(std::io::Error::from(std::io::ErrorKind::NotFound));
    let pointer = original.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error;
    let mut refused = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbError::from(pack::PackError::TransportFailure(original)));
    while refused.detail.close_step() {}
    let fault = || {
        let mut message = String::with_capacity(fixture["fairness"]["releaseCapacity"].as_u64().unwrap() as usize);
        message.push_str("original fixture cause");
        let mut fault = db_io_task_fault(db_io_fault_custody_reserve().expect("fixture prebirth custody"), DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
        while fault.detail.close_step() {}
        fault
    };
    {
        let mut owners = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(refused));
        owners[1] = Some(DbIoLostOwner::Fault(fault()));
    }
    {
        let mut owners = DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(fault()));
    }
    {
        let mut owners = DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(owners.iter().all(Option::is_none));
        owners[0] = Some(DbIoLostOwner::Fault(fault()));
    }
    *DB_IO_LOST_OWNER_CURSOR.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = 0;
    for key in ["firstCycleReleasedBytes", "secondCycleReleasedBytes"] {
        let mut actual = Vec::new();
        for expected in fixture["fairness"][key].as_array().unwrap() {
            let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
            assert!(progressed.unwrap());
            assert_eq!(born, fixture["fairness"]["newBytes"].as_u64().unwrap() as usize);
            assert_eq!(released, expected.as_u64().unwrap() as usize);
            actual.push(released);
        }
        assert_eq!(serde_json::json!(actual), fixture["fairness"][key]);
    }
    let Some(DbIoLostOwner::Fault(fault)) = DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[0].take() else { panic!("retained original fault") };
    let DbError::Pack(pack::PackError::TransportFailure(original)) = fault.into_db_error() else { panic!("original transport handback") };
    assert_eq!(original.source_error().downcast_ref::<std::io::Error>().unwrap() as *const std::io::Error, pointer);
    assert!(!db_io_lost_owner_close_step().unwrap());
    eprintln!("[DEBUG] DB fault round-robin serves primary, overflow and quarantine with heap0 while preserving unsupported original source identity");
}

#[derive(Debug)]
struct FaultCustodySource { message: String, drops: Arc<std::sync::atomic::AtomicUsize> }
impl std::fmt::Display for FaultCustodySource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) }
}
impl std::error::Error for FaultCustodySource {}
impl Drop for FaultCustodySource {
    fn drop(&mut self) { self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_prebirth_custody_preserves_full_denied_cancelled_and_stale_originals() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let expected = &fixture["reservation"];
    let cancellation = db_io_fault_custody_reserve().unwrap();
    let cancelled_slot = cancellation.slot;
    let cancelled_generation = cancellation.generation;
    let (_, born, released) = crate::test_allocation::observe_backing(|| drop(cancellation));
    assert_eq!((born, released), (0, 0));
    let custody = db_io_fault_custody_reserve().unwrap();
    assert_eq!(custody.slot, cancelled_slot);
    assert!(custody.generation > cancelled_generation);
    let (slot, generation) = (custody.slot, custody.generation);
    let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let context = pack::PackTransportContext::try_new(pack::TransportAdmission::new(4096, 0), FaultCustodyTransportPolicy).ok().unwrap();
    let mut message = String::with_capacity(256);
    message.push_str("original retained source");
    let message_pointer = message.as_ptr();
    let message_capacity = message.capacity();
    let original = context.reserve::<FaultCustodySource>(pack::PackTransportCategory::NativeIo, pack::PackRetryDisposition::Transient).unwrap().publish(FaultCustodySource { message, drops: drops.clone() });
    let pointer = original.source_error().downcast_ref::<FaultCustodySource>().unwrap() as *const FaultCustodySource;
    let fault = db_io_task_fault(custody, DbIoFaultKind::Backend, DbError::from(pack::PackError::TransportFailure(original)));
    let sentinel = || DbIoLostOwner::Fault(db_io_literal_fault(db_io_fault_custody_reserve().unwrap(), DbIoFaultKind::Backend, DbIoFaultCause::Internal, ""));
    for cell in DB_IO_LOST_OWNERS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    for cell in DB_IO_LOST_OWNER_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    for cell in DB_IO_LOST_OWNER_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter_mut() { assert!(cell.is_none()); *cell = Some(sentinel()); }
    let mut held = Vec::new();
    for _ in 0..DB_IO_FAULT_CUSTODY_SLOTS { match db_io_fault_custody_reserve() { Ok(custody) => held.push(custody), Err(DbError::LimitExceeded("db_io fault custody")) => break, Err(error) => panic!("unexpected admission refusal: {error:?}") } }
    let mut producer_calls = 0;
    let (denied, born, released) = crate::test_allocation::observe_backing(|| db_io_fault_custody_reserve().map(|custody| { producer_calls += 1; custody }));
    assert!(matches!(denied, Err(DbError::LimitExceeded("db_io fault custody"))));
    assert_eq!((born, released, producer_calls), (expected["deniedNewBytes"].as_u64().unwrap() as usize, expected["deniedReleasedBytes"].as_u64().unwrap() as usize, expected["producerCallsAtRefusal"].as_u64().unwrap() as usize));
    let (_, born, released) = crate::test_allocation::observe_backing(|| drop(fault));
    assert_eq!((born, released), (0, 0));
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), expected["sourceDropsBeforeHandoff"].as_u64().unwrap() as usize);
    let (stale, born, released) = crate::test_allocation::observe_backing(|| { let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner); db_io_fault_custody_take(&mut arena, slot, generation + 1) });
    assert!(matches!(stale, Err(DbError::StaleGeneration { .. })));
    assert_eq!((born, released), (0, 0));
    assert!(matches!(db_io_fault_custody_release(slot, generation + 1), Err(DbError::StaleGeneration { .. })));
    assert!(matches!(db_io_fault_custody_release(slot, generation), Err(DbError::LimitExceeded("db_io fault custody remains occupied"))));
    let fault = { let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner); let fault = db_io_fault_custody_take(&mut arena, slot, generation).unwrap().unwrap(); assert!(db_io_fault_custody_take(&mut arena, slot, generation).unwrap().is_none()); fault };
    let (error, born, released) = crate::test_allocation::observe_backing(|| fault.into_db_error());
    assert_eq!((born, released), (0, 0));
    let DbError::Pack(pack::PackError::TransportFailure(original)) = error else { panic!("exact original source handoff") };
    let source = original.source_error().downcast_ref::<FaultCustodySource>().unwrap();
    assert_eq!((source as *const FaultCustodySource, source.message.as_ptr(), source.message.capacity()), (pointer, message_pointer, message_capacity));
    assert_eq!(original.retry(), pack::PackRetryDisposition::Transient);
    assert_eq!(serde_json::json!({"message":source.message}), serde_json::json!({"message":"original retained source"}));
    assert!(matches!(db_io_fault_custody_release(slot, generation), Err(DbError::StaleGeneration { .. })));
    let reused = db_io_fault_custody_reserve().unwrap();
    assert_eq!(reused.slot, slot);
    assert!(reused.generation > generation);
    drop(reused);
    drop(held);
    for _ in 0..2 * (DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS) { if !db_io_lost_owner_close_step().unwrap() { break; } }
    assert!(!db_io_lost_owner_close_step().unwrap());
    let semantic_custody = db_io_fault_custody_reserve().unwrap();
    let semantic_slot = semantic_custody.slot;
    let semantic_generation = semantic_custody.generation;
    let mut semantic_message = String::with_capacity(256);
    semantic_message.push_str("original funded semantic cause");
    let semantic = db_io_task_fault(semantic_custody, DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: semantic_message }));
    drop(semantic);
    *DB_IO_LOST_OWNER_CURSOR.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = DB_IO_LOST_OWNER_SLOTS + 2 * DB_IO_LOST_OWNER_OVERFLOW_SLOTS + semantic_slot;
    let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
    assert!(progressed.unwrap());
    assert_eq!((born, released), (0, 256));
    let epoch = DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire);
    let (progressed, born, released) = crate::test_allocation::observe_backing(db_io_lost_owner_close_step);
    assert!(progressed.unwrap());
    assert_eq!((born, released), (0, 0));
    assert_eq!(DB_IO_ADMISSION_EPOCH.load(std::sync::atomic::Ordering::Acquire), epoch + 1);
    {
        let arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(arena.cells[semantic_slot].generation, semantic_generation);
        assert!(!arena.cells[semantic_slot].reserved && arena.cells[semantic_slot].owner.is_none());
    }
    drop(original);
    assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), expected["sourceDropsAfterExplicitDrop"].as_u64().unwrap() as usize);
    eprintln!("[DEBUG] DB full custody admission refuses before producer with heap0; cancellation, stale access and exactly-once original handoff preserve source pointer, capacity and retry");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_refused_task_handback_retains_original_custody_after_cause_close_or_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    for transfer in [false, true] {
        let custody = db_io_fault_custody_reserve().unwrap();
        let (slot, generation) = (custody.slot, custody.generation);
        let credit = DbIoCredit { pages: 0, bytes: size_of::<DbIoFault>() as u64 + 256, items: 1, controls: 1 };
        let operation = db_io_operation_reserve(credit).unwrap();
        db_io_operation_add_result_lease(operation).unwrap();
        let index = db_io_operation_owner_slot(operation).unwrap();
        db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[index].result_leases = 0;
        let mut message = String::with_capacity(256);
        message.push_str("original refused handback cause");
        let pointer = message.as_ptr();
        let capacity = message.capacity();
        let mut fault = db_io_task_fault(custody, DbIoFaultKind::Backend, DbError::Protocol(protocol::ProtocolError::Malformed { what: "fixture", offset: 42, detail: message }));
        fault.result_handback = Some(DbIoResultHandback { handle: DbIoTaskHandle { slot: 0, generation: 0, operation }, retained_credit: credit });
        let original = if transfer {
            let original = fault.into_db_error();
            let DbError::Protocol(protocol::ProtocolError::Malformed { what, offset, detail }) = &original else { panic!("exact transferred cause") };
            assert_eq!((*what, *offset, detail.as_ptr(), detail.capacity()), ("fixture", 42, pointer, capacity));
            Some(original)
        } else {
            assert!(fault.close_step());
            assert!(fault.close_step());
            assert!(!fault.close_step());
            let retained = fault.custody.as_ref().unwrap();
            assert_eq!((retained.slot, retained.generation), (slot, generation));
            assert!(fault.structured.is_none());
            drop(fault);
            None
        };
        let mut fault = {
            let mut arena = DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let owner = arena.cells[slot].owner.as_ref().unwrap();
            assert!(owner.result_handback.is_some());
            assert_eq!(owner.custody.as_ref().unwrap().generation, generation);
            db_io_fault_custody_take(&mut arena, slot, generation).unwrap().unwrap()
        };
        assert_eq!(fixture["reservation"][if transfer { "explicitCauseHandbackRetainsRefusedTaskCustody" } else { "failedTaskHandbackRetainsCustody" }], serde_json::json!(fault.custody.is_some()));
        db_io_operation_ledger().lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[index].result_leases = 1;
        assert!(fault.close_step());
        assert!(fault.close_step());
        assert!(!fault.close_step());
        assert!(fault.custody.is_none());
        assert!(!DB_IO_FAULT_CUSTODY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).cells[slot].reserved);
        drop(original);
    }
    eprintln!("[DEBUG] DB refused aggregate handback retains exact original reservation after semantic closure and explicit cause transfer; repaired handback releases once outside custody guard");
}

#[semio_framework_async_macros::async_test]
async fn db_io_fault_full_custody_refuses_actual_task_admission_before_backend_effects() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️fault-custody/🔣️.json")).unwrap();
    let expected = &fixture["task"];
    let storage = MemoryStorage::new(db_io_test_pool()).await.unwrap();
    let document = DbIoText::try_from_str(expected["document"].as_str().unwrap()).unwrap();
    let payload: Vec<u8> = expected["payload"].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect();
    let input = pages(&payload);
    let identity = input.pages[0].as_ref().map(|page| (page.slot, page.generation, page.operation, page.used));
    let pointer = input.page(0).unwrap().as_ptr();
    let operation = input.operation;
    let task = DbIoTask::SnapshotWrite { backend: storage.control, document, generation: expected["generation"].as_u64().unwrap(), input };
    let mut held = Vec::new();
    for _ in 0..DB_IO_FAULT_CUSTODY_SLOTS { match db_io_fault_custody_reserve() { Ok(custody) => held.push(custody), Err(DbError::LimitExceeded("db_io fault custody")) => break, Err(error) => panic!("unexpected admission refusal: {error:?}") } }
    let before = db_io_census();
    let task_arena = { let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner); (arena.free_read, arena.free_len, arena.next_generation) };
    let (refused, born, released) = crate::test_allocation::observe_backing(|| db_io_allocate_task(task));
    assert_eq!((born, released), (expected["newBytesOnRefusal"].as_u64().unwrap() as usize, expected["releasedBytesOnRefusal"].as_u64().unwrap() as usize));
    let (error, mut task) = refused.err().expect("actual task refused before admission effects");
    assert!(matches!(error, DbError::LimitExceeded("db_io fault custody")));
    let DbIoTask::SnapshotWrite { backend, document, generation, input } = &task else { panic!("exact original task") };
    assert_eq!(*backend, storage.control);
    assert_eq!(*generation, expected["generation"].as_u64().unwrap());
    assert_eq!(document.as_str(), expected["document"].as_str().unwrap());
    assert_eq!((input.operation, input.page(0).unwrap().as_ptr(), input.pages[0].as_ref().map(|page| (page.slot, page.generation, page.operation, page.used))), (operation, pointer, identity));
    assert_eq!(serde_json::json!(input.page(0).unwrap()), expected["payload"]);
    let after = db_io_census();
    assert_eq!((after.tasks, after.steps, after.turns), (before.tasks, before.steps, before.turns));
    assert_eq!({ let arena = db_io_task_arena().lock().unwrap_or_else(std::sync::PoisonError::into_inner); (arena.free_read, arena.free_len, arena.next_generation) }, task_arena);
    drop(held);
    while task.close_step().unwrap().is_some() {}
    assert_eq!(storage.latest_generation(&ArtifactId::from(expected["document"].as_str().unwrap())).await.unwrap(), None);
    storage.close().await.unwrap();
    eprintln!("[DEBUG] DB full custody refuses actual SnapshotWrite admission with heap0, unchanged original page pointer/generation/operation/backend, unchanged task arena and no producer steps");
}
```

## Hunk 72 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 565–565; stash 566–566; scope `enum ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 73 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 567–567; stash 568–568; scope `enum ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 74 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 569–569; stash 570–570; scope `enum ArtifactCommittedDurableGroupRecoveryStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 75 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 594–594; stash 595–595; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 76 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 596–596; stash 597–597; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 77 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 598–598; stash 599–599; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 78 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 606–605; stash 607–607; scope `pub(crate) enum ArtifactCommittedDurableGroupRecoveryAdvanceV1 {`

Base:
```rust

```
Stash:
```rust
    DiagnosticProgress(store::durable_group::DurableOwnedThreeStoreCommitPhaseV1, semio_framework_value::retained_clone::RetainedCloneProgress),
```

## Hunk 79 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 614–614; stash 616–616; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 80 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 616–616; stash 618–618; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 81 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 618–618; stash 620–620; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 82 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 629–629; stash 631–631; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 83 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 631–631; stash 633–633; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 84 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 633–633; stash 635–635; scope `pub(crate) struct ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 85 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 755–755; stash 761–761; scope `impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 86 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 757–757; stash 763–763; scope `impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 87 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 759–759; stash 765–765; scope `impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 88 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 813–813; stash 819–819; scope `enum ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 89 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 815–815; stash 821–821; scope `enum ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 90 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 817–817; stash 823–823; scope `enum ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 91 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 828–828; stash 834–834; scope `struct ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 92 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 830–830; stash 836–836; scope `struct ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 93 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 832–832; stash 838–838; scope `struct ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 94 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 855–855; stash 861–861; scope `struct ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 95 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 857–857; stash 863–863; scope `struct ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 96 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 859–859; stash 865–865; scope `struct ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 97 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 957–957; stash 963–963; scope `pub struct ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 98 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 959–959; stash 965–965; scope `pub struct ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 99 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs
Base 961–961; stash 967–967; scope `pub struct ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>`

Base:
```rust
    ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```
Stash:
```rust
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
```

## Hunk 100 — 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs
Base 929–928; stash 926–928; scope `fn absorb(&mut self, other: Self) {`

Base:
```rust

```
Stash:
```rust
    }

    impl protocol::ColdMutationDiff<Counter> for AddDiff {
```

