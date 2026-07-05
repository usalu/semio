/* Generated from ui/styling/tokens.json — run `bun ./script.ts generate`. */

export const STYLING_TOKENS = {
  "primary": "#ff344f",
  "secondary": "#34d1bf",
  "tertiary": "#fa9500",
  "danger": "#a60009",
  "warning": "#fccf05",
  "info": "#dbbea1",
  "success": "#7eb77f",
  "dark": "#001117",
  "gray-100": "#06171c",
  "dark-8-9": "#07181d",
  "d-d-d-g": "#091a1f",
  "dark-6-7": "#0c1c21",
  "dark-dark-gray": "#112025",
  "gray-200": "#18272a",
  "dark-7-9": "#1d2b2f",
  "dark-gray": "#243235",
  "dark-5-7": "#2e3c3d",
  "gray-300": "#334041",
  "dark-dark-gray-2": "#3e494a",
  "d-g-g-g": "#4c5756",
  "gray-400": "#555f5d",
  "dark-4-7": "#606966",
  "dark-5-9": "#666e6b",
  "gray": "#7b827d",
  "light-5-9": "#91968f",
  "light-4-7": "#979b94",
  "gray-600": "#a2a59d",
  "l-g-g-g": "#abada4",
  "light-gray-gray": "#b9bbb0",
  "gray-700": "#c4c4b9",
  "light-5-7": "#c9c8bd",
  "light-gray": "#d3d2c5",
  "light-7-9": "#dad9cb",
  "gray-800": "#dfddd0",
  "light-light-gray": "#e6e4d5",
  "light-6-7": "#ebe8d9",
  "l-l-l-g": "#eeeadb",
  "light-8-9": "#f0ecdd",
  "gray-900": "#f1edde",
  "light": "#f7f3e3",
  "black": "#000000",
  "white": "#ffffff",
  "indirect-handle": "#c4e4d5",
} as const;

export type StylingTokenKey = keyof typeof STYLING_TOKENS;

export const STYLING_STROKES = {
  "edgeBase": 2,
  "edgeSelectedMult": 1.35,
  "edgeHoveredMult": 1.2,
  "edgeMinimap": 1.12,
  "edgeOverview": 2.75,
  "edgeDashLongMult": 3,
  "edgeDashLongGapMult": 2,
  "edgeDashDotMult": 0.35,
  "edgeDashDotGapMult": 1.65,
  "edgeTipMin": 1.25,
  "edgeTipOutlineMult": 0.9,
  "edgeArrowHalfMult": 1.25,
  "nodeBody": 2,
  "wireHighlight": 2.85,
  "handle": 2,
  "selectionPreview": 1.5,
  "selectionPreviewDash": [
    5,
    4
  ],
  "gridLarge": 1,
  "gridMedium": 0.72,
  "gridSmall": 0.48,
  "gridMicro": 0.32,
  "dagNode": 1.5,
  "dagNodeSelected": 2.25,
  "dagNodeHovered": 2,
  "dagEdge": 2,
  "dagEdgeMinimap": 1,
  "dagChrome": 1.25,
  "mapRouteDefault": 2,
  "mapRegionMult": 2,
  "mapPositionMult": 2,
  "mapRoadMotorway": 2.4,
  "mapRoadPath": 0.55,
  "mapRoadClampMin": 0.35,
  "mapRoadClampMax": 2.8,
  "mapRoadMotorwayMult": 2.1,
  "mapRoadTrunkMult": 1.8,
  "mapRoadPrimaryMult": 1.45,
  "mapRoadSecondaryMult": 1.2,
  "mapRoadTertiaryMult": 0.95,
  "mapRoadResidentialMult": 0.78,
  "mapRoadServiceMult": 0.9,
  "puzzle3dOutline": 4,
  "cadConstruction": 1,
  "cadGuide": 1.5,
  "cadDimension": 2,
  "cadPick": 4,
  "cadHighlight": 5,
  "cadEmphasis": 7,
  "cadEmphasisStrong": 8,
  "cadEmphasisMax": 9,
  "cadEmphasisLine": 12,
  "mapBoundaryAdmin2": 1.75,
  "mapBoundaryDefault": 0.65,
  "mapBoundaryClampMin": 0.45,
  "mapBoundaryClampMax": 2.2,
  "mapCoastlineMult": 1.35,
  "mapCoastlineClampMin": 0.85,
  "mapCoastlineClampMax": 2.8,
  "mapLineScaleCap": 1.38,
  "mapLineScaleRaw": 280,
  "mapRoadLodRegion": 0.4,
  "mapRoadLodCity": 0.3,
  "chromeBorderHairline": 1,
  "chromeBorderDefault": 2,
  "chromeBorderFocus": 3
} as const;

