from mutgen import *

KK = lambda name: K[name]
FW = "🧰framework/🛍products/💻os/🔨modules"


def db_sources():
    mc = lambda name: rs(KK(name))
    return {
        "executor": src(rs(KK("INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES")[0])),
        "shard": src(mt(FW + "/🔌plugin/🖥host/🧵shard/🦀.rs")),
        "mcpTransport": src(mc("INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE")),
        "mcpBridge": src(mc("INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE"), mc("INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE"), join=""),
        "mcpRoot": src(mc("INTERACTIVITY_AUDIT_MCP_ROOT_FILE")),
        "storeSync": src(mc("INTERACTIVITY_AUDIT_STORE_SYNC_FILE")),
        "dbStorage": src(fam("INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY")),
        "dbSqlite": src(mc("INTERACTIVITY_AUDIT_DB_SQLITE_FILE")),
        "dbPostgres": src(mc("INTERACTIVITY_AUDIT_DB_POSTGRES_FILE")),
        "dbNeo4j": src(mc("INTERACTIVITY_AUDIT_DB_NEO4J_FILE")),
        "dbEngine": src(fam("INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY")),
        "dbSync": src(mc("INTERACTIVITY_AUDIT_DB_SYNC_FILE")),
        "dbArtifact": src(fam("INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY")),
        "dbWal": src(fam("INTERACTIVITY_AUDIT_DB_WAL_FAMILY")),
        "sprFormat": src(mc("INTERACTIVITY_AUDIT_SPR_FORMAT_FILE")),
        "packFormat": src(mc("INTERACTIVITY_AUDIT_PACK_FORMAT_FILE")),
        "dbSnapshot": src(mc("INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE")),
        "dbIndex": src(mc("INTERACTIVITY_AUDIT_DB_INDEX_FILE")),
        "dbCompact": src(mc("INTERACTIVITY_AUDIT_DB_COMPACT_FILE")),
        "dbState": src(mc("INTERACTIVITY_AUDIT_DB_STATE_FILE")),
        "dbQuery": src(mc("INTERACTIVITY_AUDIT_DB_QUERY_FILE")),
        "dbCli": src(mc("INTERACTIVITY_AUDIT_DB_CLI_FILE")),
        "dbTestkit": src(mc("INTERACTIVITY_AUDIT_DB_TESTKIT_FILE")),
        "hubBin": src(fam("INTERACTIVITY_AUDIT_HUB_BIN_FAMILY")),
        "asyncRuntime": src(mc("INTERACTIVITY_AUDIT_ASYNC_FILE")),
        "p1xContract": src(mc("INTERACTIVITY_P1X_CONTRACT_FILE")),
        "p1yContract": src(mc("INTERACTIVITY_P1Y_CONTRACT_FILE")),
        "p1zContract": src(mc("INTERACTIVITY_P1Z_CONTRACT_FILE")),
        "replicationWire": src(rs("🧰framework/🔨modules/📡replication/📡wire/🦀.rs")),
        "preparedRaster": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE")),
        "preparedRasterDraw": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE")),
        "preparedRasterGpu": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE")),
        "canvasRaster": src(mc("INTERACTIVITY_AUDIT_CANVAS_RASTER_FILE")),
        "interpreterRaster": src(mc("INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE")),
        "rendererGlue": src(mc("INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE")),
        "rendererHost": src(mc("INTERACTIVITY_AUDIT_RENDERER_HOST_FILE")),
        "uiFrame": src(rs("🧰framework/🔨modules/🖱ui/🖌render/🖼frame/🦀.rs")),
        "uiScene": src(rs("🧰framework/🔨modules/🖱ui/🖌render/🎬scene/🦀.rs")),
    }


def call(fn, *args, k=None):
    row = {"fn": fn, "args": list(args)}
    if k:
        row["k"] = k
    return row


