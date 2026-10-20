import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🏃️run/🦀️.rs"
rows = [
 (34, "run_resolve_weather", "Copies one weather record into the retained weather table, or finishes the weather resolution and starts the precompute."),
 (73, "run_initialize_zones", "Initializes the zone states of the simulation model."),
 (94, "run_initialize_surfaces", "Initializes the surface states of the simulation model."),
 (158, "run_initialize_warmup_history", "Initializes the warmup history of every zone."),
 (183, "run_warmup_timestep", "Runs one warmup timestep."),
 (222, "run_warmup_convergence", "Checks the warmup convergence and starts the next warmup day or the run."),
 (286, "run_start_run", "Starts the run period after the warmup."),
 (341, "run_zone_timestep", "Runs one zone timestep of the run period."),
 (461, "run_encode_output", "Encodes the retained results into the output pack."),
]
armspec.build(F, "run", 20, rows, "EnergyRun", method={"self_param": "&mut self", "call": "self.{name}({args})"}, out=sys.argv[1])
