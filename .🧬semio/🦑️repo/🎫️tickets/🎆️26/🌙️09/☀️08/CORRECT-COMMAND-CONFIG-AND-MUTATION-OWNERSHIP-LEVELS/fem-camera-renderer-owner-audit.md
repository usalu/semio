# FEM Camera Renderer Owner Audit

The FEM camera values are now stored by exact window instance, but their type names remain declared at artifact roots. The 2D record has `x`, `y` and `zoom`; its shape matches the renderer's `CanvasCamera` and the infinite-canvas coordinate helpers. The 3D record instead carries an opaque `json` string with an empty-object sentinel. Both model and results windows interpret that string through the editor helper, and the viewer duplicates the same fallback to the framework default world camera.

The actual 3D renderer emits `setCamera` with a nested camera pose containing `position`, `target`, `zoom` and optional `up`. Its `WorldCameraState` lives under the framework infinite-world React implementation and also carries optional projection-family and projection-taxonomy state. FEM3D's declared command payload only contains a `json` string. This producer/consumer mismatch needs a native dispatch case built from the renderer's neutral pointer-gesture fixture before changing the persisted vocabulary. Merely moving or renaming the opaque FEM record would retain the mismatch.

The UI-scene math `Camera3d` is a different contract: position, target, up, field-of-view and near/far planes. It should not replace the persisted orbit viewport state without addressing the actual consumer and projection semantics. Framework `world3d_camera_json` and `world3d_default_camera` also build strings rather than supplying the shared typed viewport schema. No camera-production changes are made by this audit.

Sources inspected: FEM2D/FEM3D artifact roots; FEM3D editor camera command and helper; FEM3D viewer model window; framework renderer `World3dHost` camera parsing and dispatch helpers plus its pointer-gesture fixture; renderer `Canvas2dHost`; framework infinite-world React camera state; framework plugin world-scene helpers; UI-scene math camera.

## Document Facet Correction

Both document schema roots still exported unused camera declarations despite having no camera document field. A repository source scan found no consumer of their `parseFemCamera` functions or camera type imports. The unused TypeScript artifact/diff declarations, JSON Schema definitions, GraphQL types and Protocol Buffers messages are removed from ten files. The mounted window contracts and the remaining native camera type are preserved pending the reusable viewport-contract work.

The new language-neutral document corpus contains populated 2D and 3D documents and foreign `camera`, `locale` and `resultMode` fields. The first registered oracle run reproduced both artifact parsers silently accepting camera (`fem-document-admission-neutral-red.log`). They now use the existing framework `parseSchemaRecord` to enforce the document's declared keys. Adding these actual parsers to strict TypeScript exposed existing missing element/load/load-case/combination parsers, untyped coordinate tuple construction and unvalidated 3D combination values. Those parsers now implement their concrete domain schema and preserve their distinct 2D/3D combinations.

The populated corpus exercises both element variants, all three load variants, coordinate outlines and each combination representation. Both registered neutral targets and strict TypeScript pass in `fem-document-admission-neutral-green-2.log`. Native artifact/snapshot foreign-field admission laws are authored; FEM3D native6 is running before the native admission correction. The complete nested entity admission contract remains broader than this owner-boundary corpus.

Native6 stopped before FEM compilation because a concurrently relocated UI typed-component retirement file included its catalog one directory too shallow. The catalog remains at the UI contract root; its include is corrected from `../../🧾️typed/🦀️.rs` to `../../../🧾️typed/🦀️.rs`. Native7 is the subsequent admission rerun. This shared path repair adds `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧱️component/🦀️.rs` to the touched-file ledger.


## Codec Dependency Follow-Up

The UI-scene crate currently depends on replication value codecs and UI contract, without an OS-kernel dependency. `DslField` is owned by OS kernel and names its `Shape` and `FieldValue`; the kernel currently has no UI-scene dependency either. A reusable viewport record therefore needs an explicit codec ownership decision, not a copied FEM record renamed as framework state. A neutral DSL schema/field binding owner or a trait-owner binding can be evaluated against this graph; this audit does not assume that relocating the whole DSL is the only valid route. Existing board Camera and native canvas Camera also duplicate the 2D x/y/zoom state and must be accounted for in that decision.

GraphQL and Protocol Buffers compiler validation remains pending. Local resolution found no installed `graphql` or `protobufjs` package and no `protoc` executable. Existing Ajv, fast-json-patch, strict TypeScript and native codec evidence is unaffected; no external compiler success is claimed.