def mounted_sources():
    mc = lambda name: rs(KK(name))
    fsx = lambda name: fs(KK(name))
    mtx = lambda name: mt(KK(name))
    FWUI = "🧰framework/🔨modules/🖱ui/🎯targets/🧊wgpu"
    OSR = FW + "/📺renderer/🧑‍🎨engine/🎯targets/🧊wgpu"
    cur = FW + "/♾infinite/🎲board/🔌ports/➡directed/🕸dag"
    reactor_files = ["INTERACTIVITY_AUDIT_REACTOR_FILE", "INTERACTIVITY_AUDIT_REACTOR_TURN_FILE", "INTERACTIVITY_AUDIT_SHARD_FILE", "INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE", "INTERACTIVITY_AUDIT_RUN_FILE", "INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE", "INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE", "INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE", "INTERACTIVITY_AUDIT_SHELL_FILE"]
    split = set(K["POLICY_SPLIT_SOURCE_ROOTS"])
    reactor = src(*[(mtx(n) if KK(n) in split else fsx(n)) for n in reactor_files])
    turn = src(*[sl(mc(n), "//#region 🔖TurnResult", "//#endregion 🔖TurnResult") for n in ("INTERACTIVITY_AUDIT_KERNEL_UI_FILE", "INTERACTIVITY_AUDIT_KERNEL_FILE")])
    central = mc("INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE")
    ui_files = ["BUILDER", "COMPONENT", "ACCESSIBILITY", "SURFACE", "DOCUMENT", "LIMITS", "LAYOUT", "PRESENT"]
    ui_schema = {"join": "", "parts": [
        {"parts": [mtx("INTERACTIVITY_AUDIT_UI_%s_FILE" % n) for n in ui_files]},
        turn,
        sl(central, "pub const TABLE_WINDOW_COLUMNS", "//#endregion 🔖TableWindowKit"),
        names(central, "fn ", ["table_rows_max_plus_one_returns_the_exact_row_owner", "abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity"]),
        sl(central, "pub const UI_COMMAND_VALUE_DEPTH", "//#endregion 🔖ActionFactory"),
        names(central, "fn ", ["retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner", "retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original"]),
    ]}
    return {
        "uiReconcile": src(mc("INTERACTIVITY_AUDIT_UI_RECONCILE_FILE"), mc("INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE"), mc("INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE")),
        "reactorPatches": src(mc("INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE")),
        "reactor": reactor,
        "uiValue": src(mc("INTERACTIVITY_AUDIT_UI_VALUE_FILE")),
        "uiSchema": ui_schema,
        "mountedLayout": src(mt(FWUI + "/📌mounted_layout/🦀.rs")),
        "mountedEngine": src(mt(FWUI + "/⚙engine/🦀.rs"), mt(FWUI + "/⚙engine/🪟surfaces/🦀.rs")),
        "mountedTree": src(mt("🧰framework/🔨modules/🖱ui/🌳retained/🌳tree/🍃nodes/🦀.rs")),
        "mountedPaint": src(mt(FWUI + "/🖌paint/🦀.rs")),
        "mountedEvents": src(mt(FWUI + "/⚡events/🦀.rs")),
        "mountedSlots": src(fs(FWUI + "/📨scene_slots/🦀.rs")),
        "mountedInterpreter": src(mtx("INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE")),
        "rendererGlueRs": src(mc("INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE")),
        "preparedRaster": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE")),
        "preparedRasterDraw": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE")),
        "preparedRasterGpu": src(mc("INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE")),
        "uiFrame": src(rs("🧰framework/🔨modules/🖱ui/🖌render/🖼frame/🦀.rs")),
        "uiScene": src(rs("🧰framework/🔨modules/🖱ui/🖌render/🎬scene/🦀.rs")),
        "surfaceLane": src(fsx("INTERACTIVITY_AUDIT_SURFACE_LANE_FILE")),
        "uiEngine": src(fsx("INTERACTIVITY_AUDIT_UI_ENGINE_FILE")),
        "rendererHost": src(fsx("INTERACTIVITY_AUDIT_RENDERER_HOST_FILE")),
        "winitHost": src(fsx("INTERACTIVITY_AUDIT_WINIT_HOST_FILE")),
        "glueTree": src(mtx("INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE")),
        "engineCanvasTree": src(mtx("INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE")),
        "browserWorker": src(fs(OSR + "/🌐browser-worker/🦀.rs")),
        "nodeGraph": src(fs("🧰framework/🔨modules/🗺surface/🕸node-graph/🦀.rs")),
        "flowHost": src(*[mt(FW + "/🌊flow/🖥host/%s/🦀.rs" % n) for n in ("🧱core", "🎞session")]),
        "tiledMap": src(mt("🧰framework/🔨modules/🗺surface/🗺tiled-map/🦀.rs")),
        "editorTree": src(mt("🧰framework/🔨modules/✍editor/🦀.rs")),
        "dagNodes": src(*[mt(cur + "/%s/🦀.rs" % n) for n in ("🗃model", "🕸graph", "🧭host")]),
    }
