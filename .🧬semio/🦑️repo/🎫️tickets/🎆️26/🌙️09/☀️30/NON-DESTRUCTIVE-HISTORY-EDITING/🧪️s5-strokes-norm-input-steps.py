#!/usr/bin/env python3
"""🎚️ Declares the step (and, where the norm tabulates them, the snapping points) of every interactive numeric mutation
input of the norm, remodel, process and wfc plugins — design §22.8 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING.

usage: python3 🧪️s5-strokes-norm-input-steps.py census|declare|check <gate --json output> <plugin root> [<plugin root> …]

The gate (`schema mutation-inputs --under <plugin> --json`) names a leaf and an RFC 6901 pointer over its payload per
`numericUndeclared` finding. This script walks the leaf schema through `$ref` (local, or by `$id` among the given roots),
`properties`, `items` and nullable unions to the ONE property that owns the fact — almost always a snapshot `$defs`
entry several leaves share — and merges the declaration into its `x-semio-ui`. `step` is in the STORED unit.

A step comes from, in this order: the property's own row in `FIELD` (norm-tabulated values, quantities whose practice
differs from their class), the artifact's class row in `ARTIFACT_CLASS`, the quantity class in `CLASS` keyed by
(unit, displayUnit, precision). A property with no rule fails the run: nothing is written.

  census   one TSV row per distinct property (read-only)
  declare  writes the declarations; files are re-printed in their own style (asserted byte-identical before the edit)
  check    exit 0 when no finding is pending and every `FIELD` row is on disk
"""
import collections
import json
import os
import re
import sys

MM = 0.001
KN = 1000
MPA = 1_000_000


def mm(*values):
    return [round(value * MM, 9) for value in values]


def mpa(*values):
    return [value * MPA for value in values]


#region 🔖️Class
# (unit, displayUnit, precision) → step in the stored unit. One row per quantity class as the norm documents state it.
CLASS = {
    ("m", "mm", 1): 1 * MM,
    ("m", None, 0): 10,
    ("m", None, 1): 0.5,
    ("m", None, 2): 0.05,
    ("m", None, 3): 0.01,
    ("m²", None, 2): 0.5,
    ("m²", "cm²", 1): 1e-4,
    ("m²", "cm²", 2): 1e-5,
    ("m²/m", "cm²/m", 2): 1e-5,
    ("m³", None, 0): 10,
    ("m³", "cm³", 0): 1e-6,
    ("m⁴", "cm⁴", 0): 1e-8,
    ("m⁴", "cm⁴", 1): 1e-9,
    ("m⁶", "cm⁶", 0): 1e-10,
    ("N", "kN", 2): 1 * KN,
    ("N·m", "kN·m", 2): 1 * KN,
    ("N/m", "kN/m", 2): 100,
    ("N/m", "kN/m", 0): 1000 * KN,
    ("N/m³", "kN/m³", 1): 500,
    ("Pa", "N/mm²", 1): 1 * MPA,
    ("Pa", "kN/m²", 2): 500,
    ("Pa", None, 0): 50,
    (None, None, 2): 0.01,
    (None, None, 3): 0.001,
    ("1", None, 2): 0.01,
    ("1", "%", 0): 0.01,
    ("1", "%", 1): 0.001,
    ("1", "%", 2): 0.0001,
    ("°", None, 0): 1,
    ("°", None, 1): 0.5,
    ("°C", None, 0): 10,
    ("°C", None, 1): 0.5,
    ("a", None, 0): 1,
    ("s", None, 2): 0.01,
    ("s", "min", 0): 60,
    ("h", None, 1): 0.5,
    ("Hz", None, 2): 0.1,
    ("rad", None, 4): 0.0001,
    ("m/s", None, 1): 0.5,
    ("m/s²", None, 2): 0.05,
    ("kg", None, 0): 100,
    ("kg", None, 1): 0.5,
    ("kg/h", None, 2): 0.1,
    ("kg/m", None, 1): 0.5,
    ("kg/m²", None, 1): 0.5,
    ("kg/m³", None, 0): 10,
    ("1/m", None, 0): 5,
    ("1/m²", None, 2): 0.1,
    ("W", None, 0): 10,
    ("W/m²", None, 1): 0.5,
    ("W/(m·K)", None, 3): 0.005,
    ("J/(kg·K)", None, 0): 50,
    ("kWh/a", None, 0): 10,
    ("kWh/(Person·a)", None, 0): 10,
    ("m³/h", None, 0): 10,
    ("m³/s", None, 3): 0.01,
}

