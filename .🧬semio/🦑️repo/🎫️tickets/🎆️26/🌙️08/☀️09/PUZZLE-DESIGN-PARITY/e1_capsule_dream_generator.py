#!/usr/bin/env python3
"""🎛 One-off Capsule Dream compose→puzzle DSL generator (ticket-local, Wave 4 / E1)."""

from __future__ import annotations

import json
import math
import re
from pathlib import Path
from typing import Any

#region Paths
REPO = Path("/Users/ueli/Documents/semio")
KIT = REPO / "compose/fixture/kit/dev/metabolism/wip/initialKit"
TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🌙️capsule-dream-out"
#endregion

#region Helpers
def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def items(bucket: Any) -> list[dict]:
    if bucket is None:
        return []
    if isinstance(bucket, list):
        return bucket
    if isinstance(bucket, dict):
        return list(bucket.get("items") or [])
    return []


def dsl_str(value: Any) -> str:
    if value is None:
        return "none"
    text = str(value)
    if text == "":
        return "none"
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_.\-]*", text):
        return text
    return '"' + text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ") + '"'


def dsl_num(value: float | int | None) -> str:
    if value is None:
        return "0"
    if isinstance(value, bool):
        return "true" if value else "false"
    f = float(value)
    if f == 0:
        return "0"
    if abs(f - round(f)) < 1e-12:
        return str(int(round(f)))
    return repr(f)


def dsl_bool(value: bool | None) -> str:
    return "true" if value else "false"


def vec3(obj: dict | None, default: tuple[float, float, float] = (0.0, 0.0, 0.0)) -> tuple[float, float, float]:
    if not obj:
        return default
    return (float(obj.get("x", 0)), float(obj.get("y", 0)), float(obj.get("z", 0)))


def fmt_at(v: tuple[float, float, float]) -> str:
    return f"@{dsl_num(v[0])},{dsl_num(v[1])},{dsl_num(v[2])}"


def fmt_dir(v: tuple[float, float, float]) -> str:
    return f"^{dsl_num(v[0])},{dsl_num(v[1])},{dsl_num(v[2])}"


def cross(a: tuple[float, float, float], b: tuple[float, float, float]) -> tuple[float, float, float]:
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def quat_from_axes(x_axis: tuple[float, float, float], y_axis: tuple[float, float, float]) -> tuple[float, float, float, float]:
    z_axis = cross(x_axis, y_axis)
    m00, m10, m20 = x_axis
    m01, m11, m21 = y_axis
    m02, m12, m22 = z_axis
    trace = m00 + m11 + m22
    if trace > 0.0:
        s = math.sqrt(trace + 1.0) * 2.0
        qw = 0.25 * s
        qx = (m21 - m12) / s
        qy = (m02 - m20) / s
        qz = (m10 - m01) / s
    elif m00 > m11 and m00 > m22:
        s = math.sqrt(1.0 + m00 - m11 - m22) * 2.0
        qw = (m21 - m12) / s
        qx = 0.25 * s
        qy = (m01 + m10) / s
        qz = (m02 + m20) / s
    elif m11 > m22:
        s = math.sqrt(1.0 + m11 - m00 - m22) * 2.0
        qw = (m02 - m20) / s
        qx = (m01 + m10) / s
        qy = 0.25 * s
        qz = (m12 + m21) / s
    else:
        s = math.sqrt(1.0 + m22 - m00 - m11) * 2.0
        qw = (m10 - m01) / s
        qx = (m02 + m20) / s
        qy = (m12 + m21) / s
        qz = 0.25 * s
    return (qx, qy, qz, qw)


def fmt_quat(q: tuple[float, float, float, float]) -> str:
    return f"{dsl_num(q[0])},{dsl_num(q[1])},{dsl_num(q[2])},{dsl_num(q[3])}"


def piece_text(name: str) -> str:
    if "," in name:
        rest = name.split(",", 1)[1].lstrip(",")
        return rest or name
    return name


def icon_kind(type_name: str) -> str:
    slug = re.sub(r"[^A-Za-z0-9]+", "_", type_name).strip("_")
    return slug or "part"