export const STYLING_RADII = {
  "nodeDefault": 24,
  "nodeMin": 28,
  "handleDefault": 8,
  "handleMinHalfExtent": 8,
  "nodeRectDefault": 40,
  "dagHandleWorld": 5,
  "mapPositionMarker": 8,
  "mapLabelAnchorOffset": 6,
  "chrome": 0
} as const;

export const STYLING_OPACITIES = {
  "gridMinorAlpha": 0.22,
  "dimStrokeAlpha": 110,
  "dimLabelAlpha": 110,
  "disabledFillAlpha": 120,
  "disabledStrokeAlpha": 120,
  "kindHintAlpha": 140,
  "nodeSelectedFillAlpha": 0.35,
  "selectionPreviewFillAlpha": 0.12,
  "mapLandStrokeAlpha": 0.42,
  "mapLabelHaloAlpha": 0.92,
  "mapRegionFillAlpha": 0.22,
  "mapRegionStrokeAlpha": 0.9,
  "mapRouteStrokeAlpha": 0.92,
  "mapBuildingFillWeight": 220,
  "mapParkFillWeight": 90,
  "disabledMixAlpha": 96,
  "disabledPanelMixAlpha": 128,
  "labelHaloAlpha": 200,
  "selectionPreviewStrokeAlpha": 180,
  "handleFillTransparent": 0
} as const;

