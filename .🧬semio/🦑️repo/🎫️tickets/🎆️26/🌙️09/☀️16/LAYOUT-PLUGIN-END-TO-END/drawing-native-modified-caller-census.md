# Modified Request Caller Census

A read-only Rust census over 8991 framework files finds the following canonical call sites. All direct production constructors use NativePathInput; Root main model/close/status patterns preserve Single/Rows. No temporary push shim is introduced. This is a source census, not a compiler or native Host pass.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs
  - 180: NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(owner))) | NativeIoState::Scanning { paths: owner, .. } | NativeIoState::StagingScan{paths:owner,..} | NativeIoState::ClosingScanFault { paths: owner, .. } | NativeIoState::ReadingModified { paths: owner, .. } | NativeIoState::ClosingModifiedFault { paths: owner, .. } if !owner.is_empty() => return Ok((Paths,native_io_child_demands(owner.close_demand())?)),
  - 183: NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) if path.capacity() != 0 => return release(Path,path.capacity(),1),
  - 199: NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) => Some(path),
  - 200: NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(paths))) => paths.as_slice().first(),
  - 284: NativeIoRequest::Modified(input) => {
  - 402: NativeIoState::Pending(NativeIoRequest::Modified(NativePathInput::Rows(paths))) | NativeIoState::Scanning { paths, .. } | NativeIoState::StagingScan{paths,..} | NativeIoState::ClosingScanFault { paths, .. } | NativeIoState::ReadingModified { paths, .. } | NativeIoState::ClosingModifiedFault { paths, .. } => {let turn=paths.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});work=turn.work;return turn.result.and_then(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}.admit(grant,false));},
  - 410: NativeIoState::Pending(NativeIoRequest::ReadBytes(path) | NativeIoRequest::ReadPage { path, .. } | NativeIoRequest::ScanDirectory { path, .. } | NativeIoRequest::Modified(NativePathInput::Single(path))) => drop(std::mem::take(path)),

- 🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️native-io-unit/🦀️.rs
  - 183: let mut job = NativeIoJob::new(NativeIoRequest::Modified(NativePathInput::Rows(paths)));
  - 216: let request=if single{NativeIoRequest::Modified(NativePathInput::Single(path.clone()))}else{
  - 218: source.turn(|cx|{let turn=paths.push_from(&mut input,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();assert!(input.is_none());NativeIoRequest::Modified(NativePathInput::Rows(paths))

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️source/🦀️.rs
  - 42: let request=match mode{RendererSourceIoMode::Contiguous{..}=>NativeIoRequest::ReadPage{path,offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES},RendererSourceIoMode::Directory=>NativeIoRequest::ScanDirectory{path,directories_only:false,extension:Some(std::borrow::Cow::Borrowed("wasm")),first_only:true},RendererSourceIoMode::Modified=>NativeIoRequest::Modified(semio_framework_os_services_native_io::NativePathInput::Single(path)),RendererSourceIoMode::World=>NativeIoRequest::ReadBytes(path),RendererSourceIoMode::Resident=>unreachable!()};
