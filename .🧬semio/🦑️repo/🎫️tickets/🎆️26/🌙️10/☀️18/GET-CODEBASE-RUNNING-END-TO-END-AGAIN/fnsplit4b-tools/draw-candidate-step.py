import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🪜️candidate-step/🦀️.rs"
rows = [
 (29, "step_preflight_mutation", "Digests the mutation and admits its arena capacities."),
 (96, "step_locate_primary", "Locates the primary layer node of the mutation."),
 (137, "step_prepare_owned_value", "Prepares the owned value the mutation carries."),
 (270, "step_apply", "Applies the mutation to the candidate snapshot."),
 (433, "step_rebuild_source", "Rebuilds the source container index."),
 (471, "step_rebuild_destination", "Rebuilds the destination container index."),
]
armspec.build(F, "step", 20, rows, "Result<bool, &'static str>", method={"self_param": "&mut self", "call": "self.{name}({args})"},
  types={"mutation": "&DrawingMutation", "access": "&mut DrawingSourceAccess<'_, '_>"}, out=sys.argv[1], argexpr={"access": "&mut access"})