export const STYLING_METRICS = {
  "camera": {
    "zoomMin": 0.05,
    "zoomMax": 32,
    "flowZoomMax": 8,
    "wheelZoomInFactor": 1.1,
    "wheelZoomOutFactor": 0.9,
    "lodZoomFloor": 0.05
  },
  "label": {
    "minPx": 4,
    "padRatio": 0.35,
    "charWidthRatio": 0.62,
    "heightRatio": 1.6,
    "widthMin": 32,
    "widthMax": 2048,
    "heightMin": 16,
    "heightMax": 256,
    "haloStrokeRatio": 0.12,
    "haloStrokeMin": 1,
    "scaleRatio": 0.9,
    "scaleMax": 2.5,
    "verticalOffsetRatio": 0.85,
    "mapWidthMin": 24,
    "mapWidthMax": 420,
    "mapHeightMin": 14,
    "mapHeightMax": 96,
    "declutterCellRatio": 1.75,
    "declutterCellMin": 18,
    "declutterCellMax": 44,
    "dagCompactPx": 10,
    "dagDefaultPx": 11,
    "dagLabelScaleMult": 1.05,
    "dagLabelGapRatio": 0.35,
    "dagLabelGapCompactRatio": 0.2,
    "dagKindHintGapRatio": 0.85,
    "handleOffsetPx": 10,
    "puzzle2dDefaultPx": 14
  },
  "icon": {
    "fitInset": 0.76,
    "clipInset": 0.88
  },
  "typst": {
    "iconPagePt": 96,
    "iconMarginPt": 3,
    "emojiPagePt": 88,
    "emojiMarginPt": 2,
    "emojiTextPt": 44,
    "textIconPt": 28,
    "svgMarginPt": 3
  },
  "board": {
    "gridWorldLarge": 10,
    "gridWorldMedium": 2.5,
    "gridWorldSmall": 0.5,
    "gridWorldMicro": 0.1,
    "gridFactorDefault": 10,
    "worldClipTileWorld": 256,
    "maxWorldClipTiles": 768,
    "edgeHitTolerancePx": 8,
    "handleHitTolerancePx": 10,
    "indirectHandleMarkerScale": 0.8,
    "indirectHandleRingGapScale": 0.7,
    "linkDragMinDistancePx": 5,
    "linkHandleSnapExtraPx": 22,
    "linkCommitSnapTightPx": 2,
    "suggestionOffset": 80,
    "brushNodeSize": 40,
    "selectionLassoMinPointDistancePx": 3,
    "selectionClickMaxDistancePx": 4,
    "selectionMarqueeMaxDistancePx": 4,
    "selectionDragDirectionPx": 2,
    "boundedDragHitPadPx": 8,
    "proximityDistanceWorld": 48,
    "handleCapMarginMult": 2.5,
    "handleCapMarginMin": 4,
    "layoutLayerSpacing": 120,
    "layoutSiblingGap": 40
  },
  "dag": {
    "channelRowHeight": 14,
    "nodeEdgeInset": 2,
    "nodeColumnGap": 2,
    "ioColumnWidth": 20,
    "ioWidgetHeight": 28,
    "sliderKnobScreenPx": 8,
    "previewPad": 4,
    "previewRowHeight": 14,
    "previewTreeIndent": 12,
    "previewToggleWidth": 10,
    "previewMaxImage": 200,
    "previewMinSize": 20,
    "variadicPlusZoomThreshold": 1.5,
    "lodZoomShift": 0.25,
    "lodBandFloorZoom": [
      0.05,
      0.15,
      0.35,
      0.55,
      1.25,
      2.5
    ],
    "componentWidth": 40
  },
  "map": {
    "labelPxBands": [
      26,
      26,
      20,
      14,
      12.5,
      12,
      10.5,
      10.5
    ],
    "labelMaxMin": 48,
    "labelMaxMax": 140,
    "layerWeightMin": 0.25,
    "layerWeightMax": 3,
    "layerWeightDefault": 1,
    "layerWeightStep": 0.05,
    "tileBleed": 1,
    "lineScaleDampMin": 0.44,
    "lineScaleDampMax": 1,
    "wheelZoomFactor": 1.12,
    "wheelDeltaMult": 2.5
  },
  "cad": {
    "dimensionFontWorld": 0.22,
    "dragPlaneOffset": 0.06,
    "meshFaceOpacity": 0.72,
    "cameraFitPadding": 1.25,
    "cameraFitMinDistance": 2,
    "hatchSpacing": 0.4,
    "hatchLineWidth": 0.02,
    "hatchDirectionDeg": 30,
    "chunkSize": 256,
    "chunkMaxDistance": 8000
  },
  "puzzle3d": {
    "outlineThickness": 4,
    "vortexDragThresholdPx": 6,
    "lineWidthSelected": 3,
    "lineWidthDefault": 2
  },
  "chrome": {
    "uiSpacingCompactPx": 3.2,
    "navbarHeightUiSpacing": 9,
    "footerHeightUiSpacing": 9,
    "controlHeightUiSpacing": 7,
    "panelHeaderHeightUiSpacing": 7,
    "gapStandardUiSpacing": 1,
    "paddingStandardUiSpacing": 1,
    "panelInsetUiSpacing": 1
  },
  "typography": {
    "text2xsPx": 9.6,
    "textXsPx": 11.2,
    "textSmPx": 12.8,
    "textBasePx": 14.4,
    "textLgPx": 16
  },
  "dom": {
    "iconTinyUiSpacing": 3.75,
    "iconSmallUiSpacing": 6.25,
    "iconBaseUiSpacing": 7.5,
    "iconLargeUiSpacing": 10,
    "treeRowUiSpacing": 7.5,
    "treeToggleUiSpacing": 4.375,
    "treeIndentPerLevelUiSpacing": 3.125,
    "treeIndentLineExtraUiSpacing": 2.1875,
    "propertyLabelColumnUiSpacing": 30,
    "propertyInlineGapUiSpacing": 2.5,
    "propertyStackedGapUiSpacing": 1.25,
    "propertyStackedHysteresisUiSpacing": 7.5,
    "controlValueColumnUiSpacing": 50,
    "windowMeasureValueColumnUiSpacing": 32.5,
    "resizableCornerGrabUiSpacing": 7.5,
    "layoutPanelRailUiSpacing": 70,
    "layoutPanelMinUiSpacing": 46.875,
    "layoutPanelMaxUiSpacing": 150,
    "layoutEngagementMaxUiSpacing": 140,
    "controlTreeRowMinUiSpacing": 6.25,
    "windowMeasureRowMinUiSpacing": 5.625
  }
} as const;

