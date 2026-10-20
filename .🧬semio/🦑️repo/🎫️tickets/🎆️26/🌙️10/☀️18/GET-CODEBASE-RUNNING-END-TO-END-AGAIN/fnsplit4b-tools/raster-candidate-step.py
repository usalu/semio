import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🧩️mutation-candidate/🦀️.rs"
rows = [
 (138, "step_clone", "Clones the candidate snapshot."),
 (163, "step_locate_primary", "Locates the primary layer of the mutation."),
 (188, "step_locate_secondary", "Locates the secondary or destination layer of the mutation."),
 (211, "step_prepare_layer", "Prepares the layer the mutation carries."),
 (232, "step_apply", "Applies the mutation to the candidate snapshot."),
 (398, "step_prepare_asset", "Prepares the asset the mutation carries."),
 (489, "step_shift_remove", "Shifts the layers behind a removed one."),
]
armspec.build(F, "step", 137, rows, "Result<bool, &'static str>", method={"self_param": "&mut self", "call": "self.{name}({args})"}, out=sys.argv[1])
