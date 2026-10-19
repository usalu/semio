#!/bin/bash
# usage: prod.sh <scratchdir> : writes workspace <scratchdir>/ws with s -> checkout, emoji inputs/outputs/tree artifact
S="$1"; R=/Users/ueli/Documents/semio; W="$S/ws"; rm -rf "$W"; mkdir -p "$W"
cd "$R"
SCRIPT=$(ls | command grep 'script.ts$' | head -1)
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
echo "script=$SCRIPT ticket=$TICKET" >&2
cat > "$W/MODULE.bazel" <<'M'
module(name = "scratch", version = "0.0.0")
M
cat > "$W/.bazelrc" <<'M'
common --enable_bzlmod
common --enable_workspace=false
common --color=no
common --curses=no
build --symlink_prefix=/
M
ln -s "$R" "$W/s"
cat > "$W/rules.bzl" <<'M'
def _impl(ctx):
    out = ctx.actions.declare_directory(ctx.attr.out_name)
    ctx.actions.run_shell(
        inputs = ctx.files.srcs,
        outputs = [out],
        command = "mkdir -p {o}/🧰️d && cat {i} > {o}/🧰️d/📜️script.ts".format(o = out.path, i = " ".join([f.path for f in ctx.files.srcs])),
        mnemonic = "EmojiRule",
    )
    return [DefaultInfo(files = depset([out]))]
emoji_rule = rule(implementation = _impl, attrs = {"srcs": attr.label_list(allow_files = True), "out_name": attr.string()})
M
cat > "$W/BUILD.bazel" <<M
load(":rules.bzl", "emoji_rule")
filegroup(name = "fg", srcs = ["s/$SCRIPT", "s/$TICKET/brief-1019.md", "s/.bazelrc"])
genrule(name = "g", srcs = [":fg"], outs = ["out.txt"], cmd = "cat \$(SRCS) | wc -c > \$@")
genrule(name = "ge", srcs = [":fg"], outs = ["🧰️out/🟦️-out.txt"], cmd = "cat \$(SRCS) | wc -c > \$@")
genrule(name = "gu", srcs = [":fg"], outs = ["undeclared.txt"], cmd = "ls s/ 2>&1 | head -1 > \$@")
emoji_rule(name = "tree", srcs = [":fg"], out_name = "📜️script.snapshot")
M
