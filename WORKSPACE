workspace(name = "semio")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# buf (https://docs.buf.build/build-systems/bazel)
http_archive(
    name = "rules_buf",
    sha256 = "523a4e06f0746661e092d083757263a249fedca535bd6dd819a8c50de074731a",
    strip_prefix = "rules_buf-0.1.1",
    urls = [
        "https://github.com/bufbuild/rules_buf/archive/refs/tags/v0.1.1.zip",
    ],
)

load("@rules_buf//buf:repositories.bzl", "rules_buf_dependencies", "rules_buf_toolchains")

rules_buf_dependencies()

rules_buf_toolchains()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

# gazelle (https://github.com/bazelbuild/bazel-gazelle)
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "56d8c5a5c91e1af73eca71a6fab2ced959b67c86d12ba37feedb0a2dfea441a6",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.37.0/rules_go-v0.37.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.37.0/rules_go-v0.37.0.zip",
    ],
)

http_archive(
    name = "bazel_gazelle",
    sha256 = "ecba0f04f96b4960a5b250c8e8eeec42281035970aa8852dda73098274d14a1d",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.29.0/bazel-gazelle-v0.29.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.29.0/bazel-gazelle-v0.29.0.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.19.5")

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

gazelle_dependencies()

# buf gazelle (https://docs.buf.build/build-systems/bazel#gazelle)
load("@rules_buf//gazelle/buf:repositories.bzl", "gazelle_buf_dependencies")

gazelle_buf_dependencies()

load("//:buf_deps.bzl", "buf_deps")

buf_deps()

# TODO Make Poetry work
#either https://github.com/martinxsliu/rules_python_poetry or https://github.com/soniaai/rules_poetry
# Poetry (https://github.com/martinxsliu/rules_python_poetry)
# load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# http_archive(
#     name = "rules_python",
#     url = "https://github.com/bazelbuild/rules_python/releases/download/0.1.0/rules_python-0.1.0.tar.gz",
#     sha256 = "b6d46438523a3ec0f3cead544190ee13223a52f6a6765a29eae7b7cc24cc83a0",
# )

# http_archive(
#     name = "rules_python_poetry",
#     url = "https://github.com/martinxsliu/rules_python_poetry/archive/v0.1.0.tar.gz",
#     sha256 = "8f0abc58a8fcf75341b4615c6b7d9bb254119577629f45c2b1bb60f60f31b301",
#     strip_prefix = "rules_python_poetry-0.1.0"
# )

# load("@rules_python_poetry//:defs.bzl", "poetry_install_toolchain", "poetry_install")

# poetry_install_toolchain(poetry_version = "1.3.2")

# poetry_install(
#     name = "semio",
#     pyproject_toml = "//src/packages/python:pyproject.toml",
#     poetry_lock = "//src/packages/python:poetry.lock",
#     dev = True,  # Optional
# )