# Timber and aluminium members carry forces an order of magnitude below steel and concrete ones: 0.1 kN, 0.1 kN·m.
ARTIFACT_CLASS = {
    ("en1995", ("N", "kN", 2)): 100,
    ("en1995", ("N·m", "kN·m", 2)): 100,
    ("en1999", ("N", "kN", 2)): 100,
    ("en1999", ("N·m", "kN·m", 2)): 100,
}
#endregion 🔖️Class

#region 🔖️Field
FIRE_CLASSES_S = [900, 1800, 2700, 3600, 5400, 7200, 10800, 14400]
WORKING_LIFE_A = [10, 25, 50, 100]
STEEL_FY = mpa(235, 275, 355, 420, 460)
CONCRETE_FCK = mpa(20, 25, 30, 35, 40, 45, 50, 55, 60)
FATIGUE_CYCLES = [2_000_000, 5_000_000, 100_000_000]
DEFLECTION_RATIOS = [150, 200, 250, 300, 350, 400, 500]
BOLTS_STEEL = mm(12, 16, 20, 22, 24, 27, 30, 36)
THIN_SHEET = {"step": 0.01 * MM, "precision": 2, "snaps": mm(0.75, 0.88, 1.0, 1.25, 1.5)}
ROLLED_THICKNESS = {"step": 0.1 * MM}
SPACING = {"step": 5 * MM}
GRADE_FY = {"step": 5 * MPA, "snaps": STEEL_FY}
TENSILE = {"step": 10 * MPA}
K_SIGMA = {"step": 0.01, "snaps": [0.43, 4.0, 7.81, 23.9]}
PSI = {"step": 0.05, "snaps": [-1, 0, 1]}
FCK = {"step": 1 * MPA, "snaps": CONCRETE_FCK}
CYCLES = {"step": 10_000, "snaps": FATIGUE_CYCLES}
RATIO_LIMIT = {"step": 50, "snaps": DEFLECTION_RATIOS}
FIRE = {"step": 60, "snaps": FIRE_CLASSES_S}
LIFE = {"step": 1, "snaps": WORKING_LIFE_A}
COUNT = {"step": 1}


def length_m(en, de, step=0.05):
    return {"label": {"en": en, "de": de}, "widget": "stepper", "unit": "m", "precision": 2, "step": step}


def length_mm(en, de, step=1 * MM):
    return {"label": {"en": en, "de": de}, "widget": "stepper", "unit": "m", "displayUnit": "mm", "displayFactor": 1000, "precision": 1, "step": step}


def count(en, de, step=1, **more):
    return {"label": {"en": en, "de": de}, "widget": "stepper", "role": "value", "step": step, **more}


