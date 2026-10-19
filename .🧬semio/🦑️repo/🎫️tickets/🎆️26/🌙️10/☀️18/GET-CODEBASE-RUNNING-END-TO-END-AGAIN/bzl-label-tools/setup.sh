#!/bin/bash
set -e
W="$1"
rm -rf "$W/ws" "$W/src"
mkdir -p "$W/ws" "$W/src"
P="🧰️framework/🛍️products/🦑️repo"
mkdir -p "$W/src/$P/🔨️modules" "$W/src/$P/📋️data" "$W/src/🧰️framework/🛍️products/🦑️repo/🔧️tools"
printf 'export const a = 1;\n' > "$W/src/$P/🔨️modules/🟦️.ts"
printf 'export const b = 2;\n' > "$W/src/$P/🔨️modules/📜️script.ts"
printf '{"x":1}\n' > "$W/src/$P/📋️data/🔣️.json"
printf 'plain\n' > "$W/src/$P/plain.txt"
printf 'ascii\n' > "$W/src/ascii.txt"
cat > "$W/ws/MODULE.bazel" <<'M'
module(name = "scratch", version = "0.0.0")
M
cp "$2" "$W/ws/.bazelrc"
ln -s ../src "$W/ws/s"
mkdir -p "$W/ws/🧰️framework/🛍️products"
cat > "$W/ws/BUILD.bazel" <<'B'
load(":rules.bzl", "emoji_rule")

filegroup(
    name = "fg_label",
    srcs = [
        "s/🧰️framework/🛍️products/🦑️repo/🔨️modules/🟦️.ts",
        "s/🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️script.ts",
        "s/🧰️framework/🛍️products/🦑️repo/📋️data/🔣️.json",
        "s/ascii.txt",
    ],
)

genrule(
    name = "gen_label",
    srcs = [":fg_label"],
    outs = ["out-label.txt"],
    cmd = "cat $(SRCS) > $@",
)

genrule(
    name = "gen_emoji_out",
    srcs = [":fg_label"],
    outs = ["🧰️out/🟦️-out.txt"],
    cmd = "cat $(SRCS) > $@",
)

filegroup(
    name = "fg_glob",
    srcs = glob(["s/🧰️framework/**"]),
)

genrule(
    name = "gen_glob",
    srcs = [":fg_glob"],
    outs = ["out-glob.txt"],
    cmd = "for f in $(SRCS); do echo $$f; done | sort > $@",
)

genrule(
    name = "gen_deps",
    srcs = ["//🧰️framework/🛍️products:🦑️pkg_gen", "//🧰️framework/🛍️products:sub.txt"],
    outs = ["out-deps.txt"],
    cmd = "cat $(SRCS) > $@",
)

emoji_rule(
    name = "rule_emoji",
    srcs = [":fg_label"],
    out_name = "📜️script.snapshot",
)
B
cat > "$W/ws/🧰️framework/🛍️products/BUILD.bazel" <<'B'
exports_files(["sub.txt"])

genrule(
    name = "🦑️pkg_gen",
    srcs = ["sub.txt"],
    outs = ["🟦️.out"],
    cmd = "cat $(SRCS) > $@",
    visibility = ["//visibility:public"],
)
B
printf 'sub\n' > "$W/ws/🧰️framework/🛍️products/sub.txt"
cat > "$W/ws/rules.bzl" <<'B'
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
B
echo setup-done
