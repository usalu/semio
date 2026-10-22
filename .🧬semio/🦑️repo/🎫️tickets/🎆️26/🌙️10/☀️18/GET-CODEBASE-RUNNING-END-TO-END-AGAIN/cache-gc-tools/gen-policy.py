#!/usr/bin/env python3
"""Generates staged/ledger-policy.json from the declarations below; every writer is resolved against git-tracked files."""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from paths import ROOT, NEW3  # noqa: E402

GIB, MIB = 1 << 30, 1 << 20
DAY = 86_400_000
HOUR = 3_600_000
MINUTE = 60_000

tracked = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-z", "--", "🧰️framework", "nx.json", ".cargo/config.toml", ".storybook", "🌎️hub", "👴️leutwiler"], cwd=ROOT, capture_output=True, text=True).stdout.split("\0")
tracked = [row for row in tracked if row]
problems = []


def writer(suffix):
    for name in ("gc-launch.ts", "gc-reclaim.ts"):
        if suffix == "new:" + name:
            return NEW3[name]
    rows = [row for row in tracked if row.endswith(suffix)]
    if len(rows) != 1:
        problems.append("writer %r matched %d tracked file(s): %s" % (suffix, len(rows), rows[:3]))
        return suffix
    return rows[0]


def area(id, directory, purpose, writers, *, root="repository", kind="tree", **rest):
    row = {"id": id, "root": root, "kind": kind}
    if directory is not None:
        row["directory"] = directory
    row.update(rest)
    row["writers"] = [writer(item) for item in writers]
    row["purpose"] = {"en": purpose[0], "de": purpose[1]}
    return row


def collect(unused_days, quiet_ms, custody="quiet", lease=None):
    row = {"unusedAgeMs": None if unused_days is None else unused_days * DAY, "quietMs": quiet_ms, "custody": custody}
    if lease:
        row["lease"] = lease
    return row


def reason(en, de):
    return {"en": en, "de": de}


ENV = "/📚️library/🏃️process/🌿️environment/🟦️.ts"
BUDGET = "/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"
REGISTRY = "/⚡️caching/📦️artifacts/📇️registry/🟦️.ts"

areas = []
for id, directory, purpose in (
    ("tools", "b/t", ("Pinned toolchains: rustup, Bazel, Nx, SDKs.", "Fest eingebundene Werkzeugketten: rustup, Bazel, Nx, SDKs.")),
    ("graph", "b/g", ("Repository graph facts and Nx daemon data.", "Repository-Graphfakten und Nx-Daemondaten.")),
    ("dependencies", "b/d", ("Locked node_modules generations.", "Gesperrte node_modules-Generationen.")),
    ("products", "b/p", ("Bazel workspaces, outputs, actions and receipts.", "Bazel-Arbeitsbereiche, -Ausgaben, -Aktionen und Belege.")),
    ("diagnostics", "b/o", ("Command diagnostics and test captures.", "Befehlsdiagnosen und Testmitschnitte.")),
    ("nx", "b/n", ("Nx graph namespaces (funded child images), not task artifacts.", "Nx-Graph-Namensräume (finanzierte Kind-Images), keine Task-Artefakte.")),
    ("leases", "b/l", ("Resource lease data.", "Ressourcen-Lease-Daten.")),
):
    areas.append(area(id, directory, purpose, [BUDGET], root="native", kind="partition", partition=id, managed={"owner": "image"}))
