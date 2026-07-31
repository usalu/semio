"""Expand kit-store.comprehensive.compose.json to exercise every store GraphQL feature."""
import json
from pathlib import Path

POS = {
    "center": {"u": 7.0, "v": 8.0},
    "plane": {
        "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
        "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
        "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
    },
}

OFFSET = {"u": 0.25, "v": 0.25}

FEATURES = [
    "replay.golden.kit_graph_engine",
    "backbone.devJson",
    "backbone.localDotCompose",
    "read.store.wip.heads",
    "read.store.authoritative",
    "read.store.conflicts",
    "read.graph.releases",
    "read.graph.checkpoint",
    "read.query.session",
    "read.query.store",
    "read.query.node",
    "read.query.entity",
    "read.session.stores",
    "read.capture.designAndPiece",
    "read.afterWrites",
    "read.alternative.workspace",
    "mutation.session.start",
    "mutation.session.end",
    "mutation.session.localProvider",
    "mutation.theKit.startNewChange",
    "mutation.theKit.save",
    "mutation.theKit.createCheckpoint",
    "mutation.unsavedChange.save",
    "mutation.store.startAlternative",
    "mutation.store.backbone",
    "mutation.alternative.version",
    "mutation.alternative.integrateIntoTheKit",
    "mutation.kit.rename",
    "mutation.kit.changeDescription",
    "mutation.kit.createTag",
    "mutation.kit.createConcept",
    "mutation.kit.createQuality",
    "mutation.kit.createType",
    "mutation.kit.createDesign",
    "mutation.design.addFixedPiece",
    "mutation.piece.drag",
    "mutation.tag.rename",
    "mutation.stubs.metaAndDesign",
    "sidecar.stores.preview",
    "sidecar.checkpoints",
    "sidecar.installRename",
]

