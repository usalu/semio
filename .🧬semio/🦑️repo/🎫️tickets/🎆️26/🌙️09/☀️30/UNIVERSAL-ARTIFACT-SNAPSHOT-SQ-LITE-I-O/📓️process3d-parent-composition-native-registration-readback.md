# Process3D Parent Composition Native Registration Readback

Read-only current source audit; no execution or runtime claim. The earlier finite census's Process3D factory gap is corrected: the production composition installs its concrete provider through the parent-owned generic document declaration, rather than an artifact-local explicit `ArtifactCodec::bare` call.

| Authority | Current source locator | Result |
| --- | --- | --- |
| Composition alias | `🌎️hub/🧩️compositions/🏭️process/📦️packages/🦀️rust/🦀️.rs:28` | `artifacts::process3d` is the actual artifact crate. |
| Normal plugin construction | `🌎️hub/🧩️compositions/🏭️process/🦀️.rs:25–38` | `plugin()` supplies `process3d::declaration()` to the builder, then calls `try_build()`. |
| Concrete declaration | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs:1084–1090` | Adds actual schema/composers and `document_codec::<EditorApp<Process3dPlayApp>>()`. |
| Exact app owner | Artifact's `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1385–1400` | Snapshot is `Process3dSnapshot`; dialect is `PROCESS3D_DIALECT` (`s.process.process3d`, `1`, ANY); document schema is `PROCESS_3D_SCHEMA` (`process.3d`). |
| Callable relational capability | Same subset's `🧬️schema/📸️snapshot/🦀️.rs:613–614` | `ArtifactPack::sqlite_snapshot_codec()` explicitly returns `Some(Self::sqlite_codec())`, with the mounted handwritten capability implementing controlled native hooks. |
| Generic concrete factory | OS Plugin `🦀️.rs:3397–3404,3542–3586` | Declaration retains `DocumentCodecSpec::of<A>`; its factory calls `ArtifactCodec::bare::<A::Snapshot,A::Mutation>(schema)`. This is the actual concrete owner, not a dummy factory. |
| Registry plan | OS Plugin `🦀️.rs:4310,4362,4378` | Collects the codec and `NativeSnapshotRegistration::from_capability`, preflights, then supplies both document and native snapshot rows. |
| Normal builder commit | OS Plugin `🏗️builder/🦀️.rs:736–737,785–786` | Resolves declarations into runtime/registry plan, begins artifact assembly and commits the plan before returning the plugin. |
| Atomic publication | Framework `🚪️io/🦀️.rs:1920–1943` | Validates proposed native rows, commits document codecs, then extends the actual native snapshot registry under assembly registry guards. |

`NativeSnapshotRegistration::from_capability` at Framework IO `🦀️.rs:2183–2185` filters only absent `snapshot_sqlite`; Process3D's explicit `Some` bypasses that absence case. Thus this finite source chain identifies no missing Process3D provider registration. Successful host construction and callable registry dispatch remain runtime questions; standalone `ArtifactCodec::of` assertions alone do not prove this initialization completed.
