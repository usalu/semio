#!/usr/bin/env python3
"""🛣️ IO ROWS (S4-AGNOSTIC, coordinator GO 11:5x): every `io_mechanism::IoEntry` declares its native side (`direction`, set by
`serializer_entry*` → Export and `deserializer_entry*` → Import, never inferred); `io_native_routes()` lists the registry from that side;
`describe` lists the package's own io_mechanism rows beside the old composer rows (descriptor schema unchanged); laws for the constructors and
for describe. Anchored, count-asserted, staged in memory, every file re-read immediately before its write (all files in one run).
Usage: [--apply]."""
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
IO = "🧰️framework/🔨️modules/🚪️io/🦀️.rs"
LAWS = "🧰️framework/🔨️modules/🚪️io/🧪️tests/🔬️io-mechanism-laws/🦀️.rs"
HOST = "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs"
DESCRIBE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs"
DESCRIBE_TEST = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🧪️tests/🔬️unit/🦀️.rs"

EDITS = {
    IO: [
        (
            "    /// 🧾️ Type-erased io vtable row: one directed hop `from -> into` at a declared `fidelity`, an\n    /// optional `sniff` (carrier-dialect identification), and the erased `run` this hop executes.\n    pub struct IoEntry {\n        pub from: Dialect,\n        pub into: Dialect,\n        pub fidelity: IoFidelity,\n        pub sniff: Option<fn(&IoPayload) -> Confidence>,\n        pub run: fn(&IoPayload) -> IoResult<IoPayload>,\n    }\n",
            "    /// 🧲️ The side of an entry that is the registering artifact's native dialect, declared by the constructor and never inferred: an\n    /// `Export` entry serializes out of it (`from` is native), an `Import` entry deserializes into it (`into` is native).\n    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]\n    pub enum IoEntryDirection {\n        Export,\n        Import,\n    }\n\n    /// 🛤️ One registered entry seen from its native side — the shape a package descriptor lists as an import/export row.\n    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]\n    pub struct IoNativeRoute {\n        pub native: ArtifactDialect,\n        pub foreign: ArtifactDialect,\n        pub direction: IoEntryDirection,\n    }\n\n    /// 🧾️ Type-erased io vtable row: one directed hop `from -> into` at a declared `fidelity`, its native side `direction`, an\n    /// optional `sniff` (carrier-dialect identification), and the erased `run` this hop executes.\n    pub struct IoEntry {\n        pub from: Dialect,\n        pub into: Dialect,\n        pub fidelity: IoFidelity,\n        pub direction: IoEntryDirection,\n        pub sniff: Option<fn(&IoPayload) -> Confidence>,\n        pub run: fn(&IoPayload) -> IoResult<IoPayload>,\n    }\n",
        ),
        (
            "&& left.fidelity == right.fidelity && same_sniff && std::ptr::fn_addr_eq(left.run, right.run)\n",
            "&& left.fidelity == right.fidelity && left.direction == right.direction && same_sniff && std::ptr::fn_addr_eq(left.run, right.run)\n",
        ),
        (
            "            Err(_) => Vec::new(),\n        }\n    }\n    //#endregion 🔖️Identify\n",
            "            Err(_) => Vec::new(),\n        }\n    }\n\n    /// 🛣️ Every registered entry from its native side ([`IoNativeRoute`], registry order) — what `describe` lists as a package's\n    /// import/export rows. The synthesized native ↔ sqlite-snapshot hops of [`io_entries`] are generic, not registered entries.\n    pub fn io_native_routes() -> Vec<IoNativeRoute> {\n        let Ok(registry) = io_mechanism_registry().read() else { return Vec::new() };\n        registry\n            .values()\n            .map(|entry| {\n                let (from, into) = (ArtifactDialect::from(entry.from), ArtifactDialect::from(entry.into));\n                match entry.direction {\n                    IoEntryDirection::Export => IoNativeRoute { native: from, foreign: into, direction: entry.direction },\n                    IoEntryDirection::Import => IoNativeRoute { native: into, foreign: from, direction: entry.direction },\n                }\n            })\n            .collect()\n    }\n    //#endregion 🔖️Identify\n",
        ),
        ("        IoEntry { from: own, into: T::INTO, fidelity: T::FIDELITY, sniff: None, run: run::<S, T> }\n", "        IoEntry { from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }\n", 2),
        ("        IoEntry { from: T::FROM, into: own, fidelity: T::FIDELITY, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }\n", "        IoEntry { from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }\n", 2),
    ],
    LAWS: [
        (
            "    #[semio_framework_async_macros::async_test]\n    async fn conformance_runs_after_deserialize() {",
            "    /// 🧲️ LAW: the constructors declare the native side — a serializer entry exports out of its own dialect, a deserializer entry\n    /// imports into it.\n    #[semio_framework_async_macros::async_test]\n    async fn entry_constructors_declare_their_native_side() {\n        let exporting = serializer_entry::<semio_framework_value::DslValue, ChildrenEcho>(A);\n        assert_eq!((exporting.direction, ArtifactDialect::from(exporting.from), ArtifactDialect::from(exporting.into)), (IoEntryDirection::Export, ArtifactDialect::from(A), ArtifactDialect::from(C)));\n        let importing = deserializer_entry::<semio_framework_value::DslValue, JsonDeserializer>(A);\n        assert_eq!((importing.direction, ArtifactDialect::from(importing.from), ArtifactDialect::from(importing.into)), (IoEntryDirection::Import, ArtifactDialect::from(B), ArtifactDialect::from(A)));\n    }\n\n    #[semio_framework_async_macros::async_test]\n    async fn conformance_runs_after_deserialize() {",
        ),
    ],
    HOST: [
        (
            "        use semio_framework::io::io_mechanism::{IoEntry, io_register};\n",
            "        use semio_framework::io::io_mechanism::{IoEntry, IoEntryDirection, io_register};\n",
        ),
        (
            "[IoEntry { from: TEST_DIALECT, into: CARRIER_BINARY, fidelity: IoFidelity::Exact, sniff: None, run }];",
            "[IoEntry { from: TEST_DIALECT, into: CARRIER_BINARY, fidelity: IoFidelity::Exact, direction: IoEntryDirection::Export, sniff: None, run }];",
        ),
    ],
    DESCRIBE: [
        (
            "/// 🚪️ This plugin's own registered IO composer routes (`writes.artifact_kind` owned by\n/// `plugin_id`) — `ContributionSet.io_entries`/`composer_entries`, reading the real\n/// `io::list_composer_entries()` registry (`🚪️io/🦀️.rs`'s `IO_REGISTRY`). Each `(writes,\n/// reads)` composer row yields one `IoEntryDescriptor{owner: writes, counterpart: read, direction:\n/// Import}` per read dialect — `writes` is composed FROM `reads` (`ComposerEntry`'s own doc), so\n/// `Import` is the faithful direction from this package's (the `writes` owner's) perspective.\n",
            "/// 🚪️ This plugin's own registered IO routes — `ContributionSet.io_entries`/`composer_entries` — from both registries: each\n/// composer row of `io::list_composer_entries()` (`writes.artifact_kind` owned by `plugin_id`) yields one\n/// `IoEntryDescriptor{owner: writes, counterpart: read, direction: Import}` per read dialect (`writes` is composed FROM `reads`), and\n/// each `io_mechanism` entry whose native side the plugin owns (`io_mechanism::io_native_routes()`) yields\n/// `IoEntryDescriptor{owner: native, counterpart: foreign, direction}` — a serializer exports, a deserializer imports.\n",
        ),
        (
            "        composer_entries.push(ComposerEntryDescriptor { writes, reads });\n    }\n    (io_entries, composer_entries)\n}\n",
            "        composer_entries.push(ComposerEntryDescriptor { writes, reads });\n    }\n    for route in io::io_mechanism::io_native_routes() {\n        if !owns_artifact_kind(plugin_id, &route.native.artifact_kind).await {\n            continue;\n        }\n        let direction = match route.direction {\n            io::io_mechanism::IoEntryDirection::Export => IoEntryDirection::Export,\n            io::io_mechanism::IoEntryDirection::Import => IoEntryDirection::Import,\n        };\n        let row = IoEntryDescriptor { owner: route.native, counterpart: route.foreign, direction };\n        if !io_entries.contains(&row) {\n            io_entries.push(row);\n        }\n    }\n    (io_entries, composer_entries)\n}\n",
        ),
    ],
    DESCRIBE_TEST: [
        (
            "use super::*;\n",
            "use super::*;\n\n/// 🛣️ LAW: a package with one `io_mechanism` serializer and one deserializer of its own artifact describes exactly those two io rows in\n/// registry order — the deserializer as an import into its native dialect, the serializer as an export out of it.\n#[semio_framework_async_macros::async_test]\nasync fn package_descriptor_lists_its_io_mechanism_rows_with_their_native_side() {\n    use semio_framework::io::io_mechanism::{io_register, IoEntry, IoEntryDirection as Side};\n    use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};\n    const NATIVE: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: \"s.describe-io.native\", standard: semio_framework::StandardId(\"1\"), subset: semio_framework::SubsetId(\"*\") };\n    const FOREIGN: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: \"s.describe-io-foreign.format\", standard: semio_framework::StandardId(\"1\"), subset: semio_framework::SubsetId(\"*\") };\n    #[allow(clippy::unnecessary_wraps, reason = \"IoEntry test doubles implement its fallible function-pointer contract\")]\n    fn passthrough(payload: &IoPayload) -> IoResult<IoPayload> {\n        Ok(IoOutcome::clean(payload.clone()))\n    }\n    static ENTRIES: [IoEntry; 2] = [\n        IoEntry { from: NATIVE, into: FOREIGN, fidelity: IoFidelity::Lossy, direction: Side::Export, sniff: None, run: passthrough },\n        IoEntry { from: FOREIGN, into: NATIVE, fidelity: IoFidelity::Lossy, direction: Side::Import, sniff: None, run: passthrough },\n    ];\n    io_register(&ENTRIES).expect(\"the entries register\");\n    let plugin = crate::app::Plugin::<crate::app::NoPluginApp>::builder(\"describe-io\").label(\"Describe Io\").version(\"0.1.0\").package_id(\"semio:describe-io\").try_build().expect(\"the plugin assembles\");\n    let runtime = crate::plugin_runtime::PluginRuntime::new();\n    crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);\n    let value = store::pack_rt::decode_wire_value(&describe_plugin(&runtime).await).expect(\"descriptor wire decodes\");\n    let descriptor: PackageDescriptor = serde_json::from_value(value.into()).expect(\"descriptor shape decodes\");\n    let (native, foreign) = (semio_framework::ArtifactDialect::from(NATIVE), semio_framework::ArtifactDialect::from(FOREIGN));\n    assert_eq!(\n        descriptor.contributions.io_entries,\n        vec![\n            IoEntryDescriptor { owner: native.clone(), counterpart: foreign.clone(), direction: IoEntryDirection::Import },\n            IoEntryDescriptor { owner: native, counterpart: foreign, direction: IoEntryDirection::Export },\n        ]\n    );\n}\n",
        ),
    ],
}

