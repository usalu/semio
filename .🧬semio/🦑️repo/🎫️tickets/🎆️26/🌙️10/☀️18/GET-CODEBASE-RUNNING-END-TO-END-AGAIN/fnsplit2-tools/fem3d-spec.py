import json, re, sys
V = "️"
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🪜️solve-step/🍓️staged-step/🦀️.rs"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
g = {
 "mesh": child("🏗", "mesh-and-assembly-stages", "Mesh and assembly stages: the field count, assembly mounting and build, the assembly steps, the modal mass and the equation map of the staged numerical step."),
 "loads": child("⚖", "load-application-stages", "Load application stages: the right hand side, the source cases and the nodal, member, area and self weight load stages of the staged numerical step."),
 "solve": child("🧮", "solve-and-recovery-stages", "Solve and recovery stages: the CSR build, the PCG solve and the node scalar, reaction and stress recovery stages of the staged numerical step."),
 "modal": child("🎼", "modal-and-publication-stages", "Modal and publication stages: the modal, LDLT and subspace solves and the mode scalar and progress publication stages of the staged numerical step."),
}
rows = [
 (146, "PublishFieldCount", "mesh", "🔢", "Publishes the analysis node field count."),
 (152, "MountAssembly", "mesh", "🧷", "Mounts the assembly build."),
 (157, "PrepareAssembly", "mesh", "🧰", "Advances the funded assembly build until the assembly is complete."),
 (171, "Assembly", "mesh", "🔩", "Steps the assembly child to its end."),
 (181, "ReserveModalMass", "mesh", "🏋", "Reserves the modal mass storage."),
 (186, "InitializeModalMass", "mesh", "🧊", "Initializes the modal mass storage."),
 (195, "MapEquation", "mesh", "🗺", "Maps the equation numbers of the free degrees of freedom."),
 (213, "ReserveRhs", "loads", "📦", "Reserves the right hand side storage."),
 (218, "InitializeRhs", "loads", "🧹", "Initializes the right hand side storage."),
 (226, "ResolveSourceCase", "loads", "🎯", "Resolves the load case that feeds the right hand side."),
 (246, "ApplyLoad", "loads", "⬇", "Dispatches the next load of the case to its application stage."),
 (273, "ApplyNodalLoad", "loads", "📍", "Applies one nodal load."),
 (289, "ResolveMemberLoad", "loads", "📏", "Resolves one member load."),
 (303, "ResolveMemberNode", "loads", "🔗", "Resolves one node of a member load."),
 (323, "ApplyMemberScalar", "loads", "🪢", "Applies one scalar of a member load."),
 (353, "ResolveAreaLoad", "loads", "🟦", "Resolves one area load."),
 (368, "ApplyAreaNode", "loads", "🧱", "Applies one node of an area load."),
 (401, "ResolveSelfWeightMemberNode", "loads", "⚓", "Resolves one node of a member self weight."),
 (430, "ApplySelfWeightMember", "loads", "🪨", "Applies the self weight of one member."),
 (445, "ApplySelfWeightSolid", "loads", "🏛", "Applies the self weight of one solid element."),
 (506, "BuildCsr", "solve", "🗜", "Builds the CSR matrix of the assembled system."),
 (516, "BeginPcg", "solve", "🚦", "Begins the preconditioned conjugate gradient solve."),
 (531, "PreparePcg", "solve", "🔧", "Prepares the preconditioned conjugate gradient solve."),
 (542, "Pcg", "solve", "🔁", "Steps the preconditioned conjugate gradient solve."),
 (551, "ReadNodeScalar", "solve", "📖", "Reads one node scalar of the solution."),
 (568, "RecoverReaction", "solve", "↩", "Recovers one support reaction."),
 (588, "PublishNodeScalar", "solve", "📤", "Publishes one node scalar."),
 (595, "RecoverStressElement", "solve", "💥", "Recovers the stress of one element."),
 (618, "PublishStressScalar", "solve", "📡", "Publishes one stress scalar."),
 (629, "BeginModal", "modal", "🎵", "Begins the modal analysis."),
 (634, "PrepareModal", "modal", "🎶", "Prepares the modal analysis."),
 (641, "BeginLdlt", "modal", "🪜", "Begins the LDLT factorization."),
 (653, "Ldlt", "modal", "🔨", "Steps the LDLT factorization."),
 (660, "BeginSubspace", "modal", "🌌", "Begins the subspace iteration."),
 (668, "Subspace", "modal", "🛰", "Steps the subspace iteration."),
 (677, "ReadModeScalar", "modal", "📚", "Reads one mode scalar."),
 (692, "PublishModeScalar", "modal", "🎙", "Publishes one mode scalar."),
 (702, "PublishProgress", "modal", "🏁", "Publishes the final progress and completes the step."),
]
def snake(s):
    return re.sub(r"(?<!^)([A-Z])", lambda m: "_" + m.group(1), s).lower()
used = set(); arms = []
for line, stage, grp, emoji, text in rows:
    assert emoji not in used, (emoji, stage)
    used.add(emoji)
    arms.append({"line": line, "name": "step_" + snake(stage), "doc": "%s %s" % (emoji, text), "child": g[grp], "allow_returns": True, "binds": {}})
spec = {
    "file": F, "function": "step", "match_line": 145, 
    "types": {"doc": "&Fem3dSnapshot", "solver": "&mut Fem3dSolverView", "backing": "&mut Fem3dBackingCredit", "freshness": "Fem3dVisualFreshness", "operation": "semio_framework_job::Operation", "context": "&mut StepContext<'_>"},
    "ret": "Result<bool, Fem3dNumericalFailure>", "fall_tail": "Ok(self.stage == Fem3dNumericalStage::Complete)",
    "method": {"self_param": "&mut self", "impl_header": "impl Fem3dNumericalChild", "call": "return self.{name}({args})"},
    "arms": arms,
}
json.dump(spec, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
