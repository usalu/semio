#!/usr/bin/env python3
"""Inline vcompute, geom, types, host into lib.rs (single crate file)."""
from __future__ import annotations

from pathlib import Path


def strip_docs(text: str) -> str:
	lines = text.splitlines()
	while lines and (lines[0].startswith("//!") or lines[0].strip() == ""):
		lines = lines[1:]
	return "\n".join(lines)


def indent_block(text: str, prefix: str = "\t") -> str:
	out = []
	for line in text.splitlines():
		if line.strip():
			out.append(prefix + line)
		else:
			out.append("")
	return "\n".join(out)


def main() -> None:
	root = Path(__file__).resolve().parent.parent
	vcompute = strip_docs((root / "vcompute.rs").read_text(encoding="utf-8"))
	geom = strip_docs((root / "host" / "geom.rs").read_text(encoding="utf-8"))
	types = strip_docs((root / "host" / "types.rs").read_text(encoding="utf-8"))
	host_full = (root / "host" / "mod.rs").read_text(encoding="utf-8")
	host_lines = host_full.splitlines()
	# strip first 16 lines (module docs + mod geom/types + uses)
	host_body = "\n".join(host_lines[16:])
	host_body = host_body.replace(
		"use crate::vcompute::{\n\tcompute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,\n\thandle_position_on_rectangle,\n};",
		"use super::vcompute::{\n\tcompute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,\n\thandle_position_on_rectangle,\n};",
	)
	host_body = host_body.replace(
		"use geom::{\n\tpoint_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon,\n\tsegment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points,\n\tworld_boxes_overlap, inflate_world_box, cubic_bezier_point, WorldBox,\n};",
		"use super::geom_sel::{\n\tpoint_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon,\n\tsegment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points,\n\tworld_boxes_overlap, inflate_world_box, cubic_bezier_point, WorldBox,\n};",
	)
	host_wrapped = "use super::scene_json::*;\n" + host_body
	host_indented = indent_block(host_wrapped)

	board_block = (
		"mod vcompute {\n"
		+ indent_block(vcompute)
		+ "\n}\n\n"
		"mod geom_sel {\n"
		+ indent_block(geom)
		+ "\n}\n\n"
		"mod scene_json {\n"
		+ indent_block(types)
		+ "\n}\n\n"
		"pub use scene_json::{CameraJson, EdgeDescJson, FixtureV1Json, HandleDescJson, NodeDescJson, SceneDescriptorJson};\n\n"
		"mod board_host {\n"
		+ host_indented
		+ "\n}\n\n"
		"pub use board_host::BoardHost;\n"
	)

	lib_path = root / "lib.rs"
	lib = lib_path.read_text(encoding="utf-8")
	old = (
		"mod host;\n"
		"mod vcompute;\n\n"
		"pub use host::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson};\n\n"
	)
	if old not in lib:
		raise SystemExit("expected header block not found")
	new_lib = lib.replace(old, board_block + "\n", 1)
	lib_path.write_text(new_lib, encoding="utf-8")
	print("OK", lib_path)


if __name__ == "__main__":
	main()
