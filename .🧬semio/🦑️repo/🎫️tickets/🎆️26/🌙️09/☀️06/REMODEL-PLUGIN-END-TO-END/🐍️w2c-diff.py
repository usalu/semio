#!/usr/bin/env python3
"""🔺️ A third, independent statement of every `RemodelingMutation` kind's diff builder — guard order,
diagnostic code, diagnostic target and the lane the accepted branch writes — derived by reading each
kind's own `🔺️diff/🦀️.rs` DOCSTRING and control flow, not by transliterating its body.

Its only job is to author the committed `➡️after` / `🔺️diff` / `🎯️outcome` bytes for a vector whose
`⬅️before` and `🦠️mutation` are hand-chosen. Production (Rust) and the two reference implementations
(the case's Python oracle and the subset's TypeScript twin) then judge those bytes independently, so
this module is never anyone's oracle — a disagreement between it and any of the three is a real
finding about the vector it authored.
"""
from __future__ import annotations

import copy
import importlib.util
import math
from pathlib import Path

_spec = importlib.util.spec_from_file_location("w2c_bases", Path(__file__).with_name("🐍️w2c-bases.py"))
bases = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(bases)

DIFF_ORDER = bases.DIFF_ORDER
CONTENT_HANDLE_PREFIX = bases.CONTENT_HANDLE_PREFIX
MESH_STAGE_HANDLE_PREFIX = bases.MESH_STAGE_HANDLE_PREFIX

PARAMS_KEY = {
    "update-ingest-params": "ingest",
    "update-feature-params": "feature",
    "update-match-params": "matching",
    "update-sfm-params": "sfm",
    "update-dense-params": "dense",
    "update-mesh-params": "mesh",
    "update-motion-params": "motion",
    "update-geo-params": "geo",
}

TAGS = {
    "create-stream": "createStream", "delete-stream": "deleteStream", "change-stream-sync": "changeStreamSync",
    "add-stream-frame": "addStreamFrame", "remove-stream-frame": "removeStreamFrame", "replace-stream-source": "replaceStreamSource",
    "create-asset": "createAsset", "delete-asset": "deleteAsset", "create-camera-calibration": "createCameraCalibration",
    "update-camera-calibration": "updateCameraCalibration", "delete-camera-calibration": "deleteCameraCalibration",
    "create-rig-extrinsic": "createRigExtrinsic", "delete-rig-extrinsic": "deleteRigExtrinsic", "update-rig-extrinsic": "updateRigExtrinsic",
    "create-gcp": "createGcp", "delete-gcp": "deleteGcp", "add-gcp-observation": "addGcpObservation", "remove-gcp-observation": "removeGcpObservation",
    "update-ingest-params": "updateIngestParams", "update-feature-params": "updateFeatureParams", "update-match-params": "updateMatchParams",
    "update-sfm-params": "updateSfmParams", "update-dense-params": "updateDenseParams", "update-mesh-params": "updateMeshParams",
    "update-motion-params": "updateMotionParams", "update-geo-params": "updateGeoParams", "replace-job": "replaceJob",
    "replace-sparse": "replaceSparse", "replace-dense": "replaceDense", "replace-mesh-result": "replaceMeshResult",
    "replace-trajectory": "replaceTrajectory", "replace-tracks": "replaceTracks", "replace-geo-products": "replaceGeoProducts",
    "replace-qc": "replaceQc", "commit-reconstruction": "commitReconstruction",
}


class Note:
    __slots__ = ("level", "code", "target")

    def __init__(self, level: str, code: str, target: list[str]):
        self.level, self.code, self.target = level, code, target