export const STYLING_CANVAS_FONTS = {
  "mapLabelSansFallback": "sans-serif",
  "notoColorEmoji": "Noto Color Emoji"
} as const;

export const STYLING_BOARD_THEMES = {
  "light": {
    "rasterClear": [
      240,
      236,
      221,
      255
    ],
    "gridMinorStroke": [
      123,
      130,
      125,
      56
    ],
    "edgeStroke": [
      123,
      130,
      125,
      255
    ],
    "edgeStrokeHovered": [
      0,
      17,
      23,
      255
    ],
    "edgeStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "edgeStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "edgeStrokeDisabled": [
      47,
      49,
      48,
      97
    ],
    "nodeFill": [
      238,
      234,
      219,
      255
    ],
    "nodeStroke": [
      123,
      130,
      125,
      255
    ],
    "nodeFillHovered": [
      123,
      130,
      125,
      255
    ],
    "nodeStrokeHovered": [
      0,
      17,
      23,
      255
    ],
    "nodeFillSelected": [
      255,
      52,
      79,
      255
    ],
    "nodeStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "nodeFillSelectionExit": [
      165,
      202,
      189,
      255
    ],
    "nodeStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "nodeFillDisabled": [
      101,
      100,
      95,
      128
    ],
    "nodeStrokeDisabled": [
      47,
      49,
      48,
      97
    ],
    "indirectHandleFill": [
      196,
      228,
      213,
      255
    ],
    "indirectHandleStroke": [
      52,
      209,
      191,
      255
    ],
    "handleFill": [
      247,
      243,
      227,
      0
    ],
    "handleStroke": [
      123,
      130,
      125,
      255
    ],
    "handleFillHovered": [
      123,
      130,
      125,
      255
    ],
    "handleStrokeHovered": [
      123,
      130,
      125,
      255
    ],
    "handleFillSelected": [
      255,
      52,
      79,
      255
    ],
    "handleStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "handleFillSelectionExit": [
      165,
      202,
      189,
      255
    ],
    "handleStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "handleFillDisabled": [
      101,
      100,
      95,
      128
    ],
    "handleStrokeDisabled": [
      47,
      49,
      48,
      97
    ],
    "wireStroke": [
      123,
      130,
      125,
      255
    ],
    "wireStrokeHovered": [
      0,
      17,
      23,
      255
    ],
    "wireStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "wireStrokeHighlighted": [
      52,
      209,
      191,
      255
    ],
    "wireStrokeDisabled": [
      47,
      49,
      48,
      97
    ],
    "selectionPreviewFill": [
      31,
      6,
      9,
      31
    ],
    "selectionPreviewStroke": [
      255,
      52,
      79,
      179
    ],
    "labelFill": [
      123,
      130,
      125,
      255
    ],
    "labelFillHovered": [
      0,
      17,
      23,
      255
    ],
    "labelHalo": [
      240,
      236,
      221,
      199
    ]
  },
  "dark": {
    "rasterClear": [
      12,
      28,
      33,
      255
    ],
    "gridMinorStroke": [
      85,
      95,
      93,
      56
    ],
    "edgeStroke": [
      162,
      165,
      157,
      255
    ],
    "edgeStrokeHovered": [
      247,
      243,
      227,
      255
    ],
    "edgeStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "edgeStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "edgeStrokeDisabled": [
      32,
      36,
      35,
      97
    ],
    "nodeFill": [
      46,
      60,
      61,
      255
    ],
    "nodeStroke": [
      162,
      165,
      157,
      255
    ],
    "nodeFillHovered": [
      85,
      95,
      93,
      255
    ],
    "nodeStrokeHovered": [
      247,
      243,
      227,
      255
    ],
    "nodeFillSelected": [
      255,
      52,
      79,
      255
    ],
    "nodeStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "nodeFillSelectionExit": [
      47,
      96,
      92,
      255
    ],
    "nodeStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "nodeFillDisabled": [
      23,
      30,
      31,
      128
    ],
    "nodeStrokeDisabled": [
      32,
      36,
      35,
      97
    ],
    "indirectHandleFill": [
      47,
      96,
      92,
      255
    ],
    "indirectHandleStroke": [
      52,
      209,
      191,
      255
    ],
    "handleFill": [
      12,
      28,
      33,
      0
    ],
    "handleStroke": [
      162,
      165,
      157,
      255
    ],
    "handleFillHovered": [
      85,
      95,
      93,
      255
    ],
    "handleStrokeHovered": [
      162,
      165,
      157,
      255
    ],
    "handleFillSelected": [
      255,
      52,
      79,
      255
    ],
    "handleStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "handleFillSelectionExit": [
      47,
      96,
      92,
      255
    ],
    "handleStrokeSelectionExit": [
      52,
      209,
      191,
      255
    ],
    "handleFillDisabled": [
      23,
      30,
      31,
      128
    ],
    "handleStrokeDisabled": [
      32,
      36,
      35,
      97
    ],
    "wireStroke": [
      162,
      165,
      157,
      255
    ],
    "wireStrokeHovered": [
      247,
      243,
      227,
      255
    ],
    "wireStrokeSelected": [
      255,
      52,
      79,
      255
    ],
    "wireStrokeHighlighted": [
      52,
      209,
      191,
      255
    ],
    "wireStrokeDisabled": [
      32,
      36,
      35,
      97
    ],
    "selectionPreviewFill": [
      31,
      6,
      9,
      31
    ],
    "selectionPreviewStroke": [
      255,
      52,
      79,
      179
    ],
    "labelFill": [
      211,
      210,
      197,
      255
    ],
    "labelFillHovered": [
      247,
      243,
      227,
      255
    ],
    "labelHalo": [
      12,
      28,
      33,
      199
    ]
  }
} as const;