def angle_from_connector(conn: dict) -> float:
    if conn.get("t") is not None:
        return 2.0 * math.pi * float(conn["t"])
    direction = vec3(conn.get("direction"), (0.0, 0.0, 1.0))
    return math.atan2(direction[1], direction[0])


def mime_from_tags(tag_names: list[str]) -> str:
    for name in tag_names:
        if name == "representation/gltf-binary":
            return "model/gltf-binary"
        if name == "representation/vnd.3dm":
            return "model/vnd.3dm"
        if name == "representation/vnd.speckle+json":
            return "application/vnd.speckle+json"
    return "application/octet-stream"


def lod_from_tags(tag_names: list[str]) -> str | None:
    for name in tag_names:
        if name.startswith("lod/"):
            return name.split("/", 1)[1]
        if name in ("1to500", "1to200", "collider"):
            return name
    return None


def mesh_url(file_name: str) -> str:
    return f"/mesh/{file_name}"


#endregion

#region Load
def load_kit_index() -> tuple[dict, dict, dict, dict, dict]:
    kit = load_json(KIT / "kit.compose.json")
    index = load_json(KIT / "index.compose.json")
    dream = load_json(KIT / "design/capsule-dream.design.compose.json")
    flat = load_json(KIT / "design/flat.design.compose.json")

    files = {f["id"]: f for f in items(kit.get("files"))}
    tags = {t["id"]: t["name"] for t in items(kit.get("tags"))}

    ports: dict[str, dict] = {}

    def walk(node: Any) -> None:
        if isinstance(node, dict):
            if "compatiblePorts" in node and "id" in node and "name" in node:
                ports[node["id"]] = node
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(kit)

    types_by_id: dict[str, dict] = {}
    for path in sorted((KIT / "type").glob("*.type.compose.json")):
        type_doc = load_json(path)
        types_by_id[type_doc["id"]] = type_doc

    return kit, index, dream, flat, {
        "files": files,
        "tags": tags,
        "ports": ports,
        "types": types_by_id,
    }


#endregion

#region Catalog builders
def primary_glb(type_doc: dict, files: dict[str, dict], tags: dict[str, str]) -> str | None:
    for rep in items(type_doc.get("representations")):
        file_id = (rep.get("file") or {}).get("id")
        file_doc = files.get(file_id or "")
        if not file_doc:
            continue
        name = file_doc.get("name") or ""
        if not str(name).endswith(".glb"):
            continue
        tag_names = [tags.get(t["id"], t["id"]) for t in items(rep.get("tags"))]
        if "collider" in tag_names:
            continue
        return str(name)
    for rep in items(type_doc.get("representations")):
        file_id = (rep.get("file") or {}).get("id")
        file_doc = files.get(file_id or "")
        if file_doc and str(file_doc.get("name", "")).endswith(".glb"):
            return str(file_doc["name"])
    return None


def build_representations(type_doc: dict, files: dict[str, dict], tags: dict[str, str]) -> str:
    chunks: list[str] = []
    for index, rep in enumerate(items(type_doc.get("representations"))):
        file_id = (rep.get("file") or {}).get("id")
        file_doc = files.get(file_id or "")
        file_name = (file_doc or {}).get("name") or f"missing-{file_id}"
        tag_names = [tags.get(t["id"], t["id"]) for t in items(rep.get("tags"))]
        display_tags = [n for n in tag_names if not n.startswith("representation/")]
        if not display_tags:
            display_tags = ["mesh"] if str(file_name).endswith(".glb") else []
        lod = lod_from_tags(tag_names)
        mime = mime_from_tags(tag_names)
        tag_list = "[" + " ".join(dsl_str(t) for t in display_tags) + "]" if display_tags else "[]"
        lod_tok = dsl_str(lod) if lod else "_"
        chunks.append(
            " ".join(
                [
                    f"id={dsl_str(rep.get('id') or f'rep{index}')}",
                    f"name={dsl_str(rep.get('name') or 'default')}",
                    f"url={dsl_str(mesh_url(str(file_name)))}",
                    f"mime={dsl_str(mime)}",
                    f"tags={tag_list}",
                    f"lod={lod_tok}",
                    "description=none",
                ]
            )
        )
    if not chunks:
        return "[]"
    return "[ " + " ".join(chunks) + " ]"


