#!/usr/bin/env python3
"""📸️ W2-R architect group: annotates every `s.remodel.remodeling` leaf payload schema input with `x-semio-ui` (design §6,
manifest `$defs/InputUi`), including every field of every nested record. The payload structure is already true to the Rust
types (every committed fixture validates), so only annotations and the certain geographic bounds are added. Terms follow
German photogrammetry usage (Passpunkt, innere/äußere Orientierung, Bündelblockausgleichung, DOM/DGM, Orthofoto); soft
ranges follow the editor's own `setXxxParams` sliders.

    python3 <ticket>/🧪️w2-r-architect-remodeling.py
"""
import json
import os
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")


def L(en, de):
    return {"en": en, "de": de}


def ui(**body):
    return {key: body[key] for key in KEY_ORDER if key in body and body[key] is not None}


def D(en, de):
    return L(en, de)


def text(en, de, group, description=None, widget="text"):
    return ui(widget=widget, role="value", label=L(en, de), description=description, group=group)


def hidden(en, de, group, description=None):
    return ui(widget="hidden", role="value", label=L(en, de), description=description, group=group)


def toggle(en, de, group, description=None):
    return ui(widget="toggle", role="value", label=L(en, de), description=description, group=group)


def stepper(en, de, group, description=None, **facets):
    facets.setdefault("step", 1)
    facets.setdefault("precision", 0)
    return ui(widget="stepper", role="value", label=L(en, de), description=description, group=group, **facets)


def slider(en, de, group, description=None, **facets):
    return ui(widget="slider", role="value", label=L(en, de), description=description, group=group, **facets)


def choice(en, de, group, options, description=None):
    widget = "segmented" if len(options) <= 4 else "select"
    return ui(widget=widget, role="value", label=L(en, de), description=description, options={value: L(*label) for value, label in options.items()}, group=group)


def vector(en, de, group, description=None, **facets):
    return ui(widget="vector", role="value", label=L(en, de), description=description, group=group, **facets)


def block(en, de, group, description=None):
    return ui(role="value", label=L(en, de), description=description, group=group)


def target(en, de, kind, description=None, group="target"):
    ref = {"kind": kind, "domain": "assets", "granularity": "asset"} if kind == "asset" else {"kind": kind}
    return ui(widget="reference", role="target", label=L(en, de), description=description, ref=ref, group=group)


STREAM_KINDS = {"image-sequence": ("Image Sequence", "Bildsequenz"), "video": ("Video", "Video")}
PACKED = D("Packed binary data (base64); written by the reconstruction.", "Gepackte Binärdaten (Base64); von der Rekonstruktion geschrieben.")
DERIVED = D("Derived from the mesh by the reconstruction; not edited by hand.", "Von der Rekonstruktion aus dem Mesh abgeleitet; wird nicht von Hand bearbeitet.")
QC = D("Computed by the reconstruction run; not edited by hand.", "Vom Rekonstruktionslauf berechnet; wird nicht von Hand bearbeitet.")
CHILD = D("Handle of the composed stdio mesh artifact.", "Verweis auf das eingebettete stdio-Mesh-Artefakt.")
STREAM = ("Stream", "Stream", "stream")
CAMERA = ("Camera", "Kamera", "camera")
GCP = ("Ground Control Point", "Passpunkt", "gcp")
ROTATION = vector("Rotation (Quaternion w, x, y, z)", "Rotation (Quaternion w, x, y, z)", "pose", D("Unit quaternion of the camera orientation.", "Einheitsquaternion der Kameraorientierung."), step=0.001)
VIDEO_SOURCE = block("Video Source", "Videoquelle", "media", D("Provenance of a stream imported from a video file; empty for image sequences.", "Herkunft eines aus einer Videodatei importierten Streams; leer bei Bildsequenzen."))
WATERTIGHT = block("Watertightness Report", "Wasserdichtheitsbericht", "report", DERIVED)

