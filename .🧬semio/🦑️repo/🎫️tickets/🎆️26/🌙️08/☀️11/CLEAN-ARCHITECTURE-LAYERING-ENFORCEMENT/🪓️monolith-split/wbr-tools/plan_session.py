"""session: lift the retained-input capture and the run job of the energy simulation session root into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

C = lambda **k: k

SPEC = dict(
    target="session",
    imports=(11, 36),
    root_drop=["EnergyAdmissionRejected", "EnergyJob", "EnergyJobCursor", "EnergyNumericalBounds", "JobPublicationKind", "RetainedJobPublication", "InteractiveJob", "JobOutcomeDescriptor", "JobOutcomeKind", "JobOutcomeView", "JobPayloadStream", "RetainedCloneStep", "RetainedJobPayload", "ToolRunCounter", "ToolRunProgress", "ToolRunStepRing", "ValueError", "ValueRefusalKind"],
    skip_import=[],
    root_cfg={n: ["#[cfg(test)]"] for n in ["Generation", "InteractiveJobCloseStep", "JobOutcomeBorrow", "OperationId", "RetainedCloneGrant", "RetainedCloneProgress", "StepBudget", "StepContext", "RetirementDemand", "ToolRunIdentity", "ToolRunState", "ToolRunStepArg", "ToolRunStepKind", "ToolRunTickWriter", "EnergyModelSnapshot", "Arc", "EnergyModelCloseCursor"]},
    reexport_cfg={"EnergyRunCursor": ["#[cfg(test)]"]},
    children=[
        C(kind="cluster", mod="retained_input", folder="🧮️retained-input", ranges=[(302, 1014)],
          doc=["//! 🧮️ Retained input: the bounded census of the base model and its capture in grants."]),
        C(kind="cluster", mod="run_job", folder="🧵️run-job", ranges=[(1016, 1612)],
          doc=["//! 🧵️ Run job: the interactive job that drives the numerical `EnergyJob` inline and publishes progress ticks."]),
    ],
)


def anchors():
    ls = lines_of(read(snap_dir("session") + "/" + TARGETS["session"]["file"]))
    for ln, text in [(302, "//#region 🧮️RetainedInput"), (1014, "//#endregion 🧮️RetainedInput"), (1016, "//#region 🧵️RunJob"), (1612, "//#endregion 🧵️RunJob")]:
        assert ls[ln - 1].startswith(text), (ln, ls[ln - 1])
    assert ls[10].startswith("use semio_framework_job_publication"), ls[10]


def main():
    anchors()
    plan = Plan(SPEC).build()
    root = plan.out[plan.t["file"]]
    plan.out[plan.t["file"]] = root.replace("\n\n\nextern crate", "\n\nextern crate")
    stage(plan)
    gone, added = multiset_report(plan)
    print("files:", {k.split("/")[-2] if k.endswith("🦀️.rs") and "/" in k else k: len(lines_of(v)) for k, v in plan.out.items()})
    print("REMOVED:", sum(gone.values()), json.dumps(dict(gone), ensure_ascii=False)[:2500])
    print("ADDED:", sum(added.values()), json.dumps(dict(added), ensure_ascii=False)[:4500])
    for m, r in plan.report["children"].items():
        print(m, "widened", len(r["widened"]), r["widened"][:200])
        print("   imports", r["imports"])
        print("   super", r["super_imports"])
        print("   pub", r["reexport_pub"])
        print("   priv", r["reexport_priv"])


if __name__ == "__main__":
    main()
