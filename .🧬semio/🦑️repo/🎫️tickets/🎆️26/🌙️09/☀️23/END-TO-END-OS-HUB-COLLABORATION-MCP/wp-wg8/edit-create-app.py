import pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
def swap(old, new):
    global text
    assert text.count(old) == 1, old[:100]
    text = text.replace(old, new)
swap('''    struct CreateAppRequestOwner {
        wasm_path: Option<PathBuf>,
        plugin_id: Option<String>,
        app_id: Option<String>,
    }

    impl CreateAppRequestOwner {
        fn new(wasm_path: PathBuf, plugin_id: String, app_id: String) -> Self {
            Self { wasm_path: Some(wasm_path), plugin_id: Some(plugin_id), app_id: Some(app_id) }
        }

        fn into_parts(mut self) -> (PathBuf, String, String) {
            (self.wasm_path.take().expect("create request path is present"), self.plugin_id.take().expect("create request plugin is present"), self.app_id.take().expect("create request app is present"))
        }
''', '''    struct CreateAppRequestOwner {
        wasm_path: Option<PathBuf>,
        plugin_id: Option<String>,
        app_id: Option<String>,
        artifact_schema: Option<String>,
    }

    impl CreateAppRequestOwner {
        fn new(wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Self {
            Self { wasm_path: Some(wasm_path), plugin_id: Some(plugin_id), app_id: Some(app_id), artifact_schema: Some(artifact_schema) }
        }

        fn into_parts(mut self) -> (PathBuf, String, String, String) {
            (
                self.wasm_path.take().expect("create request path is present"),
                self.plugin_id.take().expect("create request plugin is present"),
                self.app_id.take().expect("create request app is present"),
                self.artifact_schema.take().expect("create request schema is present"),
            )
        }
''')
swap('''            for field in [&mut self.plugin_id, &mut self.app_id] {''', '''            for field in [&mut self.plugin_id, &mut self.app_id, &mut self.artifact_schema] {''')
swap('''            self.wasm_path.is_none() && self.plugin_id.is_none() && self.app_id.is_none()
''', '''            self.wasm_path.is_none() && self.plugin_id.is_none() && self.app_id.is_none() && self.artifact_schema.is_none()
''')
swap('''            self.wasm_path.as_ref().map_or(0, |path| path.as_os_str().len()) + self.plugin_id.as_ref().map_or(0, String::len) + self.app_id.as_ref().map_or(0, String::len)
''', '''            self.wasm_path.as_ref().map_or(0, |path| path.as_os_str().len()) + self.plugin_id.as_ref().map_or(0, String::len) + self.app_id.as_ref().map_or(0, String::len) + self.artifact_schema.as_ref().map_or(0, String::len)
''')
swap('''        pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {
            match self.submit(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(wasm_path, plugin_id, app_id) }).await {''', '''        /// 🐣️ Compiles `wasm_path` (once per content hash), activates `app_id` on it and opens its first
        /// instance. `artifact_schema` is the document schema the app edits, empty for an app that edits
        /// none: the mounted component becomes that kind's codec in this process
        /// ([`semio_framework_os_kernel::os_store::register_component_document_codec`]).
        pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, String> {
            match self.submit(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(wasm_path, plugin_id, app_id, artifact_schema) }).await {''')
swap('''                    let (wasm_path, plugin_id, app_id) = owner.into_parts();
                    KernelOutcome::Created(state.create_app(wasm_path, plugin_id, app_id).await)''', '''                    let (wasm_path, plugin_id, app_id, artifact_schema) = owner.into_parts();
                    KernelOutcome::Created(state.create_app(wasm_path, plugin_id, app_id, artifact_schema).await)''')
swap('''        async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {''', '''        async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, String> {''')
swap('''            let compiled = self.guest_runtime.compile(&package_ref, bytes).await.map_err(|error| error.to_string())?;
            let instance_id = self.next_instance_id;''', '''            let compiled = self.guest_runtime.compile(&package_ref, bytes).await.map_err(|error| error.to_string())?;
            if !artifact_schema.is_empty() {
                let codec = semio_framework_plugin_host::OwnedComponentDocumentCodec::try_new(self.guest_runtime.clone(), compiled.clone(), artifact_schema).map_err(|error| error.to_string())?;
                semio_framework_os_kernel::os_store::register_component_document_codec(Arc::new(codec)).map_err(|error| error.to_string())?;
            }
            let instance_id = self.next_instance_id;''')
path.write_text(text)
print("create_app plumbing edited")