SPECS = {
    "ChangeStreamSync.id": target(*STREAM, D("The stream whose synchronisation offset changes.", "Stream, dessen Synchronisationsversatz geändert wird.")),
    "ChangeStreamSync.newSyncOffsetMs": stepper("Sync Offset (ms)", "Synchronisationsversatz (ms)", "timing", D("Time shift that aligns this stream with the other streams.", "Zeitversatz, der diesen Stream mit den anderen Streams synchronisiert."), unit="ms", precision=1),
    "ReplaceDense.dense": block("Dense Point Cloud", "Dichte Punktwolke", "result", D("Packed dense cloud; empty clears it.", "Gepackte dichte Punktwolke; leer entfernt sie.")),
    "DenseCloud.positions": hidden("Positions", "Positionen", "result", D("Packed float32 xyz triples (base64).", "Gepackte float32-xyz-Tripel (Base64).")),
    "DenseCloud.colors": hidden("Colours", "Farben", "result", D("Packed 8-bit RGB triples (base64).", "Gepackte 8-Bit-RGB-Tripel (Base64).")),
    "DenseCloud.confidence": hidden("Confidence", "Konfidenz", "result", D("Packed float32 confidence per point (base64).", "Gepackte float32-Konfidenz je Punkt (Base64).")),
    "DenseCloud.classification": hidden("Classification", "Klassifizierung", "result", D("Packed LAS classification code per point (base64).", "Gepackter LAS-Klassifizierungscode je Punkt (Base64).")),
    "create-rig-extrinsic:CreateRigExtrinsic.extrinsic": block("Rig Extrinsic", "Äußere Orientierung im Rig", "record", D("Pose of one camera relative to the rig origin.", "Lage einer Kamera relativ zum Rig-Ursprung.")),
    "update-rig-extrinsic:UpdateRigExtrinsic.extrinsic": block("Rig Extrinsic", "Äußere Orientierung im Rig", "record", D("Complete rig pose; replaces the pose of the same camera.", "Vollständige Rig-Lage; ersetzt die Lage derselben Kamera.")),
    "RigExtrinsic.cameraId": target(*CAMERA, D("The camera this rig pose belongs to.", "Kamera, zu der diese Rig-Lage gehört.")),
    "RigExtrinsic.rotationWxyz": ROTATION,
    "RigExtrinsic.translationM": vector("Translation (m)", "Translation (m)", "pose", D("Camera offset from the rig origin.", "Versatz der Kamera zum Rig-Ursprung."), unit="m", step=0.001),
    "DeleteRigExtrinsic.cameraId": target(*CAMERA, D("The camera whose rig pose is removed.", "Kamera, deren Rig-Lage entfernt wird.")),
    "AddStreamFrame.id": target(*STREAM, D("The stream that receives the frame.", "Stream, der das Bild erhält.")),
    "AddStreamFrame.frame": block("Frame", "Bild", "record", D("The frame appended to the stream.", "Das an den Stream angehängte Bild.")),
    "AddStreamFrame.kind": choice("Stream Kind", "Streamart", "media", STREAM_KINDS, D("Capture form of the stream the frame belongs to.", "Erfassungsform des Streams, zu dem das Bild gehört.")),
    "FrameRef.index": stepper("Frame Index", "Bildindex", "media"),
    "FrameRef.timestampMs": stepper("Timestamp (ms)", "Zeitstempel (ms)", "timing", D("Media time of the frame.", "Medienzeit des Bildes."), unit="ms", precision=1),
    "FrameRef.assetId": target("Image Asset", "Bild-Asset", "asset", D("The embedded image of this frame.", "Das eingebettete Bild dieses Bildes."), group="media"),
    "RemoveStreamFrame.id": target(*STREAM, D("The stream that loses the frame.", "Stream, aus dem das Bild entfernt wird.")),
    "RemoveStreamFrame.frameIndex": stepper("Frame Index", "Bildindex", "media"),
    "ReplaceSparse.sparse": block("Sparse Point Cloud", "Dünne Punktwolke", "result", D("Packed sparse cloud from bundle adjustment; empty clears it.", "Gepackte dünne Punktwolke aus der Bündelblockausgleichung; leer entfernt sie.")),
    "SparseCloud.points": hidden("Points", "Punkte", "result", D("Packed float32 xyz triples (base64).", "Gepackte float32-xyz-Tripel (Base64).")),
    "SparseCloud.colors": hidden("Colours", "Farben", "result", D("Packed 8-bit RGB triples (base64).", "Gepackte 8-Bit-RGB-Tripel (Base64).")),
    "UpdateDenseParams.params": block("Dense Reconstruction Parameters", "Parameter der dichten Rekonstruktion", "parameters"),
    "DenseParams.resolution": choice("Resolution", "Auflösung", "parameters", {"low": ("Low", "Niedrig"), "medium": ("Medium", "Mittel"), "high": ("High", "Hoch")}),
    "DenseParams.windowRadiusPx": stepper("Window Radius (px)", "Fensterradius (px)", "parameters", unit="px"),
    "DenseParams.minViewConsistency": stepper("Min. View Consistency", "Min. Ansichtskonsistenz", "parameters", D("Views that must agree before a point is kept.", "Anzahl der Ansichten, die übereinstimmen müssen, damit ein Punkt erhalten bleibt.")),
    "DenseParams.confidenceThreshold": slider("Confidence Threshold", "Konfidenzschwelle", "parameters", step=0.01, precision=2, softMin=0, softMax=1),
    "DenseParams.maxPoints": stepper("Max. Points", "Max. Punkte", "parameters", step=1000),
    "UpdateGeoParams.params": block("Georeferencing Parameters", "Georeferenzierungsparameter", "parameters"),
    "GeoParams.enabled": toggle("Georeferencing Enabled", "Georeferenzierung aktiviert", "parameters"),
    "GeoParams.originLon": stepper("Origin Longitude", "Ursprung Längengrad", "origin", D("Longitude of the local origin (WGS 84); empty when not set.", "Längengrad des lokalen Ursprungs (WGS 84); leer, wenn nicht festgelegt."), unit="°", step=0.000001, precision=7),
    "GeoParams.originLat": stepper("Origin Latitude", "Ursprung Breitengrad", "origin", D("Latitude of the local origin (WGS 84); empty when not set.", "Breitengrad des lokalen Ursprungs (WGS 84); leer, wenn nicht festgelegt."), unit="°", step=0.000001, precision=7),
    "GeoParams.originAlt": stepper("Origin Altitude (m)", "Ursprung Höhe (m)", "origin", D("Height of the local origin; empty when not set.", "Höhe des lokalen Ursprungs; leer, wenn nicht festgelegt."), unit="m", step=0.01, precision=2),
    "GeoParams.gsdM": slider("Ground Sample Distance (m)", "Bodenpixelgröße (m)", "parameters", D("Target ground sampling distance of the orthophoto.", "Angestrebte Bodenauflösung des Orthofotos."), unit="m", step=0.01, precision=3, softMin=0.01, softMax=1, scale="log"),
    "GeoParams.dsmCellM": slider("DSM Cell Size (m)", "DOM-Rasterweite (m)", "parameters", D("Grid spacing of the digital surface model.", "Rasterweite des digitalen Oberflächenmodells."), unit="m", step=0.01, precision=2, softMin=0.01, softMax=5, scale="log"),
    "GeoParams.dtmFilterRadiusM": slider("DTM Filter Radius (m)", "DGM-Filterradius (m)", "parameters", D("Radius of the ground filter that derives the digital terrain model.", "Radius des Bodenfilters, der das digitale Geländemodell ableitet."), unit="m", step=0.1, precision=1, softMin=0.1, softMax=10, scale="log"),
    "GeoParams.orthoMaxPx": stepper("Max. Orthophoto Size (px)", "Max. Orthofotogröße (px)", "parameters", unit="px", step=256),
    "UpdateFeatureParams.params": block("Feature Detection Parameters", "Parameter der Merkmalsdetektion", "parameters"),
    "FeatureParams.detector": choice("Detector", "Detektor", "parameters", {"orb": ("ORB", "ORB"), "akaze": ("AKAZE", "AKAZE"), "harris": ("Harris", "Harris")}),
    "FeatureParams.targetCount": stepper("Target Feature Count", "Angestrebte Merkmalsanzahl", "parameters", step=100),
    "FeatureParams.octaves": stepper("Octaves", "Oktaven", "parameters", D("Scale-space octaves searched for features.", "Durchsuchte Oktaven des Skalenraums.")),
    "FeatureParams.edgeThreshold": slider("Edge Threshold", "Kantenschwelle", "parameters", step=0.5, precision=1, softMin=1, softMax=50),
    "CreateStream.stream": block("Stream", "Stream", "record", D("The new media stream with its frames.", "Neuer Medienstream mit seinen Bildern.")),
    "MediaStream.id": text("Stream ID", "Stream-ID", "identity", D("Unique ID of the new stream.", "Eindeutige ID des neuen Streams.")),
    "MediaStream.name": text("Name", "Bezeichnung", "identity"),
    "MediaStream.kind": choice("Stream Kind", "Streamart", "media", STREAM_KINDS),
    "MediaStream.cameraId": target(*CAMERA, D("Camera that captured the stream; empty when unbound.", "Kamera, die den Stream aufgenommen hat; leer, wenn nicht zugeordnet."), group="media"),
    "MediaStream.syncOffsetMs": stepper("Sync Offset (ms)", "Synchronisationsversatz (ms)", "timing", D("Time shift that aligns this stream with the other streams.", "Zeitversatz, der diesen Stream mit den anderen Streams synchronisiert."), unit="ms", precision=1),
    "MediaStream.fpsHint": stepper("Frame Rate Hint (fps)", "Bildrate als Hinweis (fps)", "timing", unit="fps", precision=2),
    "MediaStream.frames": block("Frames", "Bilder", "media"),
    "MediaStream.source": VIDEO_SOURCE,
    "VideoSource.name": text("File Name", "Dateiname", "media"),
    "VideoSource.container": text("Container", "Container", "media", D("e.g. MP4 or AVI.", "z. B. MP4 oder AVI.")),
    "VideoSource.codec": choice("Codec", "Codec", "media", {"avc": ("H.264/AVC", "H.264/AVC"), "hevc": ("H.265/HEVC", "H.265/HEVC"), "vp9": ("VP9", "VP9"), "av1": ("AV1", "AV1"), "mjpeg": ("Motion JPEG", "Motion JPEG"), "unknown": ("Unknown", "Unbekannt")}),
    "VideoSource.durationMs": stepper("Duration (ms)", "Dauer (ms)", "timing", unit="ms", precision=1),
    "VideoSource.frameCount": stepper("Frame Count", "Bildanzahl", "media"),
    "VideoSource.width": stepper("Width (px)", "Breite (px)", "media", unit="px"),
    "VideoSource.height": stepper("Height (px)", "Höhe (px)", "media", unit="px"),
    "CommitReconstruction.sparse": block("Sparse Point Cloud", "Dünne Punktwolke", "result", D("Sparse cloud produced by the run.", "Vom Lauf erzeugte dünne Punktwolke.")),
    "CommitReconstruction.trajectory": block("Camera Trajectory", "Kameratrajektorie", "result", D("Camera poses recovered by the run.", "Vom Lauf rekonstruierte Kameraposen.")),
    "CommitReconstruction.mesh": block("Mesh", "Mesh", "result", D("Reconstructed mesh; empty keeps the stored mesh.", "Rekonstruiertes Mesh; leer behält das gespeicherte Mesh.")),
    "CommitReconstruction.geo": block("Geo Products", "Geoprodukte", "result", D("DSM, DTM and orthophoto rasters produced by the run.", "Vom Lauf erzeugte DOM-, DGM- und Orthofoto-Raster.")),
    "CommitReconstruction.qc": block("Quality Report", "Qualitätsbericht", "result", QC),
    "CommitReconstruction.assets": block("Asset Bindings", "Asset-Zuordnungen", "result", D("Binds asset IDs to complete durable image content.", "Ordnet Asset-IDs vollständigen dauerhaften Bildinhalten zu.")),
    "CameraTrajectory.poses": block("Camera Poses", "Kameraposen", "result"),
    "CameraPosePreview.cameraId": target(*CAMERA, D("The camera this pose belongs to.", "Kamera, zu der diese Pose gehört."), group="pose"),
    "CameraPosePreview.rotationWxyz": ROTATION,
    "CameraPosePreview.translation": vector("Translation", "Translation", "pose", D("Camera position in reconstruction coordinates.", "Kameraposition in Rekonstruktionskoordinaten."), step=0.001),
    "RemodelingMesh.mesh": hidden("Mesh Artifact", "Mesh-Artefakt", "result", CHILD),
    "RemodelingMeshChild.childId": hidden("Child ID", "Kind-ID", "result", CHILD),
    "RemodelingMeshChild.target": hidden("Target Artifact", "Zielartefakt", "result", CHILD),
    "ArtifactRef.artifactId": hidden("Artifact ID", "Artefakt-ID", "result"),
    "ArtifactRef.dialect": hidden("Dialect", "Dialekt", "result"),
    "ArtifactDialect.artifactKind": hidden("Artifact Kind", "Artefaktart", "result"),
    "ArtifactDialect.standard": hidden("Standard", "Standard", "result"),
    "ArtifactDialect.subset": hidden("Subset", "Teilmenge", "result"),
    "RemodelingMesh.source": choice("Mesh Source", "Mesh-Herkunft", "result", {"placeholder": ("Placeholder", "Platzhalter"), "reconstructed": ("Reconstructed", "Rekonstruiert"), "imported": ("Imported", "Importiert")}),
    "RemodelingMesh.textureAssetId": target("Texture Asset", "Textur-Asset", "asset", D("Texture image of the mesh; empty when untextured.", "Texturbild des Mesh; leer, wenn untexturiert."), group="result"),
    "RemodelingMesh.watertight": WATERTIGHT,
    "QcReportSnapshot.watertight": WATERTIGHT,
    "WatertightReportSnapshot.vertexCount": hidden("Vertex Count", "Anzahl der Eckpunkte", "report"),
    "WatertightReportSnapshot.triangleCount": hidden("Triangle Count", "Anzahl der Dreiecke", "report"),
    "WatertightReportSnapshot.boundaryEdgeCount": hidden("Boundary Edges", "Randkanten", "report"),
    "WatertightReportSnapshot.boundaryLoopCount": hidden("Boundary Loops", "Randschleifen", "report"),
    "WatertightReportSnapshot.nonManifoldEdgeCount": hidden("Non-Manifold Edges", "Nicht-mannigfaltige Kanten", "report"),
    "WatertightReportSnapshot.nonManifoldVertexCount": hidden("Non-Manifold Vertices", "Nicht-mannigfaltige Eckpunkte", "report"),
    "WatertightReportSnapshot.connectedComponents": hidden("Connected Components", "Zusammenhangskomponenten", "report"),
    "WatertightReportSnapshot.consistentlyOriented": hidden("Consistently Oriented", "Konsistent orientiert", "report"),
    "WatertightReportSnapshot.eulerCharacteristic": hidden("Euler Characteristic", "Euler-Charakteristik", "report"),
    "WatertightReportSnapshot.genus": hidden("Genus", "Geschlecht", "report", D("Topological genus of a closed 2-manifold.", "Topologisches Geschlecht einer geschlossenen 2-Mannigfaltigkeit.")),
    "WatertightReportSnapshot.signedVolume": hidden("Signed Volume", "Vorzeichenbehaftetes Volumen", "report"),
    "WatertightReportSnapshot.selfIntersectionPairs": hidden("Self-Intersection Pairs", "Selbstüberschneidungspaare", "report"),
    "WatertightReportSnapshot.closedFallbackUsed": hidden("Closing Fallback Used", "Schließ-Rückfall verwendet", "report"),
    "WatertightReportSnapshot.isClosed": hidden("Closed", "Geschlossen", "report"),
    "WatertightReportSnapshot.isTwoManifold": hidden("2-Manifold", "2-Mannigfaltig", "report"),
    "WatertightReportSnapshot.isWatertight": hidden("Watertight", "Wasserdicht", "report"),
    "GeoProducts.dsmAssetId": target("DSM Raster", "DOM-Raster", "asset", D("Digital surface model raster.", "Raster des digitalen Oberflächenmodells."), group="result"),
    "GeoProducts.dtmAssetId": target("DTM Raster", "DGM-Raster", "asset", D("Digital terrain model raster.", "Raster des digitalen Geländemodells."), group="result"),
    "GeoProducts.orthoAssetId": target("Orthophoto", "Orthofoto", "asset", D("True orthophoto raster.", "Raster des Orthofotos."), group="result"),
    "QcReportSnapshot.reprojectionRmsPx": hidden("Reprojection RMS (px)", "RMS des Rückprojektionsfehlers (px)", "report", QC),
    "QcReportSnapshot.gcpCheckpointRmse": hidden("Checkpoint RMSE", "RMSE der Kontrollpunkte", "report", D("Root mean square error at the ground control checkpoints.", "Mittlerer quadratischer Fehler an den Kontrollpunkten.")),
    "QcReportSnapshot.meanTrackLength": hidden("Mean Track Length", "Mittlere Spurlänge", "report", QC),
    "QcReportSnapshot.registeredFrameRatio": hidden("Registered Frame Ratio", "Anteil registrierter Bilder", "report", QC),
    "QcReportSnapshot.denseCoverageRatio": hidden("Dense Coverage Ratio", "Abdeckungsgrad der dichten Punktwolke", "report", QC),
    "QcReportSnapshot.warnings": hidden("Warnings", "Warnungen", "report", QC),
    "ReconstructionAssetCommit.id": target("Asset", "Asset", "asset", D("The asset that is bound.", "Das zugeordnete Asset."), group="result"),
    "ReconstructionAssetCommit.contentId": target("Content", "Inhalt", "content", D("Durable content the asset binds to; empty leaves it unbound.", "Dauerhafter Inhalt, an den das Asset gebunden wird; leer lässt es ungebunden."), group="result"),
    "UpdateMotionParams.params": block("Motion Tracking Parameters", "Parameter der Bewegungsverfolgung", "parameters"),
    "MotionParams.enabled": toggle("Motion Tracking Enabled", "Bewegungsverfolgung aktiviert", "parameters"),
    "MotionParams.maxTracks": stepper("Max. Tracks", "Max. Spuren", "parameters"),
    "MotionParams.trackWindowPx": stepper("Track Window (px)", "Spurfenster (px)", "parameters", unit="px"),
    "MotionParams.minTrackQuality": slider("Min. Track Quality", "Min. Spurqualität", "parameters", step=0.01, precision=2, softMin=0, softMax=1),
    "MotionParams.minTrackLengthFrames": stepper("Min. Track Length (frames)", "Min. Spurlänge (Bilder)", "parameters"),
    "AppendContent.contentId": target("Content", "Inhalt", "content", D("Durable content the chunks are appended to.", "Dauerhafter Inhalt, an den die Blöcke angehängt werden.")),
    "AppendContent.kind": choice("Content Kind", "Inhaltsart", "media", {"sparse": ("Sparse Cloud", "Dünne Punktwolke"), "mesh": ("Mesh", "Mesh"), "image": ("Image", "Bild")}),
    "AppendContent.mime": text("Media Type", "Medientyp", "media", D("e.g. image/jpeg; empty for non-image content.", "z. B. image/jpeg; leer bei Inhalten, die keine Bilder sind.")),
    "AppendContent.width": stepper("Width (px)", "Breite (px)", "media", unit="px"),
    "AppendContent.height": stepper("Height (px)", "Höhe (px)", "media", unit="px"),
    "AppendContent.first": stepper("First Leaf Index", "Erster Blockindex", "media", D("Leaf index where the first chunk is placed.", "Blockindex, an dem der erste Block abgelegt wird.")),
    "AppendContent.chunks": hidden("Chunks", "Blöcke", "media", D("Base64-encoded raw leaves.", "Base64-kodierte Rohblöcke.")),
    "ReplaceStreamSource.id": target(*STREAM, D("The stream whose video provenance changes.", "Stream, dessen Videoherkunft geändert wird.")),
    "ReplaceStreamSource.source": VIDEO_SOURCE,
    "AddGcpObservation.id": target(*GCP, D("The ground control point that is measured.", "Passpunkt, der gemessen wird.")),
    "AddGcpObservation.observation": block("Observation", "Passpunktmessung", "record", D("Image position of the ground control point in one frame.", "Bildposition des Passpunkts in einem Bild.")),
    "GcpObservation.streamId": target(*STREAM, D("The stream of the measured frame.", "Stream des gemessenen Bildes."), group="media"),
    "GcpObservation.frameIndex": stepper("Frame Index", "Bildindex", "media"),
    "GcpObservation.pixel": vector("Pixel Position (px)", "Bildposition (px)", "geometry", unit="px", step=0.1),
    "RemoveContent.contentId": target("Content", "Inhalt", "content", D("Durable content that is truncated.", "Dauerhafter Inhalt, der gekürzt wird.")),
    "RemoveContent.from": stepper("From Leaf Index", "Ab Blockindex", "media", D("Leaves from this index on are removed.", "Blöcke ab diesem Index werden entfernt.")),
    "create-camera-calibration:CreateCameraCalibration.camera": block("Camera Calibration", "Kamerakalibrierung", "record", D("Interior orientation and distortion of a new camera.", "Innere Orientierung und Verzeichnung einer neuen Kamera.")),
    "update-camera-calibration:UpdateCameraCalibration.camera": block("Camera Calibration", "Kamerakalibrierung", "record", D("Complete calibration; replaces the camera with the same ID.", "Vollständige Kalibrierung; ersetzt die Kamera mit derselben ID.")),
    "create-camera-calibration:CameraCalibration.id": text("Camera ID", "Kamera-ID", "identity", D("Unique ID of the new camera.", "Eindeutige ID der neuen Kamera.")),
    "update-camera-calibration:CameraCalibration.id": target(*CAMERA, D("The camera this calibration replaces.", "Kamera, deren Kalibrierung ersetzt wird."), group="identity"),
    "CameraCalibration.label": text("Label", "Bezeichnung", "identity"),
    "CameraCalibration.model": text("Camera Model", "Kameramodell", "intrinsics", D("pinhole, brownConrady or fisheye; decides which distortion coefficients apply.", "pinhole, brownConrady oder fisheye; bestimmt, welche Verzeichnungskoeffizienten gelten.")),
    "CameraCalibration.fx": stepper("Focal Length x (px)", "Brennweite x (px)", "intrinsics", unit="px", precision=2),
    "CameraCalibration.fy": stepper("Focal Length y (px)", "Brennweite y (px)", "intrinsics", unit="px", precision=2),
    "CameraCalibration.cx": stepper("Principal Point x (px)", "Hauptpunkt x (px)", "intrinsics", unit="px", precision=2),
    "CameraCalibration.cy": stepper("Principal Point y (px)", "Hauptpunkt y (px)", "intrinsics", unit="px", precision=2),
    "CameraCalibration.skew": stepper("Skew", "Scherung", "intrinsics", step=0.001, precision=4),
    "CameraCalibration.distortion": block("Distortion [k1, k2, k3, p1, p2]", "Verzeichnung [k1, k2, k3, p1, p2]", "intrinsics", D("Radial (k) and tangential (p) distortion coefficients.", "Radiale (k) und tangentiale (p) Verzeichnungskoeffizienten.")),
    "CameraCalibration.rmsReprojectionPx": stepper("Reprojection RMS (px)", "RMS des Rückprojektionsfehlers (px)", "quality", unit="px", step=0.01, precision=3),
    "CameraCalibration.locked": toggle("Locked", "Gesperrt", "intrinsics"),
    "UpdateMeshParams.params": block("Meshing Parameters", "Vermaschungsparameter", "parameters"),
    "MeshParams.tsdfVoxelSizeMm": slider("TSDF Voxel Size (mm)", "TSDF-Voxelgröße (mm)", "parameters", unit="mm", step=0.5, precision=1, softMin=1, softMax=20),
    "MeshParams.tsdfTruncationMm": slider("TSDF Truncation (mm)", "TSDF-Kappungsabstand (mm)", "parameters", unit="mm", step=0.5, precision=1, softMin=2, softMax=60),
    "MeshParams.decimateTargetTriangles": stepper("Target Triangles (Decimation)", "Ziel-Dreiecke (Dezimierung)", "parameters", step=1000),
    "MeshParams.smoothingIterations": stepper("Smoothing Iterations", "Glättungsiterationen", "parameters"),
    "MeshParams.textureEnabled": toggle("Texture Enabled", "Textur aktiviert", "parameters"),
    "MeshParams.textureSize": stepper("Texture Size (px)", "Texturgröße (px)", "parameters", unit="px", step=1024, snaps=[1024, 2048, 4096]),
    "MeshParams.guaranteeWatertight": toggle("Guarantee Watertight", "Wasserdichtheit garantieren", "parameters"),
    "MeshParams.holeFillMaxBoundaryVerts": stepper("Hole Fill Max. Boundary Vertices", "Max. Randpunkte für Lochfüllung", "parameters"),
    "MeshParams.selfIntersectionCheck": toggle("Self-Intersection Check", "Selbstüberschneidungsprüfung", "parameters"),
    "DeleteAsset.key": target("Asset", "Asset", "asset", D("The asset to delete.", "Zu löschendes Asset.")),
    "ReplaceGeoProducts.geo": block("Geo Products", "Geoprodukte", "result", D("DSM, DTM and orthophoto rasters; empty clears them.", "DOM-, DGM- und Orthofoto-Raster; leer entfernt sie.")),
    "ReplaceTracks.tracks": block("Motion Tracks", "Bewegungsspuren", "result"),
    "MotionTrackSummary.id": text("Track ID", "Spur-ID", "identity"),
    "MotionTrackSummary.length": stepper("Length (frames)", "Länge (Bilder)", "result"),
    "MotionTrackSummary.class": choice("Track Class", "Spurklasse", "result", {"static": ("Static", "Statisch"), "moving": ("Moving", "Bewegt")}),
    "MotionTrackSummary.meanSpeedMS": stepper("Mean Speed (m/s)", "Mittlere Geschwindigkeit (m/s)", "result", unit="m/s", step=0.01, precision=2),
    "DeleteCameraCalibration.cameraId": target(*CAMERA, D("The camera whose calibration is deleted.", "Kamera, deren Kalibrierung gelöscht wird.")),
    "DeleteGcp.id": target(*GCP, D("The ground control point to delete.", "Zu löschender Passpunkt.")),
    "RemoveGcpObservation.id": target(*GCP, D("The ground control point that loses the measurement.", "Passpunkt, dessen Messung entfernt wird.")),
    "RemoveGcpObservation.observationIndex": stepper("Observation Index", "Messungsindex", "media"),
    "ReplaceTrajectory.trajectory": block("Camera Trajectory", "Kameratrajektorie", "result", D("Recovered camera poses; empty clears the trajectory.", "Rekonstruierte Kameraposen; leer entfernt die Trajektorie.")),
    "UpdateIngestParams.params": block("Ingest Parameters", "Ingest-Parameter", "parameters"),
    "IngestParams.frameSampleStride": stepper("Frame Sample Stride", "Bild-Abtastschrittweite", "parameters", D("Keeps every n-th frame.", "Übernimmt jedes n-te Bild.")),
    "IngestParams.maxFrames": stepper("Max. Frames", "Max. Bilder", "parameters"),
    "IngestParams.downscaleLongEdgePx": stepper("Downscale Long Edge (px)", "Verkleinerung der langen Kante (px)", "parameters", unit="px", step=100),
    "IngestParams.minSharpness": slider("Min. Sharpness", "Min. Schärfe", "parameters", D("Drops a frame whose sharpness is below this fraction of the recent median.", "Verwirft ein Bild, dessen Schärfe unter diesem Anteil des gleitenden Medians liegt."), step=0.01, precision=2, softMin=0, softMax=1),
    "UpdateSfmParams.params": block("Structure-from-Motion Parameters", "Structure-from-Motion-Parameter", "parameters"),
    "SfmParams.ransacIterations": stepper("RANSAC Iterations", "RANSAC-Iterationen", "parameters", step=100),
    "SfmParams.ransacThresholdPx": slider("RANSAC Threshold (px)", "RANSAC-Schwelle (px)", "parameters", unit="px", step=0.1, precision=1, softMin=0.1, softMax=10),
    "SfmParams.minTrackLength": stepper("Min. Track Length", "Min. Spurlänge", "parameters"),
    "SfmParams.baMaxIterations": stepper("Bundle Adjustment Max. Iterations", "Max. Iterationen der Bündelblockausgleichung", "parameters"),
    "SfmParams.robustLoss": choice("Robust Loss", "Robuste Verlustfunktion", "parameters", {"l2": ("L2", "L2"), "huber": ("Huber", "Huber"), "cauchy": ("Cauchy", "Cauchy")}),
    "SfmParams.huberDeltaPx": slider("Huber Delta (px)", "Huber-Delta (px)", "parameters", unit="px", step=0.1, precision=1, softMin=0.1, softMax=10),
    "ReplaceMeshResult.mesh": block("Mesh Result", "Mesh-Ergebnis", "result", D("The complete mesh result; replaces the stored one.", "Vollständiges Mesh-Ergebnis; ersetzt das gespeicherte.")),
    "CreateAsset.key": text("Asset Key", "Asset-Schlüssel", "identity", D("Unique key of the new asset.", "Eindeutiger Schlüssel des neuen Assets.")),
    "CreateAsset.asset": block("Image Asset", "Bild-Asset", "record"),
    "ImageAsset.mime": text("Media Type", "Medientyp", "media", D("e.g. image/jpeg or image/png.", "z. B. image/jpeg oder image/png.")),
    "ImageAsset.data": hidden("Image Data", "Bilddaten", "media", D("Base64-encoded image bytes.", "Base64-kodierte Bilddaten.")),
    "ImageAsset.width": stepper("Width (px)", "Breite (px)", "media", unit="px"),
    "ImageAsset.height": stepper("Height (px)", "Höhe (px)", "media", unit="px"),
    "ReplaceQc.qc": block("Quality Report", "Qualitätsbericht", "result", D("Quality metrics of the last run; empty clears them.", "Qualitätskennzahlen des letzten Laufs; leer entfernt sie.")),
    "CreateGcp.gcp": block("Ground Control Point", "Passpunkt", "record", D("A surveyed point used to georeference the reconstruction.", "Eingemessener Punkt zur Georeferenzierung der Rekonstruktion.")),
    "GroundControlPoint.id": text("Point ID", "Punkt-ID", "identity", D("Unique ID of the new ground control point.", "Eindeutige ID des neuen Passpunkts.")),
    "GroundControlPoint.name": text("Name", "Bezeichnung", "identity"),
    "GroundControlPoint.worldPosition": vector("World Position", "Weltkoordinaten", "geometry", D("Surveyed position in the project coordinate system.", "Eingemessene Lage im Projektkoordinatensystem."), step=0.001),
    "GroundControlPoint.observations": block("Observations", "Passpunktmessungen", "media"),
    "DeleteStream.id": target(*STREAM, D("The stream to delete.", "Zu löschender Stream.")),
    "UpdateMatchParams.params": block("Feature Matching Parameters", "Parameter der Merkmalszuordnung", "parameters"),
    "MatchParams.matcher": choice("Matcher", "Zuordnungsverfahren", "parameters", {"brute-force": ("Brute Force", "Brute Force"), "kd-tree": ("KD-Tree", "KD-Baum")}),
    "MatchParams.ratioTest": slider("Ratio Test", "Verhältnistest", "parameters", D("Lowe's ratio: keeps a match only when it clearly beats the second best.", "Lowe-Verhältnis: behält eine Zuordnung nur, wenn sie die zweitbeste deutlich übertrifft."), step=0.01, precision=2, softMin=0.1, softMax=1),
    "MatchParams.crossCheck": toggle("Cross Check", "Kreuzprüfung", "parameters"),
    "MatchParams.sequentialWindow": stepper("Sequential Window", "Sequenzielles Fenster", "parameters", D("Neighbouring frames each frame is matched against.", "Anzahl benachbarter Bilder, mit denen jedes Bild abgeglichen wird.")),
    "MatchParams.maxPairsPerFrame": stepper("Max. Pairs per Frame", "Max. Paare pro Bild", "parameters"),
    "MatchParams.loopClosure": toggle("Loop Closure", "Schleifenschluss", "parameters"),
}
BOUNDS = {"GeoParams.originLon": {"minimum": -180, "maximum": 180}, "GeoParams.originLat": {"minimum": -90, "maximum": 90}}
USED = set()