def connector_name(conn: dict) -> str:
    return conn.get("name") or "link"


def build_grip_templates(type_doc: dict, ports: dict[str, dict]) -> str:
    chunks: list[str] = []
    for conn in items(type_doc.get("connectors")):
        port_id = (conn.get("port") or {}).get("id")
        port = ports.get(port_id or "")
        grip_kind = (port or {}).get("name") or port_id or "grip"
        name = connector_name(conn)
        point = vec3(conn.get("point"))
        direction = vec3(conn.get("direction"), (0.0, 0.0, 1.0))
        fields = [
            f"id={dsl_str(conn['id'])}",
            f"name={dsl_str(name)}",
            f"label={dsl_str(name)}",
            "description=none",
            "icon=none",
            f"grip-kind={dsl_str(grip_kind)}",
            f"point={fmt_at(point)}",
            f"direction={fmt_dir(direction)}",
        ]
        if conn.get("t") is not None:
            fields.append(f"t={dsl_num(conn['t'])}")
        if conn.get("mandatory") is not None:
            fields.append(f"mandatory={dsl_bool(bool(conn['mandatory']))}")
        fields.append("radius=0.36")
        chunks.append(" ".join(fields))
    if not chunks:
        return "[]"
    return "[ " + " ".join(chunks) + " ]"


def build_part_kind_rows(used_types: list[dict], files: dict, tags: dict, ports: dict) -> list[str]:
    rows: list[str] = []
    for type_doc in used_types:
        name = type_doc["name"]
        rows.append(
            "  "
            + " ".join(
                [
                    dsl_str(name),
                    dsl_str(name),
                    dsl_str(name),
                    dsl_str(type_doc.get("description") or None),
                    dsl_str(type_doc.get("icon") or None),
                    dsl_str(type_doc.get("image") or None),
                    dsl_str(type_doc.get("unit") or "m"),
                    dsl_bool(bool(type_doc.get("virtual"))),
                    "[]",
                    build_representations(type_doc, files, tags),
                    build_grip_templates(type_doc, ports),
                    "[]",
                    "[]",
                ]
            )
        )
    return rows


def build_grip_kind_rows(used_port_ids: set[str], ports: dict[str, dict]) -> list[str]:
    rows: list[str] = []
    for port_id in sorted(used_port_ids, key=lambda pid: ports.get(pid, {}).get("name") or pid):
        port = ports.get(port_id)
        if not port:
            continue
        name = port["name"]
        compat_names: list[str] = []
        for ref in items(port.get("compatiblePorts")):
            other = ports.get(ref["id"])
            if other:
                compat_names.append(other["name"])
        compat = "[" + " ".join(dsl_str(n) for n in compat_names) + "]" if compat_names else "[]"
        rows.append(
            "  "
            + " ".join(
                [
                    dsl_str(name),
                    dsl_str(name),
                    dsl_str(name),
                    "0",
                    compat,
                    "none",
                    "none",
                    "none",
                    "_",
                ]
            )
        )
    return rows


def build_compat_rows(ports: dict[str, dict], used_port_ids: set[str]) -> list[str]:
    pairs: set[tuple[str, str]] = set()
    for port_id in used_port_ids:
        port = ports.get(port_id)
        if not port:
            continue
        source = port["name"]
        for ref in items(port.get("compatiblePorts")):
            other = ports.get(ref["id"])
            if not other:
                continue
            pairs.add((source, other["name"]))
    rows = []
    for source, target in sorted(pairs):
        rows.append(f"  {dsl_str(source)} {dsl_str(target)} true false general")
    return rows


#endregion

