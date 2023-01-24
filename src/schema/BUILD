load("@bazel_gazelle//:def.bzl", "gazelle", "gazelle_binary")
gazelle(
    name = "gazelle",
    gazelle = ":gazelle-buf",
)
gazelle(
    name = "gazelle-update-repos",
    args = [
        "--from_file=src/schema/buf.work.yaml",
        "-to_macro=buf_deps.bzl%buf_deps",
        "-prune",
    ],
    command = "update-repos",
    gazelle = ":gazelle-buf",
)
gazelle_binary(
    name = "gazelle-buf",
    languages = [
        "@bazel_gazelle//language/proto",
        "@rules_buf//gazelle/buf:buf",
    ],
    visibility = ["//visibility:public"],
)
