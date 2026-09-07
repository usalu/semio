#!/usr/bin/env python3
"""🧫️ The vector catalogue for `📸️mutate-remodeling-1` — every specification vector this subset
commits, as `(kind, role, base, payload)` rows. The generator beside it turns each row into the
committed quintet, its mounted Rust module, its feature rows and both references' registrations.

Four roles per kind, as far as the kind's own semantics reach:

| role | what it pins |
|---|---|
| `toy` | the pre-existing two-stream unit vector, kept verbatim so its own history stays readable |
| `realworld` | the same verb on the ten-frame two-camera orbit survey, with a durable-artifact store, a GCP network, a 104-point sparse cloud and real results |
| `refusal` | the verb's OWN documented guard — the code, the level and the diagnostic target its `🔺️diff/🦀️.rs` raises, read off that file's control flow |
| `edge` | first/last/empty position, or the `mutation.no-op` warning branch for a verb whose only non-applying path is an identical resubmission |

A kind with two distinct refusal guards gets two refusal rows; a kind whose only guard is
`mutation.no-op` says so in its `edge` row rather than inventing a refusal it does not have.
"""
from __future__ import annotations

import copy
import importlib.util
import struct
from pathlib import Path


def _load(name: str):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), Path(__file__).with_name(name))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


bases = _load("🐍️w2c-bases.py")
diffs = _load("🐍️w2c-diff.py")


def q(value: float) -> float:
    """🎚️ The base scenes' own dyadic snap (`🐍️w2c-bases.py`'s `_quantize`), for every lane the schema
    types `format: float`: a value exact in both widths, printing to the same lexeme either way."""
    return bases._quantize(value)


TOY = bases.toy_base()
REAL = bases.realworld_base()
REAL_CLEARED = copy.deepcopy(REAL)
REAL_CLEARED["id"] = "orbit-survey-unfinished"
REAL_CLEARED["results"]["trajectory"] = None
REAL_CLEARED["results"]["geo"] = None
REAL_CLEARED["results"]["qc"] = None
REAL_CLEARED["job"] = copy.deepcopy(REAL["job"]) | {"stage": "estimating-poses", "progress01": q(0.25), "stageCursor": 5}

BASES = {"toy": TOY, "real": REAL, "real-cleared": REAL_CLEARED}


class Vector:
    """🧾 One committed specification vector, before any of its derived bytes exist."""

    __slots__ = ("kind", "role", "scenario", "case", "base", "payload", "note")

    def __init__(self, kind: str, role: str, scenario: str, case: str, base: str, payload: dict, note: str):
        self.kind, self.role, self.scenario, self.case, self.base, self.payload, self.note = kind, role, scenario, case, base, payload, note


def V(kind, role, scenario, case, base, payload, note):
    return Vector(kind, role, scenario, case, base, payload, note)


# region 🔖️ReusableFragments
ORBIT_FRAME_10 = {"index": 10, "timestampMs": 5000.0, "assetId": "orbit-frame-00"}
NEW_CAMERA = {"id": "orbit-cam-quaternary", "label": "Orbit Quaternary", "model": "fisheye", "fx": 272.0, "fy": 272.0, "cx": 160.0, "cy": 120.0, "skew": 0.0, "distortion": [q(0.125), 0.0, 0.0, 0.0, 0.0], "rmsReprojectionPx": None, "locked": False}
STAGED_MESH = {
    "mesh": {"childId": "remodeling-mesh-8f19c0d2a4b76e31", "target": {"artifactId": "mesh-stage:orbit-run-7", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}},
    "source": "reconstructed", "textureAssetId": None, "watertight": None,
}
IMPORTED_MESH = {
    "mesh": {"childId": "remodeling-mesh-2f6c81b0d4a37e59", "target": {"artifactId": "orbit-survey-mesh-imported", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}},
    "source": "imported", "textureAssetId": None, "watertight": None,
}
CONTENT_HANDLE = "remodeling-content:orbit-blob-3|stage-7|2"
# endregion 🔖️ReusableFragments


