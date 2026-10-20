#!/usr/bin/env python3
"""Staged anchored edit of the renderer-host dependency policy (library, behind the HARD FREEZE). Usage: policy-renderer-hosts.py [--apply]  (default: dry run). SEMIO_ROOT overrides the repository root."""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.environ.get("SEMIO_ROOT") or HERE
while ROOT != os.path.dirname(ROOT) and not (os.path.isfile(os.path.join(ROOT, "bunfig.toml")) and os.path.isfile(os.path.join(ROOT, "bun.lock"))):
    ROOT = os.path.dirname(ROOT)

FILE = "/".join(["\U0001F9F0️framework", "\U0001F6CD️products", "\U0001F991️repo", "\U0001F528️modules", "\U0001F4DA️library", "\U0001F578️dependencies", "\U0001F9ED️direction", "\U0001F3D7️construction", "\U0001F7E8️.cjs"])

EDITS = [
    (
        'const RENDERER_HOST_UI_CONTRACT = Object.freeze({ package: "@semio-tech/framework-os", subpathPrefixes: Object.freeze(["./modules/renderer/engine/elements/"]) });\n',
        'const RENDERER_HOST_UI_CONTRACT = Object.freeze({ package: "@semio-tech/framework-os", subpathPrefixes: Object.freeze(["./modules/renderer/engine/elements/", "./testkit/renderer-host/"]) });\n'
        '/** \U0001F3AC️ The media window-kit contract the media transport host and the window kit share: the only plugin-owned subpaths a host may read. */\n'
        'const RENDERER_HOST_WINDOW_KIT_CONTRACT = Object.freeze({ package: "@semio-tech/plugin-window-kits", subpathPrefixes: Object.freeze(["./media"]) });\n'
        '/** \U0001F9EB️ Language-neutral corpora a host test reads through declared exports: published fixture modules and schema JSON modules. */\n'
        'const RENDERER_HOST_CORPUS_PATHS = Object.freeze(["(?:^|/)\U0001F9EA️testing/\U0001F9EB️fixtures/", "(?:^|/)\U0001F9EC️schema/\U0001F523️json/\U0001F7E6️\\\\.ts$"]);\n',
    ),
    (
        'const RESOLVED_NODE_BUILTIN_PATH',
        'const RENDERER_HOST_WINDOW_KIT_TARGETS = exportTargetPatterns(workspacePackages.filter(pkg => pkg.name === RENDERER_HOST_WINDOW_KIT_CONTRACT.package).map(pkg => ({ ...pkg, exportTargets: pkg.exportTargets.filter(row => RENDERER_HOST_WINDOW_KIT_CONTRACT.subpathPrefixes.some(prefix => row.subpath === prefix || row.subpath.startsWith(prefix + "/"))) })));\n'
        'const RESOLVED_NODE_BUILTIN_PATH',
    ),
    (
        '  RENDERER_HOST_ROOT,\n  ...RENDERER_HOST_UI_CONTRACT_TARGETS,\n',
        '  RENDERER_HOST_ROOT,\n  ...RENDERER_HOST_UI_CONTRACT_TARGETS,\n  ...RENDERER_HOST_WINDOW_KIT_TARGETS,\n  ...RENDERER_HOST_CORPUS_PATHS,\n',
    ),
]


def run(apply):
    path = os.path.join(ROOT, FILE)
    text = open(path, encoding="utf-8").read()
    status = []
    for old, new in EDITS:
        if new in text:
            status.append("already applied")
        elif text.count(old) == 1:
            text = text.replace(old, new, 1)
            status.append("anchored")
        else:
            raise SystemExit(f"anchor found {text.count(old)} times, expected 1:\n{old[:200]}")
    if apply:
        open(path, "w", encoding="utf-8").write(text)
    print(f"policy-renderer-hosts: {', '.join(status)} ({'written' if apply else 'dry run'})")


if __name__ == "__main__":
    run("--apply" in sys.argv[1:])
