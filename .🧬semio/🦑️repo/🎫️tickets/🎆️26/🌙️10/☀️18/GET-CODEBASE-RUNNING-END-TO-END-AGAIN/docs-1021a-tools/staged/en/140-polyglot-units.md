### 🧩️ Polyglot units and early cutoff per language

A unit is the smallest thing a build re-runs: one language, one kind (library, binary, test), its sources, inputs, dependencies, pinned externals, an interface and an action. The planner and the engine are language-neutral; an adapter per language only knows how to find units and what their interface is. {{?units::::The model and the `repo units` command are a staged set. }}State: units {{@units}}, pinned toolchains {{@toolchains}}.

```bash
repo units plan --for=go
repo units run --for=python --receipts --concurrency=4
repo units emit --for=cpp
repo units exec --for=go
```

- **The key and the cutoff.** A unit's key is the SHA-256 of its declared parts, the digests of its files and, per dependency, the **interface closure** (the dependency is used for compiling or checking: `interface`), the **output closure** (it is linked or run: `implementation`) or both (`both`: a native executable compiles against a library's interface and links its implementation); `declared` orders without keying. A dependent therefore re-runs only when that closure changes: early cutoff. Closures are recomputed from the current facts on every run, hit or not. A finding changed the TypeScript design: the TypeScript engine keyed only the direct dependencies' declaration digests, so with top, mid and low, where mid re-exports low's type, an edit in low left top cached while it was checked against low's declarations through mid. The neutral model keys the transitive interface closure (a unit may report which dependencies its interface mentions, which restores the narrow cone); the staged set makes that the TypeScript default.
- **Per language**, what the interface is and what a body edit does:

| Language | Interface | Evidence and effect |
|---|---|---|
| TypeScript | declarations of the unit (closure) | wraps the TypeScript units engine; a body edit re-checks one unit |
| Rust | the `.rmeta` digest | wraps `cargo metadata`; metadata holds spans and inlinable bodies, so a body edit also moves it: cutoff only on byte-identical metadata until an API digest exists |
| Go | export data (`__.PKGDEF` without its build-id line) | own scanner equal to `go list -overlay` on 53 of 53 compared packages of the repository; a body edit leaves the interface byte-identical, a signature edit moves it |
| Python | the declared API surface (`.pyi` replaces the module; unannotated returns ignored) | own lexer equal to CPython's `tokenize` and `ast` on 120 and about 100 standard-library files; a removed name fails the importer exactly as `import` does |
| .NET | SHA-256 of the reference assembly | project parser equal to `dotnet msbuild -getItem`; a private edit rebuilds the library, leaves the reference assembly and cuts off the app |
| C and C++ | comment- and layout-normalised public headers | CMake file API; `clang -E -P -dD` agrees on 8 of 8 header pairs; a comment edit in a header cuts off |
| WIT | canonical text of the world | own parser against `wit-parser` and its printer; a comment edit in a dependency is cut off |
| TeX | the normalised `.aux` (labels, citations, toc) | executor untested: tectonic is not installed on the primary host |

- **The same units under both tools.** `repo units emit` writes Nx targets whose inputs are the unit's own files plus the `iface.json` or `out.json` of its dependencies only (so Nx cuts off by itself), and a Bazel rule `repo_unit` whose interface dependencies appear only as `iface_deps`. Real Bazel 8.4.2: a Python body edit executes exactly one action (the dependents are cut off by Bazel itself), a signature edit executes the chain, a removed name fails at the importer; a pure-Go library is compiled with `go tool compile -importcfg` against the export archives of its dependencies, so a body edit of `a` executes `a`, its test and the binary and cuts off `b`; a C++ header comment edit executes the library and the app and cuts off the other library. Go binaries, tests and cgo or assembly packages, .NET and Rust units read dependency sources and cut off only in `repo units run`, not in Bazel.
- **Speed.** The Go graph of the repository (60 packages after the canonical layout merges member directories) plans in 14 s, dominated by loading the canonical plan of 44 modules; the engine's hot no-op measures 87 to 110 ms with the file store and 31 ms in memory; the Python plan of the repository takes 26 s for 10,232 files (9,927 are ticket scripts).
- **Pinned toolchains** ({{@toolchains}}). Identity is the pins plus the host plus the declared variables that are set, never an installation path, and no spawn is needed to compute it. Go 1.26.9 is a repository tool provisioned into the tools partition (size and SHA-256 checked, one atomic rename) with `GOTOOLCHAIN=local`: nothing depends on the Go on the `PATH`, and the user's machine is not modified. Python comes from uv-managed interpreters only, in an environment per lock (`.cache/b/d/m/python/environments/<id>-<16 hex>`, shared by agents; a new lock means a new directory, no live environment is mutated). The .NET SDK is pinned by `global.json` (`rollForward: disable`) and a raw build writes below `.cache/b/p/m/dotnet/raw/<project>`. CMake 4.4.4 and Ninja 1.13.2 are checksum-pinned for six hosts. Hermetic classes for these toolchains are declared and flip to provisioned by a data edit.
- **Limits.** The Bazel rule was run on Python, Go, C++ fixtures, not on the repository; Nx targets were emitted and shape-tested, not run; Windows and Linux were not run; the adapters cover what their oracles cover (7 of the 60 Go packages sit in emoji parent directories whose import path `go list` itself refuses, so they are not compared).
