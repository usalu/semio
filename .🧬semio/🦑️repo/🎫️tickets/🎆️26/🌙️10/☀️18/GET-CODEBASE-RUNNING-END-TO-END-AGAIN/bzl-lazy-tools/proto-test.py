#!/usr/bin/env python3
# Prototype: turns the selected task of a generated workspace into a Bazel test rule and measures `bazel test` (stub runner).
import json, os, re, shutil, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import measure as M

REPO_TEST = '''
def _test_impl(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ".test")
    snapshots = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    name = ctx.label.name
    lines = ["#!/bin/sh", "bin=${TEST_SRCDIR%/*}", "rm -rf \\"$bin/" + name + ".snapshot\\" && mkdir -p \\"$bin/" + name + ".snapshot\\" || exit 1", "cd \\"$TEST_SRCDIR/$TEST_WORKSPACE\\" || exit 1", "echo stub-test " + name + " $TEST_SHARD_INDEX/$TEST_TOTAL_SHARDS", "exit 0"]
    ctx.actions.write(launcher, "\\n".join(lines) + "\\n", is_executable = True)
    files = [ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots
    runfiles = ctx.runfiles(files = files, transitive_files = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]))
    return [DefaultInfo(executable = launcher, files = depset([launcher]), runfiles = runfiles), testing.ExecutionInfo({"no-remote-exec": "1", "no-sandbox": "1"})]

repo_test = rule(implementation = _test_impl, test = True, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})
'''

def build(src, dst, label):
    if os.path.exists(dst): shutil.rmtree(dst)
    shutil.copytree(src, dst, symlinks=True)
    p = dst + "/tasks.bzl"
    open(p, "a").write(REPO_TEST)
    b = dst + "/BUILD.bazel"
    t = open(b).read()
    t = t.replace('load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task")', 'load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task", "repo_test")')
    name = label.split(":")[1]
    n = t.count('repo_task(\n    name = "%s",' % name)
    assert n == 1, n
    t = t.replace('repo_task(\n    name = "%s",' % name, 'repo_test(\n    timeout = "eternal",\n    name = "%s",' % name)
    open(b, "w").write(t)

def main():
    name, src, label = sys.argv[1], sys.argv[2], sys.argv[3]
    dst = src + "-test"
    build(src, dst, label)
    w = M.Ws(name + "-test", dst)
    out = {"name": name, "label": label}
    w.wipe_ob(); w.wipe_dc()
    flags = ["--build_tests_only", "--test_output=errors"]
    out["test_cold"] = w.run("test", [label], flags)
    out["test_warm"] = w.run("test", [label], flags)
    out["test_nocache"] = w.run("test", [label], flags + ["--nocache_test_results"])
    subprocess.run([M.BZ, *w.startup(), "clean"], cwd=dst, env=M.ENV, capture_output=True)
    out["test_after_clean"] = w.run("test", [label], flags)
    w.wipe_ob()
    out["test_after_server_loss"] = w.run("test", [label], flags)
    # runfiles size
    bin_ = subprocess.run([M.BZ, *w.startup(), "info", "bazel-bin"], cwd=dst, env=M.ENV, capture_output=True, text=True).stdout.strip().splitlines()[-1]
    rf = os.path.join(bin_, name.split(":")[-1] if False else label.split(":")[1] + ".runfiles")
    n = 0
    if os.path.isdir(rf):
        for r, d, f in os.walk(rf): n += len(f)
    out["runfilesEntries"] = n
    out["binPath"] = bin_
    w.kill()
    print(json.dumps(out))

if __name__ == "__main__": main()
