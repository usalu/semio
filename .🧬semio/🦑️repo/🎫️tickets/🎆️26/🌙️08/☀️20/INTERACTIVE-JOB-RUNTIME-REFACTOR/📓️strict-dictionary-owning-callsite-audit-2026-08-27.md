# Strict Neural Dictionary Adoption: Owning Callsite Audit

## Boundary and Inventory

This is a source audit, not a native or runtime result. The OrderedMap native gate remains pending; Dictionary's backing has not changed. A direct Rust `Dictionary` symbol census across framework and plugins found 71 files: 58 neural-related references and 13 excluded homonyms/UI literals (Forms 2, Norm 5, stdio 6). The neural-related groups are neural engine 1, framework Flow 8, Flow plugin 13, Procedural 9, Imperative 23, and Sequence 4. This inventory covers direct symbol references; types containing Widget/Tree/StepParams transitively need the owning-boundary changes below even when their file never spells Dictionary.

No `Dictionary::Drop` cleanup loop is acceptable. A nonempty retained Dictionary must remain guarded. Explicit cold owners/builders may drain domain retirement at a visibly cold boundary; retained code must transfer ownership into a cursor. A shared alias can only be released through an exact atomic ownership transfer, not a `strong_count` test followed by ordinary Drop.

## Inspected Owning Boundaries