# <artifact>/<S|A|L:leaf>#<path> → the keys merged into the property's `x-semio-ui` (`None` removes a key).
# S: snapshot schema `$defs`, A: artifact schema `$defs`, L: a leaf's own payload property.
FIELD = {
    # DIN EN 16798 / DIN V 18599 / DIN 4108
    "din18599/S#EnvelopeElement.orientationDeg": {"step": 1, "snaps": [0, 45, 90, 135, 180, 225, 270, 315]},
    "din18599/S#EnvelopeElement.tiltDeg": {"step": 1, "snaps": [0, 30, 45, 60, 90]},
    "din18599/S#CoolingPlant.eer": {"step": 0.1},
    "din18599/S#Renewables.pvEfficiency": {"step": 0.005},
    # EN 1990
    "en1990/L:change-beta-computed#.newBetaComputed": {"step": 0.01, "snaps": [3.3, 3.8, 4.2, 4.3, 4.7, 5.2]},
    "en1990/L:change-design-working-life-years#.newDesignWorkingLifeYears": LIFE,
    "en1990/L:change-reference-period-years#.newReferencePeriodYears": {"step": 1, "snaps": [1, 50]},
    "en1990/S#BridgeSls.deckAccelerationLimit": {"step": 0.05, "snaps": [3.5, 5.0]},
    "en1990/S#Member.deflectionLimitRatio": RATIO_LIMIT,
    # EN 1991
    "en1991/S#FloorArea.assumedQkConcentrated": {"step": 500},
    "en1991/S#FloorArea.assumedPartitions": {"step": 100, "snaps": [500, 800, 1200]},
    "en1991/S#RoofArea.pitchDeg": {"step": 1, "snaps": [5, 15, 30, 45, 60, 75]},
    "en1991/S#RoofArea.cE": {"step": 0.1, "snaps": [0.8, 1.0, 1.2]},
    "en1991/S#RoofArea.cT": {"step": 0.05},
    "en1991/S#WindFace.z": length_m("Reference height z", "Bezugshöhe z", 0.5),
    "en1991/S#WindFace.cPe10": {"step": 0.05},
    "en1991/S#WindFace.cPe1": {"step": 0.05},
    "en1991/S#WindFace.cPi": {"step": 0.05},
    "en1991/S#WindFace.loadedArea": {"step": 0.5, "snaps": [1, 10]},
    # EN 1992
    "en1992/S#Anchor.fUk": {"step": 10 * MPA, "snaps": mpa(400, 500, 700, 800, 1000)},
    "en1992/S#Anchor.d": {"step": 1 * MM, "snaps": mm(6, 8, 10, 12, 16, 20, 24, 27, 30)},
    "en1992/S#Anchor.c1": SPACING,
    "en1992/S#BarLayer.anchorageLength": {"step": 10 * MM},
    "en1992/S#BarLayer.lapLength": {"step": 10 * MM},
    "en1992/S#BarLayer.aggregateSize": {"step": 1 * MM, "snaps": mm(8, 16, 22, 32)},
    "en1992/S#Stirrups.diameter": {"step": 1 * MM, "snaps": mm(6, 8, 10, 12, 14, 16)},
    "en1992/S#PunchingSpec.columnWidth": {"step": 10 * MM},
    "en1992/S#PunchingSpec.columnDepth": {"step": 10 * MM},
    "en1992/S#RcMember.hdOverH": {"step": 0.5, "snaps": [5, 35]},
    "en1992/S#RcMember.liquidFCtEff": {"step": 0.1 * MPA},
    # EN 1993
    "en1993/S#ColdFormedMember.thickness": THIN_SHEET,
    "en1993/S#ColdFormedMember.kSigma": K_SIGMA,
    "en1993/S#ColdFormedMember.psi": PSI,
    "en1993/S#ColdFormedMember.fy": {"step": 5 * MPA, "snaps": mpa(220, 250, 280, 320, 350)},
    "en1993/S#CraneRunway.webThickness": ROLLED_THICKNESS,
    "en1993/S#CraneRunway.fy": GRADE_FY,
    "en1993/S#FatigueBand.cycles": CYCLES,
    "en1993/S#SteelJoint.boltDiameter": {"step": 1 * MM, "snaps": BOLTS_STEEL},
    "en1993/S#SteelJoint.pitch": SPACING,
    "en1993/S#SteelJoint.gauge": SPACING,
    "en1993/S#SteelJoint.endDistance": SPACING,
    "en1993/S#SteelJoint.edgeDistance": SPACING,
    "en1993/S#SteelJoint.plateFu": TENSILE,
    "en1993/S#SteelJoint.weldThroat": {"step": 0.5 * MM},
    "en1993/S#SteelJoint.weldLength": SPACING,
    "en1993/S#SteelJoint.weldFu": TENSILE,
    "en1993/S#SteelJoint.frictionMu": {"step": 0.01, "snaps": [0.2, 0.3, 0.4, 0.5]},
    "en1993/S#SteelJoint.slipFactorKs": {"step": 0.01, "snaps": [0.63, 0.7, 0.76, 0.85, 1.0]},
    "en1993/S#SteelMaterial.fy": GRADE_FY,
    "en1993/S#SteelMaterial.fu": TENSILE,
    "en1993/S#SteelMaterial.eModulus": {"step": 1000 * MPA, "snaps": mpa(210_000)},
    "en1993/S#SteelMaterial.gModulus": {"step": 1000 * MPA, "snaps": mpa(81_000)},
    "en1993/S#SteelMember.endMomentRatioPsi": PSI,
    "en1993/S#SteelMember.deflectionLimitRatio": RATIO_LIMIT,
    "en1993/S#PlatedPanel.a": {"step": 10 * MM},
    "en1993/S#PlatedPanel.b": {"step": 10 * MM},
    "en1993/S#PlatedPanel.fy": GRADE_FY,
    "en1993/S#PlatedPanel.kSigma": K_SIGMA,
    "en1993/S#SteelSection.tw": ROLLED_THICKNESS,
    "en1993/S#SteelSection.tf": ROLLED_THICKNESS,
    "en1993/S#SiloShell.radius": length_m("Shell radius r", "Schalenradius r"),
    "en1993/S#SiloShell.fy": GRADE_FY,
    # EN 1994
    "en1994/S#SteelSection.twM": ROLLED_THICKNESS,
    "en1994/S#SteelSection.tfM": ROLLED_THICKNESS,
    "en1994/S#CompositeBeam.concreteFCkPa": FCK,
    "en1994/S#CompositeBeam.concreteECmPa": {"step": 1000 * MPA},
    "en1994/S#ProfiledSheeting.thicknessM": THIN_SHEET,
    "en1994/S#HeadedStuds.heightM": SPACING,
    "en1994/S#CompositeBeam.barSpacingM": SPACING,
    "en1994/S#CompositeBeam.wkLimitM": {"step": 0.1 * MM, "snaps": mm(0.2, 0.3, 0.4)},
    "en1994/S#CompositeBeam.nCycles": CYCLES,
    "en1994/S#CompositeColumn.wallThicknessM": ROLLED_THICKNESS,
    "en1994/S#CompositeColumn.concreteFCkPa": FCK,
    "en1994/S#CompositeColumn.reinforcementFYkPa": {"step": 10 * MPA, "snaps": mpa(400, 500, 600)},
    "en1994/S#CompositeSlab.fCkPa": FCK,
    "en1994/S#CompositeSlab.mFactor": {"step": 1},
    # EN 1995
    "en1995/S#TimberConnection.diameterM": {"step": 0.1 * MM},
    "en1995/S#TimberConnection.spacingM": SPACING,
    "en1995/S#TimberConnection.edgeDistanceM": SPACING,
    "en1995/S#TimberConnection.endDistanceM": SPACING,
    "en1995/S#TimberConnection.fUK": {"step": 10 * MPA, "snaps": mpa(400, 600, 800, 1000)},
    "en1995/S#TimberMember.dampingXi": {"step": 0.005, "snaps": [0.01, 0.015]},
    "en1995/S#TimberMember.fireDurationS": FIRE,
    "en1995/S#TimberMember.bridgeTLYears": LIFE,
    # EN 1997
    "en1997/S#SoilLayer.depthTop": {"step": 0.1},
    "en1997/S#SoilLayer.depthBottom": {"step": 0.1},
    "en1997/S#SoilLayer.cohesionEffective": {"step": 1000},
    "en1997/S#SoilLayer.cohesionUndrained": {"step": 1000},
    "en1997/S#SoilLayer.cptQc": {"step": 0.5 * MPA},
    "en1997/S#SoilLayer.sptN": COUNT,
    "en1997/S#Pile.diameter": {"step": 10 * MM},
    "en1997/S#Pile.unitShaftResistance": {"step": 1000},
    "en1997/S#Pile.unitBaseResistance": {"step": 0.1 * MPA},
    # EN 1998
    "en1998/S#En1998System.q0": {"step": 0.1, "snaps": [1.5, 2.0, 3.0, 4.5]},
    "en1998/S#En1998System.alphaUOverAlpha1": {"step": 0.05, "snaps": [1.0, 1.1, 1.2, 1.3]},
    "en1998/S#En1998System.kW": {"step": 0.05, "snaps": [0.5, 1.0]},
    "en1998/S#En1998Member.minDimensionM": {"step": 10 * MM},
    "en1998/S#En1998Building.ct": {"step": 0.005, "snaps": [0.05, 0.075, 0.085]},
    "en1998/S#En1998Building.nu": {"step": 0.05, "snaps": [0.4, 0.5]},
    "en1998/S#En1998Building.accidentalEccentricityRatio": {"step": 0.005, "snaps": [0.05, 0.1]},
    "en1998/S#En1998Foundation.pRdPa": {"step": 10 * KN},
    "en1998/S#En1998Foundation.kFoundation": {"step": 10_000},
    "en1998/S#En1998Foundation.kSoil": {"step": 10_000},
    "en1998/S#En1998RetainingWall.r": {"step": 0.5, "snaps": [1.0, 1.5, 2.0]},
    "en1998/S#En1998Silo.qNominal": {"step": 0.1},
    "en1998/S#En1998Tower.qNominal": {"step": 0.1},
    # EN 1999
    "en1999/S#AluminiumMember.length": {**length_m("System length L", "Systemlänge L"), "description": None},
    "en1999/S#AluminiumMember.c1": {"step": 0.01},
    "en1999/S#ColdFormedSheet.thickness": {"step": 0.1 * MM},
    "en1999/S#ColdFormedSheet.width": length_mm("Flat width b", "Ebene Breite b"),
    "en1999/S#BoltGroup.diameter": {"step": 1 * MM, "snaps": mm(6, 8, 10, 12, 16, 20, 24)},
    "en1999/S#BoltGroup.edgeDistance": SPACING,
    "en1999/S#BoltGroup.pitch": SPACING,
    "en1999/S#BoltGroup.gauge": SPACING,
    "en1999/S#BoltGroup.plateThickness": {"step": 0.5 * MM},
    "en1999/S#WeldGroup.length": SPACING,
    "en1999/S#WeldGroup.hazExtent": {"step": 5 * MM, "snaps": mm(20, 30, 35, 40)},
    "en1999/S#FatigueDetail.nCycles": CYCLES,
    "en1999/S#FatigueDetail.m1": {"step": 0.1},
    "en1999/S#FatigueDetail.m2": {"step": 0.1},
    "en1999/S#FireScenario.durationS": FIRE,
    "en1999/S#AluminiumSection.height": length_mm("Section depth h", "Profilhöhe h"),
    "en1999/S#AluminiumSection.width": length_mm("Section width b", "Profilbreite b"),
    "en1999/S#AluminiumSection.flangeThickness": ROLLED_THICKNESS,
    "en1999/S#AluminiumSection.webThickness": ROLLED_THICKNESS,
    "en1999/S#PlateElement.width": length_mm("Flat width b", "Ebene Breite b"),
    "en1999/S#AluminiumShell.radius": length_m("Mid-surface radius r", "Radius der Mittelfläche r"),
    "en1999/S#AluminiumShell.thickness": ROLLED_THICKNESS,
    # VDI 3805
    "vdi3805/S#CurvePoint.x": {"label": {"en": "Abscissa x", "de": "Abszisse x"}, "widget": "stepper", "precision": 3, "step": 0.1},
    "vdi3805/S#CurvePoint.y": {"label": {"en": "Ordinate y", "de": "Ordinate y"}, "widget": "stepper", "precision": 3, "step": 0.1},
    # wfc
    "grid3d/L:change-cell-sizes#.sizes[]": {"label": {"en": "Cell size", "de": "Zellgröße"}, "widget": "stepper", "role": "value", "step": 0.1},
    # process
    "process3d/L:create-machine#.index": count("Position", "Position", description={"en": "Where the machine is inserted in the workshop.", "de": "Wo die Maschine in der Werkstatt eingefügt wird."}),
    "process3d/L:create-step#.index": count("Position", "Position", description={"en": "Where the step is inserted in the process.", "de": "Wo der Schritt im Prozess eingefügt wird."}),
    "process3d/L:create-machine#.machine.capabilities[].parameters[].value": {**length_mm("Size", "Maß"), "role": "value"},
    "process3d/L:replace-machine#.newCapabilities[].parameters[].value": {**length_mm("Size", "Maß"), "role": "value"},
    "process3d/L:reorder-steps#.toIndex": count("Target position", "Zielposition", description={"en": "The position the step moves to.", "de": "Die Position, an die der Schritt rückt."}),
    # remodel: counts step by one; pixel sizes and budgets by the unit their engines quantize to
    "remodeling/A#GcpObservation.frameIndex": COUNT,
    "remodeling/A#FrameRef.index": count("Frame", "Bild", description={"en": "The frame's index in its stream.", "de": "Der Index des Bilds in seinem Stream."}),
    "remodeling/A#FrameRef.timestampMs": {"widget": "stepper", "precision": 0, "step": 1},
    "remodeling/A#CameraCalibration.fx": {"widget": "stepper", "precision": 1, "step": 1},
    "remodeling/A#CameraCalibration.fy": {"widget": "stepper", "precision": 1, "step": 1},
    "remodeling/A#CameraCalibration.cx": {"widget": "stepper", "precision": 1, "step": 1},
    "remodeling/A#CameraCalibration.cy": {"widget": "stepper", "precision": 1, "step": 1},
    "remodeling/A#CameraCalibration.skew": {"widget": "stepper", "precision": 4, "step": 0.001},
    "remodeling/A#MediaStream.syncOffsetMs": {"widget": "stepper", "precision": 0, "step": 1},
    "remodeling/A#MediaStream.fpsHint": {"widget": "stepper", "precision": 2, "step": 1, "snaps": [24, 25, 30, 50, 60, 120]},
    "remodeling/A#VideoSource.durationMs": {"widget": "stepper", "precision": 0, "step": 1},
    "remodeling/A#VideoSource.frameCount": COUNT,
    "remodeling/A#VideoSource.width": count("Width", "Breite", unit="px", description={"en": "The video's frame width, in pixels.", "de": "Die Bildbreite des Videos, in Pixeln."}),
    "remodeling/A#VideoSource.height": count("Height", "Höhe", unit="px", description={"en": "The video's frame height, in pixels.", "de": "Die Bildhöhe des Videos, in Pixeln."}),
    "remodeling/A#DenseParams.windowRadiusPx": COUNT,
    "remodeling/A#DenseParams.minViewConsistency": COUNT,
    "remodeling/A#DenseParams.maxPoints": {"step": 10_000},
    "remodeling/A#FeatureParams.targetCount": {"step": 100},
    "remodeling/A#FeatureParams.octaves": COUNT,
    "remodeling/A#GeoParams.originLon": {"widget": "stepper", "precision": 6, "step": 0.0001},
    "remodeling/A#GeoParams.originLat": {"widget": "stepper", "precision": 6, "step": 0.0001},
    "remodeling/A#GeoParams.originAlt": {"widget": "stepper", "precision": 2, "step": 0.1},
    "remodeling/A#GeoParams.orthoMaxPx": {"step": 256, "snaps": [1024, 2048, 4096, 8192]},
    "remodeling/A#IngestParams.frameSampleStride": COUNT,
    "remodeling/A#IngestParams.maxFrames": {"step": 10},
    "remodeling/A#IngestParams.downscaleLongEdgePx": {"step": 64, "snaps": [0, 1280, 1920, 2560, 3840]},
    "remodeling/A#MatchParams.sequentialWindow": COUNT,
    "remodeling/A#MatchParams.maxPairsPerFrame": COUNT,
    "remodeling/A#MeshParams.decimateTargetTriangles": {"step": 1000},
    "remodeling/A#MeshParams.smoothingIterations": COUNT,
    "remodeling/A#MeshParams.textureSize": {"step": 256, "snaps": [512, 1024, 2048, 4096, 8192]},
    "remodeling/A#MeshParams.holeFillMaxBoundaryVerts": COUNT,
    "remodeling/A#MotionParams.maxTracks": {"step": 100},
    "remodeling/A#MotionParams.trackWindowPx": COUNT,
    "remodeling/A#MotionParams.minTrackLengthFrames": COUNT,
    "remodeling/A#SfmParams.ransacIterations": {"step": 100},
    "remodeling/A#SfmParams.minTrackLength": COUNT,
    "remodeling/A#SfmParams.baMaxIterations": {"step": 10},
}
#endregion 🔖️Field

