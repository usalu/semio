load("@rules_buf//buf:defs.bzl", "buf_dependencies")

def buf_deps():
    buf_dependencies(
        name = "buf_deps_src_schema_semio",
        modules = [
            "buf.build/googleapis/googleapis:75b4300737fb4efca0831636be94e517",
            "buf.build/grpc-ecosystem/grpc-gateway:a1ecdc58eccd49aa8bea2a7a9022dc27",
        ],
    )