export type StylingThemeName = keyof typeof STYLING_BOARD_THEMES;

export const STYLING_CANVAS_THEMES = {
  "light": {
    "rasterClear": [
      240,
      236,
      221,
      255
    ],
    "iconFg": [
      0,
      0,
      0,
      255
    ],
    "iconBg": [
      255,
      255,
      255,
      255
    ],
    "labelFill": [
      0,
      17,
      23,
      255
    ],
    "labelHalo": [
      247,
      243,
      227,
      255
    ]
  },
  "dark": {
    "rasterClear": [
      12,
      28,
      33,
      255
    ],
    "iconFg": [
      247,
      243,
      227,
      255
    ],
    "iconBg": [
      0,
      17,
      23,
      255
    ],
    "labelFill": [
      247,
      243,
      227,
      255
    ],
    "labelHalo": [
      12,
      28,
      33,
      255
    ]
  }
} as const;


export const STYLING_CHROME_THEMES = {
  "light": {
    "base": [
      247,
      243,
      227,
      255
    ],
    "canvas": [
      240,
      236,
      221,
      255
    ],
    "window": [
      235,
      232,
      217,
      255
    ],
    "panel": [
      201,
      200,
      189,
      255
    ],
    "foreground": [
      0,
      17,
      23,
      255
    ],
    "mutedForeground": [
      123,
      130,
      125,
      255
    ],
    "accent": [
      255,
      52,
      79,
      255
    ],
    "accentForeground": [
      0,
      17,
      23,
      255
    ],
    "activeBase": [
      255,
      52,
      79,
      255
    ],
    "activeForeground": [
      0,
      17,
      23,
      255
    ],
    "activeHover": [
      230,
      47,
      71,
      255
    ],
    "hoverInteractiveFill": [
      123,
      130,
      125,
      255
    ],
    "hoverWindow": [
      211,
      210,
      197,
      255
    ],
    "hoverPanel": [
      162,
      165,
      157,
      255
    ],
    "borderNormal": [
      123,
      130,
      125,
      255
    ],
    "borderEmphasized": [
      0,
      17,
      23,
      255
    ],
    "borderElement": [
      123,
      130,
      125,
      255
    ],
    "temporary": [
      151,
      155,
      148,
      255
    ],
    "overlayBg": [
      151,
      155,
      148,
      250
    ]
  },
  "dark": {
    "base": [
      0,
      17,
      23,
      255
    ],
    "canvas": [
      12,
      28,
      33,
      255
    ],
    "window": [
      7,
      24,
      29,
      255
    ],
    "panel": [
      29,
      43,
      47,
      255
    ],
    "foreground": [
      247,
      243,
      227,
      255
    ],
    "mutedForeground": [
      123,
      130,
      125,
      255
    ],
    "accent": [
      255,
      52,
      79,
      255
    ],
    "accentForeground": [
      247,
      243,
      227,
      255
    ],
    "activeBase": [
      255,
      52,
      79,
      255
    ],
    "activeForeground": [
      247,
      243,
      227,
      255
    ],
    "activeHover": [
      230,
      47,
      71,
      255
    ],
    "hoverInteractiveFill": [
      123,
      130,
      125,
      255
    ],
    "hoverWindow": [
      36,
      50,
      53,
      255
    ],
    "hoverPanel": [
      85,
      95,
      93,
      255
    ],
    "borderNormal": [
      123,
      130,
      125,
      255
    ],
    "borderEmphasized": [
      247,
      243,
      227,
      255
    ],
    "borderElement": [
      123,
      130,
      125,
      255
    ],
    "temporary": [
      36,
      50,
      53,
      255
    ],
    "overlayBg": [
      36,
      50,
      53,
      250
    ]
  }
} as const;


