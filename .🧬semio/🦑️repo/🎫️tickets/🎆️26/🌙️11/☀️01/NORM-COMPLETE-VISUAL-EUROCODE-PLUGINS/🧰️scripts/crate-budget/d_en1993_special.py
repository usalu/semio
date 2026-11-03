"""🪚 EN 1993 checks-special: frame, storey, restraint, section, built-up, weld, parametric and hollow-joint checks become `-checks-special-frames`."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from d_lib import *
ART = NORM + "/🗿️artifacts/🔩️en1993"
C = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
P = "semio-s-artifact-norm-en1993-checks-special"
N = P + "-frames"
SRC = C + "/🔗️special-element-checks"
root = SRC + "/🦀️.rs"
s = rd(root)
MODS = [("🕸️frame-checks", "frame_checks", False, "check_frames"), ("🌐️storey-checks", "storey_checks", False, "check_storeys"), ("🧷️restraint-checks", "restraint_checks", True, "check_restraints"), ("🥫️thin-walled-section-checks", "thin_walled_section_checks", True, "check_cold_formed_section"), ("🪜️built-up-checks", "built_up_checks", False, "check_built_up_members"), ("🧵️weld-group-checks", "weld_group_checks", False, "check_weld_groups"), ("📐️parametric-checks", "parametric_checks", False, "check_parametric_sections"), ("🕳️hollow-joint-checks", "hollow_joint_checks", False, "check_hollow_joints")]
if "frame_checks;" in s:
    lib = '''//! 🕸️ EN 1993 special frame checks: second-order frames, storeys, lateral restraints, thin-walled and built-up sections, weld groups, parametric sections and hollow joints.

#![allow(async_fn_in_trait)]
#![allow(unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1993_model::{LoadCase, SteelJoint, SteelMaterial, SteelMember, SteelSection};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub use semio_s_artifact_norm_en1993_checks_core::*;
pub use semio_s_artifact_norm_en1993_frame_eigen as frame_eigen;

'''
    for d, m, pub, fn in MODS:
        f = SRC + "/" + d + "/🦀️.rs"
        t = rd(f)
        t = t.replace("pub(super) fn " + fn, "pub fn " + fn)
        wr(f, t)
        blk = re.search(r'#\[path = "%s/🦀️.rs"\]\n(?:pub )?mod %s;\n(?:pub )?use %s::[^\n]*\n(?:pub use %s::[^\n]*\n)?\n?' % (re.escape(d), m, m, m), s)
        assert blk, m
        s = s.replace(blk.group(0), "")
        lib += '#[path = "../%s/%s/🦀️.rs"]\npub mod %s;\npub use %s::%s;\n' % ("🔗️special-element-checks", d, m, m, fn)
        if m == "restraint_checks":
            lib += "pub use restraint_checks::{restraint_summary, RestraintSummary};\n"
        lib += "\n"
    s = s.replace("pub use semio_s_artifact_norm_en1993_clauses_annex as clauses_annex;\n", "pub use semio_s_artifact_norm_en1993_clauses_annex as clauses_annex;\npub use %s::*;\n" % N.replace("-", "_"), 1)
    s = s.replace("pub use semio_s_artifact_norm_en1993_frame_eigen as frame_eigen;\n", "", 1)
    toml = C + "/🔗️special/📦️packages/🦀️rust/Cargo.toml"
    deps = ["semio-framework-os-kernel", "semio-framework-schema", "semio-framework-value", "semio-framework-value-derive", P.replace("special", "core"), "semio-s-artifact-norm-en1993-clauses", "semio-s-artifact-norm-en1993-frame-eigen", "semio-s-artifact-norm-en1993-model", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-mechanics"]
    new_crate(ART, C + "/🕸️special-frames", N, "EN 1993 frame, storey, restraint, section and weld checks", deps, P, toml)
    add_dep(toml, N)
    wr(C + "/🕸️special-frames/🦀️.rs", lib.rstrip() + "\n")
    wr(root, s)