#region Instance builders
def part_grips_dsl(type_doc: dict, ports: dict[str, dict]) -> str:
    chunks: list[str] = []
    for conn in items(type_doc.get("connectors")):
        port_id = (conn.get("port") or {}).get("id")
        port = ports.get(port_id or "")
        grip_kind = (port or {}).get("name") or port_id or "grip"
        name = connector_name(conn)
        point = vec3(conn.get("point"))
        direction = vec3(conn.get("direction"), (0.0, 0.0, 1.0))
        fields = [
            f"id={dsl_str(conn['id'])}",
            f"name={dsl_str(name)}",
            f"label={dsl_str(name)}",
            f"grip-kind={dsl_str(grip_kind)}",
            f"point={fmt_at(point)}",
            f"direction={fmt_dir(direction)}",
        ]
        if conn.get("t") is not None:
            fields.append(f"t={dsl_num(conn['t'])}")
        if conn.get("mandatory") is not None:
            fields.append(f"mandatory={dsl_bool(bool(conn['mandatory']))}")
        fields.append("radius=0.36")
        chunks.append(" ".join(fields))
    if not chunks:
        return "[]"
    return "[ " + " ".join(chunks) + " ]"


def part_2d_rec(piece: dict, pose: dict | None, type_doc: dict) -> str:
    center = (pose or {}).get("center") or {}
    x = float(center.get("u", 0) if pose else 0)
    y = float(center.get("v", 0) if pose else 0)
    text = piece_text(piece.get("name") or "")
    kind = icon_kind(type_doc["name"])
    return (
        "{"
        + f"x={dsl_num(x)} y={dsl_num(y)} shape=circle radius=20 text={dsl_str(text)} icon-kind={dsl_str(kind)}"
        + "}"
    )


def part_3d_rec(piece: dict, pose: dict | None, type_doc: dict, files: dict, tags: dict) -> str:
    glb = primary_glb(type_doc, files, tags)
    url = mesh_url(glb) if glb else "/mesh/missing.glb"
    if pose and pose.get("plane"):
        plane = pose["plane"]
        origin = vec3(plane.get("origin"))
        x_axis = vec3(plane.get("xAxis"), (1.0, 0.0, 0.0))
        y_axis = vec3(plane.get("yAxis"), (0.0, 1.0, 0.0))
        orientation = quat_from_axes(x_axis, y_axis)
    else:
        origin = (0.0, 0.0, 0.0)
        orientation = (0.0, 0.0, 0.0, 1.0)
    label = f"{type_doc['name']} · {piece_text(piece.get('name') or '')}"
    return (
        "{"
        + f"origin={fmt_at(origin)} mesh-url={dsl_str(url)} orientation={fmt_quat(orientation)} label={dsl_str(label)}"
        + "}"
    )


def build_part_row(
    piece: dict,
    type_doc: dict,
    ports: dict,
    files: dict,
    tags: dict,
) -> str:
    pose = piece.get("pose")
    anchor = "fixed" if pose else "derived"
    return (
        "  "
        + " ".join(
            [
                dsl_str(piece["id"]),
                dsl_str(type_doc["name"]),
                anchor,
                part_2d_rec(piece, pose, type_doc),
                part_3d_rec(piece, pose, type_doc, files, tags),
                part_grips_dsl(type_doc, ports),
            ]
        )
    )


def build_fastener_row(conn: dict) -> str:
    source = f"{conn['parent']['piece']['id']}:{conn['parent']['connector']['id']}"
    target = f"{conn['child']['piece']['id']}:{conn['child']['connector']['id']}"
    return (
        "  "
        + " ".join(
            [
                dsl_str(conn["id"]),
                dsl_str(source),
                dsl_str(target),
                "_",
                dsl_num(conn.get("gap", 0)),
                dsl_num(conn.get("shift", 0)),
                dsl_num(conn.get("rise", 0)),
                dsl_num(conn.get("rotation", 0)),
                dsl_num(conn.get("turn", 0)),
                dsl_num(conn.get("tilt", 0)),
                dsl_num(conn.get("u", 0)),
                dsl_num(conn.get("v", 0)),
            ]
        )
    )


#endregion

