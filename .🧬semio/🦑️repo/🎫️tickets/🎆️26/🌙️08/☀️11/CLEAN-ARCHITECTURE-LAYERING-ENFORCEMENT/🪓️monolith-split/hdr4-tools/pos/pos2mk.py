"""🏗️ POS2 family writer: the family specs of the second sweep (slices that finished after round 1). Each spec is `crates` (working tree) and `base` (revision 1bae6115bc5, the last auto-commit before wave 5: crates that did not exist
there are skipped by the tool and listed as `absent`). Writes `families/<name>.json` and `families/groups.json` (slice -> sub-families). Read only except for those files.
usage: pos2mk.py"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from poslib import GEN

HERE = os.path.dirname(os.path.abspath(__file__))
REV = "1bae6115bc5"
idx = json.load(open(GEN + "/pos2-manifest-index.json", encoding="utf-8"))
F = "semio-framework-"
OS = F + "os-"
S = "semio-s-artifact-"
FAM = {}
GROUPS = {}


def fam(group, name, crates, base=None, note=""):
    bad = [c for c in crates if c not in idx]
    assert not bad, (name, bad)
    FAM[name] = dict(name=name, crates=crates, base=dict(rev=REV, crates=base if base is not None else crates), note=note)
    GROUPS.setdefault(group, []).append(name)


def pair(group, name, prod, tests, extra=(), base_extra=(), note=""):
    fam(group, name, [*prod, *tests, *extra], base=[*prod, *base_extra, *extra], note=note)


fam("okwp8", "okwp8", [OS + "store-tests-%02d" % i for i in range(1, 11)] + [OS + "store-sync-tests-1", OS + "store-sync-tests-2", OS + "store-member-open-tests", OS + "store-durable-group-tests", OS + "store-testing-fixtures", OS + "store-testing-doubles"] + json.load(open(HERE + "/families/okstore.json"))["crates"], base=json.load(open(HERE + "/families/okstore.json"))["crates"], note="OS-KERNEL-WP8: 14 test crates + 2 helpers over the 47 store crates (absolute pairing: the pre-carve tests lived in the kernel facade)")

dm2v = [F + "value", F + "value-factory", F + "value-retirement", F + "value-retirement-queue", F + "value-original-aliases", F + "value-shared-issuance", F + "value-intrinsic", F + "value-paged-list", F + "value-paged-text", F + "value-ordered", F + "value-native-encoding", F + "value-native-decoding", F + "value-clone-source", F + "value-retained-map", F + "value-retained-clone", F + "mutation-diff"]
fam("dm2", "dm2v", [*dm2v, F + "value-testing-allocator", F + "value-clone-source-testing"], base=dm2v, note="DM2 value test support: allocator + clone-source-testing owners and their 16 consumers")
d3 = [F + "3d", F + "3d-brep-boolean", F + "3d-brep-construction", F + "3d-brep-engine", F + "3d-brep-offset", F + "3d-brep-representation", F + "3d-brep-sweep", F + "3d-brep-tessellation", F + "3d-brep-validation"]
fam("dm2", "dm2d", [*d3, F + "3d-testing-support", F + "3d-mesh-contract", F + "3d-mesh-clipping", F + "3d-mesh-document", F + "3d-mesh-reconstruction"], base=d3, note="DM2 3d tessellation observer owner and the nine consumers")
hist = [OS + "history", OS + "history-record-decode"]
fam("dm2", "dm2h", [*hist, OS + "history-testing-retirement"], base=hist, note="DM2 history retirement helper")
fam("dm2", "dm2k", [F + "native-build-compiler-resources", F + "value-derive", OS + "kernel-dsl-derive", F + "process-routing-workspace", OS + "run", OS + "store-sync", "semio-repo-test-host"], base=[F + "value-derive", OS + "kernel-dsl-derive", OS + "run", OS + "store-sync", "semio-repo-test-host"], note="DM2 compiler-resources and routing-workspace owners with consumers")
fam("dm2", "dm2u", [F + "ui-testing-paint-policy", F + "ui", F + "ui-text", F + "ui-viewport", F + "artifact-reference", F + "dsl-record-ownership-oracle"], base=[F + "ui", F + "ui-text", F + "ui-viewport", F + "artifact-reference"], note="DM2 ui paint-policy / ownership-oracle owners with consumers")
fam("dm2", "dm2p", [F + "pack-schema-index", F + "pack-record-schema", OS + "pack", F + "pack", F + "pack-intrinsic-media-laws"], base=[F + "pack-record-schema", OS + "pack", F + "pack"], note="DM2 pack slot index + intrinsic media laws macro owner")
fam("dm2", "dm2m", [OS + "mcp-foundation", OS + "mcp-plugin-runtime", OS + "mcp", F + "trace-memory-testing"], note="DM2 requested-allocation observer re-pointing (mcp)")
fam("dm2", "dm2e", [F + "plugin-host-engine", F + "plugin-host-wake", F + "plugin-host-operation", F + "plugin-interpreter", F + "plugin-interpreter-runtime"], note="DM2 plugin host deadline/root moves")
fam("dm2", "dm2t", [F + "test-transient-preparation-law", S + "writer-writer", S + "reasoning-wires", S + "forms-forms"], base=[S + "writer-writer", S + "reasoning-wires", S + "forms-forms"], note="DM2 transient-preparation-law owner and its three consumers")
fam("dm2", "dm2l", [S + "lowpoly-lowpoly-mut-laws", S + "lowpoly-lowpoly-mut-geometry", S + "lowpoly-lowpoly-mut-objects", S + "lowpoly-lowpoly-mut-paint", S + "remodel-remodeling-mut-vectors", S + "remodel-remodeling-mut-calib", S + "remodel-remodeling-mut-media", S + "remodel-remodeling-mut-params", S + "remodel-remodeling-mut-results"], base=[S + "lowpoly-lowpoly-mut-geometry", S + "lowpoly-lowpoly-mut-objects", S + "lowpoly-lowpoly-mut-paint", S + "remodel-remodeling-mut-calib", S + "remodel-remodeling-mut-media", S + "remodel-remodeling-mut-params", S + "remodel-remodeling-mut-results"], note="DM2 mutation law libraries (plugin artifact crates)")

fam("frs", "frs", [F + "number", F + "async", F + "dsl", OS + "channel"], note="FRAMEWORK-RUST-FILE-SPLITS")
fam("tgm", "tgm", [F + "tool-run", F + "tool-machine", F + "machine", F + "graph-dsl", F + "graph-engine", F + "graph-layout-run"], note="TOOL-GRAPH-MACHINE-SPLITS")
fam("orv", "orv", [OS + "run", OS + "vcs", OS + "pack", OS + "io", OS + "kernel-neural-engine-model"], note="OS-RUN-VCS-IO-SPLITS")
fam("svm", "svm", [F + "schema", F + "schema-validator", F + "value-derive", F + "value-resident", F + "math-sampling-processors", F + "math-sampling-constraints"], note="SCHEMA-VALUE-MATH-SPLITS")

pair("otcp", "otcps", [OS + "services"], [OS + "services-tests"], note="OTCP services")
pair("otcp", "otcpd", [OS + "directory"], [OS + "directory-tests-1", OS + "directory-tests-2"], note="OTCP directory (2 companions)")
pair("otcp", "otcpc", [OS + "directory-client"], [OS + "directory-client-tests"], extra=[OS + "directory", OS + "services"], note="OTCP directory client (+ directory and services, whose names the client tests import)")
pair("otcp", "otcpm", [OS + "command"], [OS + "command-tests"], note="OTCP command")
pair("otcp", "otcph", [OS + "history-record-decode"], [OS + "history-record-decode-tests"], note="OTCP history record decode")
pair("otcp", "otcpch", [OS + "channel"], [OS + "channel-tests"], note="OTCP channel")
pair("otcp", "otcpq", [F + "io-sqlite-snapshot"], [F + "io-sqlite-snapshot-tests"], note="OTCP io-sqlite-snapshot")
pair("otcp", "otcpg", [OS + "config"], [OS + "config-tests-1", OS + "config-tests-2"], note="OTCP config (2 companions)")
fam("otcp", "otcpx", [OS + "kernel-dsl-derive", OS + "kernel-dsl-derive-tests", OS + "kernel-dsl-derive-expand", F + "native-build-compiler-resources"], base=[OS + "kernel-dsl-derive"], note="OTCP dsl-derive + expand (+ compiler-resources owner of DM2)")

pair("ftc", "ftcb", [F + "io-base64"], [F + "io-base64-tests"], note="FTC io-base64")
pair("ftc", "ftcp", [F + "print"], [F + "print-tests"], note="FTC print")
pair("ftc", "ftcs", [F + "server"], [F + "server-tests"], note="FTC server")
pair("ftc", "ftcq", [F + "quiz"], [F + "quiz-tests", F + "quiz-primitives-tests"], note="FTC quiz (2 companions)")
pair("ftc", "ftcj", [F + "pack-json"], [F + "pack-json-tests"], note="FTC pack-json")
pair("ftc", "ftcx", [F + "pixels"], [F + "pixels-tests"], note="FTC pixels")
pair("ftc", "ftcw", [F + "artifact-workflow-run"], [F + "artifact-workflow-run-tests"], extra=[OS + "space-testing-sqlite-native-caller", F + "io-sqlite-snapshot-testing-backing"], note="FTC workflow-run (+ the DM2 sqlite test helpers its laws glob-import)")
pair("ftc", "ftca", [F + "async"], [F + "async-tests", F + "async-wasm-pool-tests"], note="FTC async (2 companions)")
pair("ftc", "ftcu", [F + "ui-host"], [F + "ui-host-tests"], note="FTC ui-host")
pair("ftc", "ftcv", [F + "value", *[c for c in dm2v if c != F + "value"]], [F + "value-tests", F + "value-testing-allocator", F + "value-clone-source-testing"], note="FTC value (+ the value crates of DM2 so the re-exported modules resolve)")
fam("ftc", "ftcm", [F + "mesh-engine", F + "artifact-space-collection", OS + "space-testing-sqlite-native-caller", F + "io-sqlite-snapshot-testing-backing"], base=[F + "mesh-engine", F + "artifact-space-collection"], note="FTC mesh-engine, space-collection (inputs read at run time) + the DM2 sqlite test helpers")

pair("ttc", "ttc", [F + "tool-run", F + "tool-machine", F + "machine"], [F + "tool-run-tests", F + "tool-machine-tests"], note="TOOL-TEST-COMPANIONS")

hub = ["semio-hub-directory", "semio-hub-directory-contract", "semio-hub-directory-dispatch", "semio-hub-directory-service", "semio-hub-directory-sqlite", "semio-hub-directory-postgres", "semio-hub-directory-neo4j", "semio-hub-directory-laws", "semio-hub-directory-backend-laws", "semio-hub-directory-route-laws", "semio-hub-route-models", "semio-hub-route-testkit", "semio-hub-document-routes", "semio-hub-session-routes", "semio-hub-session-route-laws", "semio-hub-presence-route-laws", "semio-hub-artifact-creation-service", "semio-hub-local-bootstrap", "semio-hub-stores", "semio-hub-ledgers", "semio-hub-open-plan-laws", "semio-hub-boot-laws", "semio-hub-hostile-input-laws", "semio-hub-lag-rebootstrap"]
fam("hdc", "hdc", hub, note="HUB-DIRECTORY-CAPABILITIES: port replaced by 15 capability traits (trait-in-scope risk)")

pk = [OS + "pack", F + "pack-record-schema", F + "pack-record", F + "pack-record-encode", F + "pack-record-retained", F + "pack-record-preflight", F + "pack"]
fam("opd", "opd", [*pk, OS + "store-text-io", OS + "store-codec-factory", OS + "store-durable-group", OS + "store-durable-group-wire"], note="OS-PACK-DEDUPE: os-pack vs pack-record-schema + consumers")
fam("orc", "orc", [*pk, OS + "store-command", OS + "store-document", OS + "store-engine-core", OS + "store-space-schema", OS + "store-text-io", OS + "dsl", "semio-framework-os-flow-mesh"], base=[*pk, "semio-framework-pack-record-token", OS + "store-command", OS + "store-document", OS + "store-engine-core", OS + "store-space-schema", OS + "store-text-io", OS + "dsl", "semio-framework-os-flow-mesh"], note="OS-PACK-RECORD-CODEC-DEDUPE: os-pack codec removed, pack-record-token dissolved")

fam("pqs", "pqs", [F + "pack-record", F + "pack-format-catalog", F + "quiz-schema"], note="PACK-QUIZ-FILE-SPLITS")
fam("rf8", "rf8", [F + "repo-statutes", F + "repo-cli-verbs", F + "repo-events", F + "repo-metrics", F + "repo-move", "semio-s-spatial-kernel-semio-session"], note="RUST-FILE-SPLITS-8")
fam("wbr", "wbr", [S + "energy-model-viewer", S + "energy-model-session", S + "remodel-remodeling-mut-vectors", S + "remodel-remodeling-mut-params"], note="WIRING-BUDGET-ROOTS")
fam("pfa", "pfa", [S + "bim-model", S + "process-process3d-host", S + "raster-raster", S + "norm-vdi3805-inferences", S + "norm-vdi3805-checks", S + "norm-en1999-inferences", S + "norm-en1999-checks", S + "norm-en1993-inferences", S + "norm-en1993-checks"], note="PLUGIN-FILE-SPLITS-A")
fam("pfb", "pfb", ["semio-s-plugin-energy-engine-sim", "semio-s-plugin-energy-engine-sim-restore", "semio-s-plugin-energy-engine-sim-retirement", "semio-s-plugin-energy-engine-sim-wire", S + "fem-3d-editor-visual-job", S + "fem-3d-editor-session", S + "energy-model-mutations"], note="PLUGIN-FILE-SPLITS-B")
fam("pch", "pch", [S + "stdio-mp3-io-metadata", S + "stdio-mp3-metadata", S + "stdio-mp3-mutations", "semio-s-plugin-puzzle-board-scene", "semio-s-plugin-puzzle-jobs", "semio-s-plugin-puzzle-retained", "semio-s-wfc-sqlite-bitmap", "semio-s-procedural-core-generation-sqlite", "semio-s-procedural-core-generation-sqlite-test-support", "semio-s-plugin-process-catalog-json-oracle", "semio-s-plugin-sourcing-catalog-oracle", OS + "space-sqlite-cells"], base=[S + "stdio-mp3-metadata", S + "stdio-mp3-mutations", "semio-s-plugin-puzzle-jobs", "semio-s-plugin-puzzle-retained", "semio-s-procedural-core-generation-sqlite", OS + "space-sqlite-cells"], note="PLUGIN-CRATE-HYGIENE: crates created or re-homed by the slice")
fam("pgg", "pgg", [S + "energy-model-editor-edit", S + "energy-model-session", S + "energy-model-viewer", S + "norm-vdi3805-inferences", F + "test-original-snapshot-owners"], note="PLUGIN-GATES-GREEN")

for n, d in FAM.items():
    with open(f"{HERE}/families/{n}.json", "w", encoding="utf-8") as f:
        json.dump(d, f, ensure_ascii=False, indent=1)
with open(f"{HERE}/families/groups2.json", "w", encoding="utf-8") as f:
    json.dump(GROUPS, f, ensure_ascii=False, indent=1)
print(len(FAM), "families;", {k: len(v) for k, v in GROUPS.items()})