MARKERS = {IO: "pub fn io_native_routes()", LAWS: "fn entry_constructors_declare_their_native_side", HOST: "direction: IoEntryDirection::Export, sniff: None, run }];", DESCRIBE: "io::io_mechanism::io_native_routes()", DESCRIBE_TEST: "fn package_descriptor_lists_its_io_mechanism_rows_with_their_native_side"}


def literal_direction(text: str) -> tuple[str, int]:
    return re.subn(r"(IoEntry \{ from: [A-Z_]+, into: [A-Z_]+, fidelity: IoFidelity::\w+,) (sniff: )", r"\1 direction: IoEntryDirection::Export, \2", text)


def main() -> None:
    apply = "--apply" in sys.argv
    staged = {}
    for path, edits in EDITS.items():
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            before = handle.read()
        if MARKERS[path] in before:
            print(f"SKIP {path}")
            continue
        after = before
        for edit in edits:
            old, new = edit[0], edit[1]
            expected = edit[2] if len(edit) > 2 else 1
            if after.count(old) != expected:
                sys.exit(f"ANCHOR {path}: {after.count(old)} != {expected} for {old[:80]!r}")
            after = after.replace(old, new)
        if path == LAWS:
            after, count = literal_direction(after)
            if count != 19:
                sys.exit(f"LAWS: {count} IoEntry literals, expected 19")
        staged[path] = (before, after)
    for path, (before, after) in staged.items():
        if not apply:
            print(f"WOULD {path} (+{after.count(chr(10)) - before.count(chr(10))} lines)")
            continue
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            if handle.read() != before:
                sys.exit(f"RACE {path}")
    if apply:
        for path, (_, after) in staged.items():
            with open(f"{ROOT}/{path}", "w", encoding="utf-8") as handle:
                handle.write(after)
            print(f"WROTE {path}")


if __name__ == "__main__":
    main()