def catalogue() -> list[Vector]:
    real_streams = REAL["streams"]
    rows: list[Vector] = []
    add = rows.append

    # region 🔖️Streams
    add(V("create-stream", "realworld", "create-stream-realworld", "🛰️adds-a-third-orbit-stream-bound-to-the-tertiary-camera", "real",
          {"stream": {"id": "orbit-tertiary", "name": "Orbit Tertiary", "kind": "image-sequence", "cameraId": "orbit-cam-tertiary", "syncOffsetMs": 80.0, "fpsHint": 2.0, "frames": [], "source": None}},
          "appends a third stream to a survey that already carries two, bound to the rig's spare camera"))
    add(V("create-stream", "refusal", "create-stream-duplicate-id", "🔂️rejects-a-duplicate-stream-id-the-survey-already-carries", "real",
          {"stream": copy.deepcopy(real_streams[0])},
          "the duplicate-id guard fires before the camera-reference guard, so an id clash is reported even when the camera is known"))
    add(V("create-stream", "refusal", "create-stream-unknown-camera", "👻️rejects-a-stream-bound-to-a-camera-the-calibration-never-had", "real",
          {"stream": {"id": "orbit-quaternary", "name": "Orbit Quaternary", "kind": "image-sequence", "cameraId": "orbit-cam-absent", "syncOffsetMs": 0.0, "fpsHint": 2.0, "frames": [], "source": None}},
          "an unknown camera binding is a Fatal invariant reported by the STREAM id, not the camera id"))
    add(V("create-stream", "edge", "create-stream-unbound", "🎞️adds-an-unbound-stream-with-no-camera-and-no-frames", "real",
          {"stream": {"id": "orbit-handheld", "name": "Handheld Pass", "kind": "image-sequence", "cameraId": None, "syncOffsetMs": 0.0, "fpsHint": 1.0, "frames": [], "source": None}},
          "a null camera binding skips the camera guard entirely, and an empty frame list is never invented into"))

    add(V("delete-stream", "realworld", "delete-stream-realworld", "🪓removes-the-spare-pass-no-ground-control-point-observes", "real",
          {"id": "orbit-spare"},
          "the only stream of the survey no GCP observation names, and therefore the only one this verb accepts"))
    add(V("delete-stream", "refusal", "delete-stream-missing", "🚫️refuses-to-remove-a-stream-the-survey-never-had", "real",
          {"id": "orbit-absent"}, "an unknown id is an Error, never a silent no-op"))
    add(V("delete-stream", "refusal", "delete-stream-referenced", "⛓️refuses-to-remove-a-stream-three-ground-control-points-still-observe", "real",
          {"id": "orbit-primary"},
          "observations belong to their GCPs, not to the stream, so the delete refuses instead of severing another record's data"))

    add(V("change-stream-sync", "realworld", "change-stream-sync-realworld", "⏱️retimes-the-secondary-stream-to-a-negative-rolling-shutter-offset", "real",
          {"id": "orbit-secondary", "newSyncOffsetMs": q(-16.5)}, "a real rolling-shutter correction on a rig's second stream"))
    add(V("change-stream-sync", "refusal", "change-stream-sync-missing", "🚫️refuses-to-retime-a-stream-the-survey-never-had", "real",
          {"id": "orbit-absent", "newSyncOffsetMs": 12.0}, "the missing-target guard runs before the identical-offset guard"))
    add(V("change-stream-sync", "edge", "change-stream-sync-noop", "🔁️warns-that-the-primary-stream-already-carries-this-offset", "real",
          {"id": "orbit-primary", "newSyncOffsetMs": 0.0}, "an identical resubmission is a Warning with an empty diff, not an Error"))

    add(V("add-stream-frame", "realworld", "add-stream-frame-realworld", "🎞️appends-an-eleventh-frame-to-the-primary-orbit-stream", "real",
          {"id": "orbit-primary", "frame": ORBIT_FRAME_10, "kind": "video"},
          "index 10 sorts after every frame already present, so the canonical insert lands at the end here"))
    add(V("add-stream-frame", "refusal", "add-stream-frame-missing", "🚫️refuses-to-append-a-frame-to-a-stream-the-survey-never-had", "real",
          {"id": "orbit-absent", "frame": ORBIT_FRAME_10, "kind": "video"}, "the owner stream is resolved before the frame is inspected"))
    add(V("add-stream-frame", "refusal", "add-stream-frame-kind", "🎬️refuses-a-frame-that-declares-a-media-kind-the-stream-does-not-have", "real",
          {"id": "orbit-primary", "frame": ORBIT_FRAME_10, "kind": "image-sequence"},
          "`kind` asserts the owner stream's provenance instead of rewriting it, which is what gives this verb an inverse"))
    add(V("add-stream-frame", "edge", "add-stream-frame-noop", "🔁️warns-that-the-primary-stream-already-holds-this-exact-frame", "real",
          {"id": "orbit-primary", "frame": copy.deepcopy(real_streams[0]["frames"][3]), "kind": "video"},
          "equality is on the WHOLE frame record, not on its index alone"))

    add(V("remove-stream-frame", "realworld", "remove-stream-frame-realworld", "✂️drops-the-middle-frame-of-the-primary-orbit-stream", "real",
          {"id": "orbit-primary", "frameIndex": 4}, "removing a middle member is what pins that the inverse restores POSITION, not just membership"))
    add(V("remove-stream-frame", "refusal", "remove-stream-frame-out-of-range", "🚫️refuses-a-frame-index-past-the-end-of-the-stream", "real",
          {"id": "orbit-primary", "frameIndex": 99}, "an out-of-range index is reported as target-missing against the STREAM id"))
    add(V("remove-stream-frame", "edge", "remove-stream-frame-first", "⏮️drops-the-first-frame-so-every-survivor-shifts-down-one-slot", "real",
          {"id": "orbit-secondary", "frameIndex": 0}, "index 0 is the boundary the range guard must admit"))

    add(V("replace-stream-source", "realworld", "replace-stream-source-realworld", "🎥️reingests-the-primary-stream-from-a-longer-master-clip", "real",
          {"id": "orbit-primary", "source": {"name": "orbit-primary-master.mov", "container": "mov", "codec": "hevc", "durationMs": 6000.0, "frameCount": 12, "width": 320, "height": 240}},
          "the replacement keeps the raster consistent with the camera's own principal point"))
    add(V("replace-stream-source", "refusal", "replace-stream-source-missing", "🚫️refuses-to-reingest-a-stream-the-survey-never-had", "real",
          {"id": "orbit-absent", "source": None}, "this verb has no no-op guard at all, so target-missing is its only diagnostic"))
    add(V("replace-stream-source", "edge", "replace-stream-source-attaches", "📼️attaches-a-source-to-a-stream-that-had-none", "real",
          {"id": "orbit-secondary", "source": {"name": "orbit-secondary.mkv", "container": "mkv", "codec": "avc", "durationMs": 5000.0, "frameCount": 10, "width": 320, "height": 240}},
          "moving a source slot from null to a value is the empty-boundary case"))
    # endregion 🔖️Streams

    # region 🔖️Assets
    add(V("create-asset", "realworld", "create-asset-realworld", "🖼️stores-an-eleventh-orbit-frame-as-a-durable-child-leaf", "real",
          {"key": "orbit-frame-10", "asset": {"mime": "image/png", "data": "b3JiaXQtZnJhbWUtMTA=", "width": 320, "height": 240}},
          "the accepted branch writes BOTH the assets map and the durable-artifact store, which is the only verb that writes two lanes"))
    add(V("create-asset", "refusal", "create-asset-staging-handle", "🚫️refuses-an-asset-whose-payload-is-a-private-staging-handle", "real",
          {"key": "orbit-frame-11", "asset": {"mime": "image/png", "data": CONTENT_HANDLE, "width": 320, "height": 240}},
          "a replayable content handle is accepted only by commit-reconstruction, never by create-asset"))
    add(V("create-asset", "edge", "create-asset-upsert", "♻️overwrites-an-existing-asset-key-because-a-retried-import-must-succeed", "real",
          {"key": "orbit-frame-00", "asset": {"mime": "image/png", "data": "b3JiaXQtZnJhbWUtMDAtcmVzY2Fu", "width": 320, "height": 240}},
          "deliberately NOT a duplicate-id refusal — the leaf's own docstring records that import retries depend on upsert"))

    add(V("delete-asset", "realworld", "delete-asset-realworld", "🗑️sweeps-the-calibration-target-raster-nothing-in-the-survey-references", "real",
          {"key": "orbit-orphan"}, "the accepted branch drops the durable leaf with the entry, which is what makes create-asset its inverse"))
    add(V("delete-asset", "refusal", "delete-asset-missing", "🚫️refuses-to-remove-an-asset-key-the-survey-never-had", "real",
          {"key": "orbit-absent"}, "an unknown key is an Error against the key itself"))
    add(V("delete-asset", "refusal", "delete-asset-referenced-frames", "🖼️refuses-to-remove-a-captured-asset-two-stream-frames-still-reference", "real",
          {"key": "orbit-frame-09"}, "a stream frame is a reference, so the document never ends up naming a leaf that is gone"))
    add(V("delete-asset", "refusal", "delete-asset-geo-product", "🗺️refuses-to-remove-the-dsm-raster-the-geo-products-still-name", "real",
          {"key": "orbit-dsm"}, "the guard reaches results.geo's asset slots and the mesh texture, not only stream frames"))
    # endregion 🔖️Assets

    # region 🔖️Calibration
    add(V("create-camera-calibration", "realworld", "create-camera-calibration-realworld", "📷️adds-a-fourth-fisheye-camera-to-the-survey-calibration", "real",
          {"camera": NEW_CAMERA}, "a fisheye intrinsic joins the two rectilinear ones without touching the rig"))
    add(V("create-camera-calibration", "refusal", "create-camera-calibration-duplicate", "🚫️refuses-a-camera-id-the-calibration-already-carries", "real",
          {"camera": copy.deepcopy(REAL["calibration"]["cameras"][0])}, "the only guard this verb has is Fatal duplicate-id"))

    add(V("update-camera-calibration", "realworld", "update-camera-calibration-realworld", "🔍️refines-the-primary-intrinsic-after-a-second-bundle-adjustment", "real",
          {"camera": copy.deepcopy(REAL["calibration"]["cameras"][0]) | {"fx": q(271.5), "fy": q(271.5), "rmsReprojectionPx": q(0.3125)}},
          "a real post-BA refinement: focal length and residual move together"))
    add(V("update-camera-calibration", "refusal", "update-camera-calibration-missing", "🚫️refuses-to-refine-a-camera-the-calibration-never-had", "real",
          {"camera": NEW_CAMERA}, "missing-target precedes both the no-op and the finiteness guard"))
    add(V("update-camera-calibration", "edge", "update-camera-calibration-noop", "🔁️warns-that-the-secondary-camera-is-already-at-this-calibration", "real",
          {"camera": copy.deepcopy(REAL["calibration"]["cameras"][1])}, "the identical-resubmission Warning is checked AFTER the finiteness invariant, as everywhere else"))

    add(V("delete-camera-calibration", "realworld", "delete-camera-calibration-realworld", "🚫️removes-the-spare-tertiary-camera-nothing-references", "real",
          {"cameraId": "orbit-cam-tertiary"}, "the one camera with neither a stream nor a rig entry behind it"))
    add(V("delete-camera-calibration", "refusal", "delete-camera-calibration-missing", "🚫️refuses-to-remove-a-camera-the-calibration-never-had", "real",
          {"cameraId": "orbit-cam-absent"}, "the missing-target guard runs before the referential one"))
    add(V("delete-camera-calibration", "refusal", "delete-camera-calibration-referenced", "⛓️refuses-to-remove-a-camera-a-rig-entry-and-a-stream-still-name", "real",
          {"cameraId": "orbit-cam-secondary"}, "a calibration owns nothing, so it may not leave while a stream binding or a rig entry depends on it"))

    add(V("create-rig-extrinsic", "realworld", "create-rig-extrinsic-realworld", "🔗️places-the-tertiary-camera-into-the-rig-for-the-first-time", "real",
          {"extrinsic": {"cameraId": "orbit-cam-tertiary", "rotationWxyz": [q(0.9961947), 0.0, q(0.08715574), 0.0], "translationM": [q(-0.125), 0.0, q(0.03125)]}},
          "the only camera in the survey that is calibrated but not yet placed"))
    add(V("create-rig-extrinsic", "refusal", "create-rig-extrinsic-duplicate", "🚫️refuses-a-second-rig-entry-for-a-camera-already-placed", "real",
          {"extrinsic": copy.deepcopy(REAL["calibration"]["rig"][0])}, "duplicate-id is checked before the unknown-camera invariant"))
    add(V("create-rig-extrinsic", "refusal", "create-rig-extrinsic-unknown-camera", "🚫️refuses-a-rig-entry-for-a-camera-the-calibration-never-had", "real",
          {"extrinsic": {"cameraId": "orbit-cam-absent", "rotationWxyz": [1.0, 0.0, 0.0, 0.0], "translationM": [0.0, 0.0, 0.0]}},
          "a rig entry may only name a calibrated camera"))

    add(V("delete-rig-extrinsic", "realworld", "delete-rig-extrinsic-realworld", "✂️unplaces-the-secondary-camera-from-the-rig", "real",
          {"cameraId": "orbit-cam-secondary"}, "the camera stays calibrated; only its placement is dropped"))
    add(V("delete-rig-extrinsic", "refusal", "delete-rig-extrinsic-missing", "🚫️refuses-to-unplace-a-camera-the-rig-never-held", "real",
          {"cameraId": "orbit-cam-tertiary"}, "a calibrated camera with no rig entry is still a missing target for this verb"))
    add(V("delete-rig-extrinsic", "edge", "delete-rig-extrinsic-first", "⏮️unplaces-the-first-rig-entry-so-the-survivor-moves-to-position-zero", "real",
          {"cameraId": "orbit-cam-primary"}, "removing the reference camera is the position-sensitive boundary"))

    add(V("update-rig-extrinsic", "realworld", "update-rig-extrinsic-realworld", "📍️retunes-the-secondary-camera-baseline-after-rig-self-calibration", "real",
          {"extrinsic": {"cameraId": "orbit-cam-secondary", "rotationWxyz": [q(0.99879545), 0.0, q(0.04906767), 0.0], "translationM": [q(0.1275), q(0.0025), 0.0]}},
          "a real baseline correction of about 2.5 mm after rig self-calibration"))
    add(V("update-rig-extrinsic", "refusal", "update-rig-extrinsic-missing", "🚫️refuses-to-retune-a-camera-the-rig-never-held", "real",
          {"extrinsic": {"cameraId": "orbit-cam-tertiary", "rotationWxyz": [1.0, 0.0, 0.0, 0.0], "translationM": [0.0, 0.0, 0.0]}},
          "missing-target precedes the finiteness invariant and the no-op Warning alike"))
    add(V("update-rig-extrinsic", "edge", "update-rig-extrinsic-noop", "🔁️warns-that-the-primary-rig-entry-is-already-at-this-pose", "real",
          {"extrinsic": copy.deepcopy(REAL["calibration"]["rig"][0])}, "unlike update-camera-calibration, here the invariant is checked BEFORE the no-op"))
    # endregion 🔖️Calibration

    # region 🔖️GroundControl
    add(V("create-gcp", "realworld", "create-gcp-realworld", "📍️adds-a-quay-bollard-control-point-seen-from-both-streams", "real",
          {"gcp": {"id": "gcp-quay-bollard", "name": "Quay Bollard", "worldPosition": [q(-0.75), q(0.875), q(-1.0)],
                   "observations": [{"streamId": "orbit-primary", "frameIndex": 1, "pixel": [q(96.5), q(178.25)]},
                                    {"streamId": "orbit-secondary", "frameIndex": 8, "pixel": [q(214.75), q(169.0)]}]}},
          "a real control point enters the network already carrying its own cross-stream observations"))
    add(V("create-gcp", "refusal", "create-gcp-duplicate", "🚫️refuses-a-control-point-id-the-network-already-carries", "real",
          {"gcp": copy.deepcopy(REAL["gcps"][0])}, "the only guard this verb has is Fatal duplicate-id"))
    add(V("create-gcp", "edge", "create-gcp-unobserved", "🕳️adds-a-control-point-with-no-observations-at-all", "real",
          {"gcp": {"id": "gcp-benchmark", "name": "Levelling Benchmark", "worldPosition": [q(0.0), q(0.0), q(-1.25)], "observations": []}},
          "an empty observation list is admitted: a surveyed point may be entered before it is picked"))

    add(V("delete-gcp", "realworld", "delete-gcp-realworld", "🚮removes-the-south-pier-point-and-both-of-its-observations", "real",
          {"id": "gcp-south-pier"}, "the observation cascade is reported by count, and the inverse must restore both"))
    add(V("delete-gcp", "refusal", "delete-gcp-missing", "🚫️refuses-to-remove-a-control-point-the-network-never-had", "real",
          {"id": "gcp-absent"}, "an unknown id is an Error against the id itself"))
    add(V("delete-gcp", "edge", "delete-gcp-unobserved", "🕳️removes-an-unobserved-control-point-so-no-cascade-is-reported", "real",
          {"id": "gcp-unsurveyed"}, "zero observations is the boundary at which the cascade note must NOT be emitted"))

    add(V("add-gcp-observation", "realworld", "add-gcp-observation-realworld", "🔎️picks-the-south-pier-point-in-a-third-frame", "real",
          {"id": "gcp-south-pier", "observation": {"streamId": "orbit-primary", "frameIndex": 9, "pixel": [q(88.25), q(146.5)]}},
          "a third pick strengthens a two-view control point into a three-view one"))
    add(V("add-gcp-observation", "refusal", "add-gcp-observation-missing", "🚫️refuses-to-pick-a-control-point-the-network-never-had", "real",
          {"id": "gcp-absent", "observation": {"streamId": "orbit-primary", "frameIndex": 0, "pixel": [q(10.0), q(20.0)]}},
          "the owner point is resolved before the observation is inspected"))
    add(V("add-gcp-observation", "edge", "add-gcp-observation-noop", "🔁️warns-that-this-exact-pick-is-already-recorded", "real",
          {"id": "gcp-north-pier", "observation": copy.deepcopy(REAL["gcps"][0]["observations"][1])},
          "equality is on the whole observation, so the same pixel in a different frame is NOT a no-op"))

    add(V("remove-gcp-observation", "realworld", "remove-gcp-observation-realworld", "🚷drops-the-middle-observation-of-the-north-pier-point", "real",
          {"id": "gcp-north-pier", "observationIndex": 1}, "a mis-picked middle observation, whose inverse must restore its POSITION"))
    add(V("remove-gcp-observation", "refusal", "remove-gcp-observation-out-of-range", "🚫️refuses-an-observation-index-past-the-end-of-the-point", "real",
          {"id": "gcp-south-pier", "observationIndex": 7}, "an out-of-range index is reported as target-missing against the POINT id"))
    add(V("remove-gcp-observation", "edge", "remove-gcp-observation-first", "⏮️drops-the-first-observation-so-the-others-shift-down-one-slot", "real",
          {"id": "gcp-roof-vent", "observationIndex": 0}, "index 0 is the boundary the range guard must admit"))
    # endregion 🔖️GroundControl

    # region 🔖️Params
    params_rows = [
        ("update-ingest-params", "ingest", {"frameSampleStride": 1, "maxFrames": 64, "downscaleLongEdgePx": 640, "minSharpness": q(0.1875)},
         "📥️widens-the-ingest-budget-and-doubles-the-working-resolution",
         {"frameSampleStride": 1, "maxFrames": 0, "downscaleLongEdgePx": 320, "minSharpness": q(0.125)},
         "🚫️refuses-an-ingest-budget-of-zero-frames"),
        ("update-feature-params", "feature", {"detector": "harris", "targetCount": 2400, "octaves": 5, "edgeThreshold": q(8.5)},
         "🌟️moves-the-detector-to-harris-and-quadruples-the-target-count",
         {"detector": "akaze", "targetCount": 0, "octaves": 4, "edgeThreshold": 10.0},
         "🚫️refuses-a-feature-budget-of-zero-keypoints"),
        ("update-match-params", "matching", {"matcher": "kd-tree", "ratioTest": q(0.75), "crossCheck": True, "sequentialWindow": 10, "maxPairsPerFrame": 24, "loopClosure": True},
         "🌳️switches-to-a-kd-tree-matcher-with-a-tighter-ratio-test",
         {"matcher": "brute-force", "ratioTest": q(1.5), "crossCheck": True, "sequentialWindow": 6, "maxPairsPerFrame": 16, "loopClosure": True},
         "🚫️refuses-a-ratio-test-above-one"),
        ("update-geo-params", "geo", {"enabled": True, "originLon": q(8.5416), "originLat": q(47.3769), "originAlt": q(408.5), "gsdM": q(0.025), "dsmCellM": q(0.05), "dtmFilterRadiusM": q(1.5), "orthoMaxPx": 8192},
         "🌐️halves-the-ground-sample-distance-and-doubles-the-ortho-budget",
         {"enabled": True, "originLon": q(8.5), "originLat": q(47.375), "originAlt": 408.0, "gsdM": q(0.05), "dsmCellM": q(0.1), "dtmFilterRadiusM": 2.0, "orthoMaxPx": 0},
         "🚫️refuses-a-zero-pixel-ortho-budget"),
        ("update-sfm-params", "sfm", {"ransacIterations": 4000, "ransacThresholdPx": q(1.25), "minTrackLength": 3, "baMaxIterations": 60, "robustLoss": "cauchy", "huberDeltaPx": q(0.875)},
         "🎯️tightens-the-ransac-threshold-and-switches-to-a-cauchy-loss", None, None),
        ("update-dense-params", "dense", {"resolution": "high", "windowRadiusPx": 4, "minViewConsistency": 4, "confidenceThreshold": q(0.6875), "maxPoints": 200000},
         "🧊️sharpens-the-fusion-to-a-high-resolution-consistency-window", None, None),
        ("update-mesh-params", "mesh", {"tsdfVoxelSizeMm": q(50.0), "tsdfTruncationMm": q(165.0), "decimateTargetTriangles": 60000, "smoothingIterations": 2, "textureEnabled": True, "textureSize": 2048, "guaranteeWatertight": True, "holeFillMaxBoundaryVerts": 1024, "selfIntersectionCheck": True},
         "🔳️halves-the-voxel-size-and-turns-texturing-on", None, None),
        ("update-motion-params", "motion", {"enabled": True, "maxTracks": 192, "trackWindowPx": 31, "minTrackQuality": q(0.45), "minTrackLengthFrames": 6},
         "🏃️triples-the-motion-track-budget-and-raises-the-quality-floor", None, None),
    ]
    for kind, slot, tuned, tuned_case, invalid, invalid_case in params_rows:
        add(V(kind, "realworld", f"{kind}-realworld", tuned_case, "real", {"params": tuned},
              "a real retune of one parameter block; every other block is left untouched"))
        if invalid is not None:
            add(V(kind, "refusal", f"{kind}-invariant", invalid_case, "real", {"params": invalid},
                  "the block's own Fatal invariant, checked before the identical-resubmission Warning"))
        add(V(kind, "edge", f"{kind}-noop", f"🔁️warns-that-the-{slot}-block-is-already-at-these-values", "real",
              {"params": copy.deepcopy(REAL["params"][slot])},
              "an identical resubmission is a Warning with an empty diff — for four of the eight blocks it is the ONLY non-applying path"))
    # endregion 🔖️Params

    # region 🔖️Results
    add(V("replace-job", "realworld", "replace-job-realworld", "🎨️advances-the-orbit-run-from-dense-stereo-into-texturing", "real",
          {"job": copy.deepcopy(REAL["job"]) | {"stage": "texturing", "progress01": q(0.8125), "stageCursor": 12}},
          "a real stage advance carrying its own preview payload forward"))
    add(V("replace-job", "edge", "replace-job-noop", "🔁️warns-that-the-job-record-is-already-at-this-value", "real",
          {"job": copy.deepcopy(REAL["job"])}, "this verb has NO refusal branch at all: the no-op Warning is its only non-applying path"))

    add(V("replace-sparse", "realworld", "replace-sparse-realworld", "✨️swaps-in-a-re-triangulated-sparse-cloud-of-sixteen-points", "real",
          {"sparse": {"points": bases.pack_f32([q(value) for index in range(16) for value in (index * 0.125, index * -0.0625, index * 0.25)]), "colors": bases.pack_u8([(index * 11) % 256 for index in range(48)])}},
          "a re-triangulation replaces the whole buffer; there is no partial-point mutation in this vocabulary"))
    add(V("replace-sparse", "edge", "replace-sparse-noop", "🔁️warns-that-the-sparse-cloud-is-already-this-buffer", "real",
          {"sparse": copy.deepcopy(REAL["results"]["sparse"])}, "the no-op Warning is this verb's only non-applying path"))

    add(V("replace-dense", "realworld", "replace-dense-realworld", "☁️swaps-in-a-denser-fusion-with-confidence-and-classification-lanes", "real",
          {"dense": {"positions": bases.pack_f32([q(value) for index in range(24) for value in (index * 0.0625, index * 0.03125, index * -0.125)]),
                     "colors": bases.pack_u8([(index * 17) % 256 for index in range(72)]),
                     "confidence": bases.pack_f32([q(0.25 + (index % 12) / 16.0) for index in range(24)]),
                     "classification": bases.pack_u8([6 if index % 5 == 0 else 2 for index in range(24)])}},
          "all four dense lanes move together, which is what a fusion rerun produces"))
    add(V("replace-dense", "edge", "replace-dense-noop", "🔁️warns-that-the-dense-cloud-is-already-these-buffers", "real",
          {"dense": copy.deepcopy(REAL["results"]["dense"])}, "the no-op Warning is this verb's only non-applying path"))

    add(V("replace-mesh-result", "realworld", "replace-mesh-result-realworld", "🕸️swaps-the-reconstructed-mesh-for-an-imported-reference-mesh", "real",
          {"mesh": IMPORTED_MESH}, "an imported mesh drops the watertight report the reconstructed one carried"))
    add(V("replace-mesh-result", "refusal", "replace-mesh-result-staged", "🚫️refuses-a-private-reconstruction-staging-mesh-handle", "real",
          {"mesh": STAGED_MESH}, "a `mesh-stage:` artifact id is accepted only by commit-reconstruction"))
    add(V("replace-mesh-result", "edge", "replace-mesh-result-noop", "🔁️warns-that-the-mesh-result-is-already-this-handle", "real",
          {"mesh": copy.deepcopy(REAL["results"]["mesh"])}, "the staging guard is checked BEFORE the no-op, so an identical staged handle is still refused"))

    add(V("replace-trajectory", "realworld", "replace-trajectory-realworld", "🛣️swaps-in-a-three-pose-trajectory-from-a-shorter-solve", "real",
          {"trajectory": {"poses": copy.deepcopy(REAL["results"]["trajectory"]["poses"][:3])}},
          "a shorter solve replaces the whole pose list, positions included"))
    add(V("replace-trajectory", "refusal", "replace-trajectory-absent", "🚫️refuses-to-clear-a-trajectory-the-run-never-produced", "real-cleared",
          {"trajectory": None}, "clearing an already-absent value is an Error against the DOCUMENT id, not a no-op"))
    add(V("replace-trajectory", "edge", "replace-trajectory-clears", "🕳️drops-the-trajectory-a-finished-run-had-produced", "real",
          {"trajectory": None}, "moving a present value to null is admitted; only null-onto-null is refused"))

    add(V("replace-tracks", "realworld", "replace-tracks-realworld", "🏃️swaps-in-two-longer-motion-tracks", "real",
          {"tracks": [{"id": "orbit-track-d", "length": 10, "class": "moving", "meanSpeedMS": q(1.25)}, {"id": "orbit-track-e", "length": 11, "class": "static", "meanSpeedMS": 0.0}]},
          "the track list is replaced wholesale by a motion rerun"))
    add(V("replace-tracks", "edge", "replace-tracks-empty", "🕳️clears-every-motion-track-with-an-empty-list", "real",
          {"tracks": []}, "the empty list is a real value here, not an absence: `tracks` has no null form"))
    add(V("replace-tracks", "edge", "replace-tracks-noop", "🔁️warns-that-the-track-list-is-already-this-value", "real",
          {"tracks": copy.deepcopy(REAL["results"]["tracks"])}, "the no-op Warning is this verb's only non-applying path"))

    add(V("replace-geo-products", "realworld", "replace-geo-products-realworld", "🗺️records-a-dtm-and-an-orthophoto-beside-the-existing-dsm", "real",
          {"geo": {"dsmAssetId": "orbit-dsm", "dtmAssetId": "orbit-dtm", "orthoAssetId": "orbit-ortho"}},
          "a georeferencing pass fills the two slots the survey had left null"))
    add(V("replace-geo-products", "refusal", "replace-geo-products-absent", "🚫️refuses-to-clear-geo-products-the-run-never-produced", "real-cleared",
          {"geo": None}, "clearing an already-absent value is an Error against the DOCUMENT id"))
    add(V("replace-geo-products", "edge", "replace-geo-products-clears", "🧹️clears-the-geo-products-a-finished-run-had-recorded", "real",
          {"geo": None}, "moving a present value to null is admitted"))

    add(V("replace-qc", "realworld", "replace-qc-realworld", "✅️files-a-qc-report-carrying-the-mesh-watertight-block", "real",
          {"qc": {"reprojectionRmsPx": q(0.3125), "gcpCheckpointRmse": q(0.125), "watertight": copy.deepcopy(REAL["results"]["mesh"]["watertight"]),
                  "meanTrackLength": q(7.25), "registeredFrameRatio": 1.0, "denseCoverageRatio": q(0.8125), "warnings": []}},
          "a QC pass after meshing folds the watertight report into the report itself"))
    add(V("replace-qc", "refusal", "replace-qc-absent", "🚫️refuses-to-clear-a-qc-report-the-run-never-produced", "real-cleared",
          {"qc": None}, "clearing an already-absent value is an Error against the DOCUMENT id"))
    add(V("replace-qc", "edge", "replace-qc-clears", "🧹️clears-the-qc-report-a-finished-run-had-recorded", "real",
          {"qc": None}, "moving a present value to null is admitted"))
    # endregion 🔖️Results

    # region 🔖️Commit
    commit_base = {"job": copy.deepcopy(REAL["job"]) | {"stage": "done", "progress01": 1.0, "stageCursor": 17}, "trajectory": None, "mesh": None, "geo": None, "qc": None, "assets": []}
    add(V("commit-reconstruction", "refusal", "commit-reconstruction-sparse", "⭐️rejects-an-unstaged-terminal-sparse-cloud", "real",
          commit_base | {"sparse": copy.deepcopy(REAL["results"]["sparse"])},
          "a plain point buffer names no staged run, so there is nothing to publish"))
    add(V("commit-reconstruction", "refusal", "commit-reconstruction-asset", "🖼️rejects-an-unstaged-terminal-raster-asset", "real",
          commit_base | {"sparse": None, "assets": [{"id": "orbit-frame-10", "asset": {"mime": "image/png", "data": "b3JiaXQtZnJhbWUtMTA=", "width": 320, "height": 240}}]},
          "the asset guard runs after the sparse guard, so a null sparse reaches it"))
    add(V("commit-reconstruction", "refusal", "commit-reconstruction-mesh", "🕸️rejects-an-unstaged-terminal-mesh-handle", "real",
          commit_base | {"sparse": None, "mesh": IMPORTED_MESH},
          "the mesh guard runs last of the three, and reports the mesh's own child id rather than a slot name"))
    # endregion 🔖️Commit

    return rows