#region Projections
def write_5d(
    out_path: Path,
    dream: dict,
    used_types: list[dict],
    used_port_ids: set[str],
    ports: dict,
    files: dict,
    tags: dict,
    types_by_id: dict,
) -> dict[str, int]:
    pieces = items(dream.get("pieces"))
    connections = items(dream.get("connections"))
    lines: list[str] = []
    lines.append("semio puzzle.puzzle5d.dsl v1")
    lines.append('schema=puzzle.5d domain=architecture label="Capsule Dream"')
    lines.append("meta {")
    lines.append(
        '  description="Capsule Dream transferred from compose metabolism kit; 2d and 3d views project from this model."'
    )
    lines.append("}")
    lines.append("kind-catalogs=")
    lines.append(
        "parts [id:TEXT name:TEXT label:TEXT description:TEXT icon:TEXT image:TEXT unit:TEXT is-abstract:BOOL base-kinds:LIST representations:LIST grips:LIST attributes:LIST authors:LIST] {"
    )
    lines.extend(build_part_kind_rows(used_types, files, tags, ports))
    lines.append("}")
    lines.append(
        "grips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF] {"
    )
    lines.extend(build_grip_kind_rows(used_port_ids, ports))
    lines.append("}")
    lines.append("fasteners [id:TEXT name:TEXT label:TEXT] {")
    lines.append("}")
    lines.append("ropes [id:TEXT name:TEXT label:TEXT default-fastener-kind:REF] {")
    lines.append("}")
    lines.append("")
    lines.append(
        "kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT] {"
    )
    lines.extend(build_compat_rows(ports, used_port_ids))
    lines.append("}")
    lines.append(
        "parts [id:TEXT part-kind:REF anchor:TEXT part-2d:REC part-3d:REC grips:LIST] {"
    )
    fixed = derived = 0
    for piece in pieces:
        type_doc = types_by_id[piece["type"]["id"]]
        if piece.get("pose"):
            fixed += 1
        else:
            derived += 1
        lines.append(build_part_row(piece, type_doc, ports, files, tags))
    lines.append("}")
    lines.append(
        "fasteners [id:TEXT source:TEXT target:TEXT fastener-kind:REF gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM] {"
    )
    for conn in connections:
        lines.append(build_fastener_row(conn))
    lines.append("}")
    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return {
        "parts": len(pieces),
        "fasteners": len(connections),
        "part_kinds": len(used_types),
        "grip_kinds": len(used_port_ids),
        "fixed": fixed,
        "derived": derived,
        "bytes": out_path.stat().st_size,
    }


