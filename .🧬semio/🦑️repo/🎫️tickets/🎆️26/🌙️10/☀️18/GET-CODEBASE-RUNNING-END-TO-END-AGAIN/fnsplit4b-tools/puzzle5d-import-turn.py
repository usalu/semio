import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/📥️snapshot-import/🦀️.rs"
rows = [
 (213, "turn_decode", "Decodes one record of the imported snapshot."),
 (257, "turn_census_parts", "Counts the parts of the imported snapshot."),
 (336, "turn_reserve_mutations", "Reserves the mutation slots of the import."),
 (450, "turn_parts", "Builds one part of the imported snapshot."),
 (490, "turn_part_vortices", "Builds the vortices of one part."),
 (546, "turn_grips", "Builds the grips of the imported snapshot."),
 (577, "turn_compatibility", "Checks the compatibility of the imported snapshot."),
 (621, "turn_complete", "Completes the import and publishes the result."),
]
armspec.build(F, "turn", 206, rows, "crate::puzzle_job::JobTurn", method={"self_param": "&mut self", "call": "self.{name}({args})"}, out=sys.argv[1])
