#!/usr/bin/env python3
"""M-2 helper: writes `r2-m2-mapping.json` (original launch name -> dashboard command -> status) for the 79 `.claude` entries
(from `claude-mapping.json`, rendered by `r2-m2-tables.py`), the 4 VS Code compounds and every M-1a deviation / dropped / conflict item."""
import json

T = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/"
S = "semio run "
claude = json.load(open(T + "🗑️generated/m2/claude-mapping.json", encoding="utf8"))

compounds = [
    {"original": "🧭️compound🖥️s⚛️react🗄️os-hub", "command": S + "compound:workspace/s-with-hub", "status": "exact", "note": "hub (ready 8787 /admin) then shell 6070 with S_HUB_URL"},
    {"original": "🧭️compound🖥️s⚛️react🌉️os-mcp", "command": S + "compound:workspace/s-with-os-mcp", "status": "exact", "note": "os-mcp http (no ready, as in VS Code) then shell 6070"},
    {"original": "🧭️compound🖥️s👥️users🗄️os-hub", "command": S + "compound:workspace/s-users-with-hub", "status": "exact", "note": "hub, slot 1 (6072, s-user1), slot 2 (6073, s-user2), each with S_HUB_URL"},
    {"original": "🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor", "command": S + "compound:@teaching/architecture-quiz/quiz-with-proctor", "status": "exact", "note": "proctor 8791 then quiz 6061"},
]
for compound in compounds:
    compound["note"] += "; stopAll true = stop together"