def _finite(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def ordered_index(items, key_value, key_of) -> int:
    """🔢️ `mutations::ordered_index` — the canonical insertion point that keeps a keyed collection in
    ascending key order, so a member removed from anywhere comes back exactly where it was."""
    position = 0
    while position < len(items) and key_of(items[position]) < key_value:
        position += 1
    return position


def _find(items, value, key="id"):
    for index, item in enumerate(items):
        if item.get(key) == value:
            return index, item
    return -1, None


def empty_diff() -> dict:
    return {key: None for key in DIFF_ORDER}


def _lane(**fields) -> dict:
    diff = empty_diff()
    diff.update(fields)
    return diff


def diff_of(kind: str, base: dict, payload: dict) -> tuple[list[Note], dict]:
    """🔺️ `(messages, diff)` for one payload against one base — the guard order each leaf documents."""
    streams, gcps = base["streams"], base["gcps"]
    calibration, params, results = base["calibration"], base["params"], base["results"]

    if kind == "create-stream":
        if any(stream["id"] == payload["stream"]["id"] for stream in streams):
            return [Note("fatal", "mutation.duplicate-id", [payload["stream"]["id"]])], empty_diff()
        camera_id = payload["stream"].get("cameraId")
        if camera_id is not None and not any(camera["id"] == camera_id for camera in calibration["cameras"]):
            return [Note("fatal", "mutation.invariant", [payload["stream"]["id"]])], empty_diff()
        updated = copy.deepcopy(streams)
        updated.insert(ordered_index(updated, payload["stream"]["id"], lambda stream: stream["id"]), copy.deepcopy(payload["stream"]))
        return [], _lane(streams={"values": updated})

    if kind == "delete-stream":
        if not any(stream["id"] == payload["id"] for stream in streams):
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        referencing = [gcp["id"] for gcp in gcps if any(observation["streamId"] == payload["id"] for observation in gcp["observations"])]
        if referencing:
            return [Note("error", "mutation.referenced", referencing)], empty_diff()
        kept = [copy.deepcopy(stream) for stream in streams if stream["id"] != payload["id"]]
        return [], _lane(streams={"values": kept})

    if kind == "change-stream-sync":
        _, existing = _find(streams, payload["id"])
        if existing is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        if not _finite(payload["newSyncOffsetMs"]):
            return [Note("fatal", "mutation.invariant", [payload["id"]])], empty_diff()
        if existing["syncOffsetMs"] == payload["newSyncOffsetMs"]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(streams)
        _find(updated, payload["id"])[1]["syncOffsetMs"] = payload["newSyncOffsetMs"]
        return [], _lane(streams={"values": updated})

    if kind == "add-stream-frame":
        _, stream = _find(streams, payload["id"])
        if stream is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        if payload["kind"] != stream["kind"]:
            return [Note("fatal", "mutation.invariant", [payload["id"]])], empty_diff()
        if any(frame == payload["frame"] for frame in stream["frames"]):
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(streams)
        target = _find(updated, payload["id"])[1]
        key = (payload["frame"]["index"], payload["frame"]["assetId"])
        target["frames"].insert(ordered_index(target["frames"], key, lambda frame: (frame["index"], frame["assetId"])), copy.deepcopy(payload["frame"]))
        return [], _lane(streams={"values": updated})

    if kind == "remove-stream-frame":
        _, stream = _find(streams, payload["id"])
        if stream is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        if payload["frameIndex"] >= len(stream["frames"]):
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        updated = copy.deepcopy(streams)
        _find(updated, payload["id"])[1]["frames"].pop(payload["frameIndex"])
        return [], _lane(streams={"values": updated})

    if kind == "replace-stream-source":
        if not any(stream["id"] == payload["id"] for stream in streams):
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        updated = copy.deepcopy(streams)
        _find(updated, payload["id"])[1]["source"] = copy.deepcopy(payload["source"])
        return [], _lane(streams={"values": updated})

    if kind == "create-asset":
        if payload["asset"]["data"].startswith(CONTENT_HANDLE_PREFIX):
            return [Note("error", "mutation.invalid-asset-payload", [payload["key"]])], empty_diff()
        artifact = bases.durable_remodeling_asset(payload["asset"])
        if artifact is None:
            return [Note("error", "mutation.invalid-asset-payload", [payload["key"]])], empty_diff()
        handle = bases.image_asset_child_handle(payload["key"], payload["asset"])
        assets = dict(copy.deepcopy(base["assets"]))
        durable = dict(copy.deepcopy(base["durableArtifacts"]))
        previous = base["assets"].get(payload["key"])
        if previous is not None:
            durable.pop(previous["childId"], None)
        assets[payload["key"]] = handle
        durable[handle["childId"]] = artifact
        return [], _lane(assets={key: assets[key] for key in sorted(assets)}, durableArtifacts={key: durable[key] for key in sorted(durable)})

    if kind == "delete-asset":
        if payload["key"] not in base["assets"]:
            return [Note("error", "mutation.target-missing", [payload["key"]])], empty_diff()
        referencing = [stream["id"] for stream in streams if any(frame["assetId"] == payload["key"] for frame in stream["frames"])]
        if results["mesh"]["textureAssetId"] == payload["key"]:
            referencing.append("results.mesh.textureAssetId")
        if results["geo"] is not None:
            referencing += [f"results.geo.{lane}AssetId" for lane in ("dsm", "dtm", "ortho") if results["geo"][f"{lane}AssetId"] == payload["key"]]
        if referencing:
            return [Note("error", "mutation.referenced", referencing)], empty_diff()
        assets = {key: value for key, value in copy.deepcopy(base["assets"]).items() if key != payload["key"]}
        durable = dict(copy.deepcopy(base["durableArtifacts"]))
        durable.pop(base["assets"][payload["key"]]["childId"], None)
        return [], _lane(assets=assets, durableArtifacts=durable)

    if kind == "create-camera-calibration":
        if any(camera["id"] == payload["camera"]["id"] for camera in calibration["cameras"]):
            return [Note("fatal", "mutation.duplicate-id", [payload["camera"]["id"]])], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["cameras"].insert(ordered_index(updated["cameras"], payload["camera"]["id"], lambda camera: camera["id"]), copy.deepcopy(payload["camera"]))
        return [], _lane(calibration=updated)

    if kind == "update-camera-calibration":
        index, existing = _find(calibration["cameras"], payload["camera"]["id"])
        if existing is None:
            return [Note("error", "mutation.target-missing", [payload["camera"]["id"]])], empty_diff()
        camera = payload["camera"]
        lanes = [camera["fx"], camera["fy"], camera["cx"], camera["cy"], camera["skew"], *camera["distortion"]]
        if camera["rmsReprojectionPx"] is not None:
            lanes.append(camera["rmsReprojectionPx"])
        if not all(_finite(value) for value in lanes):
            return [Note("fatal", "mutation.invariant", [camera["id"]])], empty_diff()
        if existing == camera:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["cameras"][index] = copy.deepcopy(camera)
        return [], _lane(calibration=updated)

    if kind == "delete-camera-calibration":
        if not any(camera["id"] == payload["cameraId"] for camera in calibration["cameras"]):
            return [Note("error", "mutation.target-missing", [payload["cameraId"]])], empty_diff()
        referencing = [stream["id"] for stream in streams if stream["cameraId"] == payload["cameraId"]]
        if any(entry["cameraId"] == payload["cameraId"] for entry in calibration["rig"]):
            referencing.append(f"calibration.rig.{payload['cameraId']}")
        if referencing:
            return [Note("error", "mutation.referenced", referencing)], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["cameras"] = [camera for camera in updated["cameras"] if camera["id"] != payload["cameraId"]]
        return [], _lane(calibration=updated)

    if kind == "create-rig-extrinsic":
        camera_id = payload["extrinsic"]["cameraId"]
        if any(entry["cameraId"] == camera_id for entry in calibration["rig"]):
            return [Note("fatal", "mutation.duplicate-id", [camera_id])], empty_diff()
        if not any(camera["id"] == camera_id for camera in calibration["cameras"]):
            return [Note("fatal", "mutation.invariant", [camera_id])], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["rig"].insert(ordered_index(updated["rig"], camera_id, lambda entry: entry["cameraId"]), copy.deepcopy(payload["extrinsic"]))
        return [], _lane(calibration=updated)

    if kind == "delete-rig-extrinsic":
        if not any(entry["cameraId"] == payload["cameraId"] for entry in calibration["rig"]):
            return [Note("error", "mutation.target-missing", [payload["cameraId"]])], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["rig"] = [entry for entry in updated["rig"] if entry["cameraId"] != payload["cameraId"]]
        return [], _lane(calibration=updated)

    if kind == "update-rig-extrinsic":
        camera_id = payload["extrinsic"]["cameraId"]
        index, existing = _find(calibration["rig"], camera_id, key="cameraId")
        if existing is None:
            return [Note("error", "mutation.target-missing", [camera_id])], empty_diff()
        if not all(_finite(value) for value in payload["extrinsic"]["rotationWxyz"] + payload["extrinsic"]["translationM"]):
            return [Note("fatal", "mutation.invariant", [camera_id])], empty_diff()
        if existing == payload["extrinsic"]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(calibration)
        updated["rig"][index] = copy.deepcopy(payload["extrinsic"])
        return [], _lane(calibration=updated)

    if kind == "create-gcp":
        if any(gcp["id"] == payload["gcp"]["id"] for gcp in gcps):
            return [Note("fatal", "mutation.duplicate-id", [payload["gcp"]["id"]])], empty_diff()
        updated = copy.deepcopy(gcps)
        updated.insert(ordered_index(updated, payload["gcp"]["id"], lambda gcp: gcp["id"]), copy.deepcopy(payload["gcp"]))
        return [], _lane(gcps={"values": updated})

    if kind == "delete-gcp":
        _, gcp = _find(gcps, payload["id"])
        if gcp is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        kept = [copy.deepcopy(entry) for entry in gcps if entry["id"] != payload["id"]]
        notes = [Note("info", "mutation.cascade", [])] if gcp["observations"] else []
        return notes, _lane(gcps={"values": kept})

    if kind == "add-gcp-observation":
        _, gcp = _find(gcps, payload["id"])
        if gcp is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        if not any(stream["id"] == payload["observation"]["streamId"] for stream in streams):
            return [Note("fatal", "mutation.invariant", [payload["observation"]["streamId"]])], empty_diff()
        if payload["observation"] in gcp["observations"]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(gcps)
        observations = _find(updated, payload["id"])[1]["observations"]
        key = (payload["observation"]["streamId"], payload["observation"]["frameIndex"])
        observations.insert(ordered_index(observations, key, lambda observation: (observation["streamId"], observation["frameIndex"])), copy.deepcopy(payload["observation"]))
        return [], _lane(gcps={"values": updated})

    if kind == "remove-gcp-observation":
        _, gcp = _find(gcps, payload["id"])
        if gcp is None:
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        if payload["observationIndex"] >= len(gcp["observations"]):
            return [Note("error", "mutation.target-missing", [payload["id"]])], empty_diff()
        updated = copy.deepcopy(gcps)
        _find(updated, payload["id"])[1]["observations"].pop(payload["observationIndex"])
        return [], _lane(gcps={"values": updated})

    if kind in PARAMS_KEY:
        slot, incoming = PARAMS_KEY[kind], payload["params"]
        current = params[slot]
        if kind == "update-ingest-params":
            if not _finite(incoming["minSharpness"]) or incoming["minSharpness"] < 0.0 or incoming["maxFrames"] == 0 or incoming["frameSampleStride"] == 0:
                return [Note("fatal", "mutation.invariant", [])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-feature-params":
            if incoming["targetCount"] == 0 or not _finite(incoming["edgeThreshold"]) or incoming["edgeThreshold"] < 0.0:
                return [Note("fatal", "mutation.invariant", [])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-match-params":
            if not _finite(incoming["ratioTest"]) or incoming["ratioTest"] <= 0.0 or incoming["ratioTest"] > 1.0:
                return [Note("fatal", "mutation.invariant", [])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-geo-params":
            distances_ok = all(_finite(value) and value > 0.0 for value in (incoming["gsdM"], incoming["dsmCellM"], incoming["dtmFilterRadiusM"]))
            lat = incoming["originLat"]
            lon = incoming["originLon"]
            lat_ok = lat is None or (_finite(lat) and -90.0 <= lat <= 90.0)
            lon_ok = lon is None or (_finite(lon) and -180.0 <= lon <= 180.0)
            if not distances_ok or incoming["orthoMaxPx"] == 0 or not lat_ok or not lon_ok:
                return [Note("fatal", "mutation.invariant", [])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-sfm-params":
            if not _finite(incoming["ransacThresholdPx"]) or not _finite(incoming["huberDeltaPx"]):
                return [Note("fatal", "mutation.invariant", [base["id"]])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-dense-params":
            if not _finite(incoming["confidenceThreshold"]):
                return [Note("fatal", "mutation.invariant", [base["id"]])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-mesh-params":
            if not _finite(incoming["tsdfVoxelSizeMm"]) or not _finite(incoming["tsdfTruncationMm"]):
                return [Note("fatal", "mutation.invariant", [base["id"]])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        elif kind == "update-motion-params":
            if not _finite(incoming["minTrackQuality"]):
                return [Note("fatal", "mutation.invariant", [base["id"]])], empty_diff()
            if incoming == current:
                return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(params)
        updated[slot] = copy.deepcopy(incoming)
        return [], _lane(params=updated)

    if kind == "replace-job":
        if payload["job"] == base["job"]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        return [], _lane(job=copy.deepcopy(payload["job"]))

    if kind in ("replace-sparse", "replace-dense", "replace-tracks"):
        slot = {"replace-sparse": "sparse", "replace-dense": "dense", "replace-tracks": "tracks"}[kind]
        if payload[slot] == results[slot]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(results)
        updated[slot] = copy.deepcopy(payload[slot])
        return [], _lane(results=updated)

    if kind == "replace-mesh-result":
        if payload["mesh"]["mesh"]["target"]["artifactId"].startswith(MESH_STAGE_HANDLE_PREFIX):
            return [Note("error", "mutation.incomplete-mesh", [payload["mesh"]["mesh"]["childId"]])], empty_diff()
        if payload["mesh"] == results["mesh"]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(results)
        updated["mesh"] = copy.deepcopy(payload["mesh"])
        return [], _lane(results=updated)

    if kind in ("replace-trajectory", "replace-geo-products", "replace-qc"):
        slot = {"replace-trajectory": "trajectory", "replace-geo-products": "geo", "replace-qc": "qc"}[kind]
        key = {"replace-trajectory": "trajectory", "replace-geo-products": "geo", "replace-qc": "qc"}[kind]
        if payload[key] is None and results[slot] is None:
            return [Note("error", "mutation.target-missing", [base["id"]])], empty_diff()
        if payload[key] == results[slot]:
            return [Note("warning", "mutation.no-op", [])], empty_diff()
        updated = copy.deepcopy(results)
        updated[slot] = copy.deepcopy(payload[key])
        return [], _lane(results=updated)

    if kind == "commit-reconstruction":
        sparse = payload["sparse"]
        if sparse is not None and sparse["points"] != "" and not sparse["points"].startswith(CONTENT_HANDLE_PREFIX):
            return [Note("error", "mutation.invalid-reconstruction-sparse", ["sparse"])], empty_diff()
        for committed in payload["assets"]:
            if not committed["asset"]["data"].startswith(CONTENT_HANDLE_PREFIX):
                return [Note("error", "mutation.invalid-reconstruction-asset", [committed["id"]])], empty_diff()
        mesh = payload["mesh"]
        if mesh is not None and not mesh["mesh"]["target"]["artifactId"].startswith(MESH_STAGE_HANDLE_PREFIX):
            return [Note("error", "mutation.invalid-reconstruction-mesh", [mesh["mesh"]["childId"]])], empty_diff()
        raise AssertionError("commit-reconstruction publishes from process-global staging state; only its refusal paths are authorable as a static vector")

    raise AssertionError(f"no diff rule for kind {kind!r}")


def apply_diff(diff: dict, base: dict) -> dict:
    """🩹 `MutationDiff::apply` — every present lane replaces its snapshot field, `values` unwrapped."""
    after = copy.deepcopy(base)
    if diff["schema"] is not None:
        after["schema"] = diff["schema"]
    if diff["id"] is not None:
        after["id"] = diff["id"]
    if diff["streams"] is not None:
        after["streams"] = copy.deepcopy(diff["streams"]["values"])
    if diff["assets"] is not None:
        after["assets"] = copy.deepcopy(diff["assets"])
    if diff["durableArtifacts"] is not None:
        after["durableArtifacts"] = copy.deepcopy(diff["durableArtifacts"])
    if diff["calibration"] is not None:
        after["calibration"] = copy.deepcopy(diff["calibration"])
    if diff["params"] is not None:
        after["params"] = copy.deepcopy(diff["params"])
    if diff["gcps"] is not None:
        after["gcps"] = copy.deepcopy(diff["gcps"]["values"])
    if diff["job"] is not None:
        after["job"] = copy.deepcopy(diff["job"])
    if diff["results"] is not None:
        after["results"] = copy.deepcopy(diff["results"])
    return after


def refused(notes: list[Note]) -> Note | None:
    for note in notes:
        if note.level in ("error", "fatal"):
            return note
    return None


# region 🔖️ProductionInverse
def remodeling_asset(base: dict, key: str):
    """🖼️ `remodeling_asset` — the payload a captured handle can be reconstituted from, which is the
    document's OWN durable leaf for that handle's content address (`🦀️.rs:233`), not a process cache.
    `None` when the document carries the handle but not the leaf, which is what makes the two asset
    verbs' inverses conditional."""
    handle = base["assets"].get(key)
    if handle is None:
        return None
    artifact = base["durableArtifacts"].get(handle["childId"])
    if artifact is None or artifact["kind"] != "image" or artifact["mime"] is None:
        return None
    import base64 as _b64

    raw = b"".join(_b64.b64decode(chunk, validate=True) for chunk in artifact["chunks"])
    if not raw:
        return None
    return {"mime": artifact["mime"], "data": _b64.b64encode(raw).decode("ascii"), "width": artifact["width"], "height": artifact["height"]}


def prod_inverse(kind: str, base: dict, payload: dict) -> list[tuple[str, dict]]:
    """↩️ PRODUCTION's own `↩️inverse/🦀️.rs` for one kind, mirrored so the generator can ask whether a
    candidate vector's inverse really restores its before-document before committing a scenario that
    claims it does. Every one of these is BASE-derived and, since the canonical-order invariant landed,
    single-step and position-exact: the generator measures that, and refuses to commit a vector whose
    inverse does not restore."""
    streams, gcps = base["streams"], base["gcps"]
    cameras, rig = base["calibration"]["cameras"], base["calibration"]["rig"]
    if kind == "create-stream":
        return [] if _find(streams, payload["stream"]["id"])[1] else [("delete-stream", {"id": payload["stream"]["id"]})]
    if kind == "delete-stream":
        _, stream = _find(streams, payload["id"])
        return [("create-stream", {"stream": copy.deepcopy(stream)})] if stream else []
    if kind == "change-stream-sync":
        _, stream = _find(streams, payload["id"])
        return [("change-stream-sync", {"id": payload["id"], "newSyncOffsetMs": stream["syncOffsetMs"]})] if stream else []
    if kind == "add-stream-frame":
        _, stream = _find(streams, payload["id"])
        if stream is None or payload["kind"] != stream["kind"] or any(frame == payload["frame"] for frame in stream["frames"]):
            return []
        key = (payload["frame"]["index"], payload["frame"]["assetId"])
        return [("remove-stream-frame", {"id": payload["id"], "frameIndex": ordered_index(stream["frames"], key, lambda frame: (frame["index"], frame["assetId"]))})]
    if kind == "remove-stream-frame":
        _, stream = _find(streams, payload["id"])
        if stream is None or payload["frameIndex"] >= len(stream["frames"]):
            return []
        return [("add-stream-frame", {"id": payload["id"], "frame": copy.deepcopy(stream["frames"][payload["frameIndex"]]), "kind": stream["kind"]})]
    if kind == "replace-stream-source":
        _, stream = _find(streams, payload["id"])
        return [("replace-stream-source", {"id": payload["id"], "source": copy.deepcopy(stream["source"])})] if stream else []
    if kind == "create-asset":
        if payload["key"] in base["assets"]:
            old = remodeling_asset(base, payload["key"])
            return [("create-asset", {"key": payload["key"], "asset": old})] if old else []
        return [("delete-asset", {"key": payload["key"]})]
    if kind == "delete-asset":
        old = remodeling_asset(base, payload["key"])
        return [("create-asset", {"key": payload["key"], "asset": old})] if old else []
    if kind == "create-camera-calibration":
        return [] if _find(cameras, payload["camera"]["id"])[1] else [("delete-camera-calibration", {"cameraId": payload["camera"]["id"]})]
    if kind == "update-camera-calibration":
        _, camera = _find(cameras, payload["camera"]["id"])
        return [("update-camera-calibration", {"camera": copy.deepcopy(camera)})] if camera else []
    if kind == "delete-camera-calibration":
        _, camera = _find(cameras, payload["cameraId"])
        return [("create-camera-calibration", {"camera": copy.deepcopy(camera)})] if camera else []
    if kind == "create-rig-extrinsic":
        return [] if _find(rig, payload["extrinsic"]["cameraId"], key="cameraId")[1] else [("delete-rig-extrinsic", {"cameraId": payload["extrinsic"]["cameraId"]})]
    if kind == "delete-rig-extrinsic":
        _, extrinsic = _find(rig, payload["cameraId"], key="cameraId")
        return [("create-rig-extrinsic", {"extrinsic": copy.deepcopy(extrinsic)})] if extrinsic else []
    if kind == "update-rig-extrinsic":
        _, extrinsic = _find(rig, payload["extrinsic"]["cameraId"], key="cameraId")
        return [("update-rig-extrinsic", {"extrinsic": copy.deepcopy(extrinsic)})] if extrinsic else []
    if kind == "create-gcp":
        return [] if _find(gcps, payload["gcp"]["id"])[1] else [("delete-gcp", {"id": payload["gcp"]["id"]})]
    if kind == "delete-gcp":
        _, gcp = _find(gcps, payload["id"])
        return [("create-gcp", {"gcp": copy.deepcopy(gcp)})] if gcp else []
    if kind == "add-gcp-observation":
        _, gcp = _find(gcps, payload["id"])
        if gcp is None or payload["observation"] in gcp["observations"] or not any(stream["id"] == payload["observation"]["streamId"] for stream in streams):
            return []
        key = (payload["observation"]["streamId"], payload["observation"]["frameIndex"])
        return [("remove-gcp-observation", {"id": payload["id"], "observationIndex": ordered_index(gcp["observations"], key, lambda observation: (observation["streamId"], observation["frameIndex"]))})]
    if kind == "remove-gcp-observation":
        _, gcp = _find(gcps, payload["id"])
        if gcp is None or payload["observationIndex"] >= len(gcp["observations"]):
            return []
        return [("add-gcp-observation", {"id": payload["id"], "observation": copy.deepcopy(gcp["observations"][payload["observationIndex"]])})]
    if kind in PARAMS_KEY:
        return [(kind, {"params": copy.deepcopy(base["params"][PARAMS_KEY[kind]])})]
    if kind == "replace-job":
        return [("replace-job", {"job": copy.deepcopy(base["job"])})]
    if kind == "replace-sparse":
        return [("replace-sparse", {"sparse": copy.deepcopy(base["results"]["sparse"])})]
    if kind == "replace-dense":
        return [("replace-dense", {"dense": copy.deepcopy(base["results"]["dense"])})]
    if kind == "replace-mesh-result":
        return [("replace-mesh-result", {"mesh": copy.deepcopy(base["results"]["mesh"])})]
    if kind == "replace-trajectory":
        return [("replace-trajectory", {"trajectory": copy.deepcopy(base["results"]["trajectory"])})]
    if kind == "replace-tracks":
        return [("replace-tracks", {"tracks": copy.deepcopy(base["results"]["tracks"])})]
    if kind == "replace-geo-products":
        return [("replace-geo-products", {"geo": copy.deepcopy(base["results"]["geo"])})]
    if kind == "replace-qc":
        return [("replace-qc", {"qc": copy.deepcopy(base["results"]["qc"])})]
    if kind == "commit-reconstruction":
        steps = [
            ("replace-job", {"job": copy.deepcopy(base["job"])}),
            ("replace-sparse", {"sparse": copy.deepcopy(base["results"]["sparse"])}),
            ("replace-trajectory", {"trajectory": copy.deepcopy(base["results"]["trajectory"])}),
            ("replace-mesh-result", {"mesh": copy.deepcopy(base["results"]["mesh"])}),
            ("replace-geo-products", {"geo": copy.deepcopy(base["results"]["geo"])}),
            ("replace-qc", {"qc": copy.deepcopy(base["results"]["qc"])}),
        ]
        for committed in payload["assets"]:
            old = remodeling_asset(base, committed["id"])
            steps.append(("create-asset", {"key": committed["id"], "asset": old}) if old else ("delete-asset", {"key": committed["id"]}))
        return steps
    raise AssertionError(f"no production inverse rule for kind {kind!r}")


def inverse_restores(kind: str, base: dict, payload: dict, after: dict) -> bool:
    """🔍 Whether production's own inverse, applied step by step to the post-mutation document the way
    `undo_remodeling_mutation_json` does, really lands back on `base`."""
    current = after
    for step_kind, step_payload in prod_inverse(kind, base, payload):
        _notes, step_diff = diff_of(step_kind, current, step_payload)
        current = apply_diff(step_diff, current)
    return current == base
# endregion 🔖️ProductionInverse
