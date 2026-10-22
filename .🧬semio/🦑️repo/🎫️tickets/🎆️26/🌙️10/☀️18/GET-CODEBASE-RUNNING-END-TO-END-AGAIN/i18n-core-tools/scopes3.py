"""Increment 3: the remaining CLI-path console/stream prose, one catalog per owning module, biggest first."""
import os, re
from lib import *
from mig import *

L2 = L
INCREMENTS = {
    "commands": dict(roots=[L + "/🎛️commands"], texts=L + "/🎛️commands/📖️texts", say="sayCommandsV1", handle="COMMANDS_TEXTS_V1", doc="the repo commands generate, setup, examples, dev, os, dependency-lint and the schema projection"),
    "caching": dict(roots=[L + "/⚡️caching"], texts=L + "/⚡️caching/📖️texts", say="sayCachingV1", handle="CACHING_TEXTS_V1", doc="the cache, bootstrap, graph, Vite and CI tooling of the build library"),
    "dependencies": dict(roots=[L + "/🕸️dependencies"], texts=L + "/🕸️dependencies/📖️texts", say="sayDependenciesV1", handle="DEPENDENCIES_TEXTS_V1", doc="the dependency truth and direction lints"),
    "build": dict(owner="../..", roots=[L + "/🏗️build", L + "/🏃️process"], texts=L + "/🏗️build/📖️texts", say="sayBuildV1", handle="BUILD_TEXTS_V1", doc="the build entry, its tool adapters and the Cargo launcher"),
    "authoring": dict(roots=[L + "/🏗️authoring"], texts=L + "/🏗️authoring/📖️texts", say="sayAuthoringV1", handle="AUTHORING_TEXTS_V1", doc="`repo new` and the crate and artifact tree authoring"),
    "cleanup": dict(owner="../..", roots=[L + "/🧹️normalization", L + "/🧼️workspace-cleanup"], texts=L + "/🧼️workspace-cleanup/📖️texts", say="sayCleanupV1", handle="CLEANUP_TEXTS_V1", doc="`repo clean` and the taxonomy normalization workflow"),
    "misc": dict(owner="../..", roots=[L + "/🔍️discovery", L + "/📏️ownership", L + "/🖱️ui", L + "/🤖️agent-clients", L + "/🧪️testing", L + "/🔄️transactions", L + "/🚨️policy", L + "/🎮️playground", L + "/🌐️network"], texts=L + "/🔍️discovery/📖️texts", say="sayDiscoveryV1", handle="DISCOVERY_TEXTS_V1", doc="the discovery, ownership, UI, agent-client, testing, transaction, policy and playground lines"),
    "process": dict(roots=[P], texts=P + "/📖️texts", say="sayProcessV1", handle="PROCESS_TEXTS_V1", doc="the process module: native build, wasm build, test tooling and lease tools"),
}
KINDS = ("console", "stream")
SKIP_DIRS = ("🧪️tests", "🧫️fixtures", "node_modules", ".cache", "🗑️generated", "target", "dist", "📖️texts")


def scope_of(rel, root):
    parts = rel[len(root) + 1:].split("/")
    head = parts[0] if len(parts) > 1 else os.path.basename(root)
    name = re.sub(r"^[^A-Za-z]+", "", head if len(parts) > 1 else head)
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-") or "root"


def discover(tree, name, exclude=()):
    """{file: scope} of every source below the increment's roots with a prose console/stream site; verify/commit/bootstrap catalogs are already done."""
    import specgen
    files = {}
    for root in INCREMENTS[name]["roots"]:
        for b, ds, ns in os.walk(tree.path(root)):
            ds[:] = [d for d in ds if d not in SKIP_DIRS]
            for n in ns:
                if not n.endswith((".ts", ".mjs", ".mts", ".cjs")) or n.startswith("."):
                    continue
                rel = os.path.relpath(os.path.join(b, n), tree.root)
                if "/⚖️laws/🗣️language-catalogs" in rel or rel in exclude:
                    continue
                if any(s["kind"] in KINDS for s in specgen.prose_sites(tree.text(rel))):
                    files[rel] = scope_of(rel, root)
    return dict(sorted(files.items()))


def catalog_config(name, tree):
    c = dict(INCREMENTS[name])
    c["files"] = discover(tree, name)
    c["kinds"] = KINDS
    return c
