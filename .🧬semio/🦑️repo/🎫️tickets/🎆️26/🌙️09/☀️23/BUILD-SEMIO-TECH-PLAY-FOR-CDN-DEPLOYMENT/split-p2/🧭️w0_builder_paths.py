#!/usr/bin/env python3
"""🏗️ Wave 0: after the mutation leaf re-exports left the dispatch module, `…::schema::mutations::<leaf>` names only the
leaf MODULE. Every import or call that meant the leaf's builder function is pointed at `…::mutations::<leaf>::<leaf>`
(lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
LEAVES = "create_stream delete_stream change_stream_sync add_stream_frame remove_stream_frame replace_stream_source create_asset delete_asset create_camera_calibration update_camera_calibration delete_camera_calibration create_rig_extrinsic delete_rig_extrinsic update_rig_extrinsic create_gcp delete_gcp add_gcp_observation remove_gcp_observation update_ingest_params update_feature_params update_match_params update_sfm_params update_dense_params update_mesh_params update_motion_params update_geo_params append_content remove_content commit_reconstruction replace_sparse replace_dense replace_mesh_result replace_trajectory replace_tracks replace_geo_products replace_qc".split()
USE = re.compile(r"(?ms)^([ \t]*(?:pub(?:\([^)]*\))?\s+)?use\s+)([^;]*schema::mutations::(?:\{[^;]*\}|[a-z_]+))(\s*;)")
changed = 0
for dirpath, _, files in os.walk(ART):
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        body = USE.sub("", text)
        def fix_import(m):
            head, path_text, tail = m.groups()
            def leaf_spec(leaf):
                fn = re.search(r"(?<![\w:])" + leaf + r"\s*\(", body) is not None
                module = re.search(r"(?<![\w:])" + leaf + r"\s*::", body) is not None
                return f"{leaf}::{{self, {leaf}}}" if fn and module else (f"{leaf}::{leaf}" if fn else leaf)
            g = re.match(r"(?s)(.*schema::mutations::)\{(.*)\}$", path_text)
            if g:
                parts = [p.strip() for p in g.group(2).split(",") if p.strip()]
                parts = [leaf_spec(p) if p in LEAVES else p for p in parts]
                return head + g.group(1) + "{" + ", ".join(parts) + "}" + tail
            g = re.match(r"(?s)(.*schema::mutations::)([a-z_]+)$", path_text)
            if g and g.group(2) in LEAVES:
                return head + g.group(1) + leaf_spec(g.group(2)) + tail
            return m.group(0)
        new = USE.sub(fix_import, text)
        new = re.sub(r"schema::mutations::(" + "|".join(LEAVES) + r")(\s*\()", r"schema::mutations::\1::\1\2", new)
        if new != text:
            open(path, "w", encoding="utf-8").write(new)
            changed += 1
print("files changed", changed)