fixture = {
    "kind": "compose.kit_store.comprehensive",
    "schema": "🎆️26🌙️06⬆️1",
    "title": "Kit Store Comprehensive Scenario Catalog",
    "summary": "Ordered steps exercising every store GraphQL read head, VCS command, kit/design/piece write, replay engine, backbone replay, and compose-store sidecar flow.",
    "storeId": "test-store",
    "sidecarStoreId": "e0",
    "goldenOps": "kit-store.golden.ops.compose.json",
    "goldenExpected": "kit-store.golden.expected.compose.json",
    "features": FEATURES,
    "replayEngines": ["apply_create_fixed_piece", "kit_graph_engine"],
    "coverage": {
        "reads": [
            "store.wip (initialKit, theKit, checkpoints, alternatives, releases, checkpoint)",
            "store.authoritative.theKit",
            "store.conflicts",
            "query.session",
            "query.store",
            "query.node",
            "query.entity",
            "session.stores",
            "store.wip.alternative(id)",
        ],
        "writes": [
            "session.start",
            "session.end",
            "session.localProvider",
            "theKit.startNewChange",
            "theKit.save",
            "theKit.createCheckpoint",
            "unsavedChange.save",
            "store.startAlternative",
            "store.backbone",
            "alternative.version",
            "alternative.integrateIntoTheKit",
            "kit.rename",
            "kit.changeDescription",
            "kit.createTag",
            "kit.createConcept",
            "kit.createQuality",
            "kit.createType",
            "kit.createDesign",
            "design.addFixedPiece",
            "piece.drag",
            "tag.rename",
            "stubs (delete*, design flatten, piece fix/move, type port/connector, …)",
        ],
        "replay": ["createdFixedPiece via kit_graph_engine and apply_create_fixed_piece"],
        "backbone": ["devJson", "localDotCompose"],
        "sidecar": ["stores preview", "checkpoints", "install+rename"],
    },
    "steps": [
        {
            "id": "replay-golden-ops",
            "kind": "replayGoldenOps",
            "engine": "kit_graph_engine",
            "feature": "replay.golden.kit_graph_engine",
        },
        {
            "id": "read-wip-and-heads",
            "kind": "graphql",
            "feature": "read.store.wip.heads,read.store.authoritative,read.store.conflicts,read.graph.releases",
            "query": (
                "query ComprehensiveReadHeads {\n"
                "  store {\n"
                "    wip {\n"
                "      initialKit { name }\n"
                "      theKit { kit { name } }\n"
                "      checkpoints { edges { node { id } } }\n"
                "      alternatives { edges { node { id name } } }\n"
                "      releases { edges { node { id } } }\n"
                "    }\n"
                "    authoritative {\n"
                "      theKit { kit { designs { edges { node { pieces { edges { node { id } } } } } } } }\n"
                "    }\n"
                "    conflicts { edges { node { id } } }\n"
                "  }\n"
                "}"
            ),
            "expect": {
                "/store/wip/initialKit/name": "the kit",
                "/store/wip/theKit/kit/name": "the kit",
            },
            "expectArrayEmpty": ["/store/wip/alternatives/edges", "/store/conflicts/edges"],
            "expectAuthoritativeDesignsHaveNoPieces": True,
            "expectArrayMinLen": {"/store/wip/checkpoints/edges": 1},
            "capture": {"checkpointId": "/store/wip/checkpoints/edges/0/node/id"},
        },
        {
            "id": "read-query-session",
            "kind": "graphql",
            "feature": "read.query.session",
            "query": "query { session { id } }",
            "capture": {"sessionId": "/session/id"},
        },
        {
            "id": "read-query-store",
            "kind": "graphql",
            "feature": "read.query.store",
            "query": "query { store { wip { theKit { kit { name } } } } }",
            "expect": {"/store/wip/theKit/kit/name": "the kit"},
        },
        {
            "id": "read-query-node",
            "kind": "graphql",
            "feature": "read.query.node",
            "query": "query($id: ID!) { node(id: $id) { __typename } }",
            "variables": {"id": "${sessionId}"},
        },
        {
            "id": "read-query-entity",
            "kind": "graphql",
            "feature": "read.query.entity",
            "query": "query($hash: ID!) { entity(hash: $hash) { __typename } }",
            "variables": {"hash": "${sessionId}"},
        },
        {
            "id": "read-session-stores",
            "kind": "graphql",
            "feature": "read.session.stores",
            "query": (
                "query { session { stores { edges { node { wip { theKit { kit { name } } } } } } } }"
            ),
            "expect": {"/session/stores/edges/0/node/wip/theKit/kit/name": "the kit"},
        },
        {
            "id": "read-graph-checkpoint",
            "kind": "graphql",
            "feature": "read.graph.checkpoint",
            "query": (
                "query($cp: ID!) { store { wip { checkpoint(id: $cp) { id } } } }"
            ),
            "variables": {"cp": "${checkpointId}"},
        },
        {
            "id": "capture-design-and-piece",
            "kind": "graphql",
            "feature": "read.capture.designAndPiece",
            "query": (
                "query { store { wip { theKit { kit { designs { edges { node { id pieces { edges { node { id } } } } } } } } } } }"
            ),
            "capture": {
                "designId": "/store/wip/theKit/kit/designs/edges/0/node/id",
                "pieceId": "/store/wip/theKit/kit/designs/edges/0/node/pieces/edges/0/node/id",
            },
        },
        {
            "id": "session-start",
            "kind": "graphql",
            "feature": "mutation.session.start",
            "query": (
                "mutation { session { start { ok result { ... on IdResult { value } } } } }"
            ),
        },
        {
            "id": "open-unsaved-change",
            "kind": "graphql",
            "feature": "mutation.theKit.startNewChange",
            "query": (
                "mutation { session { store(id: \"${storeId}\") { theKit { startNewChange { ok result { ... on IdResult { value } } } } } } }"
            ),
            "capture": {"txId": "/session/store/theKit/startNewChange/result/value"},
        },
        {
            "id": "write-rename-kit",
            "kind": "graphql",
            "feature": "mutation.kit.rename",
            "query": (
                "mutation($tx: ID!, $n: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { rename(newName: $n) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "n": "ComprehensiveRenamed"},
        },
        {
            "id": "write-change-description",
            "kind": "graphql",
            "feature": "mutation.kit.changeDescription",
            "query": (
                "mutation($tx: ID!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { changeDescription(newDescription: \"comprehensive-desc\") { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}"},
        },
        {
            "id": "write-create-tag",
            "kind": "graphql",
            "feature": "mutation.kit.createTag",
            "query": (
                "mutation($tx: ID!, $name: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { createTag(name: $name) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "name": "comprehensive-tag"},
        },
        {
            "id": "write-create-concept",
            "kind": "graphql",
            "feature": "mutation.kit.createConcept",
            "query": (
                "mutation($tx: ID!, $name: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { createConcept(name: $name) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "name": "comprehensive-concept"},
        },
        {
            "id": "write-create-quality",
            "kind": "graphql",
            "feature": "mutation.kit.createQuality",
            "query": (
                "mutation($tx: ID!, $key: String!, $value: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { createQuality(key: $key, value: $value) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "key": "comprehensive-quality", "value": "1"},
        },
        {
            "id": "write-create-type",
            "kind": "graphql",
            "feature": "mutation.kit.createType",
            "query": (
                "mutation($tx: ID!, $name: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { createType(name: $name) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "name": "comprehensive-type"},
        },
        {
            "id": "write-create-design",
            "kind": "graphql",
            "feature": "mutation.kit.createDesign",
            "query": (
                "mutation($tx: ID!, $name: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { kit { createDesign(name: $name) { ok } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "name": "comprehensive-design-2"},
        },
        {
            "id": "write-design-add-fixed-piece",
            "kind": "graphql",
            "feature": "mutation.design.addFixedPiece",
            "query": (
                "mutation($tx: ID!, $designId: ID!, $bp: ID!, $pos: PositionInput!) {"
                " session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) {"
                " kit { design(id: $designId) { addFixedPiece(blueprintId: $bp, position: $pos) { ok } } }"
                " } } } } }"
            ),
            "variables": {"tx": "${txId}", "designId": "${designId}", "bp": "bp-golden-4", "pos": POS},
        },
        {
            "id": "capture-tag-id",
            "kind": "graphql",
            "feature": "mutation.tag.rename",
            "query": (
                "query { store { wip { theKit { kit { tags { edges { node { id name } } } } } } } }"
            ),
            "capture": {"tagId": "/store/wip/theKit/kit/tags/edges/0/node/id"},
        },
        {
            "id": "write-tag-rename",
            "kind": "graphql",
            "feature": "mutation.tag.rename",
            "query": (
                "mutation($tx: ID!, $tagId: ID!, $n: String!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) {"
                " kit { tag(id: $tagId) { rename(newName: $n) { ok } } } } } } } }"
            ),
            "variables": {"tx": "${txId}", "tagId": "${tagId}", "n": "comprehensive-tag-renamed"},
        },
        {
            "id": "write-piece-drag",
            "kind": "graphql",
            "feature": "mutation.piece.drag",
            "query": (
                "mutation($tx: ID!, $designId: ID!, $pieceId: ID!, $off: OffsetInput!) {"
                " session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) {"
                " kit { design(id: $designId) { piece(id: $pieceId) { drag(offset: $off) { ok } } } }"
                " } } } } }"
            ),
            "variables": {
                "tx": "${txId}",
                "designId": "${designId}",
                "pieceId": "${pieceId}",
                "off": OFFSET,
            },
        },
        {
            "id": "write-unsaved-save",
            "kind": "graphql",
            "feature": "mutation.unsavedChange.save",
            "query": (
                "mutation($tx: ID!) { session { store(id: \"${storeId}\") { theKit { unsavedChange(id: $tx) { save { ok } } } } } }"
            ),
            "variables": {"tx": "${txId}"},
        },
        {
            "id": "write-save",
            "kind": "graphql",
            "feature": "mutation.theKit.save",
            "query": (
                "mutation { session { store(id: \"${storeId}\") { theKit { save { ok } } } } }"
            ),
        },
        {
            "id": "write-create-checkpoint",
            "kind": "graphql",
            "feature": "mutation.theKit.createCheckpoint",
            "query": (
                "mutation($m: String!) { session { store(id: \"${storeId}\") { theKit { createCheckpoint(message: $m) { ok } } } } }"
            ),
            "variables": {"m": "comprehensive checkpoint"},
        },
        {
            "id": "write-start-alternative",
            "kind": "graphql",
            "feature": "mutation.store.startAlternative",
            "query": (
                "mutation($n: String!) { session { store(id: \"${storeId}\") { startAlternative(name: $n) { ok result { ... on IdResult { value } } } } } }"
            ),
            "variables": {"n": "comprehensive-branch"},
            "capture": {"altId": "/session/store/startAlternative/result/value"},
        },
        {
            "id": "sleep-wip-apply",
            "kind": "sleepMs",
            "ms": 150,
        },
        {
            "id": "read-after-writes",
            "kind": "graphql",
            "feature": "read.afterWrites",
            "query": (
                "query($designId: ID!) { store { wip { theKit { kit { name tags { edges { node { name } } }"
                " concepts { edges { node { name } } } qualities { edges { node { key } } }"
                " design(id: $designId) { pieces { edges { node { id } } } } } }"
                " alternatives { edges { node { id name } } } } } }"
            ),
            "variables": {"designId": "${designId}", "tagId": "${tagId}"},
            "expect": {
                "/store/wip/theKit/kit/name": "ComprehensiveRenamed",
                "/store/wip/theKit/kit/tag/name": "comprehensive-tag-renamed",
            },
            "expectArrayMinLen": {
                "/store/wip/alternatives/edges": 1,
                "/store/wip/theKit/kit/design/pieces/edges": 1,
            },
            "expectRelayContains": {
                "/store/wip/theKit/kit/concepts/edges": {"field": "name", "value": "comprehensive-concept"},
                "/store/wip/theKit/kit/qualities/edges": {"field": "key", "value": "comprehensive-quality"},
            },
        },
        {
            "id": "read-alternative-workspace",
            "kind": "graphql",
            "feature": "read.alternative.workspace",
            "query": (
                "query($id: ID!) { store { wip { alternative(id: $id) { name kit { name } } } } }"
            ),
            "variables": {"id": "${altId}"},
            "expect": {"/store/wip/alternative/name": "comprehensive-branch"},
        },
        {
            "id": "open-change-for-stubs",
            "kind": "graphql",
            "feature": "mutation.stubs.metaAndDesign",
            "query": (
                "mutation { session { store(id: \"${storeId}\") { theKit { startNewChange { ok result { ... on IdResult { value } } } } } } }"
            ),
            "capture": {"txStub": "/session/store/theKit/startNewChange/result/value"},
        },
        {
            "id": "write-stub-commands",
            "kind": "graphql",
            "feature": "mutation.stubs.metaAndDesign,mutation.alternative.version,mutation.alternative.integrateIntoTheKit",
            "query": (
                "mutation($tx: ID!, $designId: ID!, $pieceId: ID!, $altId: ID!) {"
                " session { store(id: \"${storeId}\") {"
                " theKit { unsavedChange(id: $tx) { kit {"
                " deleteTag(id: \"00000000-0000-7000-8000-000000000099\") { ok }"
                " design(id: $designId) { flatten { ok } piece(id: $pieceId) { fix { ok } } } }"
                " } }"
                " alternative(id: $altId) { version { ok } integrateIntoTheKit { ok } }"
                " } } }"
            ),
            "variables": {
                "tx": "${txStub}",
                "designId": "${designId}",
                "pieceId": "${pieceId}",
                "altId": "${altId}",
            },
        },
        {
            "id": "write-store-backbone",
            "kind": "graphql",
            "feature": "mutation.store.backbone",
            "query": (
                "mutation { session { store(id: \"${storeId}\") { backbone { detach { ok } sync { ok } } } } }"
            ),
        },
        {
            "id": "session-local-provider",
            "kind": "graphql",
            "feature": "mutation.session.localProvider",
            "query": (
                "mutation { session { localProvider { createBackbone(uri: \"dev://empty\") { ok } attachBackbone(store: \"${storeId}\") { ok } } } }"
            ),
        },
        {
            "id": "session-end",
            "kind": "graphql",
            "feature": "mutation.session.end",
            "query": "mutation { session { end { ok } } }",
        },
    ],
    "nativeSteps": [
        {
            "id": "backbone-dev-json-replay",
            "kind": "replayGoldenOpsBackbone",
            "backbone": "devJson",
            "feature": "backbone.devJson",
        },
        {
            "id": "backbone-sqlite-replay",
            "kind": "replayGoldenOpsBackbone",
            "backbone": "localDotCompose",
            "feature": "backbone.localDotCompose",
        },
    ],
    "sidecarSteps": [
        {
            "id": "sidecar-preview-wip",
            "kind": "graphql",
            "feature": "sidecar.stores.preview",
            "query": (
                "query { session { stores { edges { node { wip { initialKit { name } theKit { kit { name } } } } } } } }"
            ),
            "expect": {
                "/data/session/stores/edges/0/node/wip/initialKit/name": "the kit",
                "/data/session/stores/edges/0/node/wip/theKit/kit/name": "the kit",
            },
        },
        {
            "id": "sidecar-checkpoints-materialization",
            "kind": "graphql",
            "feature": "sidecar.checkpoints",
            "query": (
                "query { session { stores { edges { node { wip { checkpoints { edges { node { initial { name } kit { name } } } } } } } } } }"
            ),
            "expect": {
                "/data/session/stores/edges/0/node/wip/checkpoints/edges/0/node/initial/name": "the kit",
                "/data/session/stores/edges/0/node/wip/checkpoints/edges/0/node/kit/name": "the kit",
            },
        },
        {
            "id": "sidecar-install-rename-flow",
            "kind": "sidecarInstallRename",
            "feature": "sidecar.installRename",
            "installName": "SeedName",
            "renamedName": "SidecarComprehensiveRenamed",
        },
    ],
}

# Tag replay engine features as covered by replayEngines array (validated in Rust).
for eng in fixture["replayEngines"]:
    fixture["features"].append(f"replay.engine.{eng}")

out = Path(r"c:\git\compose\compose\assets\compose\kit-store.comprehensive.compose.json")
out.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("wrote", out, "steps", len(fixture["steps"]), "features", len(fixture["features"]))