def write_3d(
    out_path: Path,
    dream: dict,
    used_types: list[dict],
    used_port_ids: set[str],
    ports: dict,
    files: dict,
    tags: dict,
    types_by_id: dict,
) -> dict[str, int]:
    pieces = items(dream.get("pieces"))
    connections = items(dream.get("connections"))
    lines: list[str] = []
    lines.append("semio puzzle.puzzle3d.dsl v1")
    lines.append("schema=puzzle.3d.fixture domain=architecture")
    lines.append("meta {")
    lines.append("  kind-catalogs=")
    lines.append(
        "  objects [id:TEXT name:TEXT label:TEXT description:TEXT icon:TEXT image:TEXT unit:TEXT abstract:BOOL base-kinds:LIST representations:LIST vortices:LIST attributes:LIST authors:LIST] {"
    )
    for type_doc in used_types:
        name = type_doc["name"]
        # 3d vortices mirror grip templates with vortex-kind terminology
        vortex_chunks: list[str] = []
        for conn in items(type_doc.get("connectors")):
            port_id = (conn.get("port") or {}).get("id")
            port = ports.get(port_id or "")
            grip_kind = (port or {}).get("name") or port_id or "grip"
            point = vec3(conn.get("point"))
            direction = vec3(conn.get("direction"), (0.0, 0.0, 1.0))
            vortex_chunks.append(
                " ".join(
                    [
                        f"vortex-kind={dsl_str(grip_kind)}",
                        f"point={fmt_at(point)}",
                        f"direction={fmt_dir(direction)}",
                        "radius=0.36",
                    ]
                )
            )
        vortices = "[ " + " ".join(vortex_chunks) + " ]" if vortex_chunks else "[]"
        lines.append(
            "    "
            + " ".join(
                [
                    dsl_str(name),
                    dsl_str(name),
                    dsl_str(name),
                    "none",
                    "none",
                    "none",
                    dsl_str(type_doc.get("unit") or "m"),
                    dsl_bool(bool(type_doc.get("virtual"))),
                    "[]",
                    build_representations(type_doc, files, tags),
                    vortices,
                    "[]",
                    "[]",
                ]
            )
        )
    lines.append("  }")
    lines.append(
        "  vortices [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-cable-kind:REF] {"
    )
    for row in build_grip_kind_rows(used_port_ids, ports):
        lines.append("  " + row)
    lines.append("  }")
    lines.append("  attractions [id:TEXT name:TEXT label:TEXT] {")
    lines.append("  }")
    lines.append("  cables [id:TEXT name:TEXT label:TEXT default-attraction-kind:REF] {")
    lines.append("  }")
    lines.append("}")
    lines.append(
        "kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT] {"
    )
    lines.extend(build_compat_rows(ports, used_port_ids))
    lines.append("}")
    lines.append(
        "objects [id:TEXT label:TEXT object-kind:REF anchor:TEXT origin:CRD orientation:TUPLE scale:LIST mesh-url:TEXT vortices:LIST hidden:BOOL locked:BOOL] {"
    )
    for piece in pieces:
        type_doc = types_by_id[piece["type"]["id"]]
        pose = piece.get("pose")
        anchor = "fixed" if pose else "derived"
        if pose and pose.get("plane"):
            plane = pose["plane"]
            origin = vec3(plane.get("origin"))
            orientation = quat_from_axes(
                vec3(plane.get("xAxis"), (1.0, 0.0, 0.0)),
                vec3(plane.get("yAxis"), (0.0, 1.0, 0.0)),
            )
        else:
            origin = (0.0, 0.0, 0.0)
            orientation = (0.0, 0.0, 0.0, 1.0)
        glb = primary_glb(type_doc, files, tags)
        url = mesh_url(glb) if glb else "/mesh/missing.glb"
        label = f"{type_doc['name']} · {piece_text(piece.get('name') or '')}"
        vortex_chunks = []
        for conn in items(type_doc.get("connectors")):
            port_id = (conn.get("port") or {}).get("id")
            port = ports.get(port_id or "")
            grip_kind = (port or {}).get("name") or port_id or "grip"
            point = vec3(conn.get("point"))
            direction = vec3(conn.get("direction"), (0.0, 0.0, 1.0))
            vortex_chunks.append(
                " ".join(
                    [
                        f'id={dsl_str(conn["id"])}',
                        f"vortex-kind={dsl_str(grip_kind)}",
                        f"point={fmt_at(point)}",
                        f"direction={fmt_dir(direction)}",
                        "radius=0.36",
                    ]
                )
            )
        vortices = "[ " + " ".join(vortex_chunks) + " ]" if vortex_chunks else "[]"
        lines.append(
            "  "
            + " ".join(
                [
                    dsl_str(piece["id"]),
                    dsl_str(label),
                    dsl_str(type_doc["name"]),
                    anchor,
                    fmt_at(origin),
                    fmt_quat(orientation),
                    "[]",
                    dsl_str(url),
                    vortices,
                    dsl_bool(bool(piece.get("isHidden"))),
                    dsl_bool(bool(piece.get("isLocked"))),
                ]
            )
        )
    lines.append("}")
    lines.append(
        "attractions [id:TEXT attracting:TEXT attracted:TEXT gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM] {"
    )
    for conn in connections:
        source = f"{conn['parent']['piece']['id']}:{conn['parent']['connector']['id']}"
        target = f"{conn['child']['piece']['id']}:{conn['child']['connector']['id']}"
        lines.append(
            "  "
            + " ".join(
                [
                    dsl_str(conn["id"]),
                    dsl_str(source),
                    dsl_str(target),
                    dsl_num(conn.get("gap", 0)),
                    dsl_num(conn.get("shift", 0)),
                    dsl_num(conn.get("rise", 0)),
                    dsl_num(conn.get("rotation", 0)),
                    dsl_num(conn.get("turn", 0)),
                    dsl_num(conn.get("tilt", 0)),
                    dsl_num(conn.get("u", 0)),
                    dsl_num(conn.get("v", 0)),
                ]
            )
        )
    lines.append("}")
    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return {"objects": len(pieces), "attractions": len(connections), "bytes": out_path.stat().st_size}