next(row for row in areas if row["id"] == "graph")["recreate"] = "derived"
areas.append(area("rust", "rust/fixed", ("Cargo store, compiled units and Nx task artifacts.", "Cargo-Speicher, kompilierte Einheiten und Nx-Task-Artefakte."), [BUDGET], kind="partition", partition="rust", managed={"owner": "image"}))
areas.append(area("control", "b/c", ("Bootstrap control: SQLite lease catalogs, claims and queues.", "Bootstrap-Steuerung: SQLite-Lease-Kataloge, Claims und Warteschlangen."), [BUDGET], root="native", kind="control", managed={"owner": "control"}))
areas.append(area("boot-recovery", "boot-recovery", ("Native boot recovery ledgers kept beside the partition tree.", "Native Boot-Wiederherstellungsprotokolle neben dem Partitionsbaum."), ["/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts"], root="native", budget={"bytes": 16 * MIB}, collect=collect(30, HOUR)))
areas.append(area("nx-artifacts", "nx/artifacts", ("Nx task artifacts, trimmed by Nx itself at maxCacheSize.", "Nx-Task-Artefakte, von Nx selbst bei maxCacheSize gekürzt."), ["/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts"], kind="nested", partition="rust", budget={"bytes": 3 * GIB}, managed={"owner": "nx"}))
areas.append(area("cargo-store", None, ("Cargo build, target, home and incremental state: the rest of the rust partition.", "Cargo-Build-, Target-, Home- und Inkrementalzustand: der Rest der Rust-Partition."), ["/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts"], kind="nested", partition="rust", budget={"remainder": True}, managed={"owner": "cargo"}))
areas.append(area("graph-inference", "inference", ("Optional graph caches and fact blocks, reclaimed in the background above the hard floor.", "Optionale Graph-Caches und Faktenblöcke, im Hintergrund oberhalb der harten Untergrenze zurückgewonnen."), ["/📥️inputs/💾️storage/🟨️.mjs", "new:gc-reclaim.ts"], kind="nested", partition="graph", budget={"bytes": 2 * GIB}, managed={"owner": "graph"}, reclaim={"softAvailableBytes": 1879048192, "targetAvailableBytes": 2147483648}))
areas.append(area("graph-closures", "build-input-closures", ("Content-addressed source closures of the graph.", "Inhaltsadressierte Quell-Closures des Graphen."), ["/🏗️build/📥️inputs/🟨️.mjs"], kind="nested", partition="graph", budget={"bytes": 1342177280}, managed={"owner": "graph"}))
areas.append(area("graph-lock-closures", "build-lock-closures", ("Content-addressed lock closures of the graph.", "Inhaltsadressierte Lock-Closures des Graphen."), ["/🏗️build/📥️inputs/🟨️.mjs"], kind="nested", partition="graph", budget={"bytes": 67108864}, managed={"owner": "graph"}))
areas.append(area("bazel-actions", "bazel/actions", ("Bazel disk cache, bounded by a share of the products partition.", "Bazel-Disk-Cache, begrenzt durch einen Anteil der Produkt-Partition."), ["/🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts"], kind="nested", partition="products", budget={"basisPoints": 4000}, managed={"owner": "bazel"}))