Paths below abbreviate `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`, `F` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow`, and `I` = `✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️component.rs`.

| Owner / exact callsite | Current release or failure path | Required modification |
|---|---|---|
| E:14 Dictionary.pairs; E:28 insert; E:71 merge | BTreeMap insertion/replacement drops old nested Value; merge clones all values. | Keep Dictionary terminal-guarded; explicit cold builder drives domain retirement for displaced Value. After native gate use UpdateCursor, not generic OrderedMap::insert. Pure Dictionary clone shares immutable root. |
| E:45 iter; E:50 pop_first; E:55 next_after | pop_first assumes owned movable key/value; next_after compares a full key in range lookup. | Preserve borrowed rank iterator; remove unused next_after. Replace retained pop_first consumers with domain retirement because shared entries cannot yield owned V unconditionally. Retained lookup delegates to LookupCursor. |
| E:88 Value::Dictionary; E:129 Atom::String | Nested dictionaries and text currently drop recursively. | Domain owner stack transfers Dictionary, Value, and byte buffers; exact final values from map retirement re-enter this stack. No V::drop inside a retained step. |
| E:205 FieldSpec.default; E:798 ChannelSpec.default | Replacing defaults and dropping records may own nested Value. | Explicit record retirement extracts optional default; cold construction/catalogue callers hold named cold owners until transfer. |
| E:235 Schema; E:498 SchemaComponent; E:922 OperatorInfo; E:951 Registry | Registry replacement at E:965/978 and finalize skip branches discard records with default Values; boxed SchemaComponent owns Schema. | Registry/catalogue retirement must traverse records and boxed owned implementation metadata. Cold registry builders retire replaced records explicitly; ordinary trait-object drop cannot secretly destroy a retained schema graph. |
| E:503 construct; E:523 modify; E:532 success_output | `?` and validation errors abandon partial Dictionary; E:565 and_then borrows then drops the successful instance. | Cold builder/owner guards partial output and instance; finish transfers exact Dictionary. No catch-unwind or leaking normal errors. |
| E:575 Tree; E:582 Neuron.params/tree | Vec/Box destruction recurses into params and nested trees. | Shared Flow retirement already has Tree/Neuron arms; all cold tree codecs/tests need explicit owning wrappers or retirement boundaries. |
| E:1156 NeuralCache.entries | seed/get_or_insert replacement and sweep retain discard Dictionary values; lock-error seed also drops input. | Return or queue displaced dictionary owners; cold sweep drains explicitly, retained cache maintenance uses cursor. Lock error must preserve or retire the input. |
| E:1162 NeuralCacheRetirement.close_step | strong_count/try_unwrap race; extract_if(...).next().is_some() destroys an entire Dictionary with no byte grant. | Arc::into_inner exact transfer; retain dictionary domain cursor; accept item/byte grants; strict empty-state guard rather than debug-only assertion. |
| E:1459 EvalChannels; E:1467 BudgetedEval | HashMap outputs/inputs, partial field projection `.outputs`/`.channels`, map replacement and error returns drop other owners. | EvalChannels retirement walks map entries, key bytes, dictionary cursors. Explicit cold result owner for ordinary evaluator APIs; retained evaluator owns result and cleanup frontiers. |
| E:1556–1625 budgeted evaluator | Whole seeds clone, merged Dictionary locals, early return on cache miss or PendingExtension, input/output replacements. | Add explicit owners for input/merged/partial maps and displaced records. Existing node-count budget is not byte-bounded evaluation; do not certify it from Dictionary adoption. |
| E:1653–1704 parallel-level evaluator | level_inputs/outputs, deferred_clusters(Tree,Dictionary), compute_jobs(...,Dictionary), `?` failures, scope exits. | Named cold aggregate owners and exact transfer at merge; each untransferred element retires explicitly. This remains cold/batch evaluation. |
| E:1803 collect_output_boundaries; E:1959 collect_neuron_input | Missing boundary/input error abandons accumulated dictionary; ignored non-dictionary value and merged dictionary temporary may own content. | Cold builder protects accumulation and explicit Value owner protects temporary. Replace eager unwrap_or at E:1837 with lazy construction to avoid a throwaway dictionary even on a successful lookup. |
| F/📄️artifact:134,185,225 | FlowPreviewGui.preview, Widget::Neuron.params, Widget::OutputPreview.preview; Widget::Cluster embeds Tree+FlowGui. | Shared Flow typed retirement delegates Dictionary/Value to neural domain owner; compound widgets move every field into that owner. |
| F/🖥️host:138–146; 246,326,344–346,835,1014–1027 | FlowHost outputs/export_payloads/previous_channels, replacement baselines, parsed patch, cache sweeps. | Exact displaced-output/baseline queues; cold host mutators use explicit cold ownership boundaries. A Dictionary backing swap alone would break these assignments. |
| F/🖥️host:2029 FlowHostRetirement, 2116 onward | widgets.pop().is_some(), outputs/extract_if().is_some(), previous_channels.take().is_some(), kind_infos drop. | Transfer Widget into shared FlowRetirement; map entry into key+Dictionary retirement; channels into EvalChannels retirement; metadata default Values into typed metadata retirement. Count actual bytes. |
| F/🖥️host:2222 FlowEvalSession, 2282/2333, 2397 close_step | Baseline replacement/reset and whole previous_channels/cache take; strings wait for full capacity grant. | Retained displaced-baseline/cache frontiers and bytewise strings. Current >4096-capacity text can strand session close. Fix with actual-grant tests; do not simply document or cap it. |
| F/🌉️bridge:71/93 outputs/inputs JSON; 115 preview_dict_from_connection; 158 widget_to_neuron | Cold decoded maps, dictionary temporaries and recursive Widget↔Tree conversion. | Explicit cold map/Value builders; direct return moves owner. Shared aliases remain exact; no borrowing dictionary then silently dropping final source root. |
| F/🌿️vcs:347 value_dsl_map_to_dictionary; 357 option_dsl_map_to_dictionary | Recursive cold DSL dictionary reconstruction and Widget/Tree partial decode. | Domain-aware cold builder/decoder with partial-error retirement; keep wire format exact. Borrowed serializers may stay cold without cloning Dictionary. |
| F/🧩️extensions/🕸️wasm:80 evaluate_json; 96 evaluate_function_json | Successfully decoded input/tree/output are dropped after encoding; a later decode error drops earlier tree. | Explicit cold input/output/tree owners at the JSON/Wasm boundary; their presence must be visible in source. |
| Flow plugin list extension:42 Get; 71 Set; 85 Append; 99 Size; 111 Remove; 161 read_list | read_list clones Dictionary; later `?`, early return, or normal scope exit releases it. | Borrow list where possible; otherwise named ColdDictionary owner. Protect partially built `out` on failure. Other operator helpers returning newly constructed output transfer it rather than retiring it. |
| I:14 Step.params; I:65 EffectLogEntry.input/output; I:78 RunResult.scope | Path bodies, logs, and results hold nested dictionaries. | Explicit domain aggregate retirement for Path/Step/RunResult. Patchable assignment I:48, repeat scope I:147 and merge scope I:160 must capture displaced Dictionary. |
| Imperative extension SDK evaluate_json:49 | Same parsed input/output drop as Flow SDK. | Same explicit cold boundary ownership; no automatic Dictionary destructor. |
| Sequence artifact StepParams:45; Sequence editor set_step_params_json:531, prepare recipe:954 | Transparent wrapper, direct assignment, clone-based inverse and whole prepared-owner cleanup. | StepParams delegates strict Dictionary owner; Sequence typed retirement must extract params. Preserve existing wire shape and audit ordinary host/test boundaries explicitly. |
| Procedural2d/3d snapshot DSL/binary + mutation binary; P3 replay-displaced retirement | Dictionaries embedded in Widget, decoder dictionary arrays, copied params, early decode failure. | Delegate final Dictionary/Value to neural cursor; staged decoding uses cold guards only at cold codec entry. P3 shared Flow retirement owner is already installed by its executor and will use this domain seam. |

## Decoder and Convenience Traps

Generic `OrderedMap<Value>::insert` and `Deserialize` currently complete their generic cold cleanup with `drop(V)`. Once Value::Dictionary is strict, this is not a domain retirement operation. Dictionary must use its own cold builder/visitor that drives UpdateCursor, drains displaced map roots, and recursively handles every OwnedValue. The ordinary generic APIs remain valid for plain V but must not be used for Dictionary's backing.

A custom Dictionary decoder alone does not protect a later failure in an aggregate's derived decoder: a Widget/Neuron/Step field can successfully decode params before another field fails. Those aggregate codec entry points need explicit guarded field staging/cold owners, not a Dictionary::Drop fallback. Likewise, serde error cleanup is not interactive parsing and receives no bounded runtime credit.

No change to Dictionary's internal map can by itself make node_hash, Schema::validate, Registry::dispatch, full evaluation, or borrowed get/merge byte-bounded. Retained publication must keep using its own admitted, checkpointed recipe and the shared comparison/copy/retirement seams.

## Immediate Authorized Implementation

Implement the standalone neural Value/Dictionary retirement frontier and explicit cold builder/JSON decoder helpers against the current BTreeMap/pop_first backing first. Use schema-first nested/long-key/grant/cancel fixtures, independent serialization oracle, strict nonempty-owner guard, and exact byte totals. After OrderedMap native approval, replace that frontier's backing arm with the map retirement owner and switch the cold builder to UpdateCursor. Then audit and migrate the concrete host/cache/codec owning boundaries above; no hidden run-to-completion Dictionary destructor and no broad interactive-completion claim.

## Direct Neural-Related Rust Reference Inventory


### neural (1)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`

### frameworkFlow (8)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧵️retained/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️bridge/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️component.rs`

### pluginFlow (13)

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🧾️canonical/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🗿️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs`

### procedural (9)

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`

### imperative (23)

- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step-at/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🧪️tests/mutate-imperative-1/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📝️compiler/🦀️component.rs`

### sequence (4)

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️component.rs`