def write_2d(
    out_path: Path,
    dream: dict,
    used_port_ids: set[str],
    ports: dict,
    types_by_id: dict,
) -> dict[str, int]:
    pieces = items(dream.get("pieces"))
    connections = items(dream.get("connections"))
    lines: list[str] = []
    lines.append("semio puzzle.puzzle2d.dsl v1")
    lines.append("schema=puzzle.2d.fixture")
    lines.append("camera {")
    lines.append("  x=0 y=0 zoom=0.05")
    lines.append("}")
    lines.append("meta {")
    lines.append("  manifest-id=capsule-dream")
    lines.append(
        "  kind-compatibility [source:TEXT target:TEXT bidirectional:BOOL important:BOOL specificity:ENUM] {"
    )
    for row in build_compat_rows(ports, used_port_ids):
        # 2d specificity enum uses handle analogue
        lines.append(row.replace(" general", " handle"))
    lines.append("  }")
    lines.append("}")
    lines.append(
        "nodes [id:TEXT node-kind:TEXT shape:TEXT x:NUM y:NUM radius:NUM width:NUM height:NUM text:TEXT icon-kind:TEXT root:BOOL scale:NUM visible:BOOL locked:BOOL anchor:ENUM handles:LIST] {"
    )
    for piece in pieces:
        type_doc = types_by_id[piece["type"]["id"]]
        pose = piece.get("pose")
        anchor = "fixed" if pose else "derived"
        center = (pose or {}).get("center") or {}
        x = float(center.get("u", 0) if pose else 0)
        y = float(center.get("v", 0) if pose else 0)
        text = piece_text(piece.get("name") or "")
        handle_chunks = []
        for conn in items(type_doc.get("connectors")):
            port_id = (conn.get("port") or {}).get("id")
            port = ports.get(port_id or "")
            grip_kind = (port or {}).get("name") or port_id or "grip"
            ang = angle_from_connector(conn)
            handle_chunks.append(
                " ".join(
                    [
                        f'id={dsl_str(piece["id"] + ":" + conn["id"])}',
                        f"handle-kind={dsl_str(grip_kind)}",
                        f"angle={dsl_num(ang)}rad",
                        "radius=3",
                    ]
                )
            )
        handles = "[ " + " ".join(handle_chunks) + " ]" if handle_chunks else "[]"
        lines.append(
            "  "
            + " ".join(
                [
                    dsl_str(piece["id"]),
                    dsl_str(type_doc["name"]),
                    "_",
                    dsl_num(x),
                    dsl_num(y),
                    "20",
                    "_",
                    "_",
                    dsl_str(text),
                    dsl_str(icon_kind(type_doc["name"])),
                    "_",
                    "_",
                    "_",
                    "_",
                    anchor,
                    handles,
                ]
            )
        )
    lines.append("}")
    lines.append(
        "edges [id:TEXT source:TEXT target:TEXT edge-kind:TEXT source-tip:TEXT target-tip:TEXT visible:BOOL locked:BOOL gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM] {"
    )
    for conn in connections:
        source = f"{conn['parent']['piece']['id']}:{conn['parent']['connector']['id']}"
        target = f"{conn['child']['piece']['id']}:{conn['child']['connector']['id']}"
        lines.append(
            "  "
            + " ".join(
                [
                    dsl_str(conn["id"]),
                    dsl_str(source),
                    dsl_str(target),
                    "_",
                    "_",
                    "_",
                    "true",
                    "false",
                    dsl_num(conn.get("gap", 0)),
                    dsl_num(conn.get("shift", 0)),
                    dsl_num(conn.get("rise", 0)),
                    dsl_num(conn.get("rotation", 0)),
                    dsl_num(conn.get("turn", 0)),
                    dsl_num(conn.get("tilt", 0)),
                    dsl_num(conn.get("u", 0)),
                    dsl_num(conn.get("v", 0)),
                ]
            )
        )
    lines.append("}")
    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return {"nodes": len(pieces), "edges": len(connections), "bytes": out_path.stat().st_size}