UI_ORDER = ["label", "description", "widget", "role", "unit", "displayUnit", "displayFactor", "precision", "step", "softMin", "softMax", "snaps"]


#region 🔖️Schemas
def ascii_name(name):
    return re.sub(r"^[^\x00-\x7f]+", "", name)


def load_index(roots):
    index = {}
    for root in roots:
        if not os.path.isdir(root):
            sys.exit(f"not a directory: {root}")
        for directory, _, files in os.walk(root):
            if "🧫️fixtures" in directory or "node_modules" in directory:
                continue
            if "🔣️.json" not in files:
                continue
            path = os.path.join(directory, "🔣️.json")
            try:
                document = json.load(open(path, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
            except ValueError:
                continue
            if isinstance(document, dict) and isinstance(document.get("$id"), str):
                index[document["$id"]] = (path, document)
    if not index:
        sys.exit("no schema document under the given roots")
    return index


def pointer_get(document, pointer):
    node = document
    for token in pointer.split("/")[1:] if pointer else []:
        token = token.replace("~1", "/").replace("~0", "~")
        node = node[int(token)] if isinstance(node, list) else node[token]
    return node


def deref(path, document, pointer, index):
    seen = set()
    while True:
        node = pointer_get(document, pointer)
        if (path, pointer) in seen:
            sys.exit(f"reference cycle at {path}#{pointer}")
        seen.add((path, pointer))
        if isinstance(node, dict) and isinstance(node.get("$ref"), str):
            target, _, fragment = node["$ref"].partition("#")
            if target:
                if target not in index:
                    return None
                path, document = index[target]
            pointer = fragment
            continue
        word = next((word for word in ("anyOf", "oneOf") if isinstance(node, dict) and isinstance(node.get(word), list)), None)
        if word is not None:
            branches = [i for i, branch in enumerate(node[word]) if not (isinstance(branch, dict) and branch.get("type") == "null")]
            if len(branches) == 1:
                pointer = f"{pointer}/{word}/{branches[0]}"
                continue
        return path, document, pointer


def resolve(path, document, payload_pointer, index):
    pointer, owner = "", None
    for token in payload_pointer.split("/")[1:]:
        landed = deref(path, document, pointer, index)
        if landed is None:
            return None
        path, document, pointer = landed
        node = pointer_get(document, pointer)
        if token == "-" and "items" in node:
            pointer = f"{pointer}/items"
        elif isinstance(node.get("properties"), dict) and token in node["properties"]:
            pointer = f"{pointer}/properties/{token}"
        else:
            return None
        owner = (path, pointer)
    return owner


def key_of(path, pointer):
    parts = path.split("/")
    artifact = ascii_name(parts[parts.index("🗿️artifacts") + 1])
    dotted = re.sub(r"/properties/", ".", pointer).replace("/items", "[]")
    if "🧬️mutations" in parts:
        leaf = ascii_name(parts[parts.index("🧬️mutations") + 1])
        return f"{artifact}/L:{leaf}#{dotted}"
    kind = "S" if "📸️snapshot" in parts else "A"
    return f"{artifact}/{kind}#{dotted.replace('/$defs/', '', 1).lstrip('.')}"


def pending(report, index):
    by_path = {path: document for path, document in index.values()}
    owners, unresolved = collections.OrderedDict(), []
    for finding in report["diagnostics"]:
        match = re.match(r'(\w+) at "([^"]*)"', finding["detail"])
        if not match or match.group(1) != "numericUndeclared":
            continue
        leaf = by_path.get(finding["path"])
        owner = resolve(finding["path"], leaf, match.group(2), index) if leaf is not None else None
        if owner is None:
            unresolved.append(f'{finding["path"]} {match.group(2)}')
            continue
        owners.setdefault(owner, []).append(finding["scope"].rsplit(".", 1)[-1])
    if unresolved:
        sys.exit("unresolved finding(s):\n" + "\n".join(unresolved[:20]))
    return owners
#endregion 🔖️Schemas


#region 🔖️Declare
def rule_of(key, ui):
    if key in FIELD:
        return FIELD[key]
    quantity = (ui.get("unit"), ui.get("displayUnit"), ui.get("precision"))
    step = ARTIFACT_CLASS.get((key.split("/", 1)[0], quantity), CLASS.get(quantity))
    return None if step is None else {"step": step}


def merged(ui, rule):
    out = collections.OrderedDict(ui)
    for name, value in rule.items():
        if value is None:
            out.pop(name, None)
        else:
            out[name] = value
    step, snaps = out.get("step"), out.get("snaps")
    if not (isinstance(step, (int, float)) and step > 0):
        sys.exit(f"a rule declares no positive step: {rule}")
    if snaps is not None and snaps != sorted(set(snaps)):
        sys.exit(f"snaps are not strictly ascending: {snaps}")
    for name in ("step", "snaps"):
        value = out.get(name)
        if isinstance(value, float) and value.is_integer():
            out[name] = int(value)
        if isinstance(value, list):
            out[name] = [int(item) if isinstance(item, float) and item.is_integer() else item for item in value]
    added = sorted((name for name in out if name not in ui), key=lambda name: UI_ORDER.index(name) if name in UI_ORDER else len(UI_ORDER))
    return collections.OrderedDict([(name, out[name]) for name in ui if name in out] + [(name, out[name]) for name in added])


def printed(document, raw):
    """`document` in the file's own style: two-space indent, and every scalar array the file keeps on one line kept there
    (matched by its ordinal among equal blocks, so an equal array the file prints expanded stays expanded)."""
    text = json.dumps(document, indent=2, ensure_ascii=False) + "\n"
    plan, canonical, last = [], "", 0
    for inline in re.finditer(r'^( *)("[^"\n]+": )?(\[[^\[\]\n]+\])(,?)\n', raw, re.M):
        indent, name, array, comma = inline.groups()
        expanded = json.dumps(json.loads(array), indent=2, ensure_ascii=False).replace("\n", "\n" + indent)
        block = f"{indent}{name or ''}{expanded}{comma}\n"
        canonical += raw[last:inline.start()]
        plan.append((block, inline.group(0), canonical.count(block)))
        canonical += block
        last = inline.end()
    for block, line, ordinal in reversed(plan):
        at = -1
        for _ in range(ordinal + 1):
            at = text.find(block, at + 1)
            if at < 0:
                return text
        text = text[:at] + line + text[at + len(block):]
    return text


def locate(key, index):
    artifact, rest = key.split("/", 1)
    kind, dotted = rest.split("#", 1)
    for path, document in index.values():
        parts = path.split("/")
        if "🗿️artifacts" not in parts or ascii_name(parts[parts.index("🗿️artifacts") + 1]) != artifact:
            continue
        leaf = ascii_name(parts[parts.index("🧬️mutations") + 1]) if "🧬️mutations" in parts else None
        if kind.startswith("L:"):
            if leaf != kind[2:]:
                continue
            pointer = dotted.replace("[]", "/items").replace(".", "/properties/")
        else:
            if leaf is not None or ("📸️snapshot" in parts) != (kind == "S") or parts[-2] not in ("📸️snapshot", "🧬️schema"):
                continue
            definition, _, tail = dotted.partition(".")
            pointer = f"/$defs/{definition}/properties/" + tail.replace("[]", "/items").replace(".", "/properties/")
        try:
            return path, pointer, pointer_get(document, pointer)
        except (KeyError, IndexError, TypeError):
            continue
    return None


def main():
    if len(sys.argv) < 4 or sys.argv[1] not in ("census", "declare", "check"):
        sys.exit(__doc__)
    mode, report, index = sys.argv[1], json.load(open(sys.argv[2], encoding="utf-8")), load_index(sys.argv[3:])
    documents = {path: document for path, document in index.values()}
    owners = pending(report, index)
    if mode == "census":
        print("key\tfindings\tunit\tdisplayUnit\tprecision\trule\tlabel")
        for (path, pointer), leaves in owners.items():
            ui = pointer_get(documents[path], pointer).get("x-semio-ui") or {}
            print("\t".join([key_of(path, pointer), str(len(leaves)), str(ui.get("unit")), str(ui.get("displayUnit")), str(ui.get("precision")), json.dumps(rule_of(key_of(path, pointer), ui), ensure_ascii=False), (ui.get("label") or {}).get("en", "")]))
        return
    stale = []
    for key, rule in FIELD.items():
        found = locate(key, index)
        if found is None:
            if any(key.split("/", 1)[0] == ascii_name(part) for path in documents for part in path.split("/")):
                stale.append(f"{key}: no such property")
            continue
        path, pointer, node = found
        if (path, pointer) not in owners and merged(node.get("x-semio-ui") or {}, rule) != (node.get("x-semio-ui") or {}):
            stale.append(f"{key}: on disk without its declaration and not pending")
    if stale:
        sys.exit("stale FIELD row(s):\n" + "\n".join(stale))
    if mode == "check":
        print(f"{len(owners)} pending propert(ies)")
        sys.exit(1 if owners else 0)
    missing, touched = [], collections.OrderedDict()
    for (path, pointer) in owners:
        node = pointer_get(documents[path], pointer)
        key = key_of(path, pointer)
        rule = rule_of(key, node.get("x-semio-ui") or {})
        if rule is None:
            missing.append(key)
            continue
        touched.setdefault(path, []).append((node, rule))
    if missing:
        sys.exit("no rule for:\n" + "\n".join(missing))
    for path, edits in touched.items():
        raw = open(path, encoding="utf-8").read()
        if printed(documents[path], raw) != raw:
            sys.exit(f"{path} does not re-print byte-identically; nothing written")
    for path, edits in touched.items():
        raw = open(path, encoding="utf-8").read()
        for node, rule in edits:
            node["x-semio-ui"] = merged(node.get("x-semio-ui") or {}, rule)
        open(path, "w", encoding="utf-8").write(printed(documents[path], raw))
    print(f"declared {sum(len(edits) for edits in touched.values())} propert(ies) in {len(touched)} file(s)")


main()
#endregion 🔖️Declare
