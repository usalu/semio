import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"
phspec.build(F, "check_full_actions", [
 ("check_imposed_and_fire_actions", "Checks the self weight, imposed and fire actions of the full actions check.", 162, 262),
 ("check_wind_thermal_and_accidental_actions", "Checks the wind, thermal, construction and accidental actions of the full actions check.", 265, 351),
 ("check_traffic_and_crane_actions", "Checks the bridge traffic, crane and silo actions of the full actions check.", 366, 496),
], types={"annex": "semio_s_plugin_norm_compliance::AnnexChoice", "report": "CheckReport"}, modes={"annex": "val", "report": "mut"}, keep=[159, 160, 161, 518], out=sys.argv[1])