# 🧸 Toy payloads the new referential guards moved: the committed toy vector for these two verbs
# addressed a member the document still references, which is now a refusal, so the toy role addresses
# the one member of the same scene the verb accepts. `(case name, payload)` — the case name is
# shortened by the generator exactly like a catalogue row's.
TOY_OVERRIDES: dict[str, tuple[str, dict]] = {
    "delete-stream": ("⏮️removes-the-first-stream-no-ground-control-point-observes", {"id": "stream-a"}),
    "delete-asset": ("🧹️drops-the-spare-raster-no-frame-texture-or-geo-product-names", {"key": "asset-spare"}),
}

TOY_CASES = {
    "create-stream": "🎥️adds-stream-c-458900", "delete-stream": "🚫️removes-stream-b-0f62a7", "change-stream-sync": "⏱️shifts-stream-a-5b442c",
    "add-stream-frame": "🎞️appends-a-third-8ac259", "remove-stream-frame": "🚫️removes-the-last-304bdf", "replace-stream-source": "🧹️clears-the-video-143f2b",
    "create-asset": "🖼️stores-a-new-d56283", "delete-asset": "🗑️removes-asset-a-170889", "create-camera-calibration": "📷️adds-the-cam-c-82c8fb",
    "update-camera-calibration": "🔍️refines-the-cam-0eaef0", "delete-camera-calibration": "🚫️removes-the-cam-f90b89", "create-rig-extrinsic": "🔗️adds-a-rig-2df5df",
    "delete-rig-extrinsic": "✂️drops-the-cam-a-a1f8a2", "update-rig-extrinsic": "📍️retunes-the-cam-4ca5a2", "create-gcp": "📍️adds-gcp-tower-d71a54",
    "delete-gcp": "🚫️removes-gcp-209b7d", "add-gcp-observation": "🔎️adds-the-first-05b1b5", "remove-gcp-observation": "🚫️removes-the-only-f82e64",
    "update-ingest-params": "🔍️tightens-the-499c47", "update-feature-params": "🔎️switches-the-423de9", "update-match-params": "🌳️switches-the-652d03",
    "update-sfm-params": "🎯️switches-the-7f0371", "update-dense-params": "🔬️raises-the-dense-ddb263", "update-mesh-params": "🔳️doubles-the-c245d5",
    "update-motion-params": "🏃️enables-motion-2444a3", "update-geo-params": "🌐️enables-georefere-18a68a", "replace-job": "🎨️advances-the-job-c1e878",
    "replace-sparse": "✨️swaps-in-an-6d9ae4", "replace-dense": "☁️swaps-in-a-two-c688db", "replace-mesh-result": "🕸️swaps-in-an-f23e71",
    "replace-trajectory": "🧹️clears-the-d2f81a", "replace-tracks": "⏸️replaces-the-d40c68", "replace-geo-products": "🗺️adds-the-dtm-and-64d5bb",
    "replace-qc": "📋️records-a-qc-f5caf4",
}

