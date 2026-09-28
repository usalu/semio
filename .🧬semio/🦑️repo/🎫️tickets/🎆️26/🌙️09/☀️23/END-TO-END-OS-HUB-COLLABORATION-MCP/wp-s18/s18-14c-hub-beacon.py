# -*- coding: utf-8 -*-
"""S18 §14c: the readiness beacon answers `not-found` only for a route the page itself renders as not found — `/hub` (the
hub workspace overlay) rendered the hub but raised the not-found beacon, so a harness or a readiness probe on `/hub`
read a boot failure. One predicate for page and beacon. Idempotent."""
import pathlib

SHELL = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx")
EDITS = [
    ("  const shellRoute = useMemo(() => parseShellRoute(shellUri.split(\"?\")[0] ?? \"/\"), [shellUri]);\n",
     "  const shellRoute = useMemo(() => parseShellRoute(shellUri.split(\"?\")[0] ?? \"/\"), [shellUri]);\n  /** 🧭️ The path the shell renders as not found — never the hub overlay route, which renders over the underlying session. */\n  const notFoundPath = shellRoute.kind === \"notFound\" && shellRoute.path !== SHELL_HUB_ROUTE ? shellRoute.path : null;\n"),
    ("    if (hostMode && shellRoute.kind === \"notFound\" && shellRoute.path !== SHELL_HUB_ROUTE) {\n      return <ShellRouteNotFoundPage path={shellRoute.path} onHome={() => navigateShellUri(\"/\")} />;",
     "    if (hostMode && notFoundPath !== null) {\n      return <ShellRouteNotFoundPage path={notFoundPath} onHome={() => navigateShellUri(\"/\")} />;"),
    ("    const notFound = hostMode && shellRoute.kind === \"notFound\";\n", "    const notFound = hostMode && notFoundPath !== null;\n"),
    ("  }, [session, initialExampleReady, error, pluginFilter, shellRoute.kind, hostMode, scope.rootRef]);", "  }, [session, initialExampleReady, error, pluginFilter, notFoundPath, hostMode, scope.rootRef]);"),
]
# the page memo's deps follow (`shellRoute` → `notFoundPath`) — applied by line in the same pass (the deps line is ~600 chars).


def main() -> None:
    text = SHELL.read_text(encoding="utf-8")
    for old, new in EDITS:
        if new in text:
            continue
        assert text.count(old) == 1, old[:80]
        text = text.replace(old, new)
    SHELL.write_text(text, encoding="utf-8")
    print("ok")


main()