loose = (
    ("go", "go", ("Go build cache (GOCACHE).", "Go-Build-Cache (GOCACHE)."), [ENV], GIB, collect(14, 10 * MINUTE)),
    ("playwright", "tools/ms-playwright", ("Playwright browser downloads.", "Playwright-Browser-Downloads."), [ENV], 2 * GIB, collect(90, HOUR)),
    ("bin", "🗃️bin", ("Built repo, coordinator and dashboard binaries per mode.", "Gebaute Repo-, Coordinator- und Dashboard-Binärdateien je Modus."), ["/🚨️policy/🖥️cli/🟦️.ts"], GIB, collect(30, HOUR)),
    ("vite", "vite", ("Vite dependency caches per project.", "Vite-Abhängigkeitscaches je Projekt."), ["/📋️context/🔣️policy/🟦️.ts"], 256 * MIB, collect(14, 10 * MINUTE)),
    ("vitest", "vitest", ("Vitest caches per test owner.", "Vitest-Caches je Testbesitzer."), ["🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"], 256 * MIB, collect(14, 10 * MINUTE)),
    ("agents", "agents", ("Resource lease directories of agents and plugin rebuilds.", "Ressourcen-Lease-Verzeichnisse von Agenten und Plugin-Neubauten."), ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts"], 64 * MIB, collect(7, HOUR, "sqlite")),
    ("taxonomy", "taxonomy", ("Taxonomy reference digests and work files.", "Taxonomie-Referenz-Digests und Arbeitsdateien."), ["/🧹️normalization/🧮️reference-digests/🟦️.ts"], 256 * MIB, collect(30, HOUR)),
    ("cmake", "cmake", ("CMake build trees per mode.", "CMake-Build-Bäume je Modus."), ["/🏗️build/🛠️cmake/🏃️execution/🟦️.ts"], GIB, collect(30, HOUR)),
    ("dotnet", "dotnet", (".NET build state.", ".NET-Build-Zustand."), [REGISTRY], GIB, collect(30, HOUR)),
    ("tectonic", "tectonic", ("Tectonic TeX bundle and format cache.", "Tectonic-TeX-Bundle- und Formatcache."), [REGISTRY], 512 * MIB, collect(60, HOUR)),
    ("print-fonts", "print-fonts", ("Print font catalog downloads.", "Druck-Schriftkatalog-Downloads."), ["🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts"], 128 * MIB, collect(30, HOUR)),
    ("trunk", "trunk", ("Trunk wasm staging for the wgpu renderer.", "Trunk-Wasm-Staging für den wgpu-Renderer."), ["/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts"], GIB, collect(14, HOUR)),
    ("breaches", "breaches", ("Policy breach reports.", "Richtlinien-Verstoßberichte."), ["/🚨️policy/🏃️runner/🟦️.ts"], 128 * MIB, collect(30, HOUR)),
    ("bench", "bench", ("Benchmark outputs.", "Benchmark-Ausgaben."), ["/🧑‍💻dev/📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts"], 512 * MIB, collect(14, HOUR)),
    ("build", "build", ("Nx plugin command captures.", "Nx-Plugin-Befehlsmitschnitte."), ["/🔌️nx-plugin/📤️arguments/🟨️.mjs", "new:gc-launch.ts"], 128 * MIB, collect(7, HOUR)),
    ("stages", "stages", ("Native stage cache of warm builds.", "Nativer Stage-Cache warmer Builds."), ["/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts"], GIB, collect(14, HOUR)),
    ("generated", "🤖️generated", ("Agent session logs.", "Agenten-Sitzungsprotokolle."), ["/📦️packages/🐹️go/📓️session/🐹️.go"], GIB, collect(60, HOUR)),
    ("renderer-modules", "📺️renderer-modules", ("Renderer modules served to the storybook.", "Dem Storybook bereitgestellte Renderer-Module."), [".storybook/📖️stories/🧭️coordination/🟦️.ts"], 256 * MIB, collect(30, HOUR)),
    ("browser-actor-containment", "browser-actor-child-containment", ("Test artifacts of the browser actor child containment check.", "Testartefakte der Browser-Actor-Child-Isolationsprüfung."), ["🌎️hub/📦️packages/🦀️rust/📋️project.json"], 64 * MIB, collect(7, HOUR)),
    ("elan", "tools/elan", ("Lean toolchains of the workspace-owned elan home, one per pinned elan version.", "Lean-Toolchains des arbeitsbereichseigenen elan-Verzeichnisses, eine je festgelegter elan-Version."), ["/💤️realparts-of-powers-z-n/🟦️.ts"], 3 * GIB, collect(90, HOUR)),
    ("diff", "🔀️diff", ("Checkpoint and hook diffs.", "Checkpoint- und Hook-Diffs."), ["/📦️packages/🐹️go/🎫️checkpoint/🐹️.go"], GIB, collect(60, HOUR)),
    ("cargo-protected", "cargo", ("Cargo directory kept visible to the workspace cleaner.", "Cargo-Verzeichnis, das für die Arbeitsbereichsbereinigung sichtbar bleibt."), ["/🧼️workspace-cleanup/🛡️protection/🟦️.ts"], GIB, collect(14, HOUR)),
)
for id, directory, purpose, writers, budget, rule in loose:
    areas.append(area(id, directory, purpose, writers, budget={"bytes": budget}, collect=rule))

managed = (
    ("tests", "tests", ("Test task results and reports.", "Test-Task-Ergebnisse und -Berichte."), ["🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs"], GIB, "test-evidence"),
    ("evidence", "🧪️tests", ("Test evidence routed by the test output policy.", "Vom Testausgabe-Regelwerk geleitete Testbelege."), ["/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts"], GIB, "test-evidence"),
    ("oracles", "oracles", ("Pinned test oracles.", "Fest gehaltene Test-Orakel."), [REGISTRY], GIB, "test-evidence"),
    ("dashboard", "🎛️dashboard", ("Dashboard daemon state and preferences.", "Dashboard-Daemon-Zustand und -Einstellungen."), ["/🎛️dashboard/🏛️repo-domain/🦀️.rs"], 64 * MIB, "dashboard"),
    ("generator-inputs", "🔏️generator-inputs", ("Generated registry inputs restored by Nx.", "Von Nx wiederhergestellte generierte Registry-Eingaben."), ["/⚡️caching/🔣️policy.json"], 64 * MIB, "nx"),
    ("remote-execution", "remote-execution", ("Remote execution platform descriptors restored by Nx.", "Von Nx wiederhergestellte Remote-Execution-Plattformbeschreibungen."), ["/⚡️caching/📋️project.json"], 64 * MIB, "nx"),
)
for id, directory, purpose, writers, budget, owner in managed:
    areas.append(area(id, directory, purpose, writers, budget={"bytes": budget}, managed={"owner": owner}))

areas.append(area("query-index", None, ("Go event query index keyed by a repository fingerprint.", "Go-Ereignis-Abfrageindex, geschlüsselt über einen Repository-Fingerabdruck."), ["/⌨️cli/🎊️query-cache/🐹️.go", "/🌳️tree/📦️packages/🐹️go/🎊️query-cache/🐹️.go"], budget={"bytes": 64 * MIB}, collect=collect(30, HOUR), pattern="^[0-9a-f]{64}$"))

retirements = []


def retire(root, directory, why_en, why_de, quiet=DAY):
    retirements.append({"root": root, "directory": directory, "quietMs": quiet, "reason": reason(why_en, why_de)})


for name, en, de in (
    ("build-lock-closures", "Closures moved into the graph partition (g/build-lock-closures).", "Closures liegen jetzt in der Graph-Partition (g/build-lock-closures)."),
    ("nx-lock-closures", "No writer remains.", "Kein Schreiber übrig."),
    ("cargo-preparation", "Preparation moved into the rust partition (cargo/preparation).", "Die Vorbereitung liegt jetzt in der Rust-Partition (cargo/preparation)."),
    ("leases", "Leases moved into the leases partition.", "Leases liegen jetzt in der Leases-Partition."),
):
    retire("repository", name, en, de)
for name in ("nx-tooling", "nx", "sdk", "nx-downloads", "bazel", "bazel-modules", "native-executables", "binaryen", "dashboard-cli", "tectonic"):
    retire("repository", "tools/" + name, "Tool installs moved into the tools partition.", "Werkzeug-Installationen liegen jetzt in der Werkzeug-Partition.")
for name in ("framework", "held-javascript", "bazel", "k0", "ldta"):
    retire("native", name, "Pre-partition root without a writer.", "Wurzel aus der Zeit vor den Partitionen ohne Schreiber.")

policy = {
    "version": 1,
    "roots": {
        "repository": {"directory": ".🧬semio/🦑️repo/⚡️cache", "maximumBytes": 20 * GIB, "undeclaredUnusedAgeMs": 14 * DAY, "undeclaredQuietMs": HOUR, "ignored": [".DS_Store", "🧭️prune-stamp.json", "🧭️size-ledger.json"]},
        "native": {"directory": ".cache", "maximumBytes": 256 * MIB, "undeclaredUnusedAgeMs": 7 * DAY, "undeclaredQuietMs": HOUR, "ignored": [".DS_Store"]},
    },
    "areas": areas,
    "needs": [{
        "partition": "graph", "measuredOn": "2026-10-21", "headroomPercent": 20, "floorBytes": 1073741824,
        "components": [
            {"id": "closures", "area": "graph-closures", "measuredBytes": 1136250880},
            {"id": "lock-closures", "area": "graph-lock-closures", "measuredBytes": 14123008},
            {"id": "inference", "area": "graph-inference", "measuredBytes": 1818488832},
            {"id": "publication", "bytes": 536870912, "measuredBytes": 0},
        ],
    }],
    "retirements": retirements,
}
if problems:
    raise SystemExit("\n".join(problems))
out = os.path.join(HERE, sys.argv[1] if len(sys.argv) > 1 else "staged3", "ledger-policy.json")
with open(out, "w", encoding="utf-8") as handle:
    json.dump(policy, handle, indent=2, ensure_ascii=False)
    handle.write("\n")
print("wrote", out, len(areas), "areas", len(retirements), "retirements")
