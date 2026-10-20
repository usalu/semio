#!/usr/bin/env python3
"""Course apply series for ROUND 2: x-integrate's ordered series (peer-ops, x-inputs, x-site, x-dev, x-release, x-paths, x-proctor, x-keys, reconcile, cleanup) plus
`course-bzl` (course_step.py) and, only with --with-graph, the corrected `x-graph` step (the original step looked for x-graph-tools/staged/apply.py; the script is x-graph-tools/apply.py).
Same interface as x-integrate-tools/apply-all.py: --check (read only, overlay) | --live (all or nothing), --only/--skip/--root/--json/--with-cleanup.
Extra flag: --with-graph. Do NOT pass it while another session runs against graph/admission files (Codex ticket 26/10/02 freezes owner-discovery, admission schema, graph fixture;
x-graph rewrites the admission schema + graph and invalidates every graph receipt once)."""
import importlib.util, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", "..", "..", ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️10", "☀️20", "ARCHITECTURE-COURSE-ON-BAZEL"))
if not os.path.isdir(TICKET):
    TICKET = os.path.abspath(os.path.join(HERE, *[".."] * 4, "☀️20", "ARCHITECTURE-COURSE-ON-BAZEL"))
spec = importlib.util.spec_from_file_location("x_integrate_apply_all", os.path.join(TICKET, "x-integrate-tools", "apply-all.py"))
series = importlib.util.module_from_spec(spec)
spec.loader.exec_module(series)

with_graph = "--with-graph" in sys.argv
sys.argv = [argument for argument in sys.argv if argument != "--with-graph"]
GRAPH_APPLY = os.path.join(TICKET, "x-graph-tools", "apply.py")
REBASED = os.path.abspath(os.path.join(HERE, "..", "rebased"))
series.release_step.STAGED = os.path.join(REBASED, "x-release")
series.STAGED["paths"] = os.path.join(REBASED, "x-paths")
BUILD = series.BUILD
GRAPH_TESTS = f"{BUILD}/🕸️graph/🚪️admission/🧪️tests/🗃️closure-cache"
GRAPH_FILES = [f"{BUILD}/🕸️graph/🟦️.ts", f"{BUILD}/🕸️graph/🚪️admission/🟦️.ts", f"{BUILD}/🧬️schema/🚪️admission.json", f"{GRAPH_TESTS}/🟦️.ts", f"{GRAPH_TESTS}/🔣️.json", f"{GRAPH_TESTS}/🧬️schema/🔣️.json", f"{GRAPH_TESTS}/🐍️oracle.py", f"{GRAPH_TESTS}/🏃️step/🟦️.ts"]
SITE = series.SITE
COURSE_FILES = [f"{SITE}/🎭️e2e/🟦️.ts", f"{SITE}/🧪️tests/🧱️local-stack/🟦️.ts", f"{SITE}/🧫️fixtures/🎭️e2e-proctor/🔣️.json", f"{SITE}/🧫️fixtures/🎭️e2e-proctor/🧬️schema/🔣️.json"]


def step_graph(root):
    series.sh([series.PYTHON, GRAPH_APPLY, "--live", "--root", root])


def step_course(root):
    series.sh([series.PYTHON, os.path.join(HERE, "course_step.py"), "--root", root])


rows = [row for row in series.STEPS if row[0] != "x-graph"]
cleanup = [row for row in rows if row[0] == "cleanup"]
rows = [row for row in rows if row[0] != "cleanup"]
rows.append(("course-bzl", "course residue: test-e2e --proctor resolves against the repository root (+ fixture, schema, test)", step_course, None))
if with_graph:
    rows.append(("x-graph", "persisted owner records + admission closure receipt (invalidates every graph receipt once)", step_graph, None))
series.STEPS[:] = rows + cleanup
series.TOUCHED.extend(path for path in COURSE_FILES + (GRAPH_FILES if with_graph else []) if path not in series.TOUCHED)

if __name__ == "__main__":
    sys.exit(series.main())