def spec(leaf, title, key):
    for name in ("%s:%s.%s" % (leaf, title, key), "%s.%s" % (title, key)):
        if name in SPECS:
            USED.add(name)
            return name, SPECS[name]
    raise SystemExit("📸️ %s: no annotation for %s.%s" % (leaf, title, key))


def objects(node):
    if not isinstance(node, dict):
        return
    if "properties" in node:
        yield node
    for key in ("anyOf", "oneOf"):
        for branch in node.get(key, []):
            yield from objects(branch)
    if "items" in node:
        yield from objects(node["items"])


def annotate(node, leaf):
    title = node.get("title")
    properties = {}
    for index, (key, value) in enumerate(node["properties"].items()):
        name, annotation = spec(leaf, title, key)
        body = {field: item for field, item in value.items() if field != "x-semio-ui"}
        body.update(BOUNDS.get(name.split(":")[-1], {}))
        for child in objects(body):
            annotate(child, leaf)
        body["x-semio-ui"] = dict(annotation, order=(index + 1) * 10)
        properties[key] = body
    node["properties"] = properties


def main():
    count = 0
    for directory in sorted(os.listdir(ROOT)):
        path = os.path.join(ROOT, directory, "🧬️schema", "🔣️.json")
        if not os.path.exists(path):
            continue
        schema = json.load(open(path, encoding="utf-8"))
        leaf = schema["$id"].split("/mutation/")[1].split("/")[0]
        annotate(schema, leaf)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
        count += 1
    unused = sorted(set(SPECS) - USED)
    if unused:
        raise SystemExit("📸️ unused annotations: %s" % ", ".join(unused))
    print("annotated %d remodeling leaf schemas" % count)


if __name__ == "__main__":
    sys.exit(main())