def write_golden(out_path: Path, dream: dict, flat: dict) -> dict[str, int]:
    dream_pieces = items(dream.get("pieces"))
    flat_by_name = {p["name"]: p for p in items(flat.get("pieces"))}
    golden: dict[str, Any] = {}
    missing = 0
    for piece in dream_pieces:
        flat_piece = flat_by_name.get(piece["name"])
        if not flat_piece or "pose" not in flat_piece:
            missing += 1
            continue
        pose = flat_piece["pose"]
        plane = pose["plane"]
        center = pose.get("center") or {}
        golden[piece["id"]] = {
            "origin": {
                "x": plane["origin"]["x"],
                "y": plane["origin"]["y"],
                "z": plane["origin"]["z"],
            },
            "xAxis": {
                "x": plane["xAxis"]["x"],
                "y": plane["xAxis"]["y"],
                "z": plane["xAxis"]["z"],
            },
            "yAxis": {
                "x": plane["yAxis"]["x"],
                "y": plane["yAxis"]["y"],
                "z": plane["yAxis"]["z"],
            },
            "center": {
                "x": center.get("u", 0),
                "y": center.get("v", 0),
            },
        }
    out_path.write_text(json.dumps(golden, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return {"poses": len(golden), "missing": missing, "bytes": out_path.stat().st_size}


#endregion

#region Main
def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    _kit, _index, dream, flat, ctx = load_kit_index()
    files = ctx["files"]
    tags = ctx["tags"]
    ports = ctx["ports"]
    types_by_id = ctx["types"]

    pieces = items(dream.get("pieces"))
    connections = items(dream.get("connections"))
    used_type_ids = sorted({p["type"]["id"] for p in pieces}, key=lambda tid: types_by_id[tid]["name"])
    used_types = [types_by_id[tid] for tid in used_type_ids]

    used_port_ids: set[str] = set()
    for type_doc in used_types:
        for conn in items(type_doc.get("connectors")):
            port_id = (conn.get("port") or {}).get("id")
            if port_id:
                used_port_ids.add(port_id)

    stats = {
        "input_pieces": len(pieces),
        "input_connections": len(connections),
        "used_types": len(used_types),
        "used_ports": len(used_port_ids),
    }
    stats["5d"] = write_5d(
        OUT / "🗣️dream.5d.dsl.semio",
        dream,
        used_types,
        used_port_ids,
        ports,
        files,
        tags,
        types_by_id,
    )
    stats["3d"] = write_3d(
        OUT / "🗣️dream.3d.dsl.semio",
        dream,
        used_types,
        used_port_ids,
        ports,
        files,
        tags,
        types_by_id,
    )
    stats["2d"] = write_2d(
        OUT / "🗣️dream.2d.dsl.semio",
        dream,
        used_port_ids,
        ports,
        types_by_id,
    )
    stats["golden"] = write_golden(OUT / "🏅golden-poses.json", dream, flat)

    summary_path = OUT / "📊e1-generator-stats.json"
    summary_path.write_text(json.dumps(stats, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(stats, indent=2))

    assert stats["5d"]["parts"] == 2880, stats["5d"]
    assert stats["5d"]["fasteners"] == 2864, stats["5d"]
    assert stats["golden"]["poses"] == 2880, stats["golden"]
    assert stats["3d"]["objects"] == 2880, stats["3d"]
    assert stats["3d"]["attractions"] == 2864, stats["3d"]
    assert stats["2d"]["nodes"] == 2880, stats["2d"]
    assert stats["2d"]["edges"] == 2864, stats["2d"]
    print("[DEBUG] e1 capsule-dream generator OK")


if __name__ == "__main__":
    main()
#endregion