export const STYLING_MAP_THEMES = {
  "light": {
    "surfaceClear": [
      12,
      28,
      33,
      255
    ],
    "landFill": [
      46,
      60,
      61,
      255
    ],
    "landStroke": [
      51,
      64,
      65,
      107
    ],
    "labelFill": [
      247,
      243,
      227,
      255
    ],
    "labelHalo": [
      12,
      28,
      33,
      235
    ],
    "regionFill": [
      52,
      209,
      191,
      56
    ],
    "regionStroke": [
      52,
      209,
      191,
      230
    ],
    "routeStroke": [
      250,
      149,
      0,
      235
    ],
    "positionFill": [
      255,
      52,
      79,
      255
    ],
    "positionStroke": [
      247,
      243,
      227,
      255
    ]
  },
  "dark": {
    "surfaceClear": [
      6,
      23,
      28,
      255
    ],
    "landFill": [
      12,
      28,
      33,
      255
    ],
    "landStroke": [
      51,
      64,
      65,
      107
    ],
    "labelFill": [
      247,
      243,
      227,
      255
    ],
    "labelHalo": [
      6,
      23,
      28,
      235
    ],
    "regionFill": [
      52,
      209,
      191,
      56
    ],
    "regionStroke": [
      52,
      209,
      191,
      230
    ],
    "routeStroke": [
      250,
      149,
      0,
      235
    ],
    "positionFill": [
      255,
      52,
      79,
      255
    ],
    "positionStroke": [
      247,
      243,
      227,
      255
    ]
  }
} as const;