KIND_DIRECTORY = {
    "create-stream": "🌱create-stream", "delete-stream": "🪓delete-stream", "change-stream-sync": "⏱️change-stream-sync",
    "add-stream-frame": "➕add-stream-frame", "remove-stream-frame": "➖remove-stream-frame", "replace-stream-source": "🔁replace-stream-source",
    "create-asset": "🧷create-asset", "delete-asset": "🗞️delete-asset", "create-camera-calibration": "🔭create-camera-calibration",
    "update-camera-calibration": "🛠️update-camera-calibration", "delete-camera-calibration": "🚫delete-camera-calibration",
    "create-rig-extrinsic": "⛓️create-rig-extrinsic", "delete-rig-extrinsic": "✂️delete-rig-extrinsic", "update-rig-extrinsic": "🔩update-rig-extrinsic",
    "create-gcp": "🧿create-gcp", "delete-gcp": "🚮delete-gcp", "add-gcp-observation": "🔎add-gcp-observation", "remove-gcp-observation": "🚷remove-gcp-observation",
    "update-ingest-params": "🥣update-ingest-params", "update-feature-params": "🌠update-feature-params", "update-match-params": "🪢update-match-params",
    "update-sfm-params": "🧮update-sfm-params", "update-dense-params": "🌁update-dense-params", "update-mesh-params": "🕸️update-mesh-params",
    "update-motion-params": "🏎️update-motion-params", "update-geo-params": "🌐update-geo-params", "replace-job": "🏗️replace-job",
    "replace-sparse": "⭐replace-sparse", "replace-dense": "☁️replace-dense", "replace-mesh-result": "🧱replace-mesh-result",
    "replace-trajectory": "🛣️replace-trajectory", "replace-tracks": "🚂replace-tracks", "replace-geo-products": "🗾replace-geo-products",
    "replace-qc": "🧾replace-qc", "commit-reconstruction": "🏁commit-reconstruction",
}


if __name__ == "__main__":
    rows = catalogue()
    print(f"{len(rows)} new vectors + {len(TOY_CASES)} regenerated toy vectors")
    by_role: dict[str, int] = {}
    for row in rows:
        by_role[row.role] = by_role.get(row.role, 0) + 1
    print(by_role)
    missing = sorted(set(KIND_DIRECTORY) - {row.kind for row in rows})
    print("kinds without a new vector:", missing or "none")
