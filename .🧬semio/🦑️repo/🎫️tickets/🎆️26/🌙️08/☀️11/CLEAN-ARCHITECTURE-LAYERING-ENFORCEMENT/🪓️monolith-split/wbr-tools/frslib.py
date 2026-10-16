"""Shared helpers of the frs (FRAMEWORK-RUST-FILE-SPLITS) tools: paths, coverage-checked planning, staging, multiset proofs."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/wbr-work"

TARGETS = {
    "session": dict(
        file="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs",
        tree="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session",
        lib="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs",
        crate="semio_s_artifact_energy_model_session", pkg="semio-s-artifact-energy-model-session",
        cfgs=["lib", "test", "wasm", "wasmtest"], features=[], walk=True,
        alias=["kernel", "model", "results", "schedule", "sim", "sim_retirement", "sim_wire", "snapshot", "examples"],
        ext=["semio_framework_job_publication", "semio_framework_job", "semio_framework_value", "semio_framework_ui_locale", "semio_framework_tool_run",
             "kernel", "model", "results", "schedule", "sim", "sim_retirement", "sim_wire", "snapshot", "examples", "serde_json"]),
    "viewer": dict(
        file="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        tree="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer",
        lib="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        crate="semio_s_artifact_energy_model_viewer", pkg="semio-s-artifact-energy-model-viewer",
        cfgs=["lib", "test", "wasm", "wasmtest"], features=[], walk=True,
        alias=["dsl", "protocol", "store", "mutations", "scene", "snapshot", "examples"],
        ext=["semio_framework", "semio_framework_plugin", "semio_framework_ui_locale", "semio_framework_value_derive", "semio_framework_value", "semio_framework_artifact_reference",
             "semio_framework_dsl_record", "semio_framework_dsl_record_derive", "semio_framework_pack_json", "semio_framework_2d", "semio_framework_job", "semio_framework_async_macros",
             "semio_framework_composition_admission", "semio_s_artifact_stdio_semio", "semio_framework_diagnostic",
             "dsl", "protocol", "store", "mutations", "scene", "snapshot", "examples", "serde_json"]),
    "vectors": dict(
        file="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs",
        tree="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors",
        lib="✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs",
        crate="semio_s_artifact_remodel_remodeling_mut_vectors", pkg="semio-s-artifact-remodel-remodeling-mut-vectors",
        cfgs=["lib", "test", "wasm", "wasmtest"], features=[],
        traits=["Mutation", "ToValue", "FromValue"],
        ext=["protocol", "semio_framework_os_kernel", "semio_framework_pack_json", "semio_framework_value", "semio_framework_diagnostic",
             "semio_s_artifact_remodel_remodeling_model", "semio_s_artifact_remodel_remodeling_model_diff", "semio_s_artifact_remodel_remodeling_mutations"]),
}


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def lines_of(text):
    """Lines without terminators; a trailing newline yields no extra empty element."""
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def dedent(ls, n=4):
    out = []
    for l in ls:
        k = 0
        while k < n and k < len(l) and l[k] == " ":
            k += 1
        out.append(l[k:])
    return out


class Coverage:
    """Every original line (1-based) must be claimed exactly once."""

    def __init__(self, total):
        self.total = total
        self.owner = [None] * (total + 1)

    def claim(self, who, a, b):
        for i in range(a, b + 1):
            assert self.owner[i] is None, f"line {i} claimed by {self.owner[i]} and {who}"
            self.owner[i] = who

    def check(self):
        missing = [i for i in range(1, self.total + 1) if self.owner[i] is None]
        assert not missing, f"unclaimed lines {missing[:20]} ({len(missing)})"


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


def diff_multiset(orig_ls, new_ls):
    a, b = multiset(orig_ls), multiset(new_ls)
    return a - b, b - a


def stage_dir(name):
    return f"{W}/{name}/stage"


def snap_dir(name):
    return f"{W}/{name}/snap"
