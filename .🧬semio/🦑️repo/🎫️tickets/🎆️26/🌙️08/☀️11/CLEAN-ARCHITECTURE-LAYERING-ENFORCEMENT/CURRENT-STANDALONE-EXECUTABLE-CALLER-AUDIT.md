# Standalone Executable Caller Audit

Full current owning manifest/script/launch/CLI/test/helper/fixture authority frames are retained under `🗑️generated/standalone-caller-independent/current-frames-1.json`. Searches cover Framework, S, Hub, root scripts and .vscode executable assets, excluding historical ticket/generated source snapshots. This is read-only source audit; no new process or native assertions ran.

No executable production/owning script invocation of standalone pack or spr was found through cargo --bin, target debug/release executable paths, CARGO_BIN_EXE, Command::new or direct literal process argv. Owning OS scripts register library native tests; native-bin Cargo declarations identify binary targets but do not invoke their CLI. Current launch rows invoke the ticket standalone-native helper rather than hand-written standalone argv.

The active helper supplies all three required globals for actual Pack from-dsl and SPR compile fixture creators: transport-credit1048576,input-credit1048576,timeout-ms30000. Its18 fixture rows intentionally include refusal cases for missing/invalid globals; those are negative process laws, not callers needing defaults. Help cases intentionally need no policy. Actual runtime remains pending.

Direct main_impl SDK/unit-law calls are different: they supply explicit CommandContext and need no CLI policy prefix. Actual Pack/SPR standalone binary entry points construct CommandContext from parsed policy and pass the same token to stdin cancellation. Existing direct main_impl test calls retain explicit test_command_context. Historical ticket inputs, retained compiler snapshots and earlier source epochs are evidence, not fresh executable callers; no wholesale historical rewriting is warranted.

No concrete current executable caller omitting mandatory policy was identified in the inspected scope. This does not prove an external user invocation roster, and does not introduce a default/fallback compatibility requirement.
