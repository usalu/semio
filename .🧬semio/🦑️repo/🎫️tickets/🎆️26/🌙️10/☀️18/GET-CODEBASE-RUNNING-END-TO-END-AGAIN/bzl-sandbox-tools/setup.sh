#!/bin/bash
# usage: setup.sh <wsdir>
W="$1"; rm -rf "$W"; mkdir -p "$W"
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
printf 'hello\n' > "$W/in.txt"
cat > "$W/BUILD.bazel" <<'M'
genrule(name = "g", srcs = ["in.txt"], outs = ["out.txt"], cmd = "cat $(SRCS) > $@")
M
