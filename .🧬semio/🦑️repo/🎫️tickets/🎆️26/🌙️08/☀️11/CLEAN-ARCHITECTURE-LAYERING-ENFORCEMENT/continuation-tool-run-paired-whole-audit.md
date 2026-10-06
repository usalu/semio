# ToolRun Actual Paired Whole And Compiler Audit

{
  "ready": true,
  "wholeGreen": true,
  "pairedRed": true,
  "redCompileOnly": true,
  "redDiagnostic": "E0433 unresolved original dsl::os_pack defining owner",
  "selected": 27,
  "completed": 27,
  "passed": 27,
  "failed": 0,
  "skipped": 0,
  "typescriptCases": 21,
  "typescriptExpectCalls": 3341,
  "redTypescriptCases": 21,
  "redTypescriptExpectCalls": 3307,
  "typescriptExpectCallsDynamic": true,
  "nativeRoster": [
    "component::tests::actions_chords_arguments_and_legality_match_the_fixture_and_the_matrix",
    "component::tests::definition_round_trips_through_serde_and_value_and_validates",
    "component::tests::labels_reserved_reasons_and_templates_match_the_fixture",
    "component::tests::lifecycle_invariants_hold_over_the_matrix_and_scenarios",
    "component::tests::lifecycle_matrix_covers_every_state_and_event_pair_exactly_once",
    "component::tests::limits_match_the_fixture",
    "component::tests::malformed_wire_values_are_rejected",
    "component::tests::panel_group_keys_and_reveals_follow_the_fixture",
    "component::tests::reducer_matches_every_lifecycle_case",
    "component::tests::reducer_matches_every_lifecycle_matrix_row",
    "component::tests::reducer_replays_every_lifecycle_scenario_with_its_commit_count",
    "component::tests::settings_pointers_resolve_like_the_rfc_6901_oracle",
    "component::tests::start_admission_follows_the_lane_law",
    "component::tests::step_ring_matches_the_fixture",
    "component::tests::tick_codec_enforces_the_tick_byte_cap",
    "component::tests::tick_writer_matches_the_fixture",
    "component::tests::tick_writer_resumes_from_a_provisional_base_and_retracts_its_entities",
    "component::tests::tick_writer_sequences_steps_pages_and_retracts_pending_appends",
    "component::tests::ticks_encode_to_and_decode_from_the_fixture_bytes",
    "component::tests::tool_run_trace_cursor_round_trips_the_schema_json_and_value_forms",
    "component::tests::trace_page_codec_keeps_full_u64_range_and_enforces_the_op_cap",
    "component::tests::trace_pages_encode_to_and_decode_from_the_fixture_bytes",
    "component::tests::trace_store_delivery_matches_the_fixture",
    "component::tests::trace_store_keeps_every_record_of_a_five_thousand_candidate_run",
    "component::tests::trace_store_rejects_stale_pages_and_resets_on_a_new_run",
    "component::tests::trace_store_residency_matches_the_fixture",
    "component::tests::writer_payload_is_the_latest_and_makes_a_tick"
  ],
  "physical": [
    {
      "kind": "red",
      "units": 44,
      "checks": 658,
      "pairs": 656,
      "unavailablePre": 191,
      "preGaps": 0,
      "currentGaps": 0,
      "snapshotGaps": 0,
      "terminal": 1
    },
    {
      "kind": "green",
      "units": 45,
      "checks": 664,
      "pairs": 660,
      "unavailablePre": 191,
      "preGaps": 0,
      "currentGaps": 0,
      "snapshotGaps": 0,
      "terminal": 0
    }
  ],
  "fullInversePhysicalPairs": 1316,
  "bindings": [
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/plan-1.json",
      "sha256": "4e09b4d40c15c22f6c354de9cd62a6111bc5b8434a084668a05cf6ad2cd2d7d7"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/metadata-1.json",
      "sha256": "fce22439c5b174ba339b3f2fd1949324db179fe57f2dba5d65ff488287fd8acc"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/independent-plan-provider-admission-2.json",
      "sha256": "eee39ac8422784acb2d5fb810005ae20e1245b83b03250f460b3a1afd4cc6d1b"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/source-1.json",
      "sha256": "e33920730672da8881f78bb9ec2235c8af4b98cabde011f92131e742815dc64d"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/source-independent-1.json",
      "sha256": "2d16b65e1c4b4fc7710bbc4f079ffe0d72362df57bb136a5a9613a500f84ed39"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/paired-helper-input-independent-2.json",
      "sha256": "3f84d4b250f02190343e0d88a064542faca0064630c8feb664c945dca6e531dd"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/paired-inherited-floor-1.json",
      "sha256": "0023596b3a07405ffac67ffebafa9be65cc490423f15d39886050a4f7c999681"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/tool-run-owner-paired-root-inputs-2/📜️script.ts",
      "sha256": "43f7337557d820fa05f796920bb594677a9afe99ae711be942b875e9751cab6f"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/native-json-inputs/📜️script.ts",
      "sha256": "d4a62d95cc4e9a633276470aa2032a11ce5aebe3117e3572e17bf162d40d8494"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/red/immediate-pre-1.json",
      "sha256": "7881d739f671591b6802aea9b5c0d4ae8d9b25c68d43c406977419b078d75807"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/red/terminal-1.json",
      "sha256": "d808995ddf86d01e8bd256a33fab38c861f285e1f0a1589df372091ff796c498"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/red/compiler-post-1.json",
      "sha256": "4773b8eb3b69fa7b973b7951e112c15ba83d86d00426c2b3c1639cd88f8d5d4c"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/paired-original-red-1.log",
      "sha256": "69ad2efa44d97deec2609f5733c95d2d46fd7e72e92379326441e0874ebed748"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/green/immediate-pre-1.json",
      "sha256": "b33bbfaa2af102ad1ecafc0668ca2469999b6814876351a711246f8e4188f33b"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/green/terminal-1.json",
      "sha256": "5d1ed27142b1171a9567256bbb6de7c6d85020abef0e13be6b3fdee8385c0cd3"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/green/compiler-post-1.json",
      "sha256": "5957324ca54d7e2ad18236da31f66bf8e3cdd2e64bf2d86ccd8c73127bfafedb"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/paired-original-green-1.log",
      "sha256": "dbb77b90f5bcca98c2abd0e899330ab970eca95faeb9a5cb72c7452399951981"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-direct-pack/native-original-whole-2.log",
      "sha256": "21a25c63ba802a976ce2d60db7c621064762862014d1d0cc1928b9ea271b80aa"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🔏️hash/🟦️.ts",
      "sha256": "3afcc48c0cb41e770d0ac55a3cbe4d4805f627184a9eec5483de4f124af98469"
    },
    {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/tool-run-owner-paired-root/epoch-1/independent-paired-compiler-full-inverse-ledger-1.json",
      "sha256": "c4fc5f0726b8f1aa6043df543b8dcf99ed61dc1dfadd9b1b015ccfe6b4a5e7b7"
    }
  ],
  "errors": [],
  "proposedDirectPackRowsExecuted": true,
  "publicationScope": "Only exact three direct Pack source/manifest/export pairs; independent current publication plan gate still required",
  "productDeletedClosureReady": false,
  "allCompiledPreidentityClaimed": false,
  "atomicIdentityClaimed": false,
  "scope": "Actual original whole paired ToolRun RED/GREEN and independent compiler-qualified current source joins; RED compilation failed before native assertions, GREEN original27native plus21TS; explicit unavailablePre preserved without preidentity or broader deletion claims",
  "nativeRosterQualification": "Current nextest pads first9 ordinals; original whole predecessor emits per-test libtest success envelopes. Both actual logs enumerate identical27passed names."
}

Both raw native logs and terminal/pre/physical records bound exactly. Every physical pair body/hash, retained snapshot predecessor and current physical body checked. Every raw compiler .dchecksum row independently recomputed through captured owned BLAKE3 implementation; no preidentity claimed for unavailable191rows perphase. Native27test names exactly match original whole predecessor roster. Initial parser missed padded first9 current ordinals and predecessor libtest envelopes; corrected successor reads both actual original formats. Dynamic TypeScript expect counts recorded from actual logs (RED3307, GREEN3341). Full physical forward/inverse ledger retains all1316pairs; no native assertion rerun performed in this audit lane.