deviations = [
    {"family": "launch-only facts (12 rows)", "items": [
        {"original": "user-slot rows 131-134 (S_HUB_URL, S_DATA_DIR)", "command": S + "playground:s --param renderer=react|wgpu-wasm --param user-slot=1|2 --param hub", "status": "exact", "note": "ports 6072/6073/6067/6068, S_DATA_DIR .🧬semio/🔗space/s-user<N>; resolved"},
        {"original": "S_LOCAL_ONLY row 700", "command": S + "playground:s --param renderer=react --param local-only", "status": "exact", "note": "S_LOCAL_ONLY=1, port 6070; resolved"},
        {"original": "PLAYGROUND_LOCKED_EXAMPLE_ID rows 665-666", "command": S + "playground:puzzle5d --param renderer=react|wgpu-wasm --param example=capsule-dream", "status": "exact", "note": "exports the catalog slug, ports 6014/6114; resolved"},
        {"original": "viewer ready suffix rows 467-468", "command": S + "playground:generation3d --param renderer=react|wgpu-wasm --param app-role=viewer", "status": "exact", "note": "ready path /?plugin=generation3d&role=viewer, ports 6018/6118; resolved"},
        {"original": "SEMIO_DEBUG_CLOSE_PHASE=1 (row 336)", "command": S + "@semio-tech/process-process3d-rs:test --param build-mode=ship --param test-level=long --env SEMIO_DEBUG_CLOSE_PHASE=1 -- <args>", "status": "exact", "note": "free --env; resolved"},
        {"original": "SEMIO_CARGO_PREPARATION_TIMING=1 (row 2284)", "command": S + "@semio-tech/framework-os-kernel:test --param cache=skip-local --param test-level=long --param dependencies --param nextest-output=immediate --param cargo-jobs=1 --env SEMIO_CARGO_PREPARATION_TIMING=1 -- <args>", "status": "exact", "note": "free --env; resolved"},
    ]},
    {"family": "dead knobs (10 rows)", "items": [
        {"original": "CAD_JS_RENDERER_PLAY_PORT / PUZZLE_3D_PLAY_PORT / PUZZLE_5D_PLAY_PORT / SHOOTING_PLAY_PORT, `fixture concrete|base-icon`", "command": S + "playground:cad|puzzle3d|puzzle5d|shooting --param renderer=react|wgpu-wasm", "status": "intentionally dropped", "note": "no file reads the variables or interprets a `fixture` segment; the server binds the catalog port (6020/6120, 6013/6113, 6014/6114, 6019/6119); resolved"},
    ]},
    {"family": "axes not offered (5 rows)", "items": [
        {"original": "--skip-nx-cache on playground rows 461, 704", "command": S + "playground:generation3d|draw --param renderer=react -- served", "status": "intentionally dropped", "note": "a dev server is never cached; --env SEMIO_BUILD_BUDGET_MS=<ms> still works (resolved)"},
        {"original": "NEXTEST_SUCCESS_OUTPUT on bun test rows 2324, 2455, 2457", "command": S + "tool:workspace/bun-test --param file=<file> --param nextest-output=immediate", "status": "exact", "note": "A-1 amendment 4 offers axes with appliesTo.verbs on tools; resolved"},
    ]},
    {"family": "merged groups (3 rows)", "items": [
        {"original": "initial-record row 2527 (jpg, tiff, pdf only)", "command": S + "group:workspace/stdio-initial-record-tests --param cache=skip-local -- --lib --features component-app-assembly ordinary_and_controlled_initial_record_pack_body_diagnostic --no-fail-fast", "status": "declared now", "note": "new group of the three projects; resolved"},
        {"original": "snapshot sqlite native / source rows 2528-2529 (48 projects)", "command": S + "group:workspace/snapshot-sqlite-native | group:workspace/snapshot-sqlite-source", "status": "intentionally merged", "note": "runs every project that declares the target (superset, no drift); the 48-name list was a snapshot of that set"},
    ]},
    {"family": "port conflicts (2 rows)", "items": [
        {"original": "capsule-dream rows 665-666 waited on dead PUZZLE_5D_PLAY_PORT 6015/6115", "command": S + "playground:puzzle5d --param renderer=react|wgpu-wasm --param example=capsule-dream", "status": "exact", "note": "ready port is the catalog port 6014/6114 (the old ready action never fired)"},
    ]},
    {"family": "stale variant (1 row)", "items": [
        {"original": "native -- trinity (row 692)", "command": S + "playground:trinity-jack --param renderer=wgpu-native", "status": "exact", "note": "resolves to framework-os-dev:run-trinity-jack-native-dev"},
    ]},
    {"family": "literal token (1 row)", "items": [
        {"original": "workspace:dev -- mcp repo with MCP_PROXY_AUTH_TOKEN=repo-mcp-token (row 671)", "command": S + "workspace:dev-mcp-repo", "status": "declared now", "note": "inspector generates its own token; ready.printed:true gives the tokenised URL"},
    ]},
    {"family": "dropped dead rows", "items": [
        {"original": "nx run-many -t describe (rows 201-203 and the later parallel=1 row)", "command": "-", "status": "intentionally dropped", "note": "target describe removed from the seven extension projects in commit 653; the three groups were removed from the root manifest (check: 14 problems -> 0)"},
        {"original": "framework-snapshot-sqlite-io (.claude 72)", "command": "-", "status": "intentionally dropped", "note": "target test-snapshot-sqlite-io no longer exists"},
    ]},
    {"family": "prompted inputs without default (4 rows)", "items": [
        {"original": "proctor restore", "command": S + "@teaching/proctor:restore --param file=<backup>", "status": "exact", "note": "file is required"},
        {"original": "proctor erase / erase dry-run", "command": S + "@teaching/proctor:erase --param handle=<h> [--param dry-run]", "status": "exact", "note": "tag and learner are the router's alternatives; one of the three is needed"},
        {"original": "hub admin capability", "command": S + "@semio-tech/s-services-native:security-check --param admin-capability=<file>", "status": "exact", "note": "required; requires os-hub:dev"},
    ]},
    {"family": "M-1a conflicts", "items": [
        {"original": "7 rows on @semio-tech/repo-cli-rs:daemon|run|preferences", "command": S + "@semio-tech/repo-dashboard-rs:daemon|run|preferences", "status": "exact", "note": "retargeted by M-1a"},
        {"original": "s-host-foreign-kind-s / s-host-pinch-diagram-contrast-s (ticket scripts as Nx targets)", "command": S + "@semio-tech/framework-os-dev:s-host-foreign-kind-s", "status": "exact", "note": "requires playground:s; resolved. Owning ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END is open: moving them to its commands.json would remove Nx targets outside this slice"},
        {"original": "per-backend hub data roots", "command": S + "os-hub:dev-postgres|dev-neo4j", "status": "exact", "note": "OS_HUB_DATA .🧬semio/🌐hub/hub-dev-postgres|neo4j; resolved"},
        {"original": "stdio-docx-rs has no test-snapshot-sqlite-source", "command": S + "@semio-tech/stdio-docx-rs:test-snapshot-sqlite -- source", "status": "exact", "note": "resolved"},
    ]},
    {"family": "second quiz stack / steady (.claude entry-only overrides)", "items": [
        {"original": "TEACHING_ARCHITECTURE_QUIZ_WATCH=off", "command": S + "@teaching/architecture-quiz:dev --param steady", "status": "declared now", "note": "resolved"},
        {"original": "ports 6063/8793 and PROCTOR_DATA", "command": S + "@teaching/architecture-quiz:dev --param stack=beside", "status": "declared now", "note": "ready 6063; Nx options.env pin removed; resolved"},
    ]},
]

json.dump({"claude": claude, "compounds": compounds, "deviations": deviations}, open(T + "r2-m2-mapping.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(claude), len(compounds), sum(len(f["items"]) for f in deviations))
