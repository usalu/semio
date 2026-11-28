# Wave 1127 H52: actor_input decoder/wire peak (type-erased receive core)

## Measured
- Baseline: `repo cargo --store=shared --configuration=prototype --fresh-lane -- build -p semio-plugin-actor-input-decoder --lib` (nice 10, cold lane, 51 min, 481 units): decoder unit host peak 1121 MB, wall 84 s (telemetry `units-darwin-arm64.jsonl`, key semio-plugin-actor-input-decoder|host|prototype|lib). Fleet alarms were 1558 (host m13) / 2280 (wasm). The host peak here is already under 1536; the wasm peak was not measured.
- Dependency peaks for comparison: plugin-abi 512 MB, receiving 161 MB, native-receiving-original 132 MB. So the 84 s / 1.1 GB is spent in the decoder crate itself: monomorphising `OriginalJsonOutputSource<ActorInput>::{receive_step,take_step}` (generic over `T: NativeReceive<DslValue>`; `ActorInput`'s derived receiver tree is instantiated against the whole native receiving engine).

## Not done
- No fix implemented, no carving entry removed, nothing landed. Reasons: (1) each A/B needs a 50-min cold lane (the shared lane was not warm for a second build; the second `build -v` started recompiling syn again and was stopped); (2) `repo cargo -- rustc -p ... -- -Z time-passes` is refused ("Cargo compiler forwarding belongs to original repository admission"), so no per-pass profile; (3) coordinator 08:15 quiet window forbids manifest/graph landings.
- A doc-only edit I made to the decoder source for a `-v` capture was reverted byte-identical (cmp ok).

## Findings / plan
- Instantiation sites: decoder (`receive_step`/`take_step`), wire (`close_step`/`close_owned_step` in `📡️wire/🛬️decode.rs`, 19 lines) - both tiny wrappers; wire's 636 lines are not the cost.
- Candidate A (cheap, staged idea): one function per crate for each `Source<ActorInput>` method, so each crate pays only its method's instantiation; measure each.
- Candidate B (the clean fix): make the T-independent parts of `OriginalJsonOutputReceiving<T>` (parse, bind, admission, source; `🫴️unique/🦀️.rs` lines 50-82 `advance` loop, close_*) non-generic in the receiving crate with only the `NativeReceivingOriginal<DslValue,T>` slot and `take_into` typed; or generate a concrete `ActorInput` decoder from `📊️actor-input.json`.
- To profile: get the rustc argv (`ps -ww` while the decoder unit compiles, or `build -v` in a warm lane) and rerun with `-Z time-passes -Z print-mono-items` in a lane under a slot/quiet-window-compatible time.
- Carving entries for decoder/wire stay in `🎟️admission/🧬️schema/🔣️.json`.
