#!/usr/bin/env python3
"""Merged copy of the langs-conc set on top of repo-launcher: the Windows bootstrap header no longer lists Bun (the launcher provisions the JavaScript runtime), so the Go removal re-anchors on `installs Node.js, Go, uv and rustup`; the python and go dependency commands that pm-seam round 2 renders from the manager descriptors are the anchors of deps-python / deps-go. (READY4 re-based go-plan-env on the live runner: no longer dropped here.)"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/langs-conc-tools", "langs-conc")
patch(T + "/patches_root.py", '"installs Bun, Node.js, Go, uv and rustup from checksum-pinned archives", "installs Bun, Node.js, uv and rustup from checksum-pinned archives (Go', '"installs Node.js, Go, uv and rustup from checksum-pinned archives", "installs Node.js, uv and rustup from checksum-pinned archives (Go')
D = T + "/patches_deps.py"
patch(D, '''("deps-python", '  if (kind === "python") await run("uv", ["sync", "--locked", "--all-packages", "--all-groups"]);\\n',''', '''("deps-python", '  if (kind === "python") await run(...render("python", "sync", { scope: "all" }));\\n',''')
patch(D, '''("deps-go", '  else if (kind === "go") await run("go", ["mod", "download"], false, { ...process.env, GOWORK: join(workspace, "go.work") });\\n',''', '''("deps-go", '  else if (kind === "go") await run(...render("go", "sync"), false, { ...process.env, GOWORK: join(workspace, "go.work") });\\n',''')
patch(D, '''  else if (kind === "go") await run("go", ["mod", "download"], false, { ...(await goEnvironment(process.env)), GOWORK: join(workspace, "go.work") });''', '''  else if (kind === "go") await run(...render("go", "sync"), false, { ...(await goEnvironment(process.env)), GOWORK: join(workspace, "go.work") });''')
print("langs-conc merged copy:", T)
