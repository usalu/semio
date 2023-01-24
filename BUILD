load("@bazel_gazelle//:def.bzl", "gazelle", "gazelle_binary")
gazelle_binary(
    name = "gazelle-buf",
    languages = [
        "@bazel_gazelle//language/proto:go_default_library",
        "@semio//gazelle/buf:buf",
    ],
)
gazelle(
    name = "gazelle",
    gazelle = ":gazelle-buf",
)