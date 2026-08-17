#!/usr/bin/env python3
"""Emit register_all_artifact_schema_descriptors() body for framework-schema tests."""
from pathlib import Path

calls = [
    ("semio_s_plugin_animate", "semio_s_plugin_animate::artifacts::present::engine::register_artifact_schema"),
    ("semio_s_plugin_architect", "semio_s_plugin_architect::artifacts::program::engine::register_artifact_schema"),
    ("semio_s_plugin_block", "semio_s_plugin_block::artifacts::block2d::engine::register_artifact_schema"),
    ("semio_s_plugin_block", "semio_s_plugin_block::artifacts::block3d::engine::register_artifact_schema"),
    ("semio_s_plugin_block", "semio_s_plugin_block::artifacts::block5d::engine::register_artifact_schema"),
    ("semio_s_plugin_cad", "semio_s_plugin_cad::artifacts::cad::engine::register_artifact_schema"),
    ("semio_s_plugin_dag", "semio_s_plugin_dag::artifacts::dag::engine::register_artifact_schema"),
    ("semio_s_plugin_demonstrator", "semio_s_plugin_demonstrator::artifacts::playground::engine::register_artifact_schema"),
    ("semio_s_plugin_draw", "semio_s_plugin_draw::artifacts::draw::engine::register_artifact_schema"),
    ("semio_s_plugin_energy", "semio_s_plugin_energy::artifacts::model::engine::register_artifact_schema"),
    ("semio_s_plugin_fem", "semio_s_plugin_fem::artifacts::fem2d::engine::register_artifact_schema"),
    ("semio_s_plugin_fem", "semio_s_plugin_fem::artifacts::fem3d::engine::register_artifact_schema"),
    ("semio_s_plugin_flow", "semio_s_plugin_flow::artifacts::flow::engine::register_artifact_schema"),
    ("semio_s_plugin_forms", "semio_s_plugin_forms::artifacts::forms::engine::register_artifact_schema"),
    ("semio_s_plugin_gis", "semio_s_plugin_gis::artifacts::gisterrain::engine::register_artifact_schema"),
    ("semio_s_plugin_gis", "semio_s_plugin_gis::artifacts::gismap::engine::register_artifact_schema"),
    ("semio_s_plugin_imperative", "semio_s_plugin_imperative::artifacts::imperative::engine::register_artifact_schema"),
    ("semio_s_plugin_layout", "semio_s_plugin_layout::artifacts::layout::engine::register_artifact_schema"),
    ("semio_s_plugin_lowpoly", "semio_s_plugin_lowpoly::artifacts::lowpoly::engine::register_artifact_schema"),
    ("semio_s_plugin_mathematical", "semio_s_plugin_mathematical::artifacts::mathematical::engine::register_artifact_schema"),
    ("semio_s_plugin_note", "semio_s_plugin_note::artifacts::note::engine::register_artifact_schema"),
    ("semio_s_plugin_playbook", "semio_s_plugin_playbook::artifacts::playbook::engine::register_artifact_schema"),
    ("semio_s_plugin_procedural", "semio_s_plugin_procedural::artifacts::procedural2d::engine::register_artifact_schema"),
    ("semio_s_plugin_procedural", "semio_s_plugin_procedural::artifacts::procedural3d::engine::register_artifact_schema"),
    ("semio_s_plugin_process", "semio_s_plugin_process::artifacts::process3d::engine::register_artifact_schema"),
    ("semio_s_plugin_puzzle", "semio_s_plugin_puzzle::artifacts::puzzle2d::engine::register_artifact_schemas"),
    ("semio_s_plugin_raster", "semio_s_plugin_raster::artifacts::raster::engine::register_artifact_schema"),
    ("semio_s_plugin_reasoning", "semio_s_plugin_reasoning::artifacts::wires::engine::register_artifact_schema"),
    ("semio_s_plugin_remodel", "semio_s_plugin_remodel::artifacts::remodel::engine::register_artifact_schema"),
    ("semio_s_plugin_sequence", "semio_s_plugin_sequence::artifacts::sequence::engine::register_artifact_schema"),
    ("semio_s_plugin_shooting", "semio_s_plugin_shooting::artifacts::shooting::engine::register_artifact_schema"),
    ("semio_s_plugin_sourcing", "semio_s_plugin_sourcing::artifacts::curate::engine::register_artifact_schema"),
    ("semio_s_plugin_space", "semio_s_plugin_space::artifacts::home::engine::register_artifact_schema"),
    ("semio_s_plugin_trinity", "semio_s_plugin_trinity::artifacts::jack::engine::register_artifact_schema"),
    ("semio_s_plugin_trinity", "semio_s_plugin_trinity::artifacts::rewrite::engine::register_artifact_schema"),
    ("semio_s_plugin_vcs", "semio_s_plugin_vcs::artifacts::vcs::engine::register_artifact_schema"),
    ("semio_s_plugin_writer", "semio_s_plugin_writer::artifacts::writer::engine::register_artifact_schema"),
]
norm = [
    "din4108", "din16798", "din18599", "en1990", "en1991", "en1992", "en1993", "en1994",
    "en1995", "en1996", "en1997", "en1998", "en1999", "iso16757", "vdi3805",
]
for key in norm:
    calls.append(
        (
            "semio_s_plugin_norm",
            f"semio_s_plugin_norm::artifacts::{key}::engine::register_artifact_schema",
        )
    )

lines = ["fn register_all_plugin_artifact_schema_descriptors() {"]
for _, call in calls:
    lines.append(f"    {call}();")
lines.append("}")
print("\n".join(lines))
print(f"// total calls: {len(calls)}", file=__import__("sys").stderr)